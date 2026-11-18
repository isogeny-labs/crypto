# Why the n=83 Frobenius-orbit factor-base sweep produced zero relations

Forensic diagnostic, 2026-11-18. Read-only on research data; no launcher, pod or
heavy job was run. Every number below is cited to a file; quantities I computed
from those files are labelled **estimate** and the arithmetic is in the
reproduction note (`REPRO.md`), checked against the sweep's own
`support-moments.json`.

## TL;DR verdict

The zero-relation outcome is **not** one thing. It splits cleanly by summand
count `m` on the primary `a=0`, 81-bit curve (order
`2417851639230796216685689`, `log2 r = 81.000`), factor base = `K` signed
Frobenius-orbit columns (`N` distinct points; `K=64/256/600 -> N=10,624 /
42,496 / 99,600`):

- **m <= 4: FUNDAMENTAL (base undersized).** The expected number of
  `m`-decompositions of a uniform target is `E = N^m / m! / r`. For `a=0`,
  `K=600`: `m=2 -> 2.1e-15`, `m=3 -> 6.8e-11`, `m=4 -> 1.7e-6`. These are not
  caps: the `m=2` complement oracle made **87,966,720 exhaustive lookups over
  1,728 targets and found zero** relations
  (`koblitz_n83_factor_base_sweep_20261008/RESULT.md:269`), and the `m=4` (2+2)
  meet-in-the-middle probes on the primary **completed every state**
  (172,640 at K=64 in 39.6 s; 2,730,368 at K=256 in 676.9 s) and returned
  `MISS` (`RESULT.md:94-109`). A completed exhaustive search that finds nothing
  is the signature of a structurally empty sumset, not of starvation. More
  compute cannot raise a `1e-6`-to-`1e-15` hit rate into relations.

- **m = 5: MARGINAL probability AND solver-cap-limited (INCONCLUSIVE).**
  `E(m=5) = 0.0338` for `a=0 K=600` (`RESULT.md:167,283`; matches
  `support-moments.json`). So decompositions exist for ~3.4% of targets,
  ~30 targets per relation **estimate**. But every `m=5` natural search was a
  conflict-capped SAT call returning zero models: 1,000 / 5,000 / 10,000 /
  100,000 and finally 1,000,000-1,500,000 conflicts
  (`RESULT.md:214,375,376,84-92`; `README.md:3-9,254-279`). The 100,000-conflict
  K2048 call burned 2,846.7 s on **one** target and found nothing
  (`RESULT.md:84-92`). The sweep ran ~1 natural target per arm (< the ~30
  needed) *and* never completed a single per-target solve.

- **m = 6: relations are PLENTIFUL, but the only tool tried was the wrong one
  (TUNING / TOOLING).** `E(m=6) = 560.8` for `a=0 K=600` (`RESULT.md:283` calls
  the bound "vacuous", i.e. essentially every target decomposes). The sweep
  attacked `m=6` **only** with conflict-capped XOR-SAT (CryptoMiniSat / WDSat),
  which exhausted 5,000 / 20,000 / 100,000 conflicts with zero models
  (`RESULT.md:111-130`, `README.md:80-98`). It never built the direct 3+3 orbit
  meet-in-the-middle decomposer that the group arithmetic plainly permits. SAT
  is a poor search engine for a hash-join problem.

**The single decisive control already in the evidence:** when the five summand
x-coordinates were *fixed* to a planted witness, the identical SAT encoding
returned a group-verified model in **787 conflicts**
(`RESULT.md:69-74,156-165`). So the encoding, model-lifting and group-check all
work. The wall is the *natural search over the orbit domain*, nothing else.

**Was the ~23,100 s wasted on "too few candidates"?** No -- it was mostly not
spent on relation collection at all. The ledger is construction, generic
replay, S3 round-trips, capacity/model-build gates and a handful of
single-target conflict-capped SAT calls (`RESULT.md:214,237`,
`README.md:34`). There was never a many-target relation-collection campaign at
a viable `(m, K)` point. Re-running *this* configuration at scale is worthless.
A *modest new build* (m=6 3+3 orbit MITM) is the thing to price.

## Arms vs caps vs outcomes (primary a=0 unless noted)

