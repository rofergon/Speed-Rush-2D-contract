// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface ICarNFT {
    struct CarComposition {
        uint256[] partIds;
        string carImageURI;
    }

    struct CompactCarStats {
        string imageURI;
        uint8 speed;
        uint8 acceleration;
        uint8 handling;
        uint8 driftFactor;
        uint8 turnFactor;
        uint8 maxSpeed;
        uint8 condition;
    }

    function ownerOf(uint256 tokenId) external view returns (address);
    function exists(uint256 tokenId) external view returns (bool);
    function repairCar(uint256 carId) external;
    function degradeCar(uint256 carId) external;
    function getCarComposition(uint256 carId) external view returns (uint256[] memory partIds, string memory carImageURI);
    function getCompactCarStats(uint256 carId) external view returns (CompactCarStats memory);
    function setWorkshopContract(address _workshop) external;
    function setLeaderboardContract(address _leaderboard) external;
} 