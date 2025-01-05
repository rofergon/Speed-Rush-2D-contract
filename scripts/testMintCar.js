const { Wallet, Provider, Contract, utils } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0x0F6cdE471bBdA5a59d33e55C676ede09fC4aA16e";
    const CAR_PART_ADDRESS = "0x1B6E32D29800479d9F3fa56D75A835F5633147DC";
    const MARKETPLACE_ADDRESS = "0xfb10ab4Ef5AcF3d064857C20a4df79Fe3Ca0b8C9";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contracts
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const marketplaceArtifact = await deployer.loadArtifact("CarMarketplace");

    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, wallet);
    const marketplace = new Contract(MARKETPLACE_ADDRESS, marketplaceArtifact.abi, wallet);

    console.log("Testing mintCar and marketplace listing with contracts:");
    console.log("CarNFT:", CAR_NFT_ADDRESS);
    console.log("CarPart:", CAR_PART_ADDRESS);
    console.log("Marketplace:", MARKETPLACE_ADDRESS);

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
        console.log("\nPart Details:", partsData);

        // Mint the car
        const tx = await carNFT.mintCar(carImageURI, partsData, {
            value: mintPrice,
            gasLimit: 5000000 // Gas limit fijo para asegurar
        });
        console.log("\nMint transaction sent:", tx.hash);
        
        // Wait for confirmation
        const receipt = await tx.wait();
        console.log("\nMint transaction confirmed!");

        // Get the last minted car ID
        const carId = await carNFT.getLastTokenId();
        console.log("\nMinted car ID:", carId.toString());

        // Get car composition to verify parts
        const [partIds, , slotOccupied] = await carNFT.getCarComposition(carId);
        console.log("\nCar composition:");
        console.log("Part IDs:", partIds.map(id => id.toString()));
        console.log("Slots occupied:", slotOccupied);

        console.log("\nApproving NFTs for marketplace...");
        
        // Aprobar el carro para el marketplace
        const approveTxCar = await carNFT.approve(MARKETPLACE_ADDRESS, carId);
        await approveTxCar.wait();
        console.log("Car approved for marketplace");

        // Aprobar todas las partes para el marketplace
        for (let i = 0; i < partIds.length; i++) {
            if (slotOccupied[i]) {
                const approveTxPart = await carPart.approve(MARKETPLACE_ADDRESS, partIds[i]);
                await approveTxPart.wait();
                console.log(`Part ${partIds[i]} approved for marketplace`);
            }
        }

        // Preparar el listado en el marketplace
        const listingPrice = BigInt("100000000000000000"); // 0.1 ETH en wei
        const includeSlots = [true, true, true]; // Incluir todas las partes

        console.log("\nListing car in marketplace...");
        console.log("Listing price:", listingPrice.toString(), "wei");
        console.log("Including all slots:", includeSlots);

        // Verificar aprobaciones antes de listar
        const approvalStatus = await marketplace.getListingApprovalStatus(carId, includeSlots);
        console.log("\nApproval status:");
        console.log("Car approved:", approvalStatus[0]);
        console.log("Parts approved:", approvalStatus[1]);

        // Listar en el marketplace
        const listTx = await marketplace.listCar(carId, listingPrice, includeSlots, {
            gasLimit: 5000000
        });
        console.log("\nListing transaction sent:", listTx.hash);
        
        const listReceipt = await listTx.wait();
        console.log("\nListing transaction confirmed!");

        // Verificar el listado
        const listing = await marketplace.carListings(carId);
        console.log("\nListing details:");
        console.log("Seller:", listing.seller);
        console.log("Price:", listing.price.toString(), "wei");
        console.log("Active:", listing.active);
        console.log("Part slots:", listing.partSlots);

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