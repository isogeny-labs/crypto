# LEAD-ICP-shared-coefficient-membership

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (2026-11-01 review; disposition_20261101 was open_unexamined). Family: ic-prime. Last reviewed 2026-10-30.
- Title: Shared divisor-coefficient circuits for high-degree implicit factor-base membership. Cost: est. medium. Why it might beat sqrt(p): "Cheaper complete circuit for high-degree implicit membership; no data".
- next_step_excerpt (truncated in leads.json): "Prove chart correspondence. Freeze k=3,4 censuses only after checking at least five usable disti..."
- Atlas survey text: shared divisor-coefficient circuits (norm A^2-B^2 f = (X-r)U from L(6O), Graeffe tower); no measurement; solver-slope claims must go through the m=3 protocol of cryptanalysis PR #1347 (natural-only e<0.5, setup a<1.5); m=2 cells can never be exponent evidence.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 17-31, 87-97, 242 via grep); /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 138 via grep). The census definitions (k=3,4, "five usable distinct...") were not recovered; the full lead text is encrypted.

## 2. Hypothesis
The "chart correspondence" step is the claim that a function A+By in L(6O) on y^2=f(x) (deg A<=3, A3!=0, deg B<=1) has F_p-rational affine zeros exactly at (r, -A(r)/B(r)) for the F_p-roots r of the norm N(x)=A^2-B^2 f with B(r)!=0, so the norm polynomial is a complete membership test, and (Abel) whenever N splits into 6 distinct F_p-roots the six zero points sum to O. H: over random (A,B) on toy curves, zero-set and root-set agree in 100% of trials, the Abel sum is O in 100% of fully split trials, and the fully split rate equals 1/720 (generic degree-6 splitting) within 4 sigma. This tests only the mathematical correspondence and the generic splitting yield, not any cost advantage of a shared circuit.

## 3. Preregistered prediction and falsification criterion
- Statistics: chart violations (count), Abel-sum violations (count), split-rate z-score vs 1/720 (binomial), A/A z (two independent seeds, same p).
- Prediction: 0 chart violations, 0 Abel violations, |z|<4, |A/A z|<4, at least 30 split trials.
- Falsified if any chart or Abel violation occurs (the correspondence as stated is wrong, e.g. missing degenerate charts), or the rate deviates by >=4 sigma (the "census" would then not be a generic-model one). Inconclusive if fewer than 30 split trials.
- Positive control: Abel sum = O on genuine split zero sets (detector sees the identity). Negative control: replace one zero by its negation; the sum must then be O in <=2% of split trials (checks the sum test is not vacuous). A/A: seeds 11 vs 12 at p=521, same code and instrument.
- Degenerate charts (B(x)=0, forcing A(x)=0) are excluded by definition and are counted outside the test.

## 4. Method
Python 3 stdlib. Curves y^2=x^3+ax+b over F_p, p=521 (seeds 11, 12; 60000 trials each) and p=1031 (seed 13; 20000 trials), a,b from random.Random(seed). Each trial draws A0..A2 uniform, A3 uniform nonzero, B0,B1 uniform. Zero set by brute force over all affine points; root set by brute-force evaluation of N over all x; compare. Fully split: 6 roots and 6 zero points with distinct x. Group sums by affine chord-tangent addition. Cost estimate: about 4-6 minutes on one core, under 50 MB RAM. Command: python3 -I run.py (--smoke for the 300-trial check).

## 5. Classification
runnable_local. The correspondence and split yield are decidable by a stdlib toy.

## 6. What a run cannot establish
Nothing about solver slopes, the m=3 protocol gates (e<0.5, a<1.5), the Graeffe tower, cost of a shared circuit versus matched rho (5a pricing at 0/0.1/1 GAE), or the k=3,4 census. Toy sizes (p~500-1000) cannot show asymptotics. A pass is a sanity check of a known identity and generic yield, not an improvement. Results are labelled [pcc:unverified].
