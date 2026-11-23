# Idea 10: bit-sliced multi-target propagation (pilot, 2026-11-14)

Pilot only: toy sizes, shared 4-core container, one core per run. Every number
below is an unverified pilot assertion, tagged [pcc:unverified]. The protocol
(`PROTOCOL.md`) was written before any measured run.

## Verdict against the pre-written kill criterion

**Kill criterion met, through K2.** [pcc:unverified]

- K1 (bit-sliced scan not below bit-sliced enumeration at the largest l): not
  met. At l = 10 the bit-sliced scan takes 0.084 (random targets) and 0.089
  (planted) of the wall time per target of bit-sliced enumeration.
  [pcc:unverified] By the reading rule fixed in the protocol this ratio is the
  2^l-versus-2^(2l) gap of yesterday's linear 2+1 scan and is not credited to
  bit-slicing.
- The ratio the protocol credits to bit-slicing, bit-sliced scan / scalar scan
  (same algorithm, wall time per target): 0.25 at l = 5 rising to 1.02 at
  l = 10 on random targets; 0.92 to 1.06 on planted targets. [pcc:unverified]
  The protocol asked for below 1/8 with no upward trend; measured is about 1
  with an upward trend.
- K2 (median common decision-prefix fraction below 10% in every cell): met.
  CaDiCaL 1.9.5 on the target-specialised encoding E1: the median common
  prefix is 0 decisions in all three cells (l = 5, 6, 7); 2 to 3 of 105-120
  pairs per cell share 10% or more. On the universal encoding E2 (target as
  assumptions) the median is 1 decision, at most 10, at most 2.6% of a trace.
  [pcc:unverified]

## What was run

Problem (all arms): F_{2^31} = F_2[x]/(x^31+x^3+1), curve y^2+xy = x^3+x^2+b,
V = span of l seeded random elements. Count ordered pairs (x1, x2) in V x V,
x1 != x2, with S3(x1, x2, X) = 0 for a target x-coordinate X. This is the
m = 2 leaf of the linear 2+1 scan for m = 3. All arms count to completion.

Part (a), `bs.rs` (standalone, `rustc -O -C target-cpu=native`, one thread),
l = 5..10, 256 planted + 256 random targets per cell, 3 interleaved
repetitions, minimum wall time reported:

| arm | what | counted unit |
|---|---|---|
| `enum_scalar` | every ordered pair, per target, clmul arithmetic | tuple evaluations |
| `enum_bitsliced` | 64 targets per u64, Gray code over x1 and x2, 16-plane filter then exact check | u64 word operations |
| `scan_scalar` | linear 2+1 leaf: per x1 solve the quadratic in x2 (half-trace table, batch inversion), test membership in V | leaf solves |
| `scan_bitsliced` (treatment) | 64 targets per u64, (l+16) x (l+1) affine system in bit planes, Gray code over x1, constant-time bit-sliced Gauss, survivors checked in scalar | u64 word operations |
| `join` | pairs enumerated once, solved for X, hash lookup in all 512 targets | quadratic solves / target |

Part (b), `part_b.py` (python-sat): encodings E1 (coefficients of the 31 XOR
rows specialised to X) and E2 (X bits as variables set by assumptions, one
clause set for all targets); 8 planted + 8 random targets per cell;
CaDiCaL 1.9.5 with an observing IPASIR-UP propagator for decision traces;
CaDiCaL 1.5.3, Glucose 4, MiniSat 2.2 untreated for counts.

Validation (`validation.json`, and assertions inside both programs):

- All five compiled arms return identical solution counts on all 3072 targets
  (asserted; `"arms_agree": true` rows in `raw_part_a.jsonl`). Planted targets:
  256/256 decomposable in every cell; random targets: 0/256.
- Independent Python field code, brute force over all pairs at l = 5 and 6:
  32/32 targets match the compiled counts.
- Point level: 16 targets built as R = P1 + P2 from real curve points with
  x(Pi) in V are SAT under E1 and E2, and 16/16 E1 models lift to curve
  points summing to +-R.
- Every SAT/UNSAT answer in part (b) is asserted equal to the compiled
  reference count, for every solver and both encodings.

## Part (a) results [pcc:unverified]

Wall nanoseconds per target (minimum of 3) and counted units per target.

Random (non-decomposable) targets:

| l | enum_scalar ns | tuple evals | enum_bitsliced ns | word ops | scan_scalar ns | leaf solves | scan_bitsliced ns | word ops | scan_bs / enum_bs | scan_bs / scan_scalar |
|---|---|---|---|---|---|---|---|---|---|---|
| 5 | 5374 | 992 | 475 | 528 | 2179 | 32 | 550 | 700 | 1.158 | 0.253 |
| 6 | 20580 | 4032 | 1737 | 1758 | 3133 | 64 | 1232 | 1721 | 0.709 | 0.393 |
| 7 | 82871 | 16256 | 6324 | 6420 | 5012 | 128 | 2754 | 4232 | 0.435 | 0.549 |
| 8 | 331454 | 65280 | 24287 | 24685 | 8946 | 256 | 6260 | 10406 | 0.258 | 0.700 |
| 9 | 1354972 | 261632 | 95764 | 97098 | 16803 | 512 | 14354 | 25495 | 0.150 | 0.854 |
| 10 | 5384367 | 1047552 | 381836 | 385398 | 31549 | 1024 | 32014 | 61829 | 0.084 | 1.015 |

