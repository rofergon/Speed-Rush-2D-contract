const { Wallet } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const { HardhatRuntimeEnvironment } = require("hardhat/types");
const hre = require("hardhat");

async function main() {
  console.log("Starting deployment on zkSync Era...");

  // Initialize wallet from private key in .env
  const wallet = new Wallet(process.env.PRIVATE_KEY);
  console.log("Deploying contracts with account:", wallet.address);

  // Create deployer object
  const deployer = new Deployer(hre, wallet);

  try {
    // 1. Deploy CarPart
    console.log("Deploying CarPart...");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const carPart = await deployer.deploy(carPartArtifact);
    await carPart.waitForDeployment();
    const carPartAddress = await carPart.getAddress();
    console.log("CarPart deployed at:", carPartAddress);

    // 2. Deploy CarNFT
    console.log("Deploying CarNFT...");
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = await deployer.deploy(carNFTArtifact, [carPartAddress]);
    await carNFT.waitForDeployment();
    const carNFTAddress = await carNFT.getAddress();
    console.log("CarNFT deployed at:", carNFTAddress);

    // 3. Configure CarPart to know about CarNFT
    console.log("Configuring CarPart...");
    const tx1 = await carPart.setCarContract(carNFTAddress);
    await tx1.wait();
    console.log("CarPart configured");

    // 4. Deploy CarWorkshop
    console.log("Deploying CarWorkshop...");
    const repairPrice = "1000000000000000"; // 0.001 GRASS
    const carWorkshopArtifact = await deployer.loadArtifact("CarWorkshop");
    const carWorkshop = await deployer.deploy(carWorkshopArtifact, [carNFTAddress, repairPrice]);
    await carWorkshop.waitForDeployment();
    const carWorkshopAddress = await carWorkshop.getAddress();
    console.log("CarWorkshop deployed at:", carWorkshopAddress);

    // 5. Deploy RaceLeaderboard
    console.log("Deploying RaceLeaderboard...");
    const raceLeaderboardArtifact = await deployer.loadArtifact("RaceLeaderboard");
    const raceLeaderboard = await deployer.deploy(raceLeaderboardArtifact, [carNFTAddress]);
    await raceLeaderboard.waitForDeployment();
    const raceLeaderboardAddress = await raceLeaderboard.getAddress();
    console.log("RaceLeaderboard deployed at:", raceLeaderboardAddress);

    // 6. Configure CarNFT with Workshop and Leaderboard addresses
    console.log("Configuring CarNFT...");
    const tx2 = await carNFT.setWorkshopContract(carWorkshopAddress);
    await tx2.wait();
    const tx3 = await carNFT.setLeaderboardContract(raceLeaderboardAddress);
    await tx3.wait();
    console.log("CarNFT configured");

    console.log("\nContract addresses:");
    console.log("CarPart:", carPartAddress);
    console.log("CarNFT:", carNFTAddress);
    console.log("CarWorkshop:", carWorkshopAddress);
    console.log("RaceLeaderboard:", raceLeaderboardAddress);

    // Save addresses for later verification
    console.log("\nSave these addresses for contract verification.");
  } catch (error) {
    console.error("Detailed error:", error);
    throw error;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 