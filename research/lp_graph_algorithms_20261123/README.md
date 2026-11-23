# Large-prime graph algorithms: integration of the #4917 catalog queue

Date 2026-11-23. Code: `src/cryptanalysis/large_prime_graph.rs`,
`src/cryptanalysis/large_prime_graph/eval.rs`, `examples/lp_graph_eval.rs`,
`tests/large_prime_graph.rs`. Catalog: PR #4917 ("Graph algorithms and
asymptotic costs for ECDLP index calculus"), section 8.

**Standing: implementation with exact cross-checks on synthetic streams.
No ECDLP speed claim, no benchmark row, no leaderboard change.** Wall times
below are single local runs (4-core cloud VM, release build) for
orientation only.

## What was integrated

| Catalog §8 item | Status | Where |
| --- | --- | --- |
| 1. Gain-aware DSU and singleton peeling | **new** exact DSU (`GainDsu`, `gain_dsu_rank`); peeling reused from `filter_large_primes` | `large_prime_graph.rs` |
| 2. MST merges vs lightest-row pivot | **reused** both filters, now paired on identical streams with full replay | `eval.rs` |
| 3. BFS vs low-stretch trees | **new** `bfs_forest`, `low_stretch_forest` (AKPW-style heuristic), `fundamental_cycle_length` | `large_prime_graph.rs` |
| 4. Matching, DM and width | **new** `hopcroft_karp`, `dulmage_mendelsohn` (coarse + SCC fine blocks), `minimum_degree` (fill, width = treewidth upper bound), `core_numbers` | `large_prime_graph.rs` |
| 5. Hypergraph partitioning for SpMV | **new** `fm_partition` (recursive FM bisection, column-net model), `connectivity_minus_one` | `large_prime_graph.rs` |
| 6. Krylov block and generator tuning | **not done**: crypto has only scalar Wiedemann (`pq_wiedemann`); a block Wiedemann / matrix Berlekamp–Massey solver is a separate implementation task | — |

