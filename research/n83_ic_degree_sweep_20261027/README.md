# N83 balanced index-calculus degree sweep

Requested grid: Koblitz, general binary and prime curves, n=83, decomposition arity m=2..9, balanced candidate abscissa domain 2^floor(83/m). This study preserves all 24 cells and attempts a planted and a natural target with two distinct computations each (96 attempts).

**Observed:** all 96 attempts ran; 53 exited at the process time limit and 43 with Singular's no-more-memory code. No top-form degree or affine final-basis degree completed. Forty attempts stopped before the solver-start marker; 56 entered standard-basis computation. All 533 raw-manifest entries passed SHA-256 verification. One time-limit exit also printed a memory diagnostic; classifications follow the exit status and retain that diagnostic separately.

Read [the run report](reports/run-20261027T1540Z/report.md) and [PDF](reports/run-20261027T1540Z/report.pdf) for recorded outcomes. [ALGEBRA.md](ALGEBRA.md) gives a separate derivation: the prime presentation has positive-dimensional top ideals at m>=3, so it has no finite top-form regularity under the frozen definition. This is not an observed affine solving degree or an impossibility result for index calculus.

## Evidence

- [Frozen design](PROTOCOL.md), [original protocol](protocol.json), and [premeasurement correction](protocol-v2.json).
- [V1 startup failure](startup-failure-v1.json), [corrected request](request-v2.json), [submission receipt](submission-v2.json), and [terminal receipt](terminal-v2.json).
- [Unified export](experiment.export.json), [compact observations](evidence/observations.json), [raw rows](evidence/results.json), and [storage verification](evidence/storage.json).
- Native [generator and solver driver](src/main.rs); derived [report/export formatter](src/bin/report.rs). The mathematical source and its frozen parser were unchanged during the run. The formatter adds log-based operational diagnostics without changing the raw rows.

Each solver gets 40 seconds plus five seconds of kill grace and a 3 GiB virtual-memory allowance. Whole-process limits include input expansion and witness checking. A timeout or incomplete standard basis supplies no final degree and no degree lower bound. Singular exit 14 with `no more memory` is retained as the original `incomplete` status and explained as a memory limit in the report. The schema's export status `failed` counts a terminated failed attempt; it does not mean the mathematical computation completed.

No full IC attack or timing comparison was measured. Candidate x-coordinate counts are not enumerated factor-base point counts. Synthetic targets lie on the full curves; no subgroup membership claim is made. Exact prime, curve coefficients, targets, witnesses, system hashes, raw traces and host details are retained. Independent scientific replay/review is still separate from producer checks.

## Reproduce development checks and formatting

```sh
cargo test --locked --release --manifest-path research/n83_ic_degree_sweep_20261027/Cargo.toml --bin n83-ic-degree-sweep
cargo build --locked --release --manifest-path research/n83_ic_degree_sweep_20261027/Cargo.toml
```

The digest-pinned runtime image and its exact argv are in `request-v2.json`. Research execution follows the repository's EKS workflow and checkpoint gate. Do not resubmit the recorded job or change the frozen files; a new run needs its own identity and protocol.

To download the existing raw evidence, take the exact URI from `evidence/storage.json`, use `aws s3 sync` into a fresh directory, and run `sha256sum -c manifest.sha256` there. Hashes cover all inputs, scripts, controls and traces. The report formatter accepts `report STUDY RAW REPORT_RELATIVE_PATH`; it requires 96 unique rows, checks every trace hash and checks the protocol, binary and image identities. It does not generate new experimental data. Run the canonical `experiment_protocol.py check-export --verify-files` against `experiment.export.json` after hydrating any `.artifacts.json` entries with the repository artifact tool.

The raw results commit precedes the rendered report commit. Root-library regression results and affected fixture rechecks are retained under `validation/`; the standalone tests and solver controls must not be mistaken for a passing repository-wide test run.
