# CT81 — Fixed-prime compatibility and moving-prime permutation audit

**Status:** [pcc:unverified]. A bounded, exact public-toy test and a classical group-theoretic deduction; **not** an ECDLP complexity lower bound or a global cross-characteristic algorithm.

## Setup and mathematical result
The CT80 witness is Cremona 11a1 over F_37, with #E(F_37)=35 and P=(9,4) of exact order 7. Its Frobenius trace is t=37+1−35=3, determinant is 37≡2 (mod 7), and eigenvalues on E[7] are 1 and 2. Therefore precisely six nonzero geometric 7-torsion vectors are rational over F_37, forming the one-dimensional 1-eigenspace without zero.

CT80's oriented point field K=Q(T) has degree 48, with embeddings indexed by all 48 nonzero geometric 7-torsion vectors, as verified for that fixed example in CT80 (not independently re-established here). Thus the six degree-one reductions at 37 correspond to six distinct oriented rational torsion points. Their three x-coordinates are 9, 27 and 28, with two ordinates each. Choosing a *source place* is selecting the image of the global oriented point.

For any chosen pair of good-reduction places v|37 and w|2, the specialization composition
    red_w ◦ (red_v|<T>)^{-1}
is an order-seven group isomorphism onto its image. Importantly **v and w must stay fixed for both encoded P and Q**. Choosing v separately for each input is not equivalent to using a single group homomorphism, even if all selected points have order seven.

Among all 6! arbitrary bijections of the nonzero elements of a cyclic order-seven group fixing its identity, exactly φ(7)=6 preserve addition; if the designated generator's image is fixed, only **one** does. The other 714 are invalid. This is a *combinatorial superset control*; we do not claim that all 720 bijections arise from algebraic specializations. For general odd prime ell there are (ell−1)! such arbitrary assignments and only ell−1 automorphisms.

## Exact independent arithmetic
- `ct81_source_prime_alignment.py`: implements the generalized Weierstrass group law over F_37 and exhaustive 720 permutation test, with no CT80 imports.
- `ct81_alignment_independent.js`: independently implements the group law and exhaustive permutation test; its first version had a doubling-slope typo, which was fixed **before** the successful execution.
- Locally ran Python normal and -O, Node normal and --no-opt: passed; group order=7, x-roots=9,27,28, all 720 assignments, six homomorphisms, one fixing the generator.
- No CI receipt or independent PCC signature yet. No timed benchmark or paid compute.

## What this proves and does not prove
It is incorrect to move P and Q across **different** local source specializations and then treat their images as though they came from one global group homomorphism. The existence and unique local p-adic lifting of individual points do not resolve this global compatibility condition. The result does not forbid a uniform map that computes both globally compatible representatives without scalar enumeration; that remains open.

## Next research acceptance criterion
Given a *fixed* source prime v of K and target prime w, recover the global representative of arbitrary encoded Q∈E(F_37)[7] from its v-adic torsion lift using root recognition **without enumerating all scalar multiples**. Explicitly price number-field construction, p-adic precision, root selection, expression of the global coordinate in K, specialization at w, and the target DLP. Prove this approach beats enumerative reconstruction on growing ell before attempting a claimed cryptanalytic advantage.

## Sources / provenance
- CT80 `research/cross_characteristic_global_torsion_20261112/ct80/CT80_SEVEN_POINT_FIELD.md` and CT73 shared specialization.
- CT71 field stabilizer and CT74–75 local lifting.
- V. Shoup, *Lower Bounds for Discrete Logarithms and Related Problems* (1997), https://www.shoup.net/papers/dlbounds1.pdf — generic-group theorem; **not applied as a lower bound for this algebraic recognition task**.
- Repository `AGENTS.md`, dated session log 2026-11-13; shared research-KB MCP and full checkout unavailable in this connector session, so KB receipt and full Rust/Conductor gates remain pending.
