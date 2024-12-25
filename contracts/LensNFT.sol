// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract LensNFT is ERC721, Ownable {
    uint256 private _nextTokenId;
    uint256 public MINT_PRICE = 0.01 ether;
    string public baseTokenURI;

    constructor() ERC721("LensNFT", "LNFT") Ownable(msg.sender) {
        baseTokenURI = "";
    }

    function mint() public payable {
        require(msg.value >= MINT_PRICE, "Insuficiente GRASS enviado");
        
        uint256 tokenId = _nextTokenId++;
        _safeMint(msg.sender, tokenId);
    }

    function setMintPrice(uint256 _newPrice) public onlyOwner {
        MINT_PRICE = _newPrice;
    }

    function setBaseURI(string memory _newBaseURI) public onlyOwner {
        baseTokenURI = _newBaseURI;
    }

    function _baseURI() internal view override returns (string memory) {
        return baseTokenURI;
    }

    function withdraw() public onlyOwner {
        uint256 balance = address(this).balance;
        (bool success, ) = payable(owner()).call{value: balance}("");
        require(success, "Error al retirar fondos");
    }
}