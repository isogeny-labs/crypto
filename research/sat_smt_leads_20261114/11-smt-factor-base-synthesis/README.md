# Idea 11 pilot: solver-driven synthesis of the factor base

Pilot, not claim data. Toy sizes, one core, shared container, 2026-11-14. Every result
line below is an unverified pilot assertion and carries `[pcc:unverified]`.
Protocol written before the runs: `PROTOCOL.md`. Raw rows: `results.json`
(`rows`, 933 instances; `defect`), `raw_*.jsonl`, `defect.json`. Full tables: `summary_tables.md`.

## What was run

- `fb.py`: self-contained F_{2^n} arithmetic, curve y^2+xy=x^3+1, subspace tools, the
  (s,p)-linearised leaf solve and 2+1 scan (L21), CNF generators (chained m=3, direct m=2).
- `validate.py`: field/curve identities at n=7..23; at (l,n)=(3,11) brute-force triple
  enumeration, L21, SAT3 (CaDiCaL 1.5.3 and Glucose4) and SAT2 agree on 23 instances
  (8 decomposable); every SAT model and every L21 solution lifts to curve points summing
  to +-R (also asserted on all 933 measured instances; planted targets always SAT).
- `defect.py`: defect distributions (200 random V per cell, structured families, two
  hill-climbs, exhaustive enumeration at l=3, subfield positive control at n=15).
- `run_solve.py`: per cell 4 best-defect bases S (polynomial spans c<1,t,..,t^(l-1)>),
  4 random A, 4 random B (A/A control), interleaved S,A,B; 4 shared random targets +
  4 planted targets per base. Arms: L21 (leaves, (s,p) candidates), SAT3 and SAT2
  (CaDiCaL conflicts/decisions/propagations). Master seed 2026111411.
- `analyze.py`: geometric means, S/A ratio next to the A/B noise floor, slopes.

Baselines named: the linear 2+1 scan count (2|F| leaves, about 2^l) on the same bases, and
the untreated SAT arm on random V (arms A and B) with the identical encoding rule.

## Results

Defect (C1). w = dim V.V, lower = larger defect; lower bound for prime n is 2l-1.

| l | n | random w min / 5% / median (200) | polynomial span w | Frobenius span, trace-zero w | blind hill-climb end | product-guided hill-climb end | def3 random median (range) | def3 polynomial span |
|---|---|---|---|---|---|---|---|---|
| 3 | 11 | 5 / 6 / 6 | 5 | 6 | 5 (5/5 runs) | 5 (5/5) | 4 (4..6) | 4 |
| 4 | 13 | 9 / 9 / 10 | 7 | 9-10 | 7-8 | 7 (5/5) | 16 (16..18) | 20 |
| 5 | 17 | 14 / 14 / 15 | 9 | 14-15 | 11-13 | 9 (5/5) | 40 (40..41) | 57 |
| 6 | 19 | 17 / 18 / 19 | 11 | 18-19 | 17 | 11 (5/5) | 80 (80..81) | 123 |
| 7 | 23 | 22 / 23 / 23 | 13 | 23 | 22 | 13 (3/5), 18, 20 | 141 (140..147) | 226 |

- [pcc:unverified] The polynomial span sits at w = 2l-1 in every cell, below the random
  minimum for l >= 4 and below the random 5% quantile at l=3. Kill 1 is NOT met.
- [pcc:unverified] No search arm found w < 2l-1. Exhaustive: all 11811 three-dimensional V
  at n=7 have w in {5 (2667), 6 (9144)}; all 174251 V containing 1 at n=11 have w in
  {5 (2387), 6 (171864)}. The search rediscovers the known bound and the base the
  repository already uses (`linear21.rs`, V = span(1..z^(d-1))); it does not exceed it.
- [pcc:unverified] Frobenius-span and trace-zero subspaces are inside the random
  distribution on w, def2 and def3. def2 (D=2 literal defect) differs by at most 3 between
  any families; def3 separates the polynomial span (e.g. 226 vs 140..147 at l=7).
- [pcc:unverified] Positive control: subfields of F_{2^15} give w = l (3 and 5) against
  random medians 6 and 14, so the detector sees structure.
- Blind hill-climbing fails from l=5 up (flat landscape); only the product-guided move
  set reaches 2l-1.

Solve cost (C2), geometric mean per instance, non-decomposable targets (decomposable
rows are in `summary_tables.md`, same picture).

| arm / unit | l | n | S | A | B | S/A | A/B (noise floor) |
|---|---|---|---|---|---|---|---|
| L21 leaves | 5 | 17 | 29.2 | 30.7 | 35.4 | 0.952 | 0.868 |
| L21 leaves | 7 | 23 | 122 | 125 | 126 | 0.980 | 0.990 |
| L21 (s,p) candidates | 3 | 11 | 1.33 | 1.84 | 1.55 | 0.724 | 1.184 |
| L21 (s,p) candidates | 4 | 13 | 3.36 | 29.1 | 21.0 | 0.115 | 1.390 |
| L21 (s,p) candidates | 5 | 17 | 3.04 | 200 | 251 | 0.015 | 0.798 |
| L21 (s,p) candidates | 6 | 19 | 14.5 | 4060 | 4080 | 0.004 | 0.994 |
| L21 (s,p) candidates | 7 | 23 | 15.2 | 16000 | 16100 | 0.001 | 0.990 |
| SAT3 conflicts | 2 | 7 | 280 | 162 | 243 | 1.728 | 0.666 |
| SAT3 conflicts | 3 | 11 | 6330 | 7110 | 6950 | 0.890 | 1.023 |
| SAT3 conflicts | 4 | 13 | 59700 | 92500 | 56300 | 0.646 | 1.643 |
| SAT2 conflicts | 6 | 13 | 1280 | 1340 | 1520 | 0.960 | 0.880 |
| SAT2 conflicts | 7 | 17 | 4180 | 4290 | 4730 | 0.973 | 0.908 |
| SAT2 conflicts | 8 | 17 | 13900 | 13500 | 14100 | 1.025 | 0.962 |

