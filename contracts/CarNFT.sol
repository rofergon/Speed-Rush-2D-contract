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
        try this.ownerOf(tokenId) returns (address) {
            return true;
        } catch {
            return false;
        }
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

    function mintCar(
        string memory carImageURI,
        PartData[] calldata partsData
    ) external {
        uint256 carId = _currentCarId;
        
        _safeMint(msg.sender, carId);
        _setTokenURI(carId, carImageURI);
        
        uint256[] memory partIds = new uint256[](partsData.length);
        
        for (uint256 i = 0; i < partsData.length; i++) {
            PartData memory part = partsData[i];
            uint256 partId = carPartContract.mint(
                msg.sender,
                part.partType,
                part.baseSpeed,
                part.baseAcceleration,
                part.baseHandling,
                part.baseDriftFactor,
                part.baseTurnFactor,
                part.baseMaxSpeed,
                part.imageURI,
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

    function getCarStats(uint256 _carId)
        public
        view
        returns (
            uint256 totalSpeed,
            uint256 totalAcceleration,
            uint256 totalHandling,
            uint256 totalDriftFactor,
            uint256 totalTurnFactor,
            uint256 totalMaxSpeed
        )
    {
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

        return (
            totalSpeed,
            totalAcceleration,
            totalHandling,
            totalDriftFactor,
            totalTurnFactor,
            totalMaxSpeed
        );
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
}

