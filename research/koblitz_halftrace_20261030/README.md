# Native cached halftrace integration

This change integrates the missing one-word cached lifting path into the
existing Rust Koblitz factor-base builder. It does not import a new Python
solver, replace the repository's mature index-calculus implementation, or
claim that existing Frobenius folding and pair joins are new work.

**Track:** generic decomposition (bounded by sqrt(r); cannot beat rho).
**Measurement status:** pending, blocked by pre-experiment S3 checkpoint access.
No native speed ratio or end-to-end improvement is established.
The [Actions preparation](https://github.com/isogeny-labs/crypto/actions/runs/41082358257)
also failed because the approved upload secret is not configured. All build and
experiment jobs were skipped; there are no scientific measurements.

## Implementation

- `src/cryptanalysis/koblitz_fast.rs`: opt-in, curve-local, lazy odd-degree
  halftrace byte tables; initialized immutable tables are shared by clones.
  Scope guards restore the previous selection on nesting and unwinding;
  different threads do not change each other's recipe.
- `src/cryptanalysis/koblitz_index_calculus.rs`: reuse the cache in the existing
  fast coordinate lift while keeping its halftrace-selected first root and
  point order. The original squaring path remains the default.
- `src/cryptanalysis/ic_framework/plugins.rs`: select the candidate through
  `koblitz-orbit:divisor=1;2,halftrace_tables=1`. Invalid selector values fail.
- `examples/koblitz_halftrace.rs`: native known-answer integration fixture,
  comparing the ordered bases, orbit maps, relation trials and recovered scalars
  for two- and three-summand collection. The target scalar is not an input to
  the existing solver API. A second test exercises the public ecbench recipe,
  comparing the recovered scalar, factor-base hash and collection/LA counters.
  These fixtures report correctness, not time.

The cache uses at most eight 256-word tables on this path, not a table over
the field. It is built inside the charged factor-base construction interval.
Small or one-shot callers can keep the prior path. The generic and 131-bit
fields already had halftrace tables; they are not reimplemented here.

## Reproduction

```sh
cargo test --release --lib --locked
cargo test --release --example koblitz_halftrace --locked
cargo build --release --bin ecbench --locked
target/release/ecbench plan --spec research/koblitz_halftrace_20261030/spec.json
```

Use [the frozen protocol](PROTOCOL.md) and the repository's isolab-eks skill
for scientific execution. Do not run the measurement locally as a substitute
for its required L2 EKS Spot environment. Native plan validation and tests are
development checks; their elapsed times are not performance evidence.

## Evidence and remaining requirements

| Requirement | Status / record |
| --- | --- |
| Existing Rust baseline and source identity | built before source edits; [BASELINE.txt](BASELINE.txt) |
| Rust cache and factor-base integration | implemented; no Python execution path |
| Native correctness and regression gates | original suite: 5,447 passed, zero failed, 123 existing ignored; final-source filters/E2E/Clippy pass; later merged-main checks hit a retained base syntax error |
| Prospective native comparison protocol | [protocol.json](protocol.json) frozen before data; canonical schema/hash and corrupted-hash negative check pass |
| Source-resume checkpoint preparation | [failed Actions receipt](development/checkpoint-actions-receipt.json); no uploader secret; all experiment jobs skipped |
| Matched native end-to-end measurement | not run; [access blocker](../../problems/native-koblitz-measurement-access.md) |
| Source-pinned EKS image, L2, A/A, independent replay | pending; no scientific job was submitted |
| m=83 confidence gate / transfer toward ECC2K-130 | not established by this one-word change |
| External arithmetic comparison | not measured; no fastest-tool claim |

The coarse existing `as_solves` count alone does not price the changed work.
No S improvement is inferred from an unchanged solve count or a stage time.
The leaderboard, progress timeline and scoreboard are unchanged because no new
admitted measurement or curve finding exists. Earlier standalone Python timings
are preserved outside this repository and do not enter its benchmark ledger.

Initial sparse-checkout/build failures and the failed checkpoint-upload attempt
are retained in `development/`. No missing fixture was replaced with invented
data: tracked fixtures were materialized from the baseline revision.
The P1 review requires the saved matched comparison before landing; auto-merge is
paused. Current-main build validation and its merge conflicts remain separate
requirements. Scientific preparation pins tested source
`7c58661935b79153ff74c1208d0592aa56ed04a1` and the unchanged spec bytes. It does
not validate later main merges. The 280 planned executions have zero attempted
or completed scientific rows; no canonical data export is fabricated.
The 16:41 UTC upstream integration repaired the previously observed methods.rs
error, then a fresh check found another upstream parse error in
binary_semaev_chain_sat.rs. Its source-bound failure is retained separately;
older passing checks are not presented as a pass of that merged source.

[Implementation and execution-status diagram](pipeline.svg).
[Printable report](report.pdf), built from [report.typ](report.typ); an
[HTML reading copy](report.html) is also retained.
