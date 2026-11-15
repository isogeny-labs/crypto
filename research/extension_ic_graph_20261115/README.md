# Cubic-extension algebraic IC with symbolic graph elimination

This integration reuses the existing `gaudry_cubic` algebraic relation finders
and `pencil-symbolic-graph` forest. It changes the auxiliary-variable eliminator,
with a protected target logarithm and protected constant coordinate. [pcc:unverified]

[Implementation report (PDF)](report.pdf) · [Frozen protocol](protocol.json)

## Authorized local measurements

The user selected **local or Runpod** after the lab-access question. The
continuation now uses the local backend, a local checkpoint and published
source/spec; [Amendment 2](AMENDMENT_2_LOCAL.md) supersedes the earlier blocked
EKS route for this work. The first frozen pilot is `p=67`, seed `3`, all five
oracles, one warmup and five measured rounds under a 900-second bound. It
completed on source `3188372cc`: all 150 executions recovered correctly and
the local controls passed. The best graph arm, symmetric hybrid S3, took
25.61 times the existing local rho reference's cold wall time (paired-round
95% interval 24.16–27.62). See the [pilot report](local-p67-all5-20261115-02/report.pdf)
and [evidence](local-p67-all5-20261115-02/README.md). Standard lab admission
and S3 custody are not asserted.
[pcc:unverified]

The [native Rust analysis](native_analysis/README.md) checks the full planned
schedule, provenance/instance identity, graph/pivot fingerprints, scalar
replays and A/A controls. It retains failed/incomplete cases, produces paired
bootstrap intervals and a Plotters SVG, and prevents the small pilot from
triggering the large-subgroup reference-parity field. Six analysis tests and
its warnings-denied Clippy check pass locally. [pcc:unverified]

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
Its [merged-head release example test target](validation/symmetric-merged-example-tests.log)
compiled successfully and ran zero example-local tests. [pcc:unverified]

The Rust comparison driver now records one warmup plus five measured rounds,
includes same-instance `run_rho3` A/A arms, and labels raw execution rows and
derived block diagnostics separately under schema v2. Warmups and failures
remain in the output. Cross-oracle input prefixes are not asserted identical;
graph/pivot prefixes match within each oracle block. [pcc:unverified]

The reference is the repository's existing cubic r-adding rho. The fastest
external reference and full native pricing remain required for a broader
parity claim. The local pilot is exploratory and does not meet that claim.
[pcc:unverified]

[Amendment 3](AMENDMENT_3_SAMPLING.md) freezes the next matched fresh/r-adding
sampler comparison. The native incremental sampler charges construction,
cycle detection and uniform restarts; it recovers through IC relations.
The corrected implementation passes all 33 filtered cubic tests, including
the exact pilot fresh-stream regression. The failed unbounded-walk prototype
is retained in validation. [pcc:unverified]

The matched successors completed on source `515a68ce9`. Walk pair-only
graph/rho is 1.294 and 2.901 on seeds 3/11, against fresh 31.47 and 40.47;
all 240 executions recover and local controls pass. The different samplers
have different residual streams. See [fresh 03](local-p67-fresh-20261115-03/report.pdf)
and [walk 04](local-p67-radding-20261115-04/report.pdf). [pcc:unverified]

[Scale 05](local-pair-scale-20261115-05/report.pdf), frozen by
[Amendment 4](AMENDMENT_4_SCALE.md), checks p127/523/1039 and fresh seeds
17/23/37 on source `2c4292443`. All 270 executions recover. At p1039,
subgroup orders exceed 2^30 and graph/rho is 5.36–14.27. One p127/seed23
A/A failure remains an inconclusive diagnostic. No measurement was retried.
[pcc:unverified]

The [prepared-oracle and epoch controls](AMENDMENT_5_PREPARED_EPOCH.md)
completed on source `5f575deda`: 1080 executions recover and all paired
oracle streams match. [Short 06](local-prepared-short-20261115-06/report.pdf)
and [long 07](local-prepared-long-20261115-07/report.pdf) retain every timing
and control failure. Prepared arithmetic costs 0.67–0.84 times the reference
graph wall time in eligible comparisons; one comparison interval includes 1.
No graph/rho interval reaches 1. [pcc:unverified]

In 17 of 18 target/sampler cases, recovery coincides with the first repeated
decomposable abscissa. This is consistent with coalescing trails and a
distinguished-point-like use of smooth points. It does not establish improved
independent IC relation density. The exception recovers with 23 distinct
decomposable abscissae and no repeat. Native post-call summaries preserve
the original acceptance strata exactly. The next distinct lead is the
existing native F4 solver for three-point S4. [pcc:unverified]

Current checks report 36 passing cubic release tests and seven passing
native-analysis tests, plus release example build/test target and both
warnings-denied Clippy checks. [pcc:unverified]

Exact visual copies are under `benchmarks/20261115-cubic-graph-*`. The
admitted benchmark index and leaderboard remain unchanged: independent
replay/recompute, strongest-reference coverage and full pricing are missing.
Historical plans retain their original component hashes; use their pinned
source checkpoints for replay, rather than the current source by default.
[pcc:unverified]

## Scope and status

| Requirement | State | Evidence / remaining step |
| --- | --- | --- |
| Native Rust integration with other IC code | Final targeted tests pass locally | `src/cryptanalysis/gaudry_cubic_graph.rs`; `run_gaudry_with_eliminator` |
| Extension-field algebraic relation finding | Existing native engines reused | Weil-restricted S3 pair test, symmetrised S4, and hybrid scan/S3 |
| Named baseline/candidate comparison | First local pilot complete | Driver, pilot report, frozen protocol/amendments |
| Preserve historical default behavior | Targeted regression passes locally | Legacy entry point and optional report-key regression |
| Numerical performance result | Local pilot, best graph/rho 25.61 | Source/raw hashes and local custody; standard lab admission unavailable |
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
The [continuation's administrative-gate snapshot](validation/symmetric-administrative-gates.txt)
records the inherited global bench-record failure without changing historical
records or progress reports.

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
