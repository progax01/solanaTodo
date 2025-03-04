const { Connection, PublicKey, Keypair, SystemProgram } = require('@solana/web3.js');
const { Program, AnchorProvider, web3, BN } = require('@project-serum/anchor');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Function to load the wallet from file path
function loadWalletFromPath(filePath) {
  try {
    // Expand ~ to home directory if needed
    if (filePath.startsWith('~')) {
      filePath = filePath.replace('~', os.homedir());
    }
    
    // Load the key data
    const keyData = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
    
    // Return keypair from the loaded data
    if (Array.isArray(keyData)) {
      return Keypair.fromSecretKey(new Uint8Array(keyData));
    } else if (keyData.secretKey) {
      return Keypair.fromSecretKey(new Uint8Array(keyData.secretKey));
    }
    
    throw new Error('Invalid key format');
  } catch (err) {
    console.error('Error loading wallet:', err);
    throw err;
  }
}

// Main class to interact with the Todo smart contract
class SolanaTodoClient {
  constructor(connection, wallet, programId) {
    this.connection = connection;
    this.wallet = wallet;
    this.programId = new PublicKey(programId);
    
    // Configure the provider
    this.provider = new AnchorProvider(
      connection,
      { publicKey: wallet.publicKey, signTransaction: async (tx) => { 
        tx.sign(wallet);
        return tx;
      }},
      { commitment: 'confirmed' }
    );
    
    // Load the IDL (would typically be generated from the program)
    // For this example, we'll define a minimal version matching our contract
    this.idl = {
      version: "0.1.0",
      name: "solana_todo",
      instructions: [
        {
          name: "initializeUser",
          accounts: [
            { name: "userProfile", isMut: true, isSigner: false },
            { name: "authority", isMut: true, isSigner: true },
            { name: "systemProgram", isMut: false, isSigner: false }
          ],
          args: []
        },
        {
          name: "createTodo",
          accounts: [
            { name: "userProfile", isMut: true, isSigner: false },
            { name: "todoAccount", isMut: true, isSigner: false },
            { name: "authority", isMut: true, isSigner: true },
            { name: "systemProgram", isMut: false, isSigner: false }
          ],
          args: [
            { name: "description", type: "string" },
            { name: "dueDate", type: "i64" }
          ]
        },
        {
          name: "updateTodoStatus",
          accounts: [
            { name: "todoAccount", isMut: true, isSigner: false },
            { name: "authority", isMut: false, isSigner: true }
          ],
          args: [
            { name: "completed", type: "bool" }
          ]
        },
        {
          name: "updateDescription",
          accounts: [
            { name: "todoAccount", isMut: true, isSigner: false },
            { name: "authority", isMut: false, isSigner: true }
          ],
          args: [
            { name: "description", type: "string" }
          ]
        },
        {
          name: "deleteTodo",
          accounts: [
            { name: "userProfile", isMut: true, isSigner: false },
            { name: "todoAccount", isMut: true, isSigner: false },
            { name: "authority", isMut: true, isSigner: true }
          ],
          args: []
        }
      ],
      accounts: [
        {
          name: "UserProfile",
          type: {
            kind: "struct",
            fields: [
              { name: "authority", type: "publicKey" },
              { name: "todoCount", type: "u64" },
              { name: "lastTodoId", type: "u64" }
            ]
          }
        },
        {
          name: "TodoItem",
          type: {
            kind: "struct",
            fields: [
              { name: "id", type: "u64" },
              { name: "description", type: "string" },
              { name: "completed", type: "bool" },
              { name: "dueDate", type: "i64" },
              { name: "owner", type: "publicKey" },
              { name: "authority", type: "publicKey" }
            ]
          }
        }
      ],
      errors: [
        { code: 6000, name: "DescriptionTooLong", msg: "Description must be 280 characters or less" },
        { code: 6001, name: "InvalidDueDate", msg: "Due date must be a valid timestamp" },
        { code: 6002, name: "UnauthorizedAccess", msg: "Only the owner can modify this todo item" }
      ]
    };
    
    // Initialize the program
    this.program = new Program(this.idl, this.programId, this.provider);
  }
  
