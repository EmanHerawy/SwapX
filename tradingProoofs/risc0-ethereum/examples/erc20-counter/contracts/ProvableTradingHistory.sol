// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {Steel} from "risc0/steel/Steel.sol";
import {IProvableTradingHistory} from "./IProvableTradingHistory.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.


contract ProvableTradingHistory  is IProvableTradingHistory {
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.BALANCE_OF_ID;

    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    /// @notice Address of the ERC-20 token contract.
    address public immutable tokenAddress;


    /// @notice Journal that is committed to by the guest.
    struct Journal {
        Steel.Commitment commitment;
        address tokenAddress;

    }

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier and ERC-20 token address.
    constructor(IRiscZeroVerifier _verifier, address _tokenAddress) {
        verifier = _verifier;
        tokenAddress = _tokenAddress;
     }

    // function increment(bytes calldata journalData, bytes calldata seal) external {
    //     // Decode and validate the journal data
    //     Journal memory journal = abi.decode(journalData, (Journal));
    //     require(journal.tokenAddress == tokenAddress, "Invalid token address");
    //     require(Steel.validateCommitment(journal.commitment), "Invalid commitment");

    //     // Verify the proof
    //     bytes32 journalHash = sha256(journalData);
    //     verifier.verify(seal, imageId, journalHash);

    //     counter += 1;
    // }

       function submitProof(bytes calldata journalData, bytes calldata seal) external {
        // Decode and validate the journal data
        Journal memory journal = abi.decode(journalData, (Journal));
        require(journal.tokenAddress == tokenAddress, "Invalid token address");
        require(Steel.validateCommitment(journal.commitment), "Invalid commitment");

        // Verify the proof
        bytes32 journalHash = sha256(journalData);
        verifier.verify(seal, imageId, journalHash);
        //TODO not secure, need to prevent replay attack and more
       // perform swap 
    }

   
}
