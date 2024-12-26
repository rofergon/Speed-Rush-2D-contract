const { Wallet } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");
const fs = require("fs");

async function saveDeployment(addresses) {
  const deploymentPath = "./deployments-zk/";
  if (!fs.existsSync(deploymentPath)) {
    fs.mkdirSync(deploymentPath, { recursive: true });
  }
  
  const network = hre.network.name;
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
  const fileName = `${deploymentPath}deployment-${network}-${timestamp}.json`;
  
  fs.writeFileSync(
    fileName,
    JSON.stringify(addresses, null, 2)
  );
  
  console.log(`\nDeployment addresses saved to: ${fileName}`);
}

async function verifyContract(address, constructorArguments) {
  try {
    await hre.run("verify:verify", {
      address: address,
      constructorArguments: constructorArguments,
    });
    console.log(`✅ Contrato verificado en: ${address}`);
  } catch (error) {
    console.log(`❌ Error verificando contrato: ${error.message}`);
  }
}

async function main() {
  console.log(`🚀 Iniciando despliegue en ${hre.network.name}...`);

  // Inicializar wallet
  const wallet = new Wallet(process.env.PRIVATE_KEY);
  console.log(`📝 Desplegando contratos con la cuenta: ${wallet.address}`);

  // Crear deployer
  const deployer = new Deployer(hre, wallet);
  const addresses = {};

  try {
    // 1. Desplegar CarPart
    console.log("\n📦 Desplegando CarPart...");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const carPart = await deployer.deploy(carPartArtifact);
    await carPart.waitForDeployment();
    addresses.carPart = await carPart.getAddress();
    console.log(`✅ CarPart desplegado en: ${addresses.carPart}`);

    // 2. Desplegar CarNFT
    console.log("\n📦 Desplegando CarNFT...");
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = await deployer.deploy(carNFTArtifact, [addresses.carPart]);
    await carNFT.waitForDeployment();
    addresses.carNFT = await carNFT.getAddress();
    console.log(`✅ CarNFT desplegado en: ${addresses.carNFT}`);

    // 3. Configurar CarPart
    console.log("\n⚙️ Configurando CarPart...");
    const tx1 = await carPart.setCarContract(addresses.carNFT);
    await tx1.wait();
    console.log("✅ CarPart configurado");

    // 4. Desplegar CarWorkshop
    console.log("\n📦 Desplegando CarWorkshop...");
    const repairPrice = hre.ethers.parseEther("0.001"); // 0.001 ETH
    const carWorkshopArtifact = await deployer.loadArtifact("CarWorkshop");
    const carWorkshop = await deployer.deploy(carWorkshopArtifact, [addresses.carNFT, repairPrice]);
    await carWorkshop.waitForDeployment();
    addresses.carWorkshop = await carWorkshop.getAddress();
    console.log(`✅ CarWorkshop desplegado en: ${addresses.carWorkshop}`);

    // 5. Desplegar RaceLeaderboard
    console.log("\n📦 Desplegando RaceLeaderboard...");
    const raceLeaderboardArtifact = await deployer.loadArtifact("RaceLeaderboard");
    const raceLeaderboard = await deployer.deploy(raceLeaderboardArtifact, [addresses.carNFT]);
    await raceLeaderboard.waitForDeployment();
    addresses.raceLeaderboard = await raceLeaderboard.getAddress();
    console.log(`✅ RaceLeaderboard desplegado en: ${addresses.raceLeaderboard}`);

    // 6. Configurar CarNFT
    console.log("\n⚙️ Configurando CarNFT...");
    const tx2 = await carNFT.setWorkshopContract(addresses.carWorkshop);
    await tx2.wait();
    const tx3 = await carNFT.setLeaderboardContract(addresses.raceLeaderboard);
    await tx3.wait();
    console.log("✅ CarNFT configurado");

    // Guardar direcciones
    await saveDeployment(addresses);

    // Verificar contratos si no estamos en localhost
    if (hre.network.name !== "hardhat" && hre.network.name !== "localhost") {
      console.log("\n🔍 Iniciando verificación de contratos...");
      
      await verifyContract(addresses.carPart, []);
      await verifyContract(addresses.carNFT, [addresses.carPart]);
      await verifyContract(addresses.carWorkshop, [addresses.carNFT, repairPrice]);
      await verifyContract(addresses.raceLeaderboard, [addresses.carNFT]);
    }

    console.log("\n✨ Despliegue completado exitosamente!");
    console.log("\n📄 Resumen de direcciones:");
    console.table(addresses);

  } catch (error) {
    console.error("\n❌ Error durante el despliegue:");
    console.error(error);
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });