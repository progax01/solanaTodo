use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("Ct2N3zw5LFiNj5mJ7hN2c4umze2pAWNjfYqazZHzDENy");

#[program]
pub mod solana_todo {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.todo_count = 0;
        user_profile.last_todo_id = 0;
        Ok(())
    }

    pub fn create_todo(
        ctx: Context<CreateTodo>,
        description: String,
        due_date: i64,
    ) -> Result<()> {
        // Validate input
        require!(description.len() <= 280, TodoError::DescriptionTooLong);
        require!(due_date >= 0, TodoError::InvalidDueDate);

        let user_profile = &mut ctx.accounts.user_profile;
        let todo_account = &mut ctx.accounts.todo_account;
        let authority = &ctx.accounts.authority;

        // Generate a new task ID
        let todo_id = user_profile.last_todo_id + 1;
        user_profile.last_todo_id = todo_id;
        user_profile.todo_count += 1;

        // Initialize the new todo
        todo_account.id = todo_id;
        todo_account.description = description;
        todo_account.completed = false;
        todo_account.due_date = due_date;
        todo_account.owner = authority.key();
        todo_account.authority = authority.key();

        Ok(())
    }

    pub fn update_todo_status(
        ctx: Context<UpdateTodo>,
        completed: bool,
    ) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        
        // Update completion status
        todo_account.completed = completed;
        
        Ok(())
    }

    pub fn update_description(
        ctx: Context<UpdateTodo>,
        description: String,
    ) -> Result<()> {
        // Validate input
        require!(description.len() <= 280, TodoError::DescriptionTooLong);
        
        let todo_account = &mut ctx.accounts.todo_account;
        
        // Update description
        todo_account.description = description;
        
        Ok(())
    }

    pub fn delete_todo(ctx: Context<DeleteTodo>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        
        // Update user profile count
        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap_or(0);
        
        // The account will be closed and lamports returned to the authority
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8, // discriminator + pubkey + todo_count + last_todo_id
        seeds = [b"user-profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTodo<'info> {
    #[account(
        mut,
        seeds = [b"user-profile", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 280 + 1 + 8 + 32 + 32, // discriminator + id + description + completed + due_date + owner + authority
        seeds = [b"todo", authority.key().as_ref(), &user_profile.last_todo_id.checked_add(1).unwrap().to_le_bytes()],
        bump
    )]
    pub todo_account: Account<'info, TodoItem>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTodo<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = todo_account.owner == authority.key() @ TodoError::UnauthorizedAccess
    )]
    pub todo_account: Account<'info, TodoItem>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTodo<'info> {
    #[account(
        mut,
        seeds = [b"user-profile", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        close = authority,
        has_one = authority,
        constraint = todo_account.owner == authority.key() @ TodoError::UnauthorizedAccess
    )]
    pub todo_account: Account<'info, TodoItem>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub todo_count: u64,
    pub last_todo_id: u64,
}

#[account]
pub struct TodoItem {
    pub id: u64,
    pub description: String,
    pub completed: bool,
    pub due_date: i64,
    pub owner: Pubkey,
    pub authority: Pubkey,
}

#[error_code]
pub enum TodoError {
    #[msg("Description must be 280 characters or less")]
    DescriptionTooLong,
    #[msg("Due date must be a valid timestamp")]
    InvalidDueDate,
    #[msg("Only the owner can modify this todo item")]
    UnauthorizedAccess,
}
