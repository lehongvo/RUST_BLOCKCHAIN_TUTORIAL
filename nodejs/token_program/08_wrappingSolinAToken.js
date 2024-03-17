//================================================================
const { clusterApiUrl, Connection, Keypair, PublicKey, LAMPORTS_PER_SOL, Transaction, sendAndConfirmTransaction, SystemProgram } = require("@solana/web3.js");
const { mintTo, transfer, getMint, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction, getAccount, createSyncNativeInstruction, closeAccount} = require('@solana/spl-token');
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

const associatedWrappingSolinAToken = async () => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);
        const contractAddressKey = new PublicKey(contractAddress);
        const tokenAccountKey = new PublicKey(tokenAccount);

        const associatedTokenAccount = await getAssociatedTokenAddress(
            NATIVE_MINT,
            payer.publicKey
        )

        // Create token account to hold your wrapped SOL
        // const ataTransaction = new Transaction().add(
        //     createAssociatedTokenAccountInstruction(
        //         payer.publicKey,
        //         associatedTokenAccount,
        //         payer.publicKey,
        //         NATIVE_MINT
        //     )
        // )
        // const sendAndConfirmTransactionTx = await sendAndConfirmTransaction(
        //     connection,
        //     ataTransaction,
        //     [payer]
        // )
        // console.log("sendAndConfirmTransactionTx", sendAndConfirmTransactionTx);
        const accountInfo = await getAccount(connection, associatedTokenAccount);
        console.log(``)
    } catch (error) {
        console.log(error);
    }
}

// associatedWrappingSolinAToken();

const WrappingSolInAToken = async () => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);

        const associatedTokenAccount = await getAssociatedTokenAddress(
            NATIVE_MINT,
            payer.publicKey
        )
        
        const solTransferTransaction = new Transaction().add(
            SystemProgram.transfer ({
                fromPubkey: payer.publicKey,
                toPubkey: associatedTokenAccount,
                lamports: 10000000000
            }),
            createSyncNativeInstruction(
                associatedTokenAccount
            )
        );
        const solTransferTransactionTx = await sendAndConfirmTransaction(
            connection,
            solTransferTransaction,
            [payer]
        )
        console.log("solTransferTransactionTx", solTransferTransactionTx);
    } catch (error) {
        console.log(error);
    }
}

// WrappingSolInAToken();

const getBalance = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);

        const solBalance = await connection.getBalance(payer.publicKey);
        console.log("getBalance", Number(solBalance) / 1e9);

    } catch (error) {
        console.log(error);
    }
}

const closeAccount = async(connection, wallet, associatedTokenAccount, destinationPublicKey, payer) => {
    

    await sendAndConfirmTransaction(
        connection,
        closeAccountTransaction,
        [payer],
        {
            commitment: 'singleGossip',
            preflightCommitment: 'singleGossip',
        }
    );
}


const unwrapSolInAToken = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);

        const associatedTokenAccount = await getAssociatedTokenAddress(
            NATIVE_MINT,
            payer.publicKey
        )


    } catch (error) {
        console.log(error);   
    }
}
unwrapSolInAToken();