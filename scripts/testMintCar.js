const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Recently deployed CarNFT contract address
    const CAR_NFT_ADDRESS = "0x33Cf5229318c39d7F754ccbB8FAf61c6470e85dc";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contract
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    console.log("Testing mintCar with contract at:", CAR_NFT_ADDRESS);

    // Test data for the car
    const carImageURI = "https://example.com/car2.jpg";
    
    // Array of car parts with their main stats
    const partsData = [
        {
            partType: 0, // ENGINE (speed, max speed, acceleration)
            stat1: 9,    // speed
            stat2: 10,   // max speed
            stat3: 8,    // acceleration
            imageURI: "https://example.com/engine2.jpg"
        },
        {
            partType: 1, // TRANSMISSION (acceleration, speed, handling)
            stat1: 9,    // acceleration
            stat2: 8,    // speed
            stat3: 9,    // handling
            imageURI: "https://example.com/transmission2.jpg"
        },
        {
            partType: 2, // WHEELS (handling, drift, turn)
            stat1: 10,   // handling
            stat2: 8,    // drift
            stat3: 9,    // turn
            imageURI: "https://example.com/wheels2.jpg"
        }
    ];

    try {
        // Get current mint price
        const mintPrice = await carNFT.mintPrice();
        console.log("\nMint price:", mintPrice.toString(), "wei");

        // Get wallet balance before minting
        const balanceAntes = await provider.getBalance(wallet.address);
        console.log("Wallet balance before minting:", balanceAntes.toString(), "wei");

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

        // Estimate required gas
        const gasEstimado = await carNFT.mintCar.estimateGas(carImageURI, partsData, {
            value: mintPrice
        });
        console.log("\nEstimated gas:", gasEstimado.toString());

        // Get gas price
        const gasPrice = await provider.getGasPrice();
        console.log("Gas price:", gasPrice.toString(), "wei");

        // Calculate total cost (mint price + gas)
        const costoGas = BigInt(gasEstimado) * BigInt(gasPrice);
        console.log("Gas cost:", costoGas.toString(), "wei");
        console.log("Total cost (mint + gas):", (BigInt(mintPrice) + costoGas).toString(), "wei");

        // Mint the car sending required value
        const tx = await carNFT.mintCar(carImageURI, partsData, {
            value: mintPrice,
            gasLimit: BigInt(Math.floor(Number(gasEstimado) * 1.1)) // Add 10% margin
        });
        console.log("\nTransaction sent:", tx.hash);
        
        // Wait for confirmation and get receipt
        const receipt = await tx.wait();
        console.log("\nTransaction confirmed!");
        console.log("Gas used:", receipt.gasUsed?.toString() || "N/A");
        console.log("Effective gas price:", receipt.effectiveGasPrice?.toString() || "N/A");
        
        const costoGasReal = receipt.gasUsed && receipt.effectiveGasPrice ? 
            (BigInt(receipt.gasUsed) * BigInt(receipt.effectiveGasPrice)).toString() : 
            "Not available";
        console.log("Total gas cost:", costoGasReal, "wei");

        // Get balance after minting
        const balanceDespues = await provider.getBalance(wallet.address);
        console.log("\nWallet balance after minting:", balanceDespues.toString(), "wei");
        console.log("Total operation cost:", (balanceAntes - balanceDespues).toString(), "wei");

        // Get the last minted car ID
        const carId = 2; // This will be the second car
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