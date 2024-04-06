import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

const connection = new web3.Connection(
  "https://api.devnet.solana.com",
  "confirmed"
);

// Configure the connection to the cluster
const provider = anchor.Provider.env();
const idl = JSON.parse(
  require("fs").readFileSync("./target/idl/catoff_api.json", "utf8")
);
const programId = new PublicKey("<Your_Program_ID>");
const program = new anchor.Program(idl, programId, provider);
const owner = provider.wallet.publicKey;

// const owner = pg.wallet;
// const program = pg.program;

// Get the secret key in string format
// const secretKeyHexString = Buffer.from(owner.keypair.secretKey).toString("hex");
// console.log("Secret Key:", secretKeyHexString);

const depositor = anchor.web3.Keypair.generate();
const depositAmount = new anchor.BN(0.001 * LAMPORTS_PER_SOL);

const secretKey = new Uint8Array([
  218, 61, 181, 179, 3, 125, 90, 34, 71, 79, 79, 220, 110, 30, 129, 196, 30,
  235, 77, 160, 0, 61, 139, 197, 146, 63, 224, 101, 130, 250, 211, 167, 186,
  152, 192, 121, 11, 56, 99, 205, 138, 248, 125, 218, 183, 154, 30, 28, 236,
  219, 221, 205, 72, 219, 114, 22, 50, 8, 165, 16, 134, 114, 169, 139,
]);
const escrowAccount = web3.Keypair.fromSecretKey(secretKey);
const escrowTokenAccount = await getUsdcTokenAccount(escrowAccount.publicKey);
const ownerTokenAccount = await getUsdcTokenAccount(owner.publicKey);
console.log(escrowTokenAccount.toString());

await fund_depositor();
// await initialize_escrow();
await deposit_sol();
await withdraw_sol();
await deposit_usdc();
await withdraw_usdc();

async function initialize_escrow() {
  await program.rpc.initializeEscrow({
    accounts: {
      escrowAccount: escrowAccount.publicKey,
      owner: owner.publicKey,
      systemProgram: web3.SystemProgram.programId,
    },
    signers: [owner.keypair, escrowAccount],
  });

  await display("After initialization");
  await displayPda();
}

async function deposit_sol() {
  const txSignature = await program.rpc.depositSol(
    new anchor.BN(depositAmount),
    {
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        depositor: depositor.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [depositor],
    }
  );

  await connection.getTransaction(txSignature, {
    commitment: "confirmed",
  });

  await display("After Depositing Sol");
  await displayPda();
}

async function withdraw_sol() {
  const withdrawAmount = new anchor.BN(0.001 * LAMPORTS_PER_SOL);

  const txSignature = await program.rpc.withdraw(
    new anchor.BN(withdrawAmount),
    Buffer.from("SOL"),
    {
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        escrowTokenAccount: escrowTokenAccount,
        authority: owner.publicKey,
        toAccount: owner.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [owner.keypair, escrowAccount],
    }
  );

  await connection.getTransaction(txSignature, {
    commitment: "confirmed",
  });

  await display("After Withdraw Sol");
  await displayPda();
}

async function deposit_usdc() {
  const txSignature = await program.rpc.depositUsdc(
    new anchor.BN(1 * 1000000),
    {
      accounts: {
        depositor: owner.publicKey,
        depositorTokenAccount: ownerTokenAccount,
        escrowTokenAccount: escrowTokenAccount,
        escrowAccount: escrowAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [owner.keypair],
    }
  );

  await connection.getTransaction(txSignature, {
    commitment: "confirmed",
  });

  await display("After Depositing USDC");
  await displayPda();
}

async function withdraw_usdc() {
  const withdrawAmount = new anchor.BN(1 * 1000000);

  const txSignature = await program.rpc.withdraw(
    new anchor.BN(withdrawAmount),
    Buffer.from("USDC"),
    {
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        escrowTokenAccount: escrowTokenAccount,
        authority: owner.publicKey,
        toAccount: ownerTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [owner.keypair, escrowAccount],
    }
  );

  await connection.getTransaction(txSignature, {
    commitment: "confirmed",
  });

  await display("After Withdraw USDC");
  await displayPda();
}

async function display(message: String) {
  console.log("\n", message);
  // console.log(
  //   "Admin Balance  : ",
  //   await connection.getBalance(owner.publicKey)
  // );
}

async function displayPda() {
  const escrow_account = await program.account.escrowAccount.fetch(
    escrowAccount.publicKey
  );
  console.log("SOL Balance    : ", escrow_account.solBalance.toString());
  console.log("USDC Balance   : ", escrow_account.usdcBalance.toString());
  // console.log("Authority    : ", escrow_account.authority.toString());

  console.log(
    "Escrow Balance : ",
    await connection.getBalance(escrowAccount.publicKey)
  );
}

async function fund_depositor() {
  const transaction = new web3.Transaction().add(
    web3.SystemProgram.transfer({
      fromPubkey: owner.publicKey,
      toPubkey: depositor.publicKey,
      lamports: 0.003 * web3.LAMPORTS_PER_SOL,
    })
  );
  await web3.sendAndConfirmTransaction(connection, transaction, [
    owner.keypair,
  ]);
}

async function getUsdcTokenAccount(addr: PublicKey) {
  const USDC_MINT = new web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC token mint address
  const tokenList = await connection.getTokenAccountsByOwner(
    new web3.PublicKey(addr),
    { mint: USDC_MINT }
  );

  let escrowTokenAccount = null;
  if (tokenList.value.length > 0) {
    const usdcTokenAccount = tokenList.value[0];
    escrowTokenAccount = usdcTokenAccount.pubkey;
  } else {
    // Create associated token accounts for the new accounts
    escrowTokenAccount = await createAssociatedTokenAccount(
      program.provider.connection,
      owner.keypair,
      USDC_MINT,
      addr
    );
  }
  return escrowTokenAccount;
}
