# Native preparation of the frozen hashed-ECDL comparison panel

The native `hashed-ecdl-panel` binary prepares all 84 public cases from
[the frozen protocol](https://github.com/isogeny-labs/crypto/blob/784b001c021e754b35157ee85638a767287e2ede/research/hashed_ecdl_speed_20261029/PROTOCOL.md).
It performs no solve, timing, profiling, image publication, or remote submission.

| Curve `(p,a,b)` | Full order | Generator | Seeds | Public target scalars |
| --- | --- | --- | --- | --- |
| `(5,2,1)` | 7 | `(0,1)` | 1–10, 101–104 | 1, 5 |
| `(7,0,3)` | 13 | `(1,2)` | 1–10, 101–104 | 1, 11 |
| `(17,2,2)` | 19 | `(5,1)` | 1–10, 101–104 | 1, 17 |

Seeds 1–10 produce 60 development inputs; seeds 101–104 produce 24 holdouts.
The manifest retains three repetitions and at least five A/A and five
interleaved A/B rounds. It does not schedule or execute them. This prime-field
panel does not replace ECC2K-130 or official ECCp-131.

## Preparation and answer separation

Build with the owning repository's manifest:

```sh
# crypto
cargo build --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --bin hashed-ecdl-panel
# cryptanalysis (identical preparer source)
cargo build --locked --release --manifest-path bindings/rust/Cargo.toml -p hashed-ecdl-tensor --bin hashed-ecdl-panel
```

For research use, complete and upload the resume checkpoint and pass
`scripts/checkpoint.sh gate` on the authenticated runner first. Then run the
built `hashed-ecdl-panel prepare NEW_DIRECTORY`. The directory must not exist.
Exclusive creation preserves previous evidence. Failed writes leave partial
output; require a successful command exit before using it. Local unit tests
are development checks, not a research execution path.

`problems/*.json` contain public curve parameters, target coordinates, fixture
seeds, and policies. No expected scalar appears in a solver input or its
filename. The separate `manifest.json` maps inputs to expected scalars and full
cardinalities. Pass only an individual problem to the solver. Freeze the
generator commit, binary and generated file hashes in the future run evidence;
checkpoint and upload the inputs before execution. A local directory does not
establish S3 durability or benchmark admission.

Before writing, a separate bounded integer oracle enumerates every affine
point, checks nonsingularity and full cardinality, and exhausts the generator
orbit through the identity. It uses brute-force field inversion and repeated
addition, without the library's curve, primality, tensor or scalar multiplication
code. Tests cross-check every orbit point with the native curve implementation.
These are bounded fixture controls; the independent general audit remains open.

## Explicit policy addendum before execution

Each input fixes two slots, 16 seed trials, 4,096 columns, 2,000,000,000 charged
steps, vector length 2,000,000, and coefficient width 1,000,000 bits. These make
the existing fixture's step allowance and API defaults explicit. The threshold
remains the algorithm's default. Both arms must receive byte-identical inputs.
Exhaustion stays an unsuccessful row; do not change a budget or discard a case.
This addendum does not rewrite the original protocol or historical receipts.

The manifest records `execution: not_run` and null measurements. S3 checkpoint
transport, a digest-pinned native image and authenticated execution remain
prerequisites. The deployed ecbench image is a different workload. Profiling,
phase accounting, admitted matched A/A and A/B measurements, independent replay,
the full audit and both challenge solves remain open. No speedup is established.

## Sources and validation

Before implementation, reviewed the frozen protocol, existing
`hashed-ecdl-tensor/examples/fixture.json`, runner fields, curve/reduction code
and previous optimization receipts. Fixed unique JSON fields follow
[RFC 8259 §4](https://www.rfc-editor.org/rfc/rfc8259#section-4); exclusive creation
follows the [Rust 1.78 API](https://doc.rust-lang.org/1.78.0/std/fs/struct.OpenOptions.html#method.create_new).
No new algorithm or theorem is proposed. Development outputs and actual access
failures are in `hashed-ecdl-panel-validation.txt`; the generator has its own
`hashed-ecdl-panel.sha256`. Historical manifests and reports remain unchanged.

## Timed A/B driver (`hashed-ecdl-panel run`), added 2026-10-31

`run` executes two **prebuilt** `hashed-ecdl` arm binaries over a prepared
panel. It does not build arms, choose hosts or attest isolation. Source:
`src/panel_driver.rs` (included by `src/bin/hashed-ecdl-panel.rs`); tests:
`tests/panel_driver.rs` plus unit tests in the driver module.

```sh
hashed-ecdl-panel hash PANEL_DIR > SHA256SUMS        # sha256sum-compatible input list
hashed-ecdl-panel run --panel PANEL_DIR --inputs-sha256 SHA256SUMS \
  --baseline BASELINE/hashed-ecdl --candidate CANDIDATE/hashed-ecdl \
  --out NEW_DIR --seed U64 --case-limit-secs S \
  [--deadline-secs 1800] [--aa-rounds 5] [--ab-rounds 5] [--label TEXT]
hashed-ecdl-panel analyze NEW_DIR/rows.jsonl         # recompute the paired summary
```

The frozen panel inputs and their hash list are committed under
`research/hashed_ecdl_speed_20261029/inputs/` and mirrored to
`s3://setec-astronomy-artifacts/validation/hashed-ecdl-panel/20261031/`.

What it does, in order:

1. Refuses any manifest or problem byte not pinned by `--inputs-sha256`, a
   manifest that is not `hashed-ecdl-panel/v1`/`prepared`, duplicate ids, or a
   problem path other than `problems/<id>.json`. Refuses an existing `--out`.
2. Refuses a schedule below the frozen protocol (all 84 cases, at least five
   A/A and five A/B rounds, deadline at most 1,800 s) unless
   `--allow-incomplete-panel` is given; such output is labelled
   `diagnostic_incomplete_panel`.
3. Hashes both arm binaries before the run and again after it; a change makes
   the run exit non-zero (rows and analysis are still kept).
4. Runs a seeded interleaved Latin schedule, one child process at a time:
   `ARM solve PROBLEM NEW_RECEIPT`, timed from spawn to reaped exit, with
   stdout/stderr to files. If the solve exits 0 and a receipt exists, it then
   runs that same arm's `ARM verify RECEIPT` as a separately timed replay.
5. Validates every receipt itself, in this order, recording each check:
   receipt present; strict JSON (duplicate/unknown fields rejected); schema
   `hashed-ecdl-tensor/v1`; echoed problem equal to the input; full
   cardinality and generator orbit through the panel's exhaustive tiny oracle
   (no library curve code) and equal to the manifest's `full_cardinality`;
   scalar in `[0, r)`; `scalar*G == target` in the oracle orbit; scalar equal to
   the manifest's `expected_scalar`; `diagnostics.scalar_decimal` consistent;
   and finally the driver's own library `reduction::verify` replay.
6. Streams one strict JSON row per scheduled execution to `rows.jsonl`
   (`hashed-ecdl-panel.run-row/v1`, fixed field set, `fsync` per row), then
   writes `analysis.json` (`hashed-ecdl-panel.analysis/v1`).

Row status is one of `validated`, `rejected` (exit 0 but a check failed, or
the arm's own replay failed), `failed` (non-zero exit), `timeout`,
`timeout_deadline` (killed because the global deadline truncated the case
limit), `not_run_deadline` (scheduled but the deadline had elapsed) or
`spawn_error`. No scheduled execution is ever dropped.

### Choices where the protocol is silent (conservative)

| Question | Choice |
| --- | --- |
| Arm command | `solve` (default tensor backend) for both arms; never `solve-selectors`/`solve-width`. |
| "3 repetitions" vs ">=5 A/B rounds" | One round = every case once per pair member. A/B rounds must be >= max(5, 3) = 5, so every input gets at least 5 timed A/B repetitions; A/A rounds >= 5. |
| A/A arm | Baseline binary twice (member 0 and member 1). |
| Round order | A/A and A/B rounds strictly alternate; the seed picks which kind is first. |
| Case order | Seeded Fisher-Yates base permutation (SplitMix64), cyclically rotated by `round * floor(cases/rounds)` each round (cyclic Latin rows): each case takes a different position in every round. |
| Order inside a pair | Member run first = `(position + round + seed bit) mod 2`: balanced (42/42) inside every round and alternating per case across rounds. |
| Warmup | One pair (baseline then candidate) on the first case of the base permutation, before round 0; retained, excluded from analysis. |
| Per-case limit | Required flag (`--case-limit-secs`), no default; capped by remaining global deadline. |
| Global deadline | Default 1,800 s (original). After it elapses, remaining executions become `not_run_deadline` rows. |
| Retries | None. |
| Replay timing | The arm's own `verify` is the only phase the driver can time disjointly. Baseline `verify` rejects candidate receipts (`unknown field prime_certificates`), so each arm replays its own receipt. |
| Other phases | The arm binaries expose no phase timers. `setup`, `proof_validation`, `enumeration`, `encoding`, `relation_attempts`, `witness_search`, `matrix` are `null` in every row, not estimated; `solve_wall_ns` spans all of them. Phase accounting stays open until the arms emit it. |
| Effect and interval | Per (round, case) pair: `ln(wall member1 / wall member0)`. Per-case mean over rounds where **both** members validated; two-sided 95% Student-t interval over cases (case is the unit; rounds are not treated as independent). |
| Unsuccessful pairs | Excluded from the interval, counted by status per member in `status_counts`; candidate-only completions are not gains. |
| Certificate identity | Reported per kind (`certificate_pairs_identical` of `certificate_pairs_compared`), not enforced for A/B. |
| Meter counters | Copied as `meter_diagnostics`; never priced. `unpriced_common_operations` and `s_over_rho` are `null`. |
| Runtime claim | `runtime_claim_admissible` is always `null`: the driver cannot attest native L2 isolation or matched hardware. |

Wall-time numbers from any local run are L0 diagnostics, not results.
Caveat for every use: on 64-bit builds lambda <= 64, so
root_u = lambda/131072 = 0 and public inputs take the finite-size fallback;
nothing here exercises the recursive subexponential kernel, and
`ic.tensor_contraction_v1` is an exponential toy (protocol H1).

### Mirror into cryptanalysis

The driver must be mirrored byte-identically into cryptanalysis
`bindings/rust/hashed-ecdl-tensor/` (not pushed from this branch):
`src/panel_driver.rs`, `src/bin/hashed-ecdl-panel.rs`,
`tests/panel_driver.rs`, `tests/data/panel-receipt-p5-development-s001-t0.json`
and this document's driver section.
