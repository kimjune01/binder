use vault_mollusk::unauthorized_withdrawal_preserves_balances;

#[test]
fn vulnerable_account_metas_allow_the_unauthorized_transfer() {
    assert!(!unauthorized_withdrawal_preserves_balances(false));
}

#[test]
fn fixed_account_metas_reject_and_rollback_the_unauthorized_transfer() {
    assert!(unauthorized_withdrawal_preserves_balances(true));
}
