# Pilot 03: cut-variable branching with bounded nogood memory (m=4)

Pilot, toy sizes, one core per run, shared 4-core container. Not claim data. Every number below is an
unverified pilot assertion [pcc:unverified]. Protocol (written first, two dated amendments): `PROTOCOL.md`.
Raw rows: `results.json` (`rows`, `calibration`, `cell_summary`, `slopes`), per-process logs in `raw_logs/`.

## What was run
- Chained m=4 encoding `S3(x1,x2,u)=0, S3(x3,x4,v)=0, S3(u,v,x(R))=0` over F_{2^n}, curve y^2+xy=x^3+x^2+1,
  V = span{1..z^(l-1)}, plain CNF (AND gates + 4-variable XOR chunks), no symmetry breaking. Generator: `pdp.py`.
- Solver: CaDiCaL 1.9.5 via python-sat. Arms (`arms.py`): D default; Dp default with an idle IPASIR-UP
  propagator attached (control); C cut-first (`decide()` returns the next unassigned bit of u, phase false);
  C-lean (C + `reduceint=10, reducetarget=100, tier glues 1/1`); C-keep (C + `reduce=0`).
- Same CNF and same targets for all arms, arm order rotated per target. Conflict cap instead of wall cap.
- Validation (`validate.py`): n=7, l=2, all 128 targets: exhaustive scan = algebraic MITM oracle = arm D = arm C
  (24 decomposable). Every finished run in the cells agrees with the oracle; every SAT model re-checked in field
  arithmetic; all 20 SAT models of the l=3 decomposable cell lifted to rational points with signs.
- Primary series n=11, l=1,2,3 (4 non-decomposable targets per cell, 5 arms; 4 decomposable targets at l=3 only).
  Secondary n=13, l=2,3,4 (2 non-decomposable targets, arms D and C, cap 600000 conflicts).
  92 rows in total. Deviations from the first protocol (n=19, l up to 5, 8 targets, planted targets, hard
  learnt cap) are in the amendments; the reason for each is a measured fact recorded under `calibration`.

## Results: median conflicts, non-decomposable targets [pcc:unverified]
Reference counts (not solver runs): enumeration 2^(3l)/4! = 0.33, 2.7, 21, 171; MITM / linear-leaf 2^(2l) = 4, 16, 64, 256
for l = 1,2,3,4. Oracle work actually done: |U| (distinct reachable u) = 2, 12, 44 at n=11.

| n | l | D | Dp | C | C-lean | C-keep | C/D | C/Dp | C cut decisions | 2^(2l) |
|---|---|---|----|---|--------|--------|-----|------|-----------------|--------|
| 11 | 1 | 375 | 278 | 233 | 299 | 233 | 0.62 | 0.84 | 144 | 4 |
| 11 | 2 | 3180 | 4206 | 6633 | 16390 | 9214 | 2.09 | 1.58 | 1895 | 16 |
| 11 | 3 | 108287 | 104745 | 101937 | 129663 | 99588 | 0.94 | 0.97 | 7088 | 64 |
| 13 | 2 | 3066 | - | 19586 | - | - | 6.39 | - | 6903 | 16 |
| 13 | 3 | 216144 | - | 164387 | - | - | 0.76 | - | 14149 | 64 |
| 13 | 4 | >600000 (2/2 capped) | - | >600000 (2/2 capped) | - | - | n/a | - | 46235 | 256 |

Fitted bits per l (least squares on log2 median conflicts), n=11, l=1..3, non-decomposable [pcc:unverified]:
D 4.08, Dp 4.28, C 4.39, C-lean 4.38, C-keep 4.37.
Last step only (l=2 -> 3): D 5.09, Dp 4.64, C 3.94, C-lean 2.98, C-keep 3.43.
n=13, two points only (l=2,3; l=4 censored in both arms): D 6.14, C 3.07.

Decomposable targets, n=11, l=3 (4 targets), median conflicts [pcc:unverified]:
D 40509, Dp 82167, C 29183, C-lean 69794, C-keep 26597 (ratio C/D 0.72, C/Dp 0.36; range per arm spans 2x to 4x).
No decomposable rows exist for l=1,2 (see "Not covered").

## Verdict against the pre-written kill criterion
KILL criterion met on the primary series [pcc:unverified]: slope(C) = 4.39 bits/l is not below
slope(D) - 0.3 = 3.78 (nor below the control Dp, 4.28). Cut-first does not move toward 2 bits per l.

What the rows show beyond the slope number:
- At l=3 the cut-first arm and the default arm are at the same level (ratio 0.94 at n=11, 0.76 at n=13 with
  2 targets); at l=2 cut-first costs 2.1x (n=11) and 6.4x (n=13) the default arm's conflicts. The lower
  "last step" figures for C come from C being worse at l=2, not from being lower at l=3. With n=13, l=4
  censored in both arms, there is no cell where C is measured clearly below D on non-decomposable targets.
- The cut is n bits, not 2l bits. C makes 7088 decisions on u bits at n=11, l=3 where only 44 values of u
  are reachable and 2^11 = 2048 values of u exist at all: learnt clauses on the cut do not act as a table
  of pair sums. Both SAT arms sit at about 2^16.7 conflicts where the MITM/linear-leaf reference is 2^6.
- The cost of this chained CNF grows with n at fixed l in the default arm (l=3: 108287 at n=11, 216144 at n=13),
  and the first calibration at n=19 did not finish at all: once (x1,x2) are fixed, u is given by a rank n-1
  F2-linear system that unit propagation over chopped XORs does not solve.
- Memory dial at fixed cut-first branching (n=11): C-lean / C = 1.28, 2.47, 1.27 and C-keep / C = 1.00, 1.39, 0.98
  for l = 1,2,3. Shrinking the learnt database costs conflicts; never deleting gives nothing measurable.
  This is CaDiCaL's reduction schedule, not a hard clause cap, and peak database size was not observable.
- Attaching the idle propagator alone (Dp vs D) moves conflicts by 0.74x to 1.32x on non-decomposable cells and 2.0x on
  the decomposable cell, so differences of that size between C and D are inside instrument noise.

## Not covered / limits
- Only 3 sizes (l=1,2,3) at one n, 4 targets per cell (2 at n=13); l=1 is degenerate-small. No confidence intervals.
- l >= 4 non-decomposable: at n=11 no such targets exist in the relaxed system; at n=13 both arms hit the cap.
- Decomposable targets only at n=11, l=3 (no rational x in V at l=2; the l=2 relaxed-decomposable generator found
  no target in about 3 minutes and was stopped, no rows).
- Hard learnt-clause caps, the "DPLL on cut bits with a bounded cut-nogood store" variant, native XOR/Gauss
  solvers (CryptoMiniSat) under cut-first, branching on v as well as u, and a 2l-bit cut (branching on (s,p) or
  on an index of the pair) were not run.
- Wall times are secondary and were taken with up to 4 processes on a shared machine.

## What a full study would need
A cut representation whose size is 2l bits rather than n (the F2-affine (s,p) form of binary S3 gives one), a
solver with real decision priorities and a hard learnt cap (patched MiniSat/CaDiCaL), Gauss-capable propagation so
u follows from (x1,x2), n tied to 4l with l up to at least 6, and at least 8 targets per cell with A/A repeats.

## Rerun
`python3 validate.py` (about 1 minute); smallest cell: `python3 run.py 11 1 4 0 200000 D,Dp,C,C-lean,C-keep /tmp/out.json`;
tables: `python3 analyze.py <dir with the *.log row files>` (e.g. `raw_logs`), which rewrites `results.json`.
