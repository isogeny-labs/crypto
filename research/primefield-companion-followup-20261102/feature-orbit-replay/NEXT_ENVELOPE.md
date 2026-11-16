# Proposed next construction: doubling an orbit-stable envelope

Status: derived candidate, not implemented or tested in this component.

After replay shrinks a candidate polynomial G dividing stable L, construct its minimal stable envelope H=lcm_(j=0..m-1) monic(G(zeta^j X)). This may permit later residue powers at smaller stable degree. It does not by itself promise a reduction: one surviving scalar in each image keeps every image fiber in the envelope.

Avoid m separate rotations by doubling covered exponent intervals. Start H0=G. Given Ht covering rotations0..2^t-1, set

    H_(t+1)=lcm(H_t(X), monic(H_t(zeta^(2^t)X))).

After ceil(log2 m) stages, the covered interval contains a full period, so H is the full envelope; repetitions do not affect the lcm. Every intermediate divides L, because each rotation of G does. Degree therefore never exceeds degree(L), despite the apparent doubling. Nonzero scaling factors must be removed to preserve monicity. Handle constant1 separately; zero-root and d-odd cases need exact checks.

Using the installed polynomial multiplication/gcd/division engine, a conservative bound is O(M(N)log(N)log(m)) field operations, plus rotations, normalization and storage, where N=degree(L). This is only a derived estimate until implementation verifies the degree invariant and accounts for intermediate products. Compute lcm as A*(B/gcd(A,B)), checking the final degree cap before multiplication.

Next checks: exact stable closure against independent root-set enumeration, minimality, rotation invariance, divisibility into original L, cases where the envelope cannot shrink, and actual degree-profile recording. Any comparison of the full replay strategies still requires a frozen protocol and matched costs. Do not assume the envelope construction pays for itself.
