# One-orbit jump history is correct but does not provide the tenfold gain

The once-only execution at pushed checkpoint `07a6025bf46` exited 0. All 160 counted calls completed on 20 fresh single-target workloads across five prime-order genus-2 and genus-3 models. Every scalar matched the held-out answer and original Jacobian multiplication; repeated outcomes and all 16 counter components matched. Online elimination and the general sparse incremental solver agreed in rank and scalar at all 420 accepted prefixes, with at most two noncore columns. All 320 modulus records satisfy the reduction identity. These are local observations, not signed PCC verdicts. [pcc:unverified]

Ratios below are control base-field REDC / candidate base-field REDC. The candidate combines one-orbit jump history with the general sparse incremental solver. Values below one mean the control is cheaper. Every registered case is retained.

| Genus | p | Old folded / candidate | Batched pencil / candidate |
|---:|---:|---:|---:|
| 2 | 101 | 0.810–1.252 | 0.773–0.877 |
| 2 | 251 | 1.056–1.206 | 0.862–1.152 |
| 2 | 601 | 1.291–1.536 | 0.629–0.729 |
| 3 | 113 | 0.848–1.846 | 0.334–0.706 |
| 3 | 421 | 0.796–0.985 | 0.241–0.316 |

The candidate does not consistently beat the stronger pencil. Every genus-2 p=601 case and every genus-3 case uses more base-field REDCs than the pencil; p=251 is mixed. There is no tenfold base-field reduction on any case. No arbitrary weighting of the two moduli is used to invent a total. [pcc:unverified]

The subgroup arithmetic improvement is larger but remains a separate component. At genus 2, p=601, old folded / candidate subgroup REDC is 35.713–56.331. At genus 3, p=421, it is 14.489–25.165. That comparison changes both walk history and recomputed versus incremental elimination, so the gain cannot be attributed solely to the generator. It also cannot establish a complete cold CPU gain while base-field work is worse than the pencil. [pcc:unverified]

The stronger reference resolves another confound. The general sparse solver requires 0.493–1.000 times the specialized online solver's subgroup REDCs across this panel, with identical base-field counts. It uses the same LP-first pivot order and processes only nonzero entries. Thus the specialized two-column representation does not outperform general sparse incremental elimination in this counted unit. Its support invariant is correct, but this experiment does not establish a new linear-algebra breakthrough. [pcc:unverified]

All public source-normalization information is retained in the core. Core sizes are 1–3 at genus 2 and 4 or 16 at genus 3; every taken factor-base step is represented by one folded column. The genus-3 p=421 runs reject nine smooth rows for having more than two noncore columns, and charge all those attempts. There are no other filtered rejections or incomplete calls. No expected scalar, source-point index or hidden factor-base logarithm is passed to a solver.

Reject this specific route as a tenfold candidate on the registered panel. Retain the correct support invariant and general sparse incremental reference. Future linear-algebra comparisons need that reference, rather than only repeated elimination or a dense control that performs arithmetic on zeros. Integrating the general incremental solver into the pencil is a possible baseline improvement; it would be engineering, not a new mathematical lead. The broader genus-2+ tenfold objective remains active.

The literature collision was recorded before execution: known-log adding walks and sparse rows were already implemented in KB source `source:b3c90d39a347ef661ccf10680a742186`; weighted graph cancellation is classical GTTD work. The adjacent low-rank coupling proposal is unmeasured. No priority or absence claim is made.

All counted setup, oracle, relation generation, elimination and internal scalar checks are charged afresh per target. Exact-order qualification, known-answer construction and the counter-disabled prefix gate are outside. These vectors are not a complete CPU model. There is no wall-time, external-fastest, matched-rho, leaderboard or independent-admission claim.

Sources, dependencies and binary were frozen and pushed before the one execution. Raw cases, prefixes, counters, exit status and hashes are retained in Git under the S3-outage instruction. There were no retries, adaptive input replacements, budget extensions or new paid resources.
