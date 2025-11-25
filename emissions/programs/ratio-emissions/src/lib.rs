use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "RATIO Emissions",
    project_url: "https://github.com/ratiohunter/ratio",
    contacts: "link:https://x.com/ratio_hunter_10,link:https://t.me/ratio_hunter_10",
    policy: "https://github.com/ratiohunter/ratio/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/ratiohunter/ratio",
    auditors: "N/A"
}

/// Ed25519 program ID
pub const ED25519_PROGRAM_ID: Pubkey = pubkey!("Ed25519SigVerify111111111111111111111111111");

declare_id!("AaV9HbmGwZ43vpUY5rnRP3m1WwyWKi2LZtKnqpQ2ZXbF");

// =============================================================================
// Errors
// =============================================================================

#[error_code]
pub enum EmissionsError {
    #[msg("The claim ticket has expired")]
    TicketExpired,

    #[msg("This ticket has already been claimed")]
    AlreadyClaimed,

    #[msg("Invalid signature on the claim ticket")]
    InvalidSignature,
}

#[program]
pub mod ratio_emissions {
    use super::*;

    /// Initialize the global config account.
    /// Sets admin, token mint, and vault PDA info.
    pub fn initialize_config(ctx: Context<InitializeConfig>, token_mint: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;

        // Derive the vault PDA
        let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault"], ctx.program_id);

        // Initialize config fields
        config.admin_pubkey = ctx.accounts.initializer.key();
        config.token_mint = token_mint;
        config.vault_pda = vault_pda;
        config.vault_bump = vault_bump;

        Ok(())
    }

    /// Claim RATIO tokens using an off-chain signed ticket.
    /// Verifies signature, expiry, and nonce uniqueness.
    pub fn claim(
        ctx: Context<Claim>,
        amount: u64,
        nonce: u64,
        expiry: i64,
        _signature: [u8; 64],
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        // 1. Check ticket has not expired
        require!(now <= expiry, EmissionsError::TicketExpired);

        // 2. Build the ticket message for signature verification
        let message = build_ticket_message(&ctx.accounts.user.key(), amount, nonce, expiry);

        // 3. Verify ed25519 signature against config.admin_pubkey
        // Uses instruction introspection to verify a preceding Ed25519Program instruction
        verify_admin_signature(
            &ctx.accounts.instructions_sysvar,
            &config.admin_pubkey,
            &message,
            &_signature,
        )?;

        // 4. Initialize the claim record (prevents double-claims)
        // The account is initialized via Anchor's `init` constraint in the context.
        // If this nonce was already used, the PDA would already exist and init would fail.
        let claim_record = &mut ctx.accounts.claim_record;
        claim_record.user = ctx.accounts.user.key();
        claim_record.nonce = nonce;
        claim_record.amount = amount;
        claim_record.claimed_at = now;

        // 5. Transfer tokens from vault to user
        let vault_bump = config.vault_bump;
        let seeds: &[&[u8]] = &[b"vault", &[vault_bump]];
        let signer_seeds = &[seeds];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.vault_pda.to_account_info(),
            },
            signer_seeds,
        );

        token::transfer(transfer_ctx, amount)?;

        Ok(())
    }
}

// =============================================================================
// Account Structs
// =============================================================================

/// Global configuration for the emissions program.
/// PDA seeds: ["config"]
#[account]
#[derive(InitSpace)]
pub struct Config {
    /// The admin authority who can manage the program
    pub admin_pubkey: Pubkey,
    /// The RATIO token mint address
    pub token_mint: Pubkey,
    /// The vault PDA that holds tokens for distribution
    pub vault_pda: Pubkey,
    /// Bump seed for the vault PDA
    pub vault_bump: u8,
}

/// Record of a claimed ticket to prevent double-claims.
/// PDA seeds: ["claim", user, nonce.to_le_bytes()]
#[account]
#[derive(InitSpace)]
pub struct ClaimRecord {
    /// The user who claimed this ticket
    pub user: Pubkey,
    /// Unique nonce for this claim (prevents replay)
    pub nonce: u64,
    /// Amount of tokens claimed
    pub amount: u64,
    /// Unix timestamp when the claim was processed
    pub claimed_at: i64,
}

// =============================================================================
// Instruction Contexts
// =============================================================================

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    /// The config PDA to be initialized
    /// Seeds: ["config"]
    #[account(
        init,
        payer = initializer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    /// The initializer who becomes the admin and pays for account creation
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// Required for creating the config account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64, nonce: u64)]
pub struct Claim<'info> {
    /// Global config account
    /// Seeds: ["config"]
    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    /// The vault PDA that owns the token account
    /// Seeds: ["vault"]
    /// CHECK: This is the vault PDA used as token authority, validated by seeds
    #[account(
        seeds = [b"vault"],
        bump = config.vault_bump,
    )]
    pub vault_pda: UncheckedAccount<'info>,

    /// The vault's token account holding RATIO tokens
    #[account(
        mut,
        token::mint = config.token_mint,
        token::authority = vault_pda,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// The user claiming tokens
    #[account(mut)]
    pub user: Signer<'info>,

    /// The user's token account to receive RATIO tokens
    #[account(
        mut,
        token::mint = config.token_mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// Claim record PDA to prevent double-claims
    /// Seeds: ["claim", user, nonce.to_le_bytes()]
    #[account(
        init,
        payer = user,
        space = 8 + ClaimRecord::INIT_SPACE,
        seeds = [b"claim", user.key().as_ref(), &nonce.to_le_bytes()],
        bump,
    )]
    pub claim_record: Account<'info, ClaimRecord>,

    /// SPL Token program
    pub token_program: Program<'info, Token>,

    /// System program for account creation
    pub system_program: Program<'info, System>,

    /// Instructions sysvar for ed25519 signature verification
    /// CHECK: This is the instructions sysvar, validated by address constraint
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}

