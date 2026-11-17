# Prime31 ECDLP frontier: exact shared-advice cost model and next experiments

2026-11-17 UTC — prospective analysis; empirical claims reference merged producer run #4513 and remain [pcc:unverified]. Not a measured result of new PR #4532.

## Why the 1024-target generic control should use a million baby steps

For Shanks BSGS on a fixed order-n group, reuse a baby table of m points across T independent targets. An approximate equal-cost group-operation model is C(m,T) = m + T*n/(2*m). Its continuous minimizer is m* = sqrt(T*n/2). At n=2,147,446,831 and T=1024, m*≈1,048,567, nearly exactly 2^20. Therefore the usual single-target sqrt(n)-entry table is under-advised for this 1024Q batch. This explains the three *preregistered* m values, not a retroactive winner selection. Real field operations, hash, normalization, setup and memory costs may move the optimal point.

| baby m | maximum giant scans | expected giant probes/Q (heuristic) | m+T*n/(2m) (rough group additions) |
| ---: | ---: | ---: | ---: |
| 46,341 (~sqrt n) | 46,341 | 23,170 | 23.77 million |
| 262,144 | 8,192 | 4,096 | 4.46 million |
| 1,048,576 | 2,048 | 1,024 | 2.10 million |

The million-entry table costs materially more memory. Report actual HashMap capacity and Linux RSS; process RSS sampled across serial arms is not isolated per-arm peak. A tuned Bernstein–Lange distinguished-point rho with equal advice remains another required generic frontier.

## Current full solver and its limitation

Merged PR #4513, 2026-11-17: existing native Rust mixed-base index-calculus-derived solver completed 1024/1024 unknown Q and solved all 61 basis logs. Fully charged same-runner IC was 1655.699ms setup + 1029.098ms descents = 2684.798ms. Native unfolded rho32 was 3017.608ms. The observed 11.03% IC/rho batch gain remains producer-only and did not compare shared-precomputed generic advice, strongest WZ or mature external IC. Readback checked the recovered scalar-point equations; there is no independent signed PCC. Using those two time components, IC's approximate break-even against that *particular* rho32 is 853 Q, not a portable threshold.

The solver uses a factor base, signed 4+4 relations and original-curve full-rank linear system, but the half-table oracle explicitly enumerates group sums. It is generic MITM-equivalent, not asymptotic/non-generic ECDLP progress.

## Index-calculus next leads: algebraic counting and real costs

For a signed four-leaf B-point factor base, the exact formal net-signature count with even L1 norm ≤4 is

D4(B) = 1 + 4 B + 8 C(B,2) + 4 B(B-1) + 8 B C(B-1,2) + 16 C(B,4).

At B=31 this is 618,885, exactly the Rust count. With negligible group collisions, an informal random-image model predicts D4(B)^2/n ordered 4+4 pairs per arbitrary target. This occupancy estimate does NOT prove curve-point coverage, rank, or runtime performance.

| B per base | exact D4(B) | heuristic pair count per Q |
| ---: | ---: | ---: |
| 23 | 188,325 | 16.52 |
| 24 | 223,105 | 23.18 |
| 28 | 412,385 | 79.19 |
| 31 | 618,885 | 178.36 |

Test B=28/29/30 versus B=31 with the SAME 1024 Q and same Rust signed relation replay; smaller B cuts table setup while raising complement probes. Measure full charged runtime, rank, misses, memory, original-curve checks, and confirm on an independently frozen target panel. Use B=23/24 as separate coverage/failure diagnostic, not presumed optimal performance.

A second measurable kernel is complement lookup: use the original Jacobian mixed-add plus batch normalization to evaluate 256 residual-minus-left candidates per block (instead of one affine inversion per candidate). Preserve exact lookup order, infinity cases and signed witness transcripts. This is a generic implementation optimization, not a new attack.

## Scientific next lead: genuine nonenumerative extraction

RR9 Hermite/norm certificates from PR #4466 verify eight points AFTER the leaves are known, but do not extract them from the public residual. The genuine algorithmic test is recovery of U,V and the eight-leaf factor polynomial from public R=aG+bQ alone subject to

U(x)^2 - (x^3+a_E*x+b_E)*V(x)^2 = c*(x-x_R)*product_j(x-x_j),

with x_j in the chosen factor-base x-domain, Hermite multiplicities, signs and original-curve checks. Explicitly forbid candidate support enumeration or generic four-sum table lookups. Prove exact positive and negative fibres at B4/B8, including known Dickson negatives, then run complete B31 unknown-scalar ECDLP with all elimination/rootfinding and failed work charged. PRs #4228 and #4349 contain previous Rust quotient/subresultant/rootfinding infrastructure.

## Leads checked and constraints

Prior PRs #4503/#4505/#4513 (mixed signed support and completion); #4454/#4486 (minor/negative source optimizations); #4466 RR9; #4381 BSGS fixes; #4108 symmetric IC; crypto-autoresearcher #3301/#3305 document Vieta, anchored m6/m8 F4, norm-lattice and tower negatives/censoring, not general impossibility. Published APS (Amadori–Pintore–Sala, Finite Fields and Their Applications 51, 2018, DOI 10.1016/j.ffa.2018.01.009) and Shanks BSGS are established prior art. The earlier n83 equal-advice control in benchmarks/20261016-n83-precomputed-rho-control refuted empty-table rho comparison. Research-KB MCP and wrapper are unavailable here, so search used connected GitHub and publisher sources.

PR #4532 freezes all three comparator arms and the complete 1024 Q before measurement. First source-specific CI exposed an E0425 counter-name error; library 40/40 and IC 3/3 tests passed, BSGS did not compile on that failed source. A sole variable-binding repair is committed without changing the frozen protocol. Do not claim new cost results until a passing exact-source run produces raw complete outputs and independent PCC verification. [pcc:unverified]
