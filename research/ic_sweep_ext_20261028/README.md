# The residual sweep, exhaustively (2026-10-28)

Lead `ICP-residual-partial-relations` (lead atlas, crypto-autoresearcher), chased past the first study
([`ic_residual_sweep_20261028`](../ic_residual_sweep_20261028/README.md)): sizes 20 to 44 bits, the sweep against rho, BSGS and
kangaroo, variants, base-shape controls and a limited-memory form. Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`,
figures `fig-*.svg` / `.png`). Benchmarks entry: [`benchmarks/20261028-ic-sweep-ext`](../../benchmarks/20261028-ic-sweep-ext/).
39 sessions, 7,716 measured runs.

## What the sweep is

For each target `R = aG + bQ` the oracle `residual-sweep:m=2` forms `R - P_i` over the base's signed points (one counted group
addition each), stores it by abscissa, and turns each collision `R - P_i = t (R' - P_i')` into the base-only relation
`(a - t a') + (b - t b') d = log P_i - t log P_i'`. Variants are oracle parameters: `fold=0` (collide only equal points),
`dp=k` (keep residuals whose hashed abscissa has k trailing zero bits), `pairs=1` (also probe the pair table).

**Classification the evidence supports: a birthday collision search that runs through the index-calculus pipeline, not an
index calculus with a better exponent.**

1. At a fixed base the counted S does not grow with the curve. Slope of log2 S against log2 r (95% bootstrap interval): sweep B=2
   -0.025 [-0.034, -0.017], B=8 -0.014 [-0.020, -0.009], B=32 -0.019 [-0.023, -0.014], B=128 -0.017 [-0.021, -0.013]; rho.negation -0.010 [-0.022, +0.001],
   bsgs.negation -0.007 [-0.013, -0.002], kangaroo -0.023 [-0.031, -0.016]; the tuned pair-table incumbent
   +0.071 [+0.069, +0.074].
2. The collision rows found equal the birthday count `N^2 / (r 2^k)` (times 2 with the sign fold, /2 for k = 0 without) in every variant
   (observed / predicted 0.7 to 1.2; table in the report).
3. A strided pseudo-random base costs 1.034 [1.000, 1.069]
   of the smallest-abscissa base: the base has no algebraic structure to contribute at B = 8.
4. Adding the pair table (hybrid) changes S by 1.004 [1.003, 1.004].
5. With a known-log base one collision row pins the logarithm (rows used = 1): that is a baby-step/giant-step-style search.
   Its S is 0.65 (progression) and 0.35 (random scalars) of the abscissa base's.

## Counted S by size (mean over runs; B = base abscissae)

| bits | rho.negation | rho.negation_lanes | bsgs.negation | kangaroo | sweep B=2 | sweep B=8 | sweep B=128 | pair-table incumbent |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | 1.14 | 1.40 | 1.31 | 2.00 | 3.73 | 4.89 | 13.95 | 20.2 |
| 22 | 1.14 | 1.45 | 0.93 | 2.43 | 3.10 | 4.10 | 13.74 | 21.9 |
| 24 | 1.03 | 1.47 | 1.00 | 3.20 | 2.93 | 4.23 | 13.28 | 22.0 |
| 26 | 0.81 | 0.98 | 1.06 | 0.87 | 3.16 | 4.28 | 11.81 | 23.9 |
| 28 | 1.03 | 1.16 | 0.96 | 2.72 | 2.59 | 3.91 | 11.70 | 24.8 |
| 30 | 0.73 | 0.81 | 1.04 | 1.76 | 2.75 | 3.84 | 11.81 | 29.6 |
| 32 | 1.02 | 1.06 | 0.88 | 1.70 | 2.82 | 4.25 | 11.55 | 33.0 |
| 34 | 0.80 | 0.82 | 0.97 | 1.53 | 3.29 | 3.95 | 11.83 | 40.1 |
| 36 | 1.29 | 1.33 | 1.29 | 1.97 | 2.28 | 3.85 | 11.89 | 42.2 |
| 38 | 0.96 | 0.99 | 1.28 | 1.45 | 2.47 | nan | nan | nan |
| 40 | 0.49 | 0.51 | 1.08 | 1.29 | 2.40 | nan | nan | nan |
| 42 | 0.90 | 0.91 | 0.88 | 1.32 | nan | nan | nan | nan |
| 44 | 1.28 | 1.29 | 0.82 | 2.13 | nan | nan | nan | nan |

