const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const { Wallet, Provider } = require("zksync-ethers");
const hre = require("hardhat");
require("dotenv").config();

async function main() {
  console.log("Obteniendo wallet...");
  
  // Inicializar el deployer
  const provider = new Provider("https://rpc.testnet.lens.dev");
  const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
  const deployer = new Deployer(hre, wallet);

  console.log("Desplegando sistema con la cuenta:", wallet.address);

  // 1. Desplegar CarAccount (implementation)
  console.log("Desplegando CarAccount (implementation)...");
  const carAccount = await deployer.deploy(await deployer.loadArtifact("CarAccount"));
  console.log("CarAccount desplegado en:", await carAccount.getAddress());

  // 2. Desplegar CarNFT
  console.log("Desplegando CarNFT...");
  const carNFT = await deployer.deploy(await deployer.loadArtifact("CarNFT"));
  console.log("CarNFT desplegado en:", await carNFT.getAddress());

  // 3. Desplegar ERC6551Registry
  console.log("Desplegando ERC6551Registry...");
  const registry = await deployer.deploy(await deployer.loadArtifact("ERC6551Registry"));
  console.log("ERC6551Registry desplegado en:", await registry.getAddress());

  // 4. Desplegar CarFactory
  console.log("Desplegando CarFactory...");
  const carFactory = await deployer.deploy(
    await deployer.loadArtifact("CarFactory"),
    [await registry.getAddress(), await carAccount.getAddress(), await carNFT.getAddress()]
  );
  console.log("CarFactory desplegado en:", await carFactory.getAddress());

  // 5. Transferir ownership del CarNFT al CarFactory
  console.log("Transfiriendo ownership de CarNFT al CarFactory...");
  const tx = await carNFT.transferOwnership(await carFactory.getAddress());
  await tx.wait();
  console.log("Ownership transferido exitosamente");

  console.log("\nResumen del despliegue:");
  console.log("------------------------");
  console.log("CarAccount (implementation):", await carAccount.getAddress());
  console.log("CarNFT:", await carNFT.getAddress());
  console.log("ERC6551Registry:", await registry.getAddress());
  console.log("CarFactory:", await carFactory.getAddress());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 