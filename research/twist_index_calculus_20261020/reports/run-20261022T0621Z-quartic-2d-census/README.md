# Two-dimensional invariant-space census

Run ID `run-20261022T0621Z-quartic-2d-census`; 22 October 2026; Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw census](../../quartic_twist_2d_census_v1.ndjson), [independent native norm/point audit](../../quartic_twist_2d_native_audit_v1.ndjson), and [graph](space_census.svg). The producer and audit returned `pass` on all eight curves.

Commands: `target/release/examples/quartic_twist_2d_census` and `target/release/examples/quartic_twist_2d_audit research/twist_index_calculus_20261020/quartic_twist_2d_census_v1.ndjson`. Source SHA-256: producer `35ce636e09ae8c82f40fee61defd407cd0ab0edbc753f10bbe96ad4bd47ddb51`; audit `b079850f8624a338d0f8d2cede398781292b39b35ab070dd66e08a8a436c8e4d`. The test enumerated every nonzero `a+b t²` in `Fp+t²Fp`; this is a structural census, not a target relation stream.

| p | line 0 | line 2 | interior | full physical base | 24-fold columns | signed pair entries |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 13 | 0 | 0 | 72 | 72 | 6 | 5,112 |
| 37 | 36 | 12 | 576 | 624 | 52 | 388,752 |
| 73 | 48 | 48 | 2,448 | 2,544 | 212 | 6,469,392 |
| 97 | 36 | 36 | 4,464 | 4,536 | 378 | 20,570,760 |
| 157 | 108 | 84 | 12,384 | 12,576 | 1,048 | 158,143,200 |
| 229 | 96 | 96 | 26,424 | 26,616 | 2,218 | 708,384,840 |
| 241 | 96 | 96 | 28,512 | 28,704 | 2,392 | 823,890,912 |
| 337 | 180 | 180 | 56,016 | 56,376 | 4,698 | 3,178,197,000 |

The independent audit applied the `Fp²` norm character and checked signed point orbits on every abscissa orbit. All 265,008 candidate abscissae agreed and every signed orbit had size 24. The pair-entry count is `B(B−1)` for the current explicit oracle before collisions; it is a derived work size, not allocated memory. No target or rho was run in this census.

![Invariant-space base size and pair-entry count](space_census.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
