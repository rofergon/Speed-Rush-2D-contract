const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");
const { ethers } = require("ethers");

async function main() {
    // Dirección del contrato CarNFT recién desplegado
    const CAR_NFT_ADDRESS = "0xC4F1Ca718b8e00d487D95Bb71A97802ACdF8a14C";
    const NEW_PRICE = "100000000000000000"; // 0.1 GRASS en wei

    // Inicializar provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Cargar contrato
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    try {
        // Verificar que somos el owner
        const owner = await carNFT.owner();
        console.log("\n👤 Owner del contrato:", owner);
        console.log("👤 Nuestra dirección:", wallet.address);
        
        if (owner.toLowerCase() !== wallet.address.toLowerCase()) {
            throw new Error("No eres el dueño del contrato");
        }

        // Obtener precio actual
        const precioActual = await carNFT.mintPrice();
        console.log("\n💰 Precio actual:", precioActual.toString(), "wei");
        console.log("💰 En GRASS:", ethers.formatEther(precioActual), "GRASS");

        // Intentar diferentes formatos para el precio
        console.log("\n🔄 Intentando establecer precio con diferentes formatos...");

        // Intento 1: Usando BigNumber
        console.log("\n📝 Intento 1: Usando BigNumber");
        const tx1 = await carNFT.setMintPrice(ethers.getBigInt(NEW_PRICE));
        console.log("Transacción enviada:", tx1.hash);
        await tx1.wait();
        
        // Verificar después del primer intento
        let precioVerificacion1 = await carNFT.mintPrice();
        console.log("Precio después del intento 1:", precioVerificacion1.toString(), "wei");

        // Esperar un poco
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Intento 2: Usando string directo
        console.log("\n📝 Intento 2: Usando string directo");
        const tx2 = await carNFT.setMintPrice(NEW_PRICE);
        console.log("Transacción enviada:", tx2.hash);
        await tx2.wait();

        // Verificar después del segundo intento
        let precioVerificacion2 = await carNFT.mintPrice();
        console.log("Precio después del intento 2:", precioVerificacion2.toString(), "wei");

        // Esperar un poco más
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Verificación final
        const precioFinal = await carNFT.mintPrice();
        console.log("\n🔍 Verificación final:");
        console.log("💰 Precio final:", precioFinal.toString(), "wei");
        console.log("💰 En GRASS:", ethers.formatEther(precioFinal), "GRASS");

        if (precioFinal.toString() !== NEW_PRICE) {
            console.log("⚠️ ADVERTENCIA: El precio final no coincide con el precio deseado");
        } else {
            console.log("✅ Precio actualizado correctamente");
        }

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