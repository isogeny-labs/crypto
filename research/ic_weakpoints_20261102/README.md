# The six weak points of EC index calculus, measured on Weil-descent systems (2026-11-02)

Scope: Gaudry/Diem descent, E: y² = x³ + ax + b over F_{p^n} with random a, b ∈ F_{p^n}, factor base
{P : x(P) ∈ F_p}, n ∈ {2, 3}; toy sizes (group ≤ 2^28 for the complete solves). All figures are
**exact counts** from the native tool [`tool/`](tool/) (Rust, std only), run on one 4-vCPU cloud
container. No wall time is reported or claimed. This is **not** an ecbench session, **not** a
leaderboard row and **not** admitted by the validation gate; it changes no table A–C row.
Protocol frozen before data: [`PROTOCOL.md`](PROTOCOL.md) (commit `97b0965fa7` adds Amendment 1).
Report PDF: [`reports/run-20261102T1639Z/REPORT.pdf`](reports/run-20261102T1639Z/REPORT.pdf). Raw records: [`data/`](data/); native summaries:
[`data/summary.jsonl`](data/summary.jsonl), [`data/summary_y2.jsonl`](data/summary_y2.jsonl), [`data/summary_dlp.jsonl`](data/summary_dlp.jsonl).

## Results by weak point

| # | Weak point | Measured here (exact) | Prior lab evidence (map) | Status after this run |
|---|---|---|---|---|
| 1 | Groebner solving degree, matrix growth | n=3: solving degree **10 in 88/88** systems, p = 31 … 1073741789 (2^5–2^30); Macaulay matrix at that degree 252 × 286, **c(3) = 1.03–1.07 × 10^6** F_p mults (dense, upper bound for F4); first fall degree absent in 87/88 (regular-sequence behaviour, Macaulay bound 3·3+1). n=2: degree 3, 6 × 10, ≈ 87 mults. **Flat in p.** | Prime-field PKM degree flat in p (`research/ic_next10_20261014/03-pkm-degree`), m=4 degree ≥ 7–8 out of reach | Prediction confirmed: cost per decomposition is a p-independent constant c(n); the question is its size, not its growth |
| 5 | Zero-dimensionality, geometry | Zero-dimensional 176/176; quotient dimension **64 = Bezout** (87/88; one p=31 instance 60) and 4 for n=2. Enumeration (n=3, p ≤ 127, 24 targets): **32 F_p-rational solutions, 5 genuine (= the 5 decompositions found independently by point arithmetic), 27 non-split (84 % spurious)**, 2 singular, 0 positive-dimensional lines, 0 split-but-off-curve | Gaudry residual quotient 61 vs 64 (`RESEARCH_RESIDUAL_WALKS.md` §11.8); char-2 positive-dimensional {t, t+1} families | Geometry is benign; ~5 solver outputs per genuine relation must be rejected (cheap cubic splitting test) |
| 3 | Yield mean and variance | 20 curves × 2000/6000 targets: mean / λ_pred = **0.88–1.19** (λ = F³/6N); P(0) = e^{-mean} to 3 decimals; dispersion index CI contains 1 in 19/20 curves. Two heavy tails (max 25, 120 decompositions) — the replayed one is **x_R ∈ F_p**, all 25 of the form R + P − P (trivial family, probability ≈ 1/p² per target) | Odd-char coverage Poisson (`examples/decomposition_coverage.rs`); dispersion ≈ 6 for prime-field m=3 | Poisson; no heavy tail on non-degenerate targets. Collectors should drop subfield targets |
| 4 | Relation rank | 15/15 complete solves: **log Q correct** (kG = Q checked). Rank shortfall versus (unknowns − empty columns): 0 on 7 curves, 1–3 on 7, 6 on one (p=61 curve 1; hypothesis, not replayed: duplicate rows from a target that is itself a factor-base point). Controls: cofactor scaling by h ∈ {2..12} gives the same k 15/15; 10 duplicated rows leave rank unchanged 15/15; dropped signs detected 12/15 (missed when every column used carries one sign, so the bug is a relabelling) | Koblitz free relations rank 1–49/599 (structural); shared-rank transfer failed 80/80 | Not a blocker: gcd(h, ℓ)=1 makes the cofactor harmless; margin of +20 rows suffices for k even when rank < full |
| 6 | End-to-end cost vs matched rho | Complete toy solves, ℓ = 2^12–2^25, §5a band (Groebner and LA native ops priced w; rho comparisons priced w). IC / rho (analytic negation map √(πℓ/4)): **w=0: 0.29–3.4; w=0.1: 2.8 × 10^4 – 3.2 × 10^5; w=1: 1.5 × 10^5 – 1.7 × 10^6**. Verdict per curve: rho ahead 7, unresolved 8, IC ahead 0 | No table A–C row below rho; best prime 7.75–9.35× | Distance remaining at toy size: 4–5 orders of magnitude at w ≥ 0.1 |
| 2 | Non-generic filter | Not addressable on this family (subfield base is the algebraic structure, F_p prime field untouched) | Every tested F_p filter negative; RT-REPRESENTATION-FILTER defined, never run | Untested here; remains the open prime-field question |
| + | Factor-base size and large primes | One-large-prime yield / exact hypergeometric expectation: Y 1.04, 1.01, 0.93; Y2 0.99, 1.04, 1.04 (φ = ½, ¼, ⅛, p ≥ 251). Full relations: Y 0.85 [0.73, 0.97], Y2 1.08 [1.02, 1.15] at φ = ½; pooled 1.04, 1.00, 0.89 | LP savings ≤ 0.56 % in JV cover; DL/B0 0.04–0.08 in GHS-weak class | φ³ and 3φ²(1−φ) laws hold; Amendment 1 rule: the Y deficit was not reproduced (opposite sign), recorded as noise; Poisson CIs are too narrow because subfield targets cluster |
| + | Pricing band | Every comparison above is ranked only where the order holds at w = 0, 0.1, 1 | 7 lab rows flip inside the band | 8/15 curves unresolved purely because w = 0 makes Groebner free |

