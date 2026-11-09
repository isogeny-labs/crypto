# Item 17 — original n=23 solver cells

The existing native runner executed its original n=23 solver package: five declared cells, sixteen requested targets per cell, seed 20261012, 150-second cell time limit, node budget 1024, and the inherited engine. It exited 0 after a reported 1069.9 seconds, including the existing census/yield legs.

Two cyclotomic w1 cells timed out before emitting target records. Both matched ONB cells emitted all sixteen target records, but resolved no solving degree and lifted no decompositions. K0's ONB cell recorded four Boolean roots without a verified lift; K1's recorded none. The cyclotomic w2 encoding reported sixteen unsupported targets. WDSat was unavailable to this run because neither source nor binary environment variable was set; a checkout obtained later is not retrospectively credited.

| Cell | Recorded targets | Outcome | Solving degree |
|---|---:|---|---|
| K0/n23/w1/m2/cyclotomic | 0 | 150-second timeout | unresolved |
| K0/n23/w1/m2/onb_matched | 16 | bounded solve finished, no lifted roots | unresolved; degree 3 reached |
| K1/n23/w1/m2/cyclotomic | 0 | 150-second timeout | unresolved |
| K1/n23/w1/m2/onb_matched | 16 | bounded solve finished, no lifted roots | unresolved; degree 3 reached |
| K0/n23/w2/m2/cyclotomic | 0 | encoding unsupported for all 16 targets | unmeasured |

The reported wall times come from a shared host with concurrent builds and trials. They are not a speed comparison. Every failure and raw output is retained. The initial `/usr/bin/time` wrapper attempt exited 127 before any scientific work; the actual run used the shell's timing facility.

This does not establish a solving-degree crossover or a negative mathematical result. Exact argv/source and binary hashes, the build-repair patch, raw JSON, generated census/yield/solver table and stderr are adjacent. The separate `oracle_integration/` package supplies the new n=83 multiword construction and membership controls; it does not supply a complete n=83 F4 solve.
