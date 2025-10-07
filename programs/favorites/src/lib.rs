use anchor_lang::prelude::*;

declare_id!("2bv4tV1FUXh7WuZoeb4MsqDLWyB9uwZUCEAdP9rxeMze");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
