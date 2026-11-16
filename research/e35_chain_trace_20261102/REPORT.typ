#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E35: solver B's pivot schedule, learned per curve and replayed with a counted check

*Run report and results, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.40 (pre-registration, commit `038e2562d3`, before any code) and §9.42 (results). Code `b1f5df9a45` (module `src/cryptanalysis/chain_trace.rs`, examples `e35_stage_a`, `e35_checks`, `e35_tables`) and `2308fa4664` (dispatch hook in `klein_v4.rs`, `quartic_folds.rs`, `mod.rs`). Binaries were built from `git archive 2308fa4664` and pinned by sha256 in `experiments/39_chain_trace/binary_sha256.txt`; the checks example was rebuilt once (sha256 `ed21fbd0…`). Records are in `experiments/39_chain_trace/`: `stage_a.log` (`stage_a.json` sha256 `6c7c8a44…`; the artifact-store upload is blocked without the AWS CLI), `checks*.{json,log}`, `identity_existing_solvers.json`, `e2e_chain-sparsest-trace_karatsuba_p*s*.json` and the log, `tables.md` and `summary.json`. Host: the 4-vCPU Claude Code container, at most two cores. isolab-eks is not reachable from it (no AWS CLI, no kubectl). No isolation tier is earned: operation counts only, no wall-time claim.

== Question and design
Solver B (sparsest pivots) multiplies $1.82$–$1.86 times$ less than solver A, E31's ranked chain solve, at 0 GAE. Its counted selection costs $1.05 dot 10^6$ native operations a call (E33), so it is unranked. If B's choices were a property of the curve, they could be learned once and replayed. This is the idea of Joux–Vitse's F4 trace reuse (lead atlas `KR-IC-f3da82`). On the binary $S_3$ line the trace-reuse engines gave $1.4$–$2.3 times$ and that lead is closed dominated (`LEAD-ICE-boolean-f4-f5-f6-engines`). E21 (§9.11) replayed a shape trace on this family's `D4` solve and lost.

*T (`chain-sparsest-trace`).* The trace is learned from three targets of a separate stream (B′'s elimination, B's choices, per-column mode), out of sample. In every contested column the replay does one lookup and two tests: the traced row must sit at its traced position among the sorted candidates, and its entry must be nonzero. On a miss it takes A's choice at no extra cost. A different matrix shape replays nothing. Learning (multiplications, entries, B′ selection, mode) is charged inside the record's totals.

== What ran
- *Stage 0 + Stage A:* E31/E33's calls exactly (E25's ten GLS curves, $p in {101,149,197,269,397}$, seeds 1 and 2, the first 1,000 targets of arm A's stream), one thread. Five solvers per call: E25, A counted, B counted, B″ and T, 50,000 solves. Stage 0 records B's pivot sequence on every call and compares it with the curve's trace and with the other curves' traces.
- *Controls:* 32 calls each of $p = 101$ seed 2 and $p = 397$ seed 1. Three corruptions (a used row, another pivot row, another shape), each with and without the check.
- *Identity:* E33's pinned binary against E35's, for seven existing configurations at $p = 101$ seed 2, and against the stored E27/E33 records.
- *End to end:* E33's command with `--solver chain-sparsest-trace --karatsuba`, ten curves, both arms (20 rows), 0 failures.

== Measured
- *Stage 0.* No call's pivot sequence equals its curve's trace, and 997–1,000 of 1,000 sequences per curve are distinct. In contested columns (334 a call), B's row equals the trace's in $69.8$–$88.4 %$, rising with $p$. Another curve's trace agrees as well, often better ($69.3$–$90.2 %$). The stable part is generic structure; the rest depends on the target.
- *Stage A.* T agrees with E25 on all 10,000 calls. It does $744$–$817$ selection operations a call and misses $55.6$–$77.5 %$ of contested columns; misses cascade. $C$ median: T $5.23$–$5.51 dot 10^5$, B $3.04$–$3.33 dot 10^5$, A $6.01$–$6.34 dot 10^5$. T keeps $18.8$–$36.8 %$ of B's saving over A. Per-call $S_A\/S_T$ is $1.092$–$1.254$ / $1.014$–$1.049$ / $0.9916$–$0.9952$ at 0 / 0.1 / 1 GAE.
- *Controls.* Every corruption was rejected (63/63 per kind) with unchanged outputs. Without the check every corrupted call aborted. A first version counted one no-op corruption as undetected (kept as `checks_v1`).
- *Identity.* Old = new for all seven configurations, and = stored for the four Karatsuba ones. All 20 T records equal E33's B-counted records outside the registered fields: every relation, log and walk is the same.
- *End to end* (ranges over 20 rows of $S_X\/S_T$ at 0 / 0.1 / 1 GAE): A $1.093$–$1.252$ / $1.020$–$1.055$ / $0.9981$–$1.0028$ (6 rows below 1 at 1 GAE); E25 $2.73$–$3.20$ / $1.421$–$1.507$ / $1.0325$–$1.0557$; B″ $0.709$–$0.807$ / $0.975$–$1.007$ / $1.053$–$1.059$; B $0.60$–$0.68$ / $5.19$–$5.48$ / $6.55$–$6.74$. Distance to the matched rho (walk (iii) of E27), $S_T\/"rho"$: $5.11 dot 10^3$–$3.53 dot 10^5$ / $2.02 dot 10^4$–$1.35 dot 10^6$ / $9.04 dot 10^4$–$6.63 dot 10^6$. Rho is ahead of every row at every weight.

