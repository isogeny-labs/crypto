# Which partial relations to store under a memory cap

Date 2026-11-25. Draft PR #5186. Started as a port of experiment T3 of isogeny-labs/workspace#144.
[pcc:unverified]

**Standing:** exact component counts on an F_p* toy and four real `ecbench_large_prime`
binary-curve streams, with source-exact tests. Not a speed claim, not a benchmark row, no
leaderboard change. "Draws" are collection trials; their cost is not priced here.

## Question and design (v2, the current design)

A collector cannot always store every partial relation. If it must decide **on arrival** which
partials to keep under a memory cap, which rule gets to a full relation matrix soonest?

- Complete relations are always kept.
- Stored partials enter `ForestWitnessExtractor`, and every closed cycle is harvested at once as a
  combined relation.
- **Score:** the draw at which the exact rank over F_ℓ of the factor-base block of kept and
  harvested rows first reaches the target, the full useful rank. A secondary score uses 90% of that
  rank, because full rank waits on the rarest factor-base prime.

The protocol, `PROTOCOL_v2.md`, was frozen at 2bffd4df4 before any v2 data.

| piece | where |
| --- | --- |
| `AdmissionRule`, `simulate_admission`, `IncrementalRank` | `src/cryptanalysis/large_prime_graph/retention.rs` |
| Evaluator (`--source fp\|relations`, `--caps`) | `examples/lp_retention_eval.rs` |
| F_p* toy collector with replay (not in the library) | `examples/lp_retention_support/fp_toy.rs` |
| Tests (5) | `tests/lp_retention.rs` |
| Source-exact wrapper crate | `native/` (`bash native/run.sh`) |

The tests check four things:
- the incremental rank equals the repo's `projected_rank::audit_projected_rank` (60 batches over
  F_7, F_101 and F_1000003);
- every harvested combination replays through `large_prime_filter::replay_complete_relation` to
  the same row (over 1,000 checked);
- `all` ≥ every rule ≥ `complete_only` in useful rank at every draw, which is a theorem: a
  subgraph's cycle space lies in the graph's;
- caps, rule semantics, and F_p* replay with a tamper check.

Results with `bash native/run.sh`: **41/41** existing `large_prime_graph` tests and **5/5** new
ones pass. `clippy -D warnings` is clean on the library, both tests and the example.

## Results

Each cell is draws to the target divided by draws with every partial admitted (`all`); lower is
better. FP cells are means over 10 seeds with 95% bootstrap intervals in `RESULTS_V2.md`.

**F_p\* toy** (p ≈ 2^31, B = 1024, |FB| = 172, L = 65536; primes FP6, FP7, FP8):

| | cap 2×\|FB\| | cap 5×\|FB\| | cap 10×\|FB\| |
| --- | --- | --- | --- |
| complete relations only | 1.85 / 1.65 / 2.09 | (same) | (same) |
| `first_come` | 1.84 / 1.65 / 1.94 | 1.60 / 1.63 / 1.91 | 1.28 / 1.27 / 1.48 |
| `max_lp` ≤ L/16 | **1.61 / 1.57 / 1.73** | **1.25 / 1.36 / 1.25** | 1.21 / 1.29 / 1.25 |
| `max_lp` ≤ L/4 | 1.81 / 1.65 / 1.92 | 1.51 / 1.44 / 1.59 | **1.10 / 1.13 / 1.00** |
| `singles_only` | 1.82 / 1.65 / 1.92 | 1.72 / 1.51 / 1.79 | 1.18 / 1.25 / 1.27 |
| `seen_lp` | 1.84 / 1.65 / 1.92 | 1.58 / 1.55 / 1.79 | 1.20 / 1.25 / 1.11 |

**Real ECDLP streams** (`ecbench_large_prime`, binary curves; one stream each, collection order;
the target is the useful rank `all` reaches on the whole stream):

| stream (\|FB\|; `all` reaches target at draw) | complete only | `first_come` @10× | `seen_lp` @10× | `singles_only` @5× | `singles_only` @10× |
| --- | --: | --: | --: | --: | --: |
| n = 20, seed 20261123 (14; 404) | 23.4 | 13.2 | 13.2 | 15.5 | **3.3** |
| n = 20, seed 1 (18; 264) | 18.6 | 11.5 | 11.5 | 11.5 | **3.6** |
| n = 20, seed 2 (12; 190) | 97.2 | 72.8 | 72.8 | 8.0 | **5.6** |
| n = 24, seed 20261123 (33; 1214) | 32.9 | 32.9 | 32.9 | 15.9 | 32.9 at 5×, **15.9** at 10× |

(For n = 24 the caps are 66, 165 and 330 rows. `singles_only` helps only at 10×, and those are the
15.9 figures.)

Decisions under the frozen rules (full lists in `RESULTS_V2.md`, `results_v2/verdict.json`):

- **B1 holds.** Partials matter on every FP prime, for both scores. Admitting all partials reaches
  full rank 1.65–2.09× sooner than complete relations alone, and 90% rank 1.54–1.56× sooner. On the
  real streams the factor is 18.6–97×.
