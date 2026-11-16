---
id: LEAD-ICP-m4-instance-set
type: lead
title: Frozen m = 4 (S_5) decomposition instance set
tags: [prime-field, ecdlp, semaev, benchmark, instrument, m4]
confidence: measured
status: closed-known
source_refs: []
added: 2026-11-04
superseded_by: null
next_action: extend to p = 2^56 and 2^72 and attach exact curve orders (SEA) before any cross-solver comparison
---

## Statement
Any claimed m = 4 oracle must be scored on one frozen instance set: p in {2^40, 2^56, 2^72, 2^96},
`B = floor(p^(1/5))`, 100 planted instances each, with the matched-rho reference.

## Delivered
`experiments/m4_instances.py` -> `experiments/m4_instances.jsonl`: p in {2^40, 2^48},
`B = floor(p^(1/5))` (256, 776), 20 planted instances per prime, fields: `x` (four base
x-coordinates), `x_R`, group-law check, `S5_value` (resultant recursion, 0 on every sampled
witness), and the relation density `log10(B^4/p) = -log10(B)`.

## Design constraints (must be kept)
* At this B a **random** target decomposes with probability `B^4/p = 1/B` (1/256 at 40 bits).
  Instances are planted; unplanted targets are negative controls. Any measured exponent must
  divide by the density.
* Arity 4 is the first arity that can beat rho: m = 2 never, m = 3 ties only with an O(1) oracle.
  Win gate: `T_dec = B^s` with `s < m-3` (`s < 1` at m = 4), equivalently `e < (m-3)/4` at the
  tie point `B = p^(1/4)`.
