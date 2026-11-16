# Native twist-assisted factor-base diagnostics

This standalone Rust crate tests a concrete public-data selection heuristic and
exhausts defined toy search spaces. It does not implement a new scalar-preserving
transfer or establish a general twist-assisted speedup. All arithmetic, selection,
relation collection, linear algebra, verification, reports, and orchestration
inside the executable are native Rust. No Python execution path is used.

## TR-04 bounded relation-finding audit

The additive `twist-tr04` executable implements the separately frozen
[finite protocol](../../research/twist_relations_tr04_20261029/PROTOCOL.md).
It covers all coefficient models over p=7,11,19, B=2,3, seven selectors,
eight candidate reads and eight G/Q walk increments. Weighted enumeration
represents every declared candidate/query word exactly; rejected selectors
and unattained ranks retain their mass and paid work. It reports exact rank
distributions and unconditional expected work vectors in separate units.

The existing workflow's manual `tr04` suite runs all arms together in three
parallel field shards on `crypto-eks-linux`, with source/evidence checkpoint
gates and independent structural verification. Its native report preserves
failed/missing attempts and hashes compact portable evidence. Primitive
counter independence and physical runtime admission remain explicitly unset.

**Current status:** protocol frozen before primary data; root-library validation
completed on source `445d84bf7` with 5,389 passed, zero failed and Cargo exit 0.
Published in PR #2890. Native checks and the documented development
preflight pass. The actual workflow attempt failed its checkpoint upload gate
before any primary field execution; no retry was submitted. The retained
[failure report](../../research/twist_relations_tr04_20261029/reports/run-20261029T233325Z/README.md)
records every missing output. Source, checkpoint and transport gates remain.
No TR-04 scientific run, finite-census result or runtime gain
is recorded. See the study README for remaining prerequisites. Existing
TR01-03 evidence and commands below retain their own scope.

Checkpoint and artifact transfer use maintained infrastructure tools; the
cryptographic computation, verification and report generation remain native.

## Build and check

From the repository root (Rust 1.99.1 is the pinned CI toolchain):

```sh
cargo test --release --locked --manifest-path tools/twist-relations/Cargo.toml
cargo clippy --release --locked --manifest-path tools/twist-relations/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path tools/twist-relations/Cargo.toml --check
```

The crate has its own workspace and lockfile, like `tools/ecdlp-surfaces`; it
does not pull the full library into every runner job. The exact small-field
correctness regression checks the complete finite S3 zero set against separate
signed group additions on the original curve and its quadratic twist. Integration
tests check artifact hashes, replay, rejection of corrupted scalars, execution
from an unrelated directory, and protection of previous output directories.

## Run on repository runners

The [workflow](../../.github/workflows/twist-relations.yml) runs native checks on
relevant pushes to main, preserving the repository's restricted PR merge set.
After merge, **Actions → Native twist and factor-base experiments → Run workflow**
selects suite, seed count, seed start, and runner. Manual diagnostics default to
the existing `crypto-eks-linux` runner label; `ubuntu-latest` is an explicit
alternative. This PR does not register runners or change repository settings.

Manual diagnostics retain the repository's checkpoint gate. The workflow writes
a resume note and runs `scripts/checkpoint.sh gate`; the runner needs its existing
checkpoint transport, or the `CHECKPOINT_UPLOAD_TOKEN` repository secret. An
unavailable transport fails before diagnostic execution; it is never bypassed.
The pre-existing checkpoint script is infrastructure, not a Python research
implementation. No manual cloud run is claimed by adding this workflow.

External runners use the same native binary after completing the repository's
checkpoint and run-routing prerequisites:

```sh
cargo build --release --locked --manifest-path tools/twist-relations/Cargo.toml
scripts/checkpoint.sh gate
tools/twist-relations/target/release/twist-relations \
  --suite all --seeds 30 --output-dir artifacts/twist-full
tools/twist-relations/target/release/twist-relations verify artifacts/twist-full/benchmark.json
tools/twist-relations/target/release/twist-relations verify artifacts/twist-full/exhaustive.json
```

