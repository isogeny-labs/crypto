#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Character-signature exclusion and sparse selection
2026-11-03 · Derived theorem / source construction · Numerical trials: zero

*Structural bound.* Outside q exceptional character-zero points, the q shifted
quadratic characters have binary signs. Thus K = min(r, 2^q + q) bounds the
number of signatures. Any function of the signature agrees with X at most
once per signature fiber, hence at most K times. If K < r, representation of
X is excluded at every polynomial degree, independently of Gram weights.

#image("pipeline.svg",width:100%)

*Public selection.* A bounded streaming routine evaluates the original cubic
features at supplied points. It retains independent rows and their original-row
combinations, applying the same elimination to target values. A zero feature
residual with nonzero target produces a normalized sparse dual. A separate
checker recomputes all moments and the pairing from public support.

*Progress proof.* At each nonterminal state, some feature combination fits
the retained independent rows. At least r−K points disagree with that fit.
Each such point either raises feature rank or produces a certificate. Under
independent uniform point sampling and K < r, conditional progress probability
is at least delta=(r−K)/r. Expected offers are at most (m+1)/delta, where
m=1+q+C(q,2)+C(q,3). No independence of character signs is assumed.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived quantity],[Bound / schedule],
 [Witness support],[At most m+1 public points],
 [Elimination at rank t],[t(m+2)+t(t−1)/2 products and subtractions],
 [Stored field entries],[tm+t+t(t+1)/2],
 [Deterministic sufficient stream],[K+1 distinct points, only if K < r],
 [Sampler / stopping],[Caller-supplied stream; explicit finite cap])

Feature evaluation, normalization, inversions, native work and independent
checking are additional and accounted in DERIVATION.md. The theorem precheck
needs no field operations. Exhausted samples mean inconclusive. No automatic
field enumeration, random sampler or signature collision table is introduced.

*Applicability.* The saved F479 width-15 failure has K=r, so the strict bound
is inconclusive there. Its earlier dual certificate remains necessary. The
retained recovery panels start outside this guaranteed regime; no saved-panel
speed improvement is claimed. Positive represented programs remain separate.

*Evidence / next.* Syntax and preserved source bytes checked statically;
no numerical, corruption, heap or performance trials. Next exploit cubic degree
to sharpen agreement or certify an interpolant when K=r. A sampled fit alone
cannot establish a representation valid on the full field.

*Sources.* NIST DLMF 27.9 (Legendre alphabet including zero); preserved
feature-span-dual report and coefficient construction. Cardinality and progress
arguments are derived here, without a novelty claim for general row elimination.
