// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./CarPart.sol";

contract CarNFT is ERC721URIStorage, Ownable {
    struct CarComposition {
        uint256[] partIds;
        string carImageURI;
    }

    mapping(uint256 => CarComposition) private _cars;
    CarPart public carPartContract;
    uint256 private _currentCarId;

    constructor(address _carPartContractAddress) ERC721("CarNFT", "CAR") Ownable(msg.sender) {
        carPartContract = CarPart(_carPartContractAddress);
        _currentCarId = 1;
    }

    function exists(uint256 tokenId) public view returns (bool) {
        return super._ownerOf(tokenId) != address(0);
    }

    struct PartData {
        CarPart.PartType partType;
        uint8 baseSpeed;
        uint8 baseAcceleration;
        uint8 baseHandling;
        uint8 baseDriftFactor;
        uint8 baseTurnFactor;
        uint8 baseMaxSpeed;
        string imageURI;
    }

    function mintCar(string memory carImageURI, PartData[] calldata partsData) external {
        uint256 carId = _currentCarId;
        _safeMint(msg.sender, carId);
        _setTokenURI(carId, carImageURI);

        uint256[] memory partIds = new uint256[](partsData.length);
        for (uint256 i = 0; i < partsData.length; i++) {
            uint256 partId = carPartContract.mint(
                msg.sender,
                partsData[i].partType,
                partsData[i].baseSpeed,
                partsData[i].baseAcceleration,
                partsData[i].baseHandling,
                partsData[i].baseDriftFactor,
                partsData[i].baseTurnFactor,
                partsData[i].baseMaxSpeed,
                partsData[i].imageURI,
                carId
            );
            partIds[i] = partId;
        }

        _cars[carId] = CarComposition({
            partIds: partIds,
            carImageURI: carImageURI
        });

        _currentCarId++;
    }

    function getCarComposition(uint256 carId) public view returns (uint256[] memory partIds, string memory carImageURI) {
        require(exists(carId), "Car does not exist");
        CarComposition storage car = _cars[carId];
        return (car.partIds, car.carImageURI);
    }

    function getCarStats(uint256 _carId) public view returns (
        uint256 totalSpeed,
        uint256 totalAcceleration,
        uint256 totalHandling,
        uint256 totalDriftFactor,
        uint256 totalTurnFactor,
        uint256 totalMaxSpeed
    ) {
        require(exists(_carId), "Car does not exist");
        CarComposition storage comp = _cars[_carId];

        for (uint256 i = 0; i < comp.partIds.length; i++) {
            CarPart.PartStats memory stats = carPartContract.getPartStats(comp.partIds[i]);
            totalSpeed += stats.baseSpeed;
            totalAcceleration += stats.baseAcceleration;
            totalHandling += stats.baseHandling;
            totalDriftFactor += stats.baseDriftFactor;
            totalTurnFactor += stats.baseTurnFactor;
            totalMaxSpeed += stats.baseMaxSpeed;
        }
    }

    function replacePart(uint256 carId, uint256 oldPartId, uint256 newPartId) external {
        require(exists(carId), "Car does not exist");
        require(ownerOf(carId) == msg.sender, "Not the car owner");
        require(carPartContract.ownerOf(newPartId) == msg.sender, "Not the new part owner");

        CarComposition storage car = _cars[carId];
        bool found = false;

        for (uint256 i = 0; i < car.partIds.length; i++) {
            if (car.partIds[i] == oldPartId) {
                car.partIds[i] = newPartId;
                found = true;
                break;
            }
        }

        require(found, "Original part not found in this car");
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

    struct StatCalculation {
        uint256 total;
        uint256 weight;
    }

    function _processPartStats(
        CarPart.PartStats memory stats,
        StatCalculation[6] memory calculations,
        CarPart.PartType partType
    ) private pure {
        uint256 speedMult = 1;
        uint256 accMult = 1;
        uint256 handMult = 1;
        uint256 driftMult = 1;
        uint256 turnMult = 1;
        uint256 maxSpeedMult = 1;

        if (partType == CarPart.PartType.ENGINE) {
            speedMult = 3;
            accMult = 3;
            maxSpeedMult = 3;
        } 
        else if (partType == CarPart.PartType.TRANSMISSION) {
            accMult = 3;
            handMult = 2;
            driftMult = 2;
        }
        else if (partType == CarPart.PartType.WHEELS) {
            handMult = 3;
            driftMult = 3;
            turnMult = 3;
        }

        calculations[0].total += stats.baseSpeed * speedMult;
        calculations[1].total += stats.baseAcceleration * accMult;
        calculations[2].total += stats.baseHandling * handMult;
        calculations[3].total += stats.baseDriftFactor * driftMult;
        calculations[4].total += stats.baseTurnFactor * turnMult;
        calculations[5].total += stats.baseMaxSpeed * maxSpeedMult;

        calculations[0].weight += speedMult;
        calculations[1].weight += accMult;
        calculations[2].weight += handMult;
        calculations[3].weight += driftMult;
        calculations[4].weight += turnMult;
        calculations[5].weight += maxSpeedMult;
    }

    mapping(uint256 => uint8) private _carConditions;
    address public workshopContract;
    address public leaderboardContract;

    function setWorkshopContract(address _workshop) external onlyOwner {
        workshopContract = _workshop;
    }

    function setLeaderboardContract(address _leaderboard) external onlyOwner {
        leaderboardContract = _leaderboard;
    }

    function degradeCar(uint256 carId) external {
        require(msg.sender == leaderboardContract, "Solo el contrato de leaderboard puede degradar carros");
        require(exists(carId), "El carro no existe");
        
        if (_carConditions[carId] == 0) {
            _carConditions[carId] = 100;
        }
        
        if (_carConditions[carId] >= 5) {
            _carConditions[carId] -= 5;
        } else {
            _carConditions[carId] = 0;
        }
    }

    function repairCar(uint256 carId) public {
        require(msg.sender == workshopContract, "Solo el taller puede reparar carros");
        require(exists(carId), "El carro no existe");
        
        _carConditions[carId] = 100;
    }

    function getCompactCarStats(uint256 carId) public view returns (CompactCarStats memory) {
        require(exists(carId), "Car does not exist");
        CarComposition storage comp = _cars[carId];
        
        StatCalculation[6] memory calculations;

        for (uint256 i = 0; i < comp.partIds.length; i++) {
            CarPart.PartStats memory stats = carPartContract.getPartStats(comp.partIds[i]);
            _processPartStats(stats, calculations, stats.partType);
        }

        uint8 condition = _carConditions[carId];
        if (condition == 0) condition = 100;

        uint256 multiplier = condition;
        
        return CompactCarStats({
            imageURI: comp.carImageURI,
            speed: uint8((calculations[0].total * multiplier) / (calculations[0].weight * 100)),
            acceleration: uint8((calculations[1].total * multiplier) / (calculations[1].weight * 100)),
            handling: uint8((calculations[2].total * multiplier) / (calculations[2].weight * 100)),
            driftFactor: uint8((calculations[3].total * multiplier) / (calculations[3].weight * 100)),
            turnFactor: uint8((calculations[4].total * multiplier) / (calculations[4].weight * 100)),
            maxSpeed: uint8((calculations[5].total * multiplier) / (calculations[5].weight * 100)),
            condition: condition
        });
    }
}
