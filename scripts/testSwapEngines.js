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
        // Obtener el precio de minteo
        const mintPrice = await carNFT.mintPrice();
        console.log("\n💰 Precio de minteo:", mintPrice.toString(), "wei");

        // Mintear el primer carro
        console.log("\n🚗 Minteando el primer carro...");
        const carImageURI1 = "https://example.com/car1.jpg";
        const partsData1 = [
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

        const tx1 = await carNFT.mintCar(carImageURI1, partsData1, { value: mintPrice });
        const receipt1 = await tx1.wait();
        console.log("✅ Primer carro minteado!");

        // Mintear el segundo carro
        console.log("\n🚗 Minteando el segundo carro...");
        const carImageURI2 = "https://example.com/car2.jpg";
        const partsData2 = [
            {
                partType: 0, // ENGINE
                stat1: 9,    // speed
                stat2: 10,   // max speed
                stat3: 8,    // acceleration
                imageURI: "https://example.com/engine2.jpg"
            },
            {
                partType: 1, // TRANSMISSION
                stat1: 9,    // acceleration
                stat2: 8,    // speed
                stat3: 9,    // handling
                imageURI: "https://example.com/transmission2.jpg"
            },
            {
                partType: 2, // WHEELS
                stat1: 10,   // handling
                stat2: 8,    // drift
                stat3: 9,    // turn
                imageURI: "https://example.com/wheels2.jpg"
            }
        ];

        const tx2 = await carNFT.mintCar(carImageURI2, partsData2, { value: mintPrice });
        const receipt2 = await tx2.wait();
        console.log("✅ Segundo carro minteado!");

        // Obtener los IDs de los carros minteados
        const lastTokenId = await carNFT.getLastTokenId();
        const carId1 = BigInt(lastTokenId) - 1n; // Primer carro
        const carId2 = lastTokenId;              // Segundo carro
        console.log("\n🔑 IDs de los carros:", carId1.toString(), "y", carId2.toString());

        // Obtener composición inicial de ambos carros
        console.log("\n📊 Composición inicial de los carros:");
        const comp1 = await carNFT.getCarComposition(carId1);
        const comp2 = await carNFT.getCarComposition(carId2);
        
        console.log("\nCarro 1:");
        console.log("Partes:", comp1.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", comp1.slotOccupied);
        let stats1 = await carNFT.getCompactCarStats(carId1);
        console.log("Velocidad:", stats1.speed.toString());
        console.log("Aceleración:", stats1.acceleration.toString());
        console.log("Manejo:", stats1.handling.toString());

        console.log("\nCarro 2:");
        console.log("Partes:", comp2.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", comp2.slotOccupied);
        let stats2 = await carNFT.getCompactCarStats(carId2);
        console.log("Velocidad:", stats2.speed.toString());
        console.log("Aceleración:", stats2.acceleration.toString());
        console.log("Manejo:", stats2.handling.toString());

        // Obtener los IDs de los motores
        const engine1Id = comp1.partIds[0];
        const engine2Id = comp2.partIds[0];

        // Intercambiar los motores
        console.log("\n🔄 Intercambiando motores...");
        
        // Primero desequipamos ambos motores
        console.log("\nDesequipando motor del carro 1...");
        const tx3 = await carNFT.unequipPart(carId1, engine1Id);
        await tx3.wait();
        console.log("✅ Motor del carro 1 desequipado!");

        console.log("\nDesequipando motor del carro 2...");
        const tx4 = await carNFT.unequipPart(carId2, engine2Id);
        await tx4.wait();
        console.log("✅ Motor del carro 2 desequipado!");

        // Luego equipamos los motores en el otro carro
        console.log("\nEquipando motor 2 en carro 1...");
        const tx5 = await carNFT.equipPart(carId1, engine2Id, 0);
        await tx5.wait();
        console.log("✅ Motor 2 equipado en carro 1!");

        console.log("\nEquipando motor 1 en carro 2...");
        const tx6 = await carNFT.equipPart(carId2, engine1Id, 0);
        await tx6.wait();
        console.log("✅ Motor 1 equipado en carro 2!");

        // Verificar la nueva composición de los carros
        console.log("\n📊 Composición final de los carros:");
        const newComp1 = await carNFT.getCarComposition(carId1);
        const newComp2 = await carNFT.getCarComposition(carId2);
        
        console.log("\nCarro 1:");
        console.log("Partes:", newComp1.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", newComp1.slotOccupied);
        stats1 = await carNFT.getCompactCarStats(carId1);
        console.log("Velocidad:", stats1.speed.toString());
        console.log("Aceleración:", stats1.acceleration.toString());
        console.log("Manejo:", stats1.handling.toString());

        console.log("\nCarro 2:");
        console.log("Partes:", newComp2.partIds.map(id => id.toString()));
        console.log("Slots ocupados:", newComp2.slotOccupied);
        stats2 = await carNFT.getCompactCarStats(carId2);
        console.log("Velocidad:", stats2.speed.toString());
        console.log("Aceleración:", stats2.acceleration.toString());
        console.log("Manejo:", stats2.handling.toString());

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