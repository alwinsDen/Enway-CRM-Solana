let web3 = require("@solana/web3.js");
let borsh = require("@project-serum/borsh");
let dotenv = require("dotenv");
let bs58 = require('bs58')

dotenv.config();

async function main() {
    const secret = JSON.parse(process.env.PRIVATE_KEY);
    const secret_key = Uint8Array.from(secret);
    const publicUserKey = web3.Keypair.fromSecretKey(secret_key);
    //smart contract program id
    const program_id = new web3.PublicKey(process.env.PROGRAM_KEY);
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
    // console.log(pda.toBase58());
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
    //fetch the data that has been sent to the network
    //fetch accounts
    const search = ""
    let accounts = await connection.getProgramAccounts(
        new web3.PublicKey(program_id),
        {
            dataSlice: {
                offset: 2, length: 18
            },
            filters: search === "" ? [] : [
                {
                    memcmp: {
                        offset: 6,
                        bytes: bs58.encode(Buffer.from(search))
                    }
                }
            ]
        }
    )
    accounts.sort((a, b) => {
        const lengthA = a.account.data.readUInt32LE(0)
        const lengthB = b.account.data.readUInt32LE(0)
        const dataA = a.account.data.slice(4, 4 + lengthA)
        const dataB = b.account.data.slice(4, 4 + lengthB)
        return dataA.compare(dataB)
    })
    accounts = accounts.map(account => account.pubkey)
    //get the paginated public keys as per a required limit
    let pageNo = 1;
    let objectPerPage = 2;
    // paginated data
    const paginatedPublicKeys = accounts.slice(
        (pageNo - 1) * objectPerPage,
        pageNo * objectPerPage
    )
    //
    let account_data = await connection.getMultipleAccountsInfo(paginatedPublicKeys);
    //deserialize accounts to get the data
    // account_data.map((acc_data,index)=>{
    //     const {title, rating} = movieInstructionLayout.decode(acc_data.data)
    //     console.log(title,rating)
    //     return 0
    // })
    const borshAccountSchema = borsh.struct([
        borsh.bool('is_initialized'),
        borsh.u8('rating'),
        borsh.str('title'),
        borsh.str('description'),
    ])
    for (let i = 0; i < 2; i++) {
        // retrieve the fields out of the account_data
        const data = borshAccountSchema.decode(account_data[i].data)
    }
}

main();
