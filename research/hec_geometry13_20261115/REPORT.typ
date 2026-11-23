#set page(paper:"a4",margin:16mm)
#set text(font:"Helvetica",size:9.7pt,fill:rgb("233746"))
#set par(leading:.45em)
#text(size:22pt,weight:"bold")[Explicit plane quartics and descent]

Genus-3 quotient reconstruction · Exact gate and proof supplement · 15 November 2026

*Kernel 1 now has an explicit smooth plane-quartic model whose Jacobian is the quotient over F127.* Kernel 0 reconstructs the curve with the opposite Serre sign; its quotient is a nontrivial quadratic twist of that Jacobian. Both equations and outcomes are retained. Divisor transport is the next construction.

#image("QUARTICS.svg",width:100%)

*Geometric classification.* The verified absence of geometric quadrics excludes polarized products, whose complete level-two map has Segre minor relations. Oort–Ueno then gives a smooth genus-3 Jacobian over the algebraic closure. A hyperelliptic Jacobian would have a quadric relation, so both target curves are nonhyperelliptic.

*Exact jet construction.* The first public place (0,44) supplies nonzero rational 2-torsion with U₀=x³+15x²+90x+109 and V₀=0. Its kernel images have Hessian rank one; eight ordinary-point controls have rank four. In the formal Mumford chart V=t₀+t₁x+t₂x², the computed U₂ and U₄ satisfy the curve equation through degree four. Unframed quotient sections avoid division by zero theta values. Each constant-plus-quadratic matrix has rank seven, giving a unique order-four section. All odd jets vanish, all coefficients descend to F127, and both quartic jets are nonzero.

*Controls and smoothness.* Thirty-two jet multiplication comparisons, 32 inverse identities and a determinant control pass. All 54 scalar Weil evaluations agree at degree zero. Native Buchberger finds a unit affine singularity ideal for each quartic (11 basis elements, eight processed pairs); infinity gcd and endpoint tests exclude every remaining geometric singularity. Fermat and singular-quartic controls pass. Exhaustive projective counts are 127 and 129, with no statistical uncertainty.

*Identification beyond the numerical gate.* The characteristic-zero Γ₀₀ section contains C−C, so its nonzero leading quartic is the canonical curve. A smooth canonical-curve lift, lifted theta data, a unit rank-seven minor and the verified nonzero special jet transfer that identity to characteristic127. Functorial tangent-space reconstruction and Serre’s precise Torelli theorem leave only a {±1} cocycle. The source quotient trace is −1; the two exact counts then determine the signs. Counts alone would not identify a Jacobian. The full argument is in RECONSTRUCTION-PROOF.md; the original empirical false identification flag remains unchanged.

*Same-field equation.* In descending x order, the kernel-1 model is
#text(size:9pt)[119x⁴+44x³y+65x³z+5x²y²+107x²yz+58x²z²+100xy³+122xy²z+90xyz²+32xz³+109y⁴+98y³z+17y²z²+119yz³+z⁴=0 (mod127).]
Its Jacobian order 2,057,786 is derived from the proved identification and prior Weil polynomial, rather than new extension point counts. It contains the target prime 1,028,893. Kernel 0’s derived Jacobian order is 2,027,974.

*Custody and scope.* Protocol dfd2fa054e and its pre-run control amendment precede source 54661e56ddfd and checkpoint 20261115T152922Z-54661e56ddfd. Clean locked offline Rust build; original run exits successfully with empty stderr, no cap or rerun. All 560 section-jet coefficients, equations, node coordinates and hashes are retained. Arithmetic is shared; no independent external validation or performance ratio. Existing native CI does not run this standalone gate. No cloud worker, historical record, default or leaderboard change. The full mathematical 10× genus 2/3/4 objective remains active.

*Sources and next step.* Original Oort–Ueno Theorem 4; Müller Section 3; Pauly–Previato Γ₀₀ identity; Serre’s appendix Theorems 2/3/5 and LRZ Theorem 1.1.1. Exact links and hypotheses are in the proofs. Next: explicit source-to-kernel-1 divisor transport, a destination solver and complete charged costs.
