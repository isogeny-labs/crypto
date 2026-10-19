# Four additional ECDLP experiments: geometry gates

2026-10-19. This is a second lead search after the [five-lead follow-up](../five_leads_e2e_20261019/REPORT.md). “Additional” means a distinct experiment for this workstream, not new mathematical priority. All numbers here are exact finite-field **geometry gates** or eligibility checks. They are not end-to-end DLP measurements, instruction counts, or speedups. The earlier 1,260-target genus-2 incumbent and source negation-rho results remain the comparison baseline.

| Lead | Distinct mechanism | First gate run here | Next discriminating test |
| --- | --- | --- | --- |
| Q1. Quartic point-incidence sieve | Replace repeated line-intersection factoring with a point-pair/line-incidence table on a plane quartic | Exact four-point line census on both published Tian equations | After an executable elliptic-to-quartic map exists, compare complete one-target harvest, linear algebra, setup and verification against Diem-style line testing and source rho |
| Q2. Dynamic small-prime base for quartic relations | Change which rational points count as small primes as the large-prime graph develops | Fixed-base eligible-line populations at three sizes, on the same two equations | Freeze a target-blind promotion rule on disjoint training curves; charge promotions, graph work, duplicate lines and rank updates against fixed bases and rho |
| G1. Genus-2 pencil sieve | Sweep the functions `y-v(x)-λu(x)` for a transferred divisor instead of testing only whether its current quadratic `u` splits | All 257 pencil members on each of 20 fixed divisors on the Scholten quintic | Produce verified target-labelled three-point relations, then compare full one-target cost with the existing two-point collector and source rho |
| T1. Direct trace-zero index calculus | Attack the trace-zero subvariety of an elliptic curve defined over a cubic extension's base field, without constructing a Jacobian quotient | Exact `j`-invariant gate rejects the published Tian cubic fixture | Freeze a separate base-defined `E/F_q` cohort with prime-order trace-zero subgroups; solve and price its decomposition systems and matched Frobenius-aware rho |

## Q1 and Q2: plane-quartic line geometry

Vitse and Wallet's [point-incidence sieve](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf) replaces many polynomial splitting tests by incidence arithmetic. For a smooth quartic, a nonvertical line with four distinct affine rational intersections gives a principal divisor relation against a fixed reference line. This first gate reuses the two published equations whose smoothness was checked in the five-lead study. It enumerates every affine rational point, groups point-slope incidences, independently recovers each line from point pairs, and re-enumerates polynomial roots for every four-point line. Tangent multiplicities, vertical lines and intersections at infinity are outside this census.

| Equation | Affine points | Complete four-point lines | Lines with at least two fixed small points: 1/8 · 1/4 · 1/2 base |
| --- | ---: | ---: | ---: |
| Tian `F_523` | 522 | 11,090 | 818 · 2,833 · 7,610 |
| Tian `F_257` | 250 | 2,451 | 190 · 634 · 1,687 |

The fixed bases are the first floor(`fraction × affine points`) under the SHA-256 point ranking in [the script](quartic_line_gate.py). The full `k=0…4` line histograms, small-point histograms, first witnesses and SHA-256 digest of the ordered complete lines are in [the receipt](line_gate_results.jsonl). These counts establish that the incidence relation pool is nonempty and that a fixed small base discards many full lines. They do not establish a faster collector or a valid transfer from the source elliptic curve. In particular, the cited paper explicitly says its quartic **double-large-prime** sieve does not provide a significant speedup in that setting; its roughly threefold result is for different Diem–Kochinke methods and is not applied to these rows.

Q2 is a separate policy question motivated by [Laine and Lauter's time–memory work](https://eprint.iacr.org/2014/346): whether promoting a carefully selected point while the graph grows reaches a useful target dependence with lower *charged total cost* than a frozen small base. A fixed-base population is only the prerequisite. An adaptive selection must be trained separately; an oracle that chooses points after seeing all target relations would invalidate the comparison. Since the present quartic transfer maps and DLP solver are still missing, both Q1 and Q2 remain **partial**.

## G1: genus-2 pencil geometry

The hyperelliptic sieve described by [Vitse and Wallet](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf) gives a new relation-generation arm for the *already executable* Scholten genus-2 transfer. The [script](genus2_pencil_gate.py) uses the exact quintic `12Y²=12+100t+299t²+380t³+187t⁴+30t⁵` over `F_257`. It checks squarefreeness, chooses 20 disjoint pairs of rational points using a fixed hash rank, forms each monic Mumford `u` and interpolant `v`, and exhausts all 257 values of `λ`. Every quotient of `(v+λu)²-h` by `u` has zero remainder. Every three-distinct-root quotient is rechecked on the curve.

This yields **1,055 complete three-root quotients out of 5,140 pencil members**, with 50–57 per divisor. Per-divisor counts, root histograms, pair identities and relation-list digests are in [the receipt](pencil_gate_result.json). These are curves' intersection identities, not 1,055 independent useful DLP dependencies. The sweep uses 257 trials per divisor, and three-point relations need a solver beyond the current signed-edge graph. No cost improvement follows yet. G1 is **partial**, with a concrete full-pipeline test now available.

## T1: trace-zero applicability

The [trace-zero index-calculus construction](https://arxiv.org/abs/1405.1059) applies to `E/F_q` and the kernel of its extension trace. This is a different input family from Tian's cubic curve over `F_{523^3}`. The [gate](trace_zero_gate.py) uses the exact Tian coefficients `a=(140,92,82)`, `b=(243,339,359)` in `F_523[w]/(w³+2)` and computes
`j = 1728·4a³/(4a³+27b²) = (413,1,0)`. Its Frobenius image is `(413,462,0)`, so **this specific source has no elliptic model over `F_523`**. The arithmetic identity for `j` is rechecked in [the receipt](trace_zero_gate_result.json). This is a rigorous negative gate for one published fixture, not a rejection of trace-zero index calculus on base-defined curves. The next cohort must be chosen independently and compared with whole-curve index calculus and a matched rho using available Frobenius symmetries. T1 is **not attempted beyond eligibility**.

## Reproduce and limits

Run from the repository root:

```sh
python3 research/more_leads_20261019/quartic_line_gate.py > /tmp/line_gate_results.jsonl
python3 research/more_leads_20261019/genus2_pencil_gate.py > /tmp/pencil_gate_result.json
python3 research/more_leads_20261019/trace_zero_gate.py > /tmp/trace_zero_gate_result.json
cmp /tmp/line_gate_results.jsonl research/more_leads_20261019/line_gate_results.jsonl
cmp /tmp/pencil_gate_result.json research/more_leads_20261019/pencil_gate_result.json
cmp /tmp/trace_zero_gate_result.json research/more_leads_20261019/trace_zero_gate_result.json
```

The scripts use Python's standard library and public published/synthetic equations only. Quartic source coefficients match [`tian_quartic_smoothness.rs`](../../src/bin/tian_quartic_smoothness.rs); the Scholten quintic matches the [transfer certificate](../three_leads_e2e_20261018/TRANSFER.md). Their independent checks are within the scripts; this is same-source replay, not an independent implementation or host. No quartic map, quartic Jacobian relation matrix, genus-2 target-labelled pencil collector, trace-zero decomposition solver, or end-to-end measurement was produced in this round. Those gaps remain explicit acceptance gates before any IC or rho claim.
