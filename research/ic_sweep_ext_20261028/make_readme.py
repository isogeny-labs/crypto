"""Write README.md from results.json (so no number in it is typed by hand)."""
import json
import pathlib

HERE = pathlib.Path(__file__).parent
d = json.loads((HERE / "results.json").read_text())
rows, fit, var = d["rows"], d["fits"], d["variants"]
g = lambda b, a, k: next((r[k] for r in rows if r["bits"] == b and r["arm"] == a), float("nan"))
sb = fit["s_vs_bits"]
fx = lambda n: f"{sb[n]['slope']:+.3f} [{sb[n]['lo']:+.3f}, {sb[n]['hi']:+.3f}]" if n in sb else "n/a"
vr = {v["arm"]: v for v in var}
sizes = sorted({r["bits"] for r in rows})
nsess = len(list((HERE / "sessions").glob("ext-*")))
nruns = sum(r["runs"] for r in rows)
scale = "\n".join(
    f"| {b} | {g(b,'rho-neg','mean_s'):.2f} | {g(b,'rho-lanes16','mean_s'):.2f} | {g(b,'bsgs-neg','mean_s'):.2f} | {g(b,'kangaroo','mean_s'):.2f} | "
    f"{g(b,'sweep-b2','mean_s'):.2f} | {g(b,'sweep-b8','mean_s'):.2f} | {g(b,'sweep-b128','mean_s'):.2f} | "
    f"{next((r['mean_s'] for r in rows if r['bits']==b and r['arm'].startswith('pair-walk-b')), float('nan')):.1f} |" for b in sizes)
vt = "\n".join(f"| {v['label']} | {v['pooled_ratio']:.3f} | [{v['lo']:.3f}, {v['hi']:.3f}] | {v['n']} |" for v in var)
txt = f"""# The residual sweep, exhaustively (2026-10-28)

Lead `ICP-residual-partial-relations` (lead atlas, crypto-autoresearcher), chased past the first study
([`ic_residual_sweep_20261028`](../ic_residual_sweep_20261028/README.md)): sizes 20 to 44 bits, the sweep against rho, BSGS and
kangaroo, variants, base-shape controls and a limited-memory form. Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`,
figures `fig-*.svg` / `.png`). Benchmarks entry: [`benchmarks/20261028-ic-sweep-ext`](../../benchmarks/20261028-ic-sweep-ext/).
{nsess} sessions, {nruns:,} measured runs.

## What the sweep is

For each target `R = aG + bQ` the oracle `residual-sweep:m=2` forms `R - P_i` over the base's signed points (one counted group
addition each), stores it by abscissa, and turns each collision `R - P_i = t (R' - P_i')` into the base-only relation
`(a - t a') + (b - t b') d = log P_i - t log P_i'`. Variants are oracle parameters: `fold=0` (collide only equal points),
`dp=k` (keep residuals whose hashed abscissa has k trailing zero bits), `pairs=1` (also probe the pair table).

**Classification the evidence supports: a birthday collision search that runs through the index-calculus pipeline, not an
index calculus with a better exponent.**

1. At a fixed base the counted S does not grow with the curve. Slope of log2 S against log2 r (95% bootstrap interval): sweep B=2
   {fx('sweep B=2')}, B=8 {fx('sweep B=8')}, B=32 {fx('sweep B=32')}, B=128 {fx('sweep B=128')}; rho.negation {fx('rho.negation')},
   bsgs.negation {fx('bsgs.negation')}, kangaroo {fx('kangaroo')}; the tuned pair-table incumbent
   {fx('pair-table incumbent (walk, base ~ r^(1/3))')}.
2. The collision rows found equal the birthday count `N^2 / (r 2^k)` (times 2 with the sign fold, /2 for k = 0 without) in every variant
   (observed / predicted 0.7 to 1.2; table in the report).
3. A strided pseudo-random base costs {vr['b8-null-strided']['pooled_ratio']:.3f} [{vr['b8-null-strided']['lo']:.3f}, {vr['b8-null-strided']['hi']:.3f}]
   of the smallest-abscissa base: the base has no algebraic structure to contribute at B = 8.
4. Adding the pair table (hybrid) changes S by {vr['b8-hybrid']['pooled_ratio']:.3f} [{vr['b8-hybrid']['lo']:.3f}, {vr['b8-hybrid']['hi']:.3f}].
5. With a known-log base one collision row pins the logarithm (rows used = 1): that is a baby-step/giant-step-style search.
   Its S is {vr['b8-klprog']['pooled_ratio']:.2f} (progression) and {vr['b8-klrand']['pooled_ratio']:.2f} (random scalars) of the abscissa base's.

## Counted S by size (mean over runs; B = base abscissae)

| bits | rho.negation | rho.negation_lanes | bsgs.negation | kangaroo | sweep B=2 | sweep B=8 | sweep B=128 | pair-table incumbent |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
{scale}

Sizes above 32 bits use prime-order curves from `gen_curves.py` (`prime_explicit`; order from a Hasse-interval baby-step giant-step,
kept only if prime). Sizes 38 to 44 bits carry two targets and two rounds (not four and three) and only the limited-memory
variants of the sweep, so their cells are noisier; the table for them has gaps by design (no pair-table incumbent, no large-base sweep: memory).

## Variants at B = 8 (paired S against the plain sweep, pooled over sizes 20 to 36)

| variant | pooled ratio | 95% interval | pairs |
|:--|--:|--:|--:|
{vt}

## Cost model

N group additions yield `N^2 / r` expected colliding pairs among the signed classes (`N^2 / (2 r)` without the fold, divided by `2^k`
with the filter, since equal abscissae hash alike). The solve ends when the column of `d` is pinned, which takes
`K = alpha B + beta` rows (fitted in the report; about half are linearly dependent), so `N = sqrt(K r)` and `S = N / sqrt(r)` is about
`c sqrt(K)`: flat in `r`, rising like `sqrt(B)` for larger bases, smallest at the smallest base. Memory is `N` entries
(`N / 2^k` with the filter). This is the birthday search's cost with the index-calculus pipeline attached.

## Pricing and placement

- Unpriced native work (one hash probe or insert per residual) is priced at 0, 0.1 and 1 GAE in the section 5a band
  (`band/`); the orderings reported above hold across the band.
- Memory is not in S. At 36 bits the plain B = 2 sweep holds {g(36,'sweep-b2','entries'):.0f} residuals, bsgs.negation
  {g(36,'bsgs-neg','entries'):.0f} baby steps, kangaroo {g(36,'kangaroo','entries'):.0f} distinguished points, rho.negation none, and the
  tuned pair-table incumbent {g(36,'pair-walk-b2048','entries'):.0f} entries.
- **No leaderboard row.** The sweep is a verified, cheap arm of `ic.pipeline`, but the owner's rule is that relabelling a generic
  search is not an improvement of the index-calculus baseline. Whether to list it, and under what label, is the owner's decision.

## Validation

- Every measured run verified (a recovered scalar equal to its planted target). `ecbench verify --replay-all` re-executes every
  measured run of all {nsess} sessions with a build that differs from the recording one (the merged tree restores three
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
"""
(HERE / "README.md").write_text(txt)
print("README.md written,", len(txt), "chars")
