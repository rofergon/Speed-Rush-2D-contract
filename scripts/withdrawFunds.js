const { Wallet, Provider, Contract } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

async function main() {
    // CarNFT contract address
    const CAR_NFT_ADDRESS = "0x33Cf5229318c39d7F754ccbB8FAf61c6470e85dc";

    // Initialize provider and wallet
    const provider = new Provider("https://rpc.testnet.lens.dev");
    const wallet = new Wallet(process.env.PRIVATE_KEY, provider);
    const deployer = new Deployer(hre, wallet);

    // Load contract
    const carNFTArtifact = await deployer.loadArtifact("CarNFT");
    const carNFT = new Contract(CAR_NFT_ADDRESS, carNFTArtifact.abi, wallet);

    try {
        // Get contract balance before withdrawal
        const balanceAntes = await provider.getBalance(CAR_NFT_ADDRESS);
        console.log("\nContract balance before withdrawal:", balanceAntes.toString(), "wei");

        // Get wallet balance before withdrawal
        const walletBalanceAntes = await provider.getBalance(wallet.address);
        console.log("Wallet balance before withdrawal:", walletBalanceAntes.toString(), "wei");

        // Verify we are the owner
        const owner = await carNFT.owner();
        if (owner.toLowerCase() !== wallet.address.toLowerCase()) {
            throw new Error("You are not the contract owner. Only the owner can withdraw funds.");
        }

        // Estimate required gas
        const gasEstimado = await carNFT.withdrawFunds.estimateGas();
        console.log("\nEstimated gas:", gasEstimado.toString());

        // Get gas price
        const gasPrice = await provider.getGasPrice();
        console.log("Gas price:", gasPrice.toString(), "wei");

        // Calculate total gas cost
        const costoGas = BigInt(gasEstimado) * BigInt(gasPrice);
        console.log("Total gas cost:", costoGas.toString(), "wei");

        console.log("\nWithdrawing funds...");
        const tx = await carNFT.withdrawFunds({
            gasLimit: BigInt(Math.floor(Number(gasEstimado) * 1.2)) // Add 20% margin
        });
        console.log("Transaction sent:", tx.hash);
        
        // Wait for confirmation and get receipt
        const receipt = await tx.wait();
        console.log("\nTransaction confirmed!");
        console.log("Gas used:", receipt.gasUsed?.toString() || "N/A");
        console.log("Effective gas price:", receipt.effectiveGasPrice?.toString() || "N/A");
        
        const costoGasReal = receipt.gasUsed && receipt.effectiveGasPrice ? 
            (BigInt(receipt.gasUsed) * BigInt(receipt.effectiveGasPrice)).toString() : 
            "Not available";
        console.log("Total gas cost:", costoGasReal, "wei");

        // Get contract balance after withdrawal
        const balanceDespues = await provider.getBalance(CAR_NFT_ADDRESS);
        console.log("\nContract balance after withdrawal:", balanceDespues.toString(), "wei");

        // Get wallet balance after withdrawal
        const walletBalanceDespues = await provider.getBalance(wallet.address);
        console.log("Wallet balance after withdrawal:", walletBalanceDespues.toString(), "wei");

        // Show withdrawn amount
        const cantidadRetirada = balanceAntes - balanceDespues;
        console.log("\nAmount withdrawn from contract:", cantidadRetirada.toString(), "wei");

        // Show change in wallet balance
        const cambioWallet = walletBalanceDespues - walletBalanceAntes;
        console.log("Change in wallet balance:", cambioWallet.toString(), "wei");
        console.log("(The difference between these values is the gas cost)");

    } catch (error) {
        console.error("\n❌ Error withdrawing funds:", error);
        throw error;
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    }); 