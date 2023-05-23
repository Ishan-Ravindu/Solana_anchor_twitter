use anchor_lang::prelude::*;

declare_id!("Gr9GxKWgvEY4dpRBPz4g1QZ3qUmYVJg7wLCzTQbxXW4R");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
