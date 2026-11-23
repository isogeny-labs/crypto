# Iteration 7 — an unconditional growing smooth-torus field family (not an ECDLP gain)

**Date:** 2026-11-22 Pacific / 2026-11-23 UTC. **Standing:** a written, source-auditable mathematical deduction, **[pcc:unverified]**. This is a new *family-qualification* note in existing draft PR #4928, not a native code or performance experiment. It does not replace or change the iteration-six frozen Rust source, correctness scope or ongoing source-specific CI. No claim of new Semaev relation-solving asymptotics, original-curve transfer, independent rank or complete solver follows.

## 1. A growing field family with a polynomially large power-of-two divisor of p+1

Let q_k=2^k, k >= 2. Let p_k be the **least rational prime** congruent to -1 modulo 2^(k+2). The residue class is coprime to 2^(k+2); existence follows from Dirichlet, but that alone gives no quantitative relation between q_k and p_k.

Linnik's theorem supplies *absolute* C,L>0 such that the least prime in every coprime residue class modulo d is at most C d^L (for large d, or by increasing C for small d). Hence

    2^(k+2)-1 <= p_k <= C (2^(k+2))^L = C 4^L q_k^L,
    q_k | (p_k+1),   4 q_k | (p_k+1),
    (p_k/(C 4^L))^(1/L) <= q_k <= (p_k+1)/4.

In particular q_k grows at least polynomially in p_k along this explicitly defined (though generally not efficiently enumerated) sequence. The lower bound p_k >= 2^(k+2)-1 ensures p_k tends to infinity. No assertion about infinitely many primes of the special form 2^h-1 appears. No *specific effective* constant or uniform prime-finding runtime is claimed; prime search and primality certification are setup costs for any implemented family. The exponent 1/L here is a **smoothness-size exponent only**, not an ECDLP algorithm exponent.

