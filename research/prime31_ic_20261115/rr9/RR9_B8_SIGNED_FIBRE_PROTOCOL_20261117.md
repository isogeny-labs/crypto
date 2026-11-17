# Frozen protocol — Prime31 RR9 B8 signed-fibre decomposition
Date: 2026-11-17 UTC. Branch: `research/prime31-rr9-b8-signed-fibres-20261117`. Tracking: #4545, stacked on #4541/#4539. Status: PROPOSED, before source changes and execution. No unknown-target log or runtime advantage expected.

## Purpose
Falsify the inference that solving the B8 squarefree seven-variable RR9 polynomial system is necessarily nonenumerative. It may be an exact union of 256 linear signed-divisor fibres. Prove the reduction and independently compare linear-system fibres against original-curve group sums for all admissible B8 public targets, on the existing Rust p=2^31-1 ordinary cofactor-one curve (n=2,147,446,831). Prior actual cost hurdle is #4532 (1024-target shared BSGS 170.113 ms versus mixed IC 2221.474 ms in one producer run, not rerun here).

## Identity under test
For R=(r,s) on E:y²=x³+ax+b, x_i != r, base P_i=(x_i,y_i), V(x)=v0+v1*x+v2*x²+x³, A(x)=a0+...+a3*x³, and M=D_R V²-(x-r)A²-2sAV,
```
  -(x_i-r)*M(x_i)
    = ((x_i-r)*A(x_i)+(s+y_i)*V(x_i))
    * ((x_i-r)*A(x_i)+(s-y_i)*V(x_i)).
```
Derivation: (x-r)D_R+s² = x³+ax+b; expand. Hence M(x_i)=0 iff one signed linear evaluation row vanishes. When B8 has eight distinct x, squarefree M monic degree8 and H_B monic degree8, M=H_B iff the eight signed linear factors vanish for one of 256 sign vectors. This is a union of linear fibres, not evidence that F4 can extract coefficients without sign support search.

## Preregistered execution
- Reuse existing `research/prime31_ic_20261115/native/examples/prime31_rr9_evaluation.rs` and its original-curve Arithmetic; no Python DLP implementation, replacement ECC code, or paid compute.
- B8: first eight rational-x points from the existing B31 point generator; all 256 sign vectors are the test-only exhaustive oracle. Compute each public R by the original curve's group law, record multiplicities of possible collisions, and skip only R=O or x(R) coinciding with one of eight base x (exceptional chart branch).
- For **each admissible public R**, solve all 256 candidate 8x7 affine linear systems by exact F_p Gaussian elimination for chart unknowns (a0..a3,v0..v2), with v3 fixed to 1. Compare the entire recovered mask fibre to the independent group-sum fibre, never just planted positive masks. For every recovered F, assert M=H_B, all eight signed F(P_i)=0, public F(-R)=0, radical support and original group sum. Run four additional target offsets R+kG selected deterministically outside the 256-sum support, and require empty extracted fibres; count any rejected exceptional targets and all attempts.
- At every i with x_i != r verify the factorized identity numerically for all 256 masks in the first admissible positive panel; do not treat the product identity at x_i=r as a root test.
- Fail CI on a mismatched solution set, underdetermined chart, bad signed identity, or a false positive/negative. The executor must report tested public R count, affine systems attempted, exact fibres, collisions, excluded R and negative controls. All speed/complexity claims **unsupported**: this *enumerates sign masks* and cannot establish non-generic extraction or a complete B31 ECDLP solver.

## Literature and earlier lead audit
- Internal: #4466 (RR9 Hermite/norm), #4539 (signed chart), #4541 (radical), #4228/#4349 (quotient/rootfinding), #4373 (anchored F4 censored), #4513 (complete mixed IC), #4532 (cached BSGS falsifying rho-only proxy), crypto-autoresearcher #3301/#3305 (prior negatives). B8 is an exact validation gate, not a performance proxy for B31.
- Primary prior art: Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018), DOI:10.1016/j.ffa.2018.01.009; McGuire–Müller, IACR ePrint 2017/1262. Their index-calculus techniques are not claimed novel here.
- KB unavailable: no research-kb MCP tools or CLI access in this runtime. GitHub PR source and web primary-paper catalogue fallback used. Do not infer absence from search failure.

## Decision gates
If all exact fibres match, preserve this as a *structure/falsifier* result, not a cheaper complete solver. Next candidate must demonstrably avoid 256 sign-vector enumeration at B8 and sub-support scanning at B31, and return complete unknown-Q logs with all failed algebra, setup and memory costs against same-target shared generic BSGS plus tuned rho / external IC. No status promotion without exact-source CI and independent reproduction. PCC remains unverified.
