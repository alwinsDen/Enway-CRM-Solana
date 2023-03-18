// ==========Movie review WITH SOLANA - Part 2============
let web3 = require("@solana/web3.js");
let borsh = require("@project-serum/borsh");
let dotenv = require("dotenv");
dotenv.config();

async function main() {
  const secret = JSON.parse(process.env.PRIVATE_KEY);
  const secret_key = Uint8Array.from(secret);
  const publicUserKey = web3.Keypair.fromSecretKey(secret_key);
  //smart contract program id
  const program_id = new web3.PublicKey(
    "9jXP33ikjYLzqnucmwT2X4syGTnxxoqTiHR9ecEJwZdo"
  );
  const connection = new web3.Connection("http://localhost:8899");
  // create a borsh struct constant
  const movieInstructionLayout = borsh.struct([
    borsh.u8("variant"),
    borsh.str("title"),
    borsh.u8("rating"),
    borsh.str("description"),
  ]);
  // send the review to the contract deployed on the blockchain
  let buffer = Buffer.alloc(100);
  let movieName = `BraveHeart${Math.random() * 1000}`
  //  encode the data
  movieInstructionLayout.encode(
    {
      variant: 0,
      title: movieName,
      rating: 5,
      description: "A great movie",
    },
    buffer
  );
  buffer = buffer.slice(0, movieInstructionLayout.getSpan(buffer));
  const [pda] = await web3.PublicKey.findProgramAddress(
    [publicUserKey.publicKey.toBuffer(), Buffer.from(movieName)],
    program_id
  );
  // logging the PDA
  console.log(pda.toBase58());
  // transaction process
  const transaction = new web3.Transaction();
  const instruction = new web3.TransactionInstruction({
    programId: program_id,
    data: buffer,
    keys: [
      {
        pubkey: publicUserKey.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: pda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
  });
  transaction.add(instruction);
  const transactionSign = await web3.sendAndConfirmTransaction(
    connection,
    transaction,
    [publicUserKey]
  );
  console.log(
    `Transaction: https://explorer.solana.com/tx/${transactionSign}?cluster=custom`
  );
}

main();





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
