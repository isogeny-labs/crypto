# L13: the C1/C2 degree-4 plateau and the 1488 deficit (2026-11-02)

Class: accounting. This explains a measured stall of a stand-in arm. No security claim, and nothing here says anything about Semaev arm A, d_F growth, or n=131.
Numbers come from `raw/main.jsonl` and `raw/model_p1.jsonl` through `src/analyze.py`, which writes `RESULTS.json`. The predictions are in `PROTOCOL.md`, written before any run.

## Model
In C1, each of the n equations has the form g_r = λ_r μ_r + ℓ_r (rank-n structure, L1). Its top-degree ideal per block is I_r = (λμ, λℓ, μℓ). The algebra is F2[x]/(x_i^2), and in characteristic 2 every linear form squares to 0 there.
- **Degree ≤3:** each block contributes 3 + 3(N-3) + 1 = 3N-5 dimensions. Two blocks cannot meet below degree 4. So the plateau is **low = n(3N-5)** (2821 at n=31, 3201 at n=33).
- **Degree 4:** each block contributes D(N) = 3C(N-3,2) + (N-3) dimensions. The only overlaps are the pairwise Koszul products I_r·I_s, which are 9-dimensional per pair. That gives
  **R4 = n·D(N) − 9·C(n,2)**.
  The closure stalls, meaning no degree-4 fall is forced, exactly when R4 < C(N,4).
- **Deficit with N=n+1:** C(N,4) − R4 = n(n³ − 38n² + 263n − 274)/24.
  - Its largest real root is 29.36, so n=31 is the first odd n where the closure stalls.
  - n=31: deficit 31·1152/24 = **31·48 = 1488**.
  - n=33: 4070. n=35: 7665. n=37: 12469.
  - n=29 (surplus 261) and n=27 (surplus 1341) do not stall.
- **Size argument:** R4 grows like 1.5n³ and C(N,4) like n⁴/24, so the stall persists for all larger n on this diagonal.

## Validation (P2/P3: 17/17 closure runs exact)
Each run is labelled by n, N=2k, arm, the engine used, and whether its draw was SAT or UNSAT. Draws with solutions can only lower dim and never raise low, so a SAT draw is a valid test of the stall prediction.

| case | predicted (low, dim) | measured |
|---|---|---|
| n35 N36 C1 s0, s1; C2 s0 (gf2close) | stall (3605, 54845) | identical on all 3 |
| n37 N38 C1 s0 (gf2close) | stall (4033, 65379) | identical |
| n31 **k=17** N34 C1 s0, s1 (off-diagonal) | stall (3007, 43028) | identical |
| n29 **k=16** N32 C1 s0 | stall (2639, 35148) | identical |
| n33 **k=16** N32 C1 s0, s1; C2 s0 | R4 = 36399 > 35960, no stall | 1 ∈ W_4 on all 3 |
| synthetic rank-n triples (generator-independent), N=n+1, macclose: n31 (2 seeds), n33 | stall (2821, 37293), (3201, 45507) | identical, deg4 34472 / 42306 |
| synthetic n27, n29; C1 n25, n27 (macclose) | escape | escape (low > plateau) |

The synthetic arm replaces field multiplication with random linear forms and still reproduces the plateau exactly. The plateau is therefore a property of the rank-n λμ+ℓ shape, not of the field or of the generator.

Consistent with numbers already on record (seen before the model, so these are not predictions): L1 n31 C1 (2821, 37293), and L8 n33 C1/C2 on 8 draws (3201, 45507).

## P4 (exploratory)
Prediction: the new low elements in round 3 equal the surplus R4 − C(N,4).
- **Holds** at n=27 (1341, for both C1 and the synthetic arm) and at n=29 (261, for C1 and the synthetic arm).
- **Fails** at n=25: the surplus is 1900 but only 1123 falls appear. The degree-≤3 space is already full there (2948 of 2952 columns, with 4 solutions), so the number of falls is capped.

## P1 (pure linear-algebra model): partly falsified
I ran 11 random (n,N) pairs, with exact GF(2) ranks of the degree-3 and degree-4 ideal parts.
- **Degree 3:** n(3N-5) is exact on all 11.
- **Degree 4:** exact for N ≥ 18 (4 of 4 pairs, including ones at the threshold: (11,18), (12,20), (13,20)) and at (6,12). Below that it is wrong:
  - At N ≤ 16 the measured rank is lower than predicted by 4 to 40 (for example (9,16): 1780 against a predicted min(1899, 1820)).
  - So there are extra degree-4 syzygies beyond the pairwise Koszul ones at small N. These are not identified.
- The formula is therefore validated empirically for 26 ≤ N ≤ 38 and for N=18 and 20. It is not a theorem: genericity (no higher syzygies in degree 4) is assumed and was not proved.

## Out of scope / open
- **D1** (101 quadrics at n=31, low4 2821–2839) does not have the 3n structure, so the model does not cover it.
- **A** falls below the plateau by exactly 2·(4n) in degree 4, because of the 4n Frobenius falls (L9). Those falls are not modelled here.
- **Small-N extra syzygies** are unexplained.

## Run details
- Engines reused unchanged: N1 `gf2close` via `lib.call`, and the L8 `macclose` binary (the same source as L1). Generator: N1 `lib.system`. New code: `src/model.py`, `src/run.py`, `src/analyze.py`.
- At most 3 concurrent processes. About 1000 CPU-s in total. Output is 40 KB.
- `conductor check` was not run (BRIEF §0.6 says it is already done for T-5736).
