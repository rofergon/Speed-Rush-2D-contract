const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0xabD2368daE3b292FE732C6D2760a44FbE33EaA13";
    const CAR_PART_ADDRESS = "0x48AA0974C1E4DAc4c68BFf740Cc4411D40cfe0c1";

    // Inicializar el provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Cargar los contratos
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);
    const carPart = new Contract(CAR_PART_ADDRESS, carPartArtifact.abi, wallet);

    // ID del carro a verificar
    const carId = 1;

    try {
        console.log(`\nVerificando partes del carro ID: ${carId}`);
        
        // Obtener la composición del carro (array de IDs de partes)
        const [partIds, carImageURI] = await carNFT.getCarComposition(carId);
        console.log(`\nCarro URI: ${carImageURI}`);
        console.log(`Número de partes: ${partIds.length}`);

        // Verificar cada parte
        for (let i = 0; i < partIds.length; i++) {
            const partId = partIds[i];
            console.log(`\nParte #${i + 1} (ID: ${partId}):`);
            
            // Verificar el dueño de la parte
            const owner = await carPart.ownerOf(partId);
            console.log(`Dueño: ${owner}`);

            // Obtener estadísticas de la parte
            const stats = await carPart.getPartStats(partId);
            console.log("Tipo de parte:", getPartTypeName(stats.partType));
            console.log("Estadísticas base:");
            console.log("- Velocidad:", stats.baseSpeed.toString());
            console.log("- Aceleración:", stats.baseAcceleration.toString());
            console.log("- Manejo:", stats.baseHandling.toString());
            console.log("- Factor de Derrape:", stats.baseDriftFactor.toString());
            console.log("- Factor de Giro:", stats.baseTurnFactor.toString());
            console.log("- Velocidad Máxima:", stats.baseMaxSpeed.toString());
            console.log("URI de la imagen:", stats.imageURI);
        }

    } catch (error) {
        console.error("Error al verificar las partes del carro:", error);
        throw error;
    }
}

function getPartTypeName(partType) {
    const types = ["Motor", "Transmisión", "Ruedas"];
    return types[partType] || "Desconocido";
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 