## Derived (model, not measurement)

With the measured constants τ = 3.50 trials per p, c(3) = 1.06 × 10^6, 3.97 nonzeros per row, and no
large primes, the best factor-base fraction gives IC / rho → **684 (w = 0.1)** and **3761 (w = 1)**,
constant for groups ≥ 2^60 (`data/summary.jsonl`, `"summary":"model"`). Reason: without large primes
n = 3 balances at p^{3/2}, the same exponent as rho, so the gap is a constant ∝ √(w · c(3)). Closing it
needs both the double-large-prime exponent (measured below: 1.27 ± 0.04 at r = 2/3) and a much cheaper
decomposition than a dense Macaulay solve.

## Large-prime variation (arm L, Amendments 2, 3, 3a)

n = 3, p ∈ {127, 251, 509, 1021, 2039}, 2 curves per p (groups 2^20.0–2^32.9), one walk stream per
curve feeding 0LP, 1LP and 2LP eliminators at |FB_r| ≈ p^r/2, r ∈ {1, 5/6, 3/4, 2/3, 1/2}; trial cap 40p.
T_square = trials until the full rows first reach |FB_r| + 1 (Amendment 3: the fixed +20 margin of run 1
biased every exponent low, e.g. 1.12 instead of 1.27 for 2LP at r = 2/3). Native fits on curve
geometric means, ± 1 standard error ([`data/summary_dlp.jsonl`](data/summary_dlp.jsonl), `"metric":"T_square"`):

| r | 0LP α (law 3 − 2r) | 1LP α | 2LP α | 2LP row weight (p = 2039) |
|---|---|---|---|---|
| 1 | 1.02 ± 0.03 (1) | = 0LP | = 0LP | 4.0 |
| 5/6 | 1.49 ± 0.12 (1.33; p ≤ 1021) | 1.21 ± 0.03 | 1.11 ± 0.03 | 7.8 |
| 3/4 | censored (1.5) | 1.30 ± 0.08 | 1.16 ± 0.01 | 10.4 |
| 2/3 | censored (1.67) | 1.47 ± 0.11 (p ≤ 509) | **1.27 ± 0.04** | 7.8 |
| 1/2 | censored (2) | censored | 1.35 ± 0.11 | 6.2 |

