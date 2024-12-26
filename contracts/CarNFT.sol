// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./CarPart.sol";

contract CarNFT is ERC721, Ownable {
    struct CarComposition {
        uint256[] partIds;
        string carImageURI;
    }

    mapping(uint256 => CarComposition) private _cars;
    CarPart public carPartContract;
    uint256 private _currentCarId;
    mapping(uint256 => uint8) private _carConditions;
    address public workshopContract;
    address public leaderboardContract;

    event CarMinted(uint256 indexed carId, address indexed owner);
    event PartReplaced(uint256 indexed carId, uint256 oldPartId, uint256 newPartId);
    event WorkshopSet(address indexed workshop);
    event LeaderboardSet(address indexed leaderboard);
    event MintPriceChanged(uint256 newPrice);

    uint256 public mintPrice;

    constructor(address _carPartContractAddress) ERC721("CarNFT", "CAR") Ownable(msg.sender) {
        carPartContract = CarPart(_carPartContractAddress);
        _currentCarId = 1;
        mintPrice = 0.01 ether; // Initial minting price
    }

    function setMintPrice(uint256 _newPrice) external onlyOwner {
        mintPrice = _newPrice;
        emit MintPriceChanged(_newPrice);
    }

    function withdrawFunds() external onlyOwner {
        uint256 balance = address(this).balance;
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error withdrawing funds");
    }

    struct PartData {
        CarPart.PartType partType;
        uint8 stat1;
        uint8 stat2;
        uint8 stat3;
        string imageURI;
    }

    function mintCar(string memory carImageURI, PartData[] calldata partsData) external payable {
        require(msg.value >= mintPrice, "Insufficient payment for minting");
        require(partsData.length == 3, "A car must have exactly 3 parts");
        
        uint256 carId = _currentCarId;
        _safeMint(msg.sender, carId);
        
        uint256[] memory partIds = new uint256[](partsData.length);
        
        // Verify we have one part of each type
        bool hasEngine = false;
        bool hasTransmission = false;
        bool hasWheels = false;

        for (uint256 i = 0; i < partsData.length; i++) {
            if (partsData[i].partType == CarPart.PartType.ENGINE) hasEngine = true;
            else if (partsData[i].partType == CarPart.PartType.TRANSMISSION) hasTransmission = true;
            else if (partsData[i].partType == CarPart.PartType.WHEELS) hasWheels = true;

            uint256 partId = carPartContract.mint(
                msg.sender,
                partsData[i].partType,
                partsData[i].stat1,
                partsData[i].stat2,
                partsData[i].stat3,
                partsData[i].imageURI,
                carId
            );
            partIds[i] = partId;
        }

        require(hasEngine && hasTransmission && hasWheels, "Car must have an engine, transmission, and wheels");

        _cars[carId] = CarComposition({
            partIds: partIds,
            carImageURI: carImageURI
        });

        _carConditions[carId] = 100; // New car, perfect condition
        emit CarMinted(carId, msg.sender);
        _currentCarId++;
    }

    function getCarComposition(uint256 carId) external view returns (uint256[] memory partIds, string memory carImageURI) {
        require(_ownerOf(carId) != address(0), "Car does not exist");
        CarComposition storage car = _cars[carId];
        return (car.partIds, car.carImageURI);
    }

    function replacePart(uint256 carId, uint256 oldPartId, uint256 newPartId) external {
        require(_ownerOf(carId) != address(0), "Car does not exist");
        require(ownerOf(carId) == msg.sender, "Not the car owner");
        require(carPartContract.ownerOf(newPartId) == msg.sender, "Not the owner of the new part");

        CarComposition storage car = _cars[carId];
        bool found = false;
        uint256 oldPartIndex;

        for (uint256 i = 0; i < car.partIds.length; i++) {
            if (car.partIds[i] == oldPartId) {
                oldPartIndex = i;
                found = true;
                break;
            }
        }

        require(found, "Original part not found in this car");

        // Verify that the new part is of the same type as the old one
        CarPart.PartType oldType = carPartContract.getPartType(oldPartId);
        CarPart.PartType newType = carPartContract.getPartType(newPartId);
        require(oldType == newType, "New part must be of the same type as the original");

        car.partIds[oldPartIndex] = newPartId;
        emit PartReplaced(carId, oldPartId, newPartId);
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

    struct FullCarMetadata {
        // Basic car data
        uint256 carId;
        address owner;
        string carImageURI;
        uint8 condition;
        
        // Processed car stats
        CarStats combinedStats;
        
        // Detailed part information
        PartFullMetadata[] parts;
    }

    struct CarStats {
        uint8 speed;
        uint8 acceleration;
        uint8 handling;
        uint8 driftFactor;
        uint8 turnFactor;
        uint8 maxSpeed;
    }

    struct PartFullMetadata {
        uint256 partId;
        CarPart.PartType partType;
        string imageURI;
        PartTypeStats stats;
    }

    struct PartTypeStats {
        // Engine
        uint8 speed;
        uint8 maxSpeed;
        uint8 acceleration;
        // Transmission
        uint8 transmissionAcceleration;
        uint8 transmissionSpeed;
        uint8 transmissionHandling;
        // Wheels
        uint8 handling;
        uint8 driftFactor;
        uint8 turnFactor;
    }

    function getLastTokenId() public view returns (uint256) {
        return _currentCarId - 1;
    }

    function getFullCarMetadata(uint256 carId) external view returns (FullCarMetadata memory) {
        require(_ownerOf(carId) != address(0), "Car does not exist");
        
        // Get basic car composition
        CarComposition storage car = _cars[carId];
        
        // Get processed stats
        CompactCarStats memory compactStats = this.getCompactCarStats(carId);
        
        // Prepare parts array
        PartFullMetadata[] memory parts = new PartFullMetadata[](car.partIds.length);
        
        // Get detailed information for each part
        for (uint256 i = 0; i < car.partIds.length; i++) {
            uint256 partId = car.partIds[i];
            CarPart.PartStats memory partStats = carPartContract.getPartStats(partId);
            
            // Initialize type-specific stats
            PartTypeStats memory typeStats;
            
            if (partStats.partType == CarPart.PartType.ENGINE) {
                typeStats.speed = partStats.stat1;
                typeStats.maxSpeed = partStats.stat2;
                typeStats.acceleration = partStats.stat3;
            } else if (partStats.partType == CarPart.PartType.TRANSMISSION) {
                typeStats.transmissionAcceleration = partStats.stat1;
                typeStats.transmissionSpeed = partStats.stat2;
                typeStats.transmissionHandling = partStats.stat3;
            } else { // WHEELS
                typeStats.handling = partStats.stat1;
                typeStats.driftFactor = partStats.stat2;
                typeStats.turnFactor = partStats.stat3;
            }
            
            parts[i] = PartFullMetadata({
                partId: partId,
                partType: partStats.partType,
                imageURI: partStats.imageURI,
                stats: typeStats
            });
        }

        // Create combined stats structure
        CarStats memory combinedStats = CarStats({
            speed: compactStats.speed,
            acceleration: compactStats.acceleration,
            handling: compactStats.handling,
            driftFactor: compactStats.driftFactor,
            turnFactor: compactStats.turnFactor,
            maxSpeed: compactStats.maxSpeed
        });
        
        return FullCarMetadata({
            carId: carId,
            owner: ownerOf(carId),
            carImageURI: car.carImageURI,
            condition: compactStats.condition,
            combinedStats: combinedStats,
            parts: parts
        });
    }

    function getCompactCarStats(uint256 carId) external view returns (CompactCarStats memory) {
        require(_ownerOf(carId) != address(0), "Car does not exist");
        CarComposition storage car = _cars[carId];

        uint256 totalSpeed;
        uint256 totalAcceleration;
        uint256 totalHandling;
        uint256 totalDriftFactor;
        uint256 totalTurnFactor;
        uint256 totalMaxSpeed;

        uint256 speedContributors = 0;
        uint256 accelerationContributors = 0;
        uint256 handlingContributors = 0;
        uint256 driftContributors = 0;
        uint256 turnContributors = 0;
        uint256 maxSpeedContributors = 0;

        for (uint256 i = 0; i < car.partIds.length; i++) {
            (
                uint8 baseSpeed,
                uint8 baseAcceleration,
                uint8 baseHandling,
                uint8 baseDriftFactor,
                uint8 baseTurnFactor,
                uint8 baseMaxSpeed
            ) = carPartContract.convertToLegacyStats(car.partIds[i]);

            if (baseSpeed > 1) {
                totalSpeed += baseSpeed;
                speedContributors++;
            }
            if (baseAcceleration > 1) {
                totalAcceleration += baseAcceleration;
                accelerationContributors++;
            }
            if (baseHandling > 1) {
                totalHandling += baseHandling;
                handlingContributors++;
            }
            if (baseDriftFactor > 1) {
                totalDriftFactor += baseDriftFactor;
                driftContributors++;
            }
            if (baseTurnFactor > 1) {
                totalTurnFactor += baseTurnFactor;
                turnContributors++;
            }
            if (baseMaxSpeed > 1) {
                totalMaxSpeed += baseMaxSpeed;
                maxSpeedContributors++;
            }
        }

        uint8 condition = _carConditions[carId];
        if (condition == 0) condition = 100;

        uint256 multiplier = condition;

        // Adjust divisor based on number of contributors for each stat
        return CompactCarStats({
            imageURI: car.carImageURI,
            speed: uint8((totalSpeed * multiplier) / (speedContributors * 100)),
            acceleration: uint8((totalAcceleration * multiplier) / (accelerationContributors * 100)),
            handling: uint8((totalHandling * multiplier) / (handlingContributors * 100)),
            driftFactor: uint8((totalDriftFactor * multiplier) / (driftContributors * 100)),
            turnFactor: uint8((totalTurnFactor * multiplier) / (turnContributors * 100)),
            maxSpeed: uint8((totalMaxSpeed * multiplier) / (maxSpeedContributors * 100)),
            condition: condition
        });
    }

    function setWorkshopContract(address _workshop) external onlyOwner {
        workshopContract = _workshop;
        emit WorkshopSet(_workshop);
    }

    function setLeaderboardContract(address _leaderboard) external onlyOwner {
        leaderboardContract = _leaderboard;
        emit LeaderboardSet(_leaderboard);
    }

    function degradeCar(uint256 carId) external {
        require(msg.sender == leaderboardContract, "Only leaderboard contract can degrade cars");
        require(_ownerOf(carId) != address(0), "Car does not exist");
        
        if (_carConditions[carId] == 0) {
            _carConditions[carId] = 100;
        }
        
        if (_carConditions[carId] >= 5) {
            _carConditions[carId] -= 5;
        } else {
            _carConditions[carId] = 0;
        }
    }

    function repairCar(uint256 carId) external {
        require(msg.sender == workshopContract, "Only workshop can repair cars");
        require(_ownerOf(carId) != address(0), "Car does not exist");
        _carConditions[carId] = 100;
    }

    function exists(uint256 carId) public view returns (bool) {
        return _ownerOf(carId) != address(0);
    }
}
