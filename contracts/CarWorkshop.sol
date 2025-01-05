// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./ICarNFT.sol";

contract CarWorkshop is Ownable {
    ICarNFT public carNFT;
    uint256 public repairPrice;
    
    event CarRepaired(uint256 indexed carId);
    event RepairPriceChanged(uint256 newPrice);
    
    constructor(address _carNFT, uint256 _repairPrice) {
        carNFT = ICarNFT(_carNFT);
        repairPrice = _repairPrice;
        _transferOwnership(msg.sender);
    }
    
    function setRepairPrice(uint256 _newPrice) external onlyOwner {
        repairPrice = _newPrice;
        emit RepairPriceChanged(_newPrice);
    }
    
    function repairCar(uint256 carId) external payable {
        require(msg.value >= repairPrice, "Insufficient payment");
        require(carNFT.ownerOf(carId) == msg.sender, "Not the car owner");
        
        carNFT.repairCar(carId);
        
        // Return excess payment using call
        if (msg.value > repairPrice) {
            (bool success, ) = payable(msg.sender).call{value: msg.value - repairPrice}("");
            require(success, "Error returning excess payment");
        }

        emit CarRepaired(carId);
    }
    
    function withdrawFunds() external onlyOwner {
        uint256 balance = address(this).balance;
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error withdrawing funds");
    }
} 