//================================================================
const {
    clusterApiUrl,
    Connection,
    Keypair,
    PublicKey,
    LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

const dotenv = require("dotenv");
dotenv.config();
const { 
    getOrCreateAssociatedTokenAccount,
} = require('@solana/spl-token');
//================================================================

//================================================================
const secretKeyEnv = process.env.SECRET_KEY_BYTES;
const RPC = process.env.RPC;
const contractAddress = process.env.TOKEN_CONTRACT;
//================================================================
// please code get supply here
const orCreateAssociatedTokenAccount = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const payer = Keypair.fromSecretKey(secretKey);
        
        const connection = new Connection(RPC, "confirmed");
        const mint = new PublicKey(contractAddress);
        
        const tokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            mint,
            payer.publicKey
        )
        console.log("Address", tokenAccount.address);
    } catch (error) {
        console.error(error);
    }
}

orCreateAssociatedTokenAccount();

// const getTokenAccount = async() => {
//     try {
//         const secretKeyArray = JSON.parse(secretKeyEnv);
//         const secretKey = new Uint8Array(secretKeyArray);
//         const keypair = Keypair.fromSecretKey(secretKey);
        
//         const connection = new Connection(RPC, "confirmed");
//         const tokenPublicKey = new PublicKey(contractAddress);

//         const tokenAccount = await getAccount(
//             connection,
//             tokenPublicKey.,
//         )
//         console.log("tokenAccount", tokenAccount);
//     } catch (error) {
//         console.error(error);
//     }
// }

// getTokenAccount();