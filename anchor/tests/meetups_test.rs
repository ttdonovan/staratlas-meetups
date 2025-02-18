use anchor_lang::{prelude::Pubkey as AnchorPubkey, AccountDeserialize, InstructionData};
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

use meetups::{
    instruction::{CreateEvent, InitEventManager, InitIdentityProfile, UpdateEvent},
    EventEntry, ID as PROGRAM_ID,
};

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

    // initialize an event manager
    let (event_manager, _bump) =
        Pubkey::find_program_address(&[b"manager", payer_pk.as_ref()], &program_id);
    let (event_manager_state, _bump) =
        Pubkey::find_program_address(&[b"state", payer_pk.as_ref()], &program_id);

    let atlas_mint =
        AnchorPubkey::from_str("ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx").unwrap();
    let polis_mint = AnchorPubkey::from_str("poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk").unwrap();
    let usdc_mint = AnchorPubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_pk, true), // authority (writable, signer)
            AccountMeta::new(event_manager, false), // event_manager PDA (writable)
            AccountMeta::new(event_manager_state, false), // event_manager_state PDA (writable)
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: InitEventManager {
            atlas_mint,
            polis_mint,
            usdc_mint,
        }
        .data(),
    };

    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx = Transaction::new(&[&payer_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);

    assert!(tx_result.is_ok());
    let result = tx_result.unwrap();

    assert!(result
        .logs
        .contains(&format!("Program log: Instruction: InitEventManager")));
    assert!(result.logs.contains(&format!(
        "Program log: Event Manager initialized by: {}",
        &payer_pk
    )));

    // initialize an identity profile
    let (identity_profile, _bump) =
        Pubkey::find_program_address(&[b"identity", payer_pk.as_ref()], &program_id);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_pk, true), // funder (writable, signer)
            AccountMeta::new(identity_profile, false), // identity_profile PDA (writable)
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: InitIdentityProfile {
            name: "SpaceCadet#007".to_string(),
        }
        .data(),
    };

    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx = Transaction::new(&[&payer_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);

    assert!(tx_result.is_ok());
    let result = tx_result.unwrap();

    assert!(result
        .logs
        .contains(&format!("Program log: Instruction: InitIdentityProfile")));

    // create an event
    let year: u16 = 2025;
    let month: u8 = 2;
    let day: u8 = 1;

    let (host_profile, _bump) = Pubkey::find_program_address(
        &[b"host", event_manager.as_ref(), identity_profile.as_ref()],
        &program_id,
    );

    let (event, _bump) = Pubkey::find_program_address(
        &[
            b"event",
            event_manager.as_ref(),
            host_profile.as_ref(),
            &year.to_le_bytes(),
            &month.to_le_bytes(),
            &day.to_le_bytes(),
        ],
        &program_id,
    );

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_pk, true), // funder (writable, signer)
            AccountMeta::new(identity_profile, false), // identity_profile PDA (writable)
            AccountMeta::new(host_profile, false), // host_profile PDA (writable)
            AccountMeta::new(event, false),   // event PDA (writable)
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: CreateEvent {
            event_manager_id: AnchorPubkey::new_from_array(event_manager.to_bytes()),
            year,
            month,
            day,
            name: "PENDING: Star Atlas Community Meetup".to_string(),
        }
        .data(),
    };

    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx = Transaction::new(&[&payer_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);

    assert!(tx_result.is_ok());
    let result = tx_result.unwrap();

    assert!(result
        .logs
        .contains(&format!("Program log: Instruction: CreateEvent")));

    let event_account = svm.get_account(&event).unwrap();
    let event_data = EventEntry::try_deserialize(&mut &event_account.data[..]).unwrap();
    assert_eq!(event_data.name, "PENDING: Star Atlas Community Meetup");

    // update an event
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_pk, true), // signer (writeable, signer)
            AccountMeta::new(identity_profile, false), // identity_profile PDA (writable)
            AccountMeta::new(host_profile, false), // host_profile PDA (writable)
            AccountMeta::new(event, false),   // event PDA (writable)
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: UpdateEvent {
            event_manager_id: AnchorPubkey::new_from_array(event_manager.to_bytes()),
            year,
            month,
            day,
            name: "Star Atlas Community Meetup".to_string(),
            location: "21st Amendment Brewery".to_string(),
            mappable_address: "563 2nd St, San Francisco, CA 94107".to_string(),
            start_time_at: 1708531200, // Feb 21, 2024 12:00:00 UTC
            end_time_at: 1708538400,   // Feb 21, 2024 14:00:00 UTC
            entry_token_mint: usdc_mint,
            entry_token_amount: 25_000_000, // $25 Dollars (USDC)
        }
        .data(),
    };

    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx = Transaction::new(&[&payer_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);

    assert!(tx_result.is_ok());
    let result = tx_result.unwrap();

    assert!(result
        .logs
        .contains(&format!("Program log: Instruction: UpdateEvent")));

    // todo!();
}
