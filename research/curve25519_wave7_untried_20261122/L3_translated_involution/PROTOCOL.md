# Protocol: wave-7 lead L3, translated involutions and the E' models (frozen 2026-11-22, before any data)

Phase 1 only (BRIEF.md): this file, the scripts, and a smoke test exist; no census data. Desk
derivations: [DESK.md](DESK.md). Every result sentence carries [pcc:unverified].

## Question

Do degree-2 functions invariant under a translated involution ι_T: P ↦ T − P with T ≠ O (outside
the frame theorem), including the Huff coordinates of E' = E/<T0>, give more exact order-r coverage
than x-frames, at equal t? Do the Jacobi-intersection coordinates of E' (degree 4, DESK §4) do so?

## Mechanism (DESK.md)

- **With a rational half** (T ∈ 2E(F_p)), the function is a translated x-frame, exactly
  (Theorem 1):
  - mixed-sign relations are x-frame relations, unshifted;
  - same-sign relations are x-frame relations for the targets R ∓ T.
- **With no rational half** (log T odd on E; classes (0,1), (1,0), (1,1) on E', which include Huff x
  and Huff y):
  - the function is a different F_p base, a translate over F_{p^4} (on E) or F_{p^2} (on E');
  - it has no within-fibre relations (Theorem 4), so it is predicted to be worse.
- **JI coordinates**: degree-4 pullbacks of x-frames on 2-isogenous curves, predicted at about half
  the coverage.
- **E' x-frames** have 4 self-paired cosets against 2 on E, so they are predicted slightly better at
  small t.

## Curves

Family-rule miniatures, the same curves as `frame_search` after its erratum E1:

| k | p | A | n = #E | r |
|---|---|---|---|---|
| 16 | 65437 | 1058 | 65672 | 8209 |
| 18 | 262133 | 78 | 263096 | 32887 |
| 20 | 1048573 | 926 | 1049624 | 131203 |

- E' is built by Vélu with kernel <T0>.
- Every curve fact is checked by `desk_checks.py` in the run: primality, #E by character sum, cyclic
  Z/8r, A^2 − 4 a non-square, #E' = #E, Z/2 × Z/4r, φ maps E[r] into E'[r] injectively, and the
  halving lemmas.
- The run stops (the analysis is not run) if any desk check fails.
- Targets on E' are E-targets transported by φ, which is a bijection E[r] → E'[r]. So E'[r] coverage
  equals the probability that a transported target is covered.

## Arms

Each arm runs K = 2000 frames per (k, t), each frame with a random Möbius map ν and values
V = ν([0, 2^t)).

| arm | function | T per frame | role |
|:--|:--|:--|:--|
| E.x | Montgomery u (x-frame) | O | null on E; also covered_shift for one odd T |
| E.x_noself | x-frame, within-fibre pairs removed | O | positive control (planted effect) |
| E.iota_even | h_T | T = 2S, S random | control: translated frame |
| E.iota_T0 | h_{T0} (Montgomery y/x) | T0 | control: translated by an order-4 S |
| E.iota_odd | h_T | log T odd | new |
| Ep.x | X on E' | O | null on E'; also covered_shift for one T in each nonzero class |
| Ep.iota_2E | h_T | T = 2S | control |
| Ep.iota_c01 / c10 / c11 | h_T | T random in that class | new |
| Ep.huff_x / huff_y | Huff coordinate | fixed 2-torsion T, class (0,1) | named arm (new) |
| Ep.ji_s / ji_c / ji_d | JI coordinate (degree 4) | — | named arm |

Fixed single frames, recorded for P1-style percentiles but not used as evidence:

- `frame_search` bits_lo0 and edwards_bits, which are the calibration frames;
- Montgomery y/x with λ = y;
- the identity frame y ∈ [0, 2^t) for each E' coordinate.

## Instrument (exact commands)

```bash
cd scripts && ./run_all.sh        # desk_checks.py, census.py at k = 16, 18, 20, then model.py and analyse.py
```

`census.py --k K --t 4 5 6 7 8 --K 2000 --outdir ../data/kK` computes exact integer counts per frame
(`curves.census`). There is no timing and no sampling inside a frame.

- **covered_D**: |{R ∈ E[r] \ O : R = P_i − P_j}|, the mixed-sign set. For symmetric bases it equals
  `frame_search` coverage.
- **covered_full**: |{R ∈ E[r] \ O : R = ±P_i ± P_j}|, all signed pair sums. These include the T-shifts
  D ± T (Lemma 2). For x-frames covered_full equals covered_D.
- **covered_shift_j** (null arms only): the x-frame target set D ∪ (D + T) ∪ (D − T) for a T drawn in
  class j. This is the fair null for covered_full.
- **J_D, J_full**: log-free selectors computed from the coset histogram only. A coset costs one [r]P,
  about 1.5 log2 r group operations per base point, as priced in `frame_search`.
- **n_points, n_fibres**: base size.
- **control_ok** (control arms): per frame, 1 means all three of the following hold:
  - B − S is closed under negation;
  - h_T = M∘x∘τ_{−S} holds on every base point;
  - covered_D and covered_full equal the x-frame census on frame M^{-1}(V) exactly.

  0 means a violation. 2 means fewer than 3 fibres, so M cannot be fitted.

## Seeds and units

- The seed of (k, t, arm) is the first 64 bits of sha256("wave7-L3|k=<k>|t=<t>|arm=<arm>"). Frame i
  is the i-th draw of `random.Random(seed)`: ν, then T or S if the arm needs one.
- Desk checks use seed 20261122 + k.
- Bootstrap seeds derive from the cell name (`analyse.rng_for`).
- Counted unit: distinct covered subgroup points, an exact integer. Coverage is that count / r.

## Estimators

- Per-cell means and sds.
- Ratio of means with a percentile bootstrap 95% interval: 2000 resamples, unpaired.
- Selection, as in `frame_search`: argmax J_D, ties broken by more points, then by lower frame
  index. Reported: the selected frame's covered_D, gain_vs_mean, and the oracle best.
- KS distance for distribution equality.

## Tolerances

tol_t = 0.07 (t = 4), 0.05 (t = 5), 0.04 (t = 6, 7, 8). That is about 3 standard errors of a
ratio of 2000-frame means at a CV of 0.53, 0.37 and 0.25.

## Model predictions (`scripts/model.py`, `model_predictions.json`, sha256 5daf9fb7…f9d0)

Mean covered_D ratio to the matched null, t = 4, 5, 6, 7, 8. k = 16 is shown; k = 18 and 20 are
within 0.01 except where noted.

| arm | t = 4 | t = 5 | t = 6 | t = 7 | t = 8 |
|:--|--:|--:|--:|--:|--:|
| E.iota_odd / E.x (= E.x_noself / E.x) | 0.790 | 0.887 | 0.941 | 0.971 | 0.988 (k = 20: 0.985) |
| E' no-half classes and Huff / Ep.x | 0.653 | 0.796 | 0.889 | 0.945 | 0.977 (k = 20: 0.970) |
| Ep.x / E.x | 1.210 | 1.113 | 1.058 | 1.028 | 1.012 (k = 20: 1.015) |
| Ep.ji_s / Ep.x | 0.500 | 0.501 | 0.504 | 0.517 | 0.564 (k = 20: 0.504) |
| Ep.ji_c / Ep.x | 0.500 | 0.501 | 0.504 | 0.517 | 0.564 (k = 18, 20: 0.674, 0.603, 0.557, 0.53, 0.52) |
| Ep.ji_d / Ep.x | 0.674 | 0.603 | 0.560 | 0.546 | 0.579 (k = 18, 20: 0.500 … 0.516) |
| E.iota_odd covered_full / E.x covered_shift (odd) | 1.081 | 1.041 | 1.020 | 1.009 | 1.002 |
| E' no-half covered_full / Ep.x covered_shift (same class) | 1.150 | 1.078 | 1.038 | 1.017 | 1.004 |
| controls (E.iota_even, E.iota_T0, Ep.iota_2E) / null | 1 | 1 | 1 | 1 | 1 |

Null means predicted: E 19.1, 70.2, 265.5, 990.4, 3271.6 (k = 16) and 19.0, 70.1, 268.2, 1045.9,
4087.2 (k = 20). E' is 23.1, 78.1, 281.0, 1018.6, 3310.1 at k = 16.

## Numbered predictions (decided by `analyse.py`; thresholds fixed here)

**Calibration**

- **CAL1.** E.bits_lo0 and E.edwards_bits covered_full equal `frame_search` `analysis/summary.json`
  (bits_lo0, edwards) exactly in all 30 cells (k = 16, 18, 20; t = 4..8).
- **CAL2.** The E.x mean is within 3 SE of `frame_search`'s `mobius_mean` in ≥ 14/15 cells.
- **CAL3.** The E.x and Ep.x means are within tol_t (relative) of the model in all 30 cells.

**Controls**

- **C1 (translated frames, pointwise).** control_ok = 1 for every frame of E.iota_even, E.iota_T0 and
  Ep.iota_2E, with code 2 (unfittable) ≤ 1% of frames. Any code 0 fails Theorem 1.
- **C2.** The control arms' mean covered_D is within tol_t of the null mean in ≥ 44/45 cells.
- **AA (random control).** For E.x and Ep.x split by frame parity, the 95% CI of the ratio of means
  contains 1 in ≥ 27/30 cells.
- **PC1 (positive control: an effect of the predicted size).** E.x_noself covered_D_noself / E.x
  covered_D, on independent frames:
  - the CI upper bound is < 1 at t = 4, 5, 6 in all 3 k (9/9);
  - the ratio is within tol_t of the model in ≥ 14/15 cells.

  The planted effect is exactly the within-fibre term whose absence P1 predicts. If PC1 fails, P1's
  direction tests are uninformative.
- **PC2 (positive control: the frame-search coset-selection gain).** E.x gain_vs_mean ≥ 2.0 at t = 5,
  and ≥ 1.3 at t = 6 and 7, in all 3 k (9/9). `frame_search` measured 2.53–2.84, 1.95–2.13 and
  1.49–1.68 (translation frames).

**Hypotheses**

- **P1 (no-half ι_T is below the null).** Arms E.iota_odd, Ep.iota_c01, c10, c11, huff_x and
  huff_y:
  - (a) the ratio to the matched null has CI upper bound < 1 at t = 4, 5 in every k (36/36);
  - (b) the ratio is within tol_t of the model in ≥ 85/90 cells.
- **P2 (E' x-frames above E x-frames).**
  - (a) Ep.x / E.x has CI lower bound > 1 at t = 4, 5 in every k (6/6);
  - (b) the ratio is within tol_t of the model in ≥ 14/15 cells.
- **P3 (JI about half).**
  - (a) Ep.ji_{s,c,d} / Ep.x has CI upper bound < 0.75 at t = 5, 6, 7 in every k (27/27);
  - (b) the ratio is within max(tol_t, 0.06) of the model in ≥ 40/45 cells.
- **P4a (same class means same distribution, Lemma 2).** For the pairs (huff_x, huff_y),
  (huff_x, c01) and (huff_y, c01), in cells with a null mean ≥ 50 (t ≥ 5; 36 comparisons), all of:
  - the mean ratio is in [0.95, 1.05];
  - KS D ≤ 1.95·√(2/2000) = 0.0617.
- **P4b (no class effect).** The c01, c10 and c11 pairwise mean ratios are in [0.95, 1.05] in
  ≥ 34/36 comparisons.
- **P5 (the T-shifted full set is an accounting effect).** Each no-half arm's covered_full / the
  null's covered_shift in the same class is within tol_t of the model in ≥ 50/60 cells. The ratio
  covered_full / covered_D (model about 3.0–3.5 below saturation) is reported next to the 3× target
  degree. It is never counted as a gain.
- **P6 (no fatter selection tail).** For each of the 6 no-half arms, take selected covered_D /
  selected null covered_D over the 9 cells (k × t ∈ {5, 6, 7}):
  - the geometric mean is in [0.80, 1.05];
  - no cell exceeds 1.15.

  All 6 arms must pass.
- **P7 (E' selection).** Ep.x selected / E.x selected over the same 9 cells has a geometric mean in
  [1.00, 1.30].

**Failure handling.** A failed prediction stays failed and is reported as failed, with no
retuning. A C1 failure (any code 0) falsifies Theorem 1 as implemented and stops interpretation of
every ι_T arm. A failure of CAL1–CAL3 is reported first, and the affected comparisons are flagged.

## What would count as an improvement

- An improvement would need a no-half ι_T arm, a Huff arm or a JI arm whose **selected** covered_D
  exceeds the selected x-null (P6 or P3 failing upward). The F4 degree must be unchanged, which holds
  for degree-2 mixed-sign relations (DESK §3).
- P2/P7 holding would be a small measured component gain for E' x-frames on the algebraic track,
  with predicted means of 1.21–1.01×. It would need an engine check (`pkm-semaev` on E') before any
  ledger use.
- covered_full gains are accounting.

## Distance to rho (imported, not measured here)

Source: `curve25519_frame_search_20261103/README.md`, "Costs and scope", importing
`curve25519_algebraic_ecbench_20261102`.

- The generic baseline is 7.8× rho at k = 20.
- The best algebraic arm against rho-neg is 18× (w = 0), 9.6·10^5× (w = 0.1) and 5.9·10^6× (w = 1).
- A coverage change of the size predicted here (≤ 1.2× in mean, at small t) does not change any
  order across the §5a band. Claims are confined to the algebraic track, and nothing is extrapolated
  to 255 bits.

## Run estimate

Measured in Phase 1: k = 20, 20 frames × all arms at t = 7, 8, and the full k = 20 desk checks run
once in scratch.

| step | time |
|:--|--:|
| census at t = 8 | about 240 s per k for 2000 frames |
| census at t = 7 | about 75 s per k |
| census at t ≤ 6 | about 40 s per k |
| table build | up to 7 s per k |
| desk checks | about 70 s (k = 20), 5 s (k = 16) |
| **total, one core** | **about 25 min CPU** (upper bound 1 h allowing for host sharing) |

- Peak RSS is 253 MB (k = 20 census).
- Output is 15 npz files of about 0.1–0.3 MB each per k, plus JSON. That is under 10 MB in total,
  with every file under 1 MB.
- It fits the 3 h, one-core, 4 GB budget.

## Sources checked (repository files actually read; knowledge base unreachable)

- `research/curve25519_wave7_untried_20261122/BRIEF.md`
- `research/curve25519_frame_search_20261103/README.md`, `PROTOCOL.md`, `scripts/frame_census.py`,
  `scripts/analyse.py`, `scripts/run_confirm.sh`, `scripts/run_confirm_rulefix.sh`, and
  `analysis/summary.json` (the calibration numbers)
- `research/curve25519_autolab_20261104/ideas/STRUCTURE-R4.md`: lines 154–155 and record
  R4-I2-linear-2torsion-quotient-family (arm B)

## Frozen files (sha256 at freeze)

| file | sha256 |
|:--|:--|
| scripts/curves.py | 9de7f8d3…bd83 |
| scripts/desk_checks.py | f5a54df9…42b1 |
| scripts/census.py | recorded in `FREEZE.sha256` |
| scripts/model.py | b578a136…c937 |
| scripts/analyse.py | recorded in `FREEZE.sha256` |
| scripts/run_all.sh | recorded in `FREEZE.sha256` |
| model_predictions.json | 5daf9fb7…f9d0 |

`FREEZE.sha256` holds the full list. The main session commits it before the run.
