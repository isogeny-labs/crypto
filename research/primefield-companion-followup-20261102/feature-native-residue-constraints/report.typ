#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Build constraints in the retained quotient ring
T-1852 · 2026-11-07 · Prime-field compiled-mode recovery

The traced adapter now applies its first mode constraint before constructing its second. Once the monic retained state G has degree n, scaled modes are reduced modulo G and the second constraint squares only a residue. The public sign source and existing full-polynomial reference remain unchanged.

#image("flow.svg",width:100%)

*Exact identity.* Put A=m b^((r−1)/2) and t=(b^m)^(-1). The original constraints are C0=A F0(tZ)−f0 and C2=f1² F2(tZ)−A f2 F1(tZ)². Projection modulo G preserves these operations, and gcd(G,C)=gcd(G,C mod G). Thus sequential residue construction gives the same final monic state. The unit state stays unit. If initial C0 vanishes identically, valid compiled modes have m=r−1 and all modes are constant; an explicit scalar fallback handles C2.

*Blocked reduction.* Split each borrowed coefficient vector into chunks of n. Horner evaluation uses B=(tZ)^n mod G with coefficients −t^n G_i. This avoids a full transformed vector. For input length L there are ceil(L/n)−1 block multiplication/reduction pairs. Residues use at most n coefficients; explicit products at most 2n−1. One context serves both modes, followed by one residue square/reduction. Kernel internal workspace is separate; additional reductions can trade arithmetic for storage.

#table(columns:(2.4fr,1fr,2fr),inset:4pt,
 [Native deterministic check],[Count],[Outcome],
 [Scaled mode remainders],[1,440],[Equal full substitution/remainder],
 [Sequential mode states],[4,413],[Equal original full constraints],
 [Constant-mode controls],[6],[Zero C0, nonzero C2 gives unit],
 [Inherited implicit intersections],[8,600],[All assertions pass],
 [Additional public recoveries],[617],[5,929 calls; zero unresolved],
 [Inherited public comparisons],[63],[511 calls; assertions pass])

*Evidence.* validation01 receipt pins 42 transitive inputs and stdout; native overflow checks enabled. No compile or assertion failures. PCC-20261107-1112b3 is an unsigned producer statement, not an independent verdict. No timing measurement, cloud execution or full Cargo rerun. Original 4,563-case cohort and prospective source-trace protocol unchanged.

*Next dependency.* Compiled coefficient vectors and the first C0 remain dense. Next investigate streaming those coefficients or a recurrence representation; the public-source grid remains a separately charged cost.

*Sources and replay.* DERIVATION.md contains the proof, assumptions, exact replay command and limits. Sources checked: predecessor implicit-domain Modes/adapter, scalar polynomial kernel and FLINT nmod_poly modular arithmetic documentation. KB unavailable. Standard block Horner and quotient-ring projection are reused; no novelty claim. Historical reports and canonical graphs are unchanged.
