# Chained-S3 on N83, m = 8, 9: indexed lookup and rebuild by peeling (preregistration)

Frozen 2026-11-01 before the measured run. User request: "add m=8 and m=9
after this run". Follow-up to `research/n83_chain_sat_rebuild_20261101`
(#3191): lookup + rebuild decided every public and planted target for
m = 5…8, at a median 72–82 s per query at m = 8, dominated by a binary search
in the depth-4 fingerprint set; at m = 9 (#3149) the lookup alone exceeded
120 s per query, and a rebuild would re-enumerate ≈ 1.15·10^9 depth-5 sums.

## Arms (same targets, arm order rotated per target)

| id | arm | d (m = 8 / 9) |
|---|---|---|
| A5 | `all-summands+t2.2+meet<d>+rebuild` (merged in #3191; control) | 4 / 5 |
| A6 | `all-summands+t2.2+meet<d>+rebuild+indexed` | 4 / 5 |

`indexed` (`ChainS3Tables::indexed`): membership via a bucket index on the top
fingerprint bits instead of a binary search; rebuild by peeling the hit one
signed base point at a time against the cached depth-(k−1) sets (≤ d·167
point additions per witness) instead of re-enumerating the depth-d sums. All
depth sets 2…d are built before the first query (`warmup` lines).

## Targets and caps

Seed 20261031; per m 3 public, 5 planted (same targets as the #3191 run at
m = 8, and the same generator at m = 9). Per query 300,000 conflicts, 120 s.
Commands: `run_shard.sh <bin> raw/m8 8 4` and `run_shard.sh <bin> raw/m9 9 5`,
in parallel, one core each.

## Decision rule (frozen)

- Soundness: a refuted planted target voids the arm at that m; an unverified
  witness is a soundness flag. A6 must give the same public verdicts as A5
  wherever both decide.
- Improvement (A6 over A5), per m: more decided queries (public refuted +
  planted verified), or, where both decide every query, lower median seconds
  on paired targets.

## Limits

L0 host, 8 targets per m, one base, one curve; decomposition stage only (no
DLP, no leaderboard or benchmarks row); EKS not used (not an ecbench method,
no aws CLI).