Primary source for the quantitative theorem: Linnik, as expounded in the number-theory text *Multiplicative Number Theory I*, Chapter 27, Theorem 27.1 (https://dms.umontreal.ca/~koukoulo/documents/publications/primes.pdf). The classical theorem is sufficient; no reliance on a recent advertised numerical value of L.

## 2. Exact full rational Dickson fibre and its recurrence

For each p=p_k let T={z in F_(p²)^*: z^(p+1)=1}. This is the cyclic norm-one torus of order p+1. Set q=q_k. There is an element ζ of exact order q and an element z with z^q not in {+1,-1}: because 4q divides |T|, a generator z of T satisfies that condition.

Define the trace coordinate x(u)=u+u^(-1) in F_p for u in T; indeed u^p=u^(-1). For Dickson's parameter-one polynomials,

    D_0(X)=2,   D_1(X)=X,
    D_(2^(j+1))(X) = D_(2^j)(X)^2 - 2,

so D_q(x(u))=u^q+u^(-q). Let c=z^q+z^(-q) in F_p, c ≠ ±2, and define

    F_k(X)=D_q(X)-c in F_p[X].

The q values x_j=z ζ^j+(z ζ^j)^(-1), j=0,...,q-1, are rational roots of F_k. They are pairwise distinct: x_i=x_j implies either ζ^i=ζ^j, hence i=j modulo q, or z ζ^i=(z ζ^j)^(-1); raising the latter identity to q gives z^(2q)=1, excluded by construction. Because F_k is monic of degree q, these are *all* its roots. Thus F_k is split and squarefree and its full fibre has exactly q points, not merely a low-degree *formal circuit*. This proves a family-level algebraic input qualification; it is not a finite-field arithmetic-test receipt or a sublinear common-root oracle.

One exact realization uses F_(p²)=F_p[v]/(v²-d), d a certified nonsquare; norm-one pairs (a,b) satisfy a²-d b²=1. Enumerating all q trace coordinates by repeated ζ multiplication costs Θ(q) extension-field operations and Θ(q) output slots. Computing a single D_q(x) by the doubling recurrence uses k squarings, but **does not** enumerate, isolate, or prove emptiness of the q-root fibre. Target-specific factor-base lifting and group verification remain charged. Exceptional c=±2, Frobenius-domain mistakes, singular/repeated endpoints and E-twist ghosts must retain the previous iteration-six guards.

## 3. Distinguish smooth *torus* structure from ordinary-curve ECDLP structure

For any elliptic curve E/F_p with trace of Frobenius t, #E(F_p)=p+1-t. Consequently,

    gcd(q,#E(F_p)) = gcd(q,t)      because q | p+1.

If t is odd, q is coprime to #E(F_p). A group homomorphism E(F_p) → T[q], where T[q] is the q-torsion of the norm-one torus, is then necessarily trivial: its image order divides both source and target orders. The same is true for any subgroup of E(F_p). Hence the smooth p+1 factor **cannot be silently used as an E DLP-preserving homomorphic transfer** for odd t. This refines the earlier exact-bucket obstruction with a precise Frobenius-trace identity. It does not exclude nonhomomorphic auxiliary coordinates or algorithms exploiting them.

As a purely existential ordinary, non-anomalous curve example, Deuring–Waterhouse permits trace t=3 for all sufficiently large p_k, giving #E(F_(p_k))=p_k-2 (and gcd(t,p_k)=1). This does **not** guarantee a large prime-order subgroup, a suitable generator, random-like factor-base relations, or a cryptographically representative family. In particular it must not be treated as satisfying the user's complete-solver family requirement by itself. Avoid t=1 as a security representative: trace-one curves have anomalous order p and are deliberately weak.

## 4. Costs and the actual missing theorem

The family now has q_k=Ω(p_k^(1/L)), split q_k-coordinate Dickson fibres and an explicit degree-doubling recurrence. However, for a chosen ordinary E_k it is still unknown whether a positive fraction (or the required B_k) of these trace coordinates lift to rational points of E_k in the desired large prime-order subgroup. A split Dickson polynomial says **nothing** by itself about the quadratic character of f_E(x_j). The proof also does not give a target-dependent exact empty-intersection certificate in o(q_k) charged time, nor any reduction from arbitrary group-target relations to a compact, efficiently extractable root test.

To advance beyond this family lemma, prove or falsify:
- an ordinary, non-anomalous E_k with an explicitly certified suitably large prime-order subgroup and enough **signed, liftable** q_k-fibre points, with construction/setup charged;
- an exact DLP-free, target-dependent **nonhomomorphic** oracle for the actual S3/S_m plus factor-base constraints whose empty-query cost, state construction and failure branches are sublinear against the maintained O(B) group-complement scan, rather than against dense norm only;
- complete natural-target original-curve signed relations, independently checked rank, target recovery and strongest matched algebraic IC + optimized negation-map rho, with EKS/admission and signed independent PCC status.

Precomputation P, query Q, failures F, output extraction W, matrix/target recovery L and memory M must all be stated. A small quadratic circuit (iteration six) is a *witness verifier*, not a root finder; this torus family changes none of those missing obligations. Neither the old dense scalar Dickson norm (Ω(q) empty-state materialization) nor the fully enumerated signed m-sum dictionary (Ω(B^m) setup) is recycled as an alleged asymptotic solution.

## 5. Provenance and scope gate

Primary mathematical inputs: Linnik's theorem (source above); Dickson polynomial identity/recurrence (https://encyclopediaofmath.org/wiki/Dickson_polynomial); Deuring–Waterhouse ordinary trace existence (Waterhouse, *Abelian varieties over finite fields*, Ann. Sci. Éc. Norm. Supér. 1969, and standard summaries). Existing internal negatives/controls: this PR's NEXT_IMPLICIT_INDEX.md §§A–F, NEXT_ROOT_INTERSECTION.md, iteration6/SCOPE.md and SOURCE_FREEZE.json, #4924 paired monic work, #4829 group-complement and #4901 authentic rank/cold baseline. Also keep distinct the integer 3SUM speed literature (Chan–Itakura–Ma–Yu arXiv:2610.42012) and output-sensitive generic function-inversion reporting (Dinur–Golovnev–Yan–Zhu arXiv:2611.13294): neither supplies an automatic DLP-free point-encoding reduction or complete signed IC recovery.

**Claim standing:** source-auditable written derivation only, [pcc:unverified]; no independent signed/formal proof, original-lock new tests, paid compute, admitted timing, speedup or merge. **No action on PR #4924, the frozen iteration-six sources, or the current CI execution.**
