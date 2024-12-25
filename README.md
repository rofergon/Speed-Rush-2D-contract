# Lens Network NFT Contract

A simple ERC-721 NFT contract deployed on the Lens Network Testnet that allows users to mint NFTs by paying with GRASS tokens.

## Features

- Mintable NFTs with customizable price
- Configurable base URI for NFT metadata
- Owner-only administrative functions
- Secure withdrawal mechanism

## Contract Address

The contract is deployed on Lens Network Testnet at:
`0x51D967b80eaD6601630E0fA18b2101b90f1AB1d0`

## Prerequisites

- Node.js >= v16
- npm or yarn
- A wallet with GRASS tokens (Lens Network's native token)

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd <your-repo-name>
```

2. Install dependencies:
```bash
npm install
```

3. Create a `.env` file in the root directory and add your private key:
```env
PRIVATE_KEY=your_private_key_here
```

## Contract Management Scripts

The project includes several scripts to manage the NFT contract:

### View Contract Information
```bash
node scripts/manage.js
```
Shows current mint price, base URI, owner address, and contract balance.

### Change Mint Price
```bash
node scripts/setPrice.js <new_price>
```
Example: `node scripts/setPrice.js 0.05` sets the mint price to 0.05 GRASS.

### Set Base URI
```bash
node scripts/setURI.js <new_uri>
```
Example: `node scripts/setURI.js "ipfs://QmYourIPFSHash/"` sets the base URI for NFT metadata.

### Withdraw Funds
```bash
node scripts/withdraw.js
```
Withdraws all GRASS tokens from the contract to the owner's address.

## Contract Functions

### For Users
- `mint()`: Mint a new NFT by paying the current mint price in GRASS

### For Contract Owner
- `setMintPrice(uint256 _newPrice)`: Set a new mint price
- `setBaseURI(string memory _newBaseURI)`: Set the base URI for NFT metadata
- `withdraw()`: Withdraw accumulated GRASS tokens

## Development

1. Compile the contract:
```bash
npx hardhat compile
```

2. Deploy to Lens Network Testnet:
```bash
npx hardhat run scripts/deploy.js --network lensTestnet
```

3. Verify the contract:
```bash
npx hardhat verify --network lensTestnet <contract_address>
```

## Network Configuration

The project is configured to work with Lens Network Testnet:
- Network Name: Lens Network Sepolia Testnet
- RPC URL: https://rpc.testnet.lens.dev
- Chain ID: 37111
- Currency Symbol: GRASS
- Block Explorer: https://block-explorer.testnet.lens.dev

## Security

- The contract uses OpenZeppelin's battle-tested ERC721 and Ownable implementations
- All sensitive functions are protected with `onlyOwner` modifier
- Withdrawal function uses the recommended call pattern

## License

MIT

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

For support, please open an issue in the repository or contact the development team.
