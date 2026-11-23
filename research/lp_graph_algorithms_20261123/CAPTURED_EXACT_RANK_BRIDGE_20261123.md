# Captured-stream exact useful-rank bridge — scoped Rust gate

Date: 2026-11-23 UTC. Lane: [draft PR #5043](https://github.com/isogeny-labs/crypto/pull/5043) following **merged** PRs #4929 and #4988. Claim status: **producer change; independent signed PCC unverified**. Do not merge or use paid compute.

## Bottleneck and bounded contribution

`large_prime_graph::eval::evaluate_stream` already implements the **opt-in**, capped, same-stream comparison of lazy coefficient-aware forest witnesses, the existing pivot/lightest-row filter, and the existing MST filter, including source replay and sparse exact projected factor-base rank. The native examples `examples/lp_graph_eval.rs` and `examples/lp_graph_collect.rs` previously hard-coded `EvalParams::default()`. Thus a real `ecbench_large_prime` capture or a frozen `PartialRelation` JSONL input passed through the CLI could not request the comparison.

The new *CLI bridge* adds `--exact-forest`, and an optional `--forest-max-rows N` with **1 ≤ N ≤ 4096**, to both existing native Rust example frontends. No change to `large_prime_filter`, MST merger, gain forest, collector, original curve arithmetic, solver, relation choice, or default evaluation schema. A successful collected logarithm must pass both existing fast and independent group verifications; if the solver exhausts, the diagnostic still records source relations but **does not claim a solved log**. Opt-in comparison additionally refuses any pivot/MST/forest source-replay failure. MST and forest failures remain atomic; absent ranks are never zero-rank success.

## Mathematical invariant and scope

Use `F_p` for curve coordinates and a separate prime-order relation field `F_ell`. Let the original source matrix be `[L | S | b]`, with LP block `L`, factor-base/explicit-target columns `S`, and fixed known RHS `b`. If `W` spans the left nullspace of `L`, then

```text
rank_Fell(W S)       = rank_Fell([L | S])     - rank_Fell(L)
rank_Fell([W S | Wb]) = rank_Fell([L | S | b]) - rank_Fell(L).
```

The identity is exact but applies to the **full cancellation space**, not an arbitrary fill-limited forest refusal or the possibly discarded pivot/MST output. Counted graph cycles and LP nullity are not the projected useful rank. The existing source-bound regression and `projected_rank.rs` audit check that distinction. A nonzero residual known RHS is an inconsistency only if every unknown target logarithm is an explicit small column.

`lp_graph_collect` currently collects naturally generated **binary-curve** `ecbench_large_prime` relations, not ordinary Prime31 relations. Previous real binary records at `results_real/` are 20,000–120,000 rows, above the present 4096-row exact-audit cap; they are background evidence, **not** eligible full-stream exact-rank measurements. New capped real captures and an ordinary Prime31 collector/stream are different future prerequisites. The immutable ordinary Prime31 reference parameters elsewhere in the repository are `p=2147483647, a=847514846, b=2087444724, ell=2147446831, G=(625037715,2100627251)` (cofactor one).


### Root compile blocker and exact-source fallback (executed)

[Native run #45232685589](https://github.com/isogeny-labs/crypto/actions/runs/45232685589) finished with **the standalone original-module graph integration test step passing**, then an **unshimmed root example compilation failure** before the CLI smoke could execute. The compiler error was `src/cryptanalysis/prime_root_trace.rs:4372:3`: an unclosed delimiter originating at `fn vproduct` near line 1071 (with `if` at 1108), in a file untouched by this lane. This is **not** successful whole-root validation and no candidate CLI tests ran in that job. Concurrent repository-wide workflow-syntax gate [#45232685419](https://github.com/isogeny-labs/crypto/actions/runs/45232685419) separately failed due out-of-scope `ct330-f103-rho-bsgs-paired.yml:99`.

Instead of editing other sessions' compiler-source files, use the existing source-exact native harness `research/lp_graph_algorithms_20261123/native_gain_dsu/`, which symlinks original production graph/filter modules and intentionally shims only unrelated operation meters. Add the **original** `examples/lp_graph_eval.rs` as an explicit Cargo example in the isolated harness; run the same frozen default/opt-in/cap commands using its `--manifest-path`. This qualifies only the evaluator CLI and graph modules, **not** the root crate, the binary-curve collector CLI (which depends on unshimmed `ecbench_large_prime`), or any Prime31 relation collector. The root error and missing coverage remain visible in the handoff. Source revisions `8c5d173c` (harness target), `8ec25dd6` (workflow). New terminal run pending, not a passed test. [pcc:unverified]

## Frozen correctness test before execution

Hypothesis: the opt-in bridge can run the existing same-stream forest/pivot/MST exact useful-rank auditor from the native frontends; default serialization remains backwards compatible; unsupported/capped inputs cannot silently report an exact rank.

Input: `lp_graph_eval` seeded 36-row source, seed 23, 12 LP columns, 8 factor-base columns, arity distribution weights `1,2,5` (only 0/1/2 LP arities), `F_101`; all three invocations use the same canonical source seed and label `same-seed`. Control: same input with no opt-in; candidate: `--exact-forest --forest-max-rows 64`; rejection control: `--exact-forest --forest-max-rows 8`.

Required: compare source-invariant fields `modulus, rows, lp_columns, incidences, arity_histogram`; old JSON has neither `forest_filter` nor exact-rank fields; candidate has non-refused forest, exact forest/pivot projected rank and zero source replay failures for completed arms; if MST refuses, its rank must stay absent; cap records `row_cap` without emitting a fake forest rank or paying the extra projected-rank audit.

Commands embedded in `.github/workflows/graph-ic-gain-dsu-native.yml`:
```sh
bash research/lp_graph_algorithms_20261123/native_gain_dsu/run.sh
cargo check --release --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml --example lp_graph_eval
cargo run --release --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml --example lp_graph_eval -- --seed 23 --rows 36 --lp-columns 12 --fb-columns 8 --arity 1,2,5 --modulus 101 --label same-seed --out /tmp/graph-default.json
cargo run --release --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml --example lp_graph_eval -- --seed 23 --rows 36 --lp-columns 12 --fb-columns 8 --arity 1,2,5 --modulus 101 --label same-seed --exact-forest --forest-max-rows 64 --out /tmp/graph-exact.json
cargo run --release --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml --example lp_graph_eval -- --seed 23 --rows 36 --lp-columns 12 --fb-columns 8 --arity 1,2,5 --modulus 101 --label same-seed --exact-forest --forest-max-rows 8 --out /tmp/graph-capped.json
```

No isolation-grade timing/peak-RSS measurement, cold solve, or optimized-rho comparison is claimed by this CLI check. Only one standard hosted correctness job, 12-minute timeout, no paid compute. The existing source-bound `tests/large_prime_graph.rs` regression already checks 128 original-block rank identities over five `F_ell` fields and 24 seeded same-stream cases, but this new job specifically tests the actual example admission bridge. **Record the native run's terminal outcome before calling it passing.**

A subsequent bounded *real binary* collector invocation (not executed here) is:
```sh
cargo run --locked --release --example lp_graph_collect -- \
  --n 16 --seed 20261123 --small 4 --envelope 7 --summands 4 \
  --large-primes 2 --trials 2048 --graph filter \
  --exact-forest --forest-max-rows 4096 \
  --relations captured-binary.jsonl --out captured-binary-eval.json
```
This will be a fresh data collection with group-verified `[7]G` **only if a scalar is recovered**; it must not be conflated with a natural Prime31 collector. Log observed trial/row count, exact useful rank, failed witnesses, source fill, phase times and peak RSS under the approved isolation regime. Before any Prime31 scientific comparison obtain actual original-curve source-bound two-LP relations with full point-replay witnesses and source hashes; compare complete setup/rejected trials/filter/LA/individual recovery to the strongest qualified IC and negation-map rho baselines with `[d]G=Q` verification.

## Sources checked and execution bounds

KB unavailable: this runtime offers no `kb_status` or `search_knowledge` MCP actions, and no local checkout to invoke `scripts/research-kb`; a failed tool lookup is not evidence of absence. Bounded fallback: current `AGENTS.md`, `docs/ACTIVE.md`, catalog `docs/ic/graph-algorithms/{README,SOURCES}.md`, frozen source notes and prior merged #4917/#4905/#4929/#4988, native `large_prime_filter`, `large_prime_filter_mst`, `large_prime_graph/eval.rs`, `projected_rank.rs`, `ecbench_large_prime.rs`; cross-repository search `cryptanalysis/experiments/ic-bench/prime_bridge.py` and `crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md`. [Diem, *Index calculus with double large prime variation for curves of small genus with cyclic class group* (2006)](https://arxiv.org/abs/math/0606607) on a class-group double-large-prime algorithm and [Cavallar, *Strategies in filtering in the number field sieve* (2000)](https://ir.cwi.nl/pub/4456) on sparse-matrix filtering are literature context, **not claims of novelty for this CLI bridge**. No separate agent spawn or Conductor executable available; one coordinator only.

Full unshimmed crate, independent signed PCC, true Prime31 source stream, cold solver and performance evaluation remain separate gates. Preserve other sessions' files/history. [pcc:unverified]