Planted (decomposable) targets:

| l | enum_scalar ns | enum_bitsliced ns | scan_scalar ns | scan_bitsliced ns | scan_bs / enum_bs | scan_bs / scan_scalar |
|---|---|---|---|---|---|---|
| 5 | 5086 | 476 | 2191 | 2023 | 4.246 | 0.923 |
| 6 | 20703 | 1761 | 3139 | 2641 | 1.500 | 0.841 |
| 7 | 82555 | 6375 | 5036 | 4207 | 0.660 | 0.835 |
| 8 | 335887 | 24715 | 8939 | 7655 | 0.310 | 0.856 |
| 9 | 1334359 | 95249 | 16501 | 16777 | 0.176 | 1.017 |
| 10 | 5392507 | 376373 | 31598 | 33458 | 0.089 | 1.059 |

Fitted bits per l (least squares, l = 5..10; random class; planted in
`results.json`):

| arm | counted units | wall ns | wall ns, top 3 sizes |
|---|---|---|---|
| enum_scalar | 2.01 | 2.00 | 2.01 |
| enum_bitsliced | 1.91 | 1.93 | 1.99 |
| scan_scalar | 1.00 | 0.78 | 0.91 |
| scan_bitsliced | 1.29 | 1.17 | 1.18 |

Readings [pcc:unverified]:

- Bit-slicing the enumeration: `enum_bitsliced` / `enum_scalar` wall ratio is
  0.07 to 0.09 at every l (about 0.37 word operations per target-tuple). A
  constant, as expected, and the enumeration slope stays at 2.
- Bit-slicing the scan: the per-target word-operation count grows as
  2^l * (l+16)(l+1)-ish (fitted 1.29 bits per l), against 1.00 for scalar leaf
  solves. The measured wall ratio to the scalar scan drifts from 0.25 to 1.02
  over l = 5..10. The scalar scan's small-l times carry a fixed per-target
  setup (two field inversions and the x1 = 0 slow path), which is why its
  wall slope is below 1 and why the ratio looks favourable at l = 5; the
  top-3-size wall slopes are 0.91 (scalar) and 1.18 (bit-sliced).
- Planted targets at small l cost the bit-sliced scan more (2023 ns vs 550 ns
  at l = 5) because each true solution falls back to a scalar leaf solve; the
  word-operation counts are equal across classes to within 1%.
- `join` with T = 512 targets costs 2^(2l-1)/512 quadratic solves per target
  (1023 at l = 10, 586 microseconds per target with the slow generic solver
  used there). Its per-target cost is 2^(2l-1)/T, so it goes below the scan
  only for T above 2^(l-1) times the solve-cost ratio; at T = 512 it is above
  both scan arms from l = 7 on. Context only; its quadratic solver was not
  optimised.

## Part (b) results [pcc:unverified]

Budget note: the 240 s budget ended after target 15 of 16 at l = 7. l = 8 was
not run, and (b3) has no l = 7 row. The Python propagator callbacks dominate
the run time.

Untreated solvers, median conflicts per target:

| l | class | targets | E1 CaDiCaL 1.5.3 | E1 Glucose 4 | E1 MiniSat 2.2 | E2 CaDiCaL fresh | E2 Glucose fresh | traced decisions (E1, CaDiCaL 1.9.5) | target-independent share of implied literals (E1) |
|---|---|---|---|---|---|---|---|---|---|
| 5 | planted | 8 | 93.5 | 97.5 | 85.0 | 541.5 | 664.5 | 266.5 | 0.412 |
| 5 | random | 8 | 241.5 | 240.0 | 210.5 | 1140.0 | 1437.0 | 318.5 | 0.403 |
| 6 | planted | 8 | 305.0 | 240.5 | 330.5 | 1743.0 | 1211.5 | 498.5 | 0.441 |
| 6 | random | 8 | 1010.5 | 1027.5 | 896.5 | 4363.5 | 6088.0 | 860.0 | 0.497 |
| 7 | planted | 8 | 1347.5 | 1036.0 | 876.5 | 3963.0 | 7658.0 | 1241.0 | 0.432 |
| 7 | random | 7 | 3608 | 4775 | 3558 | 22219 | 30590 | 2265 | 0.601 |

Fitted conflicts slope on random targets, l = 5..7: E1 CaDiCaL 1.95 bits per
l, E1 MiniSat 2.04, E2 CaDiCaL 2.14. The scan baseline on the same problem is
2^l leaf solves (1.00 bits per l).

(b1) common decision prefix between two targets, same solver, same seed:

