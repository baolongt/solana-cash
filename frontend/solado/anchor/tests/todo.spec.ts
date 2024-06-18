import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todo } from "../target/types/todo";

describe("todo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Todo as Program<Todo>;

  it("should run the program", async () => {
    // Add your test here.
    const tx = await program.methods.greet().rpc();
    console.log("Your transaction signature", tx);
  });
});
