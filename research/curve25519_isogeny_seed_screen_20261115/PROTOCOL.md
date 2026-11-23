# Protocol: does screening isogeny seeds by on-curve count pay? (frozen before the candidate dumps and engine runs)

2026-11-15. Tests the untested prediction of `curve25519_isogeny_seed_cause_20261115` (PR #4233): the F4 hit rate of an isogeny
seed follows the number of V x-values on the curve, which is known from the tower before any F4 work. All results [pcc:unverified].
Same instrument and conditions as PR #4213: `curve25519_family f4cost 20 isogeny@SEED 5 3 20000 2026111403 8000 2 full 0 30000000 0 0`
(k = 20 miniature, p = 1048573, A = 926, t = 5, G = 3, m = 2, 20,000 targets, target seed 2026111403, blind, `lifts=0`), binary sha256
`ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`.

## Design
- Candidates: fresh seeds 21 to 60 (none used before). For each, run the tower (`isogeny_tower`, p, t = 5) and count the V x-values
  whose Montgomery x gives a point on the curve, c(s), exactly as `analyse.py` of PR #4233 does. No engine run before this step.
- Screen: top group = the 8 seeds with the largest c, bottom group = the 8 with the smallest c; ties broken by the smaller seed.
- Arms: the 16 blind engine arms of the two groups. Unscreened reference = pooled blind arms of seeds 1 to 20 from PR #4213 (206 hits,
  70.8e9 mul-adds, 2.909 per 1e9 [2.525, 3.334]).
- Fitted model, fixed now from seeds 0 to 20 blind only: 0.0913 hits per 1e9 mul-adds per base point (base_points = 2c).

## Hypotheses (directional, fixed now)
- H13 (screening separates): top-group pooled hits per 1e9 mul-adds exceed bottom-group's, one-sided exact conditional binomial p < 0.05.
- H14 (screening beats unscreened): top-group rate has its Poisson 95% lower bound above 2.909.
- H15 (the model transfers): for each group, observed hits lie inside the exact Poisson 95% interval of 0.0913 x sum over arms of
  (mul-adds in 1e9) x base_points.
- H16 (screening is cheap): total tower group operations (search + count + build) over all 40 candidates are below 1e-3 of the
  top group's total counted F4 mul-adds, at one operation per mul-add (not the 0 to 1 GAE band; not claimed there).
Decision: supported only if the stated bound holds; otherwise not supported, reported as such. Four hypotheses, no correction.

## Descriptive
Per-seed table, Spearman of c vs arm rate over the 16 arms, bottom-group rate against the unscreened rate, the ratio top/bottom.

## Out of scope
lifts=1 arms, other t, k, G, a rho arm, a claim about cost at the 0 to 1 GAE band. A screened-in seed is a seed choice made on the
toy; it says nothing about whether the choice generalises to other primes. No Conductor task, isolab run or S3 checkpoint (counted
mul-adds are deterministic; the AWS route is absent here).
