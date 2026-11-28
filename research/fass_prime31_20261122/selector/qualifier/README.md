# Source-bound selector development qualifier

This small crate compiles the delivered production selector and the repository's
existing polynomial, quadratic, identity-certificate and SHA-256 modules.
The field, curve, RNG and polynomial-root code is the exact unchanged prefix
of `pkm_smooth_tower.rs`, ending after `poly_roots`. Its later unrelated
Hilbert-polynomial and F4 integrations are omitted. The build inventory records
each full source hash, the selected prefix bytes and its hash; no algorithm
body, arithmetic operation or brace is replaced.

`cargo test --manifest-path research/fass_prime31_20261122/selector/qualifier/Cargo.toml --release --test semaev_selector`
is the development command. The same test file is registered in the root crate.
This focused build does not close the whole-library test gate.

For preserved evidence, build with `--no-run --message-format=json`, copy the
reported test executable and `selector-record` binary into a read-only directory,
write a new `manifest` record before execution, run the copied tests, then write
a differently named post-call manifest. The manifest writer refuses existing
files and source drift since build. Raw test-harness elapsed times are incidental
output, not an admitted timing experiment. The `certificates` command records
bounded evaluated identity controls using the existing identity checker; these
are not proofs over all rings or signed PCC verification.
