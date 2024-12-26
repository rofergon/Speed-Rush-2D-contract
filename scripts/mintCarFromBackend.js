const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");
const axios = require("axios");

async function generateCarFromBackend() {
    console.log("Requesting car generation from backend...");
    try {
        const response = await axios.post(
            'https://speed-rush-2d-backend-production.up.railway.app/api/cars/generate',
            {
                prompt: "string",
                style: "cartoon",
                creatorType: "standard",
                transmissionType: "manual",
                wheelsType: "sport"
            }
        );
        console.log("Car generated successfully!");
        return response.data;
    } catch (error) {
        console.error("Error generating car from backend:", error.message);
        throw error;
    }
}

async function main() {
    // Deployed CarNFT contract address
    const CAR_NFT_ADDRESS = "0xdAA9A8c4876554b4679Dd52E76d7371Fa5F4F5a5";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contract
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    console.log("Starting car minting process...");

    try {
        // Generate car from backend
        console.log("\nGenerating car from backend (this may take a minute or more)...");
        const carData = await generateCarFromBackend();
        console.log("\nCar data received from backend!");

        // Get minting price
        const mintPrice = await carNFT.mintPrice();
        console.log("\nMint price:", mintPrice.toString(), "wei");

        // Get balance before minting
        const balanceBefore = await provider.getBalance(wallet.address);
        console.log("Wallet balance before minting:", balanceBefore.toString(), "wei");

        // Show car part details
        console.log("\nCar Part Details:");
        console.log("\nEngine:");
        console.log("- Speed:", carData.parts[0].stat1);
        console.log("- Max Speed:", carData.parts[0].stat2);
        console.log("- Acceleration:", carData.parts[0].stat3);

        console.log("\nTransmission:");
        console.log("- Acceleration:", carData.parts[1].stat1);
        console.log("- Speed:", carData.parts[1].stat2);
        console.log("- Handling:", carData.parts[1].stat3);

        console.log("\nWheels:");
        console.log("- Handling:", carData.parts[2].stat1);
        console.log("- Drift Factor:", carData.parts[2].stat2);
        console.log("- Turn Factor:", carData.parts[2].stat3);

        // Estimate required gas
        const estimatedGas = await carNFT.mintCar.estimateGas(
            carData.carImageURI,
            carData.parts,
            { value: mintPrice }
        );
        console.log("\nEstimated gas:", estimatedGas.toString());

        // Get gas price
        const gasPrice = await provider.getGasPrice();
        console.log("Gas price:", gasPrice.toString(), "wei");

        // Calculate total cost
        const gasCost = BigInt(estimatedGas) * BigInt(gasPrice);
        console.log("Gas cost:", gasCost.toString(), "wei");
        console.log("Total cost (mint + gas):", (BigInt(mintPrice) + gasCost).toString(), "wei");

        // Mint the car
        console.log("\nInitiating minting transaction...");
        const tx = await carNFT.mintCar(
            carData.carImageURI,
            carData.parts,
            {
                value: mintPrice,
                gasLimit: BigInt(Math.floor(Number(estimatedGas) * 1.1)) // 10% margin
            }
        );
        console.log("Transaction sent:", tx.hash);
        
        // Wait for confirmation
        console.log("\nWaiting for transaction confirmation...");
        const receipt = await tx.wait();
        console.log("Transaction confirmed!");
        console.log("Gas used:", receipt.gasUsed?.toString() || "N/A");
        console.log("Effective gas price:", receipt.effectiveGasPrice?.toString() || "N/A");

        // Calculate actual gas cost
        const actualGasCost = receipt.gasUsed && receipt.effectiveGasPrice ? 
            (BigInt(receipt.gasUsed) * BigInt(receipt.effectiveGasPrice)).toString() : 
            "Not available";
        console.log("Total gas cost:", actualGasCost, "wei");

        // Get balance after minting
        const balanceAfter = await provider.getBalance(wallet.address);
        console.log("\nWallet balance after minting:", balanceAfter.toString(), "wei");
        console.log("Total operation cost:", (balanceBefore - balanceAfter).toString(), "wei");

        // Get last minted car ID
        const lastTokenId = await carNFT.getLastTokenId();
        console.log("\nMinted car ID:", lastTokenId.toString());

        // Get complete car metadata
        const metadata = await carNFT.getFullCarMetadata(lastTokenId);
        console.log("\nMinted car metadata:");
        
        // Function to convert BigInt to string in objects
        const replacer = (key, value) =>
            typeof value === 'bigint'
                ? value.toString()
                : value;
        
        console.log(JSON.stringify(metadata, replacer, 2));

    } catch (error) {
        console.error("Error in process:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 