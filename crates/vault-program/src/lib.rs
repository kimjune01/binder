use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

const STATE_LEN: usize = 17;
const REJECTED: u8 = 1;

/// State layout: vault balance (u64 LE), recipient balance (u64 LE), result.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let state = next_account_info(&mut accounts.iter())?;
    if !state.is_writable || instruction_data.len() != 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let authorized = instruction_data[8] != 0;
    let mut data = state.try_borrow_mut_data()?;
    if data.len() != STATE_LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    let vault = read_u64(&data[0..8])?;
    let recipient = read_u64(&data[8..16])?;

    #[cfg(feature = "vulnerable")]
    if !authorized {
        write_u64(
            &mut data[0..8],
            vault
                .checked_sub(amount)
                .ok_or(ProgramError::InsufficientFunds)?,
        );
        write_u64(
            &mut data[8..16],
            recipient
                .checked_add(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        );
        data[16] = REJECTED;
        return Ok(());
    }

    if !authorized {
        data[16] = REJECTED;
        return Ok(());
    }

    write_u64(
        &mut data[0..8],
        vault
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?,
    );
    write_u64(
        &mut data[8..16],
        recipient
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?,
    );
    data[16] = 0;
    Ok(())
}

fn read_u64(bytes: &[u8]) -> Result<u64, ProgramError> {
    Ok(u64::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    ))
}

fn write_u64(bytes: &mut [u8], value: u64) {
    bytes.copy_from_slice(&value.to_le_bytes());
}
