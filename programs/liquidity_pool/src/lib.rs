use anchor_lang::prelude::*;

declare_id!("2pAu5U38ncw415wyXsgM8NDFNQwE4u5UPSQTwkSfs8EZ");

#[program]
pub mod liquidity_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
