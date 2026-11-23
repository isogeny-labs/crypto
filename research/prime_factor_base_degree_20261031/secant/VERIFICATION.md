# Deductive verification, 2026-10-31

This continuation checks a mathematical construction and supplied proof
certificates. It runs no factor-base search, complete target-population panel,
CAS experiment, timing comparison or remote job.

Baseline local source: 6024817e. The reused repository identity-certificate
module is unchanged and byte-identical to the published PR base:

`src/cryptanalysis/identity_certificate.rs`
SHA-256 `d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270`.

Set `CARGO_HOME=/workspace/n83-cargo`, `RUSTUP_HOME=/workspace/n83-rustup`;
Cargo/Rust 1.94.1. Exact command forms:

```sh
cargo fmt --check --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
cargo test --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
cargo clippy --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml -- -D warnings
cargo build --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
/workspace/prime-degree-lean/bin/lean research/prime_factor_base_degree_20261031/proof/SecantGraphTop.lean
```

Observed: 8 existing prime tests plus 18 secant-binary tests pass; zero failures.
The latter include 9 reused identity tests, 7 secant tests and 2 homogeneous
certificate tests. Committed identity, degree and metadata artifacts are loaded
by the tests. Known-answer secant/tangent roots are checked; the existing
independent affine group arithmetic replays the fixed G,2G,3G,7G,13G multiples.
This is not an exhaustive root-versus-group check on all targets.

Strict Clippy, formatting and release build pass. Four new Lean declarations
check using only propext, Classical.choice and Quot.sound. No sorry, admit,
custom claim axiom or native evaluation occurs. The polynomial-ring bridge,
the d=2 symbolic argument and the curve-group equivalence have written proofs;
they are not described as fully formalized in Lean.

The exact group/secant identity expands identically over Z. Issued and replayed
`IDC1h621ed1a51b5f3ca8` additionally uses the repository v1 certificate format.
For the F17 quadratic top ideal, the degree checker independently multiplies
the supplied upper recipes to all ten cubic monomials and checks a separating
quadratic dual, in all four sign branches. It rejects corrupted identities,
missing branches and invalid lower witnesses. No claimed matrix rank is trusted.

Raw development output is [development-checks.log](development-checks.log).
Elapsed compilation/test times in it are operational output, not benchmarks.
The retained [checkpoint-gate.log](checkpoint-gate.log) records exit 1 and the
unavailable S3 upload route. No experiment was launched after that failure.
The earlier root-library attempt remains incomplete as recorded in
[PUBLICATION.md](../PUBLICATION.md); this continuation changes no root source.

The two-page Typst report was compiled and both final pages were rendered and
visually inspected. The figure labels every degree as derived algebraic data.
The dump files preserve the existing ICV1 model preimage, FB1 identity preimage,
hex point format and sign/column convention; central registry/EC1 linkage is
marked pending. Source/artifact bindings are in SHA256SUMS.

## Preserved development failures

- First secant build failed because the reused identity module also required
  `hex`, and one inferred monomial-key type needed an annotation. Added the
  dependency and explicit type, then rebuilt and reran the suites.
- Offline dependency resolution initially selected cached libc 0.2.190.
  Restored the previous locked 0.2.191 and its recorded checksum; final locked
  offline tests and Clippy pass. Only the required hex dependency is added.
- Two documentation patch attempts failed their exact-context check; corrected
  the patch context before applying. No source/evidence was overwritten.
- Sparse hydration initially lacked network permission; repeated the normal
  Git operation with network permission and materialized the requested paths.
- Primary full-text 403s and timeouts are listed in SECANT_GRAPH.md; absence
  of a found exact prior statement is not promoted to a novelty proof.

External novelty, independent review, actual F4 degree, complete all-target
correctness/yield and useful scaling remain unestablished. No P-256 relation,
17-term reduction or end-to-end improvement is claimed.
