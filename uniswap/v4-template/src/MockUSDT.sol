// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDT is ERC20 {
    constructor() ERC20("Mock USDT", "mUSDT") {
        _mint(msg.sender, 1000000 * (10 ** uint256(decimals()))); // Mint 1 million mock UNI to deployer
    }

    function faucet(uint256 quantity) external {
        _mint(msg.sender, quantity * 10 ** 6);
    }
}