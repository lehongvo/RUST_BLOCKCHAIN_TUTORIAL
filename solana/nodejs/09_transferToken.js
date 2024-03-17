//================================================================
const { clusterApiUrl, Connection, Keypair, PublicKey, LAMPORTS_PER_SOL, Transaction, sendAndConfirmTransaction, SystemProgram } = require("@solana/web3.js");
const { mintTo, transfer, getMint, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction, getAccount, createSyncNativeInstruction, closeAccount, getOrCreateAssociatedTokenAccount} = require('@solana/spl-token');
const { AccountLayout, TOKEN_PROGRAM_ID, NATIVE_MINT } = require("@solana/spl-token");

const dotenv = require("dotenv");
dotenv.config();
//================================================================

//================================================================
const secretKeyEnv = process.env.SECRET_KEY_BYTES;
const RPC = process.env.RPC;
const contractAddress = process.env.TOKEN_CONTRACT;
const tokenAccount = process.env.TOKEN_ACCOUNT;
const toAddress = process.env.TO_ADDRESS;
//================================================================

const transferTokenOnSolana = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);
        const contractAddressKey = new PublicKey(contractAddress);
        const tokenAccountKey = new PublicKey(tokenAccount);
        const toAddressKey = new PublicKey(toAddress);

        const toTokenAccountKey = await getOrCreateAssociatedTokenAccount(
            connection, 
            payer, 
            contractAddressKey, 
            toAddressKey,
        );
        
        const transferTx = await transfer(
            connection,
            payer,
            tokenAccountKey,
            toTokenAccountKey.address,
            payer.publicKey,
            20000000000
        )
        console.log("transferTx", transferTx);
    } catch (error) {
        console.log(error);
    }
}

transferTokenOnSolana();