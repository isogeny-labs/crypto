# AUDIT: N3-capacity-lp-converse (2026-11-01)

Verdict: **partially_valid.** Corrected outcome: **supports, narrowly and conditionally**. The ceiling 1/(m d)
(1/32 at m=4) is a correct consequence of the saturation premise P. P is certified only at small p for m=2 (l<=3)
and, trivially, m=3 l=1. It is assumed at m=4. Most of the ceiling is already in the programme (W10 / MP-lattices-1).
The "downsets are optimal" search has no demonstrated power. No security claim is made or implied; none found.

## What I re-ran (all under scratchpad copies; nothing in the experiment dir was modified except this file)
- `armA.py m2` (all 240 pre-registered cells, ~6 min): output identical to `raw/armA_m2.json` (deterministic seeds).
- `armB.py B0`, `B2`, `B3`: JSON identical to `raw/armB0/B2/B3.json` (ignoring timing fields).
- Independent staircase optimum (own code, exact Fractions, not the author's Dinkelbach):
  brute force over all 705,432 downsets for l=3, R=10 gives 29/225 = 0.128889. Exact bisection on lambda gives
  0.141287 (l=5, R=14) and 0.152667 (l=10, R=24). All match README/RESULTS and the MP-lattices-6 AUDIT values
  (0.14129, 0.15267, 0.16158).
- Independent box closed form at l=100: m=2 0.165008, m=3 0.040942. These equal MP-lattices-6 `RESULTS_closedform.json`
  to all digits.
- Hypothesis check (A0) re-derived in PARI with full monomial enumeration. S3: 9 monomials, per-variable degree (2,2),
  top total degree 4, unique top monomial x1^2x2^2 with coefficient 1. S4: 125 monomials, degree (4,4,4), top 12, unique
  top monomial coefficient 1. S5: 6561 monomials, degree (8,8,8,8), top 32, unique top monomial coefficient 1.
  The A0 hypotheses hold as claimed.
- Recomputed from the JSONs: 164 box cells; box cells with npts>4*Mmax are all equal except the singular
  seed-1 p=13 curve (rhs(xR)=0, all 12 points singular, diag2); 4 low-npts cells are equal; there are 54 pre-registered A1
  violations (all with npts<=4*Mmax). The 375 = 240+135 cell count is right. The m2hp JSON equals its log line-for-line.
- m=3, p in {5,7,11,13,17}, seed 0: #V(F_p) = 49, 83, 149, 181, 277, with 13, 19, 55, 37, 91 affine singular points.

## Theory check (A0)
- The filtration identity c = sum_j dim W_j is correct. I re-derived it from the chain p^{l-j}L_j.
- The lead-monomial bound is correct under P: dim W_j <= #{s in M : s >= jL}. It gives c <= sum k and delta <= 1/(md).
- The inference "box equality => c<=sum k for all M inside the box" is valid:
  - the triangular rows give dim W_j >= N_j;
  - equality of the sum forces equality at every level;
  - W_j(M) is contained in W_j(box).
- `gain()` computes log_p[ker : p^l Z^n] = l n - e correctly.
- Sampling gives c_samp >= c >= sum k, so c_samp == sum k is a rigorous per-instance certificate. Good design.
- This is a certificate for the specific small-p instances (f, p). Transfer to p ~ 2^256 is an argument, not a measurement.
  The argument is plausible (Hensel on smooth points plus Lang-Weil-type density) but not written or tested at m>=3.

## Controls and nulls
- The small-p artifacts (p=3,5,7,...) behave as the positive control: the instrument does detect extra kernel when P fails.
  That is adequate for arm A.
- The corrected threshold rule (#V > 4 Mmax) was derived post hoc from the m2hp data. It then predicted the pre-registered
  cells with one exception, which was explained by singularity. That is a fair out-of-sample use at m=2.
- The rule does NOT transfer to m=3.
  - The m=3 surfaces have many singular points even in the cells that pass (55 at p=11, 91 at p=17).
  - The failing cell is p=13 seed 0, with 181 points, vs 149 points at the passing p=11.
  - All 181 points were covered by the 750..3000 samples, so c=3 there is exact. P genuinely fails for that instance.
  - Neither "smooth curve" nor "npts > threshold" explains it. The README does say "not diagnosed". Fine, but the
    corrected rule must not be read as covering m=3.
- B arm:
  - Pre-registered SA started AT the staircase optimum, so "max rho_D - stair_opt = 0.0" is true by construction.
  - The post-hoc SA from random or diamond starts never recovered the known optimum (best gap -0.0008 to -0.0037).
    So the search cannot find the optimum even when it exists. It has no power to find a better non-downset.
  - There is no planted-positive control.
  - Conclusion: this arm gives essentially no evidence for downset-optimality. The README calls it "weak evidence".
    I would call it "none beyond the families enumerated".
  - The enumerated families (boxes, rectangles, cuts, diamonds, hollow) are all below the optimum, which is the real content.

## Numbers README vs RESULTS
Matched, apart from the issues below: 375 cells, 164 box cells, +6/+4/+1 excesses, B0 values, B2 gap -0.00079,
the diamond0.6 example (0.164 vs 0.098), and the m=3 table.

## must_fix
1. **Missing excess cell.** README ("all 7 such cells ... npts 26 gives +6, 28 gives +4, 31 or 32 gives +1") and
   RESULTS `m2_box_above_when_npts_le_4Mmax` omit a pre-registered cell: seed 1, p=31, l=3, box8, npts=30, excess +2.
   It is consistent with the 32-npts rule, but the list has 8 cells, not 7.
2. **`raw/armA0.out` is not verbatim.**
   - Segments of the PARI output are elided. Compare "(Mod(1,..)*x3^4*x2^4 + Mod(0,..)*x1^3]" with the true
     "(...*x3^4*x2^4 + Mod(0,..)*x2^3)*x1^4 + ...".
   - `armA0.gp` as saved overflows the default PARI stack on S5 (needs `default(parisizemax,...)`).
   - The hypotheses are TRUE (I verified them), but the raw evidence file cannot be checked.
     Replace it with a verbatim output of a runnable script. Add monomial counts and the top-form listing.
3. **Prior art not cited.**
   - The README says "novelty_unverified ... no search was done". The REPORT §3.3 (which the task said to read) already
     records the programme's MAP_closed / W10 strategy-independent ceiling for (p,f)^t lattices on convex supports,
     including 1/32 at m=4, and MP-lattices-1's leading-monomial ceiling on ANY monomial set.
   - State that the A0 ceiling inside (p,f)^l is re-derivation of programme-internal results. The only new content is the
     extension to all g vanishing on U, which is exactly P.
4. **Headline over-reach on P.**
   - The first sentence of the README states the m=4 converse as if established. P is assumed at m=4.
   - At m=3 it is certified only at l=1, where sum k = 1. That is the trivial case: the only element is f itself.
     It is untested for j>=2, which is the content of P.
   - One of 7 m=3 l=1 cells fails undiagnosed.
   - State "conditional on P (unproved; certified at small p only for m=2, l<=3)" in the opening sentence and in
     RESULTS `outcome`.
5. **B2 deviations and power.**
   - Protocol says l in {3,5,8}; l=8 was not run, and this is not disclosed.
   - Disclose that the pre-registered SA started at the optimum (so the 0.0 gap is by construction).
   - Replace "weak evidence" with "no evidence of power", or add a planted positive control.
6. **Budget and threads.**
   - Protocol: <1 CPU-hour, <=3 threads. RESULTS `cpu_seconds_approx` sums to 4192 s (~70 min), README says ~60.
   - Several entries are timeouts, not measurements.
   - `n3lib.py` sets OMP/OPENBLAS threads to 3 per process, with up to 3 processes. Set it to 1.
7. **Unsupported assertion.** "For e-variables with e_i <= X^i the weighted bound is also 1/32" is not computed anywhere.
   Delete it or mark it as conjecture.
8. **Provenance.**
   - `raw/diag.out` shows two curves with no (p, seed) labels.
   - The m2hp grid is incomplete: seed 2 stops at p=37, while the README says "seeds 0..2, p 3 to 53".
   - `__pycache__` is left in the experiment dir.

## Not an issue
- Class C scoping and the "Not covered" list are honest, and match the REPORT's refutation caveat (capacity bounds only
  auxiliary-polynomial methods).
- Classification as accounting is correct. B·D^2 is unchanged.
- Validation against the prior MP-lattices-6 code and audit holds exactly.

## Re-audit (2026-11-01, after repair of items 1-4)
Scope: the user asked for repair of items 1-4 only. Items 5-8 were explicitly left open, so they are reported as open, not as failures of the repair.

### Items 1-4: verified
1. **Excess cells: fixed.**
   - README line 55 now says "all 8". RESULTS `m2_box_above_when_npts_le_4Mmax` has 8 entries, and the last is `[1,31,3,box8,30,2]`.
   - Check: raw/armA_m2.json has seed 1, p=31, l=3, box8 with gain 85 against ksum 83, which is +2.
   - The npts=30 comes from raw/check_threshold_m2.json, key "(1, 31)". It is not stored in armA_m2.json, where `npts` is null. The provenance note on the m2hp curve mismatch is correct (m2hp gives 31 points, +1).
2. **armA0.out: fixed.**
   - I re-ran `gp -q -f armA0_v2.gp`. It exits 0 in under 240 s, and its output matches raw/armA0.out line for line, apart from the header and the timing line.
   - Monomial counts are 9, 125 and 6561. Each top-degree form is the single monomial x^(d..d) with coefficient 1, and the total degrees are 4, 12 and 32. This agrees with my earlier check.
   - Caveat: the run used one prime and one random instance (setrand(1)). It supports the hypotheses for this instance. It does not prove them for generic S5.
3. **Prior art: fixed.** W10 / MAP_closed and MP-lattices-1 are cited, the new content is stated to be only the extension to all g vanishing on U (which is P), and "no search was done" is gone. A superseded-text marker is kept inline.
4. **Conditionality: fixed.**
   - The README's first sentence and RESULTS `outcome` both say CONDITIONAL ON P, with the correct status of P at m=2, m=3 and m=4.
   - The m=3 p=13 seed-0 failure is noted as undiagnosed, and the statement that the point-count rule does not transfer is present (also `m2_threshold_rule_transfers_to_m3: false`).
   - Minor residue: the headline still starts "SUPPORTS". The "Next step" section says the 1/80 vs 1/32 question "no longer affects the 0.169 decision". That statement is only as strong as P.

### Items 5-8: still open (not part of the repair; confirmed by inspection)
5. B2 disclosure: README still says "weak evidence" for downset-optimality. The README does not disclose that l=8 was not run, or that the pre-registered SA started at the staircase optimum.
6. Budget and threads: n3lib.py still sets OMP_NUM_THREADS and OPENBLAS_NUM_THREADS to "3". README still says "about 60 min", and RESULTS `cpu_seconds_approx` sums to 4192 s (~70 min). Timeout entries are not marked.
7. The e-variable sentence ("the weighted bound is also 1/32") is still in "Not covered". It is not computed anywhere.
8. Provenance: raw/diag.out has no (p, seed) labels, the m2hp grid is still incomplete (the README says "seeds 0..2, p 3 to 53", but seed 2 stops at p=37), and `__pycache__` is still present.

### Verdict
Repair of items 1-4: **valid**. Overall experiment: **partially valid**. The converse stays conditional on P, and the open items 5-8 are all disclosure or provenance problems. None changes a numerical result. Items 5 and 7 overstate or omit caveats and should be fixed before the README is cited.
