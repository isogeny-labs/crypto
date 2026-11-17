# Native solver development checks

This continuation ports the execution path rather than replaying saved
answers. It uses only the exact historical fixture. No clock measurements,
performance comparison or admitted benchmark are requested by this check.

Before execution, the bounded check is:

- Construct the prime-field genus-three curve, factor base and degree-four
  map; check cleared polynomial identities and every rational factor-base
  image. Match the supplied generator to the first nonzero cofactor image.
- Check Cantor arithmetic, the pole-divisor correction, Frobenius trace and
  transfer homomorphism, including the transfer multiplier of four.
- Collect at most one million interpolation candidates with explicit seeds;
  solve original relation equations over the subgroup field. Check every
  resulting factor-base logarithm against the elliptic group.
- Start a new context for each of the first two archived public targets.
  The solver receives the public point and a relation seed; it does not
  receive the archived scalar. Compare the answer only after solving.
- Include mutations of the cover formula, pole correction, relation row and
  public target. An unchanged wrong fixture must fail; a returned scalar
  must always pass multiplication against the supplied public target.

The prediction is exact agreement, with any failed identity, relation,
normalization or scalar comparison falsifying the port at that step. The
bounded correctness checks are local development, not measurement execution.
The implementation must report failures rather than silently regenerating
fixtures or replacing failed targets. Keep separate diagnostics for failures
and never interpret a development timeout as a mathematical obstruction.

```sh
cargo test --release --locked --manifest-path research/genus3_cover_cold_target_20261115/native-check/Cargo.toml
cargo run --release --locked --manifest-path research/genus3_cover_cold_target_20261115/native-check/Cargo.toml -- --native-development-check
```

The public-target interface takes a JSON object containing only `target`
(the coefficient triples, or `null` for the identity) and `relation_seed`
(an unsigned integer). It rejects extra fields, including a known scalar:

```sh
cargo run --release --locked --manifest-path research/genus3_cover_cold_target_20261115/native-check/Cargo.toml -- --solve-public-target REQUEST.json
```

It constructs a fresh solver, checks every factor-base logarithm, performs
individual descent and compares the returned scalar against the public point. The internal
RNG differs from NumPy, so relation counts need not equal the frozen audit.
The setup cap is one million interpolation candidates, with twelve CG
diagonals per attempted solve; individual descent allows at most ten thousand
shifts. A reached cap is an explicit failure. This path is for development;
it is not a timing or admitted comparison harness.

The original source archive, raw audit, unsigned checker report and PDF remain
unchanged. Native operation accounting, a matched strongest rho reference,
identity registration, execution admission and independently signed replay
remain separate obligations. The repository-wide library run from the import
has an incomplete log and no recoverable process/exit status; its outcome is
unknown, not passing.

The current PR also has a divergent PCC-ledger conflict against main. Do not
edit or rechain either ledger while resolution is pending. New claim
publication and merge remain subject to that dependency.
