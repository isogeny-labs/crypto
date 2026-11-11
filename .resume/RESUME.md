# Resume: crypto @ aburan28/kind-volta-4zo2ul

## Goal
Owner's ask (2026-11-11): run the 5-seed pilot of the relation-oversampling protocol locally.
The pilot sets the seed count before the protocol is frozen. Pilot rows are not the measurement.

## Lead / hypothesis
research/logs_oversampling_20261111/PROTOCOL_DRAFT.md: does solving full_rank + ceil(f*columns)
relations (f in 0.1, 0.25, 0.5) lower total precompute cost vs f = 0? H0 favoured a priori.

## Spec
Protocol: research/logs_oversampling_20261111/protocol.draft.json (DRAFT). Instrument: `ic oversample`
(src/bin/ic/oversample.rs), release build. Cells C1 (deg 23 m2 fi0), C2 (deg 37 m3 ic-search spec),
C3 (deg 37 m3 subgroup_orbits 7400, CADO), C4 (= C3, legacy filter). 5 seeds, fractions 0,0.1,0.25,0.5,
--aa-control --logs-arm. Route: local, at owner's explicit request (not isolab-eks); shared container,
no isolation receipt, so wall times are indicative only.

## State
Release build then four cell runs, rows to research/logs_oversampling_20261111/pilot/.

## Runs in flight
Local `ic oversample` pilot, cells C1-C4 sequentially, in this container (session_01TCbLH7MBa3sEKfh3Trpi2R).

## Stop and success conditions
Stop when all four cells complete 5 seeds (fixed sample) or any arm fails verification (report it).
Success = rows for every (cell, seed, arm), all verified; then per-cell variance of the paired
difference to size the frozen run. No improvement claim from the pilot.

## Next steps
1. Summarise variance per cell; propose seeds_per_cell; report + graph + PDF (research-visuals).
2. benchrec record close; commit on PR branch.
