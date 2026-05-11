use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("4h9runapJK84JaNPj41LdpquwyZ78W6z9vXXvfPZjshL");

const MAX_LTV_BPS: u64 = 6000;
const LIQUIDATION_LTV_BPS: u64 = 7500;
const LIQUIDATION_PENALTY_BPS: u64 = 500;
const BASIS_POINTS: u64 = 10000;

#[program]
pub mod collat_mezo {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        min_collateral: u64,
        fee_rate_bps: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.admin.key();
        vault.btc_mint = ctx.accounts.btc_mint.key();
        vault.musd_mint = ctx.accounts.musd_mint.key();
        vault.min_collateral = min_collateral;
        vault.fee_rate_bps = fee_rate_bps;
        vault.total_btc_deposited = 0;
        vault.total_musd_borrowed = 0;
        vault.bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn set_price(ctx: Context<SetPrice>, price: u64) -> Result<()> {
        let price_feed = &mut ctx.accounts.price_feed;
        price_feed.oracle = ctx.accounts.oracle.key();
        price_feed.price = price;
        price_feed.updated_at = Clock::get()?.unix_timestamp;
        price_feed.bump = ctx.bumps.price_feed;
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePrice>, price: u64) -> Result<()> {
        let price_feed = &mut ctx.accounts.price_feed;
        price_feed.price = price;
        price_feed.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        require!(amount > 0, CollatError::ZeroAmount);

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_btc_ata.to_account_info(),
                to: ctx.accounts.vault_btc_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        let position = &mut ctx.accounts.position;
        position.owner = ctx.accounts.user.key();
        position.btc_deposited = position.btc_deposited.checked_add(amount).unwrap();

        let vault = &mut ctx.accounts.vault;
        vault.total_btc_deposited = vault.total_btc_deposited.checked_add(amount).unwrap();

        emit!(CollateralDeposited {
            owner: ctx.accounts.user.key(),
            amount,
            total_btc: position.btc_deposited,
        });

        Ok(())
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        require!(amount > 0, CollatError::ZeroAmount);

        let position = &ctx.accounts.position;
        require!(position.btc_deposited >= amount, CollatError::InsufficientCollateral);
        require!(position.musd_borrowed == 0, CollatError::OutstandingLoan);

        let vault = &ctx.accounts.vault;
        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_btc_ata.to_account_info(),
                to: ctx.accounts.user_btc_ata.to_account_info(),
                authority: vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, amount)?;

        let vault = &mut ctx.accounts.vault;
        vault.total_btc_deposited = vault.total_btc_deposited.checked_sub(amount).unwrap();

        let position = &mut ctx.accounts.position;
        position.btc_deposited = position.btc_deposited.checked_sub(amount).unwrap();

        emit!(CollateralWithdrawn {
            owner: ctx.accounts.owner.key(),
            amount,
            total_btc: position.btc_deposited,
        });

        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        require!(amount > 0, CollatError::ZeroAmount);

        let vault = &ctx.accounts.vault;
        let position = &ctx.accounts.position;
        let price_feed = &ctx.accounts.price_feed;

        let collateral_value = position
            .btc_deposited
            .checked_mul(price_feed.price)
            .unwrap();

        let max_borrow = collateral_value
            .checked_mul(MAX_LTV_BPS)
            .unwrap()
            .checked_div(BASIS_POINTS)
            .unwrap()
            .checked_div(LAMPORTS_PER_SOL)
            .unwrap();

