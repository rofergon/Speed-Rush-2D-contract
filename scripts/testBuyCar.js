const { Wallet, Provider, Contract, utils } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0x0F6cdE471bBdA5a59d33e55C676ede09fC4aA16e";
    const CAR_PART_ADDRESS = "0x1B6E32D29800479d9F3fa56D75A835F5633147DC";
    const MARKETPLACE_ADDRESS = "0xfb10ab4Ef5AcF3d064857C20a4df79Fe3Ca0b8C9";

    // Initialize provider and wallet del comprador
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const buyerWallet = new Wallet(process.env.BUYER_PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, buyerWallet);

    // Load contracts
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const marketplaceArtifact = await deployer.loadArtifact("CarMarketplace");

    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, buyerWallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, buyerWallet);
    const marketplace = new Contract(MARKETPLACE_ADDRESS, marketplaceArtifact.abi, buyerWallet);

    console.log("Testing car purchase with contracts:");
    console.log("CarNFT:", CAR_NFT_ADDRESS);
    console.log("CarPart:", CAR_PART_ADDRESS);
    console.log("Marketplace:", MARKETPLACE_ADDRESS);
    console.log("Buyer address:", buyerWallet.address);

    try {
        // ID del carro que queremos comprar (el que acabamos de listar)
        const carId = 2;

        // Obtener detalles del listado
        const listing = await marketplace.carListings(carId);
        console.log("\nListing details:");
        console.log("Seller:", listing.seller);
        console.log("Price:", listing.price.toString(), "wei");
        console.log("Active:", listing.active);

        // Verificar que el listado esté activo
        if (!listing.active) {
            throw new Error("El listado no está activo");
        }

        // Obtener balance del comprador
        const balanceAntes = await provider.getBalance(buyerWallet.address);
        console.log("\nBuyer balance before purchase:", balanceAntes.toString(), "wei");

        // Verificar que tengamos suficientes fondos
        if (balanceAntes < listing.price) {
            throw new Error("Fondos insuficientes para la compra");
        }

        console.log("\nAttempting to buy car...");
        
        // Comprar el carro
        const buyTx = await marketplace.buyCar(carId, {
            value: listing.price,
            gasLimit: 5000000 // Gas limit fijo para asegurar
        });
        console.log("\nPurchase transaction sent:", buyTx.hash);
        
        // Esperar confirmación
        const receipt = await buyTx.wait();
        console.log("\nPurchase transaction confirmed!");

        // Verificar la nueva propiedad del carro
        const newOwner = await carNFT.ownerOf(carId);
        console.log("\nNew car owner:", newOwner);
        console.log("Expected owner (buyer):", buyerWallet.address);
        console.log("Transfer successful:", newOwner.toLowerCase() === buyerWallet.address.toLowerCase());

        // Obtener y mostrar las partes del carro
        const [partIds, , slotOccupied] = await carNFT.getCarComposition(carId);
        console.log("\nCar composition after purchase:");
        console.log("Part IDs:", partIds.map(id => id.toString()));
        console.log("Slots occupied:", slotOccupied);

        // Verificar la propiedad de las partes
        for (let i = 0; i < partIds.length; i++) {
            if (slotOccupied[i]) {
                const partOwner = await carPart.ownerOf(partIds[i]);
                console.log(`Part ${partIds[i]} owner:`, partOwner);
            }
        }

        // Obtener balance final
        const balanceDespues = await provider.getBalance(buyerWallet.address);
        console.log("\nBuyer balance after purchase:", balanceDespues.toString(), "wei");
        console.log("Total spent:", (balanceAntes - balanceDespues).toString(), "wei");

    } catch (error) {
        console.error("Error in purchase process:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 