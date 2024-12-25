// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

interface ICarNFT {
    function mintCarNFT(address to) external returns (uint256);
}

interface IERC6551Registry {
    function createAccount(
        address implementation,
        bytes32 salt,
        uint256 chainId,
        address tokenContract,
        uint256 tokenId
    ) external returns (address);
}

contract CarFactory is Ownable {
    /// @dev Dirección del contrato que implementa la lógica de la cuenta 6551 (el "implementation").
    address public accountImplementation;

    /// @dev Dirección del contrato ERC6551Registry.
    IERC6551Registry public registry;

    /// @dev Dirección del contrato CarNFT.
    ICarNFT public carNFT;

    /// @notice Evento que se emite cuando se crea un nuevo Carro y su cuenta 6551.
    event CarCreated(
        address indexed owner,
        uint256 indexed tokenId,
        address indexed account
    );

    /**
     * @param _registry           Dirección del ERC6551Registry ya desplegado.
     * @param _accountImpl        Dirección del contrato "implementation" para 6551.
     * @param _carNFT             Dirección del contrato CarNFT (ERC721).
     */
    constructor(
        IERC6551Registry _registry,
        address _accountImpl,
        ICarNFT _carNFT
    ) Ownable(msg.sender) {
        registry = _registry;
        accountImplementation = _accountImpl;
        carNFT = _carNFT;
    }

    /**
     * @dev Crea un nuevo Carro (ERC721) y una cuenta 6551 asociada.
     * @return tokenId El ID del Carro minteado
     * @return account La dirección de la cuenta 6551 asociada al Carro
     */
    function createCar() external returns (uint256 tokenId, address account) {
        // 1. Mintear el Carro para msg.sender
        tokenId = carNFT.mintCarNFT(msg.sender);

        // 2. Generar un salt "semi-único" (puedes mejorarlo con VRF o combos).
        bytes32 salt = keccak256(
            abi.encodePacked(block.timestamp, msg.sender, tokenId)
        );

        // 3. Crear la cuenta 6551 para este token
        account = registry.createAccount(
            accountImplementation,
            salt,
            block.chainid,
            address(carNFT),
            tokenId
        );

        // 4. Emitir un evento para rastrear la creación
        emit CarCreated(msg.sender, tokenId, account);
    }

    /**
     * @dev Ajusta la dirección del contrato implementation 6551, si quieres
     *      cambiar la lógica de las cuentas en el futuro.
     */
    function setAccountImplementation(address _newImpl) external onlyOwner {
        accountImplementation = _newImpl;
    }

    /**
     * @dev Ajusta la dirección del contrato registry si fuera necesario.
     */
    function setRegistry(IERC6551Registry _newRegistry) external onlyOwner {
        registry = _newRegistry;
    }

    /**
     * @dev Ajusta la dirección del CarNFT si fuera necesario.
     */
    function setCarNFT(ICarNFT _newCarNFT) external onlyOwner {
        carNFT = _newCarNFT;
    }
}
