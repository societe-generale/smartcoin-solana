use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{token_interface::Mint};
use spl_pod::primitives::PodBool;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
#[derive(Accounts)]
pub struct InitializeExtraAccountMeta<'info> {
    #[account(mut)]
    pub technical: Signer<'info>,

    ///CHECK:
    #[account()]
    pub registrar: UncheckedAccount<'info>,

    #[account(
        init,
        payer = technical,
        seeds = [b"redemption", mint.key().as_ref()],
        bump,
        space = Redemptions::INIT_SIZE,
    )]
    pub redemptions: Box<Account<'info, Redemptions>>,
    ///CHECK: This account's data is a buffer of TLV data
    #[account(
        init,
        space = ExtraAccountMetaList::size_of(1).unwrap(),
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = technical,
    )]
    pub extra_metas_account: UncheckedAccount<'info>,

    #[account(

    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub(crate) fn handler(ctx: Context<InitializeExtraAccountMeta>) -> Result<()> {
    let redemptions = &mut ctx.accounts.redemptions;
    redemptions.bump = ctx.bumps.redemptions;
    redemptions.max_size = 0;
    let account_metas = vec![
        ExtraAccountMeta {
            discriminator: 0,
            address_config: ctx.accounts.redemptions.key().to_bytes(),
            is_signer: PodBool::from(false),
            is_writable: PodBool::from(true),
        }
    ];


    let extra_metas_account = &ctx.accounts.extra_metas_account;
    let mut data = extra_metas_account.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &account_metas)?;


    Ok(())
}