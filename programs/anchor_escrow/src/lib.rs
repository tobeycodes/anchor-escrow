#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

declare_id!("72v75co6xJcawNjx59zcScmLpz1rXmm7czo1X4LYZzTi");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, recieve: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, recieve, amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}
