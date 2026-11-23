#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9.6pt)
#set par(leading: 0.6em)
#let card(body) = block(width: 100%, inset: 8pt, fill: rgb("eef3f8"), radius: 4pt, body)
#text(size: 21pt, weight: "bold")[Native genus-3 cover solver]

Implementation companion · 15 November 2026 · PR #4284

The continuation provides a Rust execution path for the exact historical
cubic-extension fixture. This page describes the implementation and its
bounded development contract. The original Python timing report remains
separate and unchanged.

#text(weight: "bold")[Fixed input model]

$k = F_1061$, $K = k[alpha] / (alpha^3 + alpha + 65)$.
The elliptic curve is $Y^2 = X(X-alpha)(X-alpha^1061)$, with supplied prime
subgroup order 298607311 and the archived generator. The genus-three curve
over $k$ has equation
$ y^2 = (t^3+t+65)(t^4-2t^2-520t+1). $
Parameters and transfer identity retain [pcc:unverified] standing.

#text(weight: "bold")[One public-target call]

#card[*Public point + relation seed* \
Only these two fields are accepted; a known scalar field is rejected.]
#align(center)[↓ fresh context]
#card[*Build the cover, factor base and normalized generator* \
Check polynomial map identities and rational factor-base images.]
#align(center)[↓]
#card[*Collect interpolation relations and solve over the subgroup field* \
Exact finite-field conjugate gradients; check original equations and every factor-base logarithm.]
#align(center)[↓]
#card[*Descend the target and recover its scalar* \
Pole-corrected pullback, Frobenius trace, Cantor arithmetic and shifted splitting. Check the returned scalar against the public point.]

#text(weight: "bold")[Bounded development contract]

Known-answer checks start a separate context for each of the first two saved
public targets. Other checks cover transfer homomorphism, the two-torsion
pole correction, identity and negative-generator targets, and rejected map,
relation or public-input mutations. The internal SplitMix64 stream differs
from NumPy; relation counts are not expected to match the archive.

Limits: one million interpolation candidates, twelve CG diagonals per attempt,
and ten thousand target shifts. Reaching a limit is an explicit failure.
This interface records no timings and does not establish a performance ratio.

#text(weight: "bold")[Publication and comparison dependencies]

New PCC result publication and merging await resolution of the divergent
hash-chained ledger. Neither history is selected or rewritten. Counted native
work, a matched strongest Pollard-rho reference, identity registration,
execution admission and signed independent replay remain pending.

#text(size: 8pt)[
Sources: native-check/src/solver.rs; native-check/DEVELOPMENT.md;
Joux–Vitse, Cover and Decomposition Index Calculus on Elliptic Curves Made
Practical, Appendix A.1. This implementation companion makes no novelty claim.
]
