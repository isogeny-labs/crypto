//! Cell G3 - C3 census (H-BINSTD-c72284 Stage 2 / EXP-BINSTD-372679), prepared, NOT RUN.
//!
//! std-only. The CDCL+native-XOR solver is the repository's own
//! `src/cryptanalysis/sat.rs`, compiled into this binary by path so no
//! library build (and no shared cargo target) is needed.

#[allow(dead_code, unused_imports, unused_variables, unused_mut, clippy::all)]
#[path = "../../../../src/cryptanalysis/sat.rs"]
pub mod satlib;

mod curve;
mod enc;
mod gf;
mod run;
mod stats;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = run::cli(&args);
    std::process::exit(code);
}

#[cfg(test)]
mod tests_enc;
