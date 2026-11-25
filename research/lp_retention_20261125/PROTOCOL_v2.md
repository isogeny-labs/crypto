# Online admission of partial relations under a memory cap: frozen protocol v2

Date 2026-11-25 UTC. Draft PR #5186. Frozen before any v2 measurement. [pcc:unverified]

## Why v2 replaces v1

v1 (`PROTOCOL.md`, results kept in `results/`, `results_explore/` and `results_confirm/`) scored
*offline* selections of partials by LP left nullity. Review found three design faults:

1. **Wrong metric.** LP left nullity counts large-prime-cancelling combinations, not useful
   factor-base rank, and it saturates far above the factor-base size. On the real n = 20 stream,
   v1 counted 14,738 combinations at budget 0.25 while the solver needed rank 14.
2. **Inconsistent premise.** `degree` and `spectral` must see the whole batch before choosing. That
   contradicts a memory budget, so only the size and arity rules were deployable.
3. **Little was learned.** The v1 headline (singletons first) is the one-unknown-versus-two
   arithmetic of large primes.

## Model and metric

- Relations arrive in collection order, each tagged with its draw.
- Complete relations are always kept.
- A partial is stored only if an admission rule accepts it on arrival and fewer than `cap`
  partials are stored. Nothing is evicted.
- Stored partials go into `ForestWitnessExtractor`. Each closed cycle is harvested immediately as a
  combined relation.
- **Primary score:** the draw at which the exact rank over F_ℓ of the factor-base block (kept plus
  harvested rows) first reaches the target.
- **Secondary score:** the draw at which it first reaches ⌈0.9 · target⌉, read from the same run's
  trace. Full rank waits for the rarest factor-base prime and is very noisy.

| rule | stores |
| --- | --- |
| `all` | every partial, no cap (upper reference) |
| `complete_only` | no partial (lower reference) |
| `first_come` | partials in arrival order until the cap |
| `singles_only` | one-LP partials until the cap |
| `seen_lp` | one-LP partials, and two-LP partials with a previously seen large prime, until the cap |
| `max_lp_b` | partials whose large primes are all ≤ b, until the cap (F_p* only: b = L/4, L/16) |

Correctness is checked by `tests/lp_retention.rs` (5 tests):
- the incremental rank equals `projected_rank::audit_projected_rank`;
- every harvested combination replays through `large_prime_filter::replay_complete_relation`;
- `all` ≥ every rule ≥ `complete_only` in rank at every draw;
- caps and rule semantics hold;
- the F_p* collector replays, with a tamper check.

## Streams

- **FP6, FP7, FP8.** `--source fp --p-index 6,7,8`, p = nextprime(2^31 + 11·10^6·i). B = 1024
  (|FB| = 172), L = 65536, up to 400,000 draws, **10 seeds** each (`--repeats 10`, seeds
  1000·i + r). Caps 2, 5 and 10 × |FB|. Target |FB|.
  - Parameters were chosen on the held-out prime `--p-index 9`, which is not reported. That
    calibration found that caps ≤ |FB| make every rule equal `complete_only`, and that `all` stores
    about 11 × |FB| partials before reaching full rank.
- **REAL20a, REAL20b, REAL20c.** `lp_graph_collect --n 20 --small 5 --envelope 9 --summands 4
  --large-primes 2 --trials 60000 --graph filter` with `--seed` 20261123, 1 and 2 (three binary
  curves), ℓ = the instance order.
- **REAL24.** `lp_graph_collect --n 24 --seed 20261123 --small 6 --envelope 11 --summands 4
  --large-primes 2 --trials 120000 --graph filter`.
- For every REAL stream: draw = line number, caps 2, 5 and 10 × |FB|, and target = the useful rank
  `all` reaches on the whole stream. No `max_lp` rule there, because LP identities are point labels.

## Statistic and decisions

For FP, per prime, rule and cap: the ratio `draws(rule) / draws(all)` per seed, the mean over
10 seeds, and a 95% percentile bootstrap (10,000 resamples, seed 7). A rule that never reaches the
target within the budget is censored at the budget and flagged.

- **B1.** Partials matter: `complete_only / all` has a lower bound > 1 on every FP prime, for the
  primary and secondary scores.
- **B2.** At each cap, rule X beats `first_come` if X / `first_come` has an upper bound < 1 on all
  three FP primes. Prediction: at caps 5 and 10, `max_lp_4096` and `singles_only` beat
  `first_come`; at cap 2, every rule is within 5% of `complete_only`.
- **B3.** For REAL streams, report every ratio to `all` (single streams, no interval). Prediction:
  every rule reaches the target within 10% of `all`'s draws at every cap, because LP columns are
  scarce (228 on n = 20) and cycles close fast.

## Scope

Exact counts of a component: no timing, no benchmark row, no leaderboard change, no ECDLP speed
claim. All runs are local and take minutes.
