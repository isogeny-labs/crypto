# Degree-bound audit and outstanding independent obligation

The pinned sibling `continuation-20261116/SPLIT_TOWER_BOUND.md` is unchanged.
This audit concerns the exact lifted generators, over a prime field p>3,
with coprime N_i,D_i, deg N_i=q_i>=2, deg D_i<q_i and *every* backward fiber
split into q_i distinct field roots. It does not add field equations.
The proposed bound remains d_F<=T=2 sum(q_i-1)+2 [pcc:unverified].

## Fresh proof-obligation review

1. Sum conditional projectors starting with the lowest fiber. Each sibling
   Lagrange sum equals1 as a literal polynomial, so the global sum is exactly1,
   not merely1 modulo the tower ideal. Choosing the highest differing path
   coordinate establishes Kronecker evaluation and linear independence.
2. In the coordinate identity, the carry from gate i is D_i times the next
   coordinate difference. Its total degree replaces q_i-1 projector degrees
   by at most q_i-1 denominator degrees. Every gate multiple is at mostS+1.
   For two chains the coordinate relations lie in V_(2S+1).
3. Reduce the two Vieta equations after multiplication by the two-chain
   projector. Replacing a product of coordinates needs one additional
   coordinate multiplication, so this step uses2S+2, not2S+1.
4. For r!=0, the linear slope relation has actual degree at most2S+1;
   multiplication by L+ell stays withinT and discards inconsistent blocks.
   For r=0 retain a quadratic algebra. No division by r, radicality or complete
   rationality of slope blocks is assumed. Nilpotents and nonsplit blocks
   must remain in the algebra used for the degree argument.
5. Basis representatives have degree at mostT-1. Their coordinate actions
   have certificates in V_T. When multiplying an inductive monomial relation,
   use its *actual* degree, at most max(deg monomial,T-1); this keeps the next
   multiplication inside max(T,next degree). A bound on a chosen representation
   of a polynomial would not suffice here.
6. Every original generator vanishes in the local product algebra. Its
   representative span injects into that algebra, so f in the ideal has
   NF(f)=0. The preceding degree-controlled induction then supplies the
   required last-fall statement. The empty product case is covered by1 inV_T.

No additional gap is identified in this hand derivation, but this is a
producer review, not an independent or formal theorem verdict [pcc:unverified].

## A different finite checking route

`native/tower_degree_audit.rs` constructs the exact sparse polynomial space
V_T from the original generators, closing under all permitted coordinate
multiplications and graded-leading-term Gaussian elimination. It does not use
the candidate solver or its field implementation. Distinct leading terms
ordered by total degree ensure that low-degree elements are spanned by
low-degree rows, so this closure computes the standard constructible space
for the finite case, rather than just a degree-T Macaulay span.

For the prospective cases in AUDIT_PLAN.md it checks literal coordinate
certificates/partition of unity, discarded projectors, every good local-basis
coordinate action, and truncated quotient dimensions. The mixed quadratic/
cubic case checks only projector certificates, as declared before the run.
These finite checks do not prove the quantified theorem or provide independent
signed standing. The r=0 nilpotent/nonsplit cases are deliberate controls
against silently replacing scheme algebras by rational solution sets.

## Independent obligation

A verifier separate from this producer must inspect the general coordinate
certificate and normal-form induction, or formalize them, before the bound
is promoted. Existing pending PCC statement480062 remains unchanged and
unverified. No local checker is described as an independent signature.

Primary sources read: Huang–Kosters–Yang–Yeo definitions2.1/2.5 (linked in the
original note), PKM section3.3 and internal prime_tower.rs; FLINT's half-GCD
sources only guide the separate arithmetic work. KB MCP and wrapper were
unavailable. No novelty claim.
