# Fast group order (N_1 only, baby-step giant-step) and genus 3 at p = 65537 … 262147 (frozen protocol, 2026-11-21)

Lead C4 of `research/hecdlp_campaign_20261121/CAMPAIGN.md`. The group-order routine of
`genus_ladder.rs` enumerates `N_1` and (for g >= 3) `N_2` over F_{p^2}, an O(p^2) scan that
stopped genus 3 at p ~ 2^15 and genus 4 at 2^13 (`genus_ladder_scale_20261115/RESULTS.md`).
`OrderMethod::Bsgs1` takes `N_1` only (O(p)), fixes `c_1`, and runs the existing one-dimensional
baby-step giant-step over the wider Weil window of `#J` (the unknown `c_2`, `c_3` only shape it).
`Bsgs2` is the N_1 + N_2 window of today's routine; `Enumerate` is today's routine, unchanged.
Every figure `[pcc:unverified]`.

## Part 1: equality, before any new run
`Bsgs1` (and `Bsgs2` where N_2 is enumerated), against `Enumerate`, on (a) every genus-3 instance of
`genus_ladder_20261114/analysis.json` (56 instances, p = 67 … 4099), (b) the nine of
`genus_ladder_scale_20261115/g3_analysis.json` (p = 8209, 16411, 32771), (c) 50 random curves at
p = 1031 (all seeds, undetermined/ambiguous outcomes included). Outcome compared as `ok #J` or
`err <class>`. `count_points(2)` is rewritten (Barrett reduction, one reduction per coordinate and
step); every count is compared with the pre-change enumeration on test curves, and
`cargo test --release --lib genus_ladder::` (incl. `sge_none_reproduces_frozen_counts`) must pass.
Falsified if any outcome differs, any recorded order differs, or any count changes.

## Part 2: re-find the earlier instances with the new route
`genus_ladder run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 0 --max-h 8 --seed-search 600
--order-method bsgs1`: the admitted instance lines must equal those of `scale_g3.jsonl` field for field
(curve seed, f, order, h, r), and the cell rejects must equal.

## Part 3: extension run
```sh
genus_ladder run --primes 65537,131101[,262147] --genera 3 --curves 3 --seeds 2 --arms cR --sge 8,3 \
    --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --seed-search 600 --order-method bsgs1 \
    --rho-cap 3e9 --out ext_g3.jsonl
```
Deviation from the task line, declared before data: `--rho-cap 3e9` is added, because at the default
3e7 the expected rho steps `sqrt(pi r / 4)` of an `h = 1` instance at p = 131101 (4.2e7) exceed the cap
and the rho rows would be skipped. 262147 is run only if the order search fits memory (m < 2^26, a
computed bound) and the 131101 cell's wall is under two hours. Under `nohup`; the order search's wall per
admitted instance is recorded in the instance lines (`order_search_wall_s`) and per cell
(`search_wall_s`, which includes rejected seeds).

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| O1 | every IC run (merged and unmerged) and every rho run certifies with the planted t | any does not |
| O2 | 2LP + prefilter + SGE(8,3) over rho, FPM primary: <= 0.25 at 131101 and below the 32771 value 0.283 | > 0.25 at 131101, or not below 0.283 |
| O3 | power-law exponents of the FPM cost vs p over 8209 … 131101 (joining `sge_g3.jsonl` and the rho rows of `scale_g3.jsonl`): SGE arm in [1.30, 1.42], rho in [1.40, 1.55] | outside either interval |
| O4 | per admitted instance at 131101, the order search costs less wall than the solver runs (merged arm, per seed) on it | search wall per admitted instance >= solver wall |
| O5 | Part 2 reproduces the earlier instances field for field | any mismatch |

## Limits
Same host and thread count as the earlier studies; 3 curves x 2 seeds per prime; no independent replay;
the exponent fits use four primes. Largest-p claims for the order route are memory bounds computed from
the window formula (tested up to the primes actually run).
