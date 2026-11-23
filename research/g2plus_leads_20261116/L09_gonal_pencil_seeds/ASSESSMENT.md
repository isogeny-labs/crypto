# L09 assessment: free gonal-pencil relations as large-prime graph seeds  [pcc:unverified]

Assessor: independent of the producer and of the replay. Date 2026-11-16. Everything below is
`[pcc:unverified]` (one host, no pcc claim id). Not a benchmark: no leaderboard row, no `benchmarks/` row,
no speed claim, no wall time. Assessor-written files: this file and `assess/` (an independent stdlib
re-implementation, its outputs and a comparison script). No other file was edited.

## Verdict

**INCONCLUSIVE** (headline, p = 16381) and at every prime, by the frozen rule: gate G5 failed, and PROTOCOL
section 7 says "If G5 fails, the union-find `T` is not a valid proxy and every p is INCONCLUSIVE."

The failure is not an executor error. The assessor found that G5 was **structurally unpassable as frozen**:
arm A's exact target `dim >= n_cols - 1` is unreachable by construction (section 3 below), so the A half of
G5 can never produce a single data point; and arm B's own proxy gap (median `T_exact/T_uf - 1` = 0.124
pooled; 0.0 to 0.40 by cell) is outside [0, 0.03] too. The protocol's degree-only rank argument (section 2.6)
missed two structural constraints of lines through a fixed point `O` on this model.

## 1. Frozen files, replay

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK (assessor ran it).
- Replay (`replay/REPLAY.md`): 38 of 40 output files byte-identical; the two differing files are cargo logs.

## 2. Instrument audit

| check | how | result |
|---|---|---|
| Headline numbers | re-derived from `raw/cell_p16381_b0.jsonl` with an assessor-written median and SplitMix64 bootstrap (seed 0x4C09B007, 10,000 resamples) | median r_B = 0.1554 (seed range 0.141..0.163), N = 0.0060, bootstrap CI95 [0.1503, 0.1575], median r_PC4 = 0.495, median f_B = 0.0732, N_f = 0.0055, 0 censored in all arms. Matches `RESULTS.md` Table 4. |
| beta*(p) | recomputed from A2 records only (`median(fpm_rel + 24 n_cols^2)`, at most 2 A2 censored) | 3/4, 3/8, 3/16, 1/8 for p = 251, 1021, 4093, 16381. Matches. |
| G3 | recomputed pooled fractions from `raw/census_*.jsonl` | x/y/line split 0.1560/0.1610/0.1574 (251), 0.1653/0.1661/0.1641 (1021), 0.1689/0.1679/0.1678 (4093), 0.1666/0.1667/0.1660 (16381); point sums = N1. Pass. |
| G6 | string-compared `g6_rerun.jsonl` with seeds 0, 1 of `cell_p1021_b2.jsonl` | identical. Pass. |
| G7 | recomputed `T_B <= T_A + 64` over all 480 (p, beta, seed) | 480/480, 0 violations. Pass. |
| G4 | `g4_ok` in all A and B exact records | all true (also implied by the independent check below, which computes rank_LP by elimination, not union-find). |
| Curves, points, fibres (independent) | regenerated the p = 251 curves of seeds 0..5 with an assessor-written SPEC section 3 RNG and brute-force point enumeration over F_251^2 | N1, redraws, split_x, split_y equal the census records in all 6 seeds. |
| Line trial (independent) | brute-force (1,1)-form through O, P1, P2 and intersection with the F_31 fixture curve | the 4 fixture trials: degenerate, no split, residuals (9,13) (26,23) (30,11), no split; matches SPEC section 12. |
| Whole arms A and B (independent) | `assess/indep_check.py`: curve, shuffle, FB, pair stream, brute-force lines and fibres, dedup, `Y = rows - rank_LP` by exact elimination (no union-find), exact `dim` | p = 251, beta 3/4, seeds 0..19: T_A, T_B (proxy), pencil_rows, N1, B exact T, A exact trials and final dim all equal to `raw/` in 20/20 seeds (`assess/compare_p251_b5.txt`). p = 1021, beta 3/8: see section 4. |
| FPM counters | read `field.rs`, `poly.rs`, `curve.rs`, `arms.rs` | mul counted per call, squarings as mul, inv = 10, phase snapshots around pencil / pcfree / line. Line-trial code is shared by all arms; pencil cost (about 433 FPM per fibre at p = 16381: 14.2M FPM / 2p) is charged to B; PC free rows are uncharged by design (structural comparator, not in the decision). |
| Baseline fairness | read `arms.rs` | A, B, PC share curve, O, FB and stream 1 (CRN). A pays nothing B does not. B pays an extra column (target one higher) and the pencil walk. Nothing in A is needlessly expensive relative to B. Charging B for fibre split tests is conservative here (setup already enumerated every x-fibre), and correct for a scalable algorithm. |

Bug that could produce the effect: none found. The proxy effect (B needs fewer line trials to reach `Y >=
n_cols - 1`) is reproduced exactly by an independent implementation that shares no code with the producer.

