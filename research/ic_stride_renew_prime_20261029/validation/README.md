# Validation of XP-20261029-stride-renew-prime (P20, P28)

Validator report `VAL-20261029-4aed57`: [`validation_report.yaml`](validation_report.yaml),
[`validator-report.json`](validator-report.json), [`replay-certificate.json`](replay-certificate.json).
Provenance and hashes: [`provenance.txt`](provenance.txt), [`SHA256SUMS`](SHA256SUMS).

## Execution path: native (Rust)

The validator's own recomputation, mechanism split, `[d]G = Q` check and gate-file build run natively in
[`native/`](native/), a standalone Cargo crate (`srenew-validate`, its own `[workspace]`; dependencies
`serde_json` and `sha2` only; it links neither ecbench nor the `crypto` library).

| subcommand | replaces (legacy) | does |
| --- | --- | --- |
| `recompute <session_dir> <label>` | `recompute.py` | completeness, pairing, §5a priced S, paired ratios, one-stage, stratified and two-stage cluster bootstraps (seed 29102026, 4000 resamples, plus 10 more seeds), A/A exactness, band means, stride-renew accounting, and the curve checks: coefficients from the plan points, p and r prime, points on the curve, `[r]G = O`, `#E = r` by the Hasse interval, j-invariant, and `[d]G = Q` in affine coordinates for every measured record |
| `mechanism <session_dir>` | `mechanism.py` | relation, trial, rank and GAE decomposition of walk against srenew with the two-stage bootstrap |
| `gate <study> <band_dir> <val_id> --recompute-dir D --out O` | `build_gate_files.py` | `replay-certificate.json` (with a fresh `[d]G = Q` per run) and `validator-report.json` |
| `cmpjson <a> <b> <rel_tol>` | none | structural and numeric comparison of two JSON files |

The bootstrap draws reproduce CPython's MT19937 seeding and `randrange` exactly. The sums reproduce Python 3.12+
`sum()`, which uses compensated summation, and the JSON writer reproduces `json.dumps` (float repr, indent, key
order, ASCII escaping). The outputs can therefore be compared byte for byte with the committed files. Run
everything with [`native/run.sh`](native/run.sh).

## Equivalence with the committed outputs

Command (2026-11-05, native binary built from this directory with rustc 1.97.0):

```sh
SESSION_ROOT=<validator's original worktree>/research/ic_stride_renew_prime_20261029 \
  research/ic_stride_renew_prime_20261029/validation/native/run.sh
```

`SESSION_ROOT` was the validator's original worktree, the path the committed `recompute-P*.json` record as
`session_dir`. Its `records.jsonl` and `plan.json` have the sha256 values in [`SHA256SUMS`](SHA256SUMS) and
`provenance.txt`, the same as the store copies of this directory. Each comparison is one line in
[`native/out/equivalence.jsonl`](native/out/equivalence.jsonl):

| committed file | native output | result |
| --- | --- | --- |
| `mechanism-P20.json` | `native/out/mechanism-P20.json` | byte-equal |
| `mechanism-P28.json` | `native/out/mechanism-P28.json` | byte-equal |
| `recompute-P20.json` | `native/out/recompute-P20.json` | 276 of 299 numbers bit-equal; 23 differ by at most 4.6e-16 relative |
| `recompute-P28.json` | `native/out/recompute-P28.json` | 287 of 299 numbers bit-equal; 12 differ by at most 3.5e-16 relative |
| `replay-certificate.json` | `gate` from the committed recompute files | byte-equal (sha256 `4927a26e…`) |
| `validator-report.json` | `gate` from the committed recompute files | byte-equal (sha256 `ede09d96…`) |
| `replay-certificate.json` | `gate` from the native recompute files | byte-equal |
| `validator-report.json` | `gate` from the native recompute files | 829 of 832 numbers bit-equal; 3 per-curve `recomputed` ratios differ by at most 2.7e-16 relative |

The stated tolerance is 1e-12 relative, the `rel_tol` that the validator report uses for every exact headline.
There is no structural difference: the keys, strings, booleans and integers all match.

Every value that is not bit-equal is a one-stage or stratified cluster-bootstrap bound, or a per-curve ratio. These
are the only quantities the legacy script builds from per-target sums accumulated in Python `set` iteration order.
That order depends on the string-hash seed, which changes on every run of `python3 -I`, so the legacy script cannot
reproduce its own last bits either. The native path accumulates in sorted key order. Every verdict, count and
`[d]G = Q` result is the same, with 0 failures in 1,440 measured records per session.

## Legacy Python (historical evidence, superseded)

`recompute.py`, `mechanism.py` and `build_gate_files.py` are the legacy Python implementation. On 2026-10-29 they
produced the committed `recompute-P*.json`, `mechanism-P*.json`, `replay-certificate.json` and
`validator-report.json`. AGENTS.md "Implementation language: no Python" requires keeping them unchanged as evidence,
together with their outputs and `SHA256SUMS`. The native path above supersedes them: do not extend them or run them
for new comparisons.

The `method` and `recomputed_by` strings inside the committed reports name the legacy scripts because they are the
2026-10-29 bytes. The native `gate` reproduces those strings verbatim so that its output can be byte-compared.
