//================================================================
const { clusterApiUrl, Connection, Keypair, PublicKey, LAMPORTS_PER_SOL } = require("@solana/web3.js");
const { mintTo, transfer, getMint } = require('@solana/spl-token');
const { AccountLayout, TOKEN_PROGRAM_ID } = require("@solana/spl-token");

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

const view_all_token = async () => {
    try {
        const connection = new Connection(RPC, "confirmed");
        const secretKeyArray = JSON.parse(secretKeyEnv);
        const secretKey = new Uint8Array(secretKeyArray);
        const payer = Keypair.fromSecretKey(secretKey);

        const publicKey = payer.publicKey;

        const tokenAccounts = await connection.getTokenAccountsByOwner(
            publicKey,
            {
                programId: TOKEN_PROGRAM_ID
            }
        )

        tokenAccounts.value.forEach(tokenAccount => {
            console.log("--------------------------------")
            const accountData = AccountLayout.decode(tokenAccount.account.data);
            console.log("Owner is: ", (accountData.owner).toBase58());
            console.log("Contract Address is: ", (accountData.mint).toBase58());
            console.log("Amount is: ", Number(accountData.amount).toString() / 1e9);
            console.log("--------------------------------")
        })
    } catch (error) {
        console.log(error);
    }
}

view_all_token();