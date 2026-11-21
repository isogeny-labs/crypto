# Campaign: speeding up the hyperelliptic and non-hyperelliptic curve DLP, genus 2 and up (2026-11-21)

Standing objective (user, 2026-11-15 and 2026-11-21): a significant, verified improvement for the
HECDLP at genus 2–4 and for non-hyperelliptic curves, through index calculus. The program's success
criterion counts every verified, measured improvement over the current index-calculus baseline; rho is
the long-term target, reached iteratively. Every figure the campaign produces is `[pcc:unverified]`
until an independent replay; no leaderboard row without the validation gate.

## Where the baseline stands (merged, `main`)
| genus / kind | best arm | over rho (FPM primary) | at p | source |
|---|---|--:|--:|---|
| 2, hyperelliptic | 2LP + prefilter | 3–5 (no trend to 2^24) | 2^24 | `genus_ladder_scale_20261115` |
| 3, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.28 [0.22, 0.36] | 32771 | `genus_ladder_sge_scale_20261115` |
| 3, Smith transfer + Diem on the quartic | 0.50 of 2LP IC on H; 0.65 of rho on H | 8209 | `smith_transfer_cost_20261115` |
| 3, plane quartic (random) | Diem 2LP | 0.009–0.013 of quartic rho | 8209 | `genus_ladder_bound_r_20261114` |
| 4, hyperelliptic | 2LP + prefilter | 0.031 [0.018, 0.062] (no SGE yet) | 8209 | `genus_ladder_bound_r_20261114` |
| 5, hyperelliptic | 2LP + prefilter | 0.0084 | 1031 | `genus_ladder_20261114` |

Known cost structure of the best hyperelliptic arm: Cantor steps of the walk (one per trial),
the smoothness test (halved by the parity prefilter), and the linear algebra (three quarters of the
genus-3 cost before SGE, a quarter of that after; the merge's reach falls as the core grows). Known
negatives: harvesting Cantor's intermediate divisor (native cost exceeds the saving); the genus-2
large-prime graph (cost is the walk, not the algebra).

## Leads of this round (parallel, Sonnet 5.5 agents, each with a protocol frozen before data)
| id | lead | kind | owner files | prediction to freeze |
|---|---|---|---|---|
| C1 | SGE merge depth: `w_max` 8 → 16, 32 and `fill_max` 3 → 10 at genus 3 (8209…32771) and genus 4, 5 (1031, 2053), on identical relations | measurement, existing flags | study dir only | `la` ratio falls below 0.25 at 32771 for the best setting; relation phase identical |
| C2 | factor-base constant `--cc` ∈ {1, 1.5, 2, 3, 4} for 2LP+pre+SGE at genus 3 and 4 | measurement, existing flags | study dir only | an optimum away from 2.0 that lowers FPM by ≥ 10% at the largest prime of each genus |
| C3 | plane-quartic Diem: algebraic decomposition (conic–conic resultant) instead of the O(q) scan, and SGE on its core | code + A/B | `quartic_ladder.rs`, study dir | relations identical; `rel_mul` ratio ≤ 0.4 at 8209; FPM ratio ≤ 0.6 |
| C4 | exact group orders by baby-step giant-step over the Weil window from `N_1` alone (no `N_2` enumeration), to extend genus 3 to 2^17 and genus 4 to 2^15; then the best arm against rho there | code + run | `genus_ladder.rs` (order routine), driver flag, study dir | orders identical to the enumeration route on every instance of the earlier studies; genus-3 ratio ≤ 0.25 at 2^17 |
| C5 | independent replays of the SGE study and the composition study by a separate agent, from its own build | validation | report files only | counted fields identical on every replayed row |

Consolidation (main session): a frontier table "best arm against rho by genus and curve kind" with
the stack that each lead adds, in `FRONTIER.md`, and a PR per landed lead.

## Rules every lead follows
Protocol before data (predictions, failure criteria, `frozen.sha256`); the same instances and seeds
as the earlier studies where a comparison is paired; byte-identical baseline path proven by test and
replay before any change counts; every logarithm certified; `[pcc:unverified]` on every figure;
RESULTS.md with a scorecard that reports failures as failures; rustfmt and clippy clean; no git
commands from the agents (the main session commits).
