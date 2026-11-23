# Hilbert early stop on the Joux–Vitse Nagao arm at p = 1301–1601 (2026-11-12)

Lead: `LEAD-C-jv-nagao-hilbert-at-scale`, stage 1 (counts) and stage 2 (rescore),
preregistered in `research/leads_20261111/lane_C_prime_ext_cover.md` before any of
these runs. Run because the owner asked for it.

All results are `[pcc:unverified]`:
- local L0 runs on a 4-vCPU cloud container;
- no replay certificate, no validator report;
- the rho reference is the pooled one from another experiment.

The change is classified as **engineering**: the counted cost falls while the
emitted native counter stays flat. It is not an advance.

## What was run

- **Binary.** `target/release/examples/jv_cover`, built from the commit in
  `results/SOURCE_COMMIT`, sha256 in `results/BINARY.sha256`.
- **Arm A (baseline).** The frozen line-search records
  `research/jv_cover_fbline_20261102/results/line_p{p}_s{s}.json`.
- **Drift control.** `aa_line_p1301_s1` re-runs the frozen command on today's
  binary. It reproduces the frozen record exactly: `total_muls` 265,597,174,434,
  `f4_muls`, `f4_native_ops`, `relation_digest`, relations, residuals and S are all
  identical.
- **Arm B.** The same 9 instances (p ∈ {1301, 1451, 1601}, seeds 1–3), the same
  command, plus `--stop hilbert:16`. Driver: `run.sh`.
- **Stage 1.** `compare.py`, output in `results/paired.txt` and
  `results/paired.json`.
- **Stage 2.** The unchanged `negrho_rescore` tool from
  `research/jv_cover_negrho_rescore_20261101/fit` was run on a copy of that study's
  results directory, with `inputs/cover/dlp_p*` replaced by the arm-B records. This
  is the procedure the line-search study used. Output is in `rescore/`.
  - The tool prints "consistency controls FAILED". That is expected. Those controls
    check that the *old* cover records reproduce the old study's numbers, and they
    were replaced on purpose.
  - The rho reference itself reproduces: steps/√l 0.8851 ± 0.0537, against the
    reported 0.885 ± 0.054.

## Stage 1: counts, paired

| p | seeds | S_B / S_A | f4_muls ratio | f4_native_ops ratio | relations / digest | Hilbert fallbacks |
|--:|:--|:--|:--|:--|:--|:--|
| 1301 | 1, 2, 3 | 0.5278, 0.5276, 0.5279 | 0.3895 | 0.9830 | identical 3/3 | 227, 269, 219 |
| 1451 | 1, 2, 3 | 0.5280, 0.5278, 0.5280 | 0.3895 | 0.9830 | identical 3/3 | 270, 253, 266 |
| 1601 | 1, 2, 3 | 0.5277, 0.5278, 0.5277 | 0.3894 | 0.983 | identical 3/3 | 244, 249, 222 |

The fallbacks are out of 265,687–368,577 systems with the stop applied, about 0.08 %.

The preregistered falsifiers all pass:

| Falsifier | Result |
|:--|:--|
| F1: any digest, relation or logarithm mismatch | none, 9 of 9 identical |
| F2: upper 95 % bound of S_B/S_A above 0.70 | all ratios 0.5276–0.5280 |
| F3: fallbacks above 25 % | about 0.08 % |
| F4: native counter up more than 1.10× | 0.983× |

The prediction was 0.54 [0.50, 0.60]. The measured ratio is a 1.895× reduction in
counted multiplications at equal relations.

## Stage 2: distance to matched negation rho

Pooled reference (b), 126 runs, `rescore/table.md`:

| unit | p = 1301 | p = 1451 | p = 1601 |
|:--|:--|:--|:--|
| U2 textbook F_p mults, w = 0, before (line search) | 1.900 above | 1.585 above | 1.318 above |
| **U2, w = 0, with Hilbert stop** | **1.003 [0.916, 1.104]**, not resolved | **0.837 [0.771, 0.910]**, below | **0.695 [0.642, 0.756]**, below |
| U2, w = 0.1 / 1 | 0.861 / 0.831, below | 0.718 / 0.693, below | 0.597 / 0.576, below |
| order holds across w ∈ {0, 0.1, 1} | **no** | **yes (below)** | **yes (below)** |
| U1 group additions, w = 0 | 0.417 below | 0.348 below | 0.289 below |
| modelled U2 crossover p* | 1327 [1278, 1381] | | |

The prediction was 1.03 / 0.86 / 0.71. Figure: `u2_before_after.{svg,png,pdf}`.

**Reading.** On these instances, and in this accounting, the Nagao arm of the
Joux–Vitse cover moves from 1.3–1.9× above matched negation rho in U2 to below it
at p = 1451 and 1601. There the order holds at all three §5a weights. At
p = 1301 it is level with rho (not resolved) `[pcc:unverified]`.

## What this is not

- **Not a leaderboard or benchmark row.**
  - The validation gate needs a replay certificate, a validator report and a matched
    same-binary rho reference.
  - The benchmark-record rule needs raw data in S3.
  - Neither is possible from this session, because its AWS identity is rejected.
    The lead's stage-3 Iso Lab job (9 instances × 3 arms, emitted F4-driver
    counters, 16 fresh rho runs per prime) is the route to a gated row.
- **The cover count is a lower bound.** F_p additions are not emitted on the cover
  side (`rescore/table.md`, unpriced-counter note). A "below" verdict could narrow
  under complete counts; the w = 0.1 and w = 1 columns bound this.
- **Not a statement about any deployed curve.** It concerns the weak class of curves
  over F_{p^6} with a Joux–Vitse cover, at p ≤ 1601.
