# LEAD-ICB-decomposition-encoding-and-sat

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step in leads.json (last_reviewed 2026-10-27); `disposition_20261101` = closed_obstructed.
- Family: ic-binary. Title: Decomposition-oracle encodings and SAT/enumeration solving of binary PDP (SAT/XOR, Karabina, S_m symmetrisation, chained vs direct S_4, Nagao, basis and reduction-polynomial choice, trace replay).
- Next-step excerpt (truncated): "A solver or presentation whose UNSAT cost grows by fewer than m/2 - 1 bits per unit l at m >= 3 ..."
- leads.json carries no next_step_cost or why_it_might_beat_sqrt_p for this id.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (line 206 only); docs/session-logs/2026-10-28.md (line 1095: "closed_obstructed (encoding levers are constants)"). research/ecdlp_subexp_desk_20261101/data/atlas_tally.json was a hit but not read. Nothing in cryptanalysis was searched beyond this. No primary note for the lead was found, so the mechanism comes from the excerpt plus general knowledge.

## 2. Hypothesis
For decomposition of a point R into m factor-base points (x in an l-dimensional F_2-subspace of F_{2^n}), every counted-group-operation solver implementable here, exhaustive on UNSAT targets, has a cost exponent (slope of log2 cost against log2|F|, |F| about 2^(l+1)) of at least m/2 - 1 for m >= 3. The symmetric-multiset (S_m) enumeration and the meet-in-the-middle split only change constants. Recorded obstruction: encoding levers are constants, not slope reductions.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope of log2(group additions) on log2|F| over l in {2..6}, per m in {3,4} and per solver in {direct S_m enumeration, MITM split ceil(m/2)+floor(m/2)}. Expected slopes: about m-1 (direct) and about ceil(m/2) (MITM), both above the threshold m/2-1 (0.5 for m=3, 1.0 for m=4).
- Threshold: falsified if, on arm subspace_A, the smaller solver slope is below m/2 - 1 - 0.25 for any m >= 3. Consistent if no such case, the positive control passes and the A/A slope difference is <= 0.3. Otherwise inconclusive.
- Decides: consistent leaves the closure standing for this toy family. Falsified would reopen the lead (a sub-threshold slope would need an audit for a bug first).
- Negative control: random_null arm, a non-subspace factor base of the same size per l (no linear structure). Its slopes should match subspace_A, which shows the slope comes from |F|^h and not from subspace structure.
- Positive control: planted instances (R is a sum of m factor-base points) must be found by both solvers (SAT detection works); the run is inconclusive if not.
- A/A arm: subspace_B_AA, a second independent subspace draw with the same solvers (different seed); gives the noise floor of the slope.

## 4. Method
- Curve y^2+xy=x^3+x^2+b over GF(2^13) (poly x^13+x^4+x^3+x+1), b drawn from seed 20261110; exp/log tables. Points found by half-trace. Factor base: all points with x in a random l-dimensional F_2-subspace (seeds 101, 202 for the arms, 101 for the null's size matching; null seed 303); l in 2..6, m in 2..4.
- UNSAT target: a random curve point R with no m-fold decomposition, certified by the exhaustive direct search (up to 50 candidates). Cost counted in group additions (inversions via tables, not charged).
- Solvers: direct (nested multisets of m-1 points, check R minus partial sum in F); MITM (table of h1=ceil(m/2) multiset sums, probe h2 sums).
- Cost: pure stdlib, one core, estimated well under 10 minutes (largest cell m=4, l=6: about 3.5e5 additions per solver per arm; the rest is smaller); memory under 100 MB.
- Smoke: n=7, l in {2,3}, m in {2,3}, runs in well under 1 s.
- Output: one JSON object with slopes, controls, seeds and verdict.

## 5. Classification
runnable_local. A stdlib toy can measure the slope of the encoding-independent enumeration cost directly. It does not need a SAT solver, Groebner engines or any builds.

## 6. What a run cannot establish
- It does not test real SAT/XOR solvers, Karabina, Nagao or Groebner-based elimination; the toy solvers are enumerators with counted additions, so a consistent verdict says nothing about cleverer solvers.
- l <= 6 and n = 13 are tiny; slopes are fitted over 5 points and the effect of sub-exponential factors is not visible. The toy's x-coordinate-only factor base ignores sign symmetries.
- It does not compare against Pollard rho (no section 5a pricing), and gives no evidence for or against beating sqrt(p).
- Results are labelled [pcc:unverified].
