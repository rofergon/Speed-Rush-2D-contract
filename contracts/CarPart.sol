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

    // Core storage
    mapping(uint256 => PartStats) private _partStats;
    mapping(uint256 => uint256) private _equippedInCar; // partId => carId (0 if not equipped)
    uint256 private _currentPartId;
    address public carContract;

    // New mappings for better part tracking
    mapping(address => uint256[]) private _ownerParts;                // owner => all their partIds
    mapping(address => mapping(PartType => uint256[])) private _ownerPartsByType;  // owner => type => partIds
    mapping(address => uint256[]) private _ownerEquippedParts;        // owner => their equipped partIds
    mapping(address => uint256[]) private _ownerUnequippedParts;      // owner => their unequipped partIds

    event PartMinted(uint256 indexed partId, PartType partType);
    event CarContractSet(address indexed carContract);
    event PartEquipped(uint256 indexed partId, uint256 indexed carId);
    event PartUnequipped(uint256 indexed partId, uint256 indexed carId);

    constructor() ERC721("CarPart", "PART") {
        _currentPartId = 0;
        _transferOwnership(msg.sender);
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

        // When minted, the part is automatically equipped in the car
        _equippedInCar[partId] = carId;
        _addToOwnerArrays(to, partId);
        
        emit PartMinted(partId, partType);
        emit PartEquipped(partId, carId);
        _currentPartId++;

        return partId;
    }

    // Override _transfer to update owner mappings
    function _transfer(address from, address to, uint256 tokenId) internal virtual override {
        super._transfer(from, to, tokenId);
        
        // Remove from old owner's arrays
        if (from != address(0)) {
            _removeFromOwnerArrays(from, tokenId);
        }
        
        // Add to new owner's arrays
        _addToOwnerArrays(to, tokenId);
    }

    function _addToOwnerArrays(address owner, uint256 partId) internal {
        PartStats memory stats = _partStats[partId];
        
        // Add to general parts array
        _ownerParts[owner].push(partId);
        
        // Add to type-specific array
        _ownerPartsByType[owner][stats.partType].push(partId);
        
        // Add to equipped/unequipped array
        if (_equippedInCar[partId] == 0) {
            _ownerUnequippedParts[owner].push(partId);
        } else {
            _ownerEquippedParts[owner].push(partId);
        }
    }

    function _removeFromOwnerArrays(address owner, uint256 partId) internal {
        // Remove from general parts array
        _removeFromArray(_ownerParts[owner], partId);
        
        // Remove from type-specific array
        PartStats memory stats = _partStats[partId];
        _removeFromArray(_ownerPartsByType[owner][stats.partType], partId);
        
        // Remove from equipped/unequipped array
        if (_equippedInCar[partId] == 0) {
            _removeFromArray(_ownerUnequippedParts[owner], partId);
        } else {
            _removeFromArray(_ownerEquippedParts[owner], partId);
        }
    }

    function _removeFromArray(uint256[] storage array, uint256 value) internal {
        for (uint i = 0; i < array.length; i++) {
            if (array[i] == value) {
                array[i] = array[array.length - 1];
                array.pop();
                break;
            }
        }
    }

    function setEquippedState(uint256 partId, uint256 carId) external onlyCarContract {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        address owner = ownerOf(partId);
        
        // If part was unequipped and is now being equipped
        if (_equippedInCar[partId] == 0 && carId != 0) {
            _removeFromArray(_ownerUnequippedParts[owner], partId);
            _ownerEquippedParts[owner].push(partId);
            emit PartEquipped(partId, carId);
        }
        // If part was equipped and is now being unequipped
        else if (_equippedInCar[partId] != 0 && carId == 0) {
            _removeFromArray(_ownerEquippedParts[owner], partId);
            _ownerUnequippedParts[owner].push(partId);
            emit PartUnequipped(partId, _equippedInCar[partId]);
        }
        
        _equippedInCar[partId] = carId;
    }

    // Query functions for frontend
    function getOwnerParts(address owner) external view returns (uint256[] memory) {
        return _ownerParts[owner];
    }

    function getOwnerPartsByType(address owner, PartType partType) external view returns (uint256[] memory) {
        return _ownerPartsByType[owner][partType];
    }

    function getOwnerEquippedParts(address owner) external view returns (uint256[] memory) {
        return _ownerEquippedParts[owner];
    }

    function getOwnerUnequippedParts(address owner) external view returns (uint256[] memory) {
        return _ownerUnequippedParts[owner];
    }

    function getOwnerPartsWithDetails(address owner) external view returns (
        PartStats[] memory allParts,
        PartStats[] memory equippedParts,
        PartStats[] memory unequippedParts,
        uint256[] memory equippedInCarIds
    ) {
        uint256[] memory allPartIds = _ownerParts[owner];
        uint256[] memory equippedPartIds = _ownerEquippedParts[owner];
        uint256[] memory unequippedPartIds = _ownerUnequippedParts[owner];

        allParts = new PartStats[](allPartIds.length);
        equippedParts = new PartStats[](equippedPartIds.length);
        unequippedParts = new PartStats[](unequippedPartIds.length);
        equippedInCarIds = new uint256[](equippedPartIds.length);

        // Fill all parts
        for (uint i = 0; i < allPartIds.length; i++) {
            allParts[i] = _partStats[allPartIds[i]];
        }

        // Fill equipped parts
        for (uint i = 0; i < equippedPartIds.length; i++) {
            equippedParts[i] = _partStats[equippedPartIds[i]];
            equippedInCarIds[i] = _equippedInCar[equippedPartIds[i]];
        }

        // Fill unequipped parts
        for (uint i = 0; i < unequippedPartIds.length; i++) {
            unequippedParts[i] = _partStats[unequippedPartIds[i]];
        }

        return (allParts, equippedParts, unequippedParts, equippedInCarIds);
    }

    // Existing utility functions
    function getPartStats(uint256 partId) external view returns (PartStats memory) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _partStats[partId];
    }

    function getPartType(uint256 partId) external view returns (PartType) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _partStats[partId].partType;
    }

    function isEquipped(uint256 partId) external view returns (bool) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _equippedInCar[partId] != 0;
    }

    function getEquippedCar(uint256 partId) external view returns (uint256) {
        require(_ownerOf(partId) != address(0), "Part does not exist");
        return _equippedInCar[partId];
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
