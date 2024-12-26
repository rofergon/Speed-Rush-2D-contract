const { Wallet, Provider } = require("zksync-ethers");
const { ethers } = require("ethers");
require("dotenv").config();

const CONTRACT_ADDRESS = "0x51D967b80eaD6601630E0fA18b2101b90f1AB1d0";

async function main() {
    if (!process.argv[2]) {
        console.error("Please specify the new base URI");
        process.exit(1);
    }

    const newURI = process.argv[2];
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const artifact = require("../artifacts-zk/contracts/LensNFT.sol/LensNFT.json");
    const contract = new ethers.Contract(CONTRACT_ADDRESS, artifact.abi, wallet);

    console.log(`Setting new baseURI: ${newURI}`);
    const tx = await contract.setBaseURI(newURI);
    await tx.wait();
    console.log("BaseURI updated successfully");
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    }); 