// =============================================================================
// Helpers
// =============================================================================

/// Builds the ticket message bytes for signature verification.
/// Format: user_pubkey (32 bytes) || amount (u64 LE) || nonce (u64 LE) || expiry (i64 LE)
///
/// This must match the off-chain signing format exactly.
fn build_ticket_message(user: &Pubkey, amount: u64, nonce: u64, expiry: i64) -> Vec<u8> {
    let mut message = Vec::with_capacity(32 + 8 + 8 + 8);
    message.extend_from_slice(user.as_ref());
    message.extend_from_slice(&amount.to_le_bytes());
    message.extend_from_slice(&nonce.to_le_bytes());
    message.extend_from_slice(&expiry.to_le_bytes());
    message
}

/// Verifies that the admin signed the message using ed25519.
///
/// This uses instruction introspection to verify that a preceding instruction
/// in the same transaction was an Ed25519Program signature verification
/// with the expected pubkey, message, and signature.
///
/// The client must include an Ed25519Program.createInstructionWithPublicKey
/// instruction immediately before calling this program's claim instruction.
fn verify_admin_signature(
    instructions_sysvar: &AccountInfo,
    admin_pubkey: &Pubkey,
    message: &[u8],
    signature: &[u8; 64],
) -> Result<()> {
    // Get the current instruction index
    let current_ix_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| EmissionsError::InvalidSignature)?;

    // The Ed25519 verify instruction must be immediately before our instruction
    require!(current_ix_index > 0, EmissionsError::InvalidSignature);
    let ed25519_ix_index = current_ix_index - 1;

    // Load the preceding instruction
    let ed25519_ix = load_instruction_at_checked(ed25519_ix_index as usize, instructions_sysvar)
        .map_err(|_| EmissionsError::InvalidSignature)?;

    // Verify it's from the Ed25519 program
    require!(
        ed25519_ix.program_id == ED25519_PROGRAM_ID,
        EmissionsError::InvalidSignature
    );

    // Parse and verify the Ed25519 instruction data
    verify_ed25519_ix_data(&ed25519_ix, admin_pubkey, message, signature)?;

    Ok(())
}

/// Parses the Ed25519 instruction data and verifies it matches our expected values.
///
/// Ed25519 instruction data format:
/// - 1 byte: number of signatures
/// - 1 byte: padding
/// - For each signature:
///   - 2 bytes: signature offset (u16 LE)
///   - 2 bytes: signature instruction index (u16 LE)
///   - 2 bytes: public key offset (u16 LE)
///   - 2 bytes: public key instruction index (u16 LE)
///   - 2 bytes: message data offset (u16 LE)
///   - 2 bytes: message data size (u16 LE)
///   - 2 bytes: message instruction index (u16 LE)
/// - Followed by the actual signature, pubkey, and message data
fn verify_ed25519_ix_data(
    ix: &Instruction,
    expected_pubkey: &Pubkey,
    expected_message: &[u8],
    expected_signature: &[u8; 64],
) -> Result<()> {
    let data = &ix.data;

    // Minimum size check: 2 bytes header + 14 bytes per signature descriptor
    require!(data.len() >= 16, EmissionsError::InvalidSignature);

    // Check number of signatures (should be 1)
    let num_signatures = data[0];
    require!(num_signatures == 1, EmissionsError::InvalidSignature);

    // Parse offsets from the signature descriptor (starting at byte 2)
    let sig_offset = u16::from_le_bytes([data[2], data[3]]) as usize;
    let pubkey_offset = u16::from_le_bytes([data[6], data[7]]) as usize;
    let msg_offset = u16::from_le_bytes([data[10], data[11]]) as usize;
    let msg_size = u16::from_le_bytes([data[12], data[13]]) as usize;

    // Verify bounds
    require!(
        sig_offset + 64 <= data.len(),
        EmissionsError::InvalidSignature
    );
    require!(
        pubkey_offset + 32 <= data.len(),
        EmissionsError::InvalidSignature
    );
    require!(
        msg_offset + msg_size <= data.len(),
        EmissionsError::InvalidSignature
    );

    // Extract and verify signature
    let sig_data = &data[sig_offset..sig_offset + 64];
    require!(
        sig_data == expected_signature,
        EmissionsError::InvalidSignature
    );

    // Extract and verify public key
    let pubkey_data = &data[pubkey_offset..pubkey_offset + 32];
    require!(
        pubkey_data == expected_pubkey.as_ref(),
        EmissionsError::InvalidSignature
    );

    // Extract and verify message
    let msg_data = &data[msg_offset..msg_offset + msg_size];
    require!(
        msg_data == expected_message,
        EmissionsError::InvalidSignature
    );

    Ok(())
}
