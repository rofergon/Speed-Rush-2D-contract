// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./CarNFT.sol";
import "./CarPart.sol";

contract CarMarketplace is Ownable, ReentrancyGuard {
    CarNFT public carNFT;
    CarPart public carPart;

    // Structure to handle part slots that are sold with the car
    struct PartSlot {
        bool included;    // true if the slot is included in the sale
        uint256 partId;  // ID of the part in that slot (0 if empty)
    }

    // Structure for a car listing
    struct CarListing {
        address seller;
        uint256 carId;
        uint256 price;
        PartSlot[3] partSlots;  // Fixed array of 3 slots (engine, transmission, wheels)
        bool active;
    }

    // Structure for an individual part listing
    struct PartListing {
        address seller;
        uint256 partId;
        uint256 price;
        bool active;
    }

    // Mappings to store listings
    mapping(uint256 => CarListing) public carListings;     // carId => CarListing
    mapping(uint256 => PartListing) public partListings;   // partId => PartListing
    
    // Marketplace fee (in percentage, e.g., 250 = 2.5%)
    uint256 public marketplaceFee = 250;  // 2.5% default

    event CarListed(uint256 indexed carId, address indexed seller, uint256 price, bool[3] slotsIncluded);
    event PartListed(uint256 indexed partId, address indexed seller, uint256 price);
    event CarSold(uint256 indexed carId, address indexed seller, address indexed buyer, uint256 price);
    event PartSold(uint256 indexed partId, address indexed seller, address indexed buyer, uint256 price);
    event ListingCancelled(uint256 indexed itemId, bool isCar);
    event MarketplaceFeeUpdated(uint256 newFee);

    constructor(address _carNFT, address _carPart) {
        carNFT = CarNFT(_carNFT);
        carPart = CarPart(_carPart);
        _transferOwnership(msg.sender);
    }

    // Function to list a car with selected parts
    function listCar(
        uint256 carId,
        uint256 price,
        bool[3] memory includeSlots
    ) external {
        require(carNFT.ownerOf(carId) == msg.sender, "Not the car owner");
        require(price > 0, "Price must be greater than 0");
        require(carNFT.isApprovedForAll(msg.sender, address(this)) || 
                carNFT.getApproved(carId) == address(this), 
                "Marketplace not authorized for car");

        // Get current car composition
        (uint256[] memory partIds, , bool[] memory slotOccupied) = carNFT.getCarComposition(carId);

        PartSlot[3] memory slots;
        for (uint256 i = 0; i < 3; i++) {
            if (includeSlots[i]) {
                require(slotOccupied[i], "Cannot include empty slot");
                require(carPart.ownerOf(partIds[i]) == msg.sender, "Not owner of all parts");
                require(carPart.isApprovedForAll(msg.sender, address(this)) || 
                        carPart.getApproved(partIds[i]) == address(this), 
                        "Marketplace not authorized for parts");
                slots[i] = PartSlot({
                    included: true,
                    partId: partIds[i]
                });
            }
        }

        carListings[carId] = CarListing({
            seller: msg.sender,
            carId: carId,
            price: price,
            partSlots: slots,
            active: true
        });

        emit CarListed(carId, msg.sender, price, includeSlots);
    }

    // Function to list an individual part
    function listPart(uint256 partId, uint256 price) external {
        require(carPart.ownerOf(partId) == msg.sender, "Not the part owner");
        require(price > 0, "Price must be greater than 0");
        require(!carPart.isEquipped(partId), "Part is equipped in a car");

        partListings[partId] = PartListing({
            seller: msg.sender,
            partId: partId,
            price: price,
            active: true
        });

        emit PartListed(partId, msg.sender, price);
    }

    // Function to buy a listed car
    function buyCar(uint256 carId) external payable nonReentrant {
        CarListing storage listing = carListings[carId];
        require(listing.active, "Listing not active");
        require(msg.value >= listing.price, "Insufficient payment");

        address seller = listing.seller;
        require(seller != msg.sender, "Cannot buy your own listing");

        // Verify seller still owns car and parts
        require(carNFT.ownerOf(carId) == seller, "Seller no longer owns the car");
        
        for (uint256 i = 0; i < 3; i++) {
            if (listing.partSlots[i].included) {
                require(carPart.ownerOf(listing.partSlots[i].partId) == seller, 
                    "Seller no longer owns all parts");
            }
        }

        // Calculate fee and payment to seller
        uint256 fee = (listing.price * marketplaceFee) / 10000;
        uint256 payment = listing.price - fee;

        // Transfer NFTs
        carNFT.transferFrom(seller, msg.sender, carId);
        for (uint256 i = 0; i < 3; i++) {
            if (listing.partSlots[i].included) {
                carPart.transferFrom(seller, msg.sender, listing.partSlots[i].partId);
            }
        }

        // Transfer payments
        (bool success, ) = payable(seller).call{value: payment}("");
        require(success, "Error sending payment to seller");

        // Deactivate listing
        listing.active = false;

        emit CarSold(carId, seller, msg.sender, listing.price);
    }

    // Function to buy a listed part
    function buyPart(uint256 partId) external payable nonReentrant {
        PartListing storage listing = partListings[partId];
        require(listing.active, "Listing not active");
        require(msg.value >= listing.price, "Insufficient payment");

        address seller = listing.seller;
        require(seller != msg.sender, "Cannot buy your own listing");
        require(carPart.ownerOf(partId) == seller, "Seller no longer owns the part");

        // Calculate fee and payment to seller
        uint256 fee = (listing.price * marketplaceFee) / 10000;
        uint256 payment = listing.price - fee;

        // Transfer NFT
        carPart.transferFrom(seller, msg.sender, partId);

        // Transfer payment
        (bool success, ) = payable(seller).call{value: payment}("");
        require(success, "Error sending payment to seller");

        // Deactivate listing
        listing.active = false;

        emit PartSold(partId, seller, msg.sender, listing.price);
    }

    // Function to cancel a car listing
    function cancelCarListing(uint256 carId) external {
        CarListing storage listing = carListings[carId];
        require(listing.active, "Listing not active");
        require(listing.seller == msg.sender, "Not the seller");

        listing.active = false;
        emit ListingCancelled(carId, true);
    }

    // Function to cancel a part listing
    function cancelPartListing(uint256 partId) external {
        PartListing storage listing = partListings[partId];
        require(listing.active, "Listing not active");
        require(listing.seller == msg.sender, "Not the seller");

        listing.active = false;
        emit ListingCancelled(partId, false);
    }

    // Function to update marketplace fee (owner only)
    function setMarketplaceFee(uint256 newFee) external onlyOwner {
        require(newFee <= 1000, "Maximum fee is 10%");
        marketplaceFee = newFee;
        emit MarketplaceFeeUpdated(newFee);
    }

    // Function to withdraw accumulated fees (owner only)
    function withdrawFees() external onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No funds to withdraw");
        
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error withdrawing funds");
    }

    // Helper function to check approvals
    function checkCarApproval(address owner, uint256 carId) public view returns (bool) {
        return carNFT.isApprovedForAll(owner, address(this)) || 
               carNFT.getApproved(carId) == address(this);
    }

    function checkPartApproval(address owner, uint256 partId) public view returns (bool) {
        return carPart.isApprovedForAll(owner, address(this)) || 
               carPart.getApproved(partId) == address(this);
    }

    // Function to get approval status for a car and its parts
    function getListingApprovalStatus(uint256 carId, bool[3] memory includeSlots) external view returns (
        bool carApproved,
        bool[] memory partsApproved
    ) {
        address owner = carNFT.ownerOf(carId);
        carApproved = checkCarApproval(owner, carId);
        
        (uint256[] memory partIds, , bool[] memory slotOccupied) = carNFT.getCarComposition(carId);
        partsApproved = new bool[](3);
        
        for (uint256 i = 0; i < 3; i++) {
            if (includeSlots[i] && slotOccupied[i]) {
                partsApproved[i] = checkPartApproval(owner, partIds[i]);
            }
        }
        
        return (carApproved, partsApproved);
    }
} 