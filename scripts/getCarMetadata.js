const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Deployed CarNFT contract address
    const CAR_NFT_ADDRESS = "0xe7c761359F21fB1b4afDb2F37A7b24C9aF5CF4A9";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contract
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    try {
        // Get last minted car ID
        const lastTokenId = await carNFT.getLastTokenId();
        console.log("\nLast minted car ID:", lastTokenId.toString());

        // Car ID to query (using the last minted)
        const carId = lastTokenId;
        console.log("\n=== Querying metadata for car ID:", carId.toString(), "===\n");

        // Get all car metadata
        const metadata = await carNFT.getFullCarMetadata(carId);
        
        // Show basic information
        console.log("Basic Information:");
        console.log("- Car ID:", metadata.carId.toString());
        console.log("- Owner:", metadata.owner);
        console.log("- Car Image URI:", metadata.carImageURI);
        console.log("- Condition:", metadata.condition.toString());

        // Show combined stats
        console.log("\nCombined Car Stats:");
        console.log("- Speed:", metadata.combinedStats.speed.toString());
        console.log("- Acceleration:", metadata.combinedStats.acceleration.toString());
        console.log("- Handling:", metadata.combinedStats.handling.toString());
        console.log("- Drift Factor:", metadata.combinedStats.driftFactor.toString());
        console.log("- Turn Factor:", metadata.combinedStats.turnFactor.toString());
        console.log("- Max Speed:", metadata.combinedStats.maxSpeed.toString());

        // Show detailed part information
        console.log("\nPart Details:");
        for (const part of metadata.parts) {
            const partType = ["Engine", "Transmission", "Wheels"][part.partType];
            console.log(`\n${partType} (ID: ${part.partId.toString()}):`);
            console.log("- Image URI:", part.imageURI);
            
            if (part.partType === 0) { // Engine
                console.log("Engine Stats:");
                console.log("- Base Speed:", part.stats.speed.toString());
                console.log("- Max Speed:", part.stats.maxSpeed.toString());
                console.log("- Acceleration:", part.stats.acceleration.toString());
            } else if (part.partType === 1) { // Transmission
                console.log("Transmission Stats:");
                console.log("- Acceleration:", part.stats.transmissionAcceleration.toString());
                console.log("- Speed:", part.stats.transmissionSpeed.toString());
                console.log("- Handling:", part.stats.transmissionHandling.toString());
            } else { // Wheels
                console.log("Wheel Stats:");
                console.log("- Handling:", part.stats.handling.toString());
                console.log("- Drift Factor:", part.stats.driftFactor.toString());
                console.log("- Turn Factor:", part.stats.turnFactor.toString());
            }
        }

        // Show complete metadata in JSON format
        console.log("\nComplete metadata in JSON format:");
        const replacer = (key, value) =>
            typeof value === 'bigint'
                ? value.toString()
                : value;
        console.log(JSON.stringify(metadata, replacer, 2));

    } catch (error) {
        console.error("Error querying metadata:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 