| # | Run (evidence path) | shape `m` | base `K` | tool | search cap | candidates examined | outcome |
|--:|---|---|---:|---|---|---|---|
| 1 | pilot two-summand probes `RESULT.md:269,225` | 2 | 64/256/600 (54 bases, a0+a1) | exact complement oracle | exhaustive per base | 87,966,720 lookups / 1,728 targets | **0 relations** |
| 2 | primary compact MITM K64 `RESULT.md:94-101` | 4 (2+2) | 64 | exact enumeration | none (ran to end) | all 172,640 states, 1 target | `MISS` (0) |
| 3 | primary compact MITM K256 `RESULT.md:103-109` | 4 (2+2) | 256 | exact enumeration | none (ran to end) | all 2,730,368 states, 1 target | `MISS` (0) |
| 4 | diagnostic cold x3 `RESULT.md:277`, `cold-diagnostics.json` | 4 (pair) | 64/256 (a1 53-bit) | compact/enumerate | 600 s wall | capped before completion | `UNKNOWN_budget`, empty logs |
| 5 | structured d12 orbit `RESULT.md:375` | 5 | 2,001 cols | XOR-SAT | 10,000 conflicts | 1 target | `UNKNOWN_solver_cap` (0) |
| 6 | matched K1182/K2048 `RESULT.md:376` | 5 (orbit) | 1,182 & 2,048 | XOR-SAT | 5,000 conflicts | 1 target each | `UNKNOWN_solver_cap` (0) |
| 7 | ordered first-pair K2048 `RESULT.md:214` | 5 | 2,048 | XOR-SAT | 1,000 conflicts | 1 target | 0 models/rank |
| 8 | K2048 high-cap natural `RESULT.md:84-92` | 5 | 2,048 | XOR-SAT | 100,000 conflicts (2,846.7 s) | 1 target | `UNKNOWN_solver_cap` (0) |
| 9 | K4096 orbit model `RESULT.md:63-73` | 5 | 4,096 | XOR-SAT | 5,000 conflicts | 1 target | `UNKNOWN_solver_cap` (0) |
| 10 | mixed-orbit 3+2 (K1182/K2048, K600/K2048, cross-stream K600/K600) `RESULT.md:378,380,381,212` | 5 (3+2) | nested | XOR-SAT | 5,000 conflicts | 1 target each | `UNKNOWN_search_cap` (0); cross-stream shares 0 orbits |
| 11 | hosted m6 Gray/hash pairs `RESULT.md:111-130`, `README.md:80-98` | 6 | 600 | XOR-SAT (CMS/WDSat) | 5,000 / 20,000 / 100,000 conflicts | 1 target/arm | `UNKNOWN_search_cap` (0) |
| 12 | native K2048/K4096 high-conflict `README.md:3-9,254-279` | 5 | 2,048 & 4,096 | XOR-SAT | 1,000,000-1,500,000 conflicts | 1 target | 0 models/relations/rank |
| 13 | **planted known-witness** K1182/K2048 `RESULT.md:69-74,156-165` | 5 | 1,182 & 2,048 | XOR-SAT, x-coords fixed | n/a | 1 planted target | **model found @ 787 conflicts, group-verified** |

Conservative local charge across these rounds: **~23,100 s**
(`RESULT.md:162`, `README.md:34`); `selected_best_total_runtime = null`,
`primary_a0_complete_cold_runs = 0` (`cold-diagnostics.json`).

## Expected decompositions per candidate at n=83, vs what was tried

`E(uniform target) = N^m / m! / r`. This is the sweep's own first-moment ceiling
(`SUPPORT_MOMENTS.md`, `support-moments.json`); I re-derived it independently and
it matches their stored numerator/denominator exactly (e.g. `a0 K600 m=4`
numerator `N^4/4!`, denominator `r`, `= 1.695987e-6`). **Basis & uncertainty:**
this is an *upper bound* on `E` under a uniform-target-with-repetition model. The
real signed-Frobenius decomposition must respect orbit/phase/sign structure and
is for a *fixed* public target, so the true per-target probability is <= `E`,
possibly by one or more orders. Treat all `E` below as optimistic ceilings.

Primary `a=0`, `K=600`, `N=99,600`, `log2 r=81.0`:

| m | `E` per target (ceiling) | targets per relation `1/E` (est.) | what the sweep actually tried | diagnosis |
|--:|---:|---:|---|---|
| 2 | 2.1e-15 | 4.9e14 | 87.9M exhaustive lookups, 1,728 targets | fundamental: `1/E` unreachable |
| 3 | 6.8e-11 | 1.5e10 | not attempted directly | fundamental |
| 4 | 1.7e-6 | 5.9e5 | **exhaustive** 2+2 MITM, found 0 | fundamental (full system needs ~3.5e8 decomposing targets) |
| 5 | 3.4e-2 | ~30 | ~1 target/arm, every SAT call conflict-capped, 0 completed | starved in target count AND per-solve incomplete |
| 6 | 560.8 | ~0.002 (every target) | capped SAT only; no direct MITM | plentiful, wrong tool |

