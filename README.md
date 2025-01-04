# sui_wallet

CLI tool for Sui wallets manager.

## Features

- Wallet management (create, import, edit, list)
- RPC endpoint management
- Wallet tagging system
- Balance checking
- Faucet request for testnet or devnet
- Encrypted storage for secure key management

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)

### Building from source

```bash
git clone https://github.com/yourusername/sui_wallet
cd sui_wallet
cargo build --release
```

The binary will be available at `target/release/sui_wallet`

## Initial Setup

Before using the wallet, you need to set up encryption keys for secure storage:

1. Generate encryption keys:

```bash
sui_wallet new-cipher
```

This command will output a `CIPHER_KEY` and `CIPHER_NONCE` which you'll need for the next step.

2. Create a `.env` file in your project root and add the following:

```env
# Encryption settings (Required)
CIPHER_KEY=your_generated_cipher_key
CIPHER_NONCE=your_generated_cipher_nonce
```

⚠️ **Important**: Keep your `CIPHER_KEY` and `CIPHER_NONCE` safe. These are used to encrypt and decrypt your wallet data. If you lose them, you won't be able to access your stored wallets.

## Usage

### Basic Commands

```bash
# Create a new wallet
sui_wallet create

# Import an existing wallet
sui_wallet import <mnemonic|--address <SuiAddress>>

# Edit wallets
sui_wallet edit <Alias_or_SuiAddress>

# List wallets
sui_wallet list

# Manage RPC endpoints
sui_wallet rpc add <Url> --alias <Alias>
sui_wallet rpc list
sui_wallet rpc remove <Alias_or_Url>

# Tag management
sui_wallet tag add <Names>
sui_wallet tag list
sui_wallet tag remove <Names>

# Check balance
sui_wallet balance <Alias_or_SuiAddress> --rpc <Rpc>

# Faucet Request testnet or Devnet tokens
sui_wallet faucet <Alias_or_SuiAddress> --env <Env>
```

### Configuration

The application uses a configuration file stored in the following location:

- macOS: `/Users/{username}/Library/Application Support/rs.sui_wallet_cli/wallets.yml`

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test
```
