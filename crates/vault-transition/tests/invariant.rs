use vault_transition::{Balances, Withdrawal, apply_fixed, apply_vulnerable};

type Transition = fn(Balances, Withdrawal) -> (Balances, Result<(), &'static str>);

fn preserves_balances_on_error(apply: Transition) -> bool {
    for vault in 0..=12 {
        for recipient in 0..=12 {
            for amount in 0..=12 {
                for authorized in [false, true] {
                    let before = Balances { vault, recipient };
                    let withdrawal = Withdrawal { amount, authorized };
                    let (after, result) = apply(before, withdrawal);
                    if result.is_err() && after != before {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[test]
fn vulnerable_revision_has_a_counterexample() {
    assert!(!preserves_balances_on_error(apply_vulnerable));
}

#[test]
fn fixed_revision_preserves_balances_on_every_explored_error_path() {
    assert!(preserves_balances_on_error(apply_fixed));
}

#[test]
fn fixture_exposes_the_unauthorized_withdrawal_bug() {
    let before = Balances {
        vault: 100,
        recipient: 5,
    };
    let withdrawal = Withdrawal {
        amount: 10,
        authorized: false,
    };

    let (vulnerable_after, vulnerable_result) = apply_vulnerable(before, withdrawal);
    assert!(vulnerable_result.is_err());
    assert_eq!(vulnerable_after.vault, 90);
    assert_eq!(vulnerable_after.recipient, 15);

    let (fixed_after, fixed_result) = apply_fixed(before, withdrawal);
    assert!(fixed_result.is_err());
    assert_eq!(fixed_after, before);
}