| l | encoding | pairs | median prefix (decisions) | max prefix | median fraction of trace | max fraction | pairs at 10% or more |
|---|---|---|---|---|---|---|---|
| 5 | E1 | 120 | 0 | 233 | 0.0000 | 0.806 | 2 |
| 5 | E2 | 120 | 1 | 10 | 0.0016 | 0.026 | 0 |
| 6 | E1 | 120 | 0 | 326 | 0.0000 | 0.797 | 3 |
| 6 | E2 | 120 | 1 | 8 | 0.0006 | 0.007 | 0 |
| 7 | E1 | 105 | 0 | 433 | 0.0000 | 0.396 | 3 |
| 7 | E2 | 105 | 1 | 7 | 0.0002 | 0.003 | 0 |

Projected on the summand-bit variables only, the median fraction is 0 in all
six cells. The few E1 pairs with a long common prefix (2-3 per cell, spread
over all class combinations) were not investigated.

(b2) At up to 200 sampled decision stacks per run, 40% to 60% of the literals
implied by unit propagation on the original E1 clauses are already implied by
the target-independent AND gates alone. This counts implied literals on the
original clauses, not propagation work, and ignores learnt clauses (which
under E1 are target-specific). It was predicted below 30%; measured higher.
It does not translate into a shared trace, see (b1).

(b3) E2, one solver kept across the 16 targets (learnt clauses are valid for
every target) against a fresh solver per target, total conflicts
incremental / fresh:

| l | solver | planted | random |
|---|---|---|---|
| 5 | CaDiCaL 1.5.3 | 0.901 | 0.869 |
| 5 | Glucose 4 | 0.562 | 0.914 |
| 6 | CaDiCaL 1.5.3 | 0.730 | 0.936 |
| 6 | Glucose 4 | 1.786 | 0.907 |

On random (UNSAT) targets the ratio is 0.87 to 0.94 in all four cells; planted
(SAT) targets scatter between 0.56 and 1.79. E2 itself costs about 4 to 6 times
the conflicts of E1 on the same target, so the reused universal formula stays
above the specialised one.

## Predictions versus outcome

| prediction | outcome |
|---|---|
| P1: scan_bs slope about 1, enum_bs about 2, crossing in l = 6..9 | units 1.29 vs 1.91; wall crossing between l = 5 and 6 (random), 6 and 7 (planted). Crossing earlier than predicted. |
| P2: scan_bs / scan_scalar constant in 0.3..3, flat in l | 0.25 to 1.06, not flat: rises with l |
| P3: prefix below 10%; shared implied literals below 30% | prefix: yes (median 0); shared implied literals: no, 40% to 60% |
| P4: incremental reuse changes conflicts by less than 2x | yes (0.56 to 1.79) |

## Deviations from the protocol

- l = 4 dropped from part (a): V x V has at most 240 planted X values at
  l = 4, so 256 distinct planted targets do not exist (the first launch hung
  in the generator and was killed; no timing data were produced by it). Part
  (a) covers l = 5..10, part (b) was planned for l = 5..8.
- Part (b) stopped by its time budget at l = 7 (15 of 16 targets); l = 8 not
  run; no (b3) row at l = 7.
- `enum_bitsliced` and both scan arms visit ordered pairs / all x1; no arm
  exploits the x1 <-> x2 symmetry (same factor 2 for all).
- CaDiCaL 1.5.3 reports 0 conflicts and 0 decisions on one planted l = 7
  target (index 4); the answer was SAT and matched the reference. Kept as is.

## What a full study would need

- n at the real ratio to l (n about 3l for the m = 3 leaf) and n > 64: the
  lane cost of the target-dependent linear maps grows with n while scalar
  clmul does not, so n = 31 is the favourable case for bit-slicing.
- 256/512-lane words (AVX2/AVX-512) and a tuned lazy Gauss (the l+16 rows
  here are a fixed false-positive margin, not tuned), against a scalar scan
  that is also tuned; pinned cores and an A/A arm for the timing noise floor.
- m = 3 end to end (outer scan producing the inner targets) and m = 4 / S4.
- For (b): more than 8+8 targets per cell, l >= 8, a native tracer instead of
  Python callbacks, several seeds, and an explanation of the rare long-prefix
  E1 pairs.

## Files

- `PROTOCOL.md`: pre-registered protocol.
- `bs.rs`: part (a). `part_b.py`: validation and part (b). `analyze.py`:
  builds `results.json` and prints the tables.
- `raw_part_a.jsonl`, `raw_part_b.json`, `part_b.log`, `validation.json`,
  `instances/instance_l{5..10}.json` (basis, targets, reference counts).
- `results.json`: summary plus all raw rows.
- `SESSION_LOG_ENTRY.md`: session-log entry kept here because this task may
  write only inside this directory.

Rerun:

```
S=/tmp/idea10 && mkdir -p $S instances && rustc -O -C target-cpu=native bs.rs -o $S/bs
$S/bs 5 10 instances > raw_part_a.jsonl
python3 part_b.py validate && python3 part_b.py run 5 8 240 && python3 analyze.py
```
