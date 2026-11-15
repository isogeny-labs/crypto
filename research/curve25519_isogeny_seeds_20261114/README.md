# Isogeny base across seeds 1 to 20 in the F4 engine

All claims here are `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md), committed before data.
Toy: k=20 Curve25519 miniature (p = 1048573, #E = 8·131203), t=5, G=3, m=2, 20,000 targets, target seed 2026111403,
counted F4 mul-adds (deterministic). Seed 0 comes from PR #4157 (same binary and targets). Per-seed table: [summary.txt](summary.txt).

## Result (all four pre-registered hypotheses supported)

| | outcome |
|---|---|
| H6 lead not a seed-0 artifact | pooled blind rate over seeds 1-20: 206 hits / 70.8e9 mul-adds = **2.909 per 1e9 [2.525, 3.334]**; lower bound above 1.515 (digit set 1 upper end). Supported. |
| H7 lifts neutral | pooled lifts 1620 hits / 565.5e9 = 2.864 [2.727, 3.007]; rate ratio 0.985, exact p = 0.82. Supported. |
| H8 cost shape | blind mul-adds per target 1.7702e5 to 1.7710e5, spread 0.05%. Supported. |
| H9 tower setup negligible | tower group ops 492 to 6,823 per seed; worst ratio to blind mul-adds 1.9e-6 (seed 14), threshold 1e-3. Supported at one op per mul-add only; the 0 to 1 GAE band is not tested. |

## Descriptive findings

- The seed matters for the rate. Per-seed blind rate ranges 0.85 to 4.80 per 1e9 (about 5.7x); seed 0 (3.39) is mid-to-upper, not an outlier. Lifts arm ranges 0.88 to 4.75.
- Blind counts are mildly overdispersed against a common-rate Poisson model (chi2 = 32.5, 19 df, approx. p = 0.03, Wilson-Hilferty approximation); lifts counts are strongly overdispersed (chi2 = 217.8). So the seed is a real covariate, mostly visible in the larger lifts sample. Spearman correlation of per-seed blind vs lifts rates: 0.72.
- Cost per target is fixed by the engine; the rate differences come from hits only. Seed choice is therefore a free lever worth up to a few x in hits per mul-add on this toy. Why a seed is good (coset coverage of the auxiliary V set?) is not tested here.
- Tower search cost is negligible against F4 work on this toy at unit weight. It grows with the group size, so this is a statement about k=20 only.

## Caveats

Single k, t, G and one target set. Seeds 1-20 are the first twenty, not selected after the fact. The chi-square p is an approximation. No rho arm, no 0/0.1/1 GAE pricing. Not a leaderboard row: no validation-gate evidence.

Files: `runs/` (engine JSON per seed and lifts), `tower_cost.jsonl`, `run_seed.sh`, `summarise_seeds.py`.
