# Large-prime graph algorithms: integration of the #4917 catalog queue

## M31 retained-stream continuation, 2026-11-24 UTC

The current draft continuation is [PR #5076](https://github.com/isogeny-labs/crypto/pull/5076). Native Rust replay on p=2147483647 checked the existing 645-row B677 capture and admitted 613 rows. At prefixes 256/512/613, forest, pivot and MST each retained useful and augmented ranks 131/355/452 with zero source replay failures. The [frozen receipt](evidence/native-m31-capture-20261124/README.md) and [visual report](reports/run-20261124T0200Z/REPORT.md) bind the exact source and raw outputs. Unsigned registration PCC-20261124-fcc0a3; [pcc:unverified].

This is a generic exhaustive-pair baseline with one fixed public target. The result compares graph projection payload; collection, target recovery, timing and full cold-cost comparisons remain separate. Historical entries below are preserved. No PR merge or paid compute was performed.

Date 2026-11-23. Code: `src/cryptanalysis/large_prime_graph.rs`,
`src/cryptanalysis/large_prime_graph/eval.rs`, `examples/lp_graph_eval.rs`,
`tests/large_prime_graph.rs`; follow-up (2026-11-23): `large_prime_graph/block_wiedemann.rs`,
`examples/block_wiedemann_eval.rs`, `examples/lp_graph_collect.rs`,
`tests/block_wiedemann.rs`. Catalog: PR #4917 ("Graph algorithms and
asymptotic costs for ECDLP index calculus"), section 8.

**Standing: implementation with exact cross-checks on synthetic streams and
on real `ecbench_large_prime` relation streams.
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
| 6. Krylov block and generator tuning | **new** block Wiedemann: block Krylov sequence, σ-basis matrix generator, verified kernel vectors, bordered `A x = b` with preconditioned retry; block-size schedule measured below | `large_prime_graph/block_wiedemann.rs` |

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

**Build status.** When #4929 landed, `main` did not compile (unclosed
delimiters in `ecbench/spec.rs` and `ic_framework/mod.rs:561`), so its tests
were run in a standalone crate. After the repair (#5009, `main` at
`7d61e3b45`) the real crate's library compiles and, in the real crate,
`tests/large_prime_graph.rs` passes 13/13 and `tests/block_wiedemann.rs` 5/5
(`cargo test --release`). That run stubbed `src/bin/hecx.rs` locally,
because that binary still has unclosed delimiters on `main` (lines 160, 327,
597, 766) and cargo builds every binary for integration tests; the stub was
not committed. The #4929 tests and examples imported `crypto::`, but this
crate's library is `crypto_lib`; the follow-up fixes the imports.

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

## Real relation streams (`ecbench_large_prime`)

`examples/lp_graph_collect` runs the binary-curve collector
(`ecbench_large_prime::solve_with_options`, two large primes, 4 summands,
target `[7]G`) with the new opt-in `SolveOptions::capture_relations`, which
copies the exact `PartialRelation` batch handed to `filter_large_primes`, and
evaluates it. Generic curves are `ic_boundary::random_binary_instance(n,
20261123, 16)`; Koblitz curves are `koblitz_instance_best(n)`, run with the
plain (`filter`) and the signed-Frobenius-orbit (`orbit`) large-prime graph.
Every run recovered the logarithm 7. Records: `results_real/`.

| stream | R | C | rank / left nullity | balanced / unbalanced cycles | pivot out | MST | L BFS / low-stretch | residual N | min-degree width |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| generic n=16 | 20000 | 57 | 57 / 19943 | 9153 / 3951 | 19933 | refused | 47615 / 47615 | 13 | 11 |
| generic n=20 | 60000 | 228 | 228 / 59772 | 26816 / 26813 | 59771 | refused | 193194 / 241716 | 14 | 13 |
| generic n=24 | 120000 | 965 | 965 / 119035 | 55828 / 55228 | 119035 | refused | 494523 / 494523 | 33 | 32 |
| Koblitz n=17 plain | 29726 | 57 | 57 / 29669 | 6873 / 20294 | 29669 | refused | 80587 / 80587 | 6 | 5 |
| Koblitz n=17 orbit | 29757 | 54 | 54 / 29703 | 4891 / 22285 | 29703 | refused | 80559 / 80559 | 6 | 5 |
| Koblitz n=19 plain | 60000 | 231 | 231 / 59769 | 25719 / 25741 | 59769 | refused | 180413 / 214813 | 18 | 17 |
| Koblitz n=19 orbit | 60000 | 228 | 228 / 59772 | 20331 / 31122 | 59772 | refused | 189415 / 223297 | 18 | 17 |
| Koblitz n=23 plain | 120000 | 982 | 982 / 119018 | 56262 / 56359 | 119018 | refused | 499207 / 499207 | 27 | 26 |
| Koblitz n=23 orbit | 120000 | 966 | 966 / 119034 | 12886 / 99745 | 119034 | refused | 493361 / 493361 | 27 | 26 |

