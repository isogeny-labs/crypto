> **PROVISIONAL, legacy Python, NOT FOR MERGE: violates the repo's no-Python rule; kept as labelled historical evidence; matched native re-measurement required before any claim.**

# Provisional results: paired conductor-gap smoke test over F_{p^2} (legacy Python)

Scope: toy fields `q = p^2`, `p` in {19, 23, 29, 31, 37, 41, 43}. No security
statement, no ECDLP speedup, no extrapolation. Every timing below is a
historical Python timing on a shared host (load average about 2.3 on 4 cores,
Intel Xeon 2.10 GHz, Python 3.13.16, numpy 2.5.3). Base: `bbd330a9d` (main of
2026-10-11, about 15 days behind the date of the runs, 2026-10-26).
Labels: MEASURED = produced by a run whose receipt is in this directory;
DERIVED = follows from a measured table by arithmetic; NOT ESTABLISHED = said
so explicitly. Commits: protocol `c61993d72`, library `5815a0dd5`, selection
script `bb3e6e4d7` (the revision recorded in `selection_receipt.json`), engines
and tests `99a2954da` (the revision recorded in `environment.json`), exploratory
amendment `d793e3536`.

## 1. Requirement status (original task against evidence)

| requirement | status | evidence or gap |
|:--|:--|:--|
| PROTOCOL.md written before runs | verified complete | commit `c61993d72` predates all code; amendments 1 to 4 logged with dates and reasons |
| `enumerate_pairs.py` to `curves.jsonl`, `selection_receipt.json`, `isogeny_edges.jsonl` | verified complete (legacy Python) | 57 CPU s run, outputs in this directory |
| `paired_trials.py` to `runs.jsonl`, `paired_results.csv`, `analysis.json`, same-conductor control | verified complete (legacy Python) | 576 runs; controls for all 4 pairs |
| `summarize.py`, `RESULTS.md`, PNG, `report.pdf`, volcano diagram | **not attempted** | stopped by the coordinator's no-Python order; this file replaces `RESULTS.md`; the `research-visuals` deliverables (diagram, graph, PDF) are therefore **missing** |
| unit tests (point count vs naive, f_E certificate on a hand-checked example, relation verification) | implemented and passing | 22 tests, 3.4 s: `python3 -m unittest discover -s tests` |
| G0 | **passed** (definition below) | section 3 |
| G1 | **met for the engines used; not met as worded** | section 5 |
| G2 | count criteria nominally met; **not claimed** | section 6 |
| G3 | not attempted | |
| `F_{p^3}` (S_4) | not attempted | needs a separate engine |
| native (Rust) engine or re-measurement | not done | section "Next" |
| push and pull request | not done | GitHub access was lost; the PR body is written but must not be opened as-is |

## 2. Prior work in this repository (found before building)

