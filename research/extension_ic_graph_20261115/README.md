# Cubic-extension algebraic IC with symbolic graph elimination

This integration reuses the existing `gaudry_cubic` algebraic relation finders
and `pencil-symbolic-graph` forest. It changes the auxiliary-variable eliminator,
with a protected target logarithm and protected constant coordinate. [pcc:unverified]

[Implementation report (PDF)](report.pdf) · [Frozen protocol](protocol.json)

## Rust continuation toward the rho comparison

[Amendment 1](AMENDMENT_1.md) adds a native symmetric-coordinate S3 pair
oracle and its hybrid triple variant, both integrated with the existing
collector and graph/pivot backends. The pair eliminant has degree at most
three; GCD compatibility is applied before root extraction. Dependent systems
retain a counted scan, and both-denominator-zero sums retain the constant
quadratic instead of being discarded. The new filtered release run reports
30 passing tests, including six new oracle checks and both new solvers in
the protected graph/pivot recovery test. These are producer-run development
checks, not admitted performance observations. [pcc:unverified]
The continuation's release example type check also passed locally. [pcc:unverified]

The Rust comparison driver now records one warmup plus five measured rounds,
includes same-instance `run_rho3` A/A arms, and labels raw execution rows and
derived block diagnostics separately under schema v2. Warmups and failures
remain in the output. Cross-oracle input prefixes are not asserted identical;
graph/pivot prefixes match within each oracle block. [pcc:unverified]

The reference is the repository's existing cubic r-adding rho. The fastest
external reference and full native pricing remain required for a broader
parity claim. Ratios in the driver are unadmitted diagnostics; no performance
run or parity result has been produced. [pcc:unverified]

## Scope and status

| Requirement | State | Evidence / remaining step |
| --- | --- | --- |
| Native Rust integration with other IC code | Final targeted tests pass locally | `src/cryptanalysis/gaudry_cubic_graph.rs`; `run_gaudry_with_eliminator` |
| Extension-field algebraic relation finding | Existing native engines reused | Weil-restricted S3 pair test, symmetrised S4, and hybrid scan/S3 |
| Named baseline/candidate comparison | Driver implemented, run pending | `examples/gaudry_cubic_graph_compare.rs`, `protocol.json` |
| Preserve historical default behavior | Targeted regression passes locally | Legacy entry point and optional report-key regression |
| Numerical performance result | Not measured | S3 checkpoint/EKS submission access and admission required |
| Complete native-cost ranking | Not implemented | Legacy pivot/dense counters omit additions, inversions and native collection work |
| Full parent-library validation | Incomplete, interrupted | Reported failures outside the touched module and long inherited PKM cases; see validation record |

## Two separate changes

The existing collector places the target in column `small`, but its historical
pivot eliminator uses `small` as the first auxiliary column. That permits it to
eliminate the target in partial relations. This integration preserves that entry
point for historical replay and adds an explicit comparison entry point that
protects `small + 1` columns for **both** backends. [pcc:unverified]

The comparison chain is therefore:

1. Historical pivot boundary → protected-target pivot boundary: a separately
   identified correctness/interface correction, distinct from the graph replacement.
2. Protected-target pivot → protected-target symbolic forest: the measured
   component replacement, on the same oracle and relation stream.

For a row `sum(c_i*x_i) = rhs`, the graph receives the homogeneous equation
`sum(c_i*x_i) - rhs*ONE = 0`. Its small variables include the ordinary base,
target logarithm, and `ONE`. No discrete logarithms of the base points are given
to the graph. Arbitrary coefficients, sign changes, loops and anchoring use the
existing forest implementation. [pcc:unverified]

![Native integration and measurement boundaries](integration.svg)

## Sources checked

- **KB unavailable:** no research-kb MCP server is loaded; the repository wrapper
  returned `research-kb: no executable found`. No absence or novelty inference.
- `src/cryptanalysis/gaudry_cubic.rs`: existing subspace factor base, S3/S4
  solvers, cofactor projection, relation verification and pivot-chain baseline.
- `research/pencil_large_prime_20261029/README.md`, `src/lib.rs`, `src/meter.rs`:
  existing general-coefficient symbolic graph and constructor/event accounting.
- `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`, sections 11.7/11.11:
  prior merge-depth and fill tradeoffs, kept distinct from this replacement.