- **B2.** With a binding cap on F_p*, the rule that beats `first_come` on all three primes is a
  **tighter large-prime bound**:
  - `max_lp ≤ L/16` at 2× and 5×;
  - `max_lp ≤ L/4` at 10×.
  - `singles_only` and `seen_lp` do not separate from `first_come` on the primary score.
  - When the cap stops binding (10×, 90% target), `first_come` equals `all`, and every filtering
    rule is worse than `first_come`, because it discards partials for nothing.
  - The predicted "every rule within 5% of complete-only at 2×" **failed**: `max_lp ≤ L/16` already
    helps there.
- **B3 failed badly.** I predicted every rule within 10% of `all` on the real streams. Instead,
  `first_come` and `seen_lp` stay close to complete-only, and **`singles_only` is 3–6× from `all`**
  on n = 20 at 10×.
  - The cause: about 90% of real rows are two-large-prime rows over only 228 large-prime columns
    (965 for n = 24). Filling a small cap with them closes few cycles, while one-large-prime rows
    pin components and pair up quickly.
  - This reverses the v1 conclusion that "retention barely matters on the real stream". That was
    an artifact of the v1 metric.

## What this means

- The right admission rule depends on whether the cap binds and what an LP identity means:
  - For F_p*, where identities are sizes, the classic knob wins: lower the large-prime bound to fit
    memory.
  - On these ECDLP streams, where identities are point labels and size carries no information,
    storing only one-large-prime partials is the effective rule.
  - When memory is ample, store everything.
- None of the graph-centrality ideas from the workspace#144 list helps here. `seen_lp` is their
  only online version, and it never beat `first_come` on the primary score.
- Limits:
  - Only four real streams, each a single collection order.
  - Draws are not priced: on curves a draw is a point-decomposition attempt.
  - Eviction is not modelled.
  - Factor bases are toy-sized (12–172).

## v1 (superseded)

v1 (protocol `PROTOCOL.md`; code at commit 17da67991; results in `results/`, `results_explore/`,
`results_confirm/`; scripts `analyze.py`, `confirm_h7.py`, `crosscheck_h2.py`) ranked **offline**
selections of a fixed fraction of the partials by LP left nullity. Its exact counts are correct,
including H2: workspace#144's Python agrees on 5/5 pools and 174,672 relations replay. Its design
answered the wrong question:

- the metric saturated above the factor-base size (14,738 combinations where the solver needed
  rank 14);
- `degree` and `spectral` need the whole batch in memory, which contradicts a memory budget;
- its "singles first" headline is ordinary large-prime arithmetic.

The v1 numbers should not be cited as evidence about which partials to keep. The v1 figure and PDF
were replaced by the v2 ones.

## Deviations and errors

- **v1 runs.** Two runs were killed by SIGPIPE from `| grep -m1` / `| head` and rerun. The H2
  script first applied an extra, wrong condition of mine, with unchanged counts.
- **v2 calibration** used the held-out prime `--p-index 9`, not reported. It surfaced an identical
  draw count for B = 512 and B = 1024 on one seed. I traced it in Python: the stream recomputes
  exactly, and three further seeds differ. It was a coincidence, not a bug.
- **Wrapper scope.** The wrapper runs only integration targets, because the old `native_gain_dsu`
  metering shim predates `large_prime_filter`'s `cfg(test)` unit tests.
- **Result format.** CI's `migrate` job, the artifact store, tried to move the original 2 MB FP
  result files to S3 and was refused (403). I regenerated all v2 results with the evaluator's compact
  output, which keeps `draws_to_target` and `draws_to_90pct` and drops the per-rank trace (pass
  `--trace` to keep it). All 554 outcomes matched the earlier files field for field, and
  `RESULTS_V2.md` and `verdict.json` regenerate byte-identically. Every file is under the 64 KiB
  threshold.
- **Infrastructure.** No S3 checkpoint credential in this session. research-kb unreachable (proxy
  502); the sources checked were `research/lp_graph_algorithms_20261123/` and workspace#144.

## Reproduce

```sh
bash native/run.sh                                  # tests + evaluator build
B=$CARGO_TARGET_DIR/release/examples/lp_retention_eval
for i in 6 7 8; do $B --source fp --p-index $i --repeats 10 --smooth 1024 --lp 65536 \
                      --draws 400000 --caps 2,5,10 --out results_v2/fp$i.json; done
cargo build --release --example lp_graph_collect     # root crate
C=target/release/examples/lp_graph_collect
$C --n 20 --seed 20261123 --small 5 --envelope 9 --summands 4 --large-primes 2 --trials 60000 \
   --graph filter --relations n20a.jsonl --out /dev/null   # also --seed 1, 2
$C --n 24 --seed 20261123 --small 6 --envelope 11 --summands 4 --large-primes 2 --trials 120000 \
   --graph filter --relations n24.jsonl --out /dev/null
$B --source relations --relations n20a.jsonl --modulus 65557 --caps 2,5,10 --out results_v2/real20a.json
#  ℓ: seed 1 → 262553, seed 2 → 65599, n = 24 → 2096401
python3 analyze_v2.py && python3 make_report.py
```

Stream hashes are in `results_v2/real_streams.sha256`, binary hashes in `results_v2/*.sha256`. The
streams themselves (5–11 MB each) are not committed; the commands above regenerate them.
