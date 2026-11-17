# L14: digit-set frames for the bits factor base (Curve25519 k = 20, census only)

**Class:** exploratory mechanism census in exact counts (pure Python, no engine, no
ecbench, no timing). It measures m = 2 subgroup coverage of candidate factor bases;
it does not solve a discrete log, does not produce F4 mul-adds, and is not a
leaderboard row. Rho remains ahead (T12: the best PKM arm is 6.1·10⁴× behind rho-neg
at w = 0.1 on k = 20); nothing here changes that order.

Status: **design frozen before data** (2026-11-04). No `data/` file exists at
freezing. The only execution before freezing was a syntax smoke test of
`scripts/digit_census.py` at k = 16 with K = B = 30, R = 2, written to the session
scratchpad and not kept; its numbers are not used anywhere below.

Written against the wave-4 brief (lead directory
`research/curve25519_leads_wave4_20261104/l14_digit_set_frames`). Branch
`claude/curve25519-index-calculus-i674j4`; no git operations by this lead.

## 1. Question

The engine's `bits` membership writes a base abscissa as
`x = lo + Σ_{j<t} 2^j b_j`, `b_j ∈ {0,1}`, and frame selection (#3359, T4, T9, T10)
chooses `lo` among p translation frames. The bit-encoded F4 system only sees the
*linear* coefficients of the `b_j`, so `x = lo + Σ_j c_j b_j` with arbitrary digits
`c_j ∈ F_p` gives a system of identical shape (same variables, same degrees, same
monomial support; only field constants change) over a selection space of size
p^(t+1) instead of p.

Q-A. Is a *random* digit set a different kind of object from a random translation
frame, in exact m = 2 subgroup coverage? (Null: same distribution, because each
abscissa carries a point with probability ≈ 1/2 and the cofactor cosets of the
points are pseudo-random in both families.)

Q-B. Does a *greedy* digit-by-digit construction, which selects at every step using
the log-free coset selector J, reach a coverage that a K = 2000 translation-frame
census does not, at an equal search budget (B = K = 2000 candidates per step)?

## 2. Instrument (reused, frozen)

- `research/curve25519_frame_search_20261103/scripts/frame_census.py` (#3359):
  `build(p, A, n)` returns exact, sign-folded full logs in the cyclic group Z/8r,
  indexed by Montgomery `u`; the engine's abscissa is `x_W = u + A/3`.
  `coverage(logs, n, r)` returns `(covered, cols, J)`:
  `covered = |{R ≠ O in E[r] : R = ±P_i ± P_j}|` (the engine's `covered_points`),
  `cols` = number of abscissae carrying points, `J` = number of unordered signed
  pairs whose cofactor cosets cancel (log mod 8), which is what `[r]P` tells
  without a discrete log.
- `research/curve25519_torsion_leads_20261103/scripts/torsion_census.py` (`m2`)
  computes the same quantities per coset; L14 only needs coset 0, so
  `frame_census.coverage` is used directly (one instrument for both families).
- New glue, pure Python: `scripts/digit_census.py` (families, greedy, cost counters),
  `scripts/analyse.py` (the decision rules of section 5, verbatim).

Why J is the greedy's objective: on the existing k = 20 translation census
(`curve25519_torsion_leads_20261103/data/k20.json`, K = 2000, seed 20261124)
Spearman(J, covered) = 1.000 at t = 4, 5, 6 and mean J/covered = 1.000–1.001:
at these base sizes the signed pair sums are almost all distinct, so J *is*
covered, and J needs no logs.

## 3. Design (exact commands, seeds, cells)

Curve: family miniature **k = 20, A = 926, p = 1,048,573, r = 131,203, n = 8r =
1,049,624** (family rule). Tower heights **t ∈ {4, 5, 6}**. `A3 = A/3 mod p`.

Command (the whole lead; `scripts/run.sh` runs exactly this and then the analysis):

```
python3 -I scripts/digit_census.py --k 20 --p 1048573 --A 926 --n 1049624 \
  --t 4 5 6 --K 2000 --B 2000 --R 5 --seed-base 20261104 --out-dir data/
python3 -I scripts/analyse.py          # -> data/summary.json, data/selected_digit_sets.json, data/analysis.txt
```

Per t, one file `data/k20_t{t}.json` holding:

**(a) Random families, one rng stream `random.Random(20261104 + t)`**, drawn in this
order:
1. K = 2000 translation frames: `L = rng.randrange(p)`, base abscissae
   `{(L + y) mod p : y ∈ [0, 2^t)}` (= engine form `bits@L`).
2. K = 2000 digit sets: `lo = rng.randrange(p)`, then `c_1..c_t = rng.randrange(1, p)`;
   base abscissae = all 2^t subset sums `lo + Σ c_j b_j mod p` (repeats, if any,
   collapse; `distinct_x` is recorded).
For each row: `cols, covered, J, distinct_x`. Also the fixed reference `bits lo = 0`
(instrument cross-check, section 4).

**(b) Greedy construction, R = 5 replicates, replicate `rep` on
`random.Random(20261104 + t + 1000·(rep + 1))`**:
`lo = rng.randrange(p)`, `S_0 = {lo}`; for j = 1..t: draw B = 2000 candidates
`c = rng.randrange(1, p)`, evaluate `J(S_{j-1} ∪ (S_{j-1} + c))`, keep the candidate
maximising J, **ties broken by fewer columns, then by the earlier candidate**
(fewer columns is the direction of the T10 cost model E ∝ columns/covered);
`S_j = S_{j-1} ∪ (S_{j-1} + c_j)`. Record per step `(j, c_j, cols, covered, J)`, the
final digit set, and the search cost:
- Legendre symbols = number of *new* distinct abscissae tested per candidate,
  summed (upper bound B·(2^t − 1), equal to the translation census's K·2^t minus B);
- `[r]P` labels = number of new abscissae that carry a point, summed
  (each label costs about 1.5·log₂ r group operations, the #3359 convention;
  `group_ops` is reported with that factor).
The translation census cost is counted the same way (K·2^t Legendre, Σ cols labels).
**The primary greedy result is replicate 0**; replicates 1–4 measure its spread
(P9) and are reported, never selected from.

Reference cells computed **now, from existing data** (k = 20, translation frames):

| t | bits lo=0 covered (frame_search summary, exact) | translation K=2000, seed 20261124: mean / sd / max (torsion census) | frame_search census (other seed): mean / oracle max |
|--:|--:|:--|:--|
| 4 | 12 (7 cols) | 19.26 / 10.94 / 78 | 18.93 / 64 |
| 5 | 42 (11 cols) | 70.90 / 26.62 / 184 | 69.78 / 198 |
| 6 | 162 (25 cols) | 270.54 / 67.54 / 582 | 271.05 / 536 |

So the K = 2000 translation maximum itself moves by about ±10% between seeds
(78 vs 64, 184 vs 198, 582 vs 536); the 1.3× rule of P4 sits above that noise.
The T10 winner for comparison is the isogeny tower (t = 5, seed 1307: 22 columns,
210 covered), not a bits frame; the best bits frame on this curve at t = 5 covers
184–198 with 24–25 columns.

Compute budget: one Python process, well under 10 minutes on the shared host
(log table ≈ 5·10⁵ group additions; part (a) 12,000 evaluations of ≤ 64 abscissae;
part (b) 5·(2^t − 1)·... at most 6 steps × 2000 candidates × 5 replicates per t).
No engine run, no bench lock, no background process needed; a container restart is
handled by re-running `run.sh` (deterministic; complete per-t files are skipped).

## 4. Instrument check (must hold before anything else is read)

P1 below. `bits lo = 0` must reproduce the #3359 census exactly (12 / 42 / 162) and
the translation mean must be within 10% of the 20261124-seed census. If P1 fails,
the run is an instrument failure: report it, fix nothing else, stop.

## 5. Preregistered predictions and decision rules

All rules are evaluated by `scripts/analyse.py` on `data/k20_t{4,5,6}.json`; the
script is frozen with this protocol. "Translation max" and "translation mean" are
from part (a) of the same run (same K, same seed family), not from the reference table.

1. **P1 (instrument).** `bits lo=0` covered = 12, 42, 162 at t = 4, 5, 6 exactly,
   and the K = 2000 translation mean is within ±10% of 19.26, 70.90, 270.54.
2. **P2 (Q-A null, mean).** mean(covered, digit sets) / mean(covered, translation) ∈
   [0.95, 1.05] at every t ∈ {4, 5, 6}. (Expected: holds; the standard error of the
   ratio is about 0.012 at t = 5, so this band is ≈ 4 s.e.)
3. **P3 (Q-A null, max).** max(digit) / max(translation) ∈ [0.75, 1.33] at every t.
   (Expected: holds.) P2 and P3 both holding = the null stands: random digit sets
   are not a new object; any gain must come from *selection* (Q-B).
4. **P4 (the lead's success rule, from the brief).** greedy replicate 0 covered ≥
   1.3 × translation max at **both** t = 5 and t = 6. If P4 holds, the lead stays
   open and an engine confirmation follows (section 7); if it fails, the lead is
   **closed at B = 2000 greedy** (a larger B or a beam search would be a new,
   separately preregistered lead).
5. **P5 (point prediction for Q-B).** greedy replicate 0 / translation max ∈
   [1.10, 1.45] at t = 5 and at t = 6. Reasoning fixed now: J(S_j) = J(S_{j-1}) +
   J(S_{j-1} + c) + J_cross; the greedy maximises 3/4 of the pairs at every step and
   inherits the maximised 1/4 from the previous step, so its excess over the mean is
   about (0.87 + 0.43 + 0.22 + …) ≈ 1.7× a single max-of-2000 excess; with the
   observed heavy upper tail of the translation max the ratio should land near 1.2–1.35.
   Point estimates: t = 5 ≈ 1.3, t = 6 ≈ 1.25. P4 is therefore genuinely uncertain.
6. **P6 (selection works in the digit space).** greedy replicate 0 / translation
   mean ≥ 3.0 at t = 4 and ≥ 2.0 at t = 5 and t = 6.
7. **P7 (equal budget).** greedy Legendre count ≤ 1.0 × the K = 2000 translation
   census Legendre count, and greedy `[r]P` labels ∈ [0.8, 1.2] × the translation
   census labels, at every t. (Analytic: B(2^t − 1) < K·2^t when B = K; labels ≈
   half the Legendre tests in both.) A failure here means the counters are wrong,
   and the cost claims of section 6 are void.
8. **P8 (cost-model direction).** covered/cols of greedy replicate 0 ≥ 0.9 ×
   covered/cols of the best translation frame at t = 5 and t = 6 (the T10 model prices
   E ∝ columns · r · mpt / covered, so a coverage gain bought with proportionally more
   columns is no gain).
9. **P9 (greedy is stable, not a lottery).** over the R = 5 greedy replicates,
   max covered / min covered ≤ 1.3 at t = 5 and at t = 6.

Failed predictions stay failed and are reported as such; no threshold, seed, K, B or
tie rule is changed after data. Secondary, exploratory (no rule): the modelled F4
cost ratio E(greedy)/E(best translation) = (cols_g/cov_g)/(cols_tr/cov_tr), assuming
mpt unchanged (the system shape is unchanged; this assumption is exactly what the
engine confirmation of section 7 would test); the step-by-step greedy trajectory;
the coset histogram of the selected sets.

## 6. Failure handling

- `build` raises (group not cyclic of order 8r) or P1 fails: instrument failure,
  stop, report, no results read.
- Host restart mid-run: re-run `scripts/run.sh`; per-t outputs are atomic and
  complete ones are skipped; nothing is resumed from partial state.
- P4 fails: lead closed at this budget; the selected digit sets are still recorded in
  `data/selected_digit_sets.json` for reference, no engine run follows.
- P2/P3 fail (digit sets differ from translation frames as random objects): report
  the direction; it does not rescue or sink P4, which is decided on its own rule.
- P7 fails: the cost sentence of the report is withdrawn until the counter is fixed
  and the run repeated under a dated addendum.

## 7. What an engine confirmation would need (not done here)

`curve25519_family f4cost` accepts `bits[@LO]` only (`src/bin/curve25519_family.rs`
form parser, lines ≈ 1645–1698). A `digits@LO:c1,…,ct` form would need, in
`src/cryptanalysis/ic_framework/pkm_semaev.rs`: (i) `bits_factor_base` (≈ L449–456,
`xs = (lo + k) mod p, k < 2^t`) to enumerate the subset sums `lo + Σ c_j b_j`;
(ii) the membership polynomial build at ≈ L1249, `Pol::var(n, i·t + j).scale((1 << j) mod p)`,
to scale by `c_j` instead of `2^j`; (iii) the `Membership::Bits` abscissa enumeration
at ≈ L1427 and a `new_digits` constructor beside `new_bits` (≈ L821) carrying the
digit vector; (iv) the form parser in the binary; and for ecbench, the same digits
on the `prime-interval` base and the `pkm-semaev` oracle params. The system's
degree, variable count and monomial support are unchanged by construction, so the
confirmation's prediction would be `mpt(digits) = mpt(bits@LO)` within 1% at equal
(t, G) and `covered_points` equal to this census. No Rust is written by L14.

## 8. Scope and distance remaining

- Scope: one curve (k = 20), t ∈ {4, 5, 6}, m = 2, exact coverage counts; the
  only lever tested is the choice of digits at fixed system shape.
- Even if P4 holds at its best plausible value (≈ 1.45× coverage at unchanged columns
  and mpt), the modelled F4 cost of a bits arm falls by the same ≤ 1.45×. The best
  measured PKM arm (T10 winner, isogeny t = 5 G = 4 seed 1307) is 6.1·10⁴× behind
  rho-neg at w = 0.1 and 17× at w = 0 (T12); a bits arm selected this way would
  remain ≥ 4·10⁴× behind at w = 0.1. Distance remaining to rho-neg after this lead,
  at best: about 4.2·10⁴× (w = 0.1), > 10× (w = 0). Nothing here is a leaderboard or
  improvement claim; a verified improvement would require the section-7 engine
  change plus an ecbench session priced at w = 0, 0.1, 1.
