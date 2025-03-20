use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_2022::extension::transfer_hook::TransferHookAccount;
use anchor_spl::token_interface::spl_token_2022::extension::{
    BaseStateWithExtensions, StateWithExtensions,
};
use anchor_spl::token_interface::spl_token_2022::extension::{
    BaseStateWithExtensionsMut, PodStateWithExtensionsMut,
};
use anchor_spl::token_interface::spl_token_2022::pod::PodAccount;
use anchor_spl::token_interface::spl_token_2022::state::Mint as Token2022Mint;
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::TokenAccount;
use std::cell::RefMut;
use std::str::FromStr;
#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// CHECK: we don't need to check source account, hook only looks at the mint
    pub source_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: we don't need to check recipient account, hook only looks at the mint
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    ///CHECK:
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"redemption", mint.key().as_ref()],
        bump = redemptions.bump,
    )]
    pub redemptions: Box<Account<'info, Redemptions>>,
}

pub(crate) fn handler(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
    if !ctx.accounts.assert_is_transferring() {
        return err!(error_message::TransferHookError::IsNotCurrentlyTransferring);
    }
    let mut registrar: Option<Pubkey> = None;
    let mut operations: Option<Pubkey> = None;
    let mint_info = ctx.accounts.mint.to_account_info();
    let mint_data = mint_info.data.borrow();
    let mint = StateWithExtensions::<Token2022Mint>::unpack(&mint_data)?;
    if let Ok(metadata) = mint.get_variable_len_extension::<TokenMetadata>() {
        if let Some((_, meta_paused)) = metadata
            .additional_metadata
            .iter()
            .find(|(key, _)| key == "paused")
        {
            if meta_paused != "false" {
                return err!(error_message::TransferHookError::TokenIsPaused);
            }
        } else {
            return err!(error_message::TransferHookError::TokenIsPaused);
        }
        if let Some((_, meta_registrar)) = metadata
            .additional_metadata
            .iter()
            .find(|(key, _)| key == "registrar")
        {
            registrar = Some(Pubkey::from_str(meta_registrar).unwrap());
        } else {
            return err!(error_message::TransferHookError::BadAccount);
        }
        if let Some((_, meta_operations)) = metadata
            .additional_metadata
            .iter()
            .find(|(key, _)| key == "operations")
        {
            operations = Some(Pubkey::from_str(meta_operations).unwrap());
        } else {
            return err!(error_message::TransferHookError::BadAccount);
        }
    }

    if ctx.accounts.source_account.owner != registrar.unwrap()
        && ctx.accounts.owner.key() == registrar.unwrap()
        && ctx.accounts.destination_token.owner == registrar.unwrap()
    {
        return err!(error_message::TransferHookError::RegistrarDelegate);
    }
    if ctx.accounts.source_account.owner != operations.unwrap()
        && ctx.accounts.owner.key() == operations.unwrap()
    {
        return err!(error_message::TransferHookError::OperationDelegate);
    }
    let redemptions = &mut ctx.accounts.redemptions;
    if (ctx.accounts.destination_token.owner == registrar.unwrap()
        && ctx.accounts.owner.key() != registrar.unwrap())
        || (ctx.accounts.destination_token.owner == operations.unwrap()
            && ctx.accounts.owner.key() != operations.unwrap())
    {
        let redemption_index = redemptions.redemption.iter().position(|r| {
            r.amount == amount
                && r.sender == ctx.accounts.source_account.owner.key()
                && r.recipient == ctx.accounts.destination_token.owner
                && r.done == false
                && !is_past(r.end_date)
        });
        if redemption_index.is_some() {
            redemptions.redemption[redemption_index.unwrap()].done = true;
            return Ok(());
        }
        return err!(error_message::TransferHookError::NoPendingRedemption);
    }
    Ok(())
}

impl<'info> TransferHook<'info> {
    fn assert_is_transferring(&self) -> bool {
        let source_token_info = self.source_account.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> =
            source_token_info.try_borrow_mut_data().unwrap();
        let mut account =
            PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref).unwrap();
        let account_extension = account.get_extension_mut::<TransferHookAccount>().unwrap();

        bool::from(account_extension.transferring)
    }
}
