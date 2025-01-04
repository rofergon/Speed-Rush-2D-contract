const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // Direcciones de los contratos desplegados
    const CAR_NFT_ADDRESS = "0x0a86889ab97C6911fBbFE7C0961b391a7CbAC0DC";
    const CAR_PART_ADDRESS = "0x498574252740d90e9629fF25ca31f3620C7dCB50";

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
        // Mintear un nuevo carro con todas sus partes
        console.log("\n🚗 Minteando un nuevo carro...");
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
                stat1: 7,    // acceleration
                stat2: 8,    // speed
                stat3: 8,    // handling
                imageURI: "https://example.com/transmission1.jpg"
            },
            {
                partType: 2, // WHEELS
                stat1: 8,    // handling
                stat2: 7,    // drift
                stat3: 8,    // turn
                imageURI: "https://example.com/wheels1.jpg"
            }
        ];

        const mintPrice = await carNFT.mintPrice();
        console.log("Precio de minteo:", mintPrice.toString());
        const tx1 = await carNFT.mintCar(carImageURI, partsData, { value: mintPrice });
        const receipt1 = await tx1.wait();
        console.log("✅ Carro minteado!");

        // Obtener el ID del carro minteado
        const carId = 1;

        // Obtener composición inicial del carro
        console.log("\n📊 Composición inicial del carro:");
        const comp = await carNFT.getCarComposition(carId);
        console.log("Partes:", comp.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", comp.slotOccupied);

        // Obtener estadísticas iniciales
        console.log("\nEstadísticas iniciales:");
        const statsInicial = await carNFT.getCompactCarStats(carId);
        console.log("Velocidad:", statsInicial.speed.toString());
        console.log("Aceleración:", statsInicial.acceleration.toString());
        console.log("Manejo:", statsInicial.handling.toString());

        // Desequipar el motor (primera parte)
        const engineId = comp.partIds[0];
        console.log("\n🔧 Desequipando el motor (ID:", engineId.toString(), ")...");
        const tx2 = await carNFT.unequipPart(carId, engineId);
        await tx2.wait();
        console.log("✅ Motor desequipado!");

        // Verificar la composición sin el motor
        console.log("\n📊 Composición sin motor:");
        const compSinMotor = await carNFT.getCarComposition(carId);
        console.log("Partes:", compSinMotor.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", compSinMotor.slotOccupied);

        // Obtener estadísticas sin motor
        console.log("\nEstadísticas sin motor:");
        const statsSinMotor = await carNFT.getCompactCarStats(carId);
        console.log("Velocidad:", statsSinMotor.speed.toString());
        console.log("Aceleración:", statsSinMotor.acceleration.toString());
        console.log("Manejo:", statsSinMotor.handling.toString());

        // Volver a equipar el motor
        console.log("\n🔧 Equipando el motor de nuevo...");
        const tx3 = await carNFT.equipPart(carId, engineId, 0); // 0 es el slot del motor
        await tx3.wait();
        console.log("✅ Motor equipado!");

        // Verificar la composición final
        console.log("\n📊 Composición final:");
        const compFinal = await carNFT.getCarComposition(carId);
        console.log("Partes:", compFinal.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", compFinal.slotOccupied);

        // Obtener estadísticas finales
        console.log("\nEstadísticas finales:");
        const statsFinal = await carNFT.getCompactCarStats(carId);
        console.log("Velocidad:", statsFinal.speed.toString());
        console.log("Aceleración:", statsFinal.acceleration.toString());
        console.log("Manejo:", statsFinal.handling.toString());

    } catch (error) {
        console.error("Error:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 