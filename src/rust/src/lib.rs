use extendr_api::prelude::*;

use segul::helper::alphabet;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn is_dna(string: &str) -> bool {
  alphabet::is_valid_dna(string)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rsegul;
    fn is_dna;
}