Sizes above 32 bits use prime-order curves from `gen_curves.py` (`prime_explicit`; order from a Hasse-interval baby-step giant-step,
kept only if prime). Sizes 38 to 44 bits carry two targets and two rounds (not four and three) and only the limited-memory
variants of the sweep, so their cells are noisier; the table for them has gaps by design (no pair-table incumbent, no large-base sweep: memory).

## Variants at B = 8 (paired S against the plain sweep, pooled over sizes 20 to 36)

| variant | pooled ratio | 95% interval | pairs |
|:--|--:|--:|--:|
| strided pseudo-random abscissae (null control) | 1.034 | [1.000, 1.069] | 324 |
| no sign fold | 1.331 | [1.294, 1.368] | 324 |
| pair table added (hybrid) | 1.004 | [1.003, 1.004] | 324 |
| known-log base, progression | 0.653 | [0.615, 0.694] | 324 |
| known-log base, random scalars | 0.352 | [0.332, 0.374] | 324 |
| distinguished points, k = 2 | 1.438 | [1.398, 1.479] | 324 |
| distinguished points, k = 4 | 2.570 | [2.481, 2.662] | 324 |
| distinguished points, k = 6 | 5.012 | [4.758, 5.280] | 252 |

## Cost model

N group additions yield `N^2 / r` expected colliding pairs among the signed classes (`N^2 / (2 r)` without the fold, divided by `2^k`
with the filter, since equal abscissae hash alike). The solve ends when the column of `d` is pinned, which takes
`K = alpha B + beta` rows (fitted in the report; about half are linearly dependent), so `N = sqrt(K r)` and `S = N / sqrt(r)` is about
`c sqrt(K)`: flat in `r`, rising like `sqrt(B)` for larger bases, smallest at the smallest base. Memory is `N` entries
(`N / 2^k` with the filter). This is the birthday search's cost with the index-calculus pipeline attached.

## Pricing and placement

- Unpriced native work (one hash probe or insert per residual) is priced at 0, 0.1 and 1 GAE in the section 5a band
  (`band/`); the orderings reported above hold across the band.
- Memory is not in S. At 36 bits the plain B = 2 sweep holds 375346 residuals, bsgs.negation
  103364 baby steps, kangaroo 1562 distinguished points, rho.negation none, and the
  tuned pair-table incumbent 4194304 entries.
- **No leaderboard row.** The sweep is a verified, cheap arm of `ic.pipeline`, but the owner's rule is that relabelling a generic
  search is not an improvement of the index-calculus baseline. Whether to list it, and under what label, is the owner's decision.

## Validation

- Every measured run verified (a recovered scalar equal to its planted target). `ecbench verify --replay-all` re-executes every
  measured run of all 39 sessions with a build that differs from the recording one (the merged tree restores three
  round-2 prime-rho speedups that a merge had dropped): counts identical ([`replay-certificate.json`](replay-certificate.json),
  receipts in `verify/`).
- [`validator.py`](validator.py) recomputes every headline number from the raw records without sharing code with `summarize.py`
  ([`validator-report.json`](validator-report.json)).
- The section 5a band of each size is in `band/` (tables in `band/raw/`).
- Unit tests for the oracle (`residual_sweep_recovers_the_planted_log_in_every_variant`,
  `residual_sweep_rows_hold_as_group_equations`: every emitted row checked against brute-force logarithms) run in
  `cargo test --release --lib -- residual_sweep`.
- `python3 scripts/validation_gate.py evidence benchmarks/20261028-ic-sweep-ext`.

## Reproduce

```sh
python3 gen_curves.py && python3 make_specs.py
OUT=sessions ./run_ladder.sh <bits> 1 2 3          # per size; BIN=path/to/ecbench
for s in sessions/*/; do ecbench verify --dir $s --replay-all --exit-code --out verify/$(basename $s).verify.json; done
for b in 20 22 ... 44; do ecbench band --dir sessions/ext-$b-1 sessions/ext-$b-2 sessions/ext-$b-3 --ranks --json band/ext-$b.json; done
python3 make_certificate.py && python3 summarize.py && python3 validator.py && python3 make_readme.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud container. S is a counted quantity, so the claims do not
depend on wall time; runs shared the machine with other jobs.