        let new_debt = position.musd_borrowed.checked_add(amount).unwrap();
        require!(new_debt <= max_borrow, CollatError::ExceedsMaxLTV);

        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_musd_ata.to_account_info(),
                to: ctx.accounts.user_musd_ata.to_account_info(),
                authority: vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, amount)?;

        let position = &mut ctx.accounts.position;
        position.musd_borrowed = position.musd_borrowed.checked_add(amount).unwrap();

        let vault = &mut ctx.accounts.vault;
        vault.total_musd_borrowed = vault.total_musd_borrowed.checked_add(amount).unwrap();

        emit!(Borrowed {
            owner: ctx.accounts.owner.key(),
            amount,
            total_debt: position.musd_borrowed,
        });

        Ok(())
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        require!(amount > 0, CollatError::ZeroAmount);

        let position = &mut ctx.accounts.position;
        require!(position.musd_borrowed >= amount, CollatError::OverRepay);

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_musd_ata.to_account_info(),
                to: ctx.accounts.vault_musd_ata.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        position.musd_borrowed = position.musd_borrowed.checked_sub(amount).unwrap();

        let vault = &mut ctx.accounts.vault;
        vault.total_musd_borrowed = vault.total_musd_borrowed.checked_sub(amount).unwrap();

        emit!(Repaid {
            owner: ctx.accounts.owner.key(),
            amount,
            remaining_debt: position.musd_borrowed,
        });

        Ok(())
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount: u64) -> Result<()> {
        require!(amount > 0, CollatError::ZeroAmount);

        let vault = &ctx.accounts.vault;
        let position = &ctx.accounts.position;
        let price_feed = &ctx.accounts.price_feed;

        let collateral_value = position
            .btc_deposited
            .checked_mul(price_feed.price)
            .unwrap();

        let current_ltv = position
            .musd_borrowed
            .checked_mul(BASIS_POINTS)
            .unwrap()
            .checked_mul(LAMPORTS_PER_SOL)
            .unwrap()
            .checked_div(collateral_value)
            .unwrap();

        require!(current_ltv > LIQUIDATION_LTV_BPS, CollatError::NotLiquidatable);

        let penalty = amount
            .checked_mul(LIQUIDATION_PENALTY_BPS)
            .unwrap()
            .checked_div(BASIS_POINTS)
            .unwrap();
        let seize_amount = amount.checked_add(penalty).unwrap();

        require!(
            position.btc_deposited >= seize_amount,
            CollatError::InsufficientCollateral
        );

        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_btc_ata.to_account_info(),
                to: ctx.accounts.liquidator_btc_ata.to_account_info(),
                authority: vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, seize_amount)?;

        let position = &mut ctx.accounts.position;
        position.btc_deposited = position.btc_deposited.checked_sub(seize_amount).unwrap();

        let vault = &mut ctx.accounts.vault;
        vault.total_btc_deposited = vault.total_btc_deposited.checked_sub(seize_amount).unwrap();

        emit!(Liquidated {
            owner: ctx.accounts.owner.key(),
            liquidator: ctx.accounts.liquidator.key(),
            debt_covered: amount,
            collateral_seized: seize_amount,
        });

        Ok(())
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        let position = &ctx.accounts.position;
        require!(position.btc_deposited == 0, CollatError::CollateralRemaining);
        require!(position.musd_borrowed == 0, CollatError::OutstandingLoan);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + VaultConfig::LEN,
        seeds = [b"vault", admin.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultConfig>,

    pub btc_mint: Account<'info, Mint>,
    pub musd_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        token::mint = btc_mint,
        token::authority = vault,
        seeds = [b"vault_btc", vault.key().as_ref()],
        bump
    )]
    pub vault_btc_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        token::mint = musd_mint,
        token::authority = vault,
        seeds = [b"vault_musd", vault.key().as_ref()],
        bump
    )]
    pub vault_musd_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetPrice<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,

    #[account(
        init,
        payer = oracle,
        space = 8 + PriceFeed::LEN,
        seeds = [b"price_feed"],
        bump
    )]
    pub price_feed: Account<'info, PriceFeed>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        mut,
        address = price_feed.oracle @ CollatError::UnauthorizedOracle
    )]
    pub oracle: Signer<'info>,

    #[account(
        mut,
        seeds = [b"price_feed"],
        bump = price_feed.bump
    )]
    pub price_feed: Account<'info, PriceFeed>,
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultConfig>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserPosition::LEN,
        seeds = [b"position", user.key().as_ref()],
        bump
    )]
    pub position: Account<'info, UserPosition>,

    #[account(
        mut,
        constraint = user_btc_ata.mint == vault.btc_mint
    )]
    pub user_btc_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_btc", vault.key().as_ref()],
        bump
    )]
    pub vault_btc_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position", owner.key().as_ref()],
        bump = position.bump,
        has_one = owner
    )]
    pub position: Account<'info, UserPosition>,

    #[account(
        mut,
        constraint = user_btc_ata.mint == vault.btc_mint
    )]
    pub user_btc_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_btc", vault.key().as_ref()],
        bump
    )]
    pub vault_btc_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position", owner.key().as_ref()],
        bump = position.bump,
        has_one = owner
    )]
    pub position: Account<'info, UserPosition>,

    #[account(
        seeds = [b"price_feed"],
        bump = price_feed.bump
    )]
    pub price_feed: Account<'info, PriceFeed>,

    #[account(
        mut,
        constraint = user_musd_ata.mint == vault.musd_mint
    )]
    pub user_musd_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_musd", vault.key().as_ref()],
        bump
    )]
    pub vault_musd_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position", owner.key().as_ref()],
        bump = position.bump,
        has_one = owner
    )]
    pub position: Account<'info, UserPosition>,

    #[account(
        mut,
        constraint = user_musd_ata.mint == vault.musd_mint
    )]
    pub user_musd_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_musd", vault.key().as_ref()],
        bump
    )]
    pub vault_musd_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    /// CHECK: verified via has_one on position
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position", owner.key().as_ref()],
        bump = position.bump,
        has_one = owner
    )]
    pub position: Account<'info, UserPosition>,

    #[account(
        seeds = [b"price_feed"],
        bump = price_feed.bump
    )]
    pub price_feed: Account<'info, PriceFeed>,

    #[account(
        mut,
        seeds = [b"vault_btc", vault.key().as_ref()],
        bump
    )]
    pub vault_btc_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = liquidator_btc_ata.mint == vault.btc_mint
    )]
    pub liquidator_btc_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", owner.key().as_ref()],
        bump = position.bump,
        has_one = owner,
        close = owner
    )]
    pub position: Account<'info, UserPosition>,
}

