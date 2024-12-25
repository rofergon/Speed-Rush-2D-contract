// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CarNFT is ERC721, Ownable {
    uint256 private _tokenIdCounter;

    constructor() ERC721("CarNFT", "CAR") Ownable(msg.sender) {}

    function mintCarNFT(address to) external onlyOwner returns (uint256) {
        _tokenIdCounter++;
        uint256 newItemId = _tokenIdCounter;

        _safeMint(to, newItemId);
        return newItemId;
    }
}
