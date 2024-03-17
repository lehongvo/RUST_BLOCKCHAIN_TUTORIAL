//================================================================
const {
    clusterApiUrl,
    Connection,
    Keypair,
    PublicKey,
    LAMPORTS_PER_SOL,
} = require("@solana/web3.js");
const {
    mintTo, transfer, getMint
} = require('@solana/spl-token');

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

const getBalance = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const payer = Keypair.fromSecretKey(secretKey);
        
        const connection = new Connection(RPC, "confirmed");
        const contract = new PublicKey(contractAddress);
        
        const supply = await getMint(
            connection,
            contract
        );
        console.log("Supply", supply.supply);
        console.log("Decimal", supply.decimals);
        console.log("address", (supply.address).toBase58());
        console.log("freezeAuthority", (supply.freezeAuthority).toBase58());
    } catch (error) {
        console.log(error);
    }
}

const getAccount = async() => {
    try {
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const payer = Keypair.fromSecretKey(secretKey);
        
        const connection = new Connection(RPC, "confirmed");
        const contract = new PublicKey(contractAddress);
        

        const tokenAccountKey = new PublicKey(tokenAccount);

        const getTokenAccountInfo = await getAccount(
            connection,
            tokenAccountKey
        )
        console.log("getTokenAccountInfo", getTokenAccountInfo);
    } catch (error) {
        console.log(error);
    }
}

getBalance();