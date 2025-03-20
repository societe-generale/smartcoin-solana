#[anchor_lang::error_code]
pub enum TransferHookError {
    #[msg("The transfers have been disabled")]
    TransfersDisabled,
    #[msg("The token is not currently transferring")]
    IsNotCurrentlyTransferring,
    #[msg("redemption not found")]
    RedemptionNotFound,
    #[msg("bad date")]
    BadDate,
    #[msg("bad amount")]
    BadAmount,
    #[msg("Token is paused, no transfer available.")]
    TokenIsPaused,
    #[msg("No pending redemption found.")]
    NoPendingRedemption,
    #[msg("Bad account.")]
    BadAccount,
    #[msg("Operation cannot use delegate account.")]
    OperationDelegate,
    #[msg("Permanent delegate is not allowed to transfer tokens")]
    RegistrarDelegate,
}