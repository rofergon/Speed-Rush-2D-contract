const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Deployed CarNFT contract address
    const CAR_NFT_ADDRESS = "0xEd0fA4fFDB1B33B6D6c6611B77F6806DB50b21aE";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contract
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    console.log("Testing mintCar with contract at:", CAR_NFT_ADDRESS);

    // Test data for the car
    const carImageURI = "https://example.com/car1.jpg";
    
    // Create array of car parts with their main stats
    const partsData = [
        {
            partType: 0, // ENGINE (speed, max speed, acceleration)
            stat1: 8,    // speed
            stat2: 9,    // max speed
            stat3: 7,    // acceleration
            imageURI: "https://example.com/engine1.jpg"
        },
        {
            partType: 1, // TRANSMISSION (acceleration, speed, handling)
            stat1: 8,    // acceleration
            stat2: 7,    // speed
            stat3: 8,    // handling
            imageURI: "https://example.com/transmission1.jpg"
        },
        {
            partType: 2, // WHEELS (handling, drift, turn)
            stat1: 9,    // handling
            stat2: 7,    // drift
            stat3: 8,    // turn
            imageURI: "https://example.com/wheels1.jpg"
        }
    ];

    try {
        console.log("\nAttempting to mint a new car...");
        console.log("\nPart Details:");
        console.log("\nEngine:");
        console.log("- Speed:", partsData[0].stat1);
        console.log("- Max Speed:", partsData[0].stat2);
        console.log("- Acceleration:", partsData[0].stat3);

        console.log("\nTransmission:");
        console.log("- Acceleration:", partsData[1].stat1);
        console.log("- Speed:", partsData[1].stat2);
        console.log("- Handling:", partsData[1].stat3);

        console.log("\nWheels:");
        console.log("- Handling:", partsData[2].stat1);
        console.log("- Drift:", partsData[2].stat2);
        console.log("- Turn:", partsData[2].stat3);

        const tx = await carNFT.mintCar(carImageURI, partsData);
        console.log("\nTransaction sent:", tx.hash);
        
        await tx.wait();
        console.log("Car minted successfully!");

        // Get car ID (will be 1 since it's the first one)
        const carId = 1;
        console.log("\nGetting stats for car ID:", carId);

        // Get and display car stats
        const stats = await carNFT.getCompactCarStats(carId);
        console.log("\nFinal car stats (combining all parts):");
        console.log("Speed:", stats.speed.toString());
        console.log("Acceleration:", stats.acceleration.toString());
        console.log("Handling:", stats.handling.toString());
        console.log("Drift Factor:", stats.driftFactor.toString());
        console.log("Turn Factor:", stats.turnFactor.toString());
        console.log("Max Speed:", stats.maxSpeed.toString());
        console.log("Condition:", stats.condition.toString());
        console.log("Image URI:", stats.imageURI);

    } catch (error) {
        console.error("Error minting car:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 