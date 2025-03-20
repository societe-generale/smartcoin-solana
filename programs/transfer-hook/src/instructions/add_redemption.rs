use std::str::FromStr;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use anchor_spl::token_interface::spl_token_2022::state::Mint as Token2022Mint;
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;
use anchor_spl::token_2022::ID as TOKEN_2022_PROGRAM_ID;


#[derive(Accounts)]
pub struct AddRedemption<'info> {
    #[account( mut)]
    pub registrar: Signer<'info>,

    #[account(
        mut,
        seeds = [b"redemption", mint.key().as_ref()],
        bump = redemptions.bump,
        realloc = Redemptions::space(redemptions.max_size),
        realloc::payer = registrar,
        realloc::zero = true,

    )]
    pub redemptions: Box<Account<'info, Redemptions>>,
    #[account(mut, mint::token_program = TOKEN_2022_PROGRAM_ID)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct RedemptionAdded {
    pub amount: u64,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub end_date: i64,
}

pub(crate) fn handler(ctx: Context<AddRedemption>, amount: u64, sender: Pubkey, recipient: Pubkey, end_date: i64) -> Result<()> {
    if is_past(end_date) {
        return err!(error_message::TransferHookError::BadDate);
    }
    if amount == 0 {
        return err!(error_message::TransferHookError::BadAmount);
    }
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
    redemptions.redemption.retain(|r| !r.done && !is_past(r.end_date));

    let redemption = Redemption {
        amount,
        sender,
        recipient,
        done: false,
        end_date,
    };
    redemptions.redemption.push(redemption);
    if redemptions.max_size < redemptions.redemption.len() as u64 {
        redemptions.max_size = redemptions.redemption.len() as u64
    }
    emit!(RedemptionAdded {
        amount,
        sender,
        recipient,
        end_date
    });
    Ok(())
}