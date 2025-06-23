use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        context: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Return<()> {
        //the function body - actual instruction handler here
        msg!("Greetings form {}", context.program_id);
        let user_public_key = context.accounts.user.key();

        msg!("User {user_public_key}'s favourite number is {number}, favourite color is {color} and their hobbies are {hobbies:?}");

        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()]

    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}
