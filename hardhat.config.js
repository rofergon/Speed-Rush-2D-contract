require("@nomicfoundation/hardhat-toolbox");
require("@matterlabs/hardhat-zksync");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  zksolc: {
    version: "latest",
    settings: {},
  },
  defaultNetwork: "lensTestnet",
  networks: {
    lensTestnet: {
      url: "https://rpc.testnet.lens.dev",
      ethNetwork: "sepolia",
      zksync: true,
      chainId: 37111,
      accounts: [process.env.PRIVATE_KEY],
      verifyURL: 'https://block-explorer.testnet.lens.dev/contract_verification'
    }
  },
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  }
};
