# Chained-S3 SAT on N83, m = 5…7: meet-in-the-middle lookup for the middle intermediate (preregistration)

Frozen 2026-11-01 before the measured run. Follow-up to
`research/n83_chain_sat_order_20261031`, where every arm was wall-capped at
m ≥ 5 because depth-2 root-finder tables do not reach the middle
intermediates.

## Question

On the pinned N83 fixture (`icv1-f2m83-t6151469093347-cdcc5432`,
`KoblitzCurve::known_n83_k1`), base = Frobenius union of span(1, z)
(167 points), how many public and planted queries does the chained-S3 SAT
decomposer decide for m = 5, 6, 7 within the caps when the middle
intermediate y_(d−1) is confined by the meet-in-the-middle lookup
(`ChainS3Tables::meet = d`), and at what cost?

## Arms (same targets, arm order rotated per target)

| id | arm | m = 5 | m = 6 | m = 7 |
|---|---|---|---|---|
| A2 | `all-summands+t2.2` (best arm of the 10-31 run; same-run control) | ✓ | ✓ | ✓ |
| A4 | `all-summands+t2.2+meet<d>` | d = 3 | d = 3 | d = 4 |

The lookup: x-fingerprints of every signed d-sum of base points, built once
per process before the first query (its build time is reported on a
`warmup` line, not in query seconds); per query, every signed (m − d)-sum s
gives R − s, kept when its x lies in that set. Patterns with the identity
at y_(d−1) are refuted without a solve unless R is itself a signed
(m − d)-sum; all other patterns are refuted without a solve when the domain
is empty. A refutation means "no decomposition", as for A2.

## Targets and caps

- Seed 20261031, 3 public `[a]G` and 2 planted sums of m base points per m:
  the same targets as the 10-31 run at these m (same seed, counts and
  generator), so A2 also pairs with the 10-31 records.
- Per query: 300,000 conflicts and 120 s wall, lookup enumeration included.
- Command per m (`run_shard.sh <bin> raw/m<m> <m> <d>`):
  `target/release/examples/n83_chain_sat_bench <m> 2 3 2 300000 20261031 all-summands+t2.2,all-summands+t2.2+meet<d> 120`
- Three processes in parallel (m = 5, 6, 7), one core each,
  `RAYON_NUM_THREADS=1`.

## Measurement

Counted units: solver conflicts and, for A4, the lookup's point additions
(declared, not counted per query: (m − d)-multisets of the 334 signed base
points, ≈ 5.6·10^4 at m = 5 and ≈ 6.3·10^6 at m = 6, 7). Wall seconds are an
L0 diagnostic (shared 4-vCPU cloud VM). Every witness is re-added in the
curve group (`verified`).

## Decision rule (frozen)

- Positive control: an arm that returns `refuted` on a planted target voids
  that arm at that m.
- Improvement claim (A4 over A2), per m, no pooling: more decided public
  queries or more verified planted witnesses, with no planted target lost.
- Cost reported beside it: warmup seconds, median query seconds, median
  conflicts, lookup domain sizes.

## Limits declared in advance

One base size, one curve, 5 targets per m, L0 host. m = 8, 9 are out of
scope: their per-query enumeration (4-multisets, ≈ 5.4·10^8 additions)
exceeds the 120 s cap on this host. Relation collection is not a DLP; no
index-calculus-versus-rho claim and no leaderboard row. EKS (AGENTS.md §9a)
is not used: not an ecbench method, and the session has no aws CLI.