#[account]
pub struct VaultConfig {
    pub authority: Pubkey,
    pub btc_mint: Pubkey,
    pub musd_mint: Pubkey,
    pub min_collateral: u64,
    pub fee_rate_bps: u64,
    pub total_btc_deposited: u64,
    pub total_musd_borrowed: u64,
    pub bump: u8,
}

impl VaultConfig {
    const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
}

#[account]
pub struct UserPosition {
    pub owner: Pubkey,
    pub btc_deposited: u64,
    pub musd_borrowed: u64,
    pub bump: u8,
}

impl UserPosition {
    const LEN: usize = 32 + 8 + 8 + 1;
}

#[account]
pub struct PriceFeed {
    pub oracle: Pubkey,
    pub price: u64,
    pub updated_at: i64,
    pub bump: u8,
}

impl PriceFeed {
    const LEN: usize = 32 + 8 + 8 + 1;
}

#[event]
pub struct CollateralDeposited {
    pub owner: Pubkey,
    pub amount: u64,
    pub total_btc: u64,
}

#[event]
pub struct CollateralWithdrawn {
    pub owner: Pubkey,
    pub amount: u64,
    pub total_btc: u64,
}

#[event]
pub struct Borrowed {
    pub owner: Pubkey,
    pub amount: u64,
    pub total_debt: u64,
}

#[event]
pub struct Repaid {
    pub owner: Pubkey,
    pub amount: u64,
    pub remaining_debt: u64,
}

#[event]
pub struct Liquidated {
    pub owner: Pubkey,
    pub liquidator: Pubkey,
    pub debt_covered: u64,
    pub collateral_seized: u64,
}

#[error_code]
pub enum CollatError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Insufficient collateral balance")]
    InsufficientCollateral,
    #[msg("Outstanding loan must be repaid first")]
    OutstandingLoan,
    #[msg("Borrow amount exceeds maximum LTV")]
    ExceedsMaxLTV,
    #[msg("Repay amount exceeds debt")]
    OverRepay,
    #[msg("Position is not liquidatable")]
    NotLiquidatable,
    #[msg("Unauthorized oracle")]
    UnauthorizedOracle,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    #[msg("Collateral still remaining in position")]
    CollateralRemaining,
}
