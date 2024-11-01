import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestCreditSaleProgram } from "../target/types/test_credit_sale_program";

describe("test_credit_sale_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .TestCreditSaleProgram as Program<TestCreditSaleProgram>;

  it("Records a sale with Pubkey IDs and logging", async () => {
    const creditId = anchor.web3.Keypair.generate().publicKey;
    const clientId = anchor.web3.Keypair.generate().publicKey;
    const amount = new anchor.BN(5);
    const price = new anchor.BN(1000);

    const tx = await program.methods
      .retireToken(creditId, clientId, amount)
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature:", tx);
  });
});
