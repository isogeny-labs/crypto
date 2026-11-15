# Frozen degree-five cover search on the exact trace +26 pair

15 November 2026. This protocol extends the degree-four exclusion in `DEGREE4_THEOREM.md`. It is an exact-pair construction search, not a claim that any source class is vulnerable. Freeze the source and this protocol in a commit before executing the finite search. [pcc:unverified]

## Inputs and prior work

Let `K=F3[a]/(a^5+2a+1)`, `E/K: v²=u³+2u²+a`, and `Y/F3: y²=f(x)` with `f=x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1`. The selected subgroup has order 109. Prior geometric work excludes covers of degree at most three, the independently replayed exact-pair screen excludes degree four over K, and Howe Theorem 4.17 bounds some K-cover by degree twenty. The branch interpolation technique is published for genus-two quadratic-extension covers in [Fan 2022](https://onlinelibrary.wiley.com/doi/10.1155/2022/9833393); it is not claimed as a new method. [pcc:unverified]

## Exhaustive degree-five pole shapes

Translate a hypothetical degree-five map `Y_K→E` so infinity maps to the origin. Then `u=A/B` on the x-lines, `deg A=5`, `gcd(A,B)=1`, and `deg B=5-e∞` with odd `e∞∈{1,3,5}`. A finite branch root of `f` has odd pole multiplicity; a nonbranch root has even pole multiplicity. Over K, `f=(x+1)Q1...Q5`, with five irreducible quadratic factors. The rational branch root cannot occur in B because each allowed denominator degree is even. Up to monic scaling, the complete shapes are:

| `e∞` | shape | denominator count | 2-torsion assignments each |
|---:|---|---:|---:|
| 5 | `1` | 1 | `3^5` |
| 3 | `Qi` | 5 | `3^4` |
| 3 | `(x-c)^2`, `c∈K`, `c≠-1` | 242 | `3^5` |
| 1 | `Qi Qj`, `i<j` | 10 | `3^3` |
| 1 | `Qi (x-c)^2`, `c∈K`, `c≠-1` | 1210 | `3^4` |
| 1 | `H^2`, monic quadratic `H` coprime to `f` | 58,801 | `3^5` |

There are 60,269 denominators and 14,446,377 denominator/assignment cases. The 58,801 count is `243²−243−5`: exclude 243 quadratics divisible by `x+1` and the five `Qi`. [pcc:unverified]

At the rational branch root `x=-1`, a finite image must have the unique rational x-coordinate of `E[2]`. Each unpoled `Qi` has exactly three Galois-compatible x-coordinate choices in `E[2]`. Let `M=(x+1)` times the product of the unpoled `Qi`, and use CRT to obtain `h=A/B mod M`. Since `deg M≥7`, every possible numerator of degree five is uniquely `A=(B*h) mod M`. Retain only `deg A=5` and `gcd(A,B)=1`. For each survivor, test the full identity `f*C²=B*(A³+2A²B+aB³)` for a polynomial `C`; accept a map only after checking that identity and its degree. [pcc:unverified]

The `H²` shape may be solved without iterating 14.3 million products. For each of its 243 branch assignments, set `B=x⁴+c3*x³+c2*x²+c1*x+c0` and impose that coefficients of degrees 6 through 10 in `(B*h) mod f` vanish. These are five linear equations in the four `ci`. If rank four, solve the unique candidate and retain it only if B is a square of a monic quadratic coprime to f. For any consistent rank-deficient system, enumerate all 58,801 allowed H and test `B=H²` directly; if inconsistent, discard it. This exactly covers the same denominator/assignment space. The producer must report ranks, fallbacks, per-shape assignment counts, every degree-five interpolation survivor, every full-equation hit, and its own source SHA-256. [pcc:unverified]

## Acceptance and next gate

A hit requires an independently replayed polynomial identity, a checked degree-five map, and a nonzero order-109 pullback. The pullback lands in `J(Y)(K)`; to establish the intended GHS transfer, build an evaluable `F3`-rational correspondence into `J(Y)(F3)` and verify its action on the exact source subgroup. Charge construction, transfer, relation/decomposition, linear algebra, descent and verification against same-target rho before calling this a vulnerability. If the screen returns no hit, independently replay the necessary-condition exclusion before tightening the lower degree bound to six. [pcc:unverified]
