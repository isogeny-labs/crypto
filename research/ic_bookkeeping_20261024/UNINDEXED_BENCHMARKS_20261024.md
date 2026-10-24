# Unindexed benchmark directories (2026-10-24)

Record of the 61 directories under `benchmarks/` that have no row in `benchmarks/README.md`. Nothing here changes that table. The list was made on `origin/main` at `5cb553810e`.

## Why no rows were added

- `benchmarks/README.md` admits a row only with a replay certificate, a validator report and a matched rho reference, named by the directory's `VALIDATION.json` (validation gate, owner rule of 2026-10-19).
- `docs/VALIDATION_GATE.md`, "Row standing": a row that is not frozen in `PRE_GATE.json`, and not validated, has standing **unvalidated**. Its note reads "the builder refuses it, CI fails".
- `benchmarks/PRE_GATE.json` freezes the rows that existed when the gate began (116 keys). None of these 61 directories is among them, so none can render as "unvalidated (pre-gate)".
- Of the 61, none has a `VALIDATION.json`. The three directories that do have one (`20261017-isogeny-volcano-cost-v2`, `20261017-volcano-ecdlp-census`, `20261022-strong-rho-batchinv`) already have rows.
- Adding any of the 61 as a row would therefore make the builder refuse the table and fail CI.

Proposed disposition, same for every directory: validate it (produce its replay certificate, validator report and matched rho reference, then add the row) or archive it. Archiving needs a `benchmarks/ARCHIVE.json` entry with a reason (`incomplete`, `censored`, `documentary` or `superseded`) and provenance, which is an owner-side change. Directories with neither README nor RESULT are proposed for archive as `incomplete`.

One directory first listed in the earlier 62-row version, `20261017-volcano-C-finite-field`, already has a row at `benchmarks/README.md:66` and has been removed from this list. The count is therefore 61.

Date is taken from the directory name prefix. README and RESULT columns record only whether a file named exactly `README.md` or `RESULT.md` exists at the top of the directory. No content was read.

## The 61 directories

