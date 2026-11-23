# Red Team: EXP-PFDR-ada71b (reading O-SUB-BE)

Verdict: the arithmetic is sound and reproducible, but "SUB-BE" is weaker than it sounds: the six lines are indistinguishable from random bases (kappa = 1), and the data do not bound any excess exponent. V = verified by computation, R = reasoned only, M = missing.

## 0. Provenance (M)
- The records sit in a scratchpad `rehearsal/` tree, not in the repo (`runs/` holds only `.gitkeep`). Manifests show `commit: null, dirty: null`; there is no execution-receipt.json and no check log; the route is not isolab-eks. Everything below is conditional on the committed archive having these bytes: main-jobs 2bbd33…, analysis 70b37f…, design 6083c7….
- V: spec and code shas match trial-plan.json. The census is complete (20151 ok, 0 failed, c = 0..n-1 per cell): no curve selection. Pilot sizing re-derives.
- V: check.py `analyze-final` on a scratch copy prints CHECK OK, but it reads the bootstrap "as recorded". My numpy bootstrap reproduces all six slopes; CI endpoints agree within 5e-4 and the classes are identical.

## 1. What the analysis supports (V)
- delta_hat lies in [-0.0044, +0.0070]; the largest upper95 is 0.0144, 23x below 1/3.
- Per-rung kappa lies in [0.916, 1.058]. 28 of 30 CIs contain 1; the other two are deficits. Pooled kappa is 0.991 (z = -1.4): no excess anywhere.
- The small positive slopes come from kappa about 0.97 at the 16- and 20-bit rungs; A/A pooled kappa scatters as much (0.983–1.000). Noise, not growth.

## 2. Findings, by severity

**F1 (unsupported): the class tests the elasticity of kappa, not the exponent of the excess.**
- The slope of log2 kappa on log N equals the excess exponent only when kappa-1 >> 1. Break-even needs kappa > 1.27 c N^(1/3) (H-PFDR-ed7ccf).
- V: I injected Poisson excess eps = a N^delta into the actual null-replicate records, with eps(32 bits) from 0.05 to 0.8 and delta = 1/3, 1/2, 0.2. Every case classified SUB-BE or SUB-BE-GROWING; none was ALIVE.
- V: ALIVE (slope >= 0.32) needs eps(32) ≈ 230 at delta = 1/3, ≈ 37 at 1/2.
- So an excess exactly on the break-even exponent is unexcluded while eps(32) <~ 0.06–0.1, and then eps(2^64) ≈ 108–180. The data bound only amplitude: upper95 kappa is at most 1.14 per rung, 1.11 at 32 bits.

**F2 (unsupported scope): the controls validate a different regime.**
- Plants (C3) and Z/N (C4) start at kappa(16) = 1.3 and are pure power laws in kappa; they say nothing for kappa-1 < 0.1. C3's design slope reuses the plant_internal(L) that builds the plant, so recovery is partly bookkeeping.
- R: window and GAP labels are x-coordinate lattices; x -> x+w is not a morphism of E(F_p), so kappa = 1 is the a-priori expectation (near a null-vs-null identity). The GAP also lifts only about half its x values, so the lattice is randomly thinned (unpriced).
- Word any "exhausted_at_scale" as "x-lattice labels carry no pair-sum excess".

**F3 (unpriced / unsupported):**
- The "h = 3, 4 thresholds 1/5, 1/7" compare an h = 2 slope with h >= 3 thresholds; no 3-sum or 4-sum collision was counted.
- No distance remaining is reported. The level gap kappa / (1.27 c N^(1/3)) is about 1/(51c) at 16 bits and 1/(2000c) at 32 bits, so the gap grows about 40x. c and U are unpriced; no §5a band.
- kappa counts trivial relations: Z/N has kappa ~ N^0.65–0.7 yet yields no discrete log.

**F4 (minor):**
- C6 calibrates z, which the decisive rule does not use; its 0.70 sd floor is permissive (observed 0.90).
- C5 has 6 A/A lines. V: with the 20 null replicates times 2 theta, slope sd was 0.0053 against bootstrap sd 0.0045 at theta .30 (1.16x, within noise); CI coverage of 0 was 90% and 100%. Theta .35 rungs 28 and 32 have only 10 curves; no under-coverage appeared.
- The six lines share curves and null M, so they are not six independent confirmations. Multiplicity cannot create false support here.
- H-PFDR-ed7ccf.yaml still cites the v1 seed blocks 6,000,000 and 6,200,000; v2 uses 12,000,000+.

## 3. What the design could and could not detect
- Bootstrap sd of delta_hat is 0.0037–0.0046 (95% half-width 0.0073–0.0090). The smallest slope with 80% power is about 0.010–0.013 (kappa(32)/kappa(16) about 1.15), assuming a pure power law (V).
- **Untouched:** FAM-1 small_x, subgroup and dickson arms (R2); log-GAPs; the bbc53e weighted base; j = 0 and j = 1728 curves (the generator rejects a = 0 or b = 0, though `generate_prime_order_curve_j0` is vendored); theta outside {0.30, 0.35} (b <= 2352); rungs above 2^32; h > 2; any base with a real mechanism.
- Other artifacts (R): the x-axis shift from within-rung N spread is <= 0.2 bit and Hasse fluctuation <= 0.8%; both are immaterial for flat kappa.
- Negation symmetry: null var/mean is 5.4–6.1 (V), since each relation recurs 4–8 times. M >= 2500 is about 500 independent events; kappa SE is 4–6%, which the bootstrap captures.

## 4. Cheapest threatening measurement
- Zero new compute: add a "1 + a N^delta" plant (delta = 1/3, 1/2; eps(32) in {0.1, 0.4, 0.8}) to the existing records, plus an amplitude criterion (upper95 of kappa(32)-1) and the level gap. If the class cannot read ALIVE, restate the result as "no detectable excess, kappa(32) <= 1.11".
- New compute: one mechanism-bearing geometry (FAM-1 small_x, or j = 0 with omega-orbit bases and 3-sums) at the same five rungs.
