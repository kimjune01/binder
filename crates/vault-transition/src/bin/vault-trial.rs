use std::env;

use vault_transition::{Balances, Withdrawal, apply_fixed, apply_vulnerable};

type Transition = fn(Balances, Withdrawal) -> (Balances, Result<(), &'static str>);

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let [kind, revision] = args.as_slice() else {
        eprintln!("usage: vault-trial <proof|runtime> <vulnerable|fixed>");
        std::process::exit(2);
    };
    let apply: Transition = match revision.as_str() {
        "vulnerable" => apply_vulnerable,
        "fixed" => apply_fixed,
        _ => {
            eprintln!("unknown revision: {revision}");
            std::process::exit(2);
        }
    };
    let passed = match kind.as_str() {
        "proof" => bounded_proof(apply),
        "runtime" => runtime_fixture(apply),
        _ => {
            eprintln!("unknown trial: {kind}");
            std::process::exit(2);
        }
    };
    if passed {
        println!("PASS: failed withdrawals preserve vault and recipient balances");
    } else {
        eprintln!("FAIL: rejected withdrawal changed balances");
        std::process::exit(1);
    }
}

fn bounded_proof(apply: Transition) -> bool {
    for vault in 0..=12 {
        for recipient in 0..=12 {
            for amount in 0..=12 {
                for authorized in [false, true] {
                    let before = Balances { vault, recipient };
                    let (after, result) = apply(before, Withdrawal { amount, authorized });
                    if result.is_err() && after != before {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn runtime_fixture(apply: Transition) -> bool {
    let before = Balances {
        vault: 100,
        recipient: 5,
    };
    let (after, result) = apply(
        before,
        Withdrawal {
            amount: 10,
            authorized: false,
        },
    );
    result.is_err() && after == before
}
