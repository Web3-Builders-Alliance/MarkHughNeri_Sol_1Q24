use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("HWEachANA42X1vkoBH1nBiyo7hRpNGAs7zYNz1KFw3Le");


#[program]
pub mod vault_2024 {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, seed: u64, lamports: u64) -> Result<()> {
        ctx.accounts.vault.set_inner(Vault {
            maker: ctx.accounts.maker.key(),
            taker: ctx.accounts.taker.key(),
            seed,
            bump: ctx.bumps.vault,
            // created_at: Clock::get()?.unix_timestamp,
        });
        let accts = Transfer {
            from: ctx.accounts.maker.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), accts);
        transfer(cpi_ctx, lamports)
    }

    // pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    // }

    pub fn claim(ctx: Context<Claim>, seed: u64, lamports: u64) -> Result<()> {
        ctx.accounts.vault.set_inner(Vault {
            maker: ctx.accounts.maker.key(),
            taker: ctx.accounts.taker.key(),
            seed,
            bump: ctx.bumps.vault,
            // created_at: Clock::get()?.unix_timestamp,
        });
        let seed = ctx.accounts.vault.seed.to_le_bytes();
        let taker =  ctx.accounts.vault.taker.as_ref();
        let maker =  ctx.accounts.vault.maker.as_ref();

        let accts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.taker.to_account_info(),
        };
        let seeds = &[
            "vault".as_bytes(),
            seed.as_ref(),
            maker,
            taker,
            &[ctx.accounts.vault.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accts,
            signer_seeds,
        );
        // let cpi_ctx = CpiContext::new(ctx.accounts.program_id.to_account_info(), accts);
        transfer(cpi_ctx, lamports)
    }
}

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    /// CHECK: this is ok.
    pub taker: UncheckedAccount<'info>,
    #[account(init, payer=maker, seeds = [b"vault", seed.to_le_bytes().as_ref(), maker.key().as_ref(), taker.key().as_ref()], bump, space=Vault::INIT_SPACE)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Claim<'info> {
    #[account(mut)]
    /// CHECK: this is ok.
    pub taker: AccountInfo<'info>,
    /// CHECK: this is ok.
    pub maker: UncheckedAccount<'info>,
    // #[account(mut)]
    #[account(
        // init, 
        seeds = [b"vault", seed.to_le_bytes().as_ref(),  maker.key().as_ref(), taker.key().as_ref()], 
        bump, 
        // space=Vault::INIT_SPACE
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl Space for Vault {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1 ;//+ 8;
}

//8 + 32
#[account]
pub struct Vault {
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub seed: u64,
    pub bump: u8,
    // pub created_at: i64,
}
