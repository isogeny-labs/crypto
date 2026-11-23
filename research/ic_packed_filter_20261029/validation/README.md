# Independent validation of XP-20261029-packed-filter

This directory holds the PROTOCOL §10 independent validation (TASK-20261029-654f73)
of the packed-filter study:

| file | what it is |
|:--|:--|
| `validator-report.json` | validator-report/v1; pinned by sha256 `df9624c2…` in the `VALIDATION.json` files |
| `replay-certificate.json` | replay-certificate/v1; pinned by sha256 `b357028b…` |
| `REPORT.md` | the human-readable report |
| `native/` | **the execution path**: a standalone Rust crate that recomputes everything above |
| `code/` | the legacy Python validator, kept unchanged as the record of what the validator first ran |

## Execution path: `native/`

`native/` is a standalone crate (`pf-validator`). Its `Cargo.toml` has its own
empty `[workspace]`, and its only dependencies are `serde_json` (with
`preserve_order` and `float_roundtrip`) and `sha2`. It links neither ecbench nor
the crypto library, so it stays independent of the producer. It reimplements
every computing and verifying path of `code/`:

| subcommand | replaces | what it does |
|:--|:--|:--|
| `j1 <B>` | `j1_decision.py`, `vstats.py` | delta-method log-ratio CI, Student-t quantile (own incomplete beta), Welch–Satterthwaite, target-clustered bootstrap, label-swap null |
| `xcheck` | `xcheck_inpod.py` | J1's method against the in-pod `control-comparison.json` |
| `j2` | `j2_instrument.py` | instrument checks, L1/L2 re-grade, positions, counted A/A |
| `j3` | `j3_work.py` | condition 8 charged-work identity, bitop/v1 re-pricing, the 384 × `table_filter_bit_sets` identity, condition 10 |
| `j4` | `j4_band.py` | S′ at w = 0, 0.1, 1 and the §5a order |
| `scalars` | `scalars.py` | [d]G = Q on every record, using its own short-Weierstrass arithmetic over the record's prime; curve bound by j, p + 1 − t = r and [r]G = O |
| `selftest` | `selftest.py` | negative controls on the checkers |
| `report` | `build_report.py` | assembles `validator-report.json` and `replay-certificate.json` |
| `s3check`, `s3requests`, `c2` | `s3check.py`, `s3requests.py`, `c2_identity.py` | read-only S3 listing and hashing through the `aws` CLI (no SDK) |
| `warmup-seqs` | the heredoc in `code/replay.sh` | lists warm-up seqs for `native/replay.sh` |
| `equivalence` | — | the acceptance test (below) |

The outputs are byte-equal because the port reproduces the CPython behaviour
they depend on:

- MT19937 seeding and `random.randrange` / `random.random`;
- CPython's own Lanczos `math.lgamma`;
- the Neumaier-compensated builtin `sum()` of CPython 3.12+;
- int/float semantics;
- `json.dump(indent=1, sort_keys=True)` layout, including float `repr` and `ensure_ascii`.

### Run

```sh
research/ic_packed_filter_20261029/validation/native/run.sh        # offline; reuses the S3 outputs in native/out/
research/ic_packed_filter_20261029/validation/native/run.sh --s3   # also re-runs the read-only S3 checks (aws CLI)
```

`run.sh` builds the crate with `-j2` (`CARGO_TARGET_DIR` defaults to
`/dev/shm/pfv-target`). It writes every output, the report and the certificate
to `native/out/`, then writes `native/out/equivalence.jsonl`. It never writes
the published `validator-report.json` or `replay-certificate.json` in this
directory.

### Equivalence (acceptance test)

`native/out/equivalence.jsonl` has one line per output. It covers every
`code/out/*.json`, `validator-report.json` and `replay-certificate.json`. Each
line records the sha256 of both files and whether they are byte-equal.
Otherwise it records the maximum relative difference over numbers and the
structural differences. A line passes when the files are byte-equal, or when
there is no structural difference and the maximum relative difference is at
most 1e-12. All 13 outputs are byte-equal. A perturbed copy (one float changed
in the last digit, one boolean flipped) is reported as failing.

### Recorded defect reproduced on purpose

`validator-report.json` carries `"c2_identity_facts": "1.3626"`. In
`build_report.py`, the loop `for pair, c1, c2 in …` rebinds `c2`, which had
held `c2_identity.json`. As a result, the last printed per-curve claim was
written where the C2 facts belong. This was recorded as correction
CORR-20261030-1153f8. `native/src/report.rs` reproduces the string so that the
pinned report stays byte-equal, and marks it in a comment. The facts themselves
are in `code/out/c2_identity.json` (and `native/out/c2_identity.json`).

### What is not ported

- **The replay itself.** It is `ecbench verify --replay-all`, which is the
  producer's binary run on a copy of each session. That is its purpose, so the
  crate does not reimplement it. `native/replay.sh` is `code/replay.sh` with
  its embedded Python warm-up listing replaced by `pf-validator warmup-seqs`.
  The report reads the receipts that replay produced, in `code/out/replay/`.
- **`code/write_report_md.py`.** It lays out recorded values from the JSON
  outputs into `REPORT.md` and computes no result, which the repository's
  no-Python rule permits. It is kept as layout tooling.

## Legacy: `code/`

The Python files in `code/` are the validator as it first ran on 2026-10-30,
and they are kept unchanged as that record. Their outputs, in `code/out/`, are
the reference that the native crate is checked against. Do not run or extend
them for new work (AGENTS.md, "Implementation language: no Python"): use
`native/`.
