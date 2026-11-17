# Results: PKM Kummer tower, m = 3, N = 21 with msolve, 2026-10-18

Reading of PROTOCOL.md's decision rule (item 3's) over the records in
`runs/` (run report: `reports/run-20261018T0627Z/`, PDF `report.pdf`).
Class: **measurement, null result** (an instrument run that did not reach
its decision); not an improvement, not a cost row.

| system | N | outcome | reading |
|---|--:|---|---|
| `m3_t5_x1289903598` (control) | 15 | finished, max degree 7, one-element Gröbner basis | agrees with item 3's committed `D = 7`: the packaged msolve 0.6.5 is the same instrument |
| `m3_t7_x221026520` (target 0) | 21 | wall stop at 15,458 s inside the 6th degree-7 round, 15.3 GB peak | **inconclusive** (wall stop, no degree-8 step) |
| `m3_t7_x704078485` (target 1) | 21 | wall stop at 7,728 s inside the 4th degree-7 round, 8.9 GB peak | **inconclusive** |
| `m3_t8_x1227422598` (target 0) | 24 | wall stop at 8,328 s inside the 5th degree-7 round, 13.0 GB peak | **inconclusive** |

Nothing new on H0 versus H1a: neither "finished at D = 7" nor "a productive
degree-8 step" was observed. What is new is the cost of getting there: the
longest run spent 4.3 h and 15 GB and was still inside degree 7, with every
completed degree-7 round adding thousands of new basis elements (2,354,
4,269, 5,421, 5,043, 5,502) and matrices growing from 98k × 101k to
384k × 528k. At that growth the end of degree 7 at N = 21 is not within a
day of single-threaded msolve on this host; the instrument's wall, not its
memory, remains the limit at this scale (peak 15 GB under a 27 GB cap).

## Next improvement to try

Either the native `f4_fp_tower` cell (192 GB host, 14–60 h, already frozen
in `research/pkm_lead1_m3n21_20261016/PROTOCOL.md`) or msolve with
`-t 8` on a 64 GB host with a 24 h wall; the recorded per-round times give
the budget. Nothing in this run changes the N ≥ 27 requirement for H1a.

## Addendum: Amendment A1, minimal P2 (run 20261018T2208Z, `-t 16`, target 0 only)

Report: `reports/run-20261018T2208Z-mt/` (README, graph, PDF). Records:
`runs-mt/`, `progress-mt.txt`.

- Control `m3_t5` at 16 threads: identical answer, 245.9 s.
- `m3_t7_x221026520` (N = 21): same F4 trace as the first run through all of
  degree 7. Degree 7 ran out of pairs without the unit ideal. msolve then
  selected 398,409 degree-8 pairs and exited 139 inside that round at 99.6 GiB
  under the 108.8 GB cap, before the round reported `new`.
- A1.4 reading: **inconclusive** (memory stop, no productive degree-8 round,
  one target of two). Not H0 and not a plateau reading.
- Next: the same system and target 1 on a host where one process has 256 GB
  or more (A1.2 hosts, larger memory).
