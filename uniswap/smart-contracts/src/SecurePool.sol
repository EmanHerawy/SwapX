// // SPDX-License-Identifier: BSD-3-Clause-Clear

// pragma solidity ^0.8.20;

// import "fhevm/abstracts/EIP712WithModifier.sol";

// import "fhevm/lib/TFHE.sol";
// import "@hyperlane-xyz/core/contracts/interfaces/IMailbox.sol";

// //TODO add a way to protection againest simulation
// contract SecurePool is EIP712WithModifier {
//     IMailbox public outbox;
//     bytes32 public lastSender;
//     uint32 _destinationDomain;
//     uint32 _sourceDomain;
//     string public lastMessage;

//     event ReceivedMessage(uint32 origin, bytes32 sender, bytes message);
//     event SentMessage(uint32 destinationDomain, address recipient, uint256 message);

//     constructor(address _outbox) EIP712WithModifier("Authorization token", "1") {
//         outbox = IMailbox(_outbox);
//     }

//     // Sets the balance of the owner to the given encrypted balance.
//     // function decryptAmount(euint32 encryptedAmount) public  {
//     //     uint32 amount = TFHE.decrypt(encryptedAmount);
//     //     _sendData (bytes32(uint256(uint160(msg.sender))), abi.encodePacked(amount));
//     //       emit SentMessage(_destinationDomain, msg.sender, amount);

//     // }

//     function _sendData(
//         bytes32 _recipient,
//         bytes memory _message
//     ) private {
//         outbox.dispatch(_destinationDomain, _recipient, _message);
//     }

//     function handle(
//         uint32 _origin,
//         bytes32 _sender,
//         bytes calldata _message
//     ) external {
//        uint32 amount = TFHE.decrypt(TFHE.asEuint32(_message));
//         _sendData (_sender, abi.encodePacked(amount));
//        emit ReceivedMessage(_origin, _sender, _message);
//     }
// /*
//     // Transfers an encrypted amount from the message sender address to the `to` address.
//     function transfer(address to, bytes calldata encryptedAmount) public {
//         transfer(to, TFHE.asEuint32(encryptedAmount));
//     }

//     // Transfers an amount from the message sender address to the `to` address.
//     function transfer(address to, euint32 amount) public {
//         _transfer(msg.sender, to, amount);
//     }

//     // Returns the balance of the caller encrypted under the provided public key.
//     function balanceOf(
//         bytes32 publicKey,
//         bytes calldata signature
//     ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
//         return TFHE.reencrypt(balances[msg.sender], publicKey, 0);
//     }

//     // Sets the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
//     function approve(address spender, bytes calldata encryptedAmount) public {
//         address owner = msg.sender;
//         _approve(owner, spender, TFHE.asEuint32(encryptedAmount));
//     }

//     // Returns the remaining number of tokens that `spender` is allowed to spend
//     // on behalf of the caller. The returned ciphertext is under the caller public FHE key.
//     function allowance(
//         address spender,
//         bytes32 publicKey,
//         bytes calldata signature
//     ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
//         address owner = msg.sender;

//         return TFHE.reencrypt(_allowance(owner, spender), publicKey);
//     }

//     // Transfers `encryptedAmount` tokens using the caller's allowance.
//     function transferFrom(address from, address to, bytes calldata encryptedAmount) public {
//         transferFrom(from, to, TFHE.asEuint32(encryptedAmount));
//     }

//     // Transfers `amount` tokens using the caller's allowance.
//     function transferFrom(address from, address to, euint32 amount) public {
//         address spender = msg.sender;
//         _updateAllowance(from, spender, amount);
//         _transfer(from, to, amount);
//     }

//     function _approve(address owner, address spender, euint32 amount) internal {
//         allowances[owner][spender] = amount;
//     }

//     function _allowance(address owner, address spender) internal view returns (euint32) {
//         if (TFHE.isInitialized(allowances[owner][spender])) {
//             return allowances[owner][spender];
//         } else {
//             return TFHE.asEuint32(0);
//         }
//     }

//     function _updateAllowance(address owner, address spender, euint32 amount) internal {
//         euint32 currentAllowance = _allowance(owner, spender);
//         TFHE.optReq(TFHE.le(amount, currentAllowance));
//         _approve(owner, spender, TFHE.sub(currentAllowance, amount));
//     }

//     // Transfers an encrypted amount.
//     function _transfer(address from, address to, euint32 amount) internal {
//         // Make sure the sender has enough tokens.
//         TFHE.optReq(TFHE.le(amount, balances[from]));

//         // Add to the balance of `to` and subract from the balance of `from`.
//         balances[to] = balances[to] + amount;
//         balances[from] = balances[from] - amount;
//     }

//     modifier onlyContractOwner() {
//         require(msg.sender == contractOwner);
//         _;
//     }*/
// }