Observations (these instances; trial caps far above what the solve needed):

1. **Exact DSU = existing filter on real data.** Every stream has LP arity
   ≤ 2, and on every one the DSU left nullity equals the pivot filter's
   complete-relation count, except generic n=16 and n=20, where the filter
   output is 10 and 1 lower: exactly the duplicate relations it removes
   before elimination (`report.duplicate_relations`; no fill rejections). Dense rank agreed where computed
   (n=16, 17). DSU time 1.2–6.5 ms against 33–259 ms for the filter.
2. **Real LP blocks are tiny and dense:** C = 54–982 columns for 20k–120k
   rows, so the LP graph is far past its cycle threshold. With `±1`
   coefficients about half of all cycles are balanced; the signed-Frobenius
   orbit graph makes balanced cycles rarer (12886 vs 56262 at n=23), the
   cyclic-gain effect measured in workspace note `graph-theory-ic-20261121`.
3. **MST filter refuses every real stream** (column degree above its 64
   limit, and above 4096 rows). **Low-stretch trees tie or lose** against BFS
   (up to 25 % worse at n=20). **Residual matrices are N = 6–33 columns and
   dense** (min-degree width N−1, FM equal to contiguous): partitioning and the
   treewidth route have nothing to act on at these sizes.

## Block Wiedemann schedule (`block_wiedemann/`)

`examples/block_wiedemann_eval --dim N --weight 6 --dead 4 --seed 1`, modulus
`2^61−1`, `m = n = b`; every run found a verified kernel vector with the first
seed.

| N | b | products (sequence) | sequential rounds 2N/b | projection mults | generator mults | generator degree | wall |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2000 | 1 | 6000 (4008) | 4008 | 8.0 M | 16.0 M | 1991 | 0.77 s |
| 2000 | 4 | 4531 (4032) | 1008 | 32.3 M | 64.5 M | 498 | 1.22 s |
| 2000 | 16 | 4253 (4128) | 258 | 132.1 M | 266.0 M | 124 | 3.76 s |
| 8000 | 1 | 23979 (16008) | 16008 | 128.1 M | 255.2 M | 7970 | 11.2 s |
| 8000 | 4 | 18026 (16032) | 4008 | 513.0 M | 1023.2 M | 1993 | 20.1 s |
| 8000 | 16 | 16627 (16128) | 1008 | 2064.4 M | 4125.0 M | 498 | 62.1 s |

As the catalog states, the block size changes scheduling, not total product
work: sequence products stay ≈ 2N for every `b`, while the number of
sequential rounds falls as `2N/b`. Projection (`O(N² b)`) and this quadratic
σ-basis grow linearly in `b`, so on one thread wall time *rises* with `b`;
the gain needs the `b` products of a round to run in parallel (or a fast
`Õ(N b^{ω−1})` generator, not implemented). Horner reconstruction and
kernel search add `≈ N/b` products, the difference between the two product
columns.

## Next

- Prime-field and Gaudry-cubic collectors through `--input` JSONL (only the
  binary-curve collector exposes its stream so far).
- A parallel block-Wiedemann product loop and a fast matrix generator before
  any block-size speed claim.
- Any speed claim needs the repository's benchmark protocol (`ecbench`, bench
  records, validation gate) and complete cold solves.

Sources checked (KB): research-kb MCP not available in this session; fallback
was the #4917 catalog and the six source files it pins, plus the modules read
here (`large_prime_filter*.rs`, `jv_large_prime_rank_bound.rs`).
