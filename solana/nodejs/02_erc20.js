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
    createMint, 
    createAccount , 
    getOrCreateAssociatedTokenAccount,
    getMint
} = require('@solana/spl-token');
//================================================================

//================================================================
const secretKeyEnv = process.env.SECRET_KEY_BYTES;
const RPC = process.env.RPC;
//================================================================

//================================================================
const createNewToken = async () => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const keypair = Keypair.fromSecretKey(secretKey);
        const connection = new Connection(RPC, "confirmed");


        const token = await createMint(
            connection,
            keypair,
            keypair.publicKey,
            keypair.publicKey,
            9
        );
        
        console.log("token is", token)
        const contractAddress = token.toBase58();
        console.log("Contract address is: ", contractAddress);

        // const auxiliaryTokenAccount = await createAccount(
        //     connection,
        //     keypair,
        //     token,
        //     keypair.publicKey,
        //     keypair
        // );
        // console.log("auxiliaryTokenAccount is: ", auxiliaryTokenAccount)

        // const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
        //     connection,
        //     keypair,
        //     token,
        //     keypair.publicKey
        // );
        // console.log("associatedTokenAccount is: ", associatedTokenAccount)
    } catch (error) {
        console.log(error);
    }
}
//================================================================

//================================================================
createNewToken();
//================================================================