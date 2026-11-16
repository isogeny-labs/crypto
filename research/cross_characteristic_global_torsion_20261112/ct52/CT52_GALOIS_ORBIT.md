# CT52 — Exact mod-3 Galois image and point-versus-basis degree

**Scientific standing: [pcc:unverified].** This is a fixed-curve algebraic obstruction/certificate, not a cross-characteristic morphism, a scalable input transfer, or a novel Galois theorem. Extends CT51 on the separate stacked PR #4015; no native PR #3877 files changed.

## Exact instance theorem

For Cremona 11a1 over Q, E: y²+y=x³−x²−10x−20, the 3-division polynomial is

ψ₃(X)=3X⁴−4X³−60X²−237X−21.

**The image of Gal(Qbar/Q) on E[3] is exactly GL₂(F₃), of order 48.** Every nonzero 3-torsion point has degree 8 over Q; the entire division field Q(E[3]) has degree 48.

### Proof

1. The discriminant is −11⁵, so reductions at 2 and 7 are good, and 3-torsion is unramified there. The four roots of ψ₃ are the four projective lines of E[3], identifying the projective Galois action with a subgroup H of PGL₂(F₃) ≅ S₄.
2. Modulo 2, ψ₃ = X⁴+X+1, irreducible (no degree-one or degree-two factor). Frobenius at 2 induces a 4-cycle on the four 3-torsion lines.
3. Modulo 7, ψ₃ = 3X (X³+X²+X−2); the cubic has no F₇ root, so Frobenius at 7 induces a 3-cycle with one fixed projective line.
4. The order of H is divisible by both 4 and 3, hence by 12. If |H|=12, H is an index-two subgroup of S₄ and must be A₄, contradicting the presence of an odd 4-cycle. Therefore H=S₄ and |H|=24.
5. For a quartic root X mod 2, put R=X³+X² ∈ F₁₆. The explicit trace is Tr₍F₁₆/F₂₎(R)=1, equal for all conjugate roots. For a corresponding torsion point (X,Y), Y²+Y=R implies Y¹⁶=Y+1, hence Frobenius₂⁴(P)=−P. The Frobenius₂ matrix on E[3] has order 8.
6. The full image G⊂GL₂(F₃) projects surjectively onto H=S₄ with kernel G∩{±I}. If the kernel were trivial, G≅S₄ would contain an order-eight element, which is impossible for permutations of four letters. Therefore the kernel has size 2, |G|=48, and G=GL₂(F₃).
7. GL₂(F₃) acts transitively on the eight oriented nonzero vectors and the stabilizer of any vector has size 6. Therefore [Q(P):Q]=48/6=8; the full division field degree is 48 by the faithful representation.

## Conditional orbit-size certificate for a prime ell

When the mod-ell image of an elliptic E/Q is GL₂(F_ell), the following degrees are exact. These are conditional on full image; the script's ell=5/7/11 GL₂ enumerations **do not** certify that 11a1 has those images.

| Global object encoded | Orbit/field degree |
|---|---:|
| Oriented nonzero torsion point P | ell²−1 |
| Unoriented {P,−P} (the x-coordinate) | (ell²−1)/2 |
| Cyclic subgroup / projective line | ell+1 |
| Full ordered torsion basis / entire division field | ell(ell−1)(ell²−1) |

A cyclic subgroup does not determine the orientation of the public ECDLP generator. In particular, for ell=5 the x-coordinate degree 12 and cyclic-subgroup degree 6 differ. Do not substitute ell+1 for the cost of specifying an oriented point.

11a1 is non-CM because its j-invariant 496³/(−11⁵) is a noninteger rational; rational CM j-invariants are integers. Serre's open-image theorem therefore implies surjectivity for all sufficiently large prime ell for this **fixed** E. Consequently a dense absolute-field presentation of Q(P) has degree ell²−1 in that family, while a dense full-division-field presentation has degree on the order of ell⁴. The theorem does not determine the exceptional-prime cutoff and gives **no uniform bound** for input-dependent elliptic curves, higher-dimensional abelian varieties, or succinct representations. It is not a general lower bound on ECDLP complexity.

## Two independently written exact verifiers

Run, from the repository root:

    python3 -B research/cross_characteristic_global_torsion_20261112/ct52/ct52_galois.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct52/ct52_galois.py
    node research/cross_characteristic_global_torsion_20261112/ct52/ct52_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct52/ct52_independent.js

Python checks the exact polynomial factors, all 48 combinations of 4-cycles and 3-cycles in S₄, the F₁₆ trace, and GL₂ orbits for ell=3,5,7,11. JavaScript separately implements polynomial divisibility, the trace and group-orbit enumeration, and a canonical S₄ generating pair. Both reject reducible-polynomial and trace-zero mutants. All four local executions passed; optimized/no-opt outputs match their respective normal executions. Exact-head GitHub CI is separate from local tests and scientific signing is pending.

## Next unresolved problem

Provide an input-dependent family of global abelian varieties A_(p,ell)/K_(p,ell) together with a **uniform, encoded-point** algorithm lifting arbitrary source P,Q=[x]P (without x) to compatible global torsion points, then reducing at characteristic two. Account for global field-degree and coefficient heights, the chosen conjugate, second-prime residue degree, and all arithmetic costs. A uniform lower bound in this varying-family setting would also settle this approach. The fixed-E theorem above does neither.

## Sources and limitations

Prior source: CT51 README and RUN_LOG, cross-characteristic PR #4012, root crypto PR #3877, problems/lift-reduce-second-prime-transfer.md; sibling crypto-autoresearcher and cryptanalysis via GitHub search. The shared research-KB MCP and its checked-out wrapper were unavailable, so duplicate/novelty assessment is limited.

External: A. Lozano-Robledo, *Division fields of elliptic curves with minimal ramification*, 2015, https://ems.press/journals/rmi/articles/13573 ; D. Zywina, *On the surjectivity of mod ell representations associated to elliptic curves*, 2022, https://doi.org/10.1112/blms.12701 ; Serre's open-image theorem. No paid compute launched, no benchmark or scientific PCC signature claimed.