The crux the task asks for -- *did they try far fewer candidates than
`1/Prob`?* -- has two different answers:

- At **m=2/m=4** the proximate search was **exhaustive** (not capped) and found
  zero, so the problem is not "too few candidates": `1/E` is simply larger than
  the entire sumset. Fundamental.
- At **m=5/m=6** a single *candidate* is one whole-target SAT solve, each of
  which is itself a `K^m`-scale search (`600^5 ~ 2^46`, `600^6/6! ~ 2^46`
  unordered -- **estimate**). The sweep ran ~1 such solve per arm (< the ~30
  targets `1/E` wants at m=5), and **no solve ever completed** within its
  conflict cap. So it is simultaneously target-count-starved and per-solve
  incomplete -- but the binding constraint is the per-solve search cost, which
  the sweep never drove to a single natural completion. That is why m=5/m=6 is
  INCONCLUSIVE on the evidence rather than a clean "needs N more targets".

### Cross-check against the priced ecbench n=13..41 study

`ecbench_frobenius_orbit_vs_rho_20261014/README.md` measures *working*
Frobenius-orbit IC (relations collected, rank, solve) at `n=13..41`,
`log2 r = 11..39`, with the orbit-column multiplier `c = 1,1,2,3,1,2,8,90`
(README table, lines 70-79). IC completes there because `c` is grown so the base
covers the group: at `n=41` the base needs `c=90`. Scaling that to the primary
`n=83` (cover `2^81` instead of `2^39`) demands a base **~`2^42`x larger** than
the `n=41` base -- i.e. millions of columns, not `K=600` **(estimate,
order-of-magnitude)**. This is the same conclusion the free-relation census
reaches from a different direction (below), and it confirms `K<=4,096` is
3-4+ orders of magnitude too small for low-arity IC on the primary. ecbench also
shows IC stays a *constant* multiple of rho (slope `beta = 0.165/ln r`,
`README.md:9,108-120`), so even a correctly sized n=83 base does not beat rho --
but that is the separate "beat rho" question, not "why zero relations".

## Why the free-relation census got 72 rows and the sweep got zero

`experiments/koblitz-free-relations-20261012/census_n83_k600_frozen.json`:
`a=1` (53-bit diagnostic, order `8569786107849059`), `orbit_columns=600`,
`factor_base_points=99,600`, `collision_pairs=9,672`,
`trivial_only_pairs=9,600`, `nontrivial_relations=72`,
`verified_relations=72`, `homogeneous_rank=6`, `rank_deficit_vs_columns=594`.

These are **homogeneous / free** relations: zero-sums *among factor-base points
themselves*, `A+B+C+D=O`, found as birthday collisions between pair-sums
(`(A+B) = -(C+D)`) over ~29.9M enumerated pair-states. **No target is
involved.** That is why they are cheap: collisions are found by sorting, and
`ic_free_relation_census_20261011/RESULTS.md:35-49` gives the law -- distinct
free relations `~ K^4 n^3 / (3 r)`, rank `~ rows/3`. At `n=83, a=1` this
predicts ~3 (measured 6); to cover the rank you would need
`K >= (3 r / n^3)^(1/3)` = **~3,556 for the a=1 curve, ~2.33e6 for the a=0
primary** (`RESULTS.md:44-47`; I reproduced both).

The sweep needed the opposite object: **inhomogeneous** relations that decompose
an *external* target `R=[a]G+[b]Q` over the base. That is the hard direction --
it is governed by `E = N^m/m!/r` above, not by the within-base birthday
collision rate. The census succeeds (a little) because:
1. it is homogeneous (no target to hit),
2. collisions are a sort, not a `K^m` SAT search,
3. it only claims rank 6 of 599 needed -- it explicitly does **not** solve a
   DLP (`census..json` `claim_boundary`: "No target solved, no DLP recovered").

So the census and the sweep are not in tension: the census confirms that even
the *easy* homogeneous relations run out of rank at `K=600` (deficit 594), and
its scaling law independently says the base is orders of magnitude too small --
the same verdict the support moments and ecbench give for the hard
inhomogeneous direction.

## Verdict, by the task's categories

- **FUNDAMENTAL for m <= 4 on the primary.** `E <= 1.7e-6`; the exhaustive
  m=2 and m=4 searches found zero as predicted. No parameter or budget change at
  these arities yields relations; the `K<=4,096` orbit base is 3-4+ orders too
  small (support moments + free-census scaling `K~2.3e6` + ecbench `c`-scaling,
  three independent routes).
