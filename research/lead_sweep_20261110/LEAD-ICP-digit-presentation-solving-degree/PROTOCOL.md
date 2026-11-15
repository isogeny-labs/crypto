# LEAD-ICP-digit-presentation-solving-degree
## 1. Lead as recorded
- Disposition: open_next_step (reviewed 2026-11-02/10-30); family ic-prime; cost "est. expensive (CDCL(T)/F_p implementation + frozen panel)".
- Title: Digit (base-B) presentation of prime-field Semaev systems as a Weil-descent analogue - first/last fall degree, staircase and representation tricks.
- Next step (truncated in leads.json): "Continue the owned m=3 implementation after merged cryptanalysis PR #1347: build the source-boun..." (source-bound CDCL(T)/F_p slope measurement).
- Recorded evidence (atlas_survey.md line 20): d_ff = m 2^(m-1) + floor((s-2^(m-1))/2) + 1, curve-independent, reproduced by a curve-free null (EV-PFDR-1394a4); no syzygy beyond Koszul (EV-PFDR-e67f06); deg S_{m+1} = m 2^(m-1) voids the bounded-LFD conditional claim.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 20, 88, 233 only); research/ecdlp_subexp_desk_20261101/README.md (grep lines 101, 127). Not read in full.
## 2. Hypothesis
On a toy digit system (m=2, B=3, p=13, S_3 Semaev polynomial of a random curve with x_i = a_i + 3 b_i plus digit-range polynomials), the rank of the degree-d Macaulay matrix (d=3..5) is identical to that of a support-matched random-coefficient null: the Semaev structure gives no curve-specific rank deficit (no early fall) at these degrees.
## 3. Preregistered prediction and falsification criterion
- Statistic: D = max over 4 curves and d in 3..5 of |rank(real) - rank(null1)|.
- Prediction: D <= A (A = same max of |rank(null1) - rank(null2)|, two independent null draws; the A/A noise floor). Consistent with prediction.
- Falsified if D > A (a curve-specific deficit or excess exists).
- Positive control: S3 replaced by a multiple of a range polynomial (forced rank fall); must change rank vs null (detect > 0), else verdict is "inconclusive".
- Negative control / null: support-matched random S3 (same monomial support, uniform nonzero coefficients), same range polynomials.
- Decision: consistent -> supports the recorded curve-independence at toy size only; falsified -> an early fall candidate worth the expensive m=3 step.
## 4. Method
Pure stdlib sparse Gaussian elimination mod p over all monomial multiples up to degree d of the generators (6 variables, 7 generators). Seed 20261110; curves (a,b) drawn from random.Random(seed) with nonzero discriminant mod 13. Cost: a few seconds to about 2 minutes, < 100 MB, single core. `--smoke` uses p=11, 1 curve, d<=4.
## 5. Classification
runnable_local. A stdlib toy can sharpen the curve-independence claim at the lowest degrees; the real CDCL(T)/F_p slope step is out of scope.
## 6. What a run cannot establish
Toy sizes (p=13, m=2) are far below the 12/16/20-bit panel; ranks at d<=5 are below any predicted d_ff, so it tests only the pre-fall region; it says nothing about m=3, CDCL(T), staircase or asymptotic solving degree, and does not touch rho. Results are labelled [pcc:unverified].