  // Find the user profile PDA
  async findUserProfileAddress() {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("user-profile"), this.wallet.publicKey.toBuffer()],
      this.programId
    );
  }
  
  // Find a todo item PDA
  async findTodoAddress(todoId) {
    const todoIdBuffer = new BN(todoId).toArrayLike(Buffer, 'le', 8);
    return PublicKey.findProgramAddressSync(
      [Buffer.from("todo"), this.wallet.publicKey.toBuffer(), todoIdBuffer],
      this.programId
    );
  }
  
  // Initialize a new user profile
  async initializeUser() {
    const [userProfilePda, bump] = await this.findUserProfileAddress();
    
    console.log(`Initializing user profile at address: ${userProfilePda.toString()}`);
    
    try {
      const tx = await this.program.methods
        .initializeUser()
        .accounts({
          userProfile: userProfilePda,
          authority: this.wallet.publicKey,
          systemProgram: SystemProgram.programId
        })
        .signers([this.wallet])
        .rpc();
      
      console.log(`User profile initialized. Transaction signature: ${tx}`);
      return tx;
    } catch (error) {
      console.error("Error initializing user:", error);
      throw error;
    }
  }
  
  // Create a new todo item
  async createTodo(description, dueDate) {
    const [userProfilePda] = await this.findUserProfileAddress();
    
    // Get the current last todo ID
    const userProfile = await this.program.account.userProfile.fetch(userProfilePda);
    const nextTodoId = userProfile.lastTodoId.toNumber() + 1;
    
    const [todoPda] = await this.findTodoAddress(nextTodoId);
    
    console.log(`Creating todo #${nextTodoId} at address: ${todoPda.toString()}`);
    
    try {
      const tx = await this.program.methods
        .createTodo(description, new BN(dueDate))
        .accounts({
          userProfile: userProfilePda,
          todoAccount: todoPda,
          authority: this.wallet.publicKey,
          systemProgram: SystemProgram.programId
        })
        .signers([this.wallet])
        .rpc();
      
      console.log(`Todo created. Transaction signature: ${tx}`);
      return { signature: tx, todoId: nextTodoId, todoPda };
    } catch (error) {
      console.error("Error creating todo:", error);
      throw error;
    }
  }
  
  // Update a todo's completion status
  async updateTodoStatus(todoId, completed) {
    const [todoPda] = await this.findTodoAddress(todoId);
    
    console.log(`Updating todo #${todoId} status to ${completed ? "completed" : "incomplete"}`);
    
    try {
      const tx = await this.program.methods
        .updateTodoStatus(completed)
        .accounts({
          todoAccount: todoPda,
          authority: this.wallet.publicKey
        })
        .signers([this.wallet])
        .rpc();
      
      console.log(`Todo status updated. Transaction signature: ${tx}`);
      return tx;
    } catch (error) {
      console.error("Error updating todo status:", error);
      throw error;
    }
  }
  
  // Update a todo's description
  async updateDescription(todoId, description) {
    const [todoPda] = await this.findTodoAddress(todoId);
    
    console.log(`Updating description for todo #${todoId}`);
    
    try {
      const tx = await this.program.methods
        .updateDescription(description)
        .accounts({
          todoAccount: todoPda,
          authority: this.wallet.publicKey
        })
        .signers([this.wallet])
        .rpc();
      
      console.log(`Todo description updated. Transaction signature: ${tx}`);
      return tx;
    } catch (error) {
      console.error("Error updating todo description:", error);
      throw error;
    }
  }
  
  // Delete a todo item
  async deleteTodo(todoId) {
    const [userProfilePda] = await this.findUserProfileAddress();
    const [todoPda] = await this.findTodoAddress(todoId);
    
    console.log(`Deleting todo #${todoId}`);
    
    try {
      const tx = await this.program.methods
        .deleteTodo()
        .accounts({
          userProfile: userProfilePda,
          todoAccount: todoPda,
          authority: this.wallet.publicKey
        })
        .signers([this.wallet])
        .rpc();
      
      console.log(`Todo deleted. Transaction signature: ${tx}`);
      return tx;
    } catch (error) {
      console.error("Error deleting todo:", error);
      throw error;
    }
  }
  
  // Fetch all todos for the current user
  async fetchAllTodos() {
    try {
      const [userProfilePda] = await this.findUserProfileAddress();
      const userProfile = await this.program.account.userProfile.fetch(userProfilePda);
      
      const todoAccounts = await this.program.account.todoItem.all([
        {
          memcmp: {
            offset: 8 + 8 + 280 + 1 + 8, // After discriminator + id + description + completed + due_date
            bytes: this.wallet.publicKey.toBase58()
          }
        }
      ]);
      
      console.log(`Found ${todoAccounts.length} todos for user`);
      return todoAccounts;
    } catch (error) {
      console.error("Error fetching todos:", error);
      throw error;
    }
  }
  
  // Fetch a single todo by ID
  async fetchTodo(todoId) {
    try {
      const [todoPda] = await this.findTodoAddress(todoId);
      const todoAccount = await this.program.account.todoItem.fetch(todoPda);
      return { publicKey: todoPda, account: todoAccount };
    } catch (error) {
      console.error(`Error fetching todo #${todoId}:`, error);
      throw error;
    }
  }
}

// Example usage
async function main() {
  // Load configuration - these would typically come from a config file or env vars
  const config = {
    programId: "Ct2N3zw5LFiNj5mJ7hN2c4umze2pAWNjfYqazZHzDENy",
    walletPath: "/home/mylinux/.config/solana/id.json", // Change this to your keypair path
    rpcUrl: "https://api.devnet.solana.com" // Using devnet for testing
  };
  
  // Setup connection and client
  const connection = new Connection(config.rpcUrl, 'confirmed');
  const wallet = loadWalletFromPath(config.walletPath);
  const todoClient = new SolanaTodoClient(connection, wallet, config.programId);
  
  console.log(`Connected with wallet: ${wallet.publicKey.toString()}`);
  
  try {
    // 1. Initialize user (only needed once per wallet)
    // await todoClient.initializeUser();
    
    // 2. Create a new todo
    const now = Math.floor(Date.now() / 1000);
    const tomorrow = now + (24 * 60 * 60);
    const { todoId } = await todoClient.createTodo("Complete Solana TodoApp", tomorrow);
    
    // 3. Update todo status
    await todoClient.updateTodoStatus(todoId, true);
    
    // 4. Update todo description
    await todoClient.updateDescription(todoId, "Complete Solana TodoApp and deploy to mainnet");
    
    // 5. Fetch all todos
    const allTodos = await todoClient.fetchAllTodos();
    console.log("All todos:", allTodos.map(item => ({
      id: item.account.id.toString(),
      description: item.account.description,
      completed: item.account.completed,
      dueDate: new Date(item.account.dueDate * 1000).toISOString()
    })));
    
    // // 6. Delete the todo
    // await todoClient.deleteTodo(todoId);
    
    console.log("All operations completed successfully!");
  } catch (error) {
    console.error("Error in demo sequence:", error);
  }
}

// If this is being run directly, execute the main function
if (require.main === module) {
  main().then(
    () => process.exit(0),
    err => {
      console.error(err);
      process.exit(1);
    }
  );
}

// Export the client for use in other files
module.exports = { SolanaTodoClient, loadWalletFromPath };