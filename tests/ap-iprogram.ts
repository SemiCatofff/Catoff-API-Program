import * as anchor from '@project-serum/anchor';
import { ApiProgram } from '../target/types/api_program';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';


describe("deposit_sol functionality", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ApiProgram as anchor.Program<ApiProgram>;

    it("Deposits SOL successfully and emits event", async () => {
      
        const depositor = anchor.web3.Keypair.generate();
        await provider.connection.requestAirdrop(depositor.publicKey, 2 * LAMPORTS_PER_SOL);
        await new Promise((resolve) => setTimeout(resolve, 2000));
        const escrowAccount = anchor.web3.Keypair.generate();
        const depositAmount = new anchor.BN(1 * LAMPORTS_PER_SOL);
        await program.rpc.depositSol(depositAmount, {
            accounts: {
                escrowAccount: escrowAccount.publicKey,
                depositor: depositor.publicKey,
            },
            signers: [depositor],
        });
        const updatedEscrowAccount = await program.account.escrowAccount.fetch(escrowAccount.publicKey);
        console.log("Updated SOL balance in escrow account:", updatedEscrowAccount.solBalance.toString());
        expect(updatedEscrowAccount.solBalance.toNumber()).to.equal(depositAmount.toNumber());

         const txSignature = await program.rpc.depositSol(new anchor.BN(depositAmount), {
        accounts: {
            escrowAccount: escrowAccount.publicKey,
            depositor: depositor.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [depositor],
    });
    const txReceipt = await connection.getTransaction(txSignature, { commitment: "confirmed" });
    const eventLog = txReceipt.meta.logMessages.find(log => log.includes("DepositEvent"));
    expect(eventLog).to.not.be.undefined;
    console.log("Event log:", eventLog);
    });
});
