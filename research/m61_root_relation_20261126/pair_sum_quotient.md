# Pair-sum quotient algebra for six-sum relations: a concrete failed route

Research note, 2026-11-26. This spells out a coordinate-based mechanism beyond the cyclic-norm folding discussed in `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4.6. It is an **implicit representation**, not an `O~(p^(1/3))` relation generator. Summation-polynomial resultants originate with Semaev (2004), and quotient-algebra multiplication matrices and subresultants are established approaches to ECDLP polynomial systems; this representation is not claimed as new. A separate working memo had already derived this same `D₂,G,D₃` construction before its inclusion here.

## Construction

On `E/F_p: y²=x³−3x+40`, `p=2^61−1`, use the 2,447 valid x-coordinates in `μ_4950`, with both point signs. Write `B=2447`. Form the monic pair-sum polynomial `D₂(U)` from `x(P_i+P_j)`, `x(P_i−P_j)` for `i<j`, and `x(2P_i)` for each `i`. It has exactly `B²` factors counting multiplicity. Set

`F₃(u,v,w)=(u−v)²w²−2[(u+v)(uv−3)+80]w+(uv+3)²−160(u+v)`

and `G(U,X)=∏_{k=1}^B F₃(U,x_k,X)`. In the quotient algebra `A₂=F_p[U]/(D₂(U))`, multiplication by `G(U,X)` has determinant

`D₃(X)=Norm_{A₂/F_p}(G(U,X))=Res_U(D₂(U),G(U,X))`.

Every finite signed triple sum with distinct columns supplies an x-root of `D₃`. The quotient algebra has dimension `B²`, while `G` has bidegree at most `(2B,2B)`. Hence the *input* to this determinant is describable with `O(B²)` field elements. At a fixed numeric `X=t`, the known pair roots give the determinant product formula `D₃(t)=∏_{u:D₂(u)=0}G(u,t)` (with multiplicity), evaluable in `O~(B²)` field work. For a liftable `t`, however, the existing pair-sum hash already tests membership and returns a witness in `O(B)` group operations by probing `x(T±P_k)` for each `k`, where `x(T)=t`. The determinant adds no pointwise speed.

## Full ledger and exact expansion point

| Stage | Time | Memory | Status |
|---|---:|---:|---|
| Enumerate `μ_4950` and lift-valid roots | `O(B log p)` after finding the order-4950 generator | `O(B)` | Executable |
| Build signed pair-sum roots and hash/product tree | `O~(B²)` group/field operations | `O(B²)` | Executable |
| Describe `G(U,X)` and `A₂` | `O~(B²)` field operations | `O(B²)` | Symbolic circuit |
| Test one supplied triple-sum candidate | `O(B)` group operations after pair hash | `O(B²)` shared | Executable |
| Materialize the triple-sum determinant `D₃` | `Ω(B³)` coefficients/time | `Ω(B³)` | Exceeds budget |
| Direct three/three target matching | `Θ(B³)` triple generation and hash work | `Θ(B³)` | Exceeds budget |
| Direct three/four seven-term relation matching | `Θ(B⁴)` quartet generation | `Θ(B³)` triple table | Exceeds budget |

The determinant has degree between `2B³−B²` and `2B³`, counting multiplicity. At this instance, `B²=5,987,809`, `B³=14,652,168,623`, and the lower degree bound is **29,298,349,437**. A dense coefficient vector is already 4,893 times larger than the `B²` budget. To locate a six-sum, the direct target-bearing operation is a nested resultant of `D₃(X)` with its target translate under `F₃`; its intermediate polynomial has degree `Θ(B³)` even before witness extraction. Cofactor projection replaces one target by its seven torsion translates, a constant-factor coverage gain but no dimension reduction.

The most optimistic *squarefree* compression also has no obvious source. In a random-sum model at `B≈p^(1/6)`, accidental pair-sum collisions are `O(B⁴/p)=O(B^(−2))`; for this finite instance `B⁴/p≈1.55·10^−5`. This is a heuristic, not an exact collision count for the root base. It predicts that nearly every pair root remains distinct and the quotient algebra truly needs about `B²` coordinates.

**Selected-instance update (2026-11-27):** the [exact native pair census](pair_sum_census.md) finds all `5,987,809` counted x-roots distinct and no return to the root base. Thus `D₂` is squarefree with degree exactly `B²` on this M61 fixture. The conclusion remains finite-instance only; the earlier heuristic magnitude is a rough `B⁴/p` scale, whereas the exact independent-x expectation for collisions among `B²` entries is `C(B²,2)/p≈7.77×10⁻⁶`.

## What would have to change

A successful next primitive would take the compact `(D₂,G)` circuit and target `R` and **locate** a common root of the two triple-sum images, including signed witnesses, in `O~(B²)` total work and memory, without materializing `D₃` or scanning `Θ(B³)` triples. A concrete possibility to test is a structured subresultant or displacement-rank method acting directly on the multiplication-by-`G` operator. It must demonstrate that the target-dependent intersection can be solved from `O(B²)` state; fast determinant evaluation alone is insufficient. On increasing toy primes, measure intermediate dimension, arithmetic count, actual recovered relations, and rank after cofactor projection. Failure to beat cubic growth on that intersection falsifies this *specific quotient-algebra route*, not coordinate-based index calculus in general.

Prior-art pointers: Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), https://eprint.iacr.org/2004/031.pdf; and *Complexity bounds on Semaev's naive index calculus method for ECDLP* (2020), §3.4, https://doi.org/10.1515/jmc-2019-0029. The precise `D₂,G,D₃` calculation was also present in the earlier local `novel_implicit_triple_divisor.md` research memo.
