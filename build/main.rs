mod helpers;

#[allow(unused_parens, unused_imports)]
use num_bigint::*;

#[path = "../src/lifts.rs"]
mod lifts;

#[path = "../src/specs/mod.rs"]
mod specs;

fn main() {
    specs::specs();
    let _ = std::fs::write(
        "src/generated.rs",
        helpers::OUTPUT.lock().unwrap().join("\n"),
    );
}
