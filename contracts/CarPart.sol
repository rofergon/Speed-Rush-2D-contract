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
        require(msg.sender == carContract, "Only car contract can call this function");
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
        require(stat1 <= 10 && stat2 <= 10 && stat3 <= 10, "Stats must be <= 10");

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
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _partStats[partId];
    }

    function getPartType(uint256 partId) external view returns (PartType) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _partStats[partId].partType;
    }

    function exists(uint256 partId) external view returns (bool) {
        return _ownerOf(partId) != address(0);
    }

    // Helper function to convert stats to legacy format for compatibility
    function convertToLegacyStats(uint256 partId) external view returns (
        uint8 baseSpeed,
        uint8 baseAcceleration,
        uint8 baseHandling,
        uint8 baseDriftFactor,
        uint8 baseTurnFactor,
        uint8 baseMaxSpeed
    ) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        PartStats memory stats = _partStats[partId];

        if (stats.partType == PartType.ENGINE) {
            // Engine: speed, max speed, acceleration
            baseSpeed = stats.stat1;
            baseMaxSpeed = stats.stat2;
            baseAcceleration = stats.stat3;
            // Default values for other stats
            baseHandling = 1;
            baseDriftFactor = 1;
            baseTurnFactor = 1;
        } 
        else if (stats.partType == PartType.TRANSMISSION) {
            // Transmission: acceleration, speed, handling
            baseAcceleration = stats.stat1;
            baseSpeed = stats.stat2;
            baseHandling = stats.stat3;
            // Default values for other stats
            baseDriftFactor = 1;
            baseTurnFactor = 1;
            baseMaxSpeed = stats.stat1; // Acceleration affects max speed
        }
        else if (stats.partType == PartType.WHEELS) {
            // Wheels: handling, drift, turn
            baseHandling = stats.stat1;
            baseDriftFactor = stats.stat2;
            baseTurnFactor = stats.stat3;
            // Default values for other stats
            baseSpeed = 1;
            baseAcceleration = 1;
            baseMaxSpeed = 1;
        }
    }
}
