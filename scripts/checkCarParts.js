const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Deployed contract addresses
    const CAR_NFT_ADDRESS = "0xabD2368daE3b292FE732C6D2760a44FbE33EaA13";
    const CAR_PART_ADDRESS = "0x48AA0974C1E4DAc4c68BFf740Cc4411D40cfe0c1";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contracts
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, wallet);

    // Car ID to verify
    const carId = 1;

    try {
        console.log(`\nVerifying car parts for ID: ${carId}`);
        
        // Get car composition (array of part IDs)
        const [partIds, carImageURI] = await carNFT.getCarComposition(carId);
        console.log(`\nCar URI: ${carImageURI}`);
        console.log(`Number of parts: ${partIds.length}`);

        // Verify each part
        for (let i = 0; i < partIds.length; i++) {
            const partId = partIds[i];
            console.log(`\nPart #${i + 1} (ID: ${partId}):`);
            
            // Verify part owner
            const owner = await carPart.ownerOf(partId);
            console.log(`Owner: ${owner}`);

            // Get part stats
            const stats = await carPart.getPartStats(partId);
            console.log("Part Type:", getPartTypeName(stats.partType));
            console.log("Base Stats:");
            console.log("- Speed:", stats.baseSpeed.toString());
            console.log("- Acceleration:", stats.baseAcceleration.toString());
            console.log("- Handling:", stats.baseHandling.toString());
            console.log("- Drift Factor:", stats.baseDriftFactor.toString());
            console.log("- Turn Factor:", stats.baseTurnFactor.toString());
            console.log("- Max Speed:", stats.baseMaxSpeed.toString());
            console.log("Image URI:", stats.imageURI);
        }

    } catch (error) {
        console.error("Error verifying car parts:", error);
        throw error;
    }
}

function getPartTypeName(partType) {
    const types = ["Engine", "Transmission", "Wheels"];
    return types[partType] || "Unknown";
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 