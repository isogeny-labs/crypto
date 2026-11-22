# Protocol: lower t, G = 4, and the indicative distance to matched rho (frozen before the new data)

2026-11-15. Follows PRs #4213, #4233 and the scaling study (`curve25519_isogeny_scaling_20261115`). All results [pcc:unverified].
That study found t = 6 costs 17.3x more F4 mul-adds per target for 3.82x more hits per target (rate 0.22x of t = 5). The converse
(t = 4 and t = 3) was not tested, although the atlas (`curve25519_ic_atlas_20261104`, section 6) already uses t = 4 for its best F4 arm. Same engine
and binary as before (sha256 `ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`), k = 20 miniature (p = 1048573, A = 926,
#E = 1049624), m = 2, mode full, target seed 2026111403, blind arms (`lifts=0`):
`curve25519_family f4cost 20 isogeny@SEED T G TARGETS 2026111403 8000 2 full 0 30000000 0 0`.

## Matched rho
The matched rho-neg arm on this exact curve is already measured by the lab (ecbench, replay-audited): S = 1.544 / 1.654 / 2.651 at w = 0 / 0.1 / 1
(T12, 16 one-target chunks) and 1.447 / 1.547 / 2.448 (T5, 6 targets), `research/curve25519_ic_atlas_20261104/REPORT.md` section 6; S is counted GAE
divided by sqrt(r) = 362.2. A fresh arm was not run: this session's AWS identity is invalid (`aws sts get-caller-identity`:
InvalidClientTokenId), and the GitHub dispatch route builds and submits on `main` with an S3-receipt return this session cannot read.
The Part C comparison uses T12 as the matched reference, as a descriptive comparison, not an ecbench pair.

## Cells
- t in {3, 4}, fresh candidate seeds 201 to 240 at each t (40 each; none used before). Tower dumps give c(s) = on-curve V x-values (method of PR #4233).
- Per t: the 4 seeds with the largest c (top) and the 4 with the smallest c (bottom), ties to the smaller seed: 8 seeds, chosen and committed before any engine run.
- Each of the 8 seeds per t at G = 3 and G = 4: 32 blind arms. Targets per arm: 1,000,000 at t = 4 and 4,000,000 at t = 3 (the cost per target is far
  smaller than at t = 5, so the target count is raised to make hits countable). A pilot of 2,000 targets on one seed per t is allowed only to size wall time.
  If an arm would exceed 4 hours the target count is lowered for all arms of that t and recorded as an amendment before they run.

## Hypotheses (directional, fixed now)
- H29 (pairs law at lower t): at G = 3, per t, the 8 arms' pooled hits lie inside the exact Poisson 95% interval of sum over arms of
  TARGETS x b(b-1) / (2 x 1049624).
- H30 (lower t pays): t = 4, G = 3, pooled over the 8 seeds: hits per 1e9 mul-adds have a Poisson 95% lower bound above 3.334
  (the upper end of the unscreened t = 5, G = 3 interval of PR #4213).
- H31 (t = 3 beats t = 4): t = 3 G = 3 pooled rate exceeds t = 4 G = 3 pooled rate, one-sided exact conditional binomial p < 0.05.
- H32 (G = 4 helps): t = 4, pooled over 8 seeds: G = 4 rate exceeds G = 3 rate, one-sided exact conditional binomial p < 0.05.
- H33 (screening survives at t = 4): t = 4, G = 3: top 4 pooled rate exceeds bottom 4 pooled rate, one-sided exact conditional binomial p < 0.05.
- H34 (indicative distance below the atlas's best): for the best (t, G) cell by pooled rate, the indicative gap to T12's rho-neg at w = 0.1 is below 1.83e4 (the atlas's best F4 arm, seed 1193 t = 4 G = 3 with trace replay, T13).
  The indicative gap is defined here: S_IC(w) = [tower group ops (search+count+build) + w x (columns + 1) x (F4 mul-adds per hit)] / 362.2,
  gap = S_IC(w) / S_rho(w); it counts native work at w GAE per mul-add and charges only the tower as counted GAE; it omits linear algebra, the
  unpriced Legendre/sqrt/Velu native work and any replay; it is NOT an ecbench-measured S and is not a leaderboard figure.
Decision: supported only if the stated bound holds; otherwise not supported and reported as such. Six hypotheses, no correction.

## Descriptive
Per-arm table; for every (t, G) cell: pooled rate [95% interval], mul-adds per target, columns, mul-adds per hit, indicative S and gap at w = 0, 0.1, 1;
pairs-law ratio observed/expected.

## Out of scope
lifts arms, t = 5 or 6, k other than 20, seeds chosen after the fact, a rho run (see Matched rho), ecbench replay audits, a Conductor task, isolab and S3
checkpoints (counted mul-adds are deterministic; AWS identity is invalid here). Not a leaderboard or `benchmarks/` row.

## Amendment A1 (before any engine arm; pilots discarded, they had 0 hits)
The engine requires guess G < t ("pkm-semaev: guess must be below the tower height"), so G = 4 cannot run at t = 4 or t = 3. Cells become:
t = 4 at G = 3 (primary) and G = 2; t = 3 at G = 2 (primary; G = 3 is invalid). H29, H30, H33 use t = 4 G = 3; H31 compares t = 3 G = 2 with
t = 4 G = 3. H32 becomes: t = 4, pooled over the 8 seeds, G = 3 rate exceeds G = 2 rate, one-sided exact conditional binomial p < 0.05.
H29 at t = 3 uses G = 2. H34 uses the best cell over all valid cells. A G = 4 test needs t >= 5 and is out of scope here.

## Amendment A2 (before any engine arm; wall time)
The pilot measured about 15 ms of wall time per target at t = 4 (17,592 mul-adds per target, 29.4 s for 2,000 targets), so 1,000,000 targets would take about
4 hours per arm. Targets per arm are lowered to 100,000 at t = 4 (G = 3 and G = 2) and 200,000 at t = 3 (G = 2 only); the expected hit counts from the
pairs law are recorded in the analysis script. Arm count: 16 at t = 4, 8 at t = 3 = 24. Hit counts in the bottom groups will be small (b = 2 to 10);
that limits the power of H33 and H31 and is stated here rather than discovered later.
