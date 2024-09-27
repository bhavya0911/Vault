use anchor_lang::prelude::*;

mod constants;
mod state;
mod contexts;
use contexts::*;

declare_id!("DQK4nsMqqo2EH6fD2iL9yiDqgPcZqMLGLvZYNmuBP4P");

#[program]
pub mod week2_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}