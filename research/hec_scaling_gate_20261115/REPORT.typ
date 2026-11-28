#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.6em)
#text(size:23pt,weight:"bold")[Scaling amplification: fixed-curve gate]

Exact algebraic screen · Genus 2 / 3 / 4 · 15 November 2026

*All six fixtures admit only identity and negation within the tested affine class.*
A split polynomial can retain splitting under coordinate scaling while moving to a
different curve. Directly importing finite-field pinpointing therefore needs a
same-curve compatibility argument. This gate supplies that argument for affine maps.

#image("CRITERION.svg",width:100%)

*Derivation.* Let monic f have odd degree d, odd prime characteristic with p not
 dividing d. Set s = −coefficient(f,d−1)/d and F(X)=f(X+s). For
f(aX+t)=b²f(X), leading coefficients force b²=aᵈ, and the centered degree d−1
coefficient forces the translation to vanish. Thus every nonzero Fᵢ requires
 a^(d−i)=1. With K=gcd{d−i:Fᵢ≠0}, these equations are equivalent to aᴷ=1.
As d is odd, aᵈ is square exactly when a is square. The number of admissible
x-scalings is gcd(K,(p−1)/2); each has two y-signs. K=1 leaves a=1 and b=±1.

#table(columns:(1fr,1fr,1fr,1fr,1fr),align:right,
 [Genus],[Prime],[Centered gap 2],[Centered gap 3],[K / x-scalings],
 [2],[1009],[715],[922],[1 / 1],
 [2],[16381],[1094],[8497],[1 / 1],
 [3],[127],[16],[94],[1 / 1],
 [3],[1009],[51],[763],[1 / 1],
 [4],[31],[18],[17],[1 / 1],
 [4],[127],[6],[60],[1 / 1])

*Finite verification.* Frozen protocol preceded native Rust code. Scientific source
f76761dacf and checkpoint 20261115T121650Z-f76761dacff5 preceded execution.
Every nonzero a is checked by substitution into the original polynomial using
forced t=s(1−a), independently of the gcd criterion. All six fixture counts agree.
Forty polynomial controls over primes 5,7,11,17 and degrees 5,7,9 include
Xᵈ+1, Xᵈ+X, a dense pattern and its translate; p dividing d is excluded.
Some controls admit nontrivial scalings. Controls certify identities and are
not assertions that every control polynomial defines a smooth curve.

*Boundary.* This is a known-method applicability restriction, not novelty or a
runtime result. It covers infinity-fixing affine maps x↦ax+t,y↦by. It neither
classifies all Möbius automorphisms nor excludes correspondences, cross-curve
transfers or other relation-generation families. Negation was already used by
the factor-base orientation; it supplies no new independent-relation multiplier.

*Sources.* Joux, EUROCRYPT 2013, Sections 3.1/3.2, fixed-field polynomial scaling
and compatible second-side construction. Primary PDF extraction/search excerpts
were available; later PDF fetches returned 403, no checksum verification.
KB revision 9141e386254340a9bdba3036f2bb706a, source and gap receipts in PROTOCOL.md.
Evidence: CHECKS.txt, check.rs, SOURCE.txt and hashes.sha256. No performance
measurement or new benchmark admission. Next candidate needs a non-affine
same-target relation construction and a complete cost/rank case against compact.
