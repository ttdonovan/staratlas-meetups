use anchor_lang::InstructionData;
use litesvm::LiteSVM;
use solana_sdk::{
    feature_set::{disable_new_loader_v3_deployments, FeatureSet},
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};

use meetups::{instruction::InitEventManager, ID as PROGRAM_ID};

use std::path::PathBuf;
use std::str::FromStr;

fn read_meetups_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/meetups.so");
    std::fs::read(so_path).unwrap()
}

// cargo test -- --nocapture
#[test]
pub fn meetups_program_test() {
    let mut feature_set = FeatureSet::all_enabled();
    feature_set.deactivate(&disable_new_loader_v3_deployments::id());
    let mut svm = LiteSVM::default()
        .with_builtins(Some(feature_set))
        .with_lamports(1_000_000_000_000_000)
        .with_sysvars();

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();

    let program_id = pubkey!("C14GzDxp9S1UfZk1BR1BPwHK1erFoPWkvjjFRtyf4B7L");
    assert_eq!(program_id.to_string(), PROGRAM_ID.to_string());

    let program_bytes = read_meetups_program();

    svm.airdrop(&payer_pk, 10_000_000_000).unwrap();
    svm.add_program(program_id, &program_bytes);

    let (event_manager, _bump) =
        Pubkey::find_program_address(&[b"manager", payer_pk.as_ref()], &program_id);
    let (event_manager_state, _bump) =
        Pubkey::find_program_address(&[b"state", payer_pk.as_ref()], &program_id);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_pk, true), // authority (writable, signer)
            AccountMeta::new(event_manager, false), // event_manager PDA (writable)
            AccountMeta::new(event_manager_state, false), // event_manager_state PDA (writable)
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: InitEventManager {
            atlas_mint: anchor_lang::prelude::Pubkey::from_str(
                "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx",
            )
            .unwrap(),
            polis_mint: anchor_lang::prelude::Pubkey::from_str(
                "poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk",
            )
            .unwrap(),
            usdc_mint: anchor_lang::prelude::Pubkey::from_str(
                "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            )
            .unwrap(),
        }
        .data(),
    };

    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx = Transaction::new(&[&payer_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);

    assert!(tx_result.is_ok());
    let result = tx_result.unwrap();

    assert!(result.logs.contains(&format!(
        "Program log: Event Manager initialized by: {}",
        &payer_pk
    )));

    // todo!();
}
