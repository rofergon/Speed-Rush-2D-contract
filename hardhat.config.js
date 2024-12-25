require("@matterlabs/hardhat-zksync-solc");
require("@matterlabs/hardhat-zksync-deploy");
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  zksolc: {
    version: "1.5.8",
    settings: {
      libraries: {},
      enableEraVMExtensions: false,
      optimizer: {
        enabled: true,
        mode: '3'
      }
    }
  },
  networks: {
    lensTestnet: {
      url: "https://rpc.testnet.lens.dev",
      ethNetwork: "sepolia",
      zksync: true,
      chainId: 37111,
      verifyURL: "https://block-explorer.testnet.lens.dev/contract_verification",
      accounts: [process.env.PRIVATE_KEY]
    },
    hardhat: {
      zksync: true
    }
  }
};
