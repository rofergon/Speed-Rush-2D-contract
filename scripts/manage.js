const { Wallet, Provider } = require("zksync-ethers");
const { ethers } = require("ethers");
require("dotenv").config();

const CONTRACT_ADDRESS = "0x51D967b80eaD6601630E0fA18b2101b90f1AB1d0";

async function main() {
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const artifact = require("../artifacts-zk/contracts/LensNFT.sol/LensNFT.json");
    const contract = new ethers.Contract(CONTRACT_ADDRESS, artifact.abi, wallet);

    // Obtener información del contrato
    const mintPrice = await contract.MINT_PRICE();
    const baseURI = await contract.baseTokenURI();
    const owner = await contract.owner();
    const balance = await provider.getBalance(CONTRACT_ADDRESS);

    console.log("Información del contrato:");
    console.log("------------------------");
    console.log(`Dirección: ${CONTRACT_ADDRESS}`);
    console.log(`Precio de minteo: ${ethers.formatEther(mintPrice)} GRASS`);
    console.log(`Base URI: ${baseURI}`);
    console.log(`Propietario: ${owner}`);
    console.log(`Balance del contrato: ${ethers.formatEther(balance)} GRASS`);
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    }); 