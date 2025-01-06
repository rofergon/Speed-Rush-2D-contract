const { Wallet, Provider, Contract, utils } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Contract addresses from new deployment
    const CAR_NFT_ADDRESS = "0x95dA1E4C0961295ED0D1F316474c1a3a6E868349";
    const CAR_PART_ADDRESS = "0xCA4E04724234D99122C01268a8a0cD722450c67E";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contracts
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");

    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, wallet);

    console.log("Testing part tracking with contracts:");
    console.log("CarNFT:", CAR_NFT_ADDRESS);
    console.log("CarPart:", CAR_PART_ADDRESS);
    console.log("Owner address:", wallet.address);

    try {
        // 1. First, let's mint a car with parts
        const carImageURI = "https://example.com/car1.jpg";
        const partsData = [
            {
                partType: 0, // ENGINE
                stat1: 8,    // speed
                stat2: 9,    // max speed
                stat3: 7,    // acceleration
                imageURI: "https://example.com/engine1.jpg"
            },
            {
                partType: 1, // TRANSMISSION
                stat1: 8,    // acceleration
                stat2: 7,    // speed
                stat3: 8,    // handling
                imageURI: "https://example.com/transmission1.jpg"
            },
            {
                partType: 2, // WHEELS
                stat1: 9,    // handling
                stat2: 7,    // drift
                stat3: 8,    // turn
                imageURI: "https://example.com/wheels1.jpg"
            }
        ];

        console.log("\n1. Minting new car with parts...");
        const mintPrice = await carNFT.mintPrice();
        const tx = await carNFT.mintCar(carImageURI, partsData, {
            value: mintPrice,
            gasLimit: 5000000
        });
        await tx.wait();
        console.log("Car minted successfully");

        // Get the car ID and part IDs
        const carId = await carNFT.getLastTokenId();
        const [partIds, , slotOccupied] = await carNFT.getCarComposition(carId);
        console.log("\nMinted car ID:", carId.toString());
        console.log("Part IDs:", partIds.map(id => id.toString()));

        // 2. Test the new tracking functions
        console.log("\n2. Testing tracking functions...");
        
        // Get all parts
        const allParts = await carPart.getOwnerParts(wallet.address);
        console.log("\nAll parts:", allParts.map(id => id.toString()));

        // Get parts by type
        console.log("\nParts by type:");
        const engines = await carPart.getOwnerPartsByType(wallet.address, 0);
        const transmissions = await carPart.getOwnerPartsByType(wallet.address, 1);
        const wheels = await carPart.getOwnerPartsByType(wallet.address, 2);
        console.log("Engines:", engines.map(id => id.toString()));
        console.log("Transmissions:", transmissions.map(id => id.toString()));
        console.log("Wheels:", wheels.map(id => id.toString()));

        // Get equipped/unequipped parts
        const equippedParts = await carPart.getOwnerEquippedParts(wallet.address);
        const unequippedParts = await carPart.getOwnerUnequippedParts(wallet.address);
        console.log("\nEquipped parts:", equippedParts.map(id => id.toString()));
        console.log("Unequipped parts:", unequippedParts.map(id => id.toString()));

        // 3. Get full details
        console.log("\n3. Getting full part details...");
        const details = await carPart.getOwnerPartsWithDetails(wallet.address);
        console.log("\nAll parts count:", details.allParts.length);
        console.log("Equipped parts count:", details.equippedParts.length);
        console.log("Unequipped parts count:", details.unequippedParts.length);

        // Print details of each part
        console.log("\nDetailed part information:");
        for (let i = 0; i < details.allParts.length; i++) {
            const part = details.allParts[i];
            console.log(`\nPart ${i}:`);
            console.log("Type:", ["ENGINE", "TRANSMISSION", "WHEELS"][part.partType]);
            console.log("Stats:", {
                stat1: part.stat1,
                stat2: part.stat2,
                stat3: part.stat3
            });
            console.log("Image URI:", part.imageURI);
        }

        // 4. Now let's unequip a part and see how the arrays change
        console.log("\n4. Testing unequip functionality...");
        console.log("Unequipping engine...");
        
        const unequipTx = await carNFT.unequipPart(carId, partIds[0]);
        await unequipTx.wait();
        console.log("Engine unequipped successfully");

        // Check the updated arrays
        console.log("\nChecking updated arrays after unequipping:");
        const newEquippedParts = await carPart.getOwnerEquippedParts(wallet.address);
        const newUnequippedParts = await carPart.getOwnerUnequippedParts(wallet.address);
        console.log("New equipped parts:", newEquippedParts.map(id => id.toString()));
        console.log("New unequipped parts:", newUnequippedParts.map(id => id.toString()));

        // 5. Finally, let's equip the part back
        console.log("\n5. Testing equip functionality...");
        console.log("Equipping engine back...");
        
        const equipTx = await carNFT.equipPart(carId, partIds[0], 0);
        await equipTx.wait();
        console.log("Engine equipped successfully");

        // Check the final arrays
        console.log("\nChecking final arrays after equipping:");
        const finalEquippedParts = await carPart.getOwnerEquippedParts(wallet.address);
        const finalUnequippedParts = await carPart.getOwnerUnequippedParts(wallet.address);
        console.log("Final equipped parts:", finalEquippedParts.map(id => id.toString()));
        console.log("Final unequipped parts:", finalUnequippedParts.map(id => id.toString()));

    } catch (error) {
        console.error("Error in test process:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 