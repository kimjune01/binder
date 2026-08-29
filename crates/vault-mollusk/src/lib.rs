//! Mollusk-backed observation of the vault authorization boundary.
//!
//! Each revision is a distinct compiled sBPF ELF. Mollusk executes the ELF and
//! the oracle reads the program's application-level rejection from account data.

use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::{fs, path::PathBuf};

const VAULT_BALANCE: u64 = 100;
const RECIPIENT_BALANCE: u64 = 5;
const WITHDRAWAL: u64 = 10;
const REJECTED: u8 = 1;
const PROGRAM_ID: Pubkey = Pubkey::new_from_array([7; 32]);

/// Execute an unauthorized withdrawal in Mollusk's SVM and check the claim.
///
/// The instruction succeeds at the SVM level and records the application-level
/// rejection in state. The predicate requires rejected withdrawals to preserve
/// both balances.
pub fn unauthorized_withdrawal_preserves_balances(fixed: bool) -> bool {
    let state = Pubkey::new_from_array([8; 32]);
    let mut instruction_data = WITHDRAWAL.to_le_bytes().to_vec();
    instruction_data.push(0); // unauthorized
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data,
        vec![AccountMeta::new(state, false)],
    );

    let mut state_data = Vec::with_capacity(17);
    state_data.extend_from_slice(&VAULT_BALANCE.to_le_bytes());
    state_data.extend_from_slice(&RECIPIENT_BALANCE.to_le_bytes());
    state_data.push(0);

    let accounts = [(
        state,
        Account {
            lamports: 1_000_000,
            data: state_data,
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )];
    let mut mollusk = Mollusk::default();
    let elf = fs::read(elf_path(fixed)).expect("build vault-program sBPF artifacts first");
    mollusk.add_program_with_loader_and_elf(&PROGRAM_ID, &solana_sdk_ids::bpf_loader::ID, &elf);
    let result = mollusk.process_instruction(&instruction, &accounts);
    let data = result
        .resulting_accounts
        .iter()
        .find(|(key, _)| key == &state)
        .map(|(_, account)| account.data.as_slice());

    result.raw_result.is_ok()
        && data.is_some_and(|data| {
            data.get(16) == Some(&REJECTED)
                && read_u64(&data[0..8]) == VAULT_BALANCE
                && read_u64(&data[8..16]) == RECIPIENT_BALANCE
        })
}

fn elf_path(fixed: bool) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy")
        .join(if fixed { "fixed" } else { "vulnerable" })
        .join("vault_program.so")
}

fn read_u64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes.try_into().expect("eight-byte balance"))
}
