// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IRaceLeaderboard {
    struct RaceResult {
        address player;
        uint256 carId;
        uint256 time;
        uint256 timestamp;
    }

    function mintRaceResult(uint256 carId, uint256 time) external returns (uint256);
    function getRaceResult(uint256 tokenId) external view returns (RaceResult memory);
    function ownerOf(uint256 tokenId) external view returns (address);
} 