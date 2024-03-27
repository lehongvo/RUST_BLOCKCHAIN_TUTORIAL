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
const contractAddress = process.env.TOKEN_CONTRACT_ADDRESS;
//================================================================
// please code get supply here
const getSupply = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const keypair = Keypair.fromSecretKey(secretKey);

        for (let index = 0; index < 1000; index++) {
            const connection = new Connection(RPC, "confirmed");
            const tokenPublicKey = new PublicKey(contractAddress);
            const mintInfo = await getMint(
                connection,
                tokenPublicKey
            )
            console.log("mintInfo", Number(mintInfo.supply) / 1e9); 
        }
    } catch (error) {
        console.error(error);
    }
}

getSupply();