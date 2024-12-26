const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Dirección del contrato CarNFT desplegado
    const CAR_NFT_ADDRESS = "0xEd0fA4fFDB1B33B6D6c6611B77F6806DB50b21aE";

    // Inicializar el provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Cargar el contrato
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    console.log("Probando mintCar con el contrato en:", CAR_NFT_ADDRESS);

    // Datos de prueba para el carro
    const carImageURI = "https://example.com/car1.jpg";
    
    // Crear array de partes del carro con sus estadísticas principales
    const partsData = [
        {
            partType: 0, // ENGINE (velocidad, velocidad máxima, aceleración)
            stat1: 8,    // velocidad
            stat2: 9,    // velocidad máxima
            stat3: 7,    // aceleración
            imageURI: "https://example.com/engine1.jpg"
        },
        {
            partType: 1, // TRANSMISSION (aceleración, velocidad, manejo)
            stat1: 8,    // aceleración
            stat2: 7,    // velocidad
            stat3: 8,    // manejo
            imageURI: "https://example.com/transmission1.jpg"
        },
        {
            partType: 2, // WHEELS (manejo, derrape, giro)
            stat1: 9,    // manejo
            stat2: 7,    // derrape
            stat3: 8,    // giro
            imageURI: "https://example.com/wheels1.jpg"
        }
    ];

    try {
        console.log("\nIntentando mintear un nuevo carro...");
        console.log("\nDetalles de las partes:");
        console.log("\nMotor:");
        console.log("- Velocidad:", partsData[0].stat1);
        console.log("- Velocidad Máxima:", partsData[0].stat2);
        console.log("- Aceleración:", partsData[0].stat3);

        console.log("\nTransmisión:");
        console.log("- Aceleración:", partsData[1].stat1);
        console.log("- Velocidad:", partsData[1].stat2);
        console.log("- Manejo:", partsData[1].stat3);

        console.log("\nRuedas:");
        console.log("- Manejo:", partsData[2].stat1);
        console.log("- Derrape:", partsData[2].stat2);
        console.log("- Giro:", partsData[2].stat3);

        const tx = await carNFT.mintCar(carImageURI, partsData);
        console.log("\nTransacción enviada:", tx.hash);
        
        await tx.wait();
        console.log("¡Carro minteado exitosamente!");

        // Obtener el ID del carro (será 1 ya que es el primero)
        const carId = 1;
        console.log("\nObteniendo estadísticas del carro ID:", carId);

        // Obtener y mostrar las estadísticas del carro
        const stats = await carNFT.getCompactCarStats(carId);
        console.log("\nEstadísticas finales del carro (combinando todas las partes):");
        console.log("Velocidad:", stats.speed.toString());
        console.log("Aceleración:", stats.acceleration.toString());
        console.log("Manejo:", stats.handling.toString());
        console.log("Factor de Derrape:", stats.driftFactor.toString());
        console.log("Factor de Giro:", stats.turnFactor.toString());
        console.log("Velocidad Máxima:", stats.maxSpeed.toString());
        console.log("Condición:", stats.condition.toString());
        console.log("URI de la imagen:", stats.imageURI);

    } catch (error) {
        console.error("Error al mintear el carro:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 