# Translated S3 divisor: doubled-target squarefree criterion

Date: 2026-11-23 UTC. Parent PR: #4977; immutable pencil core `d6e1d03aff64ff168bd1d1d565a08c8f5032ef62`. Status: **written mathematical derivation and bounded abstract-group checks only, not independently validated or a faster query**. [pcc:unverified]

## Precise candidate equivalence

Let `E/F_p` be nonsingular, `p>3`, and let `G <= E(F_p)` have **odd order n**. Put `A={±P_i}` with rational nonzero leaves, and let `U=(A+A)\{O}` be a sign-closed set of `2D` distinct nonzero points. Let `X=x(U)`, `H(V)=prod_(u in X)(V-u)`. The original pair labels/backpointer multiplicities remain separate from this set of points.

For a **nonzero** `R in G\U`, put `r=x(R)` and form the degree-`2D` polynomial

```
P_R(V) = product_(T in U) (V-x(R-T))
       = H(r)^(-2) product_(u in X) S3(u,V,r).      (1)
```

The equality uses the previously derived translated-divisor identity; `H(r) != 0` is essential. Every factor is defined over `F_p`, since `R-T != O`.

Define the signed *ordered pair count*, not the distinct-abscissa count,

```
N_U(S) = #{ T in U : S-T in U }.
```

**Candidate lemma.** If `R != O` and `R not in U`, then every root of `P_R` has multiplicity one or two and

```
deg gcd(P_R, P_R') = N_U(2R)/2,
deg rad(P_R)       = 2D - N_U(2R)/2.                (2)
```

In particular, `2R in U+U` iff `P_R` is **not squarefree**. No signed relation is inferred just from a random candidate root; witness replay is explicit below. [pcc:unverified]

**Proof.** Translation `T -> R-T` is injective, so its image `Y_R=R-U` has `2D` distinct nonzero points. For a nonzero point `Q`, the fibre of the abscissa map is exactly `{Q,-Q}`; thus any root `x(Q)` occurs at most twice. It occurs twice precisely when both `Q` and `-Q` lie in `Y_R`. This is equivalent to

```
R-Q in U  AND  R+Q in U.
```

Those two points sum to `2R` and are distinct: equality would give `2Q=O`, impossible for nonzero `Q` in an odd-order group. Conversely any ordered solution `T+T'=2R` with `T,T' in U` yields `Q=R-T` and `-Q=R-T'`. There are no diagonal solutions because `T=T'=R` would contradict `R not in U`. Hence each repeated abscissa corresponds bijectively to one unordered pair of two distinct signed points, giving exactly `N_U(2R)/2` repeated roots. Their multiplicity is exactly two, so in characteristic not two each contributes degree one to `gcd(P_R,P_R')`. This proves (2).

**Target reduction.** Given any nonidentity target `S in G`, compute its unique group half `R=[(n+1)/2]S` (charged scalar multiplication). If `R in U`, then `S=R+R` is already a signed four-leaf relation via the existing pair backpointers; the `S3` polynomial has a forbidden `O` intermediate and equation (1) must not be applied. Otherwise, the squarefree test (2) decides exactly whether `S in U+U`. For `S=O`, sign closure of any nonempty `U` gives a relation directly. This is an **existence** oracle, not by itself all-output extraction.

**Witness extraction.** Each repeated root `q` has two rational lifts `Q,-Q`; check `R-Q` and `R+Q` against the actual signed-point dictionary `U`. They provide the unordered pair `{T,T'}`, with `T+T'=S`. Expand their original pair backpointers to four signed leaves, normalize duplicate reports, replay the group equality, then evaluate useful independent rank in the actual relation matrix. No scalar labels or discrete-log hash are assumed. A distinct requested four-leaf output can multiply through the backpointer buckets, and its cost must be charged.

## Why this does not yet accelerate empty queries

