// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface ICarWorkshop {
    function repairCar(uint256 carId) external payable;
    function repairPrice() external view returns (uint256);
    function setRepairPrice(uint256 _newPrice) external;
} 