`research/isogeny_conductor_gap_ic_20261010/` (commits `46a845e0d`,
`2ab74c68d`, `152a6e653`, `9d1d20c63`; merged with #1647) already measured
index calculus across crater-to-floor gaps with Rust/`ecbench` on prime, Koblitz
and random binary curves, and found yield, `S` and F4 solving degree flat across
the gap. Its protocol lists extension fields `F_{p^k}` as "not attempted" and its
pairs are endpoints only. This directory is only a Python probe of that open
cell (extension field, intermediate conductors). It does not supersede that
round.

## 3. Selection and G0 (MEASURED, `selection_receipt.json`, `curves.jsonl`)

- 7 fields, `2q + 6` isomorphism classes each. For every one of the 864 ordinary
  traces (both signs) the number of classes equals the Kronecker class number
  `H(t^2 - 4q)`: **0 mismatches**. An earlier version of the enumeration had a
  wrong `j`-model (`b = 2 j (1728 - j)^3`); this identity caught it, it was fixed
  before any selection run, and the unit test
  `test_enumeration_class_counts_equal_kronecker_class_numbers` guards it.
- 70 eligible groups (ordinary, `f_pi > 1`, `l | f_pi` in {2, 3, 5, 7},
  `r >= 4p`, `r^2` does not divide `N`), all `t > 0`: all fully certified
  (1375 curves, 0 failures). In every group the certified `f_E` distribution
  equals the class-number prediction `{f : h(f^2 D_K)}`. Method B (exact `n1` of
  the curve and its twist) contradicted Method A nowhere. Ineligible groups, with
  reasons per field, are in the receipt (5 groups had a prime `l` outside the
  supported set, e.g. `f_pi = 11, 13, 33`).
- **Pairs.** 51 independently verified pairs (Amendment 1 definition: both
  certified, `f_E1 != f_E2`, group-level check holds, and Method B decisive for
  both curves at a differing prime), one per group, 51 distinct `(q, t)`;
  19 certified groups had no independently verified pair. Conductor pairs by
  `f_pi`: `f_pi = 2: 1 to 2 (13)`, `3: 1 to 3 (23)`, `4: 1 to 2, 1 to 4, 2 to 4 (x2)`,
  `5: 1 to 5 (2)`, `6: 3 to 6`, `7: 1 to 7`, `8: 4 to 8`, `21: 3 to 21, 7 to 21`,
  `24: 8 to 24, 12 to 24`, `35: 5 to 35`, `48: 12 to 16`.
- **G0 (2 independently verified pairs, different conductors): passed**
  (51 against 2 required), under the operational definition above.

Trial pairs (frozen rule: first 4 groups by `p` descending; all are `p = 43`,
`q = 1849`):

| id | t | N | r (cofactor) | f_pi, D_K | f_E1 | f_E2 | ratio | differing primes (B decisive) | curves (j, twist class) | M | path |
|:--|--:|--:|--:|:--|--:|--:|:--|:--|:--|--:|:--|
| P0 | 6 | 1844 | 461 (4) | 8, -115 | 4 | 8 | 2/1 | 2 (2) | `cgp-p43-j207-w0`, `cgp-p43-j404-w0` | 23 | none within depth 3 |
| P1 | 12 | 1838 | 919 (2) | 7, -148 | 1 | 7 | 7/1 | 7 (7) | `cgp-p43-j596-w0`, `cgp-p43-j386-w0` | 18 | one explicit 7-isogeny |
| P2 | 18 | 1832 | 229 (8) | 2, -1768 | 1 | 2 | 2/1 | 2 (2) | `cgp-p43-j70-w0`, `cgp-p43-j100-w1` | 21 | none within depth 3 |
| P3 | 22 | 1828 | 457 (4) | 48, -3 | 12 | 16 | 4/3 | 2, 3 (3) | `cgp-p43-j380-w1`, `cgp-p43-j649-w1` | 21 | none within depth 3 |

P1 detail (MEASURED): the `f_E = 1` curve has `psi_7` split completely
(24 linear factors), 8 rational 7-isogenies; the `f_E = 7` curve has exactly 1;
the twist of the first has full 7-torsion in `E(F_q)` and the second none.
Same-conductor controls exist for all four (`cgp-p43-j431-w0`, `-j1327-w0`,
`-j153-w1`, `-j242-w0`); isomorphic-model controls (`u = theta`) for all four.

## 4. Paired trials (MEASURED, `analysis.json`, `paired_results.csv`, `runs.jsonl`)

12 seeds, 24 attempts per run, both engines, 576 runs in 10 s wall (6.8 CPU s
inside runs), 0 censored, 0 verification failures, 0 cross-engine
disagreements on 192 `(curve, seed, M)` keys, and the isomorphic-model run's
in-base solutions were exact images `x -> n x` of the original's in 2300 of 2300
attempts. Observed Groebner solving degree (largest S-polynomial degree in a
pure-Python Buchberger, not F4/F5): **4 on both curves of every comparison**;
quotient dimension at most 4.

Primary endpoint, `weil_gb`, `diff = ln(CPU s per independent relation)[larger
f_E] - [smaller f_E]` (so `exp(mean)` is a cost ratio), 95% paired bootstrap CI,
exact sign-flip permutation p, Holm over the 4 pairs:

| pair | mean | CI95 | cost ratio | p | Holm p | seeds +/- | relations A/B | rank A/B |
|:--|--:|:--|--:|--:|--:|:--|:--|:--|
| P0 (4 vs 8) | -0.020 | [-0.127, 0.073] | 0.980 | 0.740 | 1.0 | 7/5 | 13.9/14.5 | 12.8/13.4 |
| P1 (1 vs 7) | +0.051 | [-0.143, 0.273] | 1.052 | 0.676 | 1.0 | 5/7 | 9.5/9.5 | 9.2/9.2 |
| P2 (1 vs 2) | -0.059 | [-0.217, 0.101] | 0.943 | 0.488 | 1.0 | 6/6 | 12.2/14.3 | 10.8/12.4 |
| P3 (12 vs 16) | -0.106 | [-0.280, 0.065] | 0.899 | 0.282 | 1.0 | 4/8 | 11.5/12.8 | 11.2/12.2 |
| pooled (stratified) | -0.034 | [-0.116, 0.050] | | | | | | |

Controls, `weil_gb`: isomorphic model: P0 -0.022 [-0.090, 0.040], P1 +0.008
[-0.019, 0.034], P2 -0.000 [-0.083, 0.086], P3 -0.084 [-0.175, -0.004] (p 0.106);
same conductor: P0 +0.016 [-0.111, 0.124], P1 +0.103 [-0.107, 0.321], P2 -0.033
[-0.087, 0.030], P3 +0.026 [-0.176, 0.236]. Exhaustive engine, pairs: P0 -0.025,
P1 -0.002, P2 -0.118 [-0.248, 0.008] (Holm 0.47), P3 -0.111 [-0.274, 0.044]
(Holm 0.70); pooled -0.064 [-0.140, 0.015].

**Decision under the frozen rule: `no_reproducible_difference_detected` for all
four pairs and both engines** (no CI excludes 0; smallest Holm p is 0.47).
Reading, labelled as such: with 12 seeds the CI half-widths are about 0.1 to
0.2 in log units, so a difference in cost per independent relation of roughly
10 to 25 percent or less is below this experiment's power. Per-run CPU timing is
noisy: in the G1 replay the ratio replay/original ranged 0.53 to 1.35 (median
0.93) on identical work. The engines' cost shape is the same for every
`(a, b)` by construction (the Weil-restricted system has the same degrees), so
a null here is the expected outcome and says little about other solvers.

## 5. G1 and the replay (MEASURED, `replay_report.json`)

Seeds 1 to 3 of every comparison and engine were re-run in a new process
(144 runs): target fingerprints, relation fingerprints and ranks are
**identical in all 144**; 0 cross-engine mismatches. Every relation passed the
exact check (group law plus discrete-log identity in the order-`r` subgroup).
G1 as worded requires "F4/F5 or SAT runs": the engines used are a pure-Python
Buchberger and an exhaustive enumeration, so G1 is met for those engines and
**not met as worded**; and all of it is legacy Python.

## 6. G2 and an exploratory yield analysis

G2 count criteria (at least 10 paired seeds per pair, a same-conductor control)
are numerically satisfied (12 seeds, 4 controls). It is **not claimed**: the
runs are 5 to 20 ms Python runs with the power limits above, and the engine is
not the one G2 envisaged.

Exploratory, post hoc (Amendment 3, not a gate): exact relation yield per
curve, for all 1102 trial-eligible certified curves in the 51 pair groups,
permuting `f_E` labels within groups (20 000 permutations).

| statistic | observed | one-sided p |
|:--|--:|--:|
| normalised yield `z = y / |F|^2`, between-conductor sum of squares | 6.1e-6 | 0.48 |
| `z`, mean abs. difference (different `f_E`) minus (same `f_E`) | 1.9e-6 | 0.147 |
| calibration: a meaningless label (index parity), `z` | 1.4e-5 | 0.038 |
| raw yield `y`, between-conductor sum of squares | 1.82 | 0.0003 |
| raw yield `y`, mean abs. difference contrast | 0.0152 | 0.0001 |

Reading (interpretation, not established): once the factor-base size `|F|` is
divided out, `f_E` does not explain `z`. The raw yield is associated with `f_E`
within groups, which is consistent with the factor-base size `|F|` differing
between conductor classes, but **that was not tested** (it needs a test of `|F|`
against `f_E`, and one of group structure `n1`, which depends on `f_E`). The
meaningless calibration label reached p = 0.038, so one of six unadjusted tests
at 0.05 is within what chance gives and none of these should be read finely.
The matched trials equalise `|F|` by truncation to `M`, and in them mean
relations still differed (for example P2: 12.2 against 14.3, while its
same-conductor control differed by 12.2 against 14.9): relation yield is a
per-curve property that is not fixed by `f_E`.

## 7. What was not done and why

- No figure, diagram, `RESULTS.md`, PDF: the coordinator's order to stop Python
  compute came before `summarize.py` was written; typst-py (the only renderer
  that was available) is itself Python. `research-visuals` is therefore unmet.
- No native implementation; no `F_{p^3}`; no larger `p`; no G3.
- No push, no PR (access lost). `/home/user/wt/pr-bodies/lead2.md` holds a PR
  body that must not be opened as-is.
- The `|F|`-against-`f_E` and `n1`-against-`yield` follow-ups of section 6.

## Next (single experiment)

A matched native re-measurement of the paired relation cost on certified
`F_{p^2}` pairs, using the existing Rust stack; scoped in the coordinator report
(native conductor-gap code that already exists, and the smallest slice that adds
the missing cell).