| Suite | Charged solver diagnostics | Exhaustive audit |
|---|---|---|
| `smoke` | p=43,67; 2 seeds unless overridden | p=7,11 |
| `benchmark` | p=1019,4091,16319; 30 seeds unless overridden | none |
| `exhaustive` | none | p=7,11,19,31,43 |
| `all` | full solver panel | full audit |

`--seed-start 100 --seeds 30` selects seeds [100,130). Use disjoint seed ranges
for independent benchmark shards. `--primes P ...` selects bounded benchmark
fields; `--exhaustive-primes P ...` shards the audit by field. Fields must be
distinct primes congruent to 3 modulo 4, at least seven. Benchmark p is capped
at 1,000,003; exhaustive p is capped at 127. These are explicit implementation
bounds, not promises that every larger audit fits the runner deadline.

Output directories must be new or empty. Each run writes an atomic manifest
with parameters, UTC Unix timestamps, OS/architecture, binary SHA-256, compiled
source/lockfile hashes, and artifact hashes. A workflow-provided build revision
is distinct from the working-tree revision seen during execution. SplitMix64
with unbiased rejection sampling generates public fixtures; its streams are
**different from the historical Python prototype's streams**. Python timings
are historical diagnostics and are not comparable native measurements.

Failures and invariant/budget refusals exit nonzero and leave an incomplete or
failed manifest and any completed artifacts. This executable has no mid-search
resume support; use bounded shards and retain interrupted runs. The backlog
card therefore remains draft until its source pin, recovery contract, and
independent verification/admission requirements are satisfied.

### Incremental benchmark evidence

Each successful solve is synced to `cases/00000000.json`, then the next ordinal,
before the driver starts another case. Every case file uses the benchmark schema
and can be checked directly with `twist-relations verify FILE.json`. The final
`benchmark.json` remains the complete-panel export; its absence does not erase
the completed case files.

`benchmark-cases.jsonl` is a synced event journal. `active-case.json` is an atomic
snapshot of the latest discovery/start/progress/completion/failure event. It
records the exact parameters, seed, method, phase, search attempts, rank and
counters. Search snapshots are published at most 256 trials apart and at phase
boundaries. A caught budget error or unwinding invariant failure records actual
in-memory counters in `benchmark-failure.json` and a failed event. Evidence sink
errors fail the run; they cannot turn into a pass.

A hard timeout, SIGKILL or abort cannot write a final failure event or final
manifest. Such a run stays incomplete (`status=running` with no finish time),
and its active snapshot/journal is a last-known prefix, not a final clock or
final count. A kill can truncate the final journal line; preceding complete
newline records and atomically published case files remain readable. This is
evidence retention, not process-memory checkpoint/restore or automatic retry.

`--max-attempts-per-solve N` may tighten the existing `1000*n` cap, and is useful
for bounded failure checks; it never increases the cap. In-solve publication
time is part of `total_ns` and exposed as `evidence_io_ns`. Final-record and
aggregate exports occur after that interval. These instrumented elapsed values
remain diagnostics and are not admitted performance comparisons.

## Implemented experiment contracts

**TR-01: charged relation solver.** For each benchmark field choose the first
nonsingular `y²=x³-3x+b` with prime order and the smallest-x generator. All arms
use identical public G,Q, scalar fixtures, base size `round(n^(1/3))`, and random
walk streams per seed. Controls are random original-curve x coordinates and
an x-coordinate arithmetic progression. The candidate scans the field and ranks
original coordinates by the immediately following twist-coordinate run, capped
at eight. Its scan, square roots and sorting are paid. A direct BSGS arm includes
table setup and verification. This is not the repository's strongest rho or
optimized IC baseline, and no comparison against those implementations is claimed.

