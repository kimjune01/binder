mod escrow;

use escrow::Escrow;

fn main() {
    let mut state = Escrow {
        held: 100,
        recipient: 5,
    };
    state.release(10, true, false);
    if state.held == 100 && state.recipient == 5 {
        println!(r#"{{"observation":"stood","witness":{{"held":{},"recipient":{}}}}}"#, state.held, state.recipient);
    } else {
        println!(r#"{{"observation":"refuted","witness":{{"held":{},"recipient":{}}}}}"#, state.held, state.recipient);
    }
}
