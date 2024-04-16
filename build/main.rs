mod helpers;

#[allow(unused_parens, unused_imports)]
use num_bigint::*;

#[path = "../src/lifts.rs"]
mod lifts;

#[path = "../src/specs/mod.rs"]
mod specs;

fn main() {
    specs::specs();
    let count = helpers::COUNT.lock().unwrap();
    std::fs::write(
        "src/generated_doc.rs",
        format!(
            "//! This module contains placeholder functions decorated with contracts and concrete tests. There are {count} tests.\n{}",
            helpers::OutKind::Rustdoc.dump()
        ),
    );
    std::fs::write(
        "src/test_driver.rs",
        format!(
            "//! This module contains {count} tests, organized in functions.\n{}\n{}\nfn main(){{{}}}", 
            "#![allow(arithmetic_overflow)]
use core_spec::lifts::*;
use core_spec::*;
",
            helpers::OutKind::VanillaBin.dump(),
            helpers::list_functions()
                .iter()
                .map(|r#fn| format!("{}();", r#fn))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
}
