pub mod state;
mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use spl_transfer_hook_interface::instruction::TransferHookInstruction;
#[cfg(not(feature = "no-entrypoint"))]
use  solana_security_txt::security_txt;

declare_id!("JCiN3FoAn68Mx4JaQ546viikunXujsPNvoDFYdKupboM");

#[program]
pub mod transfer_hook {
    use super::*;

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_meta(ctx: Context<InitializeExtraAccountMeta>) -> Result<()> {
        instructions::initialize_extra_account_meta::handler(ctx)
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        instructions::transfer_hook::handler(ctx, amount)
    }

    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data)?;

        // match instruction discriminator to transfer hook interface execute instruction
        // token2022 program CPIs this instruction on token transfer
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();

                // invoke custom transfer hook instruction on our program
                __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
    }


    pub fn pause_asset(ctx: Context<PauseAsset>) -> Result<()> {
        instructions::pause_asset::handler(ctx)
    }

    pub fn unpause_asset(ctx: Context<UnpauseAsset>) -> Result<()> {
        instructions::unpause_asset::handler(ctx)
    }
    pub fn add_redemption(ctx: Context<AddRedemption>, amount: u64, sender: Pubkey, recipient: Pubkey, end_date: i64) -> Result<()> {
        instructions::add_redemption::handler(ctx, amount, sender, recipient, end_date)
    }
    pub fn cancel_redemption(ctx: Context<CancelRedemption>, amount: u64, sender: Pubkey, recipient: Pubkey, end_date: i64) -> Result<()> {
        instructions::cancel_redemption::handler(ctx, amount, sender, recipient, end_date)
    }

    #[cfg(not(feature = "no-entrypoint"))]
    security_txt! {
        name: "CoinVertible transfer hook",
        project_url: "https://www.sgforge.com/coinvertible-solana-transferhook-project-url/",
        contacts: "link:https://www.sgforge.com/coinvertible-solana-transferhook-contacts/",
        policy: "https://www.sgforge.com/coinvertible-solana-transferhook-policy/"
    }
}

