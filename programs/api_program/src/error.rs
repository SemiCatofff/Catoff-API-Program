use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The requested operation is not authorized.")]
    Unauthorized,

    #[msg("Insufficient funds to complete the operation.")]
    InsufficientFunds,

    #[msg("The specified currency is not supported.")]
    UnsupportedCurrency,

    #[msg("Failed to perform the transfer.")]
    TransferFailed,

    #[msg("Invalid input provided.")]
    InvalidInput,
    // Add more error codes as needed for your specific contract logic
}
