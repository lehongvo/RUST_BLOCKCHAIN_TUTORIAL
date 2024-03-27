const { Keypair } = require('@solana/web3.js');
const base58 = require("bs58");
const fs = require('fs');

const PRIVATE_KEY = "XHbEicjZYnEHHUZUyh7d7auHcPFqVjD13s4DMEYVZjJrgSUFaDQiRJ8M8eiCwqDyiaLfjrwQY2W4KhDEFk7SfKM"; // Private key from Phantom
const PUBLIC_KEY = "4TxsVJKRRrfLowoKWjvT4As2TQDuAhWibXxZrQGQcqV9"; // Fill with your address to verify
const secret = base58.decode(PRIVATE_KEY);

// Check if the private key is correct 
const pair = Keypair.fromSecretKey(secret);

if (pair.publicKey.toString() == PUBLIC_KEY) {
    console.log(JSON.stringify(Array.from(secret)))
}
