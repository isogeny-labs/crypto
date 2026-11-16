#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz transfers — R37
*Exact source classes and finite cofactor; no IC performance claim*

The actual deck order is bound through all four point coordinates, complete
83-power Frobenius orbits and680 saved literal function values. The signed
class E is evaluated in two independent expressions, with equal results.

#image("round37.svg",width:100%)

Both E and11E are nonprincipal. The official LMFDB coefficient data for the
complementary form77.2.l.b give P_b(-1)=3721=61². All16 Hecke-basis inverse
rows, the character/a4 recurrence and both independent matrix norms pass;
the complete Euler polynomial of degree32 is retained.

The classes3721E and61(61E) agree and are nonprincipal, while11·3721E is
principal. The predicted full mixed-carrier annihilator450241E is principal.
The final671E test is principal. With the preceding nonzero multiples this
gives exact source order671=11·61 and least positive cofactor61 to11-torsion,
under the retained proper-curve/local/complete-RR/pole bridge. The actual saved
class61E has order11. The finite projector122=2·61 is idempotent on this cyclic
group, with value one modulo11 and zero modulo61.

All subspace divisions use every original generator, with residual/kernel,
rank-nullity, recombination and final membership certificates. Every product
is checked by two different arithmetic implementations. These internal checks
are not an independent external scientific replay.

Lean: 9 audited declarations in 3 accepted runs;
2 rejected compile attempts are retained. The proofs check abstract
weight, torsion, source-order and finite-projector implications. Native arrays,
Picard implementation, normalization and factor maps are not formalized.

The intended CM-sign image remains unknown. Full quotient saturation, proper
polarized embedding, theta, node187, arbitrary collectors and larger supports
remain open. Native4GiB/1200s and pinned Lean-j1-M4096/1200s caps unchanged.
Global metric integration is pending exclusive path ownership; scoped patch
included. No timing comparison, IC relation rank or scalar recovery claim.

Sources: Khuri-Makdisi math/0409209v3; official LMFDB77.2.l / mf_hecke_nf;
retained R15/R16 CM Euler/sign obstruction and R27 mixed-Prym bridge.
