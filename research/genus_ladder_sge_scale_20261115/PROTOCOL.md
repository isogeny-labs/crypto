# Genus 3 at p = 8209 … 32771: double large prime + prefilter + SGE against rho (frozen protocol, 2026-11-15)

Composition of two measured results: the scaling study (`research/genus_ladder_scale_20261115/`,
genus-3 2LP+prefilter at 0.54–0.74 of rho with three quarters of its cost in the linear
algebra) and the SGE study (`research/genus_ladder_sge_20261115/`, SGE(8,3) cuts that
linear algebra to about a quarter on identical relations). Every figure `[pcc:unverified]`.

## Run
The `genus_ladder` binary built from the current tree (sha in `binary.sha256`; its
`sge: None` path reproduces the frozen counts, see the SGE study), on the SAME nine
genus-3 instances and the same two seeds as the scaling study (deterministic from the
curve seeds; the instance lines must match `scale_g3.jsonl` field for field):

```sh
genus_ladder run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 \
    --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --out sge_g3.jsonl
```
The driver runs the unmerged arm beside the merged one. Rho is not rerun: the rho rows of
`scale_g3.jsonl` are the matched reference (same instances, same seeds), joined by
`genus_ladder_analyze` on the concatenation of the two files.

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| Z1 | instance lines identical to the scaling study's; the unmerged arm's rows identical to `scale_g3.jsonl` field for field (except wall) | any mismatch |
| Z2 | every run certifies with the planted t | otherwise |
| Z3 | `C_double_lp+pre+sge(8,3)` over rho, FPM primary: in [0.22, 0.42] at 32771, below rho across the FPM band at all three primes | outside, or a prime not below |
| Z4 | the merged arm's `la_mul` is 0.20–0.40 of the unmerged arm's at every prime | outside |
| Z5 | GAE band still unresolved at every prime (the native smoothness work at `w = 1` is untouched by SGE) | a GAE verdict of below |

## Limits
Same host, same sample sizes as the scaling study; no independent replay.
