// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./CarNFT.sol";

contract RaceLeaderboard is ERC721, Ownable {
    struct RaceResult {
        address player;
        uint256 carId;
        uint256 time;
        uint256 timestamp;
    }

    CarNFT public carNFT;
    uint256 private _nextTokenId;
    mapping(uint256 => RaceResult) public raceResults;

    constructor(address _carNFT) ERC721("RaceResult", "RACE") Ownable(msg.sender) {
        carNFT = CarNFT(_carNFT);
        _nextTokenId = 1;
    }

    function mintRaceResult(uint256 carId, uint256 time) external returns (uint256) {
        require(carNFT.ownerOf(carId) == msg.sender, "No eres el dueno del carro");
        
        uint256 tokenId = _nextTokenId++;
        _safeMint(msg.sender, tokenId);
        
        raceResults[tokenId] = RaceResult({
            player: msg.sender,
            carId: carId,
            time: time,
            timestamp: block.timestamp
        });

        // Llamar a la funcion de degradacion del carro
        carNFT.degradeCar(carId);
        
        return tokenId;
    }

    function getRaceResult(uint256 tokenId) external view returns (RaceResult memory) {
        require(_ownerOf(tokenId) != address(0), "El resultado no existe");
        return raceResults[tokenId];
    }
} 