# Speed Rush 2D - NFT System

An NFT system for the Speed Rush 2D game deployed on the Lens Network Testnet that allows users to create and customize cars using NFT parts.

## Features

- Car parts as NFTs (Engine, Transmission, Wheels)
- Dynamic stats system where each part contributes to multiple attributes
- Repair workshop to maintain cars in good condition
- Car degradation system during races
- Race leaderboard system

## Deployed Contracts

The contracts are deployed on Lens Network Testnet at the following addresses:
- CarPart: `0x4bF1Cf69D3Cdc11dD7cBe0b1942Ce183f27FE402`
- CarNFT: `0xEd0fA4fFDB1B33B6D6c6611B77F6806DB50b21aE`
- CarWorkshop: `0x92cb777a96BE6f617959c8220388e4A046DA8669`
- RaceLeaderboard: `0x9caEBCA084c2072904083008a0b3AE99068571b6`

## Stats System

### Engine
- stat1: Speed
- stat2: Max Speed
- stat3: Acceleration

### Transmission
- stat1: Acceleration
- stat2: Speed
- stat3: Handling

### Wheels
- stat1: Handling
- stat2: Drift
- stat3: Turn

Each final car statistic is affected by at least two attributes from different parts:
- Speed: Engine (stat1) and Transmission (stat2)
- Max Speed: Engine (stat2) and Transmission (stat1)
- Acceleration: Engine (stat3) and Transmission (stat1)
- Handling: Transmission (stat3) and Wheels (stat1)
- Drift: Wheels (stat2) and Transmission (stat3)
- Turn: Wheels (stat3) and Wheels (stat2)

## Prerequisites

- Node.js >= v16
- npm or yarn
- A wallet with funds on Lens Network Testnet

## Installation

1. Clone the repository:
```bash
git clone <repo-url>
cd Speed-Rush-2D
```

2. Install dependencies:
```bash
npm install
```

3. Create a `.env` file in the root directory and add your private key:
```env
PRIVATE_KEY=your_private_key_here
```

## Management Scripts

### Mint a Car
```bash
npx hardhat run scripts/testMintCar.js --network lensTestnet
```
Creates a new car with custom parts.

### Check Car Parts
```bash
npx hardhat run scripts/checkCarParts.js --network lensTestnet
```
Shows the details of a specific car's parts.

### Repair a Car
```bash
npx hardhat run scripts/repairCar.js --network lensTestnet
```
Repairs a damaged car at the workshop.

## Main Functions

### CarNFT
- `mintCar(string memory carImageURI, PartData[] calldata partsData)`: Mints a new car with its parts
- `replacePart(uint256 carId, uint256 oldPartId, uint256 newPartId)`: Replaces a car part
- `getCompactCarStats(uint256 carId)`: Gets the stats of a car

### CarPart
- `mint(address to, PartType partType, uint8 stat1, uint8 stat2, uint8 stat3, string memory imageURI)`: Mints a new part
- `getPartStats(uint256 partId)`: Gets the stats of a part

### CarWorkshop
- `repairCar(uint256 carId)`: Repairs a damaged car
- `setRepairPrice(uint256 _newPrice)`: Sets the repair price

## Development

1. Compile the contracts:
```bash
npm run compile
```

2. Deploy to Lens Network Testnet:
```bash
npm run deploy
```

## Network Configuration

The project is configured to work with Lens Network Testnet:
- Network Name: Lens Network Sepolia Testnet
- RPC URL: https://rpc.testnet.lens.dev
- Chain ID: 37111
- Block Explorer: https://block-explorer.testnet.lens.dev

## Security

- Contracts use battle-tested OpenZeppelin implementations
- All sensitive functions are protected with appropriate modifiers
- Permission system between contracts for secure operations

## License

MIT

## Support

For support, please open an issue in the repository or contact the development team.
