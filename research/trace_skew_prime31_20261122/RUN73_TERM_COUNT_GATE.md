# Run73 — exact Newton-moment sparsity gate (2026-11-22)

## Scope and status
This is an independently recomputed **input-polynomial representation** comparison for the Prime31 symmetrized index-calculus candidate. It does not run F4, extract an unknown public relation, recover a new Prime31 discrete logarithm, or establish an end-to-end cost or asymptotic advantage. Evidence level: local exact-symbolic; native Rust/CI not run for this new gate; PCC unverified.

Reproducible command (requires SymPy):
```
python3 research/trace_skew_prime31_20261122/term_count_gate.py
```
The local exact-symbolic execution passed all nine `(m,B)` configurations for `m=2,3,4`, `B=4,6,8`.

## Method
Write the elementary symmetric variables as `e_1,...,e_m`, and recover Newton sums `p_k=sum_i x_i^k` through
`p_k=e_1 p_{k-1}-e_2 p_{k-2}+...+(-1)^(m-1)e_m p_{k-m}` for `k>m`, with the standard `k e_k` correction for `k<=m`.
For a deterministic dense monic degree-B polynomial
`h(T)=T^B+2T^(B-1)+3T^(B-2)+...+(B+1)`, set
`H_j=sum_i x_i^j h(x_i)`, `j=0..m-1`.
Apply the invertible unit-triangular transformation
`G_j=sum_(a=0)^j (-1)^a e_a H_(j-a)`, with `e_0=1`.
Expand over the **integers**, count distinct nonzero monomials in each equation, and verify the identities exactly.
This is a single deterministic coefficient specialization, NOT a bound for arbitrary sparse factor bases or finite-field cancellation.

## Exact outputs
Each tuple is `(total polynomial degree, nonzero monomials)`.

| m | B | Raw H_j | Reduced G_j |
|---:|---:|---|---|
| 2 | 4 | (4,9), (5,11) | (4,9), (4,7) |
| 2 | 6 | (6,16), (7,19) | (6,16), (6,13) |
| 2 | 8 | (8,25), (9,29) | (8,25), (8,21) |
| 3 | 4 | (4,11), (5,15), (6,21) | (4,11), (4,11), (4,8) |
| 3 | 6 | (6,23), (7,30), (8,39) | (6,23), (6,24), (6,17) |
| 3 | 8 | (8,41), (9,52), (10,65) | (8,41), (8,44), (8,32) |
| 4 | 4 | (4,12), (5,17), (6,25), (7,34) | (4,12), (4,13), (4,12), (4,8) |
| 4 | 6 | (6,27), (7,37), (8,51), (9,67) | (6,27), (6,31), (6,29), (6,19) |
| **4** | **8** | **(8,53), (9,70), (10,92), (11,117)** | **(8,53), (8,62), (8,59), (8,39)** |

For `m=4, B=8`, the sum of nonzero input-polynomial terms is **332 vs 213**, a reduction of `119/332 = 35.843373...%`; maximum input degree falls from **11 to 8**. These are not sparse-Macaulay matrix counts, a resolving-degree bound, or timings.

## Prior art and overlap
- Existing radical-pair DRL study: `research/radical_pair_image_20260921/RESULTS.md`; do not present symmetric coordinates themselves as novel.
- Faugère–Huot–Joux–Renault–Vitse, EUROCRYPT 2014: symmetrized Semaev systems (DOI 10.1007/978-3-642-55220-5_3).
- #4924 owns trace/moment representation and the original-Rust pilot; #4920 owns chained S3; #4922 owns shared elimination; #4901 owns authentic RHS cold full recovery. This file stays in #4924 rather than duplicating another solver lane.

## Decisive next experiment
Run **same-engine native Rust F4** on original `H`, triangular `G`, and the quadratic Newton circuit, holding the **factor base, public Q, signed lift, target corpus, degree/memory deadline, monomial order and runner fixed**. Report matrix rows/columns, fill, solving degree, memory, complete successful/negative costs and independent `[k]G=Q` verification. Include existing DRL and the lab's strongest IC frontier, not only the raw ordered presentation. Promote to a complete-solver comparison only if a genuinely public-only unknown-index relation extractor passes.

**Unresolved:** no demonstrated cheaper complete non-generic ordinary-prime-field ECDLP solver.