- Gaudry–Theriault–Thome (2005), sections 2.3/3.1,
  [primary paper](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf):
  full relations differ from arbitrary graph cycles; its higher-genus complexity
  theorem is not a complexity statement about this integration.

## Development checks

```sh
cargo test --release --lib --no-default-features gaudry_cubic::
cargo test --release --example gaudry_cubic_graph_compare --no-default-features
```

Tests cover symbolic projection against a full relation system after each prefix,
duplicate auxiliary coefficients, nonzero right-hand sides, invalid columns, and
actual cubic instances under all three native oracles. These are development
correctness checks, not admitted benchmark measurements. [pcc:unverified]

The earlier graph-integration filtered release-library run reports **24 passing tests**, including
six adapter/integration tests and the input-fingerprint comparisons. The touched
example compiled in its Cargo test target before that control (it defines no
example-local tests); that integration head's example also passed its release type check. These are producer-run
development checks, with no independent signed verdict. [pcc:unverified]

Raw development output: [final targeted library](validation/final-targeted-library-tests.log),
[earlier targeted library](validation/targeted-library-tests.log),
[example compilation](validation/example-build-tests.log), and
[checkpoint gate](validation/checkpoint-gate.log).
The [current-head example check](validation/current-head-example-check.log) and
[component source hashes](validation/component-source.sha256) distinguish the
final code checks from the earlier pre-fingerprint development runs.

The symmetric continuation's [30-test release output](validation/symmetric-targeted-library-tests.log)
and [source hashes](validation/symmetric-component-source.sha256) identify the
new source. Its [initial sparse-fixture compile failure](validation/symmetric-sparse-fixture-compile-failure.log)
is retained separately from the passing retry. [pcc:unverified]

The pre-fingerprint full default-feature library run was interrupted after
reported failures outside the touched module and more than twelve minutes in
test execution. It is **not a passing whole-library result**. A focused case
reproduced a missing archived session; a device case passed after its tracked
emulator assets were restored. Other failing causes are not all classified.
The [incomplete-run record](validation/full-library-interrupted.txt) pins the
raw local output; durable upload of that larger raw file is still pending.
[pcc:unverified]

The checkpoint gate reports no configured AWS profile or upload token. This
shares the existing [launch-gate access obstruction](../../problems/arity-completion-launch-gates.md#2026-11-15-cubic-extension-symbolic-graph-comparison).
No benchmark launcher was invoked. [pcc:unverified]

Protected runs additionally fingerprint the entire factor base and the admitted
projected relation stream with BLAKE3. Both backends hash the same byte encoding;
the report carries the digest and input-byte count. This hashing remains in the
unpriced native-work scope; it is not a free relation oracle. [pcc:unverified]

## Frozen measurement commands

Only launch after publishing the exact source, checkpointing, and admitting the
prescribed EKS run. The driver refuses to overwrite raw data, retains failed
attempts, and reverses arm order between rounds.

```sh
cargo build --release --no-default-features --example gaudry_cubic_graph_compare
target/release/examples/gaudry_cubic_graph_compare \
  --primes 7,13,19,31 --seeds 3,7,11,23 --warmup-rounds 1 --rounds 5 \
  --max-residuals 10000 --source-revision SOURCE_SHA --out cohort.jsonl
target/release/examples/gaudry_cubic_graph_compare \
  --primes 43 --seeds 101,103 --warmup-rounds 1 --rounds 5 \
  --max-residuals 10000 --source-revision SOURCE_SHA --out holdout.jsonl
```

`legacy_model_cold_gae` is the existing multiplication-equivalent model plus
fresh base setup. It is explicitly a diagnostic, **not** a complete native-cost
ranking: missing scalar/collection operations are listed in every row. Graph
constructor primality and inversion multiplications are retained. Legacy wall
readings remain unadmitted until the EKS/L2/A/A conditions are met.

The external linear-algebra comparison is not run. The frozen WDSat regression
suite concerns a different encoding; this protocol freezes equivalent matched
cubic-extension inputs instead, without substituting a WDSat-stage gain for a
full IC measurement.

No scoreboard or leaderboard figure changes: this packet has no admitted
performance observations. Historical Python artifacts remain outside this
repository; this execution path is native Rust throughout.
