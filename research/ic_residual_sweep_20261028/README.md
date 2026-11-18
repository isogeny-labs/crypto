# Single-large-prime residual sweep inside ic.pipeline (2026-10-28)

Lead `ICP-residual-partial-relations` (lead atlas, crypto-autoresearcher). New oracle `residual-sweep:m=2`: for each target
it forms every residual `R - P_i` over the base, one group addition each (charged exactly as `subtract` charges its
candidates), stores it by abscissa, and turns each collision `R - P_i = t (R' - P_i')` into the base-only relation
`(a - t a') + (b - t b') d = log P_i - t log P_i'`. No pair table is built. Report: [`REPORT.pdf`](REPORT.pdf)
(source `report.html`, figure `sweep-s.svg` / `.png`). Benchmarks entry:
[`benchmarks/20261028-ic-residual-sweep`](../../benchmarks/20261028-ic-residual-sweep/). `prototype.py` is the
pure-Python version used to predict the curve before the Rust build.

## Results (32-bit prime-order curve, mean S; 36 runs per arm)

| arm | S | S / rho.negation | residuals stored |
|:--|--:|--:|--:|
| rho.negation | 1.02 | 1.0 | |
| pair-table recipe, B = 1024 | 33.01 | 32.4 | |
| sweep, B = 2 | 2.82 | 2.8 | 131737 |
| sweep, B = 4 | 3.13 | 3.1 | 162503 |
| sweep, B = 8 | 4.25 | 4.2 | 233776 |
| sweep, B = 16 | 5.38 | 5.3 | 305883 |
| sweep, B = 32 | 6.80 | 6.7 | 392427 |
| sweep, B = 64 | 9.41 | 9.2 | 548427 |
| sweep, B = 128 | 11.55 | 11.3 | 675776 |

- Every one of the 7 x 36 x 4 sweep runs verified; the sweep beats the pair-table recipe at every size and every base
  (paired S 0.08 to 0.34 at 32 bits).
- At fixed B the S does not grow with the curve (slope of log2 S against bits about -0.02): the work is `c sqrt(r)`
  additions. S grows with the base (log-log slope 0.32-0.36, model 0.5), so the smallest base wins.
- The table grows like `sqrt(r B)` (131737 entries at B = 2, 675776 at B = 128, 32-bit).
  S counts group additions only, so the memory is not in it; rho.negation uses O(1).

## What this is, and what it is not

At B = 2 the base is four points and the search is a birthday collision of `aG + bQ` values with a tiny base: the same
cost profile as a baby-step/giant-step search, reached through the index-calculus pipeline. It is a verified, measured
reduction of the cost of an `ic.pipeline` arm and of its distance to rho (from 32.4x to 2.8x at 32 bits), but
the owner's rule that relabelling is not an improvement applies, and the memory is not priced. I did not add it to
tables A-C of the IC leaderboard; that is the owner's call. The pair-table recipe's own exponent (2/3) is not beaten
by an index-calculus mechanism here: the sweep reaches 1/2 by degenerating to a collision search.

## Validation

- `ecbench verify --replay-all` re-executes every measured run of all 12 sessions
  ([`replay-certificate.json`](replay-certificate.json), receipts in `verify/`): 1,440 replays, all identical.
- [`validator.py`](validator.py) recomputes all 72 headline numbers from the raw records without sharing code with
  `summarize.py` ([`validator-report.json`](validator-report.json): pass).
- The section 5a band is in `band/` (tables in `band/raw/`); every ordering holds across the 0 / 0.1 / 1 GAE band.
- `python3 scripts/validation_gate.py evidence benchmarks/20261028-ic-residual-sweep`: valid.

## Reproduce

```sh
python3 make_specs.py
OUT=sessions ./run_ladder.sh            # BIN=path/to/ecbench
for s in sessions/*/; do ecbench verify --dir $s --replay-all --exit-code --out verify/$(basename $s).verify.json; done
python3 make_certificate.py && python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud container, `--cpus auto --allow-busy` (below L2). Binary: ecbench release build of branch `lead/ic-residual-lp-20261028` (sha256 in every receipt).
