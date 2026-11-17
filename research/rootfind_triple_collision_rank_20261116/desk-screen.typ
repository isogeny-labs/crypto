#set page(paper: "a4", margin: 20mm)
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
= Triple-sum collision rank: preparation screen

*Status: proposed engineering screen, no scientific data.* Frozen protocol: `PROTOCOL.md`. Source: `tests/rootfind_triple_collision_rank.rs`. The mechanism adapts `crypto-autoresearcher:ledger/proposals/IDEA-20260928-4e2be7.yaml` and `ideas/catalogue-20260805/A1-index-calculus.md#A1-6` (internal provenance). Existing pair-collision data are at `research/rootfind_pair_collision_census_20261116/`. [pcc:unverified]

== Algebraic obligation

For physical factor-base points $F_i$, a verified equality of two triple sums yields a six-term group identity. Cofactor projection and the existing signed column map turn it into a homogeneous row modulo the subgroup order. Opposite sums give a plus row; equal sums give a difference row. A triple summing to the identity gives one row. The source test will check full point equalities before rank insertion. A matching x-coordinate by itself is insufficient. [pcc:unverified]

The actual nonzero factor-base logarithm vector annihilates every homogeneous row. Thus rank is at most $C-1$ for $C$ folded columns. Even if the ceiling is reached, one verified generator relation and one challenge relation remain necessary for recovery. [pcc:unverified]

== Fixed size screen

#table(columns: (auto, auto, auto, auto), inset: 5pt,
  [Cell], [Physical points], [All unordered triples], [Screen cap],
  [K13/d5], [33], [6,545], [6,545],
  [K19/d7], [139], [457,310], [32,768],
  [K23/d8], [275], [3,504,050], [32,768],
)

The triple counts are exactly $binom(B+2,3)$, not measured relation yields. The cap and deterministic sampling are fixed in `PROTOCOL.md`. Zero rows, dependent rows, rank increments, group additions, hash work, elimination, RSS and L0 wall diagnostics must be retained. [pcc:unverified]

*Before-data amendment.* `AMENDMENT-1.md` adds a paired generator control on the same unique triples. An on-demand cache stores the full two-point prefix, then adds the third point. The direct arm computes both additions. The source test requires exact full-point parity before collision processing and records cache hits, inserts and additions. No runtime or attack-cost ordering is inferred from this source preparation. [pcc:unverified]

`AMENDMENT-2.md` adds replay of every serialized row using the general binary-curve group law and records the first prefix reaching rank $C-1$, if any. Its prefix counters are a deterministic work projection, not a timed early-stop implementation. [pcc:unverified]

== Execution state

The science run requires a complete branch-specific S3 resume checkpoint and a passing `scripts/checkpoint.sh gate`. Conductor T-6814 currently reserves `.resume/RESUME.md`; the existing `problems/rootfinding-complete-cost-calibration.md` tracks this obstruction. The source is a prepared test only. A rank screen does not establish a complete-call improvement against F4/F5/F6 or matched rho; those comparisons require the separate frozen panel and independent replay. [pcc:unverified]

#pagebreak()
== Planned data flow

#image("flow.svg", width: 100%, height: 600pt, fit: "contain")