## 3. Why G5 could not pass: two structural rank deficiencies the protocol missed

PROTOCOL 2.6 says the rank of the relation lattice over F_ell is `n_cols - 1` (degree is the only
obstruction). That is true for all relations, but not for the sublattice the arms can generate:

1. **O/K tie (arm A).** Every A-type row has +1 on `O` and -1 on `K` (SPEC 6), so every A combination has
   `r_O + r_K = 0`, a second linear constraint besides degree. A's exact dimension is at most `n_cols - 2`.
2. **Points on O's fibres (both arms).** A (1,1)-form restricted to a fibre `x = c` is linear in `y`; through
   two points of one fibre it contains the fibre and the trial is degenerate. So a line through `O` and an FB
   point `P` with `x_P = x_O` or `y_P = y_O` is always degenerate: such `P` never appears in any A row, and its
   column is zero in A's span. In B such `P` appears only through the fibre row(s) that contain it.

Independent measurement (p = 251, beta 3/4, 20 seeds; `k` = FB points sharing a coordinate with O):
A's final exact dimension after exhausting all pairs is `n_cols - 2 - k` in 19/20 seeds and
`n_cols - 3 - k` in one (seed 11). Consistent with the producer's raw data: `dim - (n_cols - 2)` never
exceeds 0 in 180 A exact records. Arm B also misses `n_cols - 1` in 8/20 seeds at this cell
(`raw/exact_p251_b5.jsonl`), for a reason of the same kind (some FB columns are reached by too few rows).

Consequence: the A half of G5 had no admissible run by design, so G5 fails whatever the data. This is a
protocol-design defect, not an executor defect, and it does not by itself say the proxy result is wrong.

## 4. What the proxy gap does to the effect size (exploratory, assessor's own target)

Not part of the frozen rule. To see whether the union-find proxy inflates or deflates `r_B`, the assessor
stopped each arm on its own reachable exact maximum: A at its final (saturation) dimension, B at
`n_cols - 1` (B-censored seeds dropped).

| cell (seeds) | match with raw/ | A deficiency `n_cols - dim_max` | A: `T_sat/T_uf - 1` median [range] | B uncensored at `n_cols-1` | B: `T_exact/T_uf - 1` median | median paired (B ratio - A ratio) | median r_B proxy | median r_B exact-stop [range] | median (exact - proxy) r_B |
|---|---|---|---|---|---|---|---|---|---|
| p = 251, beta 3/4 (0..19) | 20/20 exact | 2 + k (19/20), 3 + k (1/20) | -0.015 [-0.16, 1.20] | 12/20 | 0.199 | +0.172 | 0.439 | 0.325 [0.0, 0.583] (n = 12) | -0.100 |
| p = 1021, beta 3/8 = beta* (0..9) | 10/10 exact | 2 + k (10/10) | -0.001 [-0.012, 0.095] | 10/10 | 0.000 | +0.001 | 0.303 | 0.302 [0.022, 0.393] (n = 10) | -0.001 |

Commands (from the lead directory): `python3 assess/indep_check.py 251 5 0 20 > assess/indep_p251_b5.jsonl`;
`python3 assess/indep_check.py 1021 3 0 5` and `... 1021 3 5 5` (seeds 0..9 only, for the CPU budget);
`python3 assess/compare.py P B FILES...`. Deterministic (no randomness beyond the SPEC seeds).
Files: `assess/indep_p251_b5.jsonl`, `assess/indep_p1021_b3_s0-4.jsonl`, `assess/indep_p1021_b3_s5-9.jsonl`,
`assess/compare_p251_b5.txt`, `assess/compare_p1021_b3.txt`; hashes in `assess/SHA256SUMS`.

Reading: at p = 251 (tiny, near pair exhaustion) the proxy overstates B's saving by about 0.10 and is not
valid. At p = 1021 at its beta*, once A's target is corrected, the proxy and exact stopping agree to 0.001
in the median for both arms; the exceptions are seeds with FB points on O's fibres (seed 3: r_B 0.309 proxy,
0.022 exact, because B must also determine those columns, which A cannot reach at all). This suggests, on 10
seeds of one cell, that the union-find proxy is adequate at p >= 1021 with corrected targets, but that is an
exploratory reading on a target the assessor chose after seeing data, so it cannot rescue the frozen verdict.

## 5. Frozen decision rule applied

PROTOCOL section 6/7, mechanically:

| p | beta* | median r_B | CI95 | thr = 0.05+N | r_PC4 | censoring ok | gates | verdict (trials) | median f_B | thr_f | verdict (FPM) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 251 | 3/4 | 0.439 | [0.404, 0.469] | 0.089 | 1.000 | no (PC4 4/20) | G5 fail | INCONCLUSIVE | 0.308 | 0.086 | INCONCLUSIVE |
| 1021 | 3/8 | 0.308 | [0.297, 0.317] | 0.072 | 1.000 | yes | G5 fail | INCONCLUSIVE | 0.171 | 0.070 | INCONCLUSIVE |
| 4093 | 3/16 | 0.221 | [0.219, 0.224] | 0.069 | 0.663 | yes | G5 fail | INCONCLUSIVE | 0.126 | 0.069 | INCONCLUSIVE |
| 16381 | 1/8 | 0.155 | [0.150, 0.157] | 0.056 | 0.495 | yes | G5 fail | **INCONCLUSIVE** (headline) | 0.073 | 0.056 | INCONCLUSIVE |

