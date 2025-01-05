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
  
  // Convert BigInt to string before saving
  const addressesForSave = {};
  for (const [key, value] of Object.entries(addresses)) {
    if (typeof value === 'bigint') {
      addressesForSave[key] = value.toString();
    } else {
      addressesForSave[key] = value;
    }
  }
  
  fs.writeFileSync(
    fileName,
    JSON.stringify(addressesForSave, null, 2)
  );
  
  console.log(`\nDeployment addresses saved to: ${fileName}`);
}

async function verifyContract(address, constructorArguments) {
  try {
    // Convert BigInt to string for verification
    const args = constructorArguments.map(arg => 
      typeof arg === 'bigint' ? arg.toString() : arg
    );
    
    await hre.run("verify:verify", {
      address: address,
      constructorArguments: args,
    });
    console.log(`✅ Contract verified at: ${address}`);
  } catch (error) {
    console.log(`❌ Error verifying contract: ${error.message}`);
  }
}

async function main() {
  console.log(`🚀 Starting deployment on ${hre.network.name}...`);

  // Initialize wallet from private key in .env
  const wallet = new Wallet(process.env.PRIVATE_KEY);
  console.log(`📝 Deploying contracts with account: ${wallet.address}`);

  // Create deployer
  const deployer = new Deployer(hre, wallet);
  const addresses = {};

  try {
    // 1. Deploy CarPart
    console.log("\n📦 Deploying CarPart...");
    const carPartArtifact = await deployer.loadArtifact("CarPart");
    const carPart = await deployer.deploy(carPartArtifact);
    await carPart.waitForDeployment();
    addresses.carPart = await carPart.getAddress();
    console.log(`✅ CarPart deployed at: ${addresses.carPart}`);

    // 2. Deploy CarNFT
    console.log("\n📦 Deploying CarNFT...");
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = await deployer.deploy(carNFTArtifact, [addresses.carPart]);
    await carNFT.waitForDeployment();
    addresses.carNFT = await carNFT.getAddress();
    console.log(`✅ CarNFT deployed at: ${addresses.carNFT}`);

    // 3. Configure CarPart
    console.log("\n⚙️ Configuring CarPart...");
    const tx1 = await carPart.setCarContract(addresses.carNFT);
    await tx1.wait();
    console.log("✅ CarPart configured");

    // 4. Deploy CarWorkshop
    console.log("\n📦 Deploying CarWorkshop...");
    const repairPrice = "1000000000000000"; // 0.001 ETH in wei
    const carWorkshopArtifact = await deployer.loadArtifact("CarWorkshop");
    const carWorkshop = await deployer.deploy(carWorkshopArtifact, [addresses.carNFT, repairPrice]);
    await carWorkshop.waitForDeployment();
    addresses.carWorkshop = await carWorkshop.getAddress();
    console.log(`✅ CarWorkshop deployed at: ${addresses.carWorkshop}`);

    // 5. Deploy RaceLeaderboard
    console.log("\n📦 Deploying RaceLeaderboard...");
    const raceLeaderboardArtifact = await deployer.loadArtifact("RaceLeaderboard");
    const raceLeaderboard = await deployer.deploy(raceLeaderboardArtifact, [addresses.carNFT]);
    await raceLeaderboard.waitForDeployment();
    addresses.raceLeaderboard = await raceLeaderboard.getAddress();
    console.log(`✅ RaceLeaderboard deployed at: ${addresses.raceLeaderboard}`);

    // 6. Deploy CarMarketplace
    console.log("\n📦 Deploying CarMarketplace...");
    const carMarketplaceArtifact = await deployer.loadArtifact("CarMarketplace");
    const carMarketplace = await deployer.deploy(carMarketplaceArtifact, [addresses.carNFT, addresses.carPart]);
    await carMarketplace.waitForDeployment();
    addresses.carMarketplace = await carMarketplace.getAddress();
    console.log(`✅ CarMarketplace deployed at: ${addresses.carMarketplace}`);

    // 7. Configure CarNFT
    console.log("\n⚙️ Configuring CarNFT...");
    const tx2 = await carNFT.setWorkshopContract(addresses.carWorkshop);
    await tx2.wait();
    const tx3 = await carNFT.setLeaderboardContract(addresses.raceLeaderboard);
    await tx3.wait();
    console.log("✅ CarNFT configured");

    // Save addresses
    await saveDeployment(addresses);

    // Verify contracts if not on localhost
    if (hre.network.name !== "hardhat" && hre.network.name !== "localhost") {
      console.log("\n🔍 Starting contract verification...");
      
      await verifyContract(addresses.carPart, []);
      await verifyContract(addresses.carNFT, [addresses.carPart]);
      await verifyContract(addresses.carWorkshop, [addresses.carNFT, repairPrice]);
      await verifyContract(addresses.raceLeaderboard, [addresses.carNFT]);
      await verifyContract(addresses.carMarketplace, [addresses.carNFT, addresses.carPart]);
    }

    console.log("\n✨ Deployment completed successfully!");
    console.log("\n📄 Address Summary:");
    // Convert BigInt to string for display
    const addressesForDisplay = {};
    for (const [key, value] of Object.entries(addresses)) {
      if (typeof value === 'bigint') {
        addressesForDisplay[key] = value.toString();
      } else {
        addressesForDisplay[key] = value;
      }
    }
    console.table(addressesForDisplay);

  } catch (error) {
    console.error("\n❌ Error during deployment:");
    console.error(error);
    process.exit(1);
  }
}

// Execute script
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 