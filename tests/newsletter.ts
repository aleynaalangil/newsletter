import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Newsletter } from "../target/types/newsletter";

describe("newsletter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Newsletter as Program<Newsletter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
