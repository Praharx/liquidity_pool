use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("2pAu5U38ncw415wyXsgM8NDFNQwE4u5UPSQTwkSfs8EZ");

#[program]
pub mod liquidity_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,pool_id:u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(pool_id:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init, 
        payer = creator, 
        space = 8 + 8 + 32 + 32 + 32, 
        seeds=[b"poolinitiate",creator.key().as_ref(),&pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub token_a_reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_reserve: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Pool {
   pub pool_id: u64,
   pub token_a_reserve: Pubkey,
   pub token_b_reserve: Pubkey,
   pub lp_mint: Pubkey
}

pub struct LiquidityProvider {
    pub address: Pubkey,
    pub pool_id: u64,
    pub lp_token_share: u64,
}