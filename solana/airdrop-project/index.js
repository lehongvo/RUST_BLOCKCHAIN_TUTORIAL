// const {
//     Connection,
//     PublicKey,
//     clusterApiUrl,
//     Keypair,
//     LAMPORTS_PER_SOL
// } = require("@solana/web3.js")

// // // const wallet = new Keypair();
// // // const publicKey = new PublicKey(wallet._keypair.publicKey)
// // // const secretKey = wallet._keypair.secretKey

// const getWalletBalance = async() => {
//     try {
//         const rpc = "https://solana-devnet.g.alchemy.com/v2/lsOoSzMo0aZwMWxeHXgqBzhogj6TRUNg";
//         const publicKey =  new PublicKey("4TxsVJKRRrfLowoKWjvT4As2TQDuAhWibXxZrQGQcqV9");
//         const connection = new Connection(rpc, 'confirmed');
//         const walletBalance = Number(await connection.getBalance(publicKey)) / 1e9;
//         console.log(`Wallet balance is ${walletBalance}`)
//     } catch(err) {
//         console.error(err)
//     }
// }

// getWalletBalance();

const {
    Connection,
    PublicKey,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL
} = require('@solana/web3.js')

const rpc = 'https://solana-devnet.g.alchemy.com/v2/lsOoSzMo0aZwMWxeHXgqBzhogj6TRUNg'
// const wallet = new Keypair()

// const publicKey = new PublicKey(wallet._keypair.publicKey)
// const secretKey = wallet._keypair.secretKey

const getWalletBalance = async () => {
    try {
        console.log('Start ==')

        const publicKey = new PublicKey(
            '4TxsVJKRRrfLowoKWjvT4As2TQDuAhWibXxZrQGQcqV9'
        )
        const connection = new Connection(rpc, 'confirmed')
        const walletBalance = await connection.getBalance(publicKey)
        console.log(`Wallet balance is ${walletBalance}`)
    } catch (err) {
        console.error(err)
    }
}

const airDropSol = async () => {
    try {
        console.log('airDropSol: ', LAMPORTS_PER_SOL)
        const publicKey = new PublicKey(
            '4TxsVJKRRrfLowoKWjvT4As2TQDuAhWibXxZrQGQcqV9'
        )
        const connection = new Connection(rpc, 'confirmed')
        ber
        const latestBlockHash = await connection.getLatestBlockhash()
        const tx = await connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: fromAirDropSignature
        })
        console.log(tx, fromAirDropSignature)
    } catch (error) {
        console.error(error)
    }
}

airDropSol()

// getWalletBalance();
