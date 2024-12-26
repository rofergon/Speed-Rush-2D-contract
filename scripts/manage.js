const { Wallet, Provider } = require("zksync-ethers");
const { ethers } = require("ethers");
require("dotenv").config();

const CONTRACT_ADDRESS = "0x51D967b80eaD6601630E0fA18b2101b90f1AB1d0";

async function main() {
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const artifact = require("../artifacts-zk/contracts/LensNFT.sol/LensNFT.json");
    const contract = new ethers.Contract(CONTRACT_ADDRESS, artifact.abi, wallet);

    // Get contract information
    const mintPrice = await contract.MINT_PRICE();
    const baseURI = await contract.baseTokenURI();
    const owner = await contract.owner();
    const balance = await provider.getBalance(CONTRACT_ADDRESS);

    console.log("Contract Information:");
    console.log("------------------------");
    console.log(`Address: ${CONTRACT_ADDRESS}`);
    console.log(`Mint Price: ${ethers.formatEther(mintPrice)} GRASS`);
    console.log(`Base URI: ${baseURI}`);
    console.log(`Owner: ${owner}`);
    console.log(`Contract Balance: ${ethers.formatEther(balance)} GRASS`);
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    }); 