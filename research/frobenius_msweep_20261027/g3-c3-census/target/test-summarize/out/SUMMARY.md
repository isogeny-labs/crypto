# Cell G3 census summary (generated; see PROTOCOL.md for the frozen rule)

## cell 17_8_2_0

label: **O-RELABEL**; tripwire: arm U flipped 0/0 planted SAT targets to UNSAT (not run in this cell)

pricing band (AGENTS.md 5a): A dearer than C at w=0,0.1,1 (3.003,3.003,3.003); B dearer than C at w=0,0.1,1 (1.200,1.200,1.200)

- control P1 S_m break: C/Cnobreak median 0.500 CI [0.500, 0.500] (expected in [0.5, 1.0]); effect detected
- control P2 gauge: B/Bnogauge median 0.059 CI [0.059, 0.059] (expected <= 1); effect detected
- control P3 Chalf/C median 0.500 CI [0.500, 0.500] (derivable, expected about 0.5; a miss voids)

| role | numerator | denominator | metric | targets | pairs | median ratio | 95% CI (cluster bootstrap) | geo-mean | frac<0.95 | sd(log) | MDE |
|---|---|---|---|---|---|---|---|---|---|---|---|
| primary | A@1 | C@1 | conflicts | 30 | 30 | 3.0030 | [2.9940, 3.0120] | 3.0017 | 0.0000 | 0.0064 | 0.9959 |
| primary | B@1 | C@1 | conflicts | 30 | 30 | 1.2000 | [1.2000, 1.2000] | 1.2000 | 0.0000 | 0.0000 | 1.0000 |
| control | Cpoly@1 | C@1 | conflicts | 30 | 30 | 1.0100 | [1.0100, 1.0100] | 1.0100 | 0.0000 | 0.0000 | 1.0000 |
| control | G@1 | C@1 | conflicts | 30 | 30 | 1.0000 | [1.0000, 1.0000] | 1.0000 | 0.0000 | 0.0000 | 1.0000 |
| primary | A@1 | C@1 | sops | 30 | 30 | 3.0030 | [2.9940, 3.0120] | 3.0017 | 0.0000 | 0.0064 | 0.9959 |
| primary | A@1 | C@1 | ops01 | 30 | 30 | 3.0030 | [2.9940, 3.0120] | 3.0017 | 0.0000 | 0.0064 | 0.9959 |
| primary | A@1 | C@1 | ops | 30 | 30 | 3.0030 | [2.9940, 3.0120] | 3.0017 | 0.0000 | 0.0064 | 0.9959 |
| replicate | A@3 | C@3 | conflicts | 30 | 30 | 3.0000 | [3.0000, 3.0000] | 3.0000 | 0.0000 | 0.0000 | 1.0000 |
| primary | B@1 | C@1 | sops | 30 | 30 | 1.2000 | [1.2000, 1.2000] | 1.2000 | 0.0000 | 0.0000 | 1.0000 |
| primary | B@1 | C@1 | ops01 | 30 | 30 | 1.2000 | [1.2000, 1.2000] | 1.2000 | 0.0000 | 0.0000 | 1.0000 |
| primary | B@1 | C@1 | ops | 30 | 30 | 1.2000 | [1.2000, 1.2000] | 1.2000 | 0.0000 | 0.0000 | 1.0000 |
| replicate | B@3 | C@3 | conflicts | 30 | 30 | 1.2000 | [1.2000, 1.2000] | 1.2000 | 0.0000 | 0.0000 | 1.0000 |
| aa | C@2 | C@1 | conflicts | 30 | 30 | 1.0010 | [0.9980, 1.0040] | 1.0006 | 0.0000 | 0.0064 | 0.9959 |
| control | C@1 | Cnobreak@1 | conflicts | 30 | 30 | 0.5000 | [0.5000, 0.5000] | 0.5000 | 1.0000 | 0.0000 | 1.0000 |
| control | B@1 | Bnogauge@1 | conflicts | 30 | 30 | 0.0588 | [0.0588, 0.0588] | 0.0588 | 1.0000 | 0.0000 | 1.0000 |
| control | Chalf@1 | C@1 | conflicts | 30 | 30 | 0.5000 | [0.5000, 0.5000] | 0.5000 | 1.0000 | 0.0000 | 1.0000 |

## O-TRIPWIRE headline (H1: literal per-instance orbit clauses are unsound)

0 of 0 cells with planted U rows show at least one planted SAT target flipped to UNSAT (the instrument sees the unsoundness); this is a statement about arm U only, not about A or B.

## overhead diagnostic (mean conflicts; excess = arm - C)

| cell | n | m | l | mean C | excess A | excess B |
|---|---|---|---|---|---|---|
| 17_8_2_0 | 17 | 2 | 8 | 1036.8333 | 2075.6227 | 207.3667 |
