# N83 summand sweep, m = 2…9: factor bases × decomposition engines (preregistration)

Frozen 2026-10-24 before any measured run. Stage diagnostic (relation
collection only) plus a complete-DLP reference; no IC-vs-rho speed claim.

## Question

On the pinned N83 fixture (`KoblitzCurve::known_n83_k1`,
`icv1-f2m83-t6151469093347-cdcc5432`, `y² + xy = x³ + x² + 1` over
`F_2[z]/(z^83 + z^45 + z^2 + z + 1)` (`0x800000000200000000007`), 53-bit
subgroup `r = 8569786107849059`, cofactor `1128547018`), what does each
decomposition engine cost per query, and what collection time does that
imply, as the summand count m runs 2…9 and the factor-base construction
changes? Where is the distance to the matched rho smallest?

## Arms (grid.tsv is the frozen list; one fresh process per cell)

| family | curve | engines | factor bases | m | probes |
|---|---|---|---|---|---|
| chained | n83k1 | chained-S3 full-width SAT: explicit domain (`chain-s3`), signed-Frobenius orbit domain (`chain-s3-orbit`), orbit + first-pair order (`chain-s3-orbit-ordered`) | Frobenius unions of `span(1, z, …, z^{S-1})`, S = 2, 3, 4 | 2…9 | public `[a]G` (4) and planted (2) |
| algebraic | n83k1 | matrix-F4, matrix-F5, inherited-F4, F6-IC, F6-IC pair index, crossbred, Weil-descent SAT (native XOR / CNF / base-restricted), MQ-FES | standard subspace D = 8 | 2…9 | planted (2) |
| companion | `KoblitzCurve::new(1, 61)` | the same ten algebraic engines, exact single-word field | standard subspace D = 8, 12, 16, 20 | 2…9 | public (16); planted (4) at D = 12, m = 2, 3 |
| reference | n83k1 | the repository's m = 4 root-table rootfinder (`koblitz_orbit_dlp_fast`, `construct:83:1:1107`, root11 and root22 layouts), full DLP | compact signed-orbit base K = 1107 | 4 | the exposed public point `point.jsonl` |
| rho | n83k1 | strong signed-Frobenius rho (`n83_rho_optimization_eval`, `rho_config.json`) | — | — | same point, 3 seeds, alternated with the IC arms |

Rootfinder: the m = 4 pipeline is the only n = 83 rootfinder in the
repository; the library pair table refuses fields wider than one word.

## Measurement

- Driver `examples/n83_msweep_cell.rs`; shard runner `run_shard.sh`
  (thin shell, `RAYON_NUM_THREADS=1`, `timeout` per cell, a `process_cap`
  record when a cell is killed); reference runner `run_reference.sh`.
- Per cell: verdict counts (witness / proved_unsat / incomplete /
  unsupported / invalid_model), verified witnesses (re-added in the
  group), seconds per query, engine work counter.
- Projection: `log2(relations_needed / p · seconds_per_query)`, with p the
  observed hit rate when a relation was seen, else the heuristic
  `|B|^m / (m! · #E)` (named in each record). **Only cells with ≥ 90 % of
  queries decided** (witness or proved UNSAT) get a projection; a cell
  whose queries return unsupported or incomplete has no price.
- Gap to rho: `log2(projected collection seconds / mean rho seconds)` on
  the same host — a distance remaining, not a pass/fail gate.

## Controls

- Positive: planted targets (a sum of m base points) per engine. An engine
  that misses a planted witness in a cell has its public rows in that cell
  marked uninformative.
- Field-exactness control: at n = 83 the algebraic engines must refuse
  (the one-word `FieldStructure` cannot hold the field). Before this
  study they returned "proved UNSAT" on planted targets; the fail-closed
  guard and regression test land in the same change.
- Same-point alternation of rho and IC reference arms.

## Limits declared in advance

One 4-vCPU cloud VM, L0 isolation, wall time only; shards run four at a
time, one core each. Relation collection is not a DLP; the companion rows
are a different curve (n = 61) and do not discharge the n = 83 gate
(AGENTS.md §8a). The rho reference uses a different polynomial basis of
the same curve (`icv1-f2m83-t6151469093347-9eba6cd5`, same trace and
order). §5a band: rho, the rootfinder and the SAT arms are priced in wall
seconds of complete native work, so the band is degenerate (no unpriced
native unit is converted).

## Model tiers

Core designs, preregisters and analyses. `experiment-director` (Sonnet)
compiles one frozen-command card per shard; the cards are relayed
verbatim to `executor-mechanical` (Haiku), which runs
`tools/task_card.py run`; the director assesses each receipt.
