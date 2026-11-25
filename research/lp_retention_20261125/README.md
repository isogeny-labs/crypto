# Which partial relations to keep under a row budget

Date 2026-11-25. Draft PR #5186. Port of experiment T3 of isogeny-labs/workspace#144 into
`large_prime_graph`. [pcc:unverified]

**Standing:** exact component counts on toy, synthetic and one real ECDLP stream, with source-exact
tests. Not a speed claim, not a benchmark row, no leaderboard change. Each count is the exact
`GainDsu` left nullity of the kept rows over F_ℓ, i.e. the number of independent combinations that
cancel every large prime.

## Code

| piece | where |
| --- | --- |
| Six policies, `split_by_arity`, exact scoring, coefficient-blind cycle count, F_p* toy collector with replay | `src/cryptanalysis/large_prime_graph/retention.rs` |
| Evaluator: `--source fp\|synthetic\|relations`, budget sweep, `--relabel` | `examples/lp_retention_eval.rs` |
| Tests (7) | `tests/lp_retention.rs` |
| Source-exact wrapper crate (symlinks the unmodified production modules) | `native/` (`bash native/run.sh`) |

The six policies are `random`, `smallest_lp`, `degree`, `spectral`, `core_then_random` and the
exploratory `singles_then_spectral`.

`bash native/run.sh` passed:
- **41/41** existing `tests/large_prime_graph.rs` tests;
- **7/7** new tests, including `GainDsu` left nullity equal to dense Gaussian rank for every policy
  on 120 random batches over F_3, F_7, F_101 and F_1000003, unit and random coefficients.

The root crate also builds at this branch: `cargo build --release --example lp_graph_collect`,
15 min 42 s on the session VM.

## Results

The protocol, `PROTOCOL.md`, was frozen before any data. Addendum A was frozen before its
confirmation data. Intervals are 95% bootstrap over 5 repeats.

| hypothesis | outcome |
| --- | --- |
| **H1** F_p*: `smallest_lp` beats `degree`/`spectral` at budget 0.25 | **supported.** smallest/degree = 1.32, smallest/spectral = 1.30 on FP0–2, every lower bound > 1.29. This independently reproduces workspace#144's Python figure (degree/smallest = 0.757, i.e. 1.32) on fresh Rust-collected pools. |
| **H2** cross-implementation | **PASS.** workspace#144's Python `counts()` on the Rust FP0 pools gives U = Rust left nullity and b1 = Rust cycle count on 5/5 pools. All 174,672 relations replay by modular exponentiation in Python (`crosscheck_h2.py`, `results/h2_crosscheck.json`). |
| **H3** H1 at every budget 0.25…0.01 | **holds at every budget.** The gap grows as the budget shrinks: smallest/degree from 1.32 to 14.5 on FP0. |
| **H4** relabelled labels: `smallest_lp` ties `random` | **falsified.** It stays 1.54× random (FPr) and 2.67× (SYNr). Cause: the product-of-identities key ranks every one-LP row (one hash, about 2^63) ahead of every two-LP row (about 2^126), whatever the labels. On FP, singletons-first is ×1.54 over random and true size order adds only ×1.15 on top. |
| **H5** synthetic stream | `smallest_lp` leads at 0.25: /degree 1.17, /spectral 1.10, /random 3.30. |
| **H6** real ECDLP stream (binary n = 20, `ecbench_large_prime`, 59,864 partials, 228 LP columns, ℓ = 65557) | Every policy is within **1.5%** of random at budgets 0.25 and 0.10. At smaller budgets they separate (ratio to random at 0.01: smallest 1.54, degree 1.50, spectral 1.48). |
| **H7** (Addendum A) `singles_then_spectral` on fresh instances | **REJECTED as a default.** On FP3–5r and SYN2r it beats `smallest_lp`, `degree` and `spectral` at every budget (worst case 1.177 [1.155, 1.202]), and on plain FP3–5 it is ≥ `smallest_lp` (worst 1.006). On REAL, `degree` beats it at 0.25 (0.1%) and 0.10 (1.5%). |

