import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTodo } from "../target/types/solana_todo";
import { expect } from "chai";
import { BN } from "bn.js";

describe("solana_todo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaTodo as Program<SolanaTodo>;
  const userWallet = anchor.AnchorProvider.env().wallet;

  // Generate a random description for the todo item
  const generateRandomDescription = () => {
    return `Task ${Math.floor(Math.random() * 10000)}`;
  };

  // Calculate PDA addresses for the user profile
  const getUserProfilePDA = async (authority: anchor.web3.PublicKey) => {
    const [userProfilePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-profile"), authority.toBuffer()],
      program.programId
    );
    return userProfilePDA;
  };

  // Calculate PDA addresses for a todo item
  const getTodoPDA = async (authority: anchor.web3.PublicKey, todoId: number) => {
    const [todoPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        authority.toBuffer(),
        new BN(todoId).toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );
    return todoPDA;
  };

  describe("User Profile", () => {
    it("Initialize a new user profile", async () => {
      // Get PDA address for the user profile
      const userProfilePda = await getUserProfilePDA(userWallet.publicKey);

      // Initialize the user profile
      const tx = await program.methods
        .initializeUser()
        .accounts({
          userProfile: userProfilePda,
          authority: userWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      console.log("User profile initialized with tx:", tx);

      // Fetch the user profile account
      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      
      // Verify the user profile was initialized correctly
      expect(userProfile.authority.toString()).to.equal(userWallet.publicKey.toString());
      expect(userProfile.todoCount.toNumber()).to.equal(0);
      expect(userProfile.lastTodoId.toNumber()).to.equal(0);
    });
  });

  describe("Todo Operations", () => {
    let userProfilePda: anchor.web3.PublicKey;
    let firstTodoPda: anchor.web3.PublicKey;
    const description = "Test todo item";
    const dueDate = new Date().getTime() / 1000 + 86400; // 1 day from now

    before(async () => {
      // Get PDA for the user profile
      userProfilePda = await getUserProfilePDA(userWallet.publicKey);
    });

    it("Create a new todo item", async () => {
      // First todo will have id = 1
      firstTodoPda = await getTodoPDA(userWallet.publicKey, 1);

      // Create a todo item
      const tx = await program.methods
        .createTodo(description, new BN(dueDate))
        .accounts({
          userProfile: userProfilePda,
          todoAccount: firstTodoPda,
          authority: userWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Todo created with tx:", tx);

      // Fetch the todo account
      const todoItem = await program.account.todoItem.fetch(firstTodoPda);
      
      // Verify the todo was created correctly
      expect(todoItem.id.toNumber()).to.equal(1);
      expect(todoItem.description).to.equal(description);
      expect(todoItem.completed).to.equal(false);
      expect(todoItem.dueDate.toNumber()).to.be.closeTo(dueDate, 5); // Allow small timestamp differences
      expect(todoItem.owner.toString()).to.equal(userWallet.publicKey.toString());
      expect(todoItem.authority.toString()).to.equal(userWallet.publicKey.toString());

      // Verify the user profile was updated
      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      expect(userProfile.todoCount.toNumber()).to.equal(1);
      expect(userProfile.lastTodoId.toNumber()).to.equal(1);
    });

    it("Update todo status to completed", async () => {
      // Update todo to completed
      const tx = await program.methods
        .updateTodoStatus(true)
        .accounts({
          todoAccount: firstTodoPda,
          authority: userWallet.publicKey,
        })
        .rpc();

      console.log("Todo status updated with tx:", tx);

      // Fetch the updated todo
      const todoItem = await program.account.todoItem.fetch(firstTodoPda);
      
      // Verify the status was updated
      expect(todoItem.completed).to.equal(true);
      expect(todoItem.description).to.equal(description); // Description should be unchanged
    });

    it("Update todo description", async () => {
      // New description
      const newDescription = "Updated todo description";

      // Update todo description
      const tx = await program.methods
        .updateDescription(newDescription)
        .accounts({
          todoAccount: firstTodoPda,
          authority: userWallet.publicKey,
        })
        .rpc();

      console.log("Todo description updated with tx:", tx);

      // Fetch the updated todo
      const todoItem = await program.account.todoItem.fetch(firstTodoPda);
      
      // Verify the description was updated
      expect(todoItem.description).to.equal(newDescription);
      expect(todoItem.completed).to.equal(true); // Completion status should be unchanged
    });

    it("Delete a todo item", async () => {
      // Delete the todo
      const tx = await program.methods
        .deleteTodo()
        .accounts({
          userProfile: userProfilePda,
          todoAccount: firstTodoPda,
          authority: userWallet.publicKey,
        })
        .rpc();

      console.log("Todo deleted with tx:", tx);

      // Verify the todo account was closed
      try {
        await program.account.todoItem.fetch(firstTodoPda);
        expect.fail("Todo account should be closed");
      } catch (error) {
        // Expected error because account is closed
        expect(error).to.exist;
      }

      // Verify the user profile was updated
      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      expect(userProfile.todoCount.toNumber()).to.equal(0);
    });
  });
});
