# RATIO Token Documentation

## Overview

RATIO is a meme-to-earn reward token on Solana. Distributed for good takes, brutal ratios, and community contributions. Fixed supply of 1 billion tokens with 800M reserved for long-term emissions.

## Emissions Program

The RATIO Emissions Program enables off-chain signed ticket claims for distributing RATIO tokens. Users receive signed tickets from the admin, which they can redeem on-chain for tokens.

### Program Deployments

#### Devnet

| Field | Value |
|-------|-------|
| **Program ID** | `AaV9HbmGwZ43vpUY5rnRP3m1WwyWKi2LZtKnqpQ2ZXbF` |
| **IDL Account** | `GzrtcP6dRCuQrecCcKprWiwzL834FE1en9brnPMsDjKe` |
| **Network** | Solana Devnet |
| **Explorer** | [View on Explorer](https://orb.helius.dev/address/AaV9HbmGwZ43vpUY5rnRP3m1WwyWKi2LZtKnqpQ2ZXbF?cluster=devnet) |

#### Mainnet

| Field | Value |
|-------|-------|
| **Program ID** | *Not yet deployed* |
| **Network** | Solana Mainnet |

### Program Instructions

1. **`initialize_config`** - Initialize the global config with admin pubkey, token mint, and vault PDA
2. **`claim`** - Claim RATIO tokens using an off-chain signed ticket with ed25519 signature verification

### PDA Seeds

- **Config**: `["config"]`
- **Vault**: `["vault"]`
- **ClaimRecord**: `["claim", user_pubkey, nonce.to_le_bytes()]`

## Tokenomics

### Basic Token Information

- **Name**: RATIO
- **Symbol**: RATIO
- **Total Supply**: 1,000,000,000 (1 billion)
- **Decimals**: 6
- **Token Standard**: SPL Token (Fungible)
- **Mint Authority**: Disabled (Fixed Supply)
- **Freeze Authority**: Disabled

### Distribution Breakdown

- **Team**: 10% (100,000,000 RATIO)
- **Governance**: 5% (50,000,000 RATIO)
- **Liquidity Pool**: 5% (50,000,000 RATIO)
- **Emissions**: 80% (800,000,000 RATIO)

The 800M emission allocation is reserved for long-term community rewards, staking incentives, and ecosystem development.

## Token Deployments

### Mainnet Tokens

#### Token 1: `EnzqcFzo5Kkxr9cKtyLQcoN6qw8MG3auDJR1sHMJjVmG`

- **Network**: Solana Mainnet
- **Supply**: 1,000,000,000 RATIO (1,000,000,000,000,000 raw units)
- **Decimals**: 6
- **Status**: Fixed supply (mint authority disabled)
- **Metadata**: Available with full token information
- **Explorer**: [View on Explorer](https://orb.helius.dev/address/EnzqcFzo5Kkxr9cKtyLQcoN6qw8MG3auDJR1sHMJjVmG)

#### Token 2: `Dzv9iTdxWrq2CmJ94xUEMQZtHgtpDZvMtK91ucy2UFdm`

- **Network**: Solana Mainnet
- **Supply**: 1,000,000,000 RATIO (1,000,000,000,000,000 raw units)
- **Decimals**: 6
- **Status**: Fixed supply (mint authority disabled)
- **Metadata**: Available with full token information
- **Explorer**: [View on Explorer](https://orb.helius.dev/address/Dzv9iTdxWrq2CmJ94xUEMQZtHgtpDZvMtK91ucy2UFdm)

### Devnet Tokens (Testing)

#### Token 1: `dkFhRYnirenXzKuSXvjAM8fYGrZPAceRGBqLeM2EmyH`

- **Network**: Solana Devnet
- **Supply**: 1,000,000,000 RATIO (1,000,000,000,000,000 raw units)
- **Decimals**: 6
- **Status**: Fixed supply (mint authority disabled)
- **Metadata**: Not available (disabled mint authority prevented metadata creation)
- **Explorer**: [View on Explorer](https://orb.helius.dev/address/dkFhRYnirenXzKuSXvjAM8fYGrZPAceRGBqLeM2EmyH?cluster=devnet)

#### Token 2: `8WZt7ANLrA641zhUuFVraMeFyGXF7RpoqJNRUdZ91pcL`

- **Network**: Solana Devnet
- **Supply**: 1,000,000,000 RATIO (1,000,000,000,000,000 raw units)
- **Decimals**: 6
- **Status**: Fixed supply (mint authority disabled)
- **Metadata**: Available with basic token information
- **Metadata PDA**: `FrQvwfUSXdJv49Je7sPNv1Pay5mKsVEteg9tE78Jokmd`
- **Explorer**: [View on Explorer](https://orb.helius.dev/address/8WZt7ANLrA641zhUuFVraMeFyGXF7RpoqJNRUdZ91pcL?cluster=devnet)

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

### Deploy to Devnet

```bash
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

## Technical Details

### Mint Authority Management

All RATIO tokens have had their mint authority permanently disabled to ensure a fixed supply cap of 1 billion tokens. This prevents any future minting and guarantees scarcity.

### Metadata Information

The tokens include comprehensive metadata with the following information:

- Token name, symbol, and description
- Logo/image hosted on IPFS via Pinata
- Social media links (Twitter, Telegram)
- Token attributes including supply, decimals, and distribution details

### Token Accounts

Each deployment includes associated token accounts for holding the initial supply:

**Mainnet Token Accounts:**

- Token 1 Account: *[Generated during minting]*
- Token 2 Account: *[Generated during minting]*

**Devnet Token Accounts:**

- Token 1 Account: `2WjzZZjxxSui2AdBhTQcNKcbfR92YqTjmozo6DLcC7NR`
- Token 2 Account: `5aoY36R5zQqhhZZGCkYS2hTvRZCGugDHop6FkqNJjbVR`

## Social Links & Resources

- **Twitter**: [@ratio_hunter_10](https://x.com/ratio_hunter_10)
- **Telegram**: [@ratio_hunter_10](https://t.me/ratio_hunter_10)
- **Logo**: [IPFS Link](https://orange-necessary-stoat-59.mypinata.cloud/ipfs/bafkreie64n4xxmaospfwqonl5h3jupdbhjhdnau2jep4mp2u3rbzipusta)

## Security Considerations

1. **Fixed Supply**: Mint authority has been permanently disabled on all tokens
2. **No Freeze Authority**: Tokens cannot be frozen in user accounts
3. **Decentralized Distribution**: 80% of supply reserved for community emissions
4. **Transparent Tokenomics**: All allocation percentages are publicly documented
5. **Ed25519 Signature Verification**: Claims require valid admin signatures verified on-chain
6. **Replay Protection**: Each claim uses a unique nonce to prevent double-claims

## CI/CD

The project uses GitHub Actions for automated deployments:

- **Devnet**: Automatic deployment on push to `main` branch
- **Mainnet**: Manual trigger with confirmation required

See `.github/workflows/` for workflow configurations.
