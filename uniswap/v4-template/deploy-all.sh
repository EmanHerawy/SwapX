#Deploy hook contract
forge script script/00_Counter.s.sol:CounterScript \
    --rpc-url https://eth-sepolia.public.blastapi.io \
    --private-key $PRIVATE_KEY \
    --broadcast

#Deploy Mock currency token contracts 
forge create --rpc-url https://eth-sepolia.public.blastapi.io --private-key $PRIVATE_KEY src/MockUSDT.sol:MockUSDT
forge create --rpc-url https://eth-sepolia.public.blastapi.io --private-key $PRIVATE_KEY src/MockUSDC.sol:MockUSDC

#Verification on Blockscout
forge verify-contract --chain-id 11155111 --watch --verifier blockscout --compiler-version v0.8.24 0x3b39F2783760a2B0D318995fE20dB587e55cADdc src/MockUSDT.sol:MockUSDT

#Create Pool
forge script script/01_CreatePool.s.sol:CreatePoolScript \
    --rpc-url https://eth-sepolia.public.blastapi.io \
    --private-key $PRIVATE_KEY \
    --broadcast

#Add Liquidity
forge script script/02_AddLiquidity.s.sol:AddLiquidityScript \    --rpc-url https://eth-sepolia.public.blastapi.io \                                                             
    --private-key $PRIVATE_KEY \
    --broadcast

#Swap 
forge script script/03_Swap.s.sol:SwapScript \    --rpc-url https://eth-sepolia.public.blastapi.io \                                                             
    --private-key $PRIVATE_KEY