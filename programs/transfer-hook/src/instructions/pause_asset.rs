use anchor_lang::prelude::*;
use anchor_spl::token_2022::ID as TOKEN_2022_PROGRAM_ID;
use anchor_spl::token_interface::spl_token_metadata_interface::state::Field;
use anchor_spl::token_interface::{
    token_metadata_update_field, Mint, Token2022, TokenMetadataUpdateField,
};

#[derive(Accounts)]
pub struct PauseAsset<'info> {
    #[account(mut)]
    pub registrar: Signer<'info>,

    #[account(mut, mint::token_program = TOKEN_2022_PROGRAM_ID)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokenPaused {
    pub registrar: Pubkey,
    pub timestamp: i64,
}

pub(crate) fn handler(ctx: Context<PauseAsset>) -> Result<()> {
    let cpi_accounts = TokenMetadataUpdateField {
        metadata: ctx.accounts.mint.to_account_info(),
        update_authority: ctx.accounts.registrar.to_account_info(),
        token_program_id: ctx.accounts.token_program.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token_metadata_update_field(
        cpi_ctx,
        Field::Key("paused".to_string()),
        "true".to_string(),
    )?;
    emit!(TokenPaused {
        registrar: ctx.accounts.registrar.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}
