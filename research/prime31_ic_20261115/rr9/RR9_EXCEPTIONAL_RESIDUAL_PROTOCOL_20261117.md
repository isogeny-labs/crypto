# Prospectively frozen: prime31 RR9 exceptional x(base)=x(R) branch
2026-11-17 UTC · [pcc:unverified] · PR #4546 (stacked on #4545/#4541/#4539)

## Question and algebraic prediction (before source changes)
The existing B8 signed-fibre test in #4545 rejects every residual R=(r,s) for which any base x_i=r: the split-quadric product has a zero x-r multiplier, so it is tautological. The *direct* norm quotient need not be tautological there. On the ordinary prime-order curve (no rational y=0 points), normalized chart
F=(x-r) A(x)+(y+s)V(x), deg A,deg V <=3, V_3=1,
M=D_R V^2-(x-r)A^2-2s AV,
D_R=x^2+rx+r^2+a,
gives at x=r:
M(r)=V(r)[(3r^2+a)V(r)-2s A(r)].
There are exactly two affine-linear zero conditions. If a selected signed leaf is **R**, the extra zero is F(R)=2sV(r)=0, hence V(r)=0. If the leaf is **-R**, already zero by construction, the necessary extra zero is the tangent local-parameter condition dF/dx|_{-R}=A(r)-(3r^2+a)V(r)/(2s)=0. This equals (3r^2+a)V(r)-2sA(r)=0. These two branches account for the residual x-root with multiplicity in the full norm, after dividing by x-r, and require s!=0. They are NOT equivalent to testing F(-R)=0 again.

## Predeclared bounded tests
1. Keep #4545 existing B8 exact positive/negative signed-fibre tests unchanged. Modify the linear solver for x=r to use the correct **one** constraint (R-point or tangent -R) for a selected sign, not to silently accept either. Verify direct M(r) factor identity for arbitrary coefficients.
2. Construct two **unplanted-in-solver** positive examples on the real p=2^31-1, n=2147446831 cofactor-one curve using existing Rust original-group arithmetic. In one, choose seven distinct-x points summing to O plus leaf R. In the other, choose seven distinct-x points summing to 2R plus leaf -R. Require all eight x distinct, R affine, and their group sum exactly R. The index-calculus linear solver sees only base and R, never the construction or scalar.
3. For both, independently enumerate all 256 B8 signed group sums **in the test oracle only**; solve 256 seven-variable linear systems on public residual R and require exact sign-mask set equality, and for every returned candidate check F(-R)=0, all eight signed F(leaves)=0 except the -R tangent is explicitly checked by jet, M=H_B, and actual signed group sum=R. Demonstrate both selected-sign exceptional branches.
4. Construct at least one absent signed-sum negative residual whose x is also a base x (try other bases' signed points), and require all 256 systems return no candidate. No target log or hidden scalar is supplied to the coefficient solver.
5. If generic rank<7 consistent exceptional fibres arise, **do not** silently discard them: return a diagnostic or implement sound affine-fibre enumeration; record exactly what happened.
6. Run native library and touched example Rust tests with pinned manifest in source-specific CI, retain raw output, and do not claim signed PCC, independent measurements, nonenumerative extraction or complete ECDLP.

## Prior work and stopping gates
This is a narrow correctness completion of #4466 (confluent Hermite), #4539 (seven-coefficient chart), #4541 (radical support), #4545 (256 signed B8 linear fibres). #4373 (capped m8 F4) and #4228/#4349 (quotient/subresultants), #4513 (mixed-base complete IC), #4532 (cached BSGS frontier) remain relevant; revisit algorithmic extraction only when this exact algebraic gate passes. Research-KB MCP unavailable in this runtime; GitHub primary artifacts used. Published background: Amadori–Pintore–Sala 2018, DOI 10.1016/j.ffa.2018.01.009. This local-parameter/jet identity is standard geometry, not a novel complexity theorem.

Historical comparison from #4532 (one producer run; NOT retested here): fully charged 1024Q mixed IC 2221.474 ms versus equal-precomputation BSGS 170.113 ms. Neither a new 1024Q run nor a verified speedup is implied by this correctness work. [pcc:unverified]
