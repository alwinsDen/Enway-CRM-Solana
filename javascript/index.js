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
    const opporInstructionLayout = borsh.struct([
        borsh.u8("variant"),
        borsh.str("account_name"),
        borsh.u32("amount"),
        borsh.bool("delivered"),
        borsh.u8("probability"),
        borsh.str("stage"),
        borsh.str("title")
    ]);
    // send the review to the contract deployed on the blockchain
    let buffer = Buffer.alloc(100);
    let opporId = `${Math.random() * 1000}`
    //  encode the data
    opporInstructionLayout.encode(
        {
            variant: 0,
            title: opporId,
            account_name: "Wild West",
            amount: 23434,
            delivered: false,
            probability: 3,
            stage: "In Pipeline",
        },
        buffer
    );
    buffer = buffer.slice(0, opporInstructionLayout.getSpan(buffer));
    const [pda] = await web3.PublicKey.findProgramAddress(
        [publicUserKey.publicKey.toBuffer(), Buffer.from(opporId)],
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
    const search = "alwin"
    let accounts = await connection.getProgramAccounts(
        new web3.PublicKey(program_id),
        {
            dataSlice: {
                offset: 6,
                length: 50
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
    let objectPerPage = 50;
    // paginated data
    const paginatedPublicKeys = accounts.slice(
        (pageNo - 1) * objectPerPage,
        pageNo * objectPerPage
    )

    let account_data = await connection.getMultipleAccountsInfo(paginatedPublicKeys);
    //deserialize accounts to get the data
    // account_data.map((acc_data,index)=>{
    //     const {title, rating} = opporInstructionLayout.decode(acc_data.data)
    //     console.log(title,rating)
    //     return 0
    // })
    const borshAccountSchema = borsh.struct([
        borsh.bool('is_initialized'),
        borsh.str("title"),
        borsh.u32("amount"),
        borsh.str("stage"),
        borsh.str("account_name"),
        borsh.bool("delivered"),
        borsh.u8("probability"),
    ])
    for (let i = 0; i < account_data.length; i++) {
        // retrieve the fields out of the account_data
        const {title} = borshAccountSchema.decode(account_data[i].data)
        console.log(title)
    }
}

main();
