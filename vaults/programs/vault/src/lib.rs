#![allow(unexpected_cfgs)]

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("CSEf8jCdaERHUwsJEe4BsAvG3ssAxP4nsnH7utj1CeZ");
const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(ctx.bumps)
    }
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<CloseVault>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        space = ANCHOR_DISCRIMINATOR_SIZE + VaultState::INIT_SPACE,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close  = user
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> CloseVault<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