| # | directory | date (from name) | README | RESULT | proposed disposition |
|--:|:--|:--|:--:|:--:|:--|
| 1 | `20261005-ic-semiregular-law` | 20261005 | yes | no | validate and add row; archive if validation cannot be completed |
| 2 | `20261010-isogeny-campaign-and-profiler` | 20261010 | yes | no | validate and add row; archive if validation cannot be completed |
| 3 | `20261011-conductor-gap-round3` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 4 | `20261011-eigen-seta-1813z-bench` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 5 | `20261011-eigen-seta-1827z-sidechannel` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 6 | `20261011-eigen-seta-1944z-l1-fix` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 7 | `20261011-eigen-seta-1946z-l3-l5-fix` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 8 | `20261011-eigen-seta-1955z-final-l1` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 9 | `20261011-eigen-seta-1955z-verify` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 10 | `20261011-eigen-seta-1956z-pke-l1` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 11 | `20261011-eigen-seta-1958z-pke-l3-l5` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 12 | `20261011-f4-f5-row-allocation` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 13 | `20261011-fiber-ic-scaling` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 14 | `20261011-glv-gls-4d-benchmark` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 15 | `20261011-glv-gls-lattice-cost-model` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 16 | `20261011-isogeny-vertical-margin-historical` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 17 | `20261011-koblitz-4e-historical` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 18 | `20261011-koblitz-m3-1955z` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 19 | `20261011-koblitz-m3-2150z` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 20 | `20261011-koblitz-m3-2357z` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 21 | `20261011-lit2026-w4-early-abort` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 22 | `20261011-lit2026-w5-gate-first-lane-roots` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 23 | `20261011-lit2026-w8-quotient-s3` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 24 | `20261011-lit2026-w8-summation-models` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 25 | `20261011-pdp-ps0-baseline` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 26 | `20261011-pdp-ps1-algebra-gates` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 27 | `20261011-pdp-ps2-scalar-group-law` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 28 | `20261011-pdp-split-hybrid` | 20261011 | yes | no | validate and add row; archive if validation cannot be completed |
| 29 | `20261012-f4-f5-dense-packing` | 20261012 | no | no | archive (incomplete): no README or RESULT |
| 30 | `20261012-f4-f5-fused-readback` | 20261012 | no | no | archive (incomplete): no README or RESULT |
| 31 | `20261012-f4-f5-native-hashing` | 20261012 | yes | no | validate and add row; archive if validation cannot be completed |
| 32 | `20261012-f4-f5-performance-history` | 20261012 | yes | no | validate and add row; archive if validation cannot be completed |
| 33 | `20261012-f4-fp-vs-msolve` | 20261012 | no | no | archive (incomplete): no README or RESULT |
| 34 | `20261012-glv-gls-4d-confirmation` | 20261012 | yes | no | validate and add row; archive if validation cannot be completed |
| 35 | `20261012-lit2026-w12-zero-minor` | 20261012 | yes | no | validate and add row; archive if validation cannot be completed |
| 36 | `20261012-lit2026-w5-gate-first-rref` | 20261012 | yes | no | validate and add row; archive if validation cannot be completed |
| 37 | `20261014-ic-program-paper` | 20261014 | yes | no | validate and add row; archive if validation cannot be completed |
| 38 | `20261014-isogeny-performance` | 20261014 | no | no | archive (incomplete): no README or RESULT |
| 39 | `20261014-p61-class-pilot` | 20261014 | yes | no | validate and add row; archive if validation cannot be completed |
| 40 | `20261015-isogeny-family-census` | 20261015 | no | no | archive (incomplete): no README or RESULT |
| 41 | `20261015-isogeny-kernel-baselines` | 20261015 | no | no | archive (incomplete): no README or RESULT |
| 42 | `20261015-isogeny-native-cpu` | 20261015 | no | no | archive (incomplete): no README or RESULT |
| 43 | `20261015-isogeny-rung4-control` | 20261015 | no | no | archive (incomplete): no README or RESULT |
| 44 | `20261016-n83-finegrid` | 20261016 | yes | no | validate and add row; archive if validation cannot be completed |
| 45 | `20261017-eigen-seta-whitepaper` | 20261017 | no | no | archive (incomplete): no README or RESULT |
| 46 | `20261018-eigen-seta-ifma` | 20261018 | no | no | archive (incomplete): no README or RESULT |
| 47 | `20261018-eigen-seta-speedups` | 20261018 | no | no | archive (incomplete): no README or RESULT |
| 48 | `20261018-jv-cover-cost-breakdown-leads` | 20261018 | yes | no | validate and add row; archive if validation cannot be completed. Linked in prose at `benchmarks/README.md:177` but has no row. |
| 49 | `20261018-jv-cover-sparse-fglm` | 20261018 | no | no | archive (incomplete): no README or RESULT |
| 50 | `20261018-r6-pooled-rho` | 20261018 | yes | no | validate and add row; archive if validation cannot be completed |
| 51 | `20261019-pdp-split-closure-ecbench` | 20261019 | no | no | archive (incomplete): no README or RESULT |
| 52 | `20261019-r6-3-coset-label` | 20261019 | yes | no | validate and add row; archive if validation cannot be completed |
| 53 | `20261020-ic-oracle-setup-blocked` | 20261020 | no | no | archive (incomplete): no README or RESULT |
| 54 | `20261020-joux-vitse-k5-edwards-replay` | 20261020 | no | no | archive (incomplete): no README or RESULT |
| 55 | `20261021-eccp131-exact-probe` | 20261021 | yes | no | validate and add row; archive if validation cannot be completed |
| 56 | `20261022-arm64-f4-defaults` | 20261022 | no | no | archive (incomplete): no README or RESULT |
| 57 | `20261022-certicom131-rho-ic-baseline` | 20261022 | yes | no | validate and add row; archive if validation cannot be completed |
| 58 | `20261023-certicom131-rho-parity` | 20261023 | yes | no | validate and add row; archive if validation cannot be completed |
| 59 | `20261023-ecbench-allocator-confound` | 20261023 | no | no | archive (incomplete): no README or RESULT |
| 60 | `20261024-perfindex-null-validation` | 20261024 | no | no | archive (incomplete): no README or RESULT |
| 61 | `20261024-table-a-rule-v4` | 20261024 | yes | no | validate and add row; archive if validation cannot be completed |
