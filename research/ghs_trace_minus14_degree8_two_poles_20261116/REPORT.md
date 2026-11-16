# Trace −14 degree-eight two-pole cover screen

The frozen exact search completed all eight representative carrier/source cells. It found zero maps of degree eight with denominator `B = L Qi Qj` or `B = L³ Qi Qj`, where `L` is the unique linear factor of `f` over K and `Qi,Qj` are distinct quadratic factors. This is a shape-specific negative result. It does not prove absence of degree-eight covers or attack vulnerability.

## Exact workload

- K = F3[a]/(a⁵+2a+1).
- E_b/K: v² = u³+u²+b. The four b masks are 11, 202, 38, and 80; each has #E_b(K)=258.
- Y_f/F3: y²=f(x). Carrier masks are 9735 and 17371. The eight cells represent 240 exact source/carrier equation pairs through the earlier source-Frobenius and carrier-PGL2 orbit census, not 240 independently rerun cells.
- Search identity: `f C² = B(A³+A²B+bB³)`, with `gcd(A,B)=1`, `deg A=8`. The source file enumerates ten quadratic-pole pairs and the two denominator shapes per cell.
- Source `screen.sage` SHA-256: `b06a004a8654cab2686c1870a311af31abef4083e24289753f86230606e0d75e` (frozen in commit `41e93427bf2a`).

| f mask | b mask | Point-filter survivors | Coprime and divisible | Square hits | CPU s |
|---:|---:|---:|---:|---:|---:|
| 9735 | 11 | 6815 | 6778 | 0 | 72.32 |
| 9735 | 38 | 6721 | 6694 | 0 | 71.77 |
| 9735 | 80 | 6934 | 6906 | 0 | 69.17 |
| 9735 | 202 | 6827 | 6802 | 0 | 71.90 |
| 17371 | 11 | 6133 | 6108 | 0 | 67.49 |
| 17371 | 38 | 6085 | 6058 | 0 | 63.52 |
| 17371 | 80 | 6164 | 6133 | 0 | 64.09 |
| 17371 | 202 | 6059 | 6033 | 0 | 64.25 |

Each cell represents 3,858,261,660 logical candidates through exact bitsets over ν. All eight together represent 30,866,093,280. The point filter admitted 51,738 candidates; every survivor received direct gcd, divisibility, and squarehood checks. The code verifies the full polynomial identity on every squarehood hit. The source additionally checks 10,800 bitset-versus-direct point evaluations per cell. Aggregate CPU time was 544.51 s. All eight processes exited zero; `audit.py` verifies hashes, schemas, counts, raw stderr completion markers, and zero hits; `DegreeEightTwoPoleCounts.lean` compiles with Lean and checks only arithmetic and representative counts.

## Scope and next gate

The earlier [complete degree-seven census](../ghs_trace_minus14_degree7_orbits_20261116/REPORT.md) gives a degree window [8,33] for the 240 equation pairs. The three [single quadratic-square screens](../ghs_trace_minus14_degree8_quadratic_20261116/REPORT.md), eight branch-supported sparse shapes, and these two-pole shapes remove named parts of degree eight only. Their union does not constitute an exhaustive degree-eight classification. The remaining denominator shapes include more complicated branch poles and nonbranch even poles; a correspondence could also exist without a low-degree map of the searched form.

No positive cover was produced here, so there is no subgroup-preserving transfer map or measured full GHS-versus-rho attack cost for this trace −14 family. Novelty cannot be assigned to a vulnerability from the negative screens. The published [odd-characteristic direct GHS classification](https://eprint.iacr.org/2015/805.pdf) and [Howe's cover bounds](https://arxiv.org/abs/2110.04221) remain the relevant comparison, and the [Joux–Vitse isogeny-assisted attack](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) is prior art for transfer by cover and isogeny.
