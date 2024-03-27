const {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

const airdropSol = async (publicAddress) => {
  try {
    for (let index = 0; index < 1000; index++) {
      const currentAddress = new PublicKey(publicAddress);
      const RPC =
        "https://solana-devnet.g.alchemy.com/v2/lsOoSzMo0aZwMWxeHXgqBzhogj6TRUNg";
      const connection = new Connection(RPC, "confirmed");
      const airdropSol = await connection.requestAirdrop(
        currentAddress,
        50 * LAMPORTS_PER_SOL
      );
      const balance = Number(await connection.getBalance(currentAddress)) / 1e9;
      console.log("Current balance is: ", balance);
      const latestBlockHash = await connection.getLatestBlockhash();
      const tx = await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSol,
      });
      console.log("Value is: ", tx); 
    }
  } catch (error) {
    console.error(error);
  }
};

airdropSol("anzbSChAgPseEssCN6mZJiwfFF8iqTRQsTyBcuqkLo4");
