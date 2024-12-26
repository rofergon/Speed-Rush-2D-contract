// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./ICarNFT.sol";

contract CarWorkshop is Ownable {
    ICarNFT public carNFT;
    uint256 public repairPrice;
    
    event CarRepaired(uint256 indexed carId);
    event RepairPriceChanged(uint256 newPrice);
    
    constructor(address _carNFT, uint256 _repairPrice) Ownable(msg.sender) {
        carNFT = ICarNFT(_carNFT);
        repairPrice = _repairPrice;
    }
    
    function setRepairPrice(uint256 _newPrice) external onlyOwner {
        repairPrice = _newPrice;
        emit RepairPriceChanged(_newPrice);
    }
    
    function repairCar(uint256 carId) external payable {
        require(msg.value >= repairPrice, "Pago insuficiente");
        require(carNFT.ownerOf(carId) == msg.sender, "No eres el dueno del carro");
        
        carNFT.repairCar(carId);
        
        // Devolver el exceso de pago usando call
        if (msg.value > repairPrice) {
            (bool success, ) = payable(msg.sender).call{value: msg.value - repairPrice}("");
            require(success, "Error al devolver el exceso de pago");
        }

        emit CarRepaired(carId);
    }
    
    function withdrawFunds() external onlyOwner {
        uint256 balance = address(this).balance;
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error al retirar los fondos");
    }
} 