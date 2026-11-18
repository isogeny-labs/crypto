#set page(paper:"a4",margin:16mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Odd-prime kernel prerequisites]

Exact Weil polynomials · All 20 archived genus-2/3/4 curves · 15 November 2026

*Three curves have exactly two abstract rational 3-kernels each.*
All 20 Weil polynomials are uniquely certified. Eleven curves have a proved
modulo-3 obstruction; six remain unresolved by this screen. No new curve,
selector, target or timing was substituted.

#image("SCREEN.svg",width:100%)

#table(columns:(1.5fr,1fr,1fr,1fr),align:right,
 [Positive curve: genus / p / family],[Dual factor degrees],[Abstract kernels],[Point field degree],
 [2 / 16381 / 1],[2 + 2],[2],[8],
 [3 / 127 / 0],[3 + 3],[2],[13],
 [4 / 127 / 1],[4 + 4],[2],[80])

*Constructive algebra.* The genus-3 characteristic polynomial modulo 3
factors as f=X³+2X+2 and g=X³+X²+2. Each simple primary component is a
stable Lagrangian, giving the two abstract kernels. Frobenius has order 13
on both. Complementary operators on J[3] are
E₀=1+2π+2π²+π⁴ and E₁=π+π²+2π⁴. Exact polynomial checks give
Eᵢ²=Eᵢ, E₀E₁=0, E₀+E₁=1 and fE₀=gE₁=0 modulo fg.

*Domain matters.* These operators separate kernels only after obtaining
3-torsion. Their integer lifts do not turn an arbitrary DLP point into
3-torsion. No Mumford kernel coordinates or Jacobian isogeny were constructed.
The field degrees are exact periods of individual nonzero kernel points,
not lower bounds for all isogeny algorithms; compressed orbit methods may
avoid explicit materialization. A genus-4 quotient need not be a Jacobian.

*Next dependency.* For genus3/p127/family0, obtain and validate 3-torsion over
Fp¹³, then apply the certified projectors. Price that setup before claiming an
advantage from a quotient or faster destination solver. Genus 2 and 4 remain
in the full mathematical 10× objective; these prerequisite results do not
establish that gain.

#pagebreak()
#text(size:20pt,weight:"bold")[Proof, controls and custody]

*Unique reconstruction.* Write the L-polynomial as
L(T)=1+a₁T+⋯+a[g]Tᵍ+⋯+pᵍT²ᵍ, with reciprocal coefficient
p^(g−j)aⱼ. Newton identities recover a₁ from C₁, a₂ from C₂ and a₃ from C₃.
Only counts through degree g−1 are needed. The missing middle coefficient
has bound |a[g]|≤binomial(2g,g)p^(g/2). Registered integer upper bounds are
6p, 20p ceil(√p), and 70p² for genera 2,3,4.

Each pinned public D₁ is valid, nonzero, and annihilated by a primality-checked
ell. Thus ell divides the actual Jacobian order L(1)=S+a[g]. Enumerating all
integers in the bound congruent to −S modulo ell leaves exactly one candidate
on every curve. This certifies the missing coefficient. All order, divisibility
and reciprocal-polynomial checks pass, with no probabilistic order inference.

*Exact point counts.* Extension characters use χ(z)=χₚ(Norm(z)), with exact
quadratic/cubic norms and a base character table built from squares. Real
extension samples total 7,226,370 elements, below the frozen ten-million
budget; no cap, error or retry. Clocks enforce only the 600-second resource
cap and are not reported as performance measurements.

*Independent controls.* Euler exponentiation agrees with norm characters
on all 1,994 elements over p=5,7,11 in degrees 2 and 3. Nine smooth genus-2
curves match zeta-derived orders against exhaustive reduced Mumford
enumeration. Forty-three congruence and two factor/module controls pass.
Six exact root-order checks and both projectors per positive curve pass.

*Kernel criterion.* The twisted dual of a monic degree-d factor P is
XᵈP(p/X)/P(0). A self-dual even-degree factor of odd multiplicity obstructs
a stable Lagrangian: its nondegenerate primary part would require a
half-dimensional invariant subspace whose dimension is not divisible by d.
For squarefree polynomials consisting of distinct dual pairs, choosing one
primary component per pair gives exactly 2^(pair count) abstract kernels.
All other cases stay unresolved in this screen.

*Unknown is not zero.* The first raw schema printed numeric 0 alongside
unknown status on six cases. That is a placeholder, not kernel absence.
Native reduction maps these counts to null; future source prints null.
Original raw outputs and source remain intact; point counts were not rerun
just for formatting.

*Custody.* Trace source da1f554b0cf6; order source 56ffb219aa1a; projector
source bd0b987de8fb. Each source/protocol was checkpointed before its execution.
Raw counts, all coefficients/factors, source/input/binary hashes, projectors
and native jq summary are retained. Existing native CI does not automatically
execute these standalone scientific gates. No old benchmark, default or
admitted standing changed. Native Rust/jq only, no scientific Python/cloud.

*Sources.* Milio arXiv:1709.06063 and Tian arXiv:2007.03172 explicitly take
an isotropic torsion kernel as input; same-field destination and divisor
transport still need checking. MIT Sutherland’s Jacobian gallery supports
subgroup/order verification; the functional equation here is derived directly.
KB/access receipts are in PROTOCOL.md. ecdb read credentials were unavailable;
pinned fixtures were used. Established methods, no novelty or speed assertion.
