# Idea 7 pilot: one homogeneous AllSAT instance for factor-base-only relations

Pilot only (toy sizes, one core per run, Glucose4 via pysat). Every number below is an
unverified pilot assertion. Protocol was written first: `PROTOCOL.md`.

## What was run
- `hallsat.py`: self-contained F_{2^n} arithmetic, curve y^2+xy=x^3+1, CNF generator
  (AND gates + XORs cut to length 4), S3 direct and S4 as chain S3(x1,x2,t) & S3(x3,x4,t),
  lex symmetry breaking x1<=x2<=... in every arm.
- Arms per base, in this order on the same base V:
  - **H0** homogeneous AllSAT, S_k(x1..xk)=0, all xi in V, blocking clause on the x bits per model, run to UNSAT.
  - **H1** = H0 + explicit y bits and curve equation per xi (removes quadratic-twist models).
  - **PT** baseline: per-target pipeline, fresh solver per target, S_k(x1..x_{k-1}, x(R))=0, 16 random curve targets per base, stop at first model.
- Family A: k=3 (per-target m=2), n=2l-1, l=4,5,6 (8 bases each), l=7 (4 bases).
- Family B: k=4 (per-target m=3), n=3l-2, l=3,4 (8 bases each).
- Validation (all passed, asserted in code on every measured cell): homogeneous curve-relation set
  equals an independent point-arithmetic MITM ground truth; every model lifted to curve points with a
  sign choice summing to O (or to +-R); per-target SAT/UNSAT equals ground-truth decomposability on all
  704 targets; planted targets SAT with correct lift (60 planted); on the validate cells the full S3
  model set (curve + twist + 2-torsion-mixed) equals brute-force evaluation of S3 over all sorted triples.

## Results (conflicts per verified nontrivial curve relation)

Q0 = (PT conflicts per relation) / (H0 conflicts per relation); Q > 1 would favour the homogeneous arm.
Brackets: 95% bootstrap over bases of log2 Q0.

| k | l | n | bases | curve rel (H) | twist models | H0 confl/rel | H1 confl/rel | PT sat/targets | PT confl SAT | PT confl UNSAT | PT confl/rel | Q0 [log2 CI] | Q1 | enum tuples/rel PT | enum tuples/rel H | linear-leaf solves/rel PT |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | 4 | 7 | 8 | 25 | 21 | 416 | 418 | 80/128 | 44.2 | 82.5 | 93.7 | 0.225 [-3.05, -1.32] | 0.224 | 218 | 261 | 1.6 |
| 3 | 5 | 9 | 8 | 33 | 50 | 2.56e3 | 2.39e3 | 74/128 | 142 | 291 | 354 | 0.138 [-3.61, -2.28] | 0.148 | 913 | 1.45e3 | 1.73 |
| 3 | 6 | 11 | 8 | 101 | 72 | 5.87e3 | 7.19e3 | 82/128 | 558 | 1.09e3 | 1.17e3 | 0.200 [-2.74, -1.94] | 0.163 | 3.25e3 | 3.62e3 | 1.56 |
| 3 | 7 | 13 | 4 | 81 | 89 | 2.55e4 | 4.22e4 | 39/64 | 1.97e3 | 4.02e3 | 4.54e3 | 0.178 [-2.87, -2.07] | 0.108 | 1.35e4 | 1.77e4 | 1.64 |
| 4 | 3 | 7 | 8 | 7 | 31 | 4.12e3 | 3.27e3 | 66/128 | 432 | 855 | 1.23e3 | 0.30 [-3.86, -0.30] | 0.377 | 233 | 377 | 15.5 |
| 4 | 4 | 10 | 8 | 20 | 62 | 6.20e4 | 4.71e4 | 62/128 | 6.95e3 | 1.50e4 | 2.30e4 | 0.37 [-2.42, -0.63] | 0.488 | 1.68e3 | 1.55e3 | 33 |

