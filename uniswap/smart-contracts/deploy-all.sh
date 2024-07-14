#Deploy hook contract
forge script script/DeployHook.s.sol:CounterScript \
    --rpc-url https://eth-sepolia.public.blastapi.io \
    --private-key $PRIVATE_KEY \
    --broadcast

#Verification on Blockscout
forge verify-contract --chain-id 11155111 --watch --verifier blockscout --compiler-version v0.8.24 $HOOK_ADDRESS src/DeployHook.sol:DeployHook

#Create Pool
forge script script/CreatePool.s.sol:CreatePoolScript \
    --rpc-url https://eth-sepolia.public.blastapi.io \
    --private-key $PRIVATE_KEY \
    --broadcast

#Add Liquidity
forge script script/AddLiquidity.s.sol:AddLiquidityScript \    --rpc-url https://eth-sepolia.public.blastapi.io \                                                             
    --private-key $PRIVATE_KEY \
    --broadcast

#Swap 
forge script script/Swap.s.sol:SwapScript \    --rpc-url https://eth-sepolia.public.blastapi.io \                                                             
    --private-key $PRIVATE_KEY