Counterfactual, stated only so the reader knows what is at stake and not as a verdict: had G5 passed, the
proxy numbers at p = 16381 would meet the SUPPORTED criterion for trials (0.155 >= 0.056, CI low 0.150 >
0.006, positive control 0.495 >= 0.056) and for FPM (0.073 >= 0.056, CI low 0.067 > 0.006). The FPM
margin is small (0.017) and falls with p.

Effect sizes with spread (proxy, seeds 0..19, median [q25, q75]): r_B 0.155 [0.150, 0.158] and f_B 0.073
[0.066, 0.075] at p = 16381; at beta* for 251 / 1021 / 4093 / 16381, f_B = 0.308 / 0.171 / 0.126 / 0.073.
The saving shrinks by roughly 0.7x per 4x in p, as the PROTOCOL section 1 prediction (saving of order beta*,
with beta* falling) expects. The trial saving at 16381 (0.155) sits at the predicted 10-25%; the FPM saving
(0.073) is above the predicted "under 5%". Structural comparison at p = 16381: median(r_B - r_PC1) = +0.038
at beta 1/8 and within +-0.01 at beta >= 3/16, so pencil rows are worth about as much as line rows of the
same LP type (slightly more at the smallest base).

## 6. Distance to matched rho

Not measured: this lead has no matched rho arm and no complete DLP; its metric is a relation-phase proxy at
p <= 2^14 on synthetic split genus-4 curves. What it implies: at most a constant-factor relation-phase
saving (proxy f_B = 0.073 at p = 16381, decreasing with p), on a curve class (split canonical quadric) that
is a positive-density subset of genus-4 curves. It does not change the exponent of the genus-4 Diem-type
2LP algorithm and so does not change the asymptotic gap to rho; whatever that gap is for the genus-4 line
method, this lead would at best shave a few percent off the relation phase at these sizes and less at
larger p.

## 7. Concerns

1. **Protocol defect, not executor defect:** G5's exact target `n_cols - 1` is unreachable for arm A (O/K tie
   plus FB points on O's fibres); G5 could never pass. The same defect means the proxy stops A at a state
   where k FB logs (those on O's fibres) are unknowable from A's rows, which B can partly reach through
   fibres. Arms therefore do not produce the same output at their stop; the comparison is of
   proxy-stopping times, not of equal deliverables.
2. **Proxy validity is measured only at p <= 1021.** The independent check suggests agreement at p = 1021
   beta* (10 seeds) and disagreement at p = 251; the headline prime has no exact check at all.
3. **Decision cell censoring:** at p = 251 PC4 is censored in 4/20 seeds at beta* (above the 2/20 limit),
   an independent reason for INCONCLUSIVE at that prime.
4. **Positive control is weak at large p in trial units:** r_PC4 = 0.495 at p = 16381 versus r_B = 0.155; it
   clears the threshold, but PC rows are free (uncharged), so the control says the detector sees supply, not
   that it prices it.
5. **Scope:** synthetic (3,3) curves on the split quadric only; smoothness over F_pbar not certified (X4);
   the result says nothing about non-split genus-4 curves (pencils not rational) or about any deployed curve.
6. **Deviations:** the 12 items in DEVIATIONS.md were checked; none changes a number in the decision cells
   (item 2 changes the pair-stream code path only where the original would hang; item 1 moves a unit test
   off a pair-exhausted cell). The FPM positive control (item 5) is an executor choice the spec left open.
7. **Assessor runs:** the two p = 1021 independent processes (5 seeds each, single thread) took roughly 6-9
   CPU-minutes each by ps sampling (not recorded); no hook refused a command.

## 8. Single most valuable next step

Re-freeze G5 with arm-specific reachable exact targets and re-run only the exact sub-runs plus a G5-sized
exact check at the headline prime: A stops at `dim >= n_cols - 2 - k_A`, B at the analogous reachable
maximum, where the deficiency is computed from the instance (O/K tie, FB points sharing a coordinate with O),
or more simply exclude from FB the points on O's two fibres (and drop the `K`/`O` double count) so that both
arms' exact targets are `n_cols - 1` and `n_cols - 2` with no instance-dependent term. Then the proxy
validity of both arms can be measured, and the frozen decision rule can be applied to the existing p = 16381
cell data if the proxy gaps of A and B agree. If they do not, the decision must move to exact-rank stopping,
which needs an exact engine fast enough for p = 16381 (the Rust engine here densifies each incoming row
and was sized for p <= 1021).
