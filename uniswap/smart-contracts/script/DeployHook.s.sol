// // SPDX-License-Identifier: MIT
// pragma solidity ^0.8.19;

// import "forge-std/Script.sol";
// import {Hooks} from "v4-core/src/libraries/Hooks.sol";
// import {PoolManager} from "v4-core/src/PoolManager.sol";
// import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
// import {PoolModifyLiquidityTest} from "v4-core/src/test/PoolModifyLiquidityTest.sol";
// import {PoolSwapTest} from "v4-core/src/test/PoolSwapTest.sol";
// import {PoolDonateTest} from "v4-core/src/test/PoolDonateTest.sol";
// import {GasPriceFeesHook} from "../src/GasPriceFeesHook.sol";
// import {HookMiner} from "../test/utils/HookMiner.sol";

// contract DeployHookScript is Script {
//     address constant CREATE2_DEPLOYER = address(0x4e59b44847b379578588920cA78FbF26c0B4956C);
//     address constant SEPOLIA_POOLMANAGER = address(0xFf34e285F8ED393E366046153e3C16484A4dD674);

//     function setUp() public {}

//     function run() public {
//         // hook contracts must have specific flags encoded in the address
//         uint160 flags = uint160(
//             Hooks.BEFORE_SWAP_FLAG | Hooks.AFTER_SWAP_FLAG | Hooks.BEFORE_ADD_LIQUIDITY_FLAG
//                 | Hooks.BEFORE_REMOVE_LIQUIDITY_FLAG
//         );

//         // Mine a salt that will produce a hook address with the correct flags
//         (address hookAddress, bytes32 salt) =
//             HookMiner.find(CREATE2_DEPLOYER, flags, type(GasPriceFeesHook).creationCode, abi.encode(address(SEPOLIA_POOLMANAGER)));

//         // Deploy the hook using CREATE2
//         vm.broadcast();
//         GasPriceFeesHook gasPriceFeesHook = new GasPriceFeesHook{salt: salt}(IPoolManager(address(SEPOLIA_POOLMANAGER)));
//         require(address(gasPriceFeesHook) == hookAddress, "DeployHookScript: hook address mismatch");
//     }
// }