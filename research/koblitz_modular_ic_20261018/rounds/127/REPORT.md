# R127 — degree-7 base fiber of the marked-23 source

At the closed point `P7(x)=x^7+x^6+x^3+x+1`, set `a=x mod P7` in `F128`. The actual R51 degree-7 base polynomial specializes to

`f23(a,Y)=(Y+123)(Y+90)^2 (Y^4+66Y^3+83Y^2+38Y+25)`

with coefficients encoded in `F128=F2[a]/P7(a)` as 7-bit integers. The quartic is irreducible over F128. Thus this source-order fiber has a simple F128 root 123, a double F128 root 90, and one degree-4 residue factor. The length-weighted residue-degree check is `1+2+4=7` over F128, or `7+14+28=49` over F2. These are fiber lengths, not normalized ramification indices. The double root is the next local normalization target; the other two factors have simple residue roots over their respective residue fields.

The producer binds the tracked R51 source by SHA-256, verifies P7 irreducible, computes derivative and gcd (`Y^2+118`), squarefree components, distinct-degree factors, and all F128 rational roots. The independent verifier reparses the R51 source, uses a separate carryless-multiply/reduction implementation, reconstructs the full factor product, enumerates roots, and certifies the quartic has no factors of degree one or two. Lean 4.34.1 checks the finite GF128 product and degree sum. Byte replay matched; a changed source was rejected before output. The first negative-control fixture had the wrong temporary directory layout and failed to exercise the intended hash gate; its corrected rerun passed. See `FAILURE-01.md`.

Source/protocol commit: `63ebb056b`. Result SHA-256: `87b87127fab784f9bf48239081194a9ebcf67456cb4204c3e1d53956e9221e30`. Run `python3 rounds/127/fiber127.py rounds/127`, `python3 rounds/127/verify127.py`, and `lean rounds/127/lean/Fiber127.lean` from the study root's parent. Exact local computation with zero cloud cost. The repository-wide Rust release library suite was previously stopped by missing tracked include fixtures in this sparse checkout; this round changes Python research files only.

Next, expand `f23(a+s,90+u)` and determine its Newton edges, then lift the double branch. Only after the base normalization can the degree-16 lambda fiber and N51 valuations at this closed point be interpreted on normalized places. This applies the [normal curve/DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y) and [simple-root lifting criterion](https://stacks.math.columbia.edu/tag/04GE) to the actual source fiber.
