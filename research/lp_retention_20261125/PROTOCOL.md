# Partial-relation retention under a row budget: frozen protocol

Date 2026-11-25 UTC. Draft PR #5186. Frozen before any measurement. [pcc:unverified]

## Question

When only a fraction of the partial relations fit, which ones should be kept? workspace#144 (T3,
Python, F_p* toy) found that keeping the partials with the **smallest large primes** beat degree and
spectral (eigenvector) centrality by 23–24%, with a coefficient-blind cycle count.

This protocol re-measures the question with:
- the exact `GainDsu` useful rank in Rust;
- a budget sweep;
- a relabel control that removes the size information `smallest_lp` relies on;
- the module's synthetic stream and, if the root crate builds, a real ECDLP stream.

## Instrument

- Code: `src/cryptanalysis/large_prime_graph/retention.rs` and `examples/lp_retention_eval.rs`.
- Build: the source-exact wrapper `research/lp_retention_20261125/native/` (`run.sh`).
- Metric per (stream, policy, budget): `GainDsuSnapshot.left_nullity` of the kept rows, the exact
  number of independent LP-cancelling combinations over F_ℓ.
- Pool: rows of canonical LP arity 1 or 2.
- Budgets: fractions 0.25, 0.10, 0.05, 0.02 and 0.01 of the pool.
- Policies: `random`, `smallest_lp`, `degree`, `spectral`, `core_then_random`.

## Streams

| id | source | parameters | repeats |
| --- | --- | --- | --- |
| FP0, FP1, FP2 | `--source fp --p-index i` | p = nextprime(2^31 + 11·10^6·i); g = least primitive root ≥ 1001; B = 4096, L = 65536, 150,000 draws | 5 (seeds 1000·i + r) |
| FP0r, FP1r, FP2r | the same pools with `--relabel` | LP identities replaced by a fixed bijective mix | 5 |
| SYN, SYNr | `--source synthetic` | 40,000 rows, LP arity weights 0:6:12, random coefficients mod 2^61 − 1; and relabelled | 5 (seeds r + 7) |
| REAL | `--source relations` | `lp_graph_collect --n 20 --seed 20261123 --small 5 --envelope 9 --summands 4 --large-primes 2 --trials 60000 --graph filter`, ℓ = the instance order `r`. Run only if the root crate builds; otherwise recorded as blocked. | 5 policy seeds on one stream |

## Statistic and decision rules

- For two policies A and B on one stream family at one budget, take the ratio
  `left_nullity(A) / left_nullity(B)` per repeat, the mean over repeats, and a 95% percentile
  bootstrap over repeats (10,000 resamples, seed 7).
- "A beats B" means the lower bound is above 1 on every stream of the family.
- "A ties B" means the interval lies within [0.97, 1.03] on every stream.

Hypotheses:

1. **H1, replication (FP, budget 0.25).** `smallest_lp` beats `degree` and `spectral`.
2. **H2, cross-implementation (FP0, all 5 repeats).** On the dumped pools, workspace#144's Python
   `counts()` gives `U` equal to Rust's full-pool `left_nullity` and `b1` equal to Rust's
   `cycle_count`. A Python replay of every relation by modular exponentiation succeeds. Any mismatch
   makes the run INVALID.
3. **H3, budget sweep (FP).** Report for each budget whether `smallest_lp` beats both centrality
   policies. Prediction: it does at every budget.
4. **H4, size information (FPr, SYNr).** `smallest_lp` ties `random`. Prediction: the best of
   `degree` and `spectral` then beats `random`.
5. **H5, synthetic (SYN, budget 0.25).** Report the same comparisons as H1.
6. **H6, real ECDLP stream (REAL).** Report every policy's ratio to `random` at every budget.

Internal check: relabelling must leave `degree`, `spectral` and `core_then_random` selections, and
so their counts, identical, because their tie-breaks use row position only. A difference makes the
run INVALID.

## Scope

Exact counts of a component: no timing, no benchmark row, no leaderboard change, no ECDLP speed
claim. All of it is development-scale local work, seconds to minutes on one core, so no isolab
submission is needed (CLAUDE.md: keep local development checks local).

## Addendum A: confirmation of `singles_then_spectral` (frozen 2026-11-25, before its confirmation data)

**Why it exists.** The frozen run falsified H4: relabelled `smallest_lp` did not tie `random`. Its
product key ranks every one-LP row before any two-LP row, whatever the labels. An exploratory sixth
policy, `singles_then_spectral` (one-LP rows first, spectral order within each arity), was added
after seeing those results. On the same data it beat every frozen policy on every stream and budget
(`results_explore/`). Those numbers are post hoc and do not count as evidence for it.

**H7 (confirmation on fresh instances).**
- Streams:
  - FP3, FP4, FP5: `--source fp --p-index 3,4,5`, new primes and seeds.
  - FP3r, FP4r, FP5r: the same pools relabelled.
  - SYN2 and SYN2r: `--source synthetic --p-index 1`, seeds 1007..1011.
- Same budgets and statistic as above.
- Claims:
  - (a) On every relabelled stream (FPr, SYN2r) and every budget, `singles_then_spectral` beats
    `smallest_lp`, `degree` and `spectral`.
  - (b) On the unrelabelled FP streams it is not worse than `smallest_lp` (lower bound ≥ 0.99) at
    every budget.
- Failing either claim on any fresh stream rejects the policy as a default.

REAL (H6), if the root crate builds, is scored with all six policies. It counts toward H7 (a) as the
one stream whose LP identities are genuine curve labels.
