#set page(paper:"a4",margin:16mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Explicit genus-3 torsion kernels]

Existing p127 / family0 curve · Correctness and construction gate · 15 November 2026

*Both predicted rational 3-kernels now have explicit Mumford coordinates.*
Each contains 27 distinct 3-torsion classes over F(127¹³). Their cross sums
give all 729 classes of J[3]. Certificate replay and rejection controls pass.

#image("KERNELS.svg",width:100%)

*Field and order.* The first registered irreducible modulus is z¹³+z+10.
Prime-degree Rabin checks pass, with q=127¹³=2235879388560037062539773567.
Newton power sums and a separate companion-matrix/Bareiss determinant agree
on the extension Jacobian order N. Its exact 3-adic valuation is 6, so
N=729h. The public target order 1028893 divides N. Full integers are in
CERTIFICATE.txt and RESULTS.md.

*Construction.* The first registered non-base-field curve sample R satisfies
[N]R=0. Its cofactor multiple Q=[h]R is nonzero and killed by 3; both
certified Frobenius projectors produce nonzero images. The bases P,πP,π²P
produce 27 distinct combinations in each primary component. Every output is
reduced, valid, killed by 3, closed and Frobenius stable. Intersection is the
identity, and all 729 cross sums are distinct and killed by 3. No retry or cap.

*Isotropy proof.* The distinct dual irreducibles f=T³+2T+2 and g=T³+T²+2
annihilate the respective subgroups. Since 127≡1 mod3, Frobenius preserves
the 3-Weil pairing. There is no equivariant map from the f-primary module
to its dual, whose polynomial is g. Thus its pairing restricts trivially;
the same holds for g. Each subgroup has dimension 3 in dimension 6, hence
is maximal isotropic. This is a primary-module certificate, not a numerical
Miller-pairing evaluation.

*Controls and replay.* Sixty-four field controls check inverse by two methods,
field laws, Frobenius and square roots. 128 lifted scalar-pair controls match
the existing prime-field Cantor implementation, including identity, opposites
and doubling. Separate replay consumes the 54 stored Mumford records and
rejects nonmonic, duplicate, wrong-primary and public-D₁ substitutions.
Replay shares arithmetic with generation; it is not independent external validation.

*Scope and next step.* These are actual kernel coordinates, with no quotient
curve, target divisor transport or speed result. Both quotient constructions
still need a same-field/Serre check and complete cost against compact. A naive
combination of the source’s degree-12 branch field and degree-13 kernel field
uses degree 156; avoiding materialization requires a concrete algorithm.
All field/order/kernel setup remains chargeable. The full mathematical 10×
genus2/3/4 objective stays active.

*Custody.* Protocol 8a67a890db preceded source 0f197ceb2879 and checkpoint
20261115T134431Z-0f197ceb2879. Fixed public seeds, locked offline Rust dependencies,
raw coordinates, exact integers and hashes retained. No scientific Python,
cloud worker, performance benchmark or old record/default change. Existing
native CI does not automatically run this standalone gate. Primary Tian
arXiv:2007.03172 Section4; native Cantor source and source gaps in PROTOCOL.md.
