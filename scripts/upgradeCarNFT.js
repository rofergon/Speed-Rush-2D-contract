const { Wallet, Provider } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    console.log("Iniciando despliegue del contrato CarNFT actualizado...");

    // Inicializar provider y wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Obtener la dirección del contrato CarPart actual
    const CAR_NFT_ADDRESS = "0x33Cf5229318c39d7F754ccbB8FAf61c6470e85dc";
    const carNFT = await deployer.loadArtifact("CarNFT");
    const oldContract = new carNFT.contract(CAR_NFT_ADDRESS, wallet);
    const carPartAddress = await oldContract.carPartContract();

    // Desplegar el nuevo contrato
    console.log("\nDesplegando nuevo contrato CarNFT...");
    const newCarNFT = await deployer.deploy(carNFT, [carPartAddress]);
    
    console.log("\nContrato CarNFT actualizado desplegado en:", await newCarNFT.getAddress());
    console.log("No olvides actualizar la dirección en los scripts de interacción.");

    // Verificar el contrato
    console.log("\nVerificando contrato...");
    await hre.run("verify:verify", {
        address: await newCarNFT.getAddress(),
        constructorArguments: [carPartAddress],
    });
    
    console.log("\n¡Despliegue y verificación completados!");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 