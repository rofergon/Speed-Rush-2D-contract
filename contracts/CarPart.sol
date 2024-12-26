// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CarPart is ERC721URIStorage, Ownable {
    uint256 private _nextTokenId;
    address public carContract;

    enum PartType { ENGINE, TRANSMISSION, WHEELS, TIRES, SUSPENSION, STEERING, DIFFERENTIAL, CHASSIS }

    struct PartStats {
        PartType partType;
        uint8 baseSpeed;
        uint8 baseAcceleration;
        uint8 baseHandling;
        uint8 baseDriftFactor;
        uint8 baseTurnFactor;
        uint8 baseMaxSpeed;
        string imageURI;
    }

    mapping(uint256 => PartStats) public partStats;
    mapping(uint256 => uint256) public partToCar;

    constructor() ERC721("CarPart", "CPART") Ownable(msg.sender) {
        _nextTokenId = 0;
    }

    function setCarContract(address _carContract) external onlyOwner {
        carContract = _carContract;
    }

    function exists(uint256 tokenId) public view returns (bool) {
        try this.ownerOf(tokenId) returns (address) {
            return true;
        } catch {
            return false;
        }
    }

    function mint(
        address to,
        PartType partType,
        uint8 baseSpeed,
        uint8 baseAcceleration,
        uint8 baseHandling,
        uint8 baseDriftFactor,
        uint8 baseTurnFactor,
        uint8 baseMaxSpeed,
        string memory imageURI,
        uint256 carId
    ) public returns (uint256) {
        require(msg.sender == owner() || msg.sender == carContract, "Only owner or car contract can mint parts");
        
        uint256 tokenId = _nextTokenId++;
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, imageURI);
        
        partStats[tokenId] = PartStats(
            partType,
            baseSpeed,
            baseAcceleration,
            baseHandling,
            baseDriftFactor,
            baseTurnFactor,
            baseMaxSpeed,
            imageURI
        );

        partToCar[tokenId] = carId;
        return tokenId;
    }

    function getPartStats(uint256 tokenId) public view returns (PartStats memory) {
        require(exists(tokenId), "Part does not exist");
        return partStats[tokenId];
    }

    function upgradePart(
        uint256 tokenId,
        uint8 newSpeed,
        uint8 newAcceleration,
        uint8 newHandling,
        uint8 newDriftFactor,
        uint8 newTurnFactor,
        uint8 newMaxSpeed
    ) public onlyOwner {
        require(exists(tokenId), "Part does not exist");

        PartStats storage stats = partStats[tokenId];
        stats.baseSpeed = newSpeed;
        stats.baseAcceleration = newAcceleration;
        stats.baseHandling = newHandling;
        stats.baseDriftFactor = newDriftFactor;
        stats.baseTurnFactor = newTurnFactor;
        stats.baseMaxSpeed = newMaxSpeed;
    }

    function updatePartImage(uint256 tokenId, string memory newImageURI) public onlyOwner {
        require(exists(tokenId), "Part does not exist");
        _setTokenURI(tokenId, newImageURI);
        partStats[tokenId].imageURI = newImageURI;
    }
} 