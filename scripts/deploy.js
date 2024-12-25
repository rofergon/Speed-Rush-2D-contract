const { Wallet, Provider } = require("zksync-ethers");
const { ethers } = require("hardhat");
require("dotenv").config();

async function main() {
  // Obtener el proveedor de zkSync
  const provider = new Provider("https://rpc.testnet.lens.dev");
  
  // Crear una wallet usando la clave privada
  const wallet = new Wallet(process.env.PRIVATE_KEY, provider);

  console.log("Desplegando contrato con la cuenta:", wallet.address);

  // Obtener el factory del contrato
  const artifact = await hre.artifacts.readArtifact("LensNFT");
  
  // Crear una factory para el contrato
  const factory = new ethers.ContractFactory(artifact.abi, artifact.bytecode, wallet);
  
  // Desplegar el contrato
  const contract = await factory.deploy();
  
  // Esperar a que se complete el despliegue
  await contract.waitForDeployment();
  
  console.log("LensNFT desplegado en:", await contract.getAddress());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 