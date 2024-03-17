//================================================================
const {
    clusterApiUrl,
    Connection,
    Keypair,
    PublicKey,
    LAMPORTS_PER_SOL,
} = require("@solana/web3.js");
const {
    mintTo, transfer
} = require('@solana/spl-token');

const dotenv = require("dotenv");
dotenv.config();
//================================================================

//================================================================
const secretKeyEnv = process.env.SECRET_KEY_BYTES;
const RPC = process.env.RPC;
const contractAddress = process.env.TOKEN_CONTRACT;
const tokenAccount = process.env.TOKEN_ACCOUNT;
//================================================================
// please code get supply here
const mintTransaction = async () => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);

        const connection = new Connection(RPC, "confirmed");

        const payer = Keypair.fromSecretKey(secretKey);
        const contractAddressKey = new PublicKey(contractAddress);
        const tokenAccountKey = new PublicKey(tokenAccount);

        const mintToTransaction = await mintTo(
            connection,
            payer,
            contractAddressKey,
            tokenAccountKey,
            payer,
            120000000000
        )
        console.log("mintToTransaction", mintToTransaction);
    } catch (error) {
        console.error(error);
    }
}

mintTransaction();