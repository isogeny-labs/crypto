#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Keep the initial image domain implicit
T-1850 · 2026-11-07 · Exact scalar-field recovery representation

Removed the unconditional dense initialization of U(Z)=Z^(d+1)−Z, d=(r−1)/m, from the traced recovery path. The initial state now holds only parameters until a nonzero constraint arrives. This targets an allocation before observed constraints and before the fiber cap, independently of the source quotient trace.

#image("flow.svg",width:100%)

*Proof.* Gcd(U,C)=gcd(C,U mod C). Binary modular powering supplies the required residue without expanding U. Zero constraints retain implicit state; nonzero constants produce the unit polynomial. Subsequent dense intersections are unchanged. Induction gives the same state after every admitted constraint sequence, including repeated or nonmonic constraints.

*Cap and costs.* Implicit pullback is U(X^m)/X^(m−1)=X^r−X. Reject degree r greater than cap before allocating. First intersection with degree-c constraint uses residues below c and multiplication outputs at most degree 2c−2. Powering costs floor(log2 N) squarings plus popcount(N) multiplications/reductions, N=d+1, then gcd. This trades modular-power work for avoiding the full-domain array; no universal timing dominance is claimed. For m≤(r−1)/2 the first mode constraint has degree floor(d/2), with nonzero leading coefficient.

#table(columns:(2fr,1fr,2fr),inset:4pt,
 [Deterministic native check],[Count],[Result],
 [Constraint intersections],[8,600],[Exact dense-gcd and pullback agreement],
 [Zero stages remaining implicit],[2,192],[No dense state created],
 [Cap rejection controls],[3,113],[Limit returned],
 [Large polynomial helper],[N=2^60],[No U expansion; exact linear intersection],
 [Additional public recoveries],[617],[5,929 calls; zero unresolved],
 [Inherited public comparisons],[63],[511 calls; same results/routes])

*Scope.* The large helper uses prime r=2^61−1 and C=Z−1; it is not a large-order public-source recovery. Mode coefficient construction and the coordinate-field source grid remain separate costs. Runtime target access remains public-point-only. Original 4,563-case cohort and the prospective trace protocol are unchanged. No cloud, timing or full Cargo rerun.

*Evidence and next construction.* DERIVATION.md supplies invariants, costs, assumptions and replay command; native receipt pins dependencies and output. PCC-20261107-83d4e9 is unsigned producer evidence. Sources: exact predecessor source/recovery/gcd and FLINT modular exponentiation; KB unavailable. Next develop a compact or streamed first-constraint representation, since the initial domain now intersects at that constraint's size. Historical evidence and canonical research graphs remain unchanged.