Each base's pair table stores `Pi+Pj` for i≤j. A tracked walk `R=aG+bQ` adds G or
Q once per trial. A hit gives `log(Pi)+log(Pj)-b*k=a mod n`. Incremental elimination
recovers base logarithms jointly with k, and every recovered scalar is replayed
in the group. Full joint rank is sufficient; a future target-identifiability
criterion could stop earlier. Attempts are capped at `1000*n` per solve.

Selection, pair-table setup, relation search, joint linear algebra, and
verification have separate exclusive counters/timers. Every phase's counters
must sum to the total. Group-add calls, inversions, characters, roots, lookups,
and modular elimination cells remain separate units. Public curve discovery and
creation of input Q are excluded equally; discovery time is recorded explicitly.
Every target pays fresh tables. Table cardinalities are recorded; byte-level
peak memory and complete field-operation/GAE prices are not measured.

**TR-02: exhaustive base audit.** Inspect all (a,b), select nonsingular models
with prime original order ≥7, and exhaust every three-x base with canonical and
both point signs. Average every tie attaining the maximum sum of twist-run
scores exactly. Also exhaust arbitrary three-element nonzero-point subsets
once per cyclic group order. Oracle log labels accelerate the audit only;
they never enter TR-01. Different coefficient models can be isomorphic.

**TR-03: exhaustive S3 audit.** Evaluate every unordered finite coordinate triple
with repetition. Independently construct the complete expected zero set using
signed additions on the original curve and the twist model
`Y²=X³+aX-b`, with embedded abscissa `x=-X`. Verify both point counts sum to
`2p+2`. Default finite-scope regression totals are 457 models, 439,435 three-x
bases, and 4,215,282 triples. These are correctness fixtures, not new performance
claims. The all-model audit deliberately includes special/anomalous models;
it is not an independent generic-curve performance sample.

## Evidence limits and next leads

`verify` is fresh replay by this executable, not an independent implementation,
external audit, or `VALIDATION.json` certificate. Timings are instrumented
diagnostics: no L2/A-A admission, matched strongest rho, external-tool parity,
or 0/0.1/1 GAE pricing is asserted. No leaderboard, dashboard or benchmark result
is promoted by this implementation PR.

The [runner card and protocol](../../backlog/XP-20261029-twist-factor-base/README.md)
freeze scope, accounting, follow-up designs and acceptance criteria. Cheap
threshold sampling, higher-arity elimination, target-identifiability stopping,
and amortized studies remain unimplemented leads; they are not reported as
executed algorithms.

## Sources checked for this port

* [SEC 1 v2, §2.2.1](https://www.secg.org/sec1-v2.pdf): short-Weierstrass group
  law, identity/inverse cases, and double-and-add arithmetic.
* [Diem, *On the discrete logarithm problem in elliptic curves*, §3](https://doi.org/10.1112/S0010437X10005075): summation polynomials and their
  signed-addition zero-set contract. Its extension-field results are not claims
  about this prime-field toy experiment.
* [Semaev, ePrint 2004/031](https://eprint.iacr.org/2004/031): the primary PDF
  fetch returned HTTP 403 in this session. The accessible Diem primary paper
  and independent exhaustive group-law comparison cover the implemented contract;
  no novelty claim is made.
* [Native prime-field work](../../RESEARCH_PRIME_FAST_ECDLP.md) and
  [endomorphism rules](../../docs/endomorphism-rules.md): pair-table methods
  already exist here; arithmetic, subgroup transfer, and solving advantage
  are separate obligations.
* [Existing quartic-twist image-base study](../../backlog/XP-20261023-quartic-twist-image-base/README.md): preserves its separate frozen instances,
  image factors and rank/cost evidence. This quadratic-character selection
  diagnostic neither repeats nor supersedes that study.
* Rust's [`File` synchronization](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all)
  and [`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
  contracts: explicit synchronization detects write failures; unwinding handlers
  cannot claim to capture SIGKILL, aborts or physical-host recovery.
