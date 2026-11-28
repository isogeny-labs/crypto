#set page(paper:"a4",margin:16mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Rational 2-isogeny kernel admission]

Exact structural gate · Genus 2 / 3 / 4 · 15 November 2026

*Both current genus-3 fixtures have no stable maximal isotropic 2-kernel.*
Thus a rational polarized degree-eight quotient of this type cannot supply
Smith’s plane-quartic route on those fixtures. Positive counts elsewhere
remain kernel admission evidence, with no map or runtime inferred.

#image("KERNELS.svg",width:100%)

*Exact model.* The 2-torsion is the vector space of even subsets of the
2g+2 branch points, modulo the all-ones class. The 2-Weil pairing is subset
intersection parity. Frobenius permutes branch labels; its cycles come from
finite irreducible-factor degrees plus infinity. Enumerate every dimension-g
isotropic subspace and test Frobenius-stability. Rational means stable as a
subgroup, not that each torsion point is rational.

*Independent obstruction.* For branch-cycle lengths c, Frobenius has
characteristic polynomial ∏(Tᶜ+1)/(T+1)² on 2-torsion. Let P=T²+T+1,
irreducible and self-reciprocal over F₂. Its primary component is
nondegenerate and splits orthogonally from the others. A stable Lagrangian
must intersect it in half its dimension. If P has odd multiplicity m, that
intersection would have odd dimension m, while every invariant subspace
has even dimension (all composition factors have degree two). Contradiction;
no semisimplicity assumption is needed.

The genus-3 cycle type (1,3,4) yields (T+1)⁴P, excluding both fixtures.
The genus-4 type (1,3,6) yields (T+1)²P³, excluding p127. The positive
p31 type (1,1,2,6) yields (T+1)⁴P²; enumeration gives three kernels.
Even multiplicity alone is not asserted sufficient.

*Checks.* Geometric Lagrangian totals 3,15,135,2295 match ∏(2ⁱ+1) for
g=1…4. Pair-generated subsets contain 3,15,105,945 spaces. All 22 genus-3
cycle types match Smith’s tractable-kernel table; pair-set and subspace
invariance agree. Control (7,1) has two stable full kernels and zero pair
kernels, preventing confusion between these conditions. All six fixed
fixtures pass polynomial factor-degree and fixed-torsion checks; 32 cases
in total. Scientific source 58eb56e46d and checkpoint
20261115T123803Z-58eb56e46dc1 preceded execution.

*Scope and next dependency.* Established theory, native Rust finite gate,
not novelty or a speed measurement. No destination curve or divisor map is
constructed. Genus-3 quotients need same-field descent and a verified
non-hyperelliptic destination; genus-4 quotients need not be Jacobians.
An explicit suitable 2-power map would preserve an odd-prime target subgroup.
Genus 2/4 remain in the full goal; this gate does not establish 10×.

*Sources and evidence.* Smith, arXiv:0806.2995v2, Sections 3/9 and Lemma 1.
KB revision 9141e386254340a9bdba3036f2bb706a; receipts in PROTOCOL.md.
CHECKS.txt, source, fixture/binary hashes, and RESULTS.md retain every case.
This standalone gate is local evidence and is not run by existing native CI.