The reduction exposes a different, exact **candidate indexed primitive**: squarefreeness of the structured, target-translated degree-`2D` divisor, including misses, rather than a componentwise zero test for the rank-three pencil. It does **not** establish that the new primitive is easier. Building `P_R` from the `D` S3 factors by a product tree and taking a polynomial gcd costs `O~(D)` field work, with `O(D)` stored coefficients and a separate `O(log n)` scalar multiplication for halving. This is the same linear-in-`D` exponent as the exact hash-complement scan; a direct dense discriminant or explicit target certificate must also pay for its coefficients. The complete empty query has `N_U(S)=0`, hence must certify `gcd(P_R,P_R')=1` without pretending that checking a planted positive suffices.

In the displayed output model, a full Bezout certificate for a degree-`2D` polynomial can have `Omega(D)` coefficients; shorter proof traffic or faster structure-specific algorithms are not ruled out. Extracting all repeated rational roots needs a separate priced root-isolation procedure. The exact baseline is the existing original-arithmetic pair scan, not a deliberately slow quadratic resultant.

A prospective result would supply an index of `O~(D)` setup/storage with **empty-inclusive** `O~(D^theta+h)` squarefreeness/repeated-root queries, `theta<1`, where `h` prices output and verification. A fixed `theta<1/2` remains the separately stated conditional complete-pipeline target when `D=Theta(B^2)`. In particular, taking a discriminant, norm, ordinary rank, or polynomial derivative **symbolically** is not an algorithmic speedup. No such index is claimed here. [pcc:unverified]

## Checks, exceptions and source provenance

An independent-from-repository combinatorial enumeration (not cryptographic Rust) checked all nonempty sign-closed subsets `U` in the odd cyclic groups of orders `7, 11, 13, 17`, and every `R not in U`: 25 + 181 + 435 + 2287 = **2928** source/target pairs. Each checked both multiplicity bound `<=2` and the equality `2*collision_count=N_U(2R)`. These are producer checks of the abstract group formula; they are **not** independent ECC, signed-PCC, Lean or original-source verification. [pcc:unverified]

Special cases retained: `R in U` produces a zero subtotal and invalidates the affine product (1); `S=O` has its own group branch; odd group order prevents rational nonzero two-torsion and makes halving unique; `p>3` retains the standard S3 polynomial; zero and nonrational pair intermediates are not silently introduced. Repeated signed backpointers are not counted as distinct elements of `U` but are restored during all-output reporting. If a future candidate processes only `x(S)`, it must explain the unique `x(R)` obtained after halving either sign of `S`.

**Sources checked:** iteration9/PAIR_SPECTRUM_RESULTANT.md, iteration10/S3_PENCIL_CONTRACTION.md (notably translated divisor (8)), quotient_followup/DICKSON_PAIR_NORM_REVIEW.md, quotient_degree_followup/BOUNDED_DEGREE_QUOTIENT_GATE.md, and problems/prime31-exact-pair-spectrum-query.md. Primary context: [Semaev 2015](https://arxiv.org/abs/1504.01175); [Dinur and Golovnev, ICALP 2026](https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html) concerns integer 3SUM indexing, not a free elliptic group reduction; [Chan–Itakura–Ma–Yu, 30 October 2026](https://arxiv.org/abs/2610.42012) concerns integer-input 3SUM, not a directly applicable elliptic DLP-free hash. Shared KB tools and their repository CLI fallback were not available in this runtime; no novelty claim is made.

**Ownership and next gate:** no native child worker was exposed. The coordinator completed read-only algebra and a disjoint abstract-group validation preparation; Rust changes were not selected or attempted. The production Rust/workflow/STATE of any other live lane and sibling PR #4979 remain unchanged. An independent reviewer should challenge the squarefree-gcd identity and then a future admitted original-Rust implementation could compare a source-frozen squarefree oracle against the full signed complement scan on misses, duplicates and target halves. Derive a genuine amortized or single-query sublinear bound **before** promoting a solver claim. [pcc:unverified]
