const { Wallet } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const { HardhatRuntimeEnvironment } = require("hardhat/types");
const hre = require("hardhat");

async function main() {
  console.log("Iniciando despliegue en zkSync Era...");

  // Inicializar el wallet desde la private key en .env
  const wallet = new Wallet(process.env.PRIVATE_KEY);
  console.log("Desplegando contratos con la cuenta:", wallet.address);

  // Crear el objeto deployer
  const deployer = new Deployer(hre, wallet);

  try {
    // 1. Desplegar CarPart
    console.log("Desplegando CarPart...");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const carPart = await deployer.deploy(carPartArtifact);
    await carPart.waitForDeployment();
    const carPartAddress = await carPart.getAddress();
    console.log("CarPart desplegado en:", carPartAddress);

    // 2. Desplegar CarNFT
    console.log("Desplegando CarNFT...");
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = await deployer.deploy(carNFTArtifact, [carPartAddress]);
    await carNFT.waitForDeployment();
    const carNFTAddress = await carNFT.getAddress();
    console.log("CarNFT desplegado en:", carNFTAddress);

    // 3. Configurar CarPart para que conozca a CarNFT
    console.log("Configurando CarPart...");
    const tx1 = await carPart.setCarContract(carNFTAddress);
    await tx1.wait();
    console.log("CarPart configurado");

    // 4. Desplegar CarWorkshop
    console.log("Desplegando CarWorkshop...");
    const repairPrice = "100000000000000000"; // 0.1 ETH
    const carWorkshopArtifact = await deployer.loadArtifact("CarWorkshop");
    const carWorkshop = await deployer.deploy(carWorkshopArtifact, [carNFTAddress, repairPrice]);
    await carWorkshop.waitForDeployment();
    const carWorkshopAddress = await carWorkshop.getAddress();
    console.log("CarWorkshop desplegado en:", carWorkshopAddress);

    // 5. Desplegar RaceLeaderboard
    console.log("Desplegando RaceLeaderboard...");
    const raceLeaderboardArtifact = await deployer.loadArtifact("RaceLeaderboard");
    const raceLeaderboard = await deployer.deploy(raceLeaderboardArtifact, [carNFTAddress]);
    await raceLeaderboard.waitForDeployment();
    const raceLeaderboardAddress = await raceLeaderboard.getAddress();
    console.log("RaceLeaderboard desplegado en:", raceLeaderboardAddress);

    // 6. Configurar CarNFT con las direcciones de Workshop y Leaderboard
    console.log("Configurando CarNFT...");
    const tx2 = await carNFT.setWorkshopContract(carWorkshopAddress);
    await tx2.wait();
    const tx3 = await carNFT.setLeaderboardContract(raceLeaderboardAddress);
    await tx3.wait();
    console.log("CarNFT configurado");

    console.log("\nDirecciones de los contratos:");
    console.log("CarPart:", carPartAddress);
    console.log("CarNFT:", carNFTAddress);
    console.log("CarWorkshop:", carWorkshopAddress);
    console.log("RaceLeaderboard:", raceLeaderboardAddress);

    // Guardar las direcciones para la verificación posterior
    console.log("\nGuarda estas direcciones para la verificación de los contratos.");
  } catch (error) {
    console.error("Error detallado:", error);
    throw error;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 