// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface ICarPart {
    enum PartType {
        ENGINE,
        TRANSMISSION,
        WHEELS,
        TIRES,
        SUSPENSION,
        STEERING,
        DIFFERENTIAL,
        CHASSIS
    }

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

    function ownerOf(uint256 tokenId) external view returns (address);
    function getPartStats(uint256 tokenId) external view returns (PartStats memory);
    function exists(uint256 tokenId) external view returns (bool);
    function setCarContract(address _carContract) external;
} 