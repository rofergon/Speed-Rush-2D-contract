// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CarPart is ERC721, Ownable {
    enum PartType { ENGINE, TRANSMISSION, WHEELS }

    struct PartStats {
        PartType partType;
        uint8 stat1;
        uint8 stat2;
        uint8 stat3;
        string imageURI;
    }

    mapping(uint256 => PartStats) private _partStats;
    uint256 private _currentPartId;
    address public carContract;

    event PartMinted(uint256 indexed partId, PartType partType);
    event CarContractSet(address indexed carContract);

    constructor() ERC721("CarPart", "PART") Ownable(msg.sender) {
        _currentPartId = 0;
    }

    modifier onlyCarContract() {
        require(msg.sender == carContract, "Solo el contrato de carros puede llamar esta funcion");
        _;
    }

    function setCarContract(address _carContract) external onlyOwner {
        carContract = _carContract;
        emit CarContractSet(_carContract);
    }

    function mint(
        address to,
        PartType partType,
        uint8 stat1,
        uint8 stat2,
        uint8 stat3,
        string memory imageURI,
        uint256 carId
    ) external onlyCarContract returns (uint256) {
        require(stat1 <= 10 && stat2 <= 10 && stat3 <= 10, "Las estadisticas deben ser <= 10");

        uint256 partId = _currentPartId;
        _safeMint(to, partId);

        _partStats[partId] = PartStats({
            partType: partType,
            stat1: stat1,
            stat2: stat2,
            stat3: stat3,
            imageURI: imageURI
        });

        emit PartMinted(partId, partType);
        _currentPartId++;

        return partId;
    }

    function getPartStats(uint256 partId) external view returns (PartStats memory) {
        require(_ownerOf(partId) != address(0), "La parte no existe");
        return _partStats[partId];
    }

    function getPartType(uint256 partId) external view returns (PartType) {
        require(_ownerOf(partId) != address(0), "La parte no existe");
        return _partStats[partId].partType;
    }

    function exists(uint256 partId) external view returns (bool) {
        return _ownerOf(partId) != address(0);
    }

    // Función auxiliar para convertir las estadísticas al formato antiguo para compatibilidad
    function convertToLegacyStats(uint256 partId) external view returns (
        uint8 baseSpeed,
        uint8 baseAcceleration,
        uint8 baseHandling,
        uint8 baseDriftFactor,
        uint8 baseTurnFactor,
        uint8 baseMaxSpeed
    ) {
        require(_ownerOf(partId) != address(0), "La parte no existe");
        PartStats memory stats = _partStats[partId];

        if (stats.partType == PartType.ENGINE) {
            // Motor: velocidad, velocidad máxima, aceleración
            baseSpeed = stats.stat1;
            baseMaxSpeed = stats.stat2;
            baseAcceleration = stats.stat3;
            // Valores por defecto para las demás estadísticas
            baseHandling = 1;
            baseDriftFactor = 1;
            baseTurnFactor = 1;
        } 
        else if (stats.partType == PartType.TRANSMISSION) {
            // Transmisión: aceleración, velocidad, manejo
            baseAcceleration = stats.stat1;
            baseSpeed = stats.stat2;
            baseHandling = stats.stat3;
            // Valores por defecto para las demás estadísticas
            baseDriftFactor = 1;
            baseTurnFactor = 1;
            baseMaxSpeed = stats.stat1; // La aceleración afecta la velocidad máxima
        }
        else if (stats.partType == PartType.WHEELS) {
            // Ruedas: manejo, derrape, giro
            baseHandling = stats.stat1;
            baseDriftFactor = stats.stat2;
            baseTurnFactor = stats.stat3;
            // Valores por defecto para las demás estadísticas
            baseSpeed = 1;
            baseAcceleration = 1;
            baseMaxSpeed = 1;
        }
    }
}
