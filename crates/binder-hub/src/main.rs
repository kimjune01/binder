use binder_hub::{load_cases, render_hub};
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("hub/dist"));
    let cases = load_cases(&root.join("hub/cases")).unwrap_or_else(|e| panic!("{e}"));
    render_hub(&root, &cases, &output).unwrap_or_else(|e| panic!("{e}"));
    eprintln!("Rendered {} cases to {}", cases.len(), output.display());
}
