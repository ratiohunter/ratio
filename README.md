# RATIO Token Documentation

## Overview

RATIO is a meme-to-earn reward token on Solana. Distributed for good takes, brutal ratios, and community contributions. Fixed supply of 1 billion tokens with 800M reserved for long-term emissions.

## Emissions Program

The RATIO Emissions Program enables off-chain signed ticket claims for distributing RATIO tokens. Users receive signed tickets from the admin, which they can redeem on-chain for tokens.

### Program Deployment

| Field | Value |
|-------|-------|
| **Program ID** | `AaV9HbmGwZ43vpUY5rnRP3m1WwyWKi2LZtKnqpQ2ZXbF` |
| **IDL Account** | `GzrtcP6dRCuQrecCcKprWiwzL834FE1en9brnPMsDjKe` |
| **Upgrade Authority** | `8AdTdtN5L7ovwmQ2rahUNxPiTwtyHKX5Uo6yKsz51foU` (Squads Multisig) |
| **Network** | Solana Mainnet |
| **Explorer** | [View on Explorer](https://orb.helius.dev/address/AaV9HbmGwZ43vpUY5rnRP3m1WwyWKi2LZtKnqpQ2ZXbF) |

### Program Instructions

1. **`initialize_config`** - Initialize the global config with admin pubkey, token mint, and vault PDA
2. **`claim`** - Claim RATIO tokens using an off-chain signed ticket with ed25519 signature verification

### PDA Seeds

- **Config**: `["config"]`
- **Vault**: `["vault"]`
- **ClaimRecord**: `["claim", user_pubkey, nonce.to_le_bytes()]`

### Vault Account

The vault PDA holds RATIO tokens for distribution via claims.

| Field | Value |
|-------|-------|
| **Vault PDA** | `2WuMZy7EvQtnGkZkMDgDGZUvAptD94EDaMuhCide91dx` |
| **Vault Token Account** | `EGTo9C1GaxJnCMbF9fDxqNNuKpyu82BESHWCu5wCASgv` |
| **Balance** | 800,000,000 RATIO |
| **Explorer** | [View on Explorer](https://orb.helius.dev/address/EGTo9C1GaxJnCMbF9fDxqNNuKpyu82BESHWCu5wCASgv) |

## Tokenomics

### Basic Token Information

- **Name**: RATIO
- **Symbol**: RATIO
- **Total Supply**: 1,000,000,000 (1 billion)
- **Decimals**: 6
- **Token Standard**: SPL Token (Fungible)
- **Mint Authority**: `8AdTdtN5L7ovwmQ2rahUNxPiTwtyHKX5Uo6yKsz51foU` (Squads Multisig)
- **Freeze Authority**: Disabled

### Distribution Breakdown

- **Team**: 10% (100,000,000 RATIO)
- **Governance**: 5% (50,000,000 RATIO)
- **Liquidity Pool**: 5% (50,000,000 RATIO)
- **Emissions**: 80% (800,000,000 RATIO)

The 800M emission allocation is reserved for long-term community rewards, staking incentives, and ecosystem development.

## Token Deployment

### Token: `Dzv9iTdxWrq2CmJ94xUEMQZtHgtpDZvMtK91ucy2UFdm`

- **Network**: Solana Mainnet
- **Supply**: 1,000,000,000 RATIO (1,000,000,000,000,000 raw units)
- **Decimals**: 6
- **Status**: Mint authority controlled by Squads multisig
- **Metadata**: Available with full token information
- **Explorer**: [View on Explorer](https://orb.helius.dev/address/Dzv9iTdxWrq2CmJ94xUEMQZtHgtpDZvMtK91ucy2UFdm)

## Development Setup

### Prerequisites

- Rust 1.84.1+
- Solana CLI 3.0.11+ (Agave)
- Anchor CLI 0.32.1+
- Node.js 18+

### Build

```bash
cd emissions
anchor build
```

### Test

```bash
anchor test
```

## Technical Details

### Mint Authority Management

The RATIO token mint authority is controlled by a Squads multisig (`8AdTdtN5L7ovwmQ2rahUNxPiTwtyHKX5Uo6yKsz51foU`). Any future minting requires multisig approval.

### Metadata Information

The tokens include comprehensive metadata with the following information:

- Token name, symbol, and description
- Logo/image hosted on IPFS via Pinata
- Social media links (Twitter, Telegram)
- Token attributes including supply, decimals, and distribution details

## Social Links & Resources

- **Twitter**: [@ratio_hunter_10](https://x.com/ratio_hunter_10)
- **Telegram**: [@ratio_hunter_10](https://t.me/ratio_hunter_10)
- **Logo**: [IPFS Link](https://orange-necessary-stoat-59.mypinata.cloud/ipfs/bafkreie64n4xxmaospfwqonl5h3jupdbhjhdnau2jep4mp2u3rbzipusta)

## Security Considerations

1. **Controlled Supply**: Mint authority is managed by Squads multisig
2. **No Freeze Authority**: Tokens cannot be frozen in user accounts
3. **Decentralized Distribution**: 80% of supply reserved for community emissions
4. **Transparent Tokenomics**: All allocation percentages are publicly documented
5. **Ed25519 Signature Verification**: Claims require valid admin signatures verified on-chain
6. **Replay Protection**: Each claim uses a unique nonce to prevent double-claims

## CI/CD

The project uses GitHub Actions for automated deployments. See `.github/workflows/` for workflow configurations.