The FP0, SYN and REAL rows are drawn in `figure.svg` / `report.pdf`. Full tables are in
`RESULTS_TABLES.md`.

The relabel internal check passed. Relabelling left `degree`, `spectral`, `core_then_random` and
`random` counts identical on every stream and budget. Re-running with six policies reproduced the
frozen five exactly, so the runs are deterministic.

## What this means

1. **In F_p\* and on Zipf-like synthetic streams, the textbook rule wins, but mostly for a reason
   other than size.**
   - Keeping one-LP rows first carries most of the effect.
   - On those streams, singletons first with spectral order inside each arity group is the best
     policy found, label-free, and confirmed on fresh instances.
   - Centrality alone (degree, spectral: the ideas II.B.5 and V.C.3 from the workspace#144 list) is
     23–24% below the textbook rule at 0.25 and far below at small budgets.
2. **On the one real ECDLP stream measured here, retention barely matters until the budget is
   small.**
   - That stream is extremely over-determined (59,864 rows on 228 columns).
   - The best a policy can do is about budget − 228, and at large budgets the winner is whichever
     policy keeps the fewest distinct columns, which is degree.
3. **No universal default.** A follow-up needs real streams where LP columns are not scarce: larger
   n, more large primes, the Koblitz orbit graph, the JV two-LP collector. Only those would show
   whether the singletons-first effect carries over to curves.

## Deviations and errors

- Twice an evaluator was run as `… 2>&1 | grep -m1` / `| head`. The pipe closed early, the process
  died of SIGPIPE before writing its JSON, and the run was repeated without the pipe. Same binary
  (sha256 in `results_confirm/binary.sha256`), deterministic output.
- `crosscheck_h2.py` first applied an extra condition of mine (b1 should differ by the number of
  squared large primes) and printed MISMATCH. Both conventions count a square as one cycle, so b1
  was equal, which is the frozen criterion. The script now applies the frozen criterion. The counts
  are unchanged.
- The wrapper runs only the integration-test targets. The existing `native_gain_dsu` metering shim
  is older than `large_prime_filter`'s own `cfg(test)` unit tests, which call newer `jv_bitops`
  functions. That is unrelated to this change.
- S3 resume checkpoint: no upload credential in this session. The pushed branch is the durable copy,
  as in earlier sessions.
- Sources checked: the research-kb MCP failed to connect (proxy 502). The fallback was the in-repo
  `research/lp_graph_algorithms_20261123/` notes (GainDsu, frame-matroid rank; Zaslavsky 1989/1991;
  Diem 2006 as cited there) and workspace#144.

## Reproduce

```sh
bash native/run.sh                                     # tests + evaluator build
B=$CARGO_TARGET_DIR/release/examples/lp_retention_eval  # or native/target/release/...
for i in 0 1 2; do $B --source fp --p-index $i --out results/fp$i.json
                   $B --source fp --p-index $i --relabel --out results/fp${i}r.json; done
$B --source synthetic --out results/syn.json; $B --source synthetic --relabel --out results/synr.json
python3 analyze.py
# Addendum A: --p-index 3,4,5 and synthetic --p-index 1 into results_confirm/; then
cargo build --release --example lp_graph_collect   # root crate
target/release/examples/lp_graph_collect --n 20 --seed 20261123 --small 5 --envelope 9 --summands 4 \
    --large-primes 2 --trials 60000 --graph filter --relations real_n20.jsonl --out eval.json
$B --source relations --relations real_n20.jsonl --modulus 65557 --out results_confirm/real.json
python3 confirm_h7.py && python3 make_report.py
```

Stream hash: `real_n20.jsonl` sha256 `caf7c3e7…296b` (`results_confirm/real_n20.jsonl.sha256`).
The stream itself is not committed (60,000 lines); the command above regenerates it.
