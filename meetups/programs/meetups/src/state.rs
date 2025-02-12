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
    /// Address of the EventsManager account
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

#[account]
#[derive(InitSpace)]
pub struct EventEntry {
    /// Address of the HostProfile account
    pub host: Pubkey,
    /// Name of the event
    #[max_len(32)]
    pub name: String,
}
