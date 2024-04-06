import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("deposit_sol functionality", () => {
  const admin = pg.wallet;
  const escrowAccount = web3.Keypair.generate();
  const depositor = anchor.web3.Keypair.generate();

  it("Deposits SOL successfully and emits event", async () => {
    await pg.program.rpc.initializeEscrow({
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        owner: admin.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [admin.keypair, escrowAccount],
    });

    const transaction = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: depositor.publicKey,
        lamports: 0.003 * web3.LAMPORTS_PER_SOL,
      })
    );
    await web3.sendAndConfirmTransaction(pg.connection, transaction, [
      admin.keypair,
    ]);

    const depositAmount = new anchor.BN(0.001 * LAMPORTS_PER_SOL);

    const listener_desposit_event = pg.program.addEventListener(
      "DepositEvent",
      (event, slot) => {
        console.log(
          `DepositEvent(${event.from}, ${event.amount}, ${event.currency})`
        );
        assert(event.from == depositor.publicKey, "Incorrect depositor amount");
        assert(event.amount == depositAmount, "Incorrect deposited amount");
        assert(event.currency == "SOL", "Incorrect deposited currency");
      }
    );

    const txSignature = await pg.program.rpc.depositSol(
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

    await pg.connection.getTransaction(txSignature, {
      commitment: "confirmed",
    });

    const updatedEscrowAccount = await pg.program.account.escrowAccount.fetch(
      escrowAccount.publicKey
    );
    assert.strictEqual(
      updatedEscrowAccount.solBalance.toNumber(),
      depositAmount.toNumber(),
      "Sol balance is not equal to deposit amount"
    );

    pg.program.removeEventListener(listener_desposit_event);
  });

  it("Withdraw SOL successfully and emits event", async () => {
    const withdrawAmount = new anchor.BN(0.001 * LAMPORTS_PER_SOL);

    const mintKp = new web3.Keypair();
    const mint = await createMint(
      pg.program.provider.connection,
      pg.wallet.keypair,
      escrowAccount.publicKey,
      null,
      0
    );
    // Create associated token accounts for the new accounts
    const escrowTokenAccount = await createAssociatedTokenAccount(
      pg.program.provider.connection,
      pg.wallet.keypair,
      mint,
      escrowAccount.publicKey
    );

    const listener_withdraw_event = pg.program.addEventListener(
      "WithdrawEvent",
      (event, slot) => {
        console.log(
          `WithdrawEvent(${event.to}, ${event.amount}, ${event.currency})`
        );
        assert(event.to == admin.publicKey, "Incorrect withdraw address");
        assert(event.amount == withdrawAmount, "Incorrect withdraw amount");
        assert(event.currency == "SOL", "Incorrect withdraw currency");
      }
    );

    const txSignature = await pg.program.rpc.withdraw(
      new anchor.BN(withdrawAmount),
      Buffer.from("SOL"),
      {
        accounts: {
          escrowAccount: escrowAccount.publicKey,
          escrowTokenAccount: escrowTokenAccount,
          authority: admin.publicKey,
          toAccount: admin.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [admin.keypair, escrowAccount],
      }
    );

    await pg.connection.getTransaction(txSignature, {
      commitment: "confirmed",
    });

    const updatedEscrowAccount = await pg.program.account.escrowAccount.fetch(
      escrowAccount.publicKey
    );
    assert.strictEqual(
      updatedEscrowAccount.solBalance.toNumber(),
      0,
      "Sol balance is incorrect"
    );

    pg.program.removeEventListener(listener_withdraw_event);
  });
});