---
title: "A6: headroom in the KLPT CVP e₁ rule does not help"
date: 2026-11-25
---

**Outcome.** Adding headroom to A5's strong-approximation rule,
$\ell^{e_1}\ge\kappa pN^3$, makes the returned norm larger for every
$\kappa\in\{2,4,8,16\}$, in every bit band, for both $\ell$. The best rule is
A5's own, $\kappa=1$. Headroom removes the $e_1$ bumps as modelled, but it
charges every run, while bumps fall on only some. A5's suggestion that headroom
"would trade some of this back" is refuted on these instances. All numbers are
unverified assertions in pcc terms [pcc:unverified].

Protocol: `../PROTOCOLS.md` §A6 (commit `cbb4eac`, before any A6 code). Code:
A5's crate with one change, a fifth command-line argument κ in the $e_1$ rule.
Native Rust.

![Median $\log_p$ of the returned norm against κ. Data: `results/k*/paired_vs_a5.csv`; layout: `plot.py`.](a6_kappa.svg)

## Controls (all passed)

| control | result |
|---|---|
| the κ=1 arm reproduces A5 in every column except wall time | 1050/1050 rows, 0 differing columns |
| A5's `runs.csv` used for pairing matches A5's manifest hash | yes |
| control 1 (our SVP equals upstream `unconstrained_norm`) | 108/108 in each arm |
| give-ups, failed exact output controls, non-primitive outputs | 0 in every arm |

## Results

Median $\log_p\ell^e$, 1050 runs per arm, paired with A5 run by run [pcc:unverified]:

| bits | ℓ | κ=1 (A5) | κ=2 | κ=4 | κ=8 | κ=16 |
|---:|---:|---:|---:|---:|---:|---:|
| 12 | 2 | 3.532 | 3.572 | 3.657 | 3.720 | 3.827 |
| 12 | 3 | 3.568 | 3.639 | 3.737 | 3.782 | 3.875 |
| 20 | 2 | 3.352 | 3.370 | 3.422 | 3.468 | 3.519 |
| 20 | 3 | 3.403 | 3.421 | 3.451 | 3.509 | 3.534 |
| 28 | 2 | 3.280 | 3.297 | 3.320 | 3.352 | 3.388 |
| 28 | 3 | 3.313 | 3.332 | 3.347 | 3.370 | 3.426 |

The median of the paired change against A5 is 0 or negative in every cell:
no arm improves the median paired run [pcc:unverified].

Bumps (ball-size bumps, then non-residue bumps) [pcc:unverified]:

| κ | ℓ=2 | ℓ=3 |
|---:|---|---|
| 1 | 340, 292 | 237, 202 |
| 2 | 165, 223 | 111, 170 |
| 4 | 67, 180 | 35, 115 |
| 8 | 16, 148 | 7, 131 |
| 16 | 2, 140 | 2, 132 |

## Against H6

| H6 clause | predicted | measured | verdict |
|---|---|---|---|
| best κ improves the 28-bit median over A5 | by 0.00–0.05 | the best κ is 1, an improvement of 0.00 | met, at the boundary |
| no κ improves it by more than 0.10 | — | none improves | met |
| κ ≥ 8 worse than A5 | — | worse by 0.04–0.11 at 28 bits | met |
| κ=1 control equals A5 | — | exact | passed |

## Interpretation

With the A5 rule, a bump costs one extra power of ℓ on the share of runs that
need it. Headroom of $\kappa$ costs $\lceil\log_\ell\kappa\rceil$ powers of ℓ
on every run in which the rule is the binding constraint. The ball-size bumps
fall as $\kappa$ grows, from 340 to 2 for ℓ=2. The non-residue bumps (ℓ not a
square mod N) stay at about 130–290 for every κ: they are structural and no
amount of headroom removes them. Pay-when-needed beats pay-in-advance here [pcc:unverified].

This closes the A5 open thread. The remaining gap to the counting optimum,
about $p^{2.2}$ at 28 bits, is not a matter of rule tuning.

## Deviations from the protocol

None. The pairing reuses A5's Rust `pair` binary, which labels its columns
"A4" for the baseline and "A5" for the arm. Here they mean A5 and the A6 arm.

## Reproducing

```
./run.sh        # all five arms and the paired tables (about 2 s)
./render.sh     # REPORT.pdf
```

The per-arm `runs.csv` files are kept out of Git; see `results/MANIFEST.md`.