Result lines:
- [pcc:unverified] Family A (k=3, l=4..7): Q0 = 0.23, 0.14, 0.20, 0.18; the homogeneous AllSAT arm spends 4.4x to 7.2x MORE Glucose4 conflicts per curve relation than the per-target SAT baseline on the same bases. Fitted slope of log2 Q0: -0.05 bits per l (decisions -0.07, propagations -0.03).
- [pcc:unverified] Family B (k=4, l=3,4): Q0 = 0.30, 0.37 (7 and 20 relations only; bootstrap intervals overlap; two-point slope +0.30 bits/l is not distinguishable from 0).
- [pcc:unverified] Fitted bits per l of conflicts per relation, family A: H0 1.90, H1 2.16, PT 1.85, plain enumeration tuples per relation 1.97 (PT) / 1.96 (H). Both SAT arms track "constant x 2^n tuples per relation"; neither gains an exponent.
- [pcc:unverified] H0 conflicts per whole instance grow at 2.87 bits per l (k=3) and 5.4 bits per l (k=4, two points): with this CNF encoding the homogeneous instance costs about one conflict per full k-tuple (or more), i.e. the last variable is not "determined for free". PT per-target conflicts grow at 1.84-1.87 bits per l for m=2 and about 4.1 for m=3 (two points), consistent with the repository's earlier native-SAT slope (3.95 at m=3).
- [pcc:unverified] H1 (curve-membership constraint) does not recover the factor ~2 lost to twist models: Q1 = 0.22, 0.15, 0.16, 0.11 in family A (worse at l=6,7), 0.38, 0.49 in family B.
- [pcc:unverified] Linear 2+1 / MITM counting baseline: 1.6-1.7 leaf scans per relation at m=2 (flat in l) and 15.5, 33 leaf solves per relation at m=3, versus 94..4540 (m=2) and 1230, 23000 (m=3) conflicts for PT and 4-7x that for H0. Both SAT arms are far from that baseline, and the distance grows by about 1.85 bits per l (m=2).
- [pcc:unverified] Learnt-clause reuse across relations is not visible: conflicts per relation in H0 are above plain-enumeration tuples per relation at every cell (ratio 1.4..1.8 in family A, 11..40 in family B).

## Verdict against the pre-written kill criterion
Kill criterion met: Q < 1 at the largest two l in family A (and at both sizes of family B), and the
family A slope of log2 Q is -0.05 bits/l, i.e. flat. The prediction (Q > 1 and growing) is contradicted
in the tested range. Family B alone is too small (2 sizes, 27 relations) to say anything about a slope.

## Deviations from PROTOCOL.md and caveats
- Sizes: the homogeneous instance cost about 2^{3l} (k=3) and worse than 2^{4l} (k=4) conflicts, so
  family A stopped at l=7 (protocol said 4..10) with 4 bases at l=7 (protocol said 8), and family B
  stopped at l=4 (protocol said 3..6). l=8 (k=3) and l=5 (k=4) were estimated above the 5-minute cap and not run.
- Trivial diagonal: the chain S4 encoding admits (a,a,a,a) (P+P-P-P) for every a; these N models per
  base were enumerated by H0/H1 (their cost is included) and are excluded from the relation count
  post hoc in `analyze.py` (recomputed from the seed). This was found after the run, not planned.
  It costs H at most N extra models; it does not explain a factor 3.
- Models with x=0 (2-torsion point mixed with two twist points) appear in H0 ("other"); counted as waste.
- The per-target arm is a fresh solver per target; an incremental per-target arm (target bits under
  assumptions, clauses reused) was not run, so "homogeneous" and "clause reuse" are not separated.
- Only Glucose4 with XORs cut into CNF. No XOR-aware solver (CryptoMiniSat Gauss, WDSat): the
  Python binding exposes no conflict counter. The WDSat-style arm (2^{(m-1)l}) might behave differently.
- n was tied to l (n=2l-1, n=3l-2) so that both pipelines have O(1) decompositions per target;
  the ratio at other n/l was not explored.
- Homogeneous relations carry no target; a full DLP still needs at least one target relation. Not costed.
- Small relation counts (7..101 per cell): intervals are wide; no multiple-testing correction.

## What a full study would need
An XOR-aware solver with counters (CryptoMiniSat/WDSat built from source) on the same instances; the
incremental per-target control; a homogeneous encoding that keeps the (s,p)-linear leaf (fix k-2
variables, last two by linear algebra) so the comparison is against 2^{(k-2)l}; l up to 9-10 for k=3 and
l up to 6 for k=4 with at least 8 bases; replay certificate and matched rho per the validation gate.

## Files
`PROTOCOL.md`, `hallsat.py` (generator, arms, validation), `analyze.py`, `raw_*.jsonl` (per-instance rows),
`results.json` (summary + slopes + all rows), `log_*.txt`.

Rerun smallest cell: `python3 hallsat.py run 3 /tmp/x.jsonl 1 4:7 16` (under 1 s); validation: `python3 hallsat.py validate` (about 1 min).
