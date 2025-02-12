use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Mints {
    pub atlas: Pubkey,
    pub polis: Pubkey,
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
    pub authority: Pubkey,
    pub mints: Mints,
}

#[account]
#[derive(InitSpace)]
pub struct EventsManagerState {
    pub authority: Pubkey,
    pub events_manager: Pubkey,
    pub vault_fee_info: VaultFeeInfo,
    pub vault_owner_info: VaultOwnerInfo,
}
