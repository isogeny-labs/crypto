# Adaptive S3 validation and retained attempts

Run `adaptive-s3-20261025` implements the committed
[protocol](../../ADAPTIVE_S3_20261025_PROTOCOL.md) and the
[pre-execution correction](../../ADAPTIVE_S3_20261025_V2.md). Protocol commit
`c845128fb99ea369b03b2cebd743159a90c7fbe4` and correction commit
`61e20f1d050482b613642519e57e030c576b5dcd` precede the first native
producer execution. The initial repository head was
`44510cf8e96dfe0ffd944895a3a9a89b428d8710`. The exact input hashes are
in `INPUTS.sha256`, the 99 canonical JSON files, receipt and certificate are
in `OUTPUTS.sha256`; every previous size-two source JSON is pinned separately
in `../../native/adaptive-s3-20261025/source-cases.sha256`.

## Commands and outcomes

| Check | Result | Record |
| --- | --- | --- |
| `bash research/quadratic_twist_evidence_20261021/run-mutant-s3.sh FRESH_DIR` | PASS; 98 cases,188 comparisons,1,384 targets; native producer/auditor under600s each; exact bytes | `producer.log`, `audit.log`, canonical `native/adaptive-s3-20261025/` |
| Independent symbolic row-space, promotion, witness, degree and actual-point replay | PASS; reduced joint24, original joint0, eleven source scalars recovered | canonical `audit.json` and cases/index |
| Rehashed corruptions: false witness, false degree, missing target, false source and false recovered scalar | all exit101, zero receipts | `controls-overlays.tar.xz`, `controls-full.sha256`, `restore-controls.sh` |
| Source-bound universal cleared identity | IDC1h9eea191ebc3a8def;32+16 points; false plus-one/source reject | canonical `certificate/`, `certificate-issue.log`, `certificate-check.log` |
| `tools/lean/proof-gate.sh --out FRESH_GATE lean lean-reduced lean-adaptive` (full paths relative to study) | PASS; new project19 audited, total226 audited/171 public research theorems; zero failing/extra trusted | `proof-gate-final/`, `proof-gate-final.log` |
| `cargo test --release --locked --lib --quiet` | PASS;4,479 passed,119 existing ignores | `cargo-lib-final.log.xz` |
| Native auditor crate `cargo test --release --locked` | PASS;22 backend tests | `backend-tests.log` |
| `python3 -m unittest scripts.test_workflows_parse` | PASS;4 parser tests, metadata only | `workflow-tests.log` |
| `backlog validate XP-20261025-reduced-s3-mutant-reinsertion` | PASS; card remains draft for main ancestry/Cairn gate | `backlog-validate.log` |
| `python3 scripts/check_curve_names.py` | PASS; zero naming problems | `naming-full.log` |
| Report `make check`, PDF text comparison and rendered two-page visual inspection | PASS; no LaTeX reference/overfull error | `report-build-final.log`, `../../reports/adaptive-s3-20261025/` |

The independent Rust auditor does not share field/group/polynomial arithmetic
with the C++ producer. It reconstructs row spaces and witnesses from the
preregistered inputs; equality of hashes alone is not treated as validation.
The positive stop requires all nonzero supplied inputs active. The F43 main
original-S3 matrix trace reports degree4 while its expanded original-input
witness reaches degree6. The reduced fixture's corresponding values are3/3;
the twist's are2/2. Both completed below the declared degree/round/expanded
caps. No wall-time or external fastest-solver comparison was admitted, and
no speedup follows from these operational degrees alone.

## Restoring failures and controls

The first producer draft produced all98 case files with the same bytes as
the final result, but its `index.json` retained only degree/status for the
eleven recovery traces. That provisional index is stored as
`attempt-01/index.json.xz` (uncompressed SHA-256
`035c811f0e492c2ecea41fd8e0828e1f4e48cb716791c7b1c931e3d51f6ebecd`).
`attempt-01/source.cpp.txt` and
`source-before-recovery-trace.cpp.txt` retain the two source snapshots;
`attempt-01/producer.log` and `attempt-02/producer.log` retain both runs.
The final source records full solver certificates for every attempted
recovery shift, including skipped identity shifts.

`lean-build-02.log` through `lean-build-06.log` record the actual failed
local formalization attempts, including an initially missing coefficient
normalization in the D=2 dual and the 17/17 root exclusion. `lean-build-07.log`
is the successful direct build. [Proof attempt notes](proof-attempts.md)
retain the decisive failed tactic forms and state the limit of the captured
source history; no failing `.lean` file is a Lake target. The unmodified full
scoped gate result is separate.

All five altered payload directories are reconstructed byte-for-byte from
the canonical cases and `controls-overlays.tar.xz` by:

```
bash research/quadratic_twist_evidence_20261021/validation/adaptive-s3-20261025/restore-controls.sh FRESH_DIRECTORY
```

The restore script checks every original control-file hash against
`controls-full.sha256`; its successful transcript is `restore-controls.log`.
This retains the precise changed JSON and failed audit log without committing
five duplicate 4.8 MiB case trees. The false-source and false-identity native
certificate payloads remain in the canonical `certificate/controls.json`.

Lean 4.34.1 used Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The full scoped gate
records only `propext`, `Classical.choice`, and `Quot.sound`, with no `sorry`,
custom axiom or `native_decide` trust. `lean4checker --fresh` was unavailable
in this environment, so no external checker is claimed. The shared Conductor
CLI remains unavailable (exit127 in the pre-edit check); no task reservation
or replacement local coordinator is claimed. Green local checks do not clear
the PR's exact-head CI, review, integration or broader mathematical gates.
