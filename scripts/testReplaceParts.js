const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

// Enum para los tipos de partes
const PartType = {
    ENGINE: 0,
    TRANSMISSION: 1,
    WHEELS: 2
};

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0x33Cf5229318c39d7F754ccbB8FAf61c6470e85dc";
    const CAR_PART_ADDRESS = "0x88E398A65727ca1743D3794B83e8405074FB92c3";

    // Inicializar provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Cargar contratos
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, wallet);

    try {
        // ID del carro al que queremos cambiarle partes
        const carId = 2;
        
        console.log("\nObteniendo composición actual del carro...");
        const carComposition = await carNFT.getCarComposition(carId);
        console.log("Partes actuales:", carComposition.partIds.map(id => id.toString()));

        // Obtener estadísticas actuales del carro
        console.log("\nEstadísticas actuales del carro:");
        const statsAntes = await carNFT.getCompactCarStats(carId);
        console.log("Velocidad:", statsAntes.speed.toString());
        console.log("Aceleración:", statsAntes.acceleration.toString());
        console.log("Manejo:", statsAntes.handling.toString());
        console.log("Factor de Derrape:", statsAntes.driftFactor.toString());
        console.log("Factor de Giro:", statsAntes.turnFactor.toString());
        console.log("Velocidad Máxima:", statsAntes.maxSpeed.toString());

        // Vamos a reemplazar el motor (primera parte)
        const oldEngineId = carComposition.partIds[0];
        
        // Verificar el tipo de la parte vieja
        console.log("\nVerificando tipo de parte actual...");
        const oldPartStats = await carPart.getPartStats(oldEngineId);
        console.log("Tipo de parte actual:", oldPartStats.partType);
        console.log("ID de parte actual:", oldEngineId.toString());

        // Buscar una parte del mismo tipo que tengamos
        let newPartId = null;
        let currentId = 1;
        const maxTries = 10; // Límite de búsqueda

        console.log("\nBuscando una parte compatible...");
        while (currentId <= maxTries && newPartId === null) {
            try {
                // Saltar la parte actual
                if (currentId.toString() === oldEngineId.toString()) {
                    currentId++;
                    continue;
                }

                const owner = await carPart.ownerOf(currentId);
                if (owner === wallet.address) {
                    const stats = await carPart.getPartStats(currentId);
                    if (stats.partType === oldPartStats.partType) {
                        newPartId = currentId;
                        console.log(`Parte compatible encontrada! ID: ${newPartId}`);
                        break;
                    }
                }
            } catch (error) {
                // Ignorar errores (probablemente el token no existe)
            }
            currentId++;
        }

        if (!newPartId) {
            throw new Error(`No se encontró una parte compatible del tipo ${oldPartStats.partType}`);
        }

        // Verificar los tipos de las partes
        console.log("\nVerificando compatibilidad...");
        const newPartStats = await carPart.getPartStats(newPartId);
        console.log("Tipo de parte vieja:", oldPartStats.partType);
        console.log("Tipo de parte nueva:", newPartStats.partType);

        console.log("\nReemplazando parte...");
        console.log("ID de parte vieja:", oldEngineId.toString());
        console.log("ID de parte nueva:", newPartId.toString());

        // Verificar que somos dueños de la nueva parte
        const newPartOwner = await carPart.ownerOf(newPartId);
        console.log("Dueño de la nueva parte:", newPartOwner);
        console.log("Nuestra dirección:", wallet.address);

        // Realizar el reemplazo
        const tx = await carNFT.replacePart(carId, oldEngineId, newPartId);
        console.log("\nTransacción enviada:", tx.hash);
        
        // Esperar confirmación
        const receipt = await tx.wait();
        console.log("Transacción confirmada!");
        
        // Obtener nuevas estadísticas del carro
        console.log("\nNuevas estadísticas del carro:");
        const statsDespues = await carNFT.getCompactCarStats(carId);
        console.log("Velocidad:", statsDespues.speed.toString());
        console.log("Aceleración:", statsDespues.acceleration.toString());
        console.log("Manejo:", statsDespues.handling.toString());
        console.log("Factor de Derrape:", statsDespues.driftFactor.toString());
        console.log("Factor de Giro:", statsDespues.turnFactor.toString());
        console.log("Velocidad Máxima:", statsDespues.maxSpeed.toString());

        // Mostrar los cambios en las estadísticas
        console.log("\nCambios en las estadísticas:");
        console.log("Velocidad:", parseInt(statsDespues.speed) - parseInt(statsAntes.speed));
        console.log("Aceleración:", parseInt(statsDespues.acceleration) - parseInt(statsAntes.acceleration));
        console.log("Manejo:", parseInt(statsDespues.handling) - parseInt(statsAntes.handling));
        console.log("Factor de Derrape:", parseInt(statsDespues.driftFactor) - parseInt(statsAntes.driftFactor));
        console.log("Factor de Giro:", parseInt(statsDespues.turnFactor) - parseInt(statsAntes.turnFactor));
        console.log("Velocidad Máxima:", parseInt(statsDespues.maxSpeed) - parseInt(statsAntes.maxSpeed));

    } catch (error) {
        console.error("Error al reemplazar parte:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 