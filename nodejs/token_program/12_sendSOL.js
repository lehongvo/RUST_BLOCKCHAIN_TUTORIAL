//================================================================
const { clusterApiUrl, Connection, Keypair, PublicKey, Transaction, sendAndConfirmTransaction, SystemProgram } = require("@solana/web3.js");
const { transfer, getOrCreateAssociatedTokenAccount, getAssociatedTokenAddress, createBurnInstruction} = require('@solana/spl-token');
const { AccountLayout, TOKEN_PROGRAM_ID, NATIVE_MINT,  } = require("@solana/spl-token");

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

const sendSOl = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);
        const contractAddressKey = new PublicKey(contractAddress);
        const tokenAccountKey = new PublicKey(tokenAccount);

        const transaction= new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: payer.publicKey,
                toPubkey: toAddress,
                lamports: 11000000000
            })
        )
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [payer]
        );
        console.log("signature", signature);
    } catch (error) {
        console.log(error);
    }
}

sendSOl();