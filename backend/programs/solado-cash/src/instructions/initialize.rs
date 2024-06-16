use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };

use crate::{ contants::{ POOL_VAULT_AMOUNT_SEED, POOL_VAULT_SEED }, state::MerkleMountainRange };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub pool_token: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = 8 + MerkleMountainRange::INIT_SPACE,
        seeds = [POOL_VAULT_SEED, POOL_VAULT_AMOUNT_SEED, pool_token.key().as_ref()],
        bump
    )]
    pub mmr_account: Account<'info, MerkleMountainRange>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = pool_token,
        associated_token::authority = mmr_account
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let mmr_account = &mut ctx.accounts.mmr_account;

    // Fake randomness
    let slot = Clock::get()?.slot;
    let slot_bytes = slot.to_le_bytes();
    let mmr_account_bytes = mmr_account.to_account_info().key.as_ref();
    let mut combined_bytes = [0u8; 32];
    combined_bytes[..8].copy_from_slice(&slot_bytes);
    combined_bytes[8..].copy_from_slice(&mmr_account_bytes[..24]);

    // Initialize with a dummy node
    mmr_account.nodes = vec![combined_bytes];
    mmr_account.peaks = vec![0];
    mmr_account.deposit_count = 0;
    Ok(())
}
