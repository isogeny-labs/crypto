---
id: LEAD-ICP-sqf-global-relations
type: lead
title: Squarefree-part collisions as a free-relation source
tags: [prime-field, ecdlp, free-relations, global-lift, closed, xedni]
confidence: measured
status: exhausted
source_refs: [LEAD-TRF-global-lift-small-height]
added: 2026-11-04
superseded_by: null
next_action: none; the census is the closing measurement
---

## Statement (as proposed)
Two base points `x1, x2 <= B` with `sqf(f(x1)) = sqf(f(x2))` lie in one Mordell-Weil group
`E(Q(sqrt(d)))`; rank 1 gives a global relation that reduces to a free relation in E(F_p). Predicted
harvest: `B^(1/2+o(1))` relations.

## Measured (2026-11-04)
Exact squarefree kernels of `x^3 + 7` for every `x <= B` (`scripts/sqf_census.py`):

| B | 256 | 1024 | 4096 | 16384 | 65536 |
|---|---|---|---|---|---|
| colliding pairs | 1 | 1 | 1 | 1 | 1 |
| predicted B^(1/2) | 16 | 32 | 64 | 128 | 256 |

The only pair is `x = 2, 212` (kernel 15: `212^3 + 7 = 15 * 797^2`). The proposed experiment
("20 colliding pairs at B = 2^12") cannot be run.

## Why the heuristic fails
A collision is an integer point on `x^3 + 7 = d m^2`. For each d this is a Mordell curve, with
finitely many integer points (Siegel); the birthday model over "random integers of size B^3" gives
`sum_d C(k_d,2) ~ (log B)/B`, not `B^(1/2)`. This re-derives the global-lift/free-relation wall of
`RESEARCH_FREE_RELATIONS_CENSUS_20261012.md`.

## Disposition
Exhausted at the measured scale (1 pair for B <= 2^16). Under the strict standard of PROOFS.md this is *not* a proof of impossibility -- Siegel is not formalised -- so the lead is `exhausted`, not `closed-obstructed`. Do not schedule; keep the census table as the count attached to the xedni lead.
