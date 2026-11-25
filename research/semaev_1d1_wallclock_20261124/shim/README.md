# Shim harness (why it exists)
Building the full `crypto` lib in release needs >9 GB rustc RSS and was SIGKILLed twice (shared 16 GB container, sibling builds running). This crate compiles the **unmodified** `src/cryptanalysis/semaev_projection.rs` via `#[path]` and supplies only its imports:
- verbatim extracts: `Fp64`/`MulModN` (prime_fast.rs L57-518), field primitives `mulm/invm/sqrtm/...`, `Curve`, `Rng` (pkm_smooth_tower.rs), `sqrt_mod_p` (ec_index_calculus.rs), `jacobi_u64` (fp_backend.rs). Upstream `opcount` is `cfg(test)`, absent exactly as in a non-test build.
- NOT verbatim: `ecc::field::FieldElement` (minimal BigUint stub). It is used only by the multi-limb reference functions, which are compiled but never called by the benchmark.
Build: `CARGO_TARGET_DIR=/tmp/shimtarget cargo build --release --offline` in this dir; run `wallclock [reps=15] [targets=200]` (source: `src/main.rs`; to run against the real lib on a lab host, swap the `use` lines for `crypto_lib::cryptanalysis::...`).
Caveat: extracts were copied at main `ee44af4289`; re-extract if those files change.
