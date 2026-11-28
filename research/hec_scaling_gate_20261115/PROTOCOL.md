# Fixed-curve scaling amplification gate

Written before code/check execution. The zero-core cold result closes
elimination-only tuning for that cohort. Next screen: can the variable-change
amplification of split polynomials produce many same-curve principal relations?
No timing or novelty claim; full mathematical genus2/3/4 objective unchanged.

## Sources checked
KB ready lexical revision9141e386254340a9bdba3036f2bb706a,
last_refresh2026-11-14T21:44:48Z predates new studies. Queries:
`genus 2 index calculus relation generation sieving compact factor base faster`,
`genus two index calculus automorphism negation relation dependence smoothness no improvement`,
`"hyperelliptic" "automorphism" "relation"`.
Opened source/context/related for source:2c067e248f39333db0437966d7bd8a1d,
crypto-autoresearcher:knowledge/literature/KN-LIT-3845.md. It is an abstract-only
bulk note, not verified applicability. Related finite-field NFS note does not
supply a curve construction. Repo source:b3c90d39a347ef661ccf10680a742186,
crypto:research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md already
accounts for negation; do not count it as a new independent-relation multiplier.
Primary Joux EUROCRYPT2013 paper first successful PDF extraction plus Sections
3.1/3.2 search excerpts read at
https://www.iacr.org/archive/eurocrypt2013/78810175/78810175.pdf.
It uses variable change preserving polynomial splitting and a compatible
second side in a finite-field representation. Repeated PDF fetch/eprint fallback
returned403; no full-file checksum verification. We do not import its cost
claims into curves. The fixed-curve argument below is derived directly.

## Exact criterion
Assume odd prime p, monic squarefree f degree d=2g+1 and p does not divide d.
Write s=-coefficient(f,d-1)/d, F(X)=f(X+s), so F has no X^(d-1).
A fixed-curve affine transformation x=aX+t, y=bY (a,b nonzero) obeys
f(aX+t)=b^2 f(X). Comparing leading terms gives b^2=a^d. In centered
coordinates, comparing degree d-1 forces the translation to vanish. Thus
for each nonzero coefficient F_i with i<d, a^(d-i)=1. Let
K=gcd{d-i:F_i!=0}. Then a^K=1; conversely these equalities suffice.
Because d is odd, a^d is square iff a is square. Admissible x-scalings
therefore number gcd(K,(p-1)/2), with two y-signs per scaling. In particular
K=1 leaves only identity and existing hyperelliptic negation.
This class fixes infinity. No statement about general Mobius automorphisms,
correspondences, cross-curve transfers or non-affine relation families.

## Frozen finite checks
Native Rust, exact arithmetic only, no scientific Python/cloud/timing.
1. Four primes5/7/11/17, d5/7/9 with p not dividing d. Four fixed monic
patterns: X^d+1; X^d+X; X^d+X^(d-2)+X^(d-3)+1; and translation by1of the
third pattern. For EVERYa!=0 enumerate coefficient identity in ORIGINAL
coordinates with forced t=s(1-a), require a^d square; count must match gcd.
Patterns test algebraic identities regardless of squarefreeness; label them
polynomial controls, not asserted smooth curves.
2. All six reused family0fixtures across genus2/3/4 from point-sieve cohort,
no selection by result. Validate degree/characteristic conditions and apply
criterion plus direct original-polynomial enumeration. Record K, admissible
x-scalings, nonzero centered gap coefficients. Any failed condition is unknown,
not an absent automorphism. No group scalar, speed or relation-rank inference.
Stop this amplification route if K1all; otherwise analyze surviving maps and
rank/target action before timing. Fixed source/protocol commit precedes check.
