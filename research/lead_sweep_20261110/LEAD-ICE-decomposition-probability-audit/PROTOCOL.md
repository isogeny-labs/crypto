# LEAD-ICE-decomposition-probability-audit

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-30; 2026-11-02 in the survey). Family: ic-engine.
- Title: Decomposition probability P(decomp) vs factor-base size, charged p_solve, and learned decomposability predictors.
- Next-step excerpt (truncated): "Run BATCH-79a580 (three F-2 28-bit fixtures with P-5 yields, nine F-1 fixtures as control) for E..." Cost: cheap-medium (BATCH-79a580).
- Why it might help: "Target-conditional variance beyond random-base null; mean fixed by conservation". Survey: learned predictors (GNN, SR) inconclusive x3 at N<=2^20.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 45, 110, 157, 221 only). BATCH-79a580 fixtures were not found here; research/ecdlp_subexp_desk_20261101/data/atlas_tally.json matched but was not read.

## 2. Hypothesis
On a cyclic prime-field elliptic-curve group of order n (2^12..2^16), the target-conditional variance of the 2-point decomposition count r(T) = #{(a,b) in F x F : a+b = T} for a size-matched, negation-closed "small x-coordinate" factor base F is not larger than for a random negation-closed factor base of the same size. Concretely, its index of dispersion D = Var_T(r)/Mean_T(r) lies inside the random-base null band. Mean r is fixed by conservation (sum r = |F|^2) and is identical for both arms.

## 3. Preregistered prediction and falsification criterion
- Statistic: per curve z = (D_xsmall - mean D_null)/sd D_null, 24 random-base null draws per curve; pooled by Stouffer z over 12 curves.
- Prediction: |Stouffer z| < 3 and every per-curve |z| < 4: consistent_with_prediction. Otherwise falsified (the x-small base shows excess or deficit target-conditional variance).
- Negative control / null: random negation-closed subsets of the group, same size.
- Positive control: the interval base {+-1..+-m/2} (an arithmetic progression, strongly clustered sums). Every curve must give z > 5, or the detector is blind and the verdict is inconclusive.
- A/A arm: two further independent random-base draws, z_AA = (D1-D2)/(sd sqrt2). Stouffer |z_AA| must be < 3, or the verdict is inconclusive.
- A falsified result decides only that the toy shows structure in x-small bases. An in-band result decides only that this toy sees none.

## 4. Method
- Python 3 stdlib only. For each log2 size in {12,14,16} (p = first prime >= 2^k with p = 3 mod 4), 4 curves y^2=x^3+ax+b with random a,b and a random point G, from seeds 20261110 + 1000*size_index + curve_index. Keep curves where ord(G) > 0.6p.
- Enumerate kG for k=1..n-1 by affine addition, so each point is labelled by its discrete log. Sums of points are then sums of logs mod n.
- Factor-base size m = 2*floor(2*sqrt(n)) (|F| ~ 4 sqrt(n), mean r ~ 16).
- x-small base: the m/2 points (one per +- pair) with smallest x, plus negatives. Random base: m/2 uniform distinct logs plus negatives. Null seed = 7*curve_seed+1.
- Compute r(T) for all T over the F x F pairs, then D.
- Cost estimate: about 3 min wall, under 100 MB RAM (largest n ~ 2^16: about 1M pairs per base, 27 bases per curve), one core.

## 5. Classification: runnable_local
A stdlib toy can decide whether x-small bases show target-conditional excess variance and sharpens the "beyond random-base null" clause. BATCH-79a580 (28-bit msolve fixtures) is not reproduced here.

## 6. What a run cannot establish
- Not a statement about 28-bit fixtures, P-5 yields, charged p_solve, or the GNN/SR learned predictors.
- Uses exact 2-fold sums by discrete logs (an oracle unavailable to a real attack), not Semaev polynomial solving or cost.
- Dispersion of r(T) over all T is a global statistic; it does not test per-target predictability from curve features.
- No pairing with matched rho or GAE pricing, so no claim about beating rho. Results are labelled [pcc:unverified].