- [pcc:unverified] L21 leaf count is base independent (S/A 0.84..1.14, inside A/B).
  L21 candidates handed to the split: S/A falls to 0.001 at l=7 with A/B at 0.99; fraction
  of leaves decided with <=1 candidate at l=7 is 0.986 (S) vs 0.000 (A, B). Fitted slopes
  of candidates over l=2..7, non-decomposable: S 0.79, A 2.95, B 3.04 bits per l (the
  random-V figure includes the transition from w < n to w = n; the asymptote predicted in
  the protocol is 2). This is the mechanism the repository's linear 2+1 arm already relies
  on; the S arm here IS that baseline, and nothing measured goes below it.
- [pcc:unverified] SAT3 (CaDiCaL conflicts, chained m=3 CNF, non-decomposable): S/A =
  1.73, 0.89, 0.65 at l=2,3,4 against A/B = 0.67, 1.02, 1.64; S per-base range 46000..89000
  lies inside the A∪B per-base range 32200..108000 at l=4. No difference beyond the A/A
  floor. Slopes over l=2..4: S 3.87, A 4.58, B 3.93 bits per l (decomposable: 4.44, 4.93, 4.26);
  the A-B gap (0.65) is as large as the S-A gap, so the slopes do not separate either.
- [pcc:unverified] SAT2 (direct m=2 CNF, non-decomposable): S/A in 0.88..1.03, A/B in
  0.88..1.18, slopes S 1.84, A 1.81, B 1.83 bits per l. No difference.
- Fraction of the shared random targets that were decomposable (S/A/B, of 16 each), m=3:
  l=2 6/6/1, l=3 1/1/0, l=4 2/0/0, l=5 1/0/1, l=6 1/0/0, l=7 0/0/0; all rows above are
  split by ground truth, so the comparison is not driven by this.

## Verdict against the pre-written kill criteria

- Kill 1 (best defect inside the random top 5%): not met. Structure exists, but the
  optimum found (w = 2l-1, polynomial span) is the base already in use and matches the
  known lower bound; exhaustive search at l=3 confirms nothing lower exists there.
- Kill 2 (no solve-cost difference beyond the A/A floor): met for both SAT arms. Only
  the linear 2+1 leaf separates, by the already-known mechanism (pre-registered outcome
  "known mechanism, nothing new to synthesise").
- Overall: kill criterion met for the lead as a SAT/SMT synthesis idea. Measured ratio for
  the record: random V costs the linear 2+1 leaf about 2^l extra candidates per leaf
  relative to the polynomial-span baseline; the polynomial span costs the same as it did
  yesterday.

## Deviations from PROTOCOL.md and what was not covered

- SAT3 at l=5 was not run (probe: 368k conflicts, 27 s for one SAT instance; 96 instances
  did not fit). A cell (l,n)=(2,7) was added instead so three sizes exist; it is degenerate
  (w = 3 for every V), so the SAT3 slope rests on l=3,4 for any S-vs-random contrast.
- Bases with fewer than 3 rational points are resampled (needed for planted targets at l=2).
- Decompositions through Q = P1+P2 equal to the 2-torsion point (t = 0) are excluded in
  every arm (clause t != 0, leaf skip): the first validation run showed that without it the
  CNF admits off-curve pairs with x1*x2 = sqrt(b).
- z3/CEGIS search over basis-change matrices: not run; replaced by exhaustive enumeration
  at l=3 and two hill-climbs. An F2-linear change of coordinate on x was not explored.
- One solver (CaDiCaL 1.5.3) for the measured SAT arms, one seed per instance, no XOR-aware
  solver, no symmetry breaking, 4 bases per arm, m=4 not touched, only a=0, b=1.
- The defect was defined on the m=2 leaf system and on V.V, not on a descended S4.
- No session-log entry was written (task rule: write only inside this directory).

## What a full study would need

Nothing here suggests one for SAT. If pursued: SAT3 at l=5,6 with an XOR-aware solver and
>= 16 bases per arm to shrink the A/A floor below the 0.65 ratio seen at l=4; and a defect
defined on the degree-3 Macaulay rows of the m=3 chained system, to test whether any V
with w = 2l-1 but different higher-degree rank exists (all polynomial spans tested had
identical def3).

Rerun smallest cell: `python3 validate.py` (about 40 s, includes two larger probes), or the full set
`python3 run_solve.py sat3 && python3 run_solve.py sat2 && python3 run_solve.py l21 && python3 defect.py && python3 analyze.py` (about 9 minutes).
