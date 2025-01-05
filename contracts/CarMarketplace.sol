// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./CarNFT.sol";
import "./CarPart.sol";

contract CarMarketplace is Ownable, ReentrancyGuard {
    CarNFT public carNFT;
    CarPart public carPart;

    // Estructura para manejar los slots de partes que se venden con el carro
    struct PartSlot {
        bool included;    // true si el slot se vende con el carro
        uint256 partId;  // ID de la parte en ese slot (0 si no hay parte)
    }

    // Estructura para un listado de carro
    struct CarListing {
        address seller;
        uint256 carId;
        uint256 price;
        PartSlot[3] partSlots;  // Array fijo de 3 slots (motor, transmisión, ruedas)
        bool active;
    }

    // Estructura para un listado de parte individual
    struct PartListing {
        address seller;
        uint256 partId;
        uint256 price;
        bool active;
    }

    // Mappings para almacenar los listados
    mapping(uint256 => CarListing) public carListings;     // carId => CarListing
    mapping(uint256 => PartListing) public partListings;   // partId => PartListing
    
    // Comisión del marketplace (en porcentaje, ej: 250 = 2.5%)
    uint256 public marketplaceFee = 250;  // 2.5% por defecto

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

    // Función para listar un carro con sus partes seleccionadas
    function listCar(
        uint256 carId,
        uint256 price,
        bool[3] memory includeSlots
    ) external {
        require(carNFT.ownerOf(carId) == msg.sender, "No eres el dueno del carro");
        require(price > 0, "El precio debe ser mayor a 0");
        require(carNFT.isApprovedForAll(msg.sender, address(this)) || 
                carNFT.getApproved(carId) == address(this), 
                "El marketplace no tiene autorizacion para el carro");

        // Obtener la composición actual del carro
        (uint256[] memory partIds, , bool[] memory slotOccupied) = carNFT.getCarComposition(carId);

        PartSlot[3] memory slots;
        for (uint256 i = 0; i < 3; i++) {
            if (includeSlots[i]) {
                require(slotOccupied[i], "No puedes incluir un slot vacio");
                require(carPart.ownerOf(partIds[i]) == msg.sender, "No eres dueno de todas las partes");
                require(carPart.isApprovedForAll(msg.sender, address(this)) || 
                        carPart.getApproved(partIds[i]) == address(this), 
                        "El marketplace no tiene autorizacion para las partes");
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

    // Función para listar una parte individual
    function listPart(uint256 partId, uint256 price) external {
        require(carPart.ownerOf(partId) == msg.sender, "No eres el dueno de la parte");
        require(price > 0, "El precio debe ser mayor a 0");
        require(!carPart.isEquipped(partId), "La parte esta equipada en un carro");

        partListings[partId] = PartListing({
            seller: msg.sender,
            partId: partId,
            price: price,
            active: true
        });

        emit PartListed(partId, msg.sender, price);
    }

    // Función para comprar un carro listado
    function buyCar(uint256 carId) external payable nonReentrant {
        CarListing storage listing = carListings[carId];
        require(listing.active, "Listado no activo");
        require(msg.value >= listing.price, "Pago insuficiente");

        address seller = listing.seller;
        require(seller != msg.sender, "No puedes comprar tu propio listado");

        // Verificar que el vendedor aún sea dueño del carro y las partes
        require(carNFT.ownerOf(carId) == seller, "El vendedor ya no es dueno del carro");
        
        for (uint256 i = 0; i < 3; i++) {
            if (listing.partSlots[i].included) {
                require(carPart.ownerOf(listing.partSlots[i].partId) == seller, 
                    "El vendedor ya no es dueno de todas las partes");
            }
        }

        // Calcular comisión y pago al vendedor
        uint256 fee = (listing.price * marketplaceFee) / 10000;
        uint256 payment = listing.price - fee;

        // Transferir NFTs
        carNFT.transferFrom(seller, msg.sender, carId);
        for (uint256 i = 0; i < 3; i++) {
            if (listing.partSlots[i].included) {
                carPart.transferFrom(seller, msg.sender, listing.partSlots[i].partId);
            }
        }

        // Transferir pagos
        (bool success, ) = payable(seller).call{value: payment}("");
        require(success, "Error al enviar el pago al vendedor");

        // Desactivar listado
        listing.active = false;

        emit CarSold(carId, seller, msg.sender, listing.price);
    }

    // Función para comprar una parte listada
    function buyPart(uint256 partId) external payable nonReentrant {
        PartListing storage listing = partListings[partId];
        require(listing.active, "Listado no activo");
        require(msg.value >= listing.price, "Pago insuficiente");

        address seller = listing.seller;
        require(seller != msg.sender, "No puedes comprar tu propio listado");
        require(carPart.ownerOf(partId) == seller, "El vendedor ya no es dueno de la parte");

        // Calcular comisión y pago al vendedor
        uint256 fee = (listing.price * marketplaceFee) / 10000;
        uint256 payment = listing.price - fee;

        // Transferir NFT
        carPart.transferFrom(seller, msg.sender, partId);

        // Transferir pago
        (bool success, ) = payable(seller).call{value: payment}("");
        require(success, "Error al enviar el pago al vendedor");

        // Desactivar listado
        listing.active = false;

        emit PartSold(partId, seller, msg.sender, listing.price);
    }

    // Función para cancelar un listado de carro
    function cancelCarListing(uint256 carId) external {
        CarListing storage listing = carListings[carId];
        require(listing.active, "Listado no activo");
        require(listing.seller == msg.sender, "No eres el vendedor");

        listing.active = false;
        emit ListingCancelled(carId, true);
    }

    // Función para cancelar un listado de parte
    function cancelPartListing(uint256 partId) external {
        PartListing storage listing = partListings[partId];
        require(listing.active, "Listado no activo");
        require(listing.seller == msg.sender, "No eres el vendedor");

        listing.active = false;
        emit ListingCancelled(partId, false);
    }

    // Función para actualizar la comisión del marketplace (solo owner)
    function setMarketplaceFee(uint256 newFee) external onlyOwner {
        require(newFee <= 1000, "Comision maxima 10%");
        marketplaceFee = newFee;
        emit MarketplaceFeeUpdated(newFee);
    }

    // Función para retirar las comisiones acumuladas (solo owner)
    function withdrawFees() external onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No hay fondos para retirar");
        
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error al retirar fondos");
    }

    // Función auxiliar para verificar aprobaciones
    function checkCarApproval(address owner, uint256 carId) public view returns (bool) {
        return carNFT.isApprovedForAll(owner, address(this)) || 
               carNFT.getApproved(carId) == address(this);
    }

    function checkPartApproval(address owner, uint256 partId) public view returns (bool) {
        return carPart.isApprovedForAll(owner, address(this)) || 
               carPart.getApproved(partId) == address(this);
    }

    // Función para obtener el estado de aprobación de un carro y sus partes
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