use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Event is not pending")]
    EventNotPending,
    #[msg("Event is not open")]
    EventNotOpen,
}
