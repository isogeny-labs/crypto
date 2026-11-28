---
title: "A5: KLPT with CVP strong approximation on math-st's matched ideals, paired with A4"
date: 2026-11-24
---

**Outcome.** Replacing A4's mod-$N$ reduction in strong approximation with the
closest points of the $(c',d')$ lattice lowers the KLPT output norm by a median
of 0.36–0.46 in $\log_p$, on every bit band, run for run. The 28-bit medians
are $p^{3.28}$ ($\ell=2$) and $p^{3.31}$ ($\ell=3$), against the exact optimum
$\approx p/12$. The KLPT/optimum ratio grows like $p^{2.19}$ and $p^{2.16}$.
H5 is met at 28 bits and on the slope. The paired drop is slightly below the
predicted range on one band of six. No falsifier fired. All numbers are
unverified assertions in pcc terms [pcc:unverified].

Protocol: `../PROTOCOLS.md` §A5 (commit `7129e1d`, before any A5 code). Code:
`src/` (A4's crate with step (d) and the $e_1$ rule changed) and `src/bin/pair.rs`
(the paired comparison). Both are native Rust.

![Median $\log_p$ of the returned norm per bit band. Dashed: A4 basic KLPT. Solid: A5 CVP. Data: `results/paired.csv`; layout: `plot.py`.](a5_paired.svg)

## What changed relative to A4

Only step (d). The admissible $(c',d')$ are $v_0+N L$, with
$L=\{(x,y):Cx+Dy\equiv0\bmod N\}$, a lattice of determinant $N^3$. A5
Gauss-reduces a basis of $NL$, enumerates every point with
$p(c'^2+d'^2)\le\ell^{e_1}$, and tries them in increasing norm. The $e_1$
rule is the least $e_1$ with $\ell^{e_1}\ge pN^3$; A4 used $pN^4/2$. The
bump policy, steps (a)–(c) and (e), the seeds and every control are
unchanged.

## Controls (all passed)

| control | result |
|---|---|
| 1. our exact SVP equals upstream `unconstrained_norm` | 108/108 |
| KLPT give-ups | 0 of 1050 |
| $\beta\in I$; $\mathrm{nrd}(\beta)/N(I)=\ell^e$; equivalent ideal is a left $\mathcal O_0$-ideal of norm $\ell^e$ | 0 failures each |
| non-primitive outputs | 0 |
| independent recheck: $\mathrm{nrd}(\beta)$ from recorded coordinates equals $\ell'\ell^e$ | 1050/1050 |
| same $N$ as the paired A4 run | 1050/1050 |

## Results (paired, `results/paired.csv`)

| bits | ℓ | A4 median $\log_p\ell^e$ | A5 median | median paired drop | A5 median $\log_p$(KLPT/optimum) |
|---:|---:|---:|---:|---:|---:|
| 12 | 2 | 3.962 | 3.532 | 0.360 | 2.843 |
| 12 | 3 | 4.013 | 3.568 | 0.410 | 2.868 |
| 20 | 2 | 3.774 | 3.352 | 0.411 | 2.518 |
| 20 | 3 | 3.828 | 3.403 | 0.415 | 2.586 |
| 28 | 2 | 3.725 | 3.280 | 0.438 | 2.434 |
| 28 | 3 | 3.770 | 3.313 | 0.457 | 2.456 |

The OLS slope of $\log_2$(KLPT/optimum) on $\log_2 p$ is 2.189 for $\ell=2$
and 2.158 for $\ell=3$. A4's were 2.660 and 2.621.

## Against H5

| H5 clause | predicted | measured | verdict |
|---|---|---|---|
| median paired drop, every band | 0.40–0.65 | 0.36–0.46 | met in 5 of 6 bands; 12-bit ℓ=2 is 0.36 |
| 28-bit median $\log_p\ell^e$ | 3.05–3.35 | 3.280, 3.313 | met |
| ratio growth | $p^{2.0\text{–}2.3}$ | $p^{2.19}$, $p^{2.16}$ | met |
| falsifiers: drop below 0.30 or above 0.80 | — | none | not triggered |

## Why the drop is smaller than $\log_p N$

This is a decomposition from `summary.json`, not a separate experiment. The
$N$ and $e_0$ parts are identical in A4 and A5, so the whole drop is in $e_1$:
0.41 (12 bits) to 0.44 (28 bits). The change of rule from $pN^4/2$ to
$pN^3$ accounts for $\log_p(N/2)\approx0.47$–$0.48$. The rest is extra $e_1$
bumps. A5 bumped 340 times for $\ell=2$ and 237 for $\ell=3$, against 2 and
1 in A4. At $\ell^{e_1}\approx pN^3$ the search ball holds about $\pi$ lattice
points. After the sum-of-two-squares and parity filters that is often too few,
so the rule pays about one extra power of $\ell$ on a sizeable share of runs.
A rule with headroom, such as $\ell^{e_1}\ge\kappa pN^3$, would trade some of
this back. It was not tested here.

## What this says about Q099

The gap between KLPT and the counting optimum is the algorithm's, not the
instances'. Basic KLPT sits near $p^{3.7}$. With CVP in strong approximation
it sits near $p^{3.3}$ at 28 bits. The optimum is $\approx p/12$, so even the
CVP variant leaves a factor of about $p^{2.2}$, and the gap widens with $p$.
None of these is an index-calculus result. The comparison is with the known
analysis of KLPT, $p^{7/2}$ and about $p^3$, and the measurements sit at those
scales plus polylogarithmic overhead.

## Deviations from the protocol

1. The A4 shuffle of free parameters in step (d) is gone, because CVP order is
   deterministic. The random stream after step (c) therefore differs from A4.
   $N$ is identical in all 1050 pairs, as the paired table shows.
2. The paired comparison needed one more native binary, `src/bin/pair.rs`. The
   protocol did not name it.

## Reproducing

```
./run.sh                                  # A5 runs and summary.json
./target/release/pair ../a4_klpt/results/runs.csv results/runs.csv > results/paired.csv
./render.sh                               # REPORT.pdf (typst wheel on PATH)
```

`runs.csv` is kept out of Git; see `results/MANIFEST.md`.
