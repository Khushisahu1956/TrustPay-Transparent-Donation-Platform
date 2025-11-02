# TrustPay - Decentralized Donation Platform

TrustPay is a Soroban smart contract that enables transparent and efficient charitable donations on the Stellar network. It allows donors to contribute to various NGOs while maintaining a transparent record of all donations on the blockchain.

## Contract Details

- **Network**: Stellar Testnet
- **Contract ID**: `CC5E6IYEXHXVZUQ4VIF6TMFGGCN2NY6EMHAZNHCLWUHMCSZ4QSJKVT6F`
- **Contract Interface**:
  - `initialize()`: Initialize the contract storage
  - `donate(donor: Address, ngo: Symbol, amount: i128) -> i128`: Make a donation
  - `get_total(ngo: Symbol) -> i128`: Get total donations for an NGO

## Prerequisites

- Rust and Cargo
- Soroban CLI
- Stellar CLI
- A funded Stellar testnet account

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli

# Install Stellar CLI
cargo install --locked stellar-cli
```

## Building the Contract

```powershell
# Navigate to the contract directory
cd contracts/hello-world

# Build the contract
soroban contract build
```

The WASM file will be generated at `target/wasm32v1-none/release/hello_world.wasm`

## Deployment

1. Generate a keypair (if you don't have one):
```powershell
stellar keys generate your_key_name --network testnet --fund
```

2. Deploy the contract:
```powershell
stellar contract deploy `
  --wasm target/wasm32v1-none/release/hello_world.wasm `
  --source-account your_key_name `
  --network testnet
```

3. Initialize the contract:
```powershell
stellar contract invoke `
  --id YOUR_CONTRACT_ID `
  --source-account your_key_name `
  --network testnet `
  -- `
  initialize
```

## Usage

### Make a Donation

```powershell
stellar contract invoke `
  --id YOUR_CONTRACT_ID `
  --source-account your_key_name `
  --network testnet `
  -- `
  donate `
  --donor YOUR_PUBLIC_KEY `
  --ngo NGO_NAME `
  --amount AMOUNT
```

### Check Donation Total

```powershell
stellar contract invoke `
  --id YOUR_CONTRACT_ID `
  --source-account your_key_name `
  --network testnet `
  -- `
  get_total `
  --ngo NGO_NAME
```

## Development

The contract is written in Rust using the Soroban SDK. The main components are:

- `lib.rs`: Contains the main contract logic
- `test.rs`: Contains unit tests

### Running Tests

```powershell
cargo test
```

## Project Structure

```
trustpay-fresh/
├── Cargo.toml
├── README.md
└── contracts/
    └── hello-world/
        ├── Cargo.toml
        ├── Makefile
        └── src/
            ├── lib.rs
            └── test.rs
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

[MIT License](LICENSE)

## Security

This contract is currently deployed on testnet and should not be used in production without proper security audits.

## Resources

- [Soroban Documentation](https://soroban.stellar.org/)
- [Stellar Documentation](https://developers.stellar.org/)
- [Contract on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CC5E6IYEXHXVZUQ4VIF6TMFGGCN2NY6EMHAZNHCLWUHMCSZ4QSJKVT6F)
- <img width="1916" height="1072" alt="image" src="https://github.com/user-attachments/assets/6d3f4684-04d9-4303-b3d7-60afe694e614" />

