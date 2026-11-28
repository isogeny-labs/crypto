# Smith discriminant and branch admission gate [pcc:unverified]

Fixed all6 maps in native-smith-affine/CONTROL.txt; no source substitution. Freeze source/DERIVATION/protocol/input/binary hashes and push before one600CPU invocation. Git custody while S3 unavailable, no S3 polling; preserve failures/no unchanged rerun. Rust overflow checks enabled.

Compute exact Fp[t] depressed-quartic discriminant using dynamic polynomials (degree exceeds native fixed buffer). Require exact disc(P)=delta4*I², delta4 degree4 squarefree, norm root degree3 squarefree and disjoint from delta4, D degree2 squarefree, gcd(F,D) degree1 and gcd(N,D)=1. Print discriminant/index coefficients, index radical degree and r power with residual. Retain all outcomes.

For both F61 maps compare native discriminants at all t with nonzero leading coefficient; require one separable irreducible specialization per map and record t. This proves arithmetic irreducibility with the stated primitivity hypothesis, not geometric connectedness or a smooth canonical model. Conditional ramification/genus reasoning is in DERIVATION.md and is separate from executable evidence. No speed or complete transport claim.
