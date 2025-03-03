use std::str::FromStr;
use std::sync::{Arc, Mutex};

use anchor_client::{
    anchor_lang::{AccountDeserialize, AnchorDeserialize, AnchorSerialize},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use log::{error, info};
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_program::{instruction::Instruction, message::Message, system_program};
use solana_sdk::{
    signature::Signature as SolanaSignature, signer::keypair::Keypair as SolanaKeypair,
};

use crate::{
    config::get_config,
    error::AppError,
    models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest},
};

#[derive(Clone)]
pub struct SolanaService {
    rpc_client: Arc<Mutex<RpcClient>>,
    program_id: Pubkey,
}

impl SolanaService {
    pub fn new() -> Result<Self, AppError> {
        let config = get_config();
        let rpc_url = &config.solana.rpc_url;
        let program_id = Pubkey::from_str(&config.solana.program_id)
            .map_err(|e| AppError::solana(format!("Invalid program ID: {}", e)))?;

        let rpc_client = RpcClient::new(rpc_url.to_string());

        Ok(Self {
            rpc_client: Arc::new(Mutex::new(rpc_client)),
            program_id,
        })
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
            &[b"user-profile", authority.as_ref()],
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
} 