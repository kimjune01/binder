fn main() {
    let revision = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: mollusk-trial <vulnerable|fixed>");
        std::process::exit(2);
    });
    let fixed = match revision.as_str() {
        "vulnerable" => false,
        "fixed" => true,
        _ => {
            eprintln!("unknown revision: {revision}");
            std::process::exit(2);
        }
    };
    if vault_mollusk::unauthorized_withdrawal_preserves_balances(fixed) {
        println!("PASS: Mollusk observed rejection with unchanged balances");
    } else {
        eprintln!("FAIL: Mollusk observed an unauthorized balance transfer");
        std::process::exit(1);
    }
}
