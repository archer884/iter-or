//! # Volunteers
//!
//! The concept behind this example is that of the cartoon cliche, the call for volunteers.
//! Specifically, a commanding officer requests volunteers for a dangerous (or possibly just
//! disgusting) mission by asking that interested parties take `n` steps forward. Instead,
//! the more alert parties present simply take `n` steps back, leaving (usually) one
//! unlucky and inattentive soul as the "volunteer."

extern crate iter_or;

use iter_or::IterOr;

fn main() {
    let contingent = vec!["Joe".to_string(), "Marty".to_string(), "Frank".to_string()];
    let volunteers = contingent.into_iter().filter(|soldier| is_willing(soldier));
    let volunteers = volunteers.or_else(|| "Joe".to_string()); // poor Joe!

    for name in volunteers {
        // only Joe appears here :(
        println!("{}", name);
    }
}

fn is_willing(_soldier: &str) -> bool {
    false
}
