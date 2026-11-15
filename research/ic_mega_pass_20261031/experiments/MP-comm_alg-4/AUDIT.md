# AUDIT MP-comm_alg-4 (2026-10-31/11-01)
audit: partially_valid. stated outcome: refutes. corrected outcome: inconclusive.

Reran (chain arm only, all 52 instances, same seeds, msolve): 0 mismatches in D or unsat vs raw_output.jsonl. Null arm not rerun (60 s timeouts). Numbers in README/RESULTS match raw data.

Code vs protocol: field/modulus/S3/trace condition for xR/seeds (1000*cell+draw) match. XL refutation code looks correct. No off-by-one or wrong-field found. Null skipped for N>=20 is disclosed.

Defects:
1. Confound (decisive). Eq-set 2 = S3(y,x3,xR) touches only dy+d3 variables (verified: 5,6,8,9,16 vars at dy=2,3,5,6,13 for l=3), with 13 Boolean eqs. Eq-set 1 touches d1+d2+dy. In nearly all cells a sub-system alone is over-determined (vars < 13), so the refutation degree is set by the local over-determination of dy+d3, not by N or min block. "D rises with N" at fixed min block is just dy+d3 growing from 6 to 16. The y-series drop at dy=l+3 likewise is this effect. The count-law vs block-law contrast is not isolated by these cells; neither law is tested.
2. Regime mismatch. All instances (51/52) are UNSAT: the experiment measures refutation degree of over-determined systems, while the block-law hypothesis concerns solving degree near the critical regime (N approx 26 or sat). D range 2-4 gives no resolution; the decision rule fired on a 2->3->4 change in D.
3. Control arm is not a valid matched null for this question. random_matched scatters monomials over all N variables, destroying the variable-support locality (eq-set 2 on dy+d3 vars) that causes the chain's low D. Delta=-1..-2 is therefore expected from locality alone and says nothing about block structure. Null D rising with N is also confounded (denser mixing).
4. Selection: null missing for N>=20 and 7 timeouts at N=18-19 (null_med entries None); the pooled null comparison is biased toward easy nulls. Blocks are independent random subspaces (no shared V), as README admits.
5. README overclaims "count law wins / hypothesis refuted": C2 rise is consistent with count law but equally with block law plus a sub-system-size effect; C1 was tie/mixed (N=14 mixed: 2.5 vs 2 vs 3, where (2,4,4,4) with min 2 gives the HIGHEST D=3, against both pre-registered directions cleanly). Also one outlier XL D*=5 at (2,4,4,4) seed 11000 unexplained.
6. The pre-registered rule technically returns "refutes" (C2 rises 2/2 with C1 tie/mixed), but the rule's premise (cells separate N from min block) fails per 1.

Corrected outcome: inconclusive. Valid residual claim: at n=13, 26 eqs, N<=25, the msolve refutation degree of the chain tracks local over-determination (vars in smallest-overdetermined sub-system), D in 2..4. Nothing about solving degree growth, nothing at n=131. Classification stays accounting; B*D^2 unchanged.
Suggested redo: sat regime (N approx 26+ or fewer eqs so every sub-system is under-determined), null that preserves per-equation variable support, report vars-per-subsystem as covariate, exact closure degree.
