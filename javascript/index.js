







// ==========CREATING HELLO WORLD WITH SOLANA - Part 1============
// //program_id HZ3xkJisWVdfLGKidym566bDiAPbyogqr9pWi2yoQh8p
// const dotenv = require("dotenv");
// const web3 = require("@solana/web3.js");
//
// async function main() {
//     let web3 = require("@solana/web3.js");
//     let dotenv = require("dotenv")
//     dotenv.config();
//     const secret = JSON.parse(process.env.PRIVATE_KEY);
//     const secret_key = Uint8Array.from(secret);
//     // this is the generated keypair
//     const publicKeyFromSecret = web3.Keypair.fromSecretKey(secret_key);
//     // program id (created after uplodaing the program)
//     const programId = new web3.PublicKey("9jXP33ikjYLzqnucmwT2X4syGTnxxoqTiHR9ecEJwZdo")
//     const connection = new web3.Connection("http://localhost:8899");
//     // transaction
//     const transaction = new web3.Transaction();
//     const instruction = new web3.TransactionInstruction({
//         keys: [],
//         programId
//     })
//     transaction.add(instruction);
//     //generate transaction signature
//     const transactionSign = await web3.sendAndConfirmTransaction(
//         connection,
//         transaction,
//         [publicKeyFromSecret]
//     );
//     console.log(
//         `Transaction: https://explorer.solana.com/tx/${transactionSign}?cluster=custom`
//     );
// }
//
// main()
