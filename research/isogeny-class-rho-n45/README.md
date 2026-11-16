# Isogeny-class rho at 46 bits (replica of P1)

Classification: replication of a null result (rho against rho on one instrument); not an index-calculus improvement, no leaderboard change, no claim about any deployed curve.

Protocol: `PROTOCOL.md` (frozen before data, sha256 `f1a2fbd5…6287`), decision rules R1–R6 inherited from `research/isogeny-class-rho/`. Run: `runs/run-20261019-n45/` (command card `run.sh`, outputs under `P2/`). Tool: `src/bin/isoclass_rho` (merged in #2174).

<!-- HEADLINE -->

## What was measured

One class, P2: `F_p`, p = 46339417407469, a = 4184642753251, b = 42214793703111, `#E = n` = 46,339,405,405,021 (46 bits, prime order). The walk carries one ECDLP instance by verified isogenies; rho (32-adding walk, distinguished points, 8 bits) is run on 38 isogenous curves × 100 seeds, plus 3 isomorphism/representation controls, an A/A replica, a planted negation fold and a planted 4-adding walk.

| item | value | expected |
|:--|--:|--:|
| solves in class mode, all correct | 4,400 | |
| pooled mean / √(πn/2) | 1.0097 | 1.016 (32-adding) |
| Kruskal–Wallis / ANOVA across curves | p 0.38 / 0.67 | |
| flagged curves | 0 of 38 | |
| controls iso0, iso1, iso2, A/A | 1.107, 0.996, 0.984, 0.974 | 1 |
| planted negation fold | 0.732 | 0.707 |
| planted 4-adding walk | 1.291 | 1.137 (Teske); P1 read 1.27 to 1.36 |
| replay, first 5 seeds | 220 of 220 iteration counts identical | R6 |
| mean ratio P2/P1 | 41.68 | √(n_P2/n_P1) = 42.11 |

## Generators family and its repeat

| run | seed | solves | KW p | ANOVA p | variance ratio | max abs z (crit 3.23) | flagged |
|:--|--:|--:|--:|--:|--:|--:|:--|
| generators (frozen card) | 20261018 | 400 | 1.2e-4 | 2.7e-5 | 2.34 | 4.76 | v158g12 (1.64×), v158g17 (0.61×) |
| gen-rep1 (`ADDENDUM-generators.md`, sha256 `571e9361…f17baf`) | 20261019 | 400 | 0.97 | 0.98 | 0.58 | 1.97 | none |

By the frozen verdict wording the first is "more extreme than the family". The repeat is a different draw of generators and not a preregistered confirmation; it did not reproduce the effect and had under-dispersed arm means. This does not explain the first result. The first study had one flagged generator in 384 (B1); the same family type here has two in 40. Treat generator-level heterogeneity as unresolved, with more generator draws as the next measurement.

## Limits

- One class at one larger size, one seed set; power about 18 % per curve, 2 % class-wide at 100 seeds. A null does not exclude smaller effects.
- Same shared VM as the first run (Xeon @ 2.10 GHz, 4 threads, isolation L0). Wall-time columns are unisolated observations, not admitted performance evidence.
- The reviewer-noted native KS tie defect and shared-seed covariance concerns on the first study apply to this tool too; the Kruskal–Wallis and ANOVA values above do not depend on the KS routine.
- No ecbench spec/session/audit records exist for this run (AGENTS.md §12); it cannot support a comparison claim.

<!-- TABLES -->