#figure(image("figures/stage0.svg", width: 100%), caption: [Stage 0. Left: per curve over 1,000 calls, the share of contested columns where B's pivot row (and position) equals the curve's trace, the same against the best other curve's trace, and whole-sequence equality (0 everywhere). Right: contested-column agreement of each curve's calls with each curve's trace. Source: `summary.json`.])
#figure(image("figures/stage_a_ratios.svg", width: 100%), caption: [Stage A per-call mean $S(g) = C + 97 g ("entries" + "selection")$ of A and B″ over T's (learning amortised over the 1,000 calls), per curve. Above 1, T is cheaper. T is behind A at 1 GAE on every curve and behind B″ at 0 and 0.1 GAE. Source: `summary.json`.])
#figure(image("figures/band.svg", width: 100%), caption: [End to end, per row ($p$/seed/arm): $S_X\/S_T$ for A (E27's records), B″ (E33's) and E25 (schoolbook; omitted at 0 GAE, $2.7$–$3.2$). B and B′ are far off scale ($0.6$–$0.68$ at 0 GAE; $2.1$–$6.7$ above). At 1 GAE, T is ahead of A on 14 rows and behind on 6. Source: `summary.json`.])

== Verdicts (pre-registered, §9.40)
G2.145, G2.146 (controls v2) and G2.147 hold. G2.144 is *falsified*: contested-column agreement is below $90 %$ on every curve. G2.148, G2.149, G2.150, G2.151 (bands), G2.152 and G2.153 (upper end of the range) are *falsified*.

== Ranks (only where the order holds at 0, 0.1 and 1 GAE on every row)
- T against A: *unresolved*. T is ahead at 0 and 0.1 GAE on every row; at 1 GAE it is ahead on 14 rows and behind on 6. A remains the ranked solver.
- T against E25: ahead at every weight on every row, as A already is (§9.24). This is not new.
- T against B, B′ and B″: *unresolved*. Each is ahead of T at 0 GAE; B″ is also ahead at 0.1 GAE on 18 of 20 rows; T is ahead of all three at 1 GAE.
- Rho is ahead of T across the band. Distance remaining at 0 GAE, best row: $5.11 dot 10^3 times$ (T, unranked); A, the ranked solver, is at $6.39 dot 10^3 times$.

== What it says, and what it does not
There is no per-curve pivot order to replay. B's tie-breaks among rows with nearly equal counts are set by accidental zeros, so they change with the target. A counted replay keeps B's choice in only $23$–$44 %$ of contested columns. It is break-even with A at 1 GAE and behind B″ at 0 and 0.1 GAE, so it is *not an improvement* under the 2026-10-15 criterion. Class: engineering, negative. It agrees with E21 on a different object. Not tested: a membership test through a row-to-lead map instead of the position test. It could recover at most the $≈ 2\/3$ of misses whose row was still a candidate, and its bookkeeping would have to be priced (exploratory, not registered). The next lever is unchanged: the Macaulay matrix itself.

== Limits
- Operation counts on a shared container; no wall time, no isolation tier.
- In-crate unit tests were not added or run: a release test build needs about 9 GB. The solver-level checks run as examples (`e35_checks`, Stage A identity), and the library and tests type-check (`cargo check`).
- $S_"E25"\/S_T$ prices T's learning in Karatsuba arithmetic (about $5 dot 10^5$ multiplications per run, $< 10^(-4)$ of $S$).
- The positive control aborts partly because of the trace's own invalid entries, not the corruption alone.
- Not validated: no replay certificate or validator report, so no leaderboard or `benchmarks/` row.