- **m = 5: INCONCLUSIVE, leaning tooling/solver-cost.** `E=0.034` is marginal
  but nonzero; the planted-witness control proves the encoding solves at 787
  conflicts, so the obstruction is the natural orbit-domain search, never
  completed in any capped SAT call. Cannot distinguish "needs a much larger
  conflict/wall budget and ~30 targets" from "the Semaev/SAT search is
  infeasible at n=83" from the evidence.
- **m = 6: TOOLING, likely affordable.** Relations are plentiful (`E~561`);
  zero was produced only because the sole tool was conflict-capped SAT. A direct
  3+3 orbit meet-in-the-middle decomposer was never built. **Estimate:** build a
  `K^3 = 600^3 = 2.16e8`-entry signed-orbit triple-sum table once (~15 h
  single-core at the ~4,000 group-adds/s the compact probes showed, `RESULT.md:94-109`;
  far faster with a bit-sliced XOR oracle), then each target is a table scan;
  with `E~561` nearly every target yields many 6-sums, so a full rank-600
  relation set is plausibly **days of single-core, parallelizable** work -- not
  centuries. This is the one regime where re-spending money is justified, and
  only with the new decomposer, not the SAT path.
- **SEARCH-CAP STARVATION as a standalone cause: rejected.** The two regimes
  that were conflict-capped (m=5, m=6) are exactly the two where the sweep used
  the wrong search engine (SAT for a MITM/enumeration job); the regime that was
  run to exhaustion (m<=4) is fundamentally empty. Raising conflict caps on the
  same SAT configuration (already taken to 1.5M conflicts, `README.md:273-279`,
  with zero models) is not the fix.

Net: **re-running the sweep as configured at scale is not worth the money.** The
decision-relevant, cheap next step is a bounded m=6 MITM probe (below). All `E`
figures are optimistic ceilings; the real orbit-structured counts could be
materially lower, which is exactly what the probe would measure.

## One cheap, bounded experiment that settles the inconclusive core

**m=6 (3+3) signed-orbit meet-in-the-middle decomposition probe on ONE primary
target.** (Describe only; do not run.)

- Build the signed-Frobenius triple-sum table for the retained `a=0 K=600`
  object: enumerate all `K^3 = 2.16e8` orbit triples with their phase/sign
  canonicalization, store `(value -> triple)` in a hash table (~a few GB; fits
  the existing 4-16 GiB cgroup). One-time, bounded, ~tens of minutes to a few
  hours.
- For one frozen public target `R`, scan the triple-sums `s` and look up `R - s`
  in the table; every hit is a verified 6-term orbit decomposition
  (group-check each, as the sweep already does for planted witnesses).
- **Measurements that decide it:** (a) wall to the first verified decomposition;
  (b) empirical decompositions-per-target vs the `E~561` ceiling (reveals how
  much orbit structure deflates the first-moment); (c) extrapolated cost of 600
  targets.
- **Decision rule:** if the probe returns decompositions at a rate within ~2
  orders of `E~561` in bounded wall time, the zero-relation result was a
  wrong-tool/tuning artifact and a full m=6 campaign is affordable (price it).
  If the real rate is deflated to near-zero despite `E~561`, the orbit factor
  base is *fundamentally* decomposition-poor at n=83 and scaling is not worth
  the money. Either way it is a few single-target solves -- no full attack, no
  linear-algebra or individual-log phase needed.

This is strictly cheaper than any arm already run (one bounded table build + one
target scan) and is the single measurement that converts the m=5/m=6
INCONCLUSIVE into a FUNDAMENTAL-or-TUNING decision.

## Sources checked

- `research/koblitz_n83_factor_base_sweep_20261008/README.md`,
  `RESULT.md`, `pilot-01/cold-diagnostics.json`, `pilot-01/support-moments.json`
  (sweep arms, caps, charges, support moments).
- `experiments/koblitz-free-relations-20261012/census_n83_k600_frozen.json`
  (the 72-free-relation census).
- `research/ic_free_relation_census_20261011/RESULTS.md`, `PREDICTION.md`
  (free-relation scaling law `K>=(3r/n^3)^(1/3)`).
- `research/ecbench_frobenius_orbit_vs_rho_20261014/README.md`,
  `tables/derived-phase-means.jsonl` (priced n=13..41 orbit-IC, column scaling,
  slope beta=0.165/ln r).

All quantitative claims flagged **estimate** are my own order-of-magnitude
arithmetic (reproduction in `REPRO.md`), not measured results; the sweep itself
measured no completed natural relation, rank, or cold runtime on the primary.