Not integrated, by design: the k=3 gain-forest harvester and coset-filtered
MITM (PR #4905 owns them); catalog families with no role in the LP pipeline
or no practical implementation (directed SSSP 2026, almost-linear max-flow,
2-SAT, #SAT, nested dissection's numerical setting, Casacuberta–Kyng sparse
inversion, minimum cycle basis).

## Correctness evidence

`cargo test --release --test large_prime_graph`: 13/13 pass (also in debug,
which runs the `debug_assert` FM cut check). The tests include:

- gain-DSU rank = dense Gaussian rank on 600 random LP blocks over
  `ℓ ∈ {3, 7, 1000003, 2^61−1}` (rows of arity ≤ 2, repeated keys and zero
  coefficients included); explicit balanced/unbalanced cycles of length 2–8;
  the catalog's triangle (rank 3) and bridge (left nullity 1, needing the bridge);
- core numbers = brute-force k-core peeling on 200 random multigraphs;
- Hopcroft–Karp size = Kuhn's algorithm on 300 random supports; structural
  rank ≥ field rank; DM block identities (`|R_over|−|C_over|` = unmatched rows,
  etc.) and no edge leaving the over-determined block;
- both forests are spanning with `e − v + c` fundamental cycles;
- minimum degree: path (fill 0, width 1), cycle (fill n−3, width 2), K5;
- FM never worse than its contiguous start at k = 2 and keeps balance;
- the evaluator replays **every** relation from both filters against the input.

**Build caveat.** `main` at `d45ac8de4` and at `021cd64a4` does not compile:
`src/cryptanalysis/ecbench/spec.rs` (test module) and
`src/cryptanalysis/ic_framework/mod.rs:561` have unclosed delimiters from a
merge. The tests above were run in a standalone crate that compiles the real
`large_prime_filter.rs`, `large_prime_filter_mst.rs`, `large_prime_filter/`,
`jv_large_prime_rank_bound.rs` and the new files unchanged, with a no-op
`jv_bitops` (operation counters only). CI on this PR cannot pass until
`main` is repaired.

## Measurements (frozen synthetic streams)

Streams: `examples/lp_graph_eval --seed 7 --rows R --lp-columns 2R/3
--fb-columns R/2 --arity 1,6,12,{0|3}` (LP arity weights 0:1:2:3), large
primes log-uniform on `[C/32, C]`, coefficients uniform nonzero mod
`2^61−1`; two `--unit` arms with all LP coefficients 1. Full records in
`results/`, table in `summary.csv`, figure `figure.svg`, PDF `report.pdf`
(`python3 make_report.py`).

| R | arity | DSU rank / left nullity | pivot out | MST out | MST/pivot Z | MST/pivot wall | L BFS / low-stretch | FM/contiguous Σ(λ−1), k=2/4/8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2000 | ≤2 | 942 / 1058 | 1058 | 1058 | 0.983 | 22.6× | 2918 / 3085 | 0.81 / 0.79 / 0.76 |
| 2000 | ≤3 | 875 / 869 (2-entry rows) | 1031 | 1031 | 0.975 | 27.7× | 2450 / 2747 | 0.83 / 0.80 / 0.77 |
| 4000 | ≤2 | 1882 / 2118 | 2118 | 2118 | 0.975 | 45.3× | 6983 / 7941 | 0.81 / 0.77 / 0.74 |
| 4000 | ≤3 | 1762 / 1674 (2-entry rows) | 2036 | 2036 | 0.979 | 52.6× | 5083 / 5843 | 0.82 / 0.80 / 0.77 |
| 8000 | ≤2 | 3819 / 4181 | 4181 | refused | — | — | 13920 / 15824 | 0.80 / 0.77 / 0.75 |
| 8000 | ≤3 | 3578 / 3329 (2-entry rows) | 4038 | refused | — | — | 11262 / 11708 | 0.82 / 0.80 / 0.78 |
| 32000 | ≤2 | 15221 / 16779 | 16779 | refused | — | — | 65538 / 68737 | 0.81 / 0.78 / 0.75 |
| 32000 | ≤3 | 14334 / 13256 (2-entry rows) | 16125 | refused | — | — | 49497 / 52947 | 0.82 / 0.80 / 0.78 |

Observations (each limited to these synthetic streams):

1. **Exact DSU agrees with the existing filter.** On every stream whose rows
   all have ≤ 2 large primes, the DSU left nullity equals the number of
   complete relations the pivot filter emits (1058, 2118, 4181, 16779), and
   the dense rank where computed. Gain-DSU time: 0.35 ms (R=2000) to 7.1 ms
   (R=32000), against 2.5–86 ms for the pivot filter. As the catalog says,
   metadata timing alone does not pass: the DSU produces rank, not relations.
2. **The conservative `StructuralRankBound` was already tight here** (lower
   bound = exact rank wherever its 4096-row contract applied), including the
   `--unit` arms with 229/419 balanced cycles: every component also held an
   unbalanced cycle or a singleton. The exact DSU differs from it only on
   components whose cycles are all balanced and unanchored (unit tests).
3. **MST merging**: same relation count, 1.7–2.5 % fewer residual nonzeros
   and 1–2 % fewer source terms than the pivot filter, at 23–53× the wall
   time; refused atomically above its default `max_rows = 4096`.
4. **Low-stretch heuristic loses on LP graphs**: L is 2–15 % *larger* than
   BFS on every stream (it wins on a 24×24 grid, 8778 vs 13754, in the unit
   test). BFS remains the right default for witness expansion here.
5. **FM partitioning** lowers the column-net communication model Σ(λ−1) of the
   residual matrix by 17–26 % against contiguous row blocks at k = 2, 4, 8.
   This is a model value; no distributed product was timed.
6. **Treewidth route not applicable**: minimum-degree width on the residual
   column graph is 0.77–0.80·N wherever elimination finished (e.g. 3095 for N = 3998), far from the
   `t = o(√N / log N)` regime in which the catalog's `O(N t² log² N)` route
   would beat `O(NZ + N²)`; at R = 32000 elimination hit the 5 M fill cap.

## Next

- Run the evaluator on real collector streams (`--input relations.jsonl`,
  one `PartialRelation` per line) from the prime-field and Gaudry-cubic lanes;
  synthetic coefficient statistics are not ECDLP statistics.
- Block Wiedemann (item 6) as its own task.
- Any speed claim from items 2 or 5 needs the repository's benchmark protocol
  (`ecbench`, bench records, validation gate) and complete cold solves.

Sources checked (KB): research-kb MCP not available in this session; fallback
was the #4917 catalog and the six source files it pins, plus the modules read
here (`large_prime_filter*.rs`, `jv_large_prime_rank_bound.rs`).
