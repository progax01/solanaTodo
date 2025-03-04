use std::str::FromStr;
use std::sync::{Arc, Mutex};
use solana_client::nonblocking::rpc_client;
//  use anchor_lang::prelude::AccountMeta;
 
  use solana_program::instruction::AccountMeta;
 
//   use solana_sdk::instruction::AccountMeta;
use actix_web::web;
use anchor_client::{
    anchor_lang::{AccountDeserialize, AnchorDeserialize, AnchorSerialize, InstructionData, ToAccountMetas},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::Transaction,
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use log::{error, info};
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_program::{instruction::Instruction as SolanaInstruction, system_program};
use solana_sdk::msg;
use solana_sdk::{
    signature::Signature as SolanaSignature, 
    signer::keypair::Keypair as SolanaKeypair,
};

use crate::{
    config::get_config,
    error::AppError,
    models::{
        todo::{CreateTodoRequest, Todo, UpdateTodoRequest},
        transaction::{PreparedTransaction, SignedTransaction},
    },
};

#[derive(Clone)]
pub struct SolanaService {
    rpc_url: String,
    program_id: Pubkey,
}

impl SolanaService {
    pub fn new() -> Result<Self, AppError> {
        let config = get_config();
        let rpc_url = config.solana.rpc_url.clone();
        let program_id = Pubkey::from_str(&config.solana.program_id)
            .map_err(|e| AppError::solana(format!("Invalid program ID: {}", e)))?;

        Ok(Self {
            rpc_url,
            program_id,
        })
    }

    fn create_rpc_client(&self) -> RpcClient {
        RpcClient::new(self.rpc_url.clone())
    }

    // Verify a signature from a Solana wallet
    pub fn verify_signature(
        &self,
        public_key: &str,
        message: &str,
        signature: &str,
    ) -> Result<bool, AppError> {
        // Convert public key string to Pubkey
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;

        // Convert signature string to Signature
        let signature_bytes = bs58::decode(signature)
            .into_vec()
            .map_err(|e| AppError::bad_request(format!("Invalid signature format: {}", e)))?;

        if signature_bytes.len() != 64 {
            return Err(AppError::bad_request("Invalid signature length"));
        }

        let signature = SolanaSignature::try_from(signature_bytes.as_slice())
            .map_err(|e| AppError::bad_request(format!("Invalid signature: {}", e)))?;

        // Convert message to bytes
        let message_bytes = message.as_bytes();

        // Verify the signature
        let is_valid = signature.verify(pubkey.as_ref(), message_bytes);

        Ok(is_valid)
    }

    // Get the PDAs for user profile and todo items
    fn get_user_profile_pda(&self, authority: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"user_profile", authority.as_ref()],
            &self.program_id,
        )
    }

    fn get_todo_pda(&self, authority: Pubkey, todo_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"todo",
                authority.as_ref(),
                &todo_id.to_le_bytes(),
            ],
            &self.program_id,
        )
    }

    // Get all todos for a user
    pub async fn get_todos(&self, public_key: &str) -> Result<Vec<Todo>, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;

        // In a real implementation, we would use the Anchor client to fetch all todos
        // Here we'll simulate the response for demonstration purposes
        
        // Mock response for demonstration
        let mock_todos = vec![
            Todo {
                id: 1,
                description: "Complete Solana program".to_string(),
                completed: false,
                due_date: 1625097600,
                owner: public_key.to_string(),
            },
            Todo {
                id: 2,
                description: "Implement frontend".to_string(),
                completed: true,
                due_date: 1625184000,
                owner: public_key.to_string(),
            },
        ];

        Ok(mock_todos)
    }

    // Create a new todo
    pub async fn create_todo(
        &self,
        public_key: &str,
        todo: CreateTodoRequest,
    ) -> Result<Todo, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;

        // Validate request
        if todo.description.is_empty() {
            return Err(AppError::bad_request("Description cannot be empty"));
        }

        if todo.description.len() > 280 {
            return Err(AppError::bad_request("Description must be 280 characters or less"));
        }

        if todo.due_date < 0 {
            return Err(AppError::bad_request("Due date must be a valid timestamp"));
        }

        // In a real implementation, we would use the Anchor client to send a transaction
        // Here we'll simulate the response for demonstration purposes
        
        // Mock response for demonstration
        let new_todo = Todo {
            id: 3, // In a real implementation, this would be the next available ID
            description: todo.description,
            completed: false,
            due_date: todo.due_date,
            owner: public_key.to_string(),
        };

        Ok(new_todo)
    }

    // Update a todo
    pub async fn update_todo(
        &self,
        public_key: &str,
        todo_id: u64,
        update: UpdateTodoRequest,
    ) -> Result<Todo, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;

        // Validate request
        if let Some(ref description) = update.description {
            if description.is_empty() {
                return Err(AppError::bad_request("Description cannot be empty"));
            }

            if description.len() > 280 {
                return Err(AppError::bad_request("Description must be 280 characters or less"));
            }
        }

        // In a real implementation, we would use the Anchor client to send a transaction
        // Here we'll simulate the response for demonstration purposes
        
        // Mock response for demonstration
        let updated_todo = Todo {
            id: todo_id,
            description: update.description.unwrap_or_else(|| "Original description".to_string()),
            completed: update.completed.unwrap_or(false),
            due_date: 1625097600,
            owner: public_key.to_string(),
        };

        Ok(updated_todo)
    }

    // Delete a todo
    pub async fn delete_todo(&self, public_key: &str, todo_id: u64) -> Result<(), AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;

        // In a real implementation, we would use the Anchor client to send a transaction
        // Here we'll simulate the response for demonstration purposes
        
        Ok(())
    }
    pub async fn prepare_create_todo_transaction(
        &self,
        public_key: &str,
        todo: CreateTodoRequest,
    ) -> Result<PreparedTransaction, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;
            
        // Clone values needed for the blocking operation
        let program_id = self.program_id;
        let rpc_url = self.rpc_url.clone();
        
        // Use web::block or tokio::task::spawn_blocking to run RPC operations
        let result = web::block(move || {
            // Create a new RPC client for this request
            let rpc_client = RpcClient::new(rpc_url);
            
            // Get the latest blockhash - this is a blocking operation
            let recent_blockhash = rpc_client.get_latest_blockhash()
                .map_err(|e| AppError::solana(format!("Failed to get recent blockhash: {}", e)))?;
                
            // 1. Get the next todo ID from the user profile
            let next_todo_id = 1; // This would be fetched from the user profile PDA
            
            // 2. Get the PDAs for the todo and user profile
            let (todo_pda, _) = Pubkey::find_program_address(
                &[b"todo", pubkey.as_ref(), &(next_todo_id as u64).to_le_bytes()],
                &program_id,
            );
            let (user_profile_pda, _) = Pubkey::find_program_address(
                &[b"user_profile", pubkey.as_ref()],
                &program_id,
            );
            
            // 3. Build the instruction for creating a todo
            let instruction = Instruction {
                program_id,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new(todo_pda, false),
                    solana_sdk::instruction::AccountMeta::new(user_profile_pda, false),
                    solana_sdk::instruction::AccountMeta::new(pubkey, true),
                    solana_sdk::instruction::AccountMeta::new_readonly(system_program::id(), false),
                ],
                data: vec![0], // Placeholder for instruction data
            };
            
            // Create the transaction
            let transaction = Transaction::new_unsigned(Message::new(
                &[instruction],
                Some(&pubkey),
            ));
            
            // Serialize the transaction
            let serialized_transaction = general_purpose::STANDARD.encode(
                bincode::serialize(&transaction)
                    .map_err(|e| AppError::internal(format!("Failed to serialize transaction: {}", e)))?
            );
            
            // Return the prepared transaction
            Ok::<PreparedTransaction, AppError>(PreparedTransaction {
                serialized_transaction,
                transaction_type: "create_todo".to_string(),
                metadata: Some(serde_json::to_string(&todo)
                    .map_err(|e| AppError::internal(format!("Failed to serialize metadata: {}", e)))?),
            })
        })
        .await
        .map_err(|e| AppError::internal(format!("Task execution error: {}", e)))?;

        result
    }
    // Prepare a transaction for updating a todo
    // pub async fn prepare_update_todo_transaction(
    //     &self,
    //     public_key: &str,
    //     todo_id: u64,
    //     update: UpdateTodoRequest,
    // ) -> Result<PreparedTransaction, AppError> {
    //     let pubkey = Pubkey::from_str(public_key)
    //         .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;
            
    //     // Create a new RPC client for this request
    //     let rpc_client = self.create_rpc_client();
        
    //     // Get the todo PDA
    //     let (todo_pda, _) = self.get_todo_pda(pubkey, todo_id);
        
    //     // Build the instruction
    //     let instruction = Instruction {
    //         program_id: self.program_id,
    //         accounts: vec![
    //             solana_sdk::instruction::AccountMeta::new(todo_pda, false),
    //             solana_sdk::instruction::AccountMeta::new(pubkey, true),
    //         ],
    //         data: vec![1], // Placeholder for instruction data
    //     };
        
    //     // Get the latest blockhash
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .map_err(|e| AppError::solana(format!("Failed to get recent blockhash: {}", e)))?;
            
    //     // Create the transaction
    //     let transaction = Transaction::new_unsigned(Message::new(
    //         &[instruction],
    //         Some(&pubkey),
    //     ));
        
    //     // Serialize the transaction
    //     let serialized_transaction = general_purpose::STANDARD.encode(bincode::serialize(&transaction)
    //         .map_err(|e| AppError::internal(format!("Failed to serialize transaction: {}", e)))?);
            
    //     // Return the prepared transaction
    //     Ok(PreparedTransaction {
    //         serialized_transaction,
    //         transaction_type: "update_todo".to_string(),
    //         metadata: Some(serde_json::to_string(&update)
    //             .map_err(|e| AppError::internal(format!("Failed to serialize metadata: {}", e)))?),
    //     })
    // }
    
    // // Prepare a transaction for deleting a todo
    // pub async fn prepare_delete_todo_transaction(
    //     &self,
    //     public_key: &str,
    //     todo_id: u64,
    // ) -> Result<PreparedTransaction, AppError> {
    //     let pubkey = Pubkey::from_str(public_key)
    //         .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;
            
    //     // Create a new RPC client for this request
    //     let rpc_client = self.create_rpc_client();
        
    //     // Get the PDAs
    //     let (todo_pda, _) = self.get_todo_pda(pubkey, todo_id);
    //     let (user_profile_pda, _) = self.get_user_profile_pda(pubkey);
        
    //     // Build the instruction
    //     let instruction = Instruction {
    //         program_id: self.program_id,
    //         accounts: vec![
    //             solana_sdk::instruction::AccountMeta::new(user_profile_pda, false),
    //             solana_sdk::instruction::AccountMeta::new(todo_pda, false),
    //             solana_sdk::instruction::AccountMeta::new(pubkey, true),
    //         ],
    //         data: vec![2], // Placeholder for instruction data
    //     };
        
    //     // Get the latest blockhash
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .map_err(|e| AppError::solana(format!("Failed to get recent blockhash: {}", e)))?;
            
    //     // Create the transaction
    //     let transaction = Transaction::new_unsigned(Message::new(
    //         &[instruction],
    //         Some(&pubkey),
    //     ));
        
    //     // Serialize the transaction
    //     let serialized_transaction = general_purpose::STANDARD.encode(bincode::serialize(&transaction)
    //         .map_err(|e| AppError::internal(format!("Failed to serialize transaction: {}", e)))?);
            
    //     // Return the prepared transaction
    //     Ok(PreparedTransaction {
    //         serialized_transaction,
    //         transaction_type: "delete_todo".to_string(),
    //         metadata: Some(serde_json::to_string(&todo_id)
    //             .map_err(|e| AppError::internal(format!("Failed to serialize metadata: {}", e)))?),
    //     })
    // }
    

    pub async fn prepare_update_todo_transaction(
        &self,
        public_key: &str,
        todo_id: u64,
        update: UpdateTodoRequest,
    ) -> Result<PreparedTransaction, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;
            
        // Clone values needed for the blocking operation
        let program_id = self.program_id;
        let rpc_url = self.rpc_url.clone();
        
        // Use web::block for RPC operations
        let result = web::block(move || {
            // Create a new RPC client for this request
            let rpc_client = RpcClient::new(rpc_url);
            
            // Get the latest blockhash
            let recent_blockhash = rpc_client.get_latest_blockhash()
                .map_err(|e| AppError::solana(format!("Failed to get recent blockhash: {}", e)))?;
            
            // Get the todo PDA
            let (todo_pda, _) = Pubkey::find_program_address(
                &[b"todo", pubkey.as_ref(), &todo_id.to_le_bytes()],
                &program_id,
            );
            
            // Build the instruction
            let mut instruction_data = vec![1]; // 1 = update instruction
            
            if let Some(completed) = update.completed {
                instruction_data.push(if completed { 1 } else { 0 });
            }
            
            if let Some(description) = &update.description {
                instruction_data.extend_from_slice(description.as_bytes());
            }
            
            let instruction = Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(todo_pda, false),
                    AccountMeta::new(pubkey, true),
                ],
                data: instruction_data,
            };
            
            // Create the transaction
            let message = Message::new(&[instruction], Some(&pubkey));
            let transaction = Transaction::new_unsigned(message);
            
            // Serialize the transaction
            let serialized_transaction = general_purpose::STANDARD.encode(
                bincode::serialize(&transaction)
                    .map_err(|e| AppError::internal(format!("Failed to serialize transaction: {}", e)))?
            );
            
            Ok::<PreparedTransaction, AppError>(PreparedTransaction {
                serialized_transaction,
                transaction_type: "update_todo".to_string(),
                metadata: Some(serde_json::to_string(&update)
                    .map_err(|e| AppError::internal(format!("Failed to serialize metadata: {}", e)))?),
            })
        })
        .await
        .map_err(|e| AppError::internal(format!("Task execution error: {}", e)))??;
        
        Ok(result)
    }
    
    pub async fn prepare_delete_todo_transaction(
        &self,
        public_key: &str,
        todo_id: u64,
    ) -> Result<PreparedTransaction, AppError> {
        let pubkey = Pubkey::from_str(public_key)
            .map_err(|e| AppError::bad_request(format!("Invalid public key: {}", e)))?;
            
        // Clone values needed for the blocking operation
        let program_id = self.program_id;
        let rpc_url = self.rpc_url.clone();
        
        // Use web::block for RPC operations
        let result = web::block(move || {
            // Create a new RPC client for this request
            let rpc_client = RpcClient::new(rpc_url);
            
            // Get the latest blockhash
            let recent_blockhash = rpc_client.get_latest_blockhash()
                .map_err(|e| AppError::solana(format!("Failed to get recent blockhash: {}", e)))?;
            
            // Get the PDAs
            let (todo_pda, _) = Pubkey::find_program_address(
                &[b"todo", pubkey.as_ref(), &todo_id.to_le_bytes()],
                &program_id,
            );
            
            let (user_profile_pda, _) = Pubkey::find_program_address(
                &[b"user_profile", pubkey.as_ref()],
                &program_id,
            );
            
            // Build the instruction
            let instruction = Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(user_profile_pda, false),
                    AccountMeta::new(todo_pda, false),
                    AccountMeta::new(pubkey, true),
                ],
                data: vec![2], // 2 = delete instruction
            };
            
            // Create the transaction
            let message = Message::new(&[instruction], Some(&pubkey));
            let transaction = Transaction::new_unsigned(message);
            
            // Serialize the transaction
            let serialized_transaction = general_purpose::STANDARD.encode(
                bincode::serialize(&transaction)
                    .map_err(|e| AppError::internal(format!("Failed to serialize transaction: {}", e)))?
            );
            
            Ok::<PreparedTransaction, AppError>(PreparedTransaction {
                serialized_transaction,
                transaction_type: "delete_todo".to_string(),
                metadata: Some(serde_json::to_string(&todo_id)
                    .map_err(|e| AppError::internal(format!("Failed to serialize metadata: {}", e)))?),
            })
        })
        .await
        .map_err(|e| AppError::internal(format!("Task execution error: {}", e)))??;
        
        Ok(result)
    }

    pub async fn submit_signed_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<String, AppError> {
        // Clone data for the blocking task
        let rpc_url = self.rpc_url.clone();
        let transaction_data_clone = signed_transaction.serialized_transaction.clone();
        
        // Use tokio::task::spawn_blocking to run the blocking RPC operations
        let signature = tokio::task::spawn_blocking(move || {
            // Create a new RPC client inside the blocking task
            let rpc_client = RpcClient::new(rpc_url);
          
            // Decode the transaction
            let transaction_data = general_purpose::STANDARD.decode(&transaction_data_clone)
                .map_err(|e| AppError::bad_request(format!("Invalid transaction data: {}", e)))?;
                
            let transaction: Transaction = bincode::deserialize(&transaction_data)
                .map_err(|e| AppError::bad_request(format!("Invalid transaction format: {}", e)))?;
                
            // Submit the transaction - this is a blocking operation
            let signature = rpc_client
                .send_transaction(&transaction)
                .map_err(|e| AppError::solana(format!("Failed to submit transaction: {}", e)))?;
                
            Ok::<String, AppError>(signature.to_string())
        })
        .await
        .map_err(|e| AppError::internal(format!("Task execution error: {}", e)))??;
        
        Ok(signature)
    }
} 