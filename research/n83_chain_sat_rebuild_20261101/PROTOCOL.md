# Chained-S3 on N83, m = 5…8: rebuilding planted witnesses from the lookup hit (preregistration)

Frozen 2026-11-01 before the measured run. Follow-up to
`research/n83_chain_sat_meet_20261101` (merged in #3149), where the
meet-in-the-middle lookup refuted every public target for m = 5…8 but no arm
found a planted witness within 120 s, although every planted target had a
non-empty lookup domain.

## Question

Does rebuilding the decomposition directly from a lookup hit
(`ChainS3Tables::rebuild`) recover verified planted witnesses for m = 5…8
within the caps, and at what cost, without changing public verdicts?

## Arms (same targets, arm order rotated per target)

| id | arm | d |
|---|---|---|
| A4 | `all-summands+t2.2+meet<d>` (merged lookup arm; control) | 3, 3, 4, 4 for m = 5, 6, 7, 8 |
| A5 | `all-summands+t2.2+meet<d>+rebuild` | same |

Rebuild: the back enumeration records the signed (m − d)-multiset s behind
each hit T = R + s; the signed d-sums are re-enumerated per query until one,
f, has x(f) = x(T); then f = ±T and R = (±f) − s, mapped to base indices and
re-added in the group before it is returned. When no hit lifts, the query
falls through to the SAT search unchanged.

## Targets and caps

- Seed 20261031; per m 3 public `[a]G` and 5 planted sums of m base points.
  The public targets and the first 2 planted targets are those of the 10-31
  and 11-01 meet runs (same generator order); planted targets 3–5 are new.
- Per query 300,000 conflicts and 120 s wall (lookup and rebuild included).
- Command per m: `run_shard.sh <bin> raw/m<m> <m> <d>` =
  `n83_chain_sat_bench <m> 2 3 5 300000 20261031 all-summands+t2.2+meet<d>,all-summands+t2.2+meet<d>+rebuild 120`.
- Four processes in parallel (m = 5, 6, 7, 8), one core each.

## Measurement

Counted: solver calls and conflicts; declared cost of a rebuild: one
re-enumeration of the signed d-multisets (C(168, 3) ≈ 7.9·10^5 at d = 3,
C(170, 4) ≈ 3.4·10^7 at d = 4, over the 167-point negation-closed base).
Wall seconds L0 diagnostic. Every witness re-added in the group.

## Decision rule (frozen)

- Soundness: any refuted planted target voids that arm at that m; any
  unverified witness is a soundness flag.
- Improvement (A5 over A4), per m, no pooling: more verified planted
  witnesses, with every public verdict unchanged (refuted by both).

## Limits declared in advance

L0 host, 8 targets per m, one base, one curve; m = 9 excluded (its lookup
alone exceeds the cap, #3149). Decomposition stage only: no relation
collection, no DLP, no leaderboard or benchmarks row. EKS not used (not an
ecbench method; no aws CLI).
