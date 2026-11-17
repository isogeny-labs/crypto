# Frozen protocol — seven-of-eight RR9 quotient split on prime31

Date: 2026-11-17 UTC. PR #4581. **Registered before the Rust example source or measurements.**

## Hypothesis and algebra
On the actual ordinary cofactor-one curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, let `A=a0+a1 X+a2 X²+a3 X³` and `V=X³+v2 X²+v1 X+v0`. For public `R=(r,s)`, `M=(X²+rX+r²+a)V²-(X-r)A²-2sAV`. At nonexceptional rational base points `P_i=(x_i,y_i)` of B8, the seven equations `M(x_i)=0` for i=0..6 each split as `L_i^+ L_i^- = -(x_i-r) M(x_i)`, where `L_i^±=(x_i-r)A(x_i)+(s±y_i)V(x_i)`.

**Claim to test:** all 128 seven-sign coefficient systems for each frozen nonexceptional residual are nonsingular, have pairwise distinct coefficient points, satisfy the first seven product equations, and the eighth equation selects 1 chart for positive and 0 for each negative. If so, the seven-quadric affine quotient is exactly a split 128-point algebra (Chinese remainder theorem), hence solving it alone does not remove the signed-branch search. This is an *explicit bounded computation*, not a general lower bound on any sparse F4 algorithm.

## Frozen inputs, gates
B8 x = [0,1,2,3,5,6,8,9], with positive y chosen canonically by the actual curve square-root operation. Fixed public residuals: positive (1911579686,548820096), negative #1 G=(625037715,2100627251), negative #2=(1923055775,307731458). No rerolling or relaxing gates.
1. Verify curve, all points, `r` distinct from each B8 x, all nonzero y.
2. For each of 3 public R and 128 first-seven sign masks, solve exactly a 7x7 affine linear system modulo p. Require full rank 7 and 128 distinct vectors, with no double sign ambiguity.
3. Independently check each coefficient witness by the **unfactored** `M(x_i)=0` for i<7, then evaluate both eighth affine factors and `M(x_7)`; require exactly [1,0,0] selected charts, with native original-curve eight-sum replay for the positive.
4. Source-specific Rust 1.100.0 release library tests and new example test; bounded runtime and raw JSON. Timings are diagnostic, not an optimized nonenumerative F4 benchmark. No secret scalar enters candidate.
5. Mark inability to compile or execute separately from mathematical failure. No complete log recovered or speedup declared.

## Prior work / non-duplication
RR9 signed identity #4545, exception #4546, seven supplied anchors #4547, D5/D6 #4548/#4549, D9 #4550, B8 barycentric #4556/#4558, generic MITM #4575; autoresearcher #3301/#3305; cryptanalysis #1536. Primary references: Petit–Kosters–Messeng, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, FFA 2018 (DOI 10.1016/j.ffa.2018.01.009); Kudo–Yokota–Takahashi–Yasuda, CANS 2018 (DOI 10.1007/978-3-030-00434-7_19). Their published methods address prime-field IC and hybrid GB methods; **this split-quotient check is a scope-specific arithmetic observation, not a novelty claim.** Research-KB MCP and local CLI unavailable in connector-only environment; no exhaustive literature absence claim.

## Decision
If all three 128-point gates pass, retire naive seven-quadric quotient *as a sign-independent algorithmic shortcut*; keep true public-only full eight-quadric D9 unit/witness extraction as the unsolved experiment. If any gate fails, preserve counterexample and investigate its exceptional stratum first.
