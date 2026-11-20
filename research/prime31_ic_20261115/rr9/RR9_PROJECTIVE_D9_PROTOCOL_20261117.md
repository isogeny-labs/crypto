# Frozen B8 RR9 degree-nine projective separator experiment

Date: 2026-11-17 UTC. PR #4550, stacked on #4549. Status **prospective; no measurements yet**. Candidate objective: prove an exact membership criterion at homogeneous Macaulay degree nine, then implement a **bounded witness-only native Rust test**. This is not a new complete ECDLP solve and no end-to-end runtime advantage is predicted.

## Immutable inputs
- Original ordinary curve: p=2,147,483,647; a=847514846; b=2087444724; n=2,147,446,831; G=(625037715,2100627251).
- B8 first rational x coordinates **[0,1,2,3,5,6,8,9]** and existing monic H_B(x).
- Public R fixtures preserved byte-identically from #4549:
  * chart-positive R=(1911579686,548820096);
  * off-support negatives G=(625037715,2100627251) and R=(1923055775,307731458).
- Existing pinned native Rust 1.100 project, field arithmetic, exact RR9 coefficient quadrics and original-curve group law. Native source base branch: `research/prime31-rr9-macaulay-degree6-20261117`.
- Do **not** substitute target points, choose new seeds, or adapt algebraic degree based on test data.

## Mathematical gates (prove independently of runtime)
1. With F(x)=x^3+a x+b nonsingular, show homogenized quadratic leading terms have **no nonzero common zero at h=0 over algebraic closure**, using F(x)U(x)^2=((x-r)A(x)+sU(x))^2 for deg A<=3 and deg U<=2.
2. Show **absence of any affine chart solution** iff the homogeneous eight-quadrics ideal has degree-nine component spanning all degree-nine forms. This is a statement about eight quadrics in eight variables and the Hilbert series (1+t)^8 on the empty-projective-locus branch; positive roots give a nonzero degree-nine evaluation functional.
3. In the fixed non-exceptional B8 chart, any affine solution implies a signed eight-leaf original-curve relation. An unsigned norm zero is NOT an unknown discrete logarithm.
4. No claim that lower degrees separate positive/negative or that the positive chart always exists for all 256 signed supports.

## Rust test gates
- Use **only** existing source curve arithmetic and RR9 equations; append narrow code rather than replace the D5/D6 experiments.
- Construct the frozen positive's *test oracle* signed support (never pass it to the public-only algebraic candidate); solve the 8-by-7 linear chart equations for seven normalized coefficients using GF(p) elimination. This is test-only known-support interpolation, not a new public extractor.
- Verify each of eight public RR9 coefficient quadrics vanishes, the direct degree-eight M is H_B, and original-curve group sum is the public R.
- Form the nonzero degree-nine monomial evaluation vector (11,440 entries). Check every sparse degree-nine Macaulay row (8*3,432=27,456) annihilates the evaluation vector, **without allocating a dense matrix**. In particular the coordinate h^9 is 1. Record wall time and arithmetic counters as a bounded **witness test only**.
- Preserve two negative controls as externally documented *test-only* off-support fixtures; do not assert a negative D9 rank unless actually computed.
- Ensure Rust 1.100 pinned library tests, example unit test and source-specific CI succeed. If any gate fails, preserve exact failure and do not relabel test outcomes.

## Decision and budgets
No paid compute. Source-specific GitHub Actions capped at 15 minutes and the witness test bounded at 300 sec. No multi-GB dense D9 elimination: D9 has N9=binom(16,7)=11,440 columns and 27,456 generator rows; a naive dense D9 pivot table has a potential >1 GiB storage cost. Negative rank/full resultant computation is a **separate prospective sparse-F4 experiment** after this math/witness gate.
No performance claim until independently verified complete blinded B31 ECDLP and same-host fully charged comparison against cached BSGS (#4532, prior 170.113ms/1024) plus tuned rho and published index-calculus frontier.

## Prior-lead/literature receipt
#4545 signed fibre, #4546 exceptional branch, #4547 seven-anchor generic equivalence, #4548 D5 Macaulay, #4549 D6 Koszul controls, #4539 chart; #4373 F4, #4228/#4349 quotient; #4513 complete mixed IC, #4532 cached BSGS; crypto-autoresearcher #3301/#3305; PKM 2016 DOI 10.1007/978-3-662-49387-8_1, APS 2018 DOI 10.1016/j.ffa.2018.01.009, Kudo et al CANS 2018 DOI 10.1007/978-3-030-00434-7_19; Macaulay2 resultant docs https://macaulay2.com/doc/Macaulay2-1.22/share/doc/Macaulay2/Resultants/html/_resultant.html. Research-KB MCP/CLI unavailable in this runtime; bounded GitHub+publisher/primary reference fallback. No claim of a first or novel resultant.
