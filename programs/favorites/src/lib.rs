use anchor_lang::prelude::*;

declare_id!("2bv4tV1FUXh7WuZoeb4MsqDLWyB9uwZUCEAdP9rxeMze");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = ctx.accounts.favorites.key();

        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("User {user_public_key}'s favorite number is {number}, favorite color is: {color}");

        msg!("User hobbies are: {:?}", hobbies);

        ctx.accounts.favorites.set_inner(Favorites {
            color,
            number,
            hobbies,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
    seeds=[b"client",user.key().as_ref()],
    space=ANCHOR_DISCRIMINATOR_SIZE+Favorites::INIT_SPACE,
    payer=user,
    bump
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    #[max_len(50)]
    pub color: String,
    pub number: u64,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
