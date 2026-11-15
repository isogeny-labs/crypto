# PROTOCOL: KM3-SPEC-LADDER (lab B, wave 2, 2026-11-15), frozen before data [pcc:unverified]

Lead record: research/kummer_m3_ns_degree_20261115/NEW_LEADS.md, id KM3-SPEC-LADDER.
Engine: examples/pkm_tower_pilot.rs `--engine hky` (closure V_{F,c}); added flags `--target-xr V`
(override x_R of random targets) and `--spec-ladder` (after each random m = 3 target, measure the
m = 2 system with target rho for every distinct F_p-root rho of S3(u, x3, x_R), x3 in the base).
Everything below is an unverified assertion [pcc:unverified]; no pcc claim id exists.

## Predictions (from the lead, copied)
- P-Lemma: d_F(3,t; x_R) >= d_F(2,t; rho) for 100% of (m = 3 draw, rho) pairs, comparing exact values;
  a censored value (d_F > cmax) on the m = 2 side with a finite m = 3 value is a violation; a censored
  m = 3 value (d_F > cmax3) makes every pair with spec value <= cmax3 pass vacuously.
- H_inherit: d_F(3,4) = 6 and d_F(3,5) = 6 on both draws; gap Delta(t) = d_F(3,t) - max_rho d_F(2,t; rho) = 2
  (m = 2 value read as 4 at t <= 5).
- d_F(3,6) = 7 if the cell completes. Declared intervals: d_F(3,4) in [6,7], d_F(3,5) in [6,8].

## Kill / survive rules
- Lemma P is falsified (or an engine defect shown) if any pair has d_F(3,t) < d_F(2,t;rho), both exact.
  Then stop and debug before any degree is read.
- H_inherit is falsified if d_F(3,4) != 6 or d_F(3,5) != 6 on either draw (>= 7 means growth of its own at m = 3).
- Delta is reported as measured, whatever its value; no pass/fail on Delta beyond H_inherit.

## Instances
- Primary: p = P1 = 2013265921, Kummer, seed 0x504B4D54 (lead's), --planted 0 --random 2 (2 draws per cell),
  m = 3: t = 4, 5 (--hky-cmax 8), t = 6 (--hky-cmax 7, --hky-max-entries 2.5e8, censor-capped, only if budget allows).
  m = 2 spec arm: --hky-cmax 7 at every t.
- Lemma P across primes: p = 40961 and p = 12289, Kummer, seed 0x504B4D54, 2 draws, m = 3, t = 2, 3, 4
  (draws with a consistent grid point are decomposable, reported and excluded).
- P1 also at t = 2, 3 (cheap) for the full ladder.

## Controls
- C+ (positive, instrument sees the effect): re-run the committed development rows
  research/prime_ic_leads_20261110/devcheck/m2_t2-10.jsonl (m = 2, t = 2..5) and m3_t2-3.jsonl (m = 3, t = 2, 3)
  with the commands in devcheck/commands.log (seed 1347106132 = 0x504B3554 passed explicitly); require identical
  d_F, ffd, x_R and muladds. This also certifies that the added flags do not move any draw (flags off).
- C+ (flag): `--target-xr` with the x_R of a committed draw reproduces that draw's d_F and muladds.
- C- (negative): (i) the reversed inequality d_F(3,t) <= d_F(2,t;rho) must be flagged by the checker on the
  cells where d_F(3) > d_F(2;rho) (shows the comparison can fail); (ii) planted (consistent) m = 3 target:
  closure never contains 1 (kill if it does); (iii) a rho that is not a root of S3(u,x3,x_R) (rho+1) is used
  as an out-of-hypothesis arm and reported separately, not counted.
- A/A: the two draws per cell are the replicate; identical re-run of one cell reproduces muladds exactly.

## Resources
1 core, OMP_NUM_THREADS=1, cargo -j1, <= 3.5 GB, <= 2.5 h. Logs under scratchpad/wave2/KM3-SPEC-LADDER/.
