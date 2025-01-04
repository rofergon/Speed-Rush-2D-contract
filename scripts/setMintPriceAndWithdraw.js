const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");
const { ethers } = require("ethers");

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0x0a86889ab97C6911fBbFE7C0961b391a7CbAC0DC";

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
        if (owner.toLowerCase() !== wallet.address.toLowerCase()) {
            throw new Error("No eres el dueño del contrato. Solo el dueño puede realizar estas operaciones.");
        }

        // Obtener precio de minteo actual
        const precioActual = await carNFT.mintPrice();
        console.log("\n💰 Precio de minteo actual:", precioActual.toString(), "wei");
        console.log("💰 En GRASS:", ethers.formatEther(precioActual), "GRASS");

        // Establecer nuevo precio (0)
        console.log("\n🔧 Estableciendo nuevo precio de minteo a 0...");
        
        // Intentar primero con el valor 0 directo
        try {
            const tx1 = await carNFT.setMintPrice(0);
            console.log("Transacción enviada:", tx1.hash);
            await tx1.wait();
            console.log("✅ Primera transacción confirmada");
        } catch (error) {
            console.log("⚠️ Primer intento falló, intentando con BigNumber...");
            // Si falla, intentar con BigNumber
            const tx1 = await carNFT.setMintPrice(ethers.parseEther("0"));
            console.log("Transacción enviada:", tx1.hash);
            await tx1.wait();
            console.log("✅ Segunda transacción confirmada");
        }

        // Esperar un momento para que la red se actualice
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Verificar nuevo precio varias veces
        for (let i = 0; i < 3; i++) {
            const precioNuevo = await carNFT.mintPrice();
            console.log(`\n💰 Verificación ${i + 1} - Nuevo precio de minteo:`, precioNuevo.toString(), "wei");
            console.log(`💰 Verificación ${i + 1} - En GRASS:`, ethers.formatEther(precioNuevo), "GRASS");
            
            if (precioNuevo > 0) {
                console.log("⚠️ El precio no se estableció a 0, intentando de nuevo...");
                const tx2 = await carNFT.setMintPrice(ethers.parseEther("0"));
                console.log("Transacción enviada:", tx2.hash);
                await tx2.wait();
                await new Promise(resolve => setTimeout(resolve, 5000));
            } else {
                console.log("✅ Precio establecido correctamente a 0!");
                break;
            }
        }

        // Verificar precio final una última vez
        const precioFinal = await carNFT.mintPrice();
        console.log("\n🔍 Verificación final del precio de minteo:", precioFinal.toString(), "wei");
        console.log("🔍 En GRASS:", ethers.formatEther(precioFinal), "GRASS");

        if (precioFinal > 0) {
            throw new Error("No se pudo establecer el precio a 0 después de varios intentos");
        }

        // Obtener balance del contrato
        const balance = await provider.getBalance(CAR_NFT_ADDRESS);
        console.log("\n💰 Balance actual del contrato:", balance.toString(), "wei");
        console.log("💰 En GRASS:", ethers.formatEther(balance), "GRASS");

        if (balance > 0) {
            console.log("\n🔄 Retirando fondos...");
            const tx3 = await carNFT.withdrawFunds();
            await tx3.wait();
            console.log("✅ Fondos retirados!");

            const balanceFinal = await provider.getBalance(CAR_NFT_ADDRESS);
            console.log("\n💰 Balance final del contrato:", balanceFinal.toString(), "wei");
            console.log("💰 En GRASS:", ethers.formatEther(balanceFinal), "GRASS");
        } else {
            console.log("\n⚠️ El contrato no tiene fondos para retirar");
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