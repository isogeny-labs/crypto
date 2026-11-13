# CT78 — Frobenius orbit census: the shortest target field is not a canonical oriented lift [pcc:unverified]

## Result and precise hypothesis

Continue CT77's exhaustive good binary elliptic reduction models and CT74–75's explicit source-local torsion lifts. For a binary elliptic curve with Frobenius trace t and odd prime ell, let F act on the **geometric, oriented** nonzero ell-torsion points E[ell]\\{0}. Frobenius has characteristic polynomial X²−tX+2 modulo ell. In the CT77 winning cases the actual action is conjugate to the companion matrix [[0,-2],[1,t]] mod ell: discriminants t²−8 are nonzero mod ell except ell=7,t=1, where the discriminant equals −7 and F is nonscalar because its integral endomorphism discriminant has ell-adic valuation one.

The **finite étale nonzero-ell-torsion algebra** has rank ell²−1 over the ground field. Over F2 it decomposes as a product of finite fields, one factor F_(2^d) for each Frobenius orbit of length d on E[ell]\\{0}. In particular, if c_d is the number of orbits of length d, its local factor-degree multiset is d repeated c_d times and \\sum_d d c_d=ell²−1. This is a statement about the **finite étale algebra**, not about a single irreducible global field: obtaining a point field of degree ell²−1 over Q additionally requires transitive global Galois image, and a naive integral primitive-element polynomial can have bad index at 2.

The minimum degree f_some from CT77 is the first d with c_d>0; it proves some nonzero ell-torsion exists in that residue extension. It does **not** make an arbitrarily selected global oriented point rational over that smallest field. The fraction of the ell²−1 geometric oriented points fixed by F^f_some is (ell^r−1)/(ell²−1), where r=dim_ell ker(F^f_some−I). The five CT77 winners all have r=1, giving **1/(ell+1)**. These are exact fractions of labeled geometric points, **not a search-cost lower bound or a probability distribution on implementable second-prime embeddings**.

## Exhaustive winning target spectra

| ell | winning trace t | f_some | f_full | Frobenius orbit degrees / multiplicities | fixed oriented points / all |
|---:|---:|---:|---:|---|---:|
| 3 | 0 | 1 | 2 | 1 × 2; 2 × 3 | 2 / 8 |
| 5 | −2 | 1 | 4 | 1 × 4; 4 × 5 | 4 / 24 |
| 7 | +1 | 3 | 21 | 3 × 2; 21 × 2 | 6 / 48 |
| 11 | −1 | 5 | 10 | 5 × 2; 10 × 11 | 10 / 120 |
| 13 | +2 | 3 | 12 | 3 × 4; 12 × 13 | 12 / 168 |

The ell=7 winning binary trace has a nontrivial ramified/Jordan action: two length-3 cycles support its small F8 subgroup, but two length-21 cycles account for the other 42 oriented points. Merely picking any of the other orientations cannot realize an F8 destination for that particular labeled point; for ell=11, 110/120 oriented points are not fixed after five binary Frobenius steps; for ell=13, 156/168 are not fixed after three steps.

**Negative/limitation check.** If a source generator P is represented by global torsion T, then a coherent target specialization at a single prime w always maps Q=[x]P to [x]T_w, regardless of the chosen orientation (CT73). A low-degree factor cannot be selected **independently** for P and Q, nor does the 1/(ell+1) count force any particular algorithm to try ell+1 candidates: computing the matching global prime and local maps is the still-missing operation. If all ell²−1 points are represented simultaneously in the full division algebra, one can see every orbit; this is not a proof of practical or uniform evaluation.

## Executable independence and reproduction

Python constructs the whole nonzero F_ell² vector set, iterates the genuine companion action and removes disjoint cycles. An independently written JavaScript implementation **does not enumerate vectors**: it computes F^d as a 2×2 matrix, calculates fixed subspace dimensions from ranks of F^d−I, and recursively subtracts proper-period fixed points (including the zero correction) to count cycles. Each covers every trace −2,…,+2 and ell=3,5,7,11,13 (25 pairwise cases), then validates all five CT77 winners separately. Explicit checks remain enabled under Python -O.

Both implementations agreed locally in Python normal/-O and Node normal/--no-opt. Thirty canonical JSONL records; SHA256 `e621037b773793defd1003e8e7ccda8acd7add015aba12fb09558fbbbf79880a`. These are **producer-local** arithmetic checks, not independent scientific PCC signatures. Scoped GitHub Actions status must be checked at the exact pushed head.

```sh
D=research/cross_characteristic_global_torsion_20261112/ct78
python3 -B "$D/ct78_cycles.py" >ct78-py.jsonl
python3 -O -B "$D/ct78_cycles.py" >ct78-pyopt.jsonl
node "$D/ct78_independent.js" >ct78-js.jsonl
node --no-opt "$D/ct78_independent.js" >ct78-jsopt.jsonl
cmp ct78-py.jsonl ct78-pyopt.jsonl
cmp ct78-py.jsonl ct78-js.jsonl
cmp ct78-py.jsonl ct78-jsopt.jsonl
echo 'e621037b773793defd1003e8e7ccda8acd7add015aba12fb09558fbbbf79880a  ct78-py.jsonl' | sha256sum -c
```

## Literature checked and boundaries

Read CT55/CT67/CT69–75/CT76–77, PR #3877 and #4015, dated feature session logs, sibling cryptanalysis PR #1523 and repo AGENTS. The finite-étale specialization and unramified Frobenius action are classical: A. Snowden, [Elliptic curves over DVRs, Lecture 8](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html), especially the n-torsion reduction corollary. The factor-degree/Frobenius-cycle correspondence is standard finite étale algebra theory and is compatible with Dedekind–Kummer subject to the good-index condition when replacing the étale algebra by a chosen integral polynomial. This result is a reproducible concrete refinement of CT77, not a novel general theorem.

Shared research-KB MCP, its checked-out wrapper, Conductor and Lean were unavailable in this tool session. No exhaustive prior-art claim. No change to active native Cantor, cryptanalysis or crypto-autoresearcher branches.

**Next gate:** compute the concrete prime w above 2 and the corresponding oriented global lift for both encoded source inputs under one coherent torsion representation, rather than relying on f_some availability. Measure factorization/recognition, heights, field arithmetic, specialization and destination solve against optimized rho and actual index-calculus protocols. No complete source→binary solver or measured speedup, paid EKS instance, protected merge, or signed PCC scientific verification. [pcc:unverified]
