use std::str::FromStr;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use anchor_spl::token_interface::spl_token_2022::state::Mint as Token2022Mint;
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;
use anchor_spl::token_2022::ID as TOKEN_2022_PROGRAM_ID;

#[derive(Accounts)]
pub struct CancelRedemption<'info> {
    #[account(
        mut
    )]
    pub registrar: Signer<'info>,

    #[account(
        mut,
        seeds = [b"redemption", mint.key().as_ref()],
        bump = redemptions.bump,
    )]
    pub redemptions: Box<Account<'info, Redemptions>>,
    #[account(mut, mint::token_program = TOKEN_2022_PROGRAM_ID)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct RedemptionCancelled {
    pub amount: u64,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub end_date: i64,
}

pub(crate) fn handler(ctx: Context<CancelRedemption>, amount: u64, sender: Pubkey, recipient: Pubkey, end_date: i64) -> Result<()> {
    let mint_info = ctx.accounts.mint.to_account_info();
    let mint_data = mint_info.data.borrow();
    let mint = StateWithExtensions::<Token2022Mint>::unpack(&mint_data)?;
    if let Ok(metadata) = mint.get_variable_len_extension::<TokenMetadata>() {
        if let Some((_, meta_registrar)) = metadata.additional_metadata.iter().find(|(key, _)| key == "registrar") {
            if Pubkey::from_str(meta_registrar).unwrap() != ctx.accounts.registrar.key() {
                return err!(error_message::TransferHookError::BadAccount);
            }
        }
    }
    let redemptions = &mut ctx.accounts.redemptions;
    let redemption_index = redemptions.redemption.iter().position(|r| r.sender == sender && r.amount == amount && r.recipient == recipient && r.end_date == end_date);
    if redemption_index.is_some(){
        redemptions.redemption.remove(redemption_index.unwrap());
    } else {
        return err!(error_message::TransferHookError::RedemptionNotFound);
    }
    emit!(RedemptionCancelled {
        amount,
        sender,
        recipient,
        end_date
    });
    Ok(())
}