- **(a)** 0LP follows 3 − 2r where it is measurable (r = 1, 5/6, within 1.2 se).
- **(b)** 2LP at r = 2/3: 1.27 ± 0.04, prediction ≤ 1.4 holds, falsifier (> 1.5) not triggered; consistent
  with the theoretical 4/3 (1.4 se) and below rho's 1.5 by 5 se. At 2039, 2LP needs 19 723 trials versus
  1LP > 62 876 (censored) and 0LP > 81 560 (censored).
- **(c)** log Q correct in **105/105** reached cells (45 censored, none wrong or undetermined).
- **(d)** fails narrowly in 1 of 25 2LP cells: mean merged row weight is up to 12.07 at p = 1021, r = 2/3,
  3.02× the 0LP weight of 4.0; every other cell is ≤ 2.6× (≤ 10.4). Row weight does not grow monotonically
  in p (7.8 at p = 2039, r = 2/3), so the linear-algebra price of 2LP is a constant factor here, not a trend.
- Replay: the Amendment-3a rerun reproduces every run-1 T, reached, trial, rank, consistency and k field
  (150/150 records); only `la_dense_mults` (43) and `merge_ops` (2) differ, from run 1's non-deterministic
  decomposition order (fixed; two runs are now byte-identical).

**Derived (extrapolation, not measurement).** Plugging the fitted laws, c(3) = 1.06 × 10^6 and the
measured row weights into IC/rho (negation map) at the best measured r: at w = 0.1, 2LP reaches 58 at
2^96, 7.7 at 2^132 and crosses 1 at about 2^180; at w = 1 it is 319, 42 and 5.3 at the same sizes and
does not cross by 2^240. The §5a band therefore does **not** rank IC ahead anywhere up to 2^240 (the
order flips between w = 0.1 and 1 above 2^180). The fits come from p ≤ 2^11 and are extended by up to
70 bits; the 2LP slopes at r = 3/4 and 5/6 (1.11–1.16) are below the asymptotic value and probably
pre-asymptotic, so the crossover is optimistic. The 0LP curve is limited by the r grid (its optimum
r = 3/4 is censored), so the earlier 0LP plateau (684 / 3761, continuous φ) remains the 0LP reference;
both models agree at 2^60 (683 / 3750).

## What this changes in the ranking

1. Row 1 is confirmed as the dominant cost but it is a **constant**, not growth: at fixed n the solving
   degree is flat from 2^5 to 2^30. The lever is c(n) itself (F4/structured elimination, or avoiding
   Groebner entirely), and its square root sets the plateau gap.
2. Rows 3, 4 and 5 are **cleared at this scope**: Poisson yield, benign geometry, full-enough rank with
   correct logs on every curve. Their failure modes (subfield targets, duplicate rows, spurious
   solutions) are cheap filters.
3. The missing row (factor-base size / large primes) is **required**, not optional: without 2LP the
   exponent equals rho's; with 2LP it is measured at 1.27 ± 0.04 (theory 4/3), but the constant
   √(w · c(3)) keeps IC behind rho at w = 1 to at least 2^240 in the extrapolated model.
4. Row 2 is untouched by this family and remains the only route for prime fields.

## Reproduce

```sh
cd research/ic_weakpoints_20261102/tool && cargo build --release && ./target/release/ic-weakpoints selftest
cd .. && ./run_all.sh                       # frozen arms A2, A3, Y, P -> data/*.jsonl
tool/target/release/ic-weakpoints yield --p 251 --curves 4 --targets 6000 --seed 20261103   # Y2 (and p=509)
gunzip -k data/*.jsonl.gz    # records over 64 KiB are committed gzipped (artifact-store policy)
tool/target/release/ic-weakpoints summarize data > data/summary.jsonl
./run_dlp.sh                                # arm L (about 65 min on 4 vCPUs)
tool/target/release/ic-weakpoints summarize-dlp data > data/summary_dlp.jsonl
```
Graphs: `reports/run-20261102T1639Z/plots.py` lays out the summary JSONL only (no statistics computed in Python).
