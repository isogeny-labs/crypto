# Balanced homogeneous reciprocal substitution

For a coefficient vector a of declared length n=d+1 define
R_a(V)=V^d a(V+V^-1). The declared length is part of this definition, even
if the last coefficient is zero. The standalone helper accepts arbitrary
coefficient vectors over the caller-certified prime field; only its use in
the certified recovery lift requires character-domain provenance.

Split at k=floor(n/2), so a=a_lo+W^k a_hi. The low vector has declared
degree k-1 and the high vector degree d-k. Then

    R_a(V)=V^(d-k+1) R_lo(V)+(V²+1)^k R_hi(V).

The low shift follows directly from its declared degree. The high term
follows from V^k(V+V^-1)^k=(V²+1)^k. Constant vectors terminate recursion.
The implementation never trims a coefficient slice before recursing;
polynomial outputs may be normalized after the declared shifts are applied.
This handles zero vectors, internal zeros and vanishing high coefficients.

## Implementation and cost

`HomogeneousLift` balances the split and caches powers (V²+1)^k in one
per-transform map. Powers use binary powering and are reused across sibling
subproblems. Cache hits return owned copies, which remain charged. All
polynomial products use the existing native polynomial engine.

Writing M(n) for regular polynomial multiplication cost, a recursion level
has total input length O(n) and the standard bound is O(M(n) log n), including
shifts/additions and the shared power construction. This is a conditional
arithmetic bound, not a timing comparison. The former coefficient loop
performed d successive multiplications by V²+1 on growing vectors, giving
quadratic dense coefficient work. Small degrees may favor that loop in
practice; no default crossover is selected by these correctness checks.

Initial character domains are still constructed and squarefree-validated.
The transformed polynomial is still explicit, of degree at most2d; this is
not a compact representation of the entire recovery state. Powers, copies,
map operations, intermediate vectors, polynomial scratch storage and all
source/replay/point-verification work remain charged. The section-slot cap
is not a total memory cap.

## Recovery integration

The optional BalancedCharacterImage uses the predecessor certified initializer
and only changes reciprocal substitution inside its lift. It retains the old
2d capacity preflight, divides one duplicate endpoint factor for each included
W=+/-2, filters selected orientations, and applies public-radius rescaling.
The phase implication and ramified-fiber proof are unchanged; see the exact
predecessor feature-character-phase-certificate/DERIVATION.md.

## Qualification design

Compare every output coefficient with an independent iterative substitution,
then check R_a(v)=v^d a(v+v^-1) at four nonzero points. Four fields, lengths
1..65 and127,128,129,255,256,257,1025, and six patterns cover zero, constant,
monomial, dense, alternating and vanishing-high-coefficient vectors. These
are deterministic correctness fixtures, not scalar-recovery search.
Retain endpoint/phase/capacity checks, direct-sign support oracles and public
recovery. The public comparison now targets the certified predecessor.

## Sources and next construction

Read predecessor certified-lift.rs, its derivation and native polynomial code.
The reciprocal identity is NIST DLMF18.5.1-2, https://dlmf.nist.gov/18.5,
already checked in the preceding construction. KB was unavailable in that
runtime; no novelty claim is made for balanced homogeneous substitution.

Initial domain expansion remains proportional to L. The next substantial
question is whether an observation-derived low-degree annihilator can be
constructed before that expansion. Given such a polynomial q(W), compute
D_L modulo q by doubling, then gcd(q,D_L-2epsilon) without constructing
D_L densely. The missing ingredient is a useful public observation-derived
q whose construction does not itself expand degree L; arbitrary or
fixture-derived q is not that construction.
