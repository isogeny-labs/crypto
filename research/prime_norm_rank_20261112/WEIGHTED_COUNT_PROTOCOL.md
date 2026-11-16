# Weighted monomial count: prospective proof contract

2026-11-14. Goal: a Lean proof supporting the growing-arity public norm
finder analysis. This component is unconditional finite combinatorics;
it does not assume or provide a finder, a small closure cutoff, or an
ECDLP algorithm. No experiment or cryptomath solver execution is authorized
by this protocol.

For positive integer weights w_i and weighted exponent vectors e with
sum_i w_i e_i <= D, introduce digits 0 <= t_i < w_i. The map
(e,t) -> (w_i e_i+t_i)_i is injective by Euclidean division. Its total
sum is <= D+sum_i(w_i-1). Stars-and-bars bounds its image by
choose(D+sum_i w_i,v), where v is the variable count. Target:

    M(w,D) * product_i w_i <= choose(D+sum_i w_i,v)
    M(w,D) * product_i w_i * v! <= (D+sum_i w_i)^v.

Specialize w_i=i+1 on Fin v, product weights=v!, sum weights<=v^2,
to obtain M(v,D)*(v!)^2 <= (D+v^2)^v, including v=0 and D=0.
All exponent vectors must be counted; no supplied support or omitted
monomials. Acceptance: source compiles without holes/extra axioms, full
study gate and import-inclusive fresh replay, source-pinned unsigned PCC
claims, editable report and inspected PDF. Retain all failed attempts.

Sources checked(KB): unavailable (no loaded tools; wrapper missing executable).
Primary fallback: pinned Mathlib Data/Sym/Card.lean (stars-and-bars),
Data/Nat/Choose/Bounds.lean (factorial-adjusted binomial bound), prior ML
NormalFormBound/LocalReductionBound, and actual P1514/P1536 audit artifacts
at crypto-autoresearcher14f7a739d. P1514 dense cutoff is sufficient, not
minimum; P1536 norm jets require a charged constructor. No novelty claim
for this elementary counting argument. Conductor executable unavailable.

Open obligations: bind growing norm input weights; construct actual full
closure/normal forms at a sufficiently small cutoff; prove public known-label
base coverage and capped-attempt success; charge signed output, extraction,
setup and all bit costs [pcc:unverified]. Any eventual asymptotic specialization requires a
separate Lean argument; this protocol claims only exact finite inequalities.
The original Wnew=Wbase+C*H jet proposal has no established map here.
