# Speed Rush 2D - NFT System

An NFT system for the Speed Rush 2D game deployed on the Lens Network Testnet that allows users to create and customize cars using NFT parts.

## Features

- Cars and parts as NFTs (Engine, Transmission, Core)
- Dynamic stats system where each part contributes to multiple attributes
- Advanced part tracking system for equipped and unequipped parts
- Detailed metadata system for cars and parts
- Configurable car minting price
- Repair workshop to maintain cars in good condition
- Car degradation system during races
- Race leaderboard system
- Marketplace for buying and selling cars and parts

## Deployed Contracts

The contracts are deployed on Lens Network Testnet at the following addresses:
- CarPart: `0xCA4E04724234D99122C01268a8a0cD722450c67E`
- CarNFT: `0x95dA1E4C0961295ED0D1F316474c1a3a6E868349`
- CarWorkshop: `0xf2BBd1BEDB23915Ab77cd69265eaD8D442C10980`
- RaceLeaderboard: `0xD75aA67d5Bb1f8aCA77b4d14da847C28CdEE9a0D`
- CarMarketplace: `0x73b378F1368D9aC0394AB1C8aB85EFB3e4216DcC`

## Stats System

### Engine
- stat1: Speed
- stat2: Max Speed
- stat3: Acceleration

### Transmission
- stat1: Acceleration
- stat2: Speed
- stat3: Handling

### Core and Alternative Skin
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

## Marketplace

The system includes a complete marketplace that allows:
- List complete cars with or without their parts
- List individual parts
- Buy cars and parts
- Cancel listings
- Configurable marketplace fee (2.5% default)

## Condition and Repair System

- Cars start with 100% condition
- Condition degrades by 5% after each race
- Cars can be repaired at the workshop for a fee
- Condition directly affects car performance

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

3. Create a `.env` file in the root directory and add your private keys:
```env
PRIVATE_KEY=your_private_key_here
BUYER_PRIVATE_KEY=buyer_private_key_here
```

## Management Test Scripts

### Mint a Car
```bash
npx hardhat run scripts/testMintCar.js --network lensTestnet
```
Creates a new car with custom parts.

### Buy a Car
```bash
npx hardhat run scripts/testBuyCar.js --network lensTestnet
```
Buys a car listed in the marketplace.

### Swap Engines
```bash
npx hardhat run scripts/testSwapEngines.js --network lensTestnet
```
Swaps engines between two cars.

## Main Functions

### CarNFT
- `mintCar(string memory carImageURI, PartData[] calldata partsData)`: Mints a new car with its parts
- `replacePart(uint256 carId, uint256 oldPartId, uint256 newPartId)`: Replaces a car part
- `unequipPart(uint256 carId, uint256 partId)`: Unequips a part
- `equipPart(uint256 carId, uint256 partId, uint256 slotIndex)`: Equips a part
- `getCompactCarStats(uint256 carId)`: Gets the basic stats of a car
- `getFullCarMetadata(uint256 carId)`: Gets detailed metadata including all parts and stats
- `setMintPrice(uint256 _newPrice)`: Sets the minting price for new cars
- `withdrawFunds()`: Allows owner to withdraw accumulated mint fees
- `getLastTokenId()`: Gets the last minted car token ID

### CarPart
- `mint(address to, PartType partType, uint8 stat1, uint8 stat2, uint8 stat3, string memory imageURI)`: Mints a new part
- `getPartStats(uint256 partId)`: Gets the stats of a part
- `getOwnerParts(address owner)`: Gets all parts owned by an address
- `getOwnerPartsByType(address owner, PartType partType)`: Gets all parts of a specific type owned by an address
- `getOwnerEquippedParts(address owner)`: Gets all equipped parts owned by an address
- `getOwnerUnequippedParts(address owner)`: Gets all unequipped parts owned by an address
- `getOwnerPartsWithDetails(address owner)`: Gets detailed information about all parts owned by an address

### CarMarketplace
- `listCar(uint256 carId, uint256 price, bool[3] memory includeSlots)`: Lists a car for sale
- `listPart(uint256 partId, uint256 price)`: Lists a part for sale
- `buyCar(uint256 carId)`: Buys a listed car
- `buyPart(uint256 partId)`: Buys a listed part

### CarWorkshop
- `repairCar(uint256 carId)`: Repairs a damaged car
- `setRepairPrice(uint256 _newPrice)`: Sets the repair price

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
- Reentrancy protection in the marketplace
- Ownership and approval checks for all NFT operations

## License

MIT

## Support

For support, please open an issue in the repository or contact the development team.
