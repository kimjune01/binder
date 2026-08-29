#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Balances {
    pub vault: u64,
    pub recipient: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Withdrawal {
    pub amount: u64,
    pub authorized: bool,
}

pub fn apply_vulnerable(
    before: Balances,
    withdrawal: Withdrawal,
) -> (Balances, Result<(), &'static str>) {
    let Some(vault) = before.vault.checked_sub(withdrawal.amount) else {
        return (before, Err("insufficient funds"));
    };
    let Some(recipient) = before.recipient.checked_add(withdrawal.amount) else {
        return (before, Err("recipient balance overflow"));
    };
    let after = Balances { vault, recipient };
    if !withdrawal.authorized {
        return (after, Err("unauthorized"));
    }
    (after, Ok(()))
}

pub fn apply_fixed(
    before: Balances,
    withdrawal: Withdrawal,
) -> (Balances, Result<(), &'static str>) {
    if !withdrawal.authorized {
        return (before, Err("unauthorized"));
    }
    let Some(vault) = before.vault.checked_sub(withdrawal.amount) else {
        return (before, Err("insufficient funds"));
    };
    let Some(recipient) = before.recipient.checked_add(withdrawal.amount) else {
        return (before, Err("recipient balance overflow"));
    };
    (Balances { vault, recipient }, Ok(()))
}
