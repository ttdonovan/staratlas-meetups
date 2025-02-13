use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Mints {
    /// Mint address of Atlas token
    pub atlas: Pubkey,
    /// Mint address of Polis token
    pub polis: Pubkey,
    /// Mint address of USDC token
    pub usdc: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct VaultFeeInfo {
    pub dao_vault_fee: u8,
    pub dev_vault_fee: u8,
    pub ops_vault_fee: u8,
    pub host_profile_fee: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct VaultOwnerInfo {
    pub dao_vault_owner: Pubkey,
    pub dev_vault_owner: Pubkey,
    pub ops_vault_owner: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct EventsManager {
    /// Authority to make changes to the EventsManager
    pub authority: Pubkey,
    pub mints: Mints,
}

#[account]
#[derive(InitSpace)]
pub struct EventsManagerState {
    /// Authority to make changes to the EventsManagerState
    pub authority: Pubkey,
    /// EventsManager PDA
    pub events_manager: Pubkey,
    pub vault_fee_info: VaultFeeInfo,
    pub vault_owner_info: VaultOwnerInfo,
}

#[account]
#[derive(InitSpace)]
pub struct IdentityProfile {
    /// Pubkey of the user's wallet
    pub owner: Pubkey,
    /// Name of the user
    #[max_len(32)]
    pub name: String,
}

#[account]
#[derive(InitSpace)]
pub struct HostProfile {
    pub events_manager: Pubkey,
    pub identity_profile: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, InitSpace)]
pub enum EventStatusType {
    /// Event when 'pending' can be updated with additional information
    Pending,
    /// Event when 'open' is open for registration
    Open,
    /// Event when 'closed' is closed for registration (can be re-opened for registration)
    Closed,
    /// Event when 'cancelled' is cancelled (no registration allowed)
    Cancelled,
    /// Event when 'completed' is completed (no registration allowed)
    Completed,
}

#[account]
#[derive(InitSpace)]
pub struct EventEntry {
    pub status: EventStatusType,
    /// HostProfile PDA
    pub host: Pubkey,
    /// Name of the event
    #[max_len(32)]
    pub name: String,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    #[max_len(32)]
    pub location: String,
    #[max_len(300)]
    pub mappable_address: String,
    pub start_time_at: u64,
    pub end_time_at: u64,
}
