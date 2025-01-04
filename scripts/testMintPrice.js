const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");
const { ethers } = require("ethers");

async function main() {
    // Dirección del contrato CarNFT recién desplegado
    const CAR_NFT_ADDRESS = "0xC4F1Ca718b8e00d487D95Bb71A97802ACdF8a14C";

    // Inicializar provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Cargar contrato
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    try {
        // Verificar precio actual
        const precio = await carNFT.mintPrice();
        console.log("\n💰 Precio actual de minteo:", precio.toString(), "wei");
        console.log("💰 En GRASS:", ethers.formatEther(precio), "GRASS");

        // Datos para mintear un carro de prueba
        const carImageURI = "https://example.com/car.jpg";
        const partsData = [
            {
                partType: 0, // ENGINE
                stat1: 5,
                stat2: 5,
                stat3: 5,
                imageURI: "https://example.com/engine.jpg"
            },
            {
                partType: 1, // TRANSMISSION
                stat1: 5,
                stat2: 5,
                stat3: 5,
                imageURI: "https://example.com/transmission.jpg"
            },
            {
                partType: 2, // WHEELS
                stat1: 5,
                stat2: 5,
                stat3: 5,
                imageURI: "https://example.com/wheels.jpg"
            }
        ];

        console.log("\n🚗 Intentando mintear un carro con el precio actual...");
        const tx = await carNFT.mintCar(carImageURI, partsData, { value: precio });
        console.log("Transacción enviada:", tx.hash);
        await tx.wait();
        console.log("✅ Carro minteado exitosamente!");

    } catch (error) {
        console.error("\n❌ Error:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 