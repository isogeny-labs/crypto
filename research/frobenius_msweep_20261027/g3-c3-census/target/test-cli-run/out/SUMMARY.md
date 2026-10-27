# Cell G3 census summary (generated; see PROTOCOL.md for the frozen rule)

## cell 7_4_2_0

label: **O-IMPEDIMENT[too few paired UNSAT targets: A 3 B 3]**; tripwire: arm U flipped 0/1 planted SAT targets to UNSAT (NO planted flip: tripwire blind in this cell)

pricing band (AGENTS.md 5a): A dearer than C at w=0,0.1,1 (37.012,36.940,36.305); B dearer than C at w=0,0.1,1 (52.899,52.807,51.994)


| role | numerator | denominator | metric | targets | pairs | median ratio | 95% CI (cluster bootstrap) | geo-mean | frac<0.95 | sd(log) | MDE |
|---|---|---|---|---|---|---|---|---|---|---|---|
| primary | A@1 | C@1 | conflicts | 3 | 3 | 8.5000 | [7.4762, 9.6429] | 8.4938 | 0.0000 | 0.1272 | 0.7727 |
| primary | B@1 | C@1 | conflicts | 3 | 3 | 8.8889 | [7.9524, 10.1429] | 8.9502 | 0.0000 | 0.1218 | 0.7813 |
| primary | A@1 | C@1 | sops | 3 | 3 | 37.0123 | [34.0046, 39.8765] | 36.8864 | 0.0000 | 0.0797 | 0.8509 |
| primary | A@1 | C@1 | ops01 | 3 | 3 | 36.9399 | [33.9558, 39.8104] | 36.8244 | 0.0000 | 0.0796 | 0.8511 |
| primary | A@1 | C@1 | ops | 3 | 3 | 36.3047 | [33.5246, 39.2276] | 36.2777 | 0.0000 | 0.0786 | 0.8529 |
| primary | B@1 | C@1 | sops | 3 | 3 | 52.8995 | [48.4392, 54.9407] | 52.0213 | 0.0000 | 0.0646 | 0.8773 |
| primary | B@1 | C@1 | ops01 | 3 | 3 | 52.8072 | [48.3642, 54.8237] | 51.9272 | 0.0000 | 0.0643 | 0.8778 |
| primary | B@1 | C@1 | ops | 3 | 3 | 51.9940 | [47.7012, 53.7975] | 51.0993 | 0.0000 | 0.0620 | 0.8820 |
| aa | C@2 | C@1 | conflicts | 3 | 3 | 0.7857 | [0.7222, 1.1429] | 0.8656 | 0.6667 | 0.2443 | 0.6096 |
| control | C@1 | Cnobreak@1 | conflicts | 0 | 0 | null | [null, null] | null | null | null | null |
| control | B@1 | Bnogauge@1 | conflicts | 0 | 0 | null | [null, null] | null | null | null | null |
| control | Chalf@1 | C@1 | conflicts | 0 | 0 | null | [null, null] | null | null | null | null |

## O-TRIPWIRE headline (H1: literal per-instance orbit clauses are unsound)

0 of 1 cells with planted U rows show at least one planted SAT target flipped to UNSAT (the instrument sees the unsoundness); this is a statement about arm U only, not about A or B.

## overhead diagnostic (mean conflicts; excess = arm - C)

| cell | n | m | l | mean C | excess A | excess B |
|---|---|---|---|---|---|---|
| 7_4_2_0 | 7 | 2 | 4 | 17.6667 | 130.6667 | 138.6667 |
