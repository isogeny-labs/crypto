# Exponent study, rerun with resources: m = 3 and m = 4 (protocol, DRAFT)

**Status: draft, not frozen.** The experiment director freezes it: commit this
file, then record the commit hash here before any data. The session that wrote
it (2026-11-10) has no AWS identity and ran nothing. Queue item Q2 of
[`research/ic_track_ruling_20261110/DECISIONS.md`](../ic_track_ruling_20261110/DECISIONS.md).

**Parent.**
[`research/curve25519_exponent_study_20261108/`](../curve25519_exponent_study_20261108/)
(PROTOCOL frozen 2026-11-08). Its m = 3 and m = 4 series were censored by the
3.5 GB per-process memory limit and the 1,200 s per-target cap on a shared
4-core host. Its preregistered verdict there is "undetermined"
([README](../curve25519_exponent_study_20261108/README.md)). The question,
system, measured quantities, estimator, thresholds and predictions are the
parent's (§§1–3, 5, 6), **unchanged**. Only the resources and the grid's upper
end change. A rerun that changed the estimator would be a new study.

## 1. Question (parent §1, unchanged)

On the symmetrised Semaev system over multiplicative-subgroup bases of the
Curve25519-family miniatures, the parent fits the exponent s of the F4 cost
T(E,m,L) against |V|. Is there an exponent lever?

- At m = 3: s < 0, with upper 95% bound < 0.
- At m = 4: s < 0.5, with upper 95% bound < 0.5.

This is the direct measurement of the algebraic track's growth exponent that
the 2026-11-10 ruling asks for.

## 2. What changes

| item | parent | rerun |
|:--|:--|:--|
| memory per process | 3.5 GB (`ulimit -v 3500000`) | 112 GiB (the larger of the two D-FD7 memory classes, 56/112Gi) |
| wall cap per target | 1,200 s | 86,400 s (`--budget 86400`), the cap D-FD7 adopted for the fall-degree jobs |
| host | shared 4-core box, L0 | isolab-eks Spot, one pod per (k, m, d) cell, 1 vCPU pinned, `RAYON_NUM_THREADS=1`, L1 isolation receipt |
| grid upper end | the first capped cell | the same ascending d lists (parent §4), continued until the first capped cell under the new caps |
| cells re-run | none | every m = 3 and m = 4 cell of the parent, re-measured from d = 3, so the fit uses one instrument |

The stopping rule is the parent's: per (k, m, form), run d ascending and stop
at the first capped or out-of-memory cell. A capped cell is censored, never
fitted. Larger d are "not attempted".

## 3. Targets and seeds

Six targets per cell. The seeds are fresh: 20261110 + 1000 m + d, disjoint
from the parent's 20261108 + 1000 m + d. The parent's cells are not reused.

## 4. Estimator and verdict (parent §5, unchanged)

For each (k, m, form) with at least 4 complete cells whose |V| spans at least
4x: ordinary least squares of ln T on ln |V|. The fit range is d ≥ 9; d = 3, 7
enter a reported sensitivity fit. The slope s carries a two-sided 95%
t-interval with n − 2 degrees of freedom. The lever thresholds are as in §1.
The looser derived m = 4 threshold (s < 1) is reported beside the verdict and
does not replace it.

## 5. Predictions (parent §6, kept; a failed prediction stays failed)

- m = 3, sym: s = 3.5 (2.5 to 5).
- m = 4, sym: s ≥ 4.5 if measurable.
- Expected outcome: no lever at m = 3 or 4.

The rerun's value does not depend on a lever. A measured s is the growth
exponent the algebraic track has to beat.

## 6. Blockers this run also measures (queue item Q3)

- **p ≥ 2^31.** The tower engine refuses these, so k = 32 is out of reach
  ([scale-up README](../curve25519_algebraic_ecbench_scale_20261105/README.md),
  "Instrument"). This study uses `curve25519_family f4sub`, not the tower
  engine, and stays at k ≤ 24. Widening the tower engine is a separate source
  change. D-FD8 §6 adopted 64-bit closure widening as engineering.
- **Wall budget at k = 28.** The algebraic arm `pkm-iso-t10-g9-tr` finished
  0 of 2 solves at k = 28, and 2 of 6 at k = 24, where 4 hit the 3,000 s wall
  limit. The rerun records, per cell, the
  `solving_degree_max`, `max_rows` and `max_cols` that the parent's §3 already
  lists. That is enough to size a k = 28 autolab budget before one is spent.

## 7. Correctness gates (parent §7, unchanged)

Every system is checked against its planted decomposition before any cost is
recorded. A cell that fails a gate is excluded and reported.

## 8. Route and records

- isolab-eks (AGENTS.md §9a): one `isolab.eks.command/v1` job per cell, or
  Option 0 `isolab_k8s_run` until D-ROUTE is implemented.
- Resume checkpoint before the first job (`scripts/checkpoint.sh init`).
- A `bench.record/v1` record per cell.
- The report follows `research-visuals`: README, graph, PDF.
