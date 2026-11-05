# Track 1: exhaustive two-summand toy audit

Implementation status: native harness implemented; scientific panel **not run**.
The parent session must freeze the complete protocol, checkpoint it and obtain
the required execution admission before invoking `run`. A completed unit-test
suite is development verification and does not discharge this experiment.

## Frozen question and inputs

For `icv1-fp5-tm1-a77cc6af`, compare the complete root sets of the split-graph
and matched nongraph systems with independently enumerated signed group
relations. The fixed component spec is [protocol.json](protocol.json). Its
values are deliberately rejected if changed; further inputs require a versioned
protocol. Both arms have four abscissae, eight signed points and four columns:

- `FB1hed5b19904cc8`: V={5,6,10,16}, h=4T²+9T+9.
- `FB1h66fbb7ed3fb6`: W={0,5,6,10}, g=8T³+6T²+7T+6, with exact membership.

The full finite target population is all 18 affine points, including both
signs. For each arm/target, the harness checks all four sign branches and all
17³ triples (x,z,m), plus all 17² dense membership/S3 assignments. Planned
counts are 36 cells, 707,472 structured assignments and 10,404 dense assignments.
No branch is discarded. The identity target is handled separately using exact
inverse pairs; it is not silently counted as a finite-target secant test.

Reference group arithmetic is independently written affine chord/tangent
arithmetic, rather than deciding correctness by re-evaluating the model's
secant equations. Every reported model relation and every dense root witness
is replayed in that group code. References include doubles. Compare exact model
roots including signs and slope, point-pair projections, dense roots, and
fresh group replay. Every discrepancy is retained and fails acceptance.

Additional independent count checks for each arm: 56 finite ordered/model
relations, 8 tangent relations, 32 finite unordered pairs, 8 ordered inverse
pairs at O, even SAT-target count, and equal root counts at R and −R. These
follow analytically from a signed eight-point set in the odd-order group;
they are preregistered checks, not reported observations.

## Build and development verification

```sh
export CARGO_HOME=/workspace/n83-cargo
export RUSTUP_HOME=/workspace/n83-rustup
/workspace/n83-cargo/bin/cargo test --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/parallel_20261031/audit/Cargo.toml
/workspace/n83-cargo/bin/cargo clippy --locked --offline --manifest-path research/prime_factor_base_degree_20261031/parallel_20261031/audit/Cargo.toml --all-targets -- -D warnings
/workspace/n83-cargo/bin/cargo build --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/parallel_20261031/audit/Cargo.toml
```

Six development tests cover supplied secant/tangent/inverse values, known
dense double membership, strict fixture rejection, full branch/control
membership export, and exact structural equality with the previously committed
exporter at supplied target (10,6), and the F4 bundle's variable/sign format.
They do not call `audit` or enumerate the
complete target panel.

## Admitted execution contract

Use one logical CPU, at most 2 GiB address space, and a 120-second outer wall
limit. The internal deadline also checks at each target boundary. The backend
must enforce CPU/address-space limits; the manifest distinguishes requested
resources and observed available parallelism. The worker must preserve caller
stdout/stderr, admission receipt, source revision, binary hash and exit status.

After admission, from the repository root:

```sh
AUDIT=research/prime_factor_base_degree_20261031/parallel_20261031/audit
"$AUDIT/target/release/secant-complete-audit" export-inputs "$AUDIT/protocol.json" FRESH-INPUT-DIRECTORY FULL-40-HEX-SOURCE-COMMIT
"$AUDIT/target/release/secant-complete-audit" export-f4 "$AUDIT/protocol.json" FRESH-F4-DIRECTORY FULL-40-HEX-SOURCE-COMMIT
"$AUDIT/target/release/secant-complete-audit" run "$AUDIT/protocol.json" FRESH-RESULT-DIRECTORY FULL-40-HEX-SOURCE-COMMIT
```

`export-inputs` writes all target/arm equations and hashes without enumerating
roots. `export-f4` writes the agreed flat `systems` bundle: 180 systems,
candidate/control × 18 targets × (four secant branches plus one dense system).
Dense terms have two exponents; secant terms have three. Expected roots are
omitted during preparation. `run` creates a fresh directory, records planned scope immediately, then
writes each cell as it completes. Existing output paths are refused. A missing
final manifest means incomplete; partial cells remain evidence. A complete run
exits nonzero on any mismatch or aggregation-invariant failure, retaining the
full failure record. Do not rerun over that directory or turn an incomplete
run into a pass by dropping cells.

The source revision is caller provenance. The manifest separately binds all
native source bytes, the lockfile, protocol and actual executable by SHA-256.
It does not pretend that a caller-provided commit changes the executable.

## Accounting boundaries

Each cell records every root and witness; assignment counts, exact survival
fractions, ordered/unordered/matrix-row duplicate counts, and tangent counts.
Matrix rows use the existing lower-y sign convention modulo 19. Independence
or rank of those rows is not inferred.

Counted field/group phases are: point inventory/base-lift checks, per-arm
reference enumeration, all structured equation evaluations plus h evaluations
used to materialize roots, all dense equation evaluations, and fresh group
replay. Every equation is evaluated, without short-circuit-dependent counting.
Inversions are reported both as calls and through their exponentiation's field
multiplications: do not charge both again when pricing. **Input-polynomial
construction, integer loop/control work, hashing, serialization and IO are not
included in those field counters.** Therefore they are not complete pipeline
operation totals.

The manifest records an observed Linux process high-water RSS snapshot when
available. It is not per-phase materialized-list storage. Payload bytes written
exclude the final manifest; cell readback and executable hashing read bytes are
separately named. Spec bytes and operating-system disk traffic are not charged
by this library. Whole-run elapsed wall time is unqualified operational output,
with `comparative_performance_claim=false`; no speedup is concluded here.

Raw outputs are deterministic except runtime/resource metadata. The parent
archives the run tree under the repository raw-run policy and verifies hashes
before publishing. No result has been produced by this implementation yet.

## Sources and limits

Read [the exact theorem](../../SECANT_GRAPH.md), existing complete exporter,
and [MIT 18.783 lecture 2](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
§2.2 including tangent/vertical exceptions. The separate reference group code
uses those standard formulas. This task checks a supplied toy construction;
it makes no novelty, P-256, larger-arity, F4 solving-degree or performance claim.

Conductor was attempted but its binary is absent; the repository documents
that installation state as fail-open. Work is confined to this track directory.
No S3/backend rule is weakened by this implementation.
