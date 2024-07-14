# Examples

## [erc20]

This example uses the [Steel] view call library to query the ERC20 interface's `balanceOf` method for a specific address within the RISC Zero zkVM.

## [erc20-counter]

This example implements a counter that increments based on off-chain view call proofs submitted to the [Counter] contract.
The contract interacts with ERC-20 tokens, using view call proofs to verify that an account holds at least 1 token before incrementing the counter.
This contract leverages RISC Zero as a [coprocessor] for generating and verifying these proofs.

[erc20]: ./erc20/README.md
[erc20-counter]: ./erc20-counter/README.md
[Counter]: ./erc20-counter/contracts/Counter.sol
[coprocessor]: https://www.risczero.com/news/a-guide-to-zk-coprocessors-for-scalability
[Steel]: ../steel


export BONSAI_API_KEY="SRzMS2QrLC2qbLXJYEixbRonMylAwq32HNFkVIQa"
export BONSAI_API_URL="ttps://api.bonsai.xyz/"
export INFURA_API_KEY="QQODwsG84TuJxALOcgfdd_M5tP1DhKMs"
export ETH_WALLET_PRIVATE_KEY="cdc4b296390c7b7367a7c1055508473a70347919eb5f8c3877727b320cea6535" 
export PRIVATE_KEY="cdc4b296390c7b7367a7c1055508473a70347919eb5f8c3877727b320cea6535" 