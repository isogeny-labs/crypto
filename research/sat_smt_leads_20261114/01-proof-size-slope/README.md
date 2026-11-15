# Idea 1 pilot: proof-size slope as a go/no-go gate (m = 3, chained S4)

Pilot, toy sizes, one 4-core container shared with 11 other agents. Every
number below is an unverified pilot assertion, tagged [pcc:unverified]. No
benchmark record, leaderboard row or pcc claim was made. Protocol written
before the measured runs: `PROTOCOL.md` (one amendment, dated, before any
measurement: own trimmer instead of drat-trim).

## What was run

- Instances: F_{2^n} polynomial basis, curve y^2+xy = x^3+x^2+b, V = low l
  bits, PDP relaxation "exists x1,x2,x3 in V and X with S3(x1,x2,X) =
  S3(x3,x(R),X) = 0" (twist values of x_i allowed). CNF by `pdp.py`
  (AND gates + XOR rows cut to width 4; no symmetry breaking).
- Ground truth for every instance by brute force (pair table + scan of x3);
  the solver answer agreed on all 90 solver rows (99 in results.json counting the 9 rows of the (17,5) cell reused for series B).
- Baseline arm B1: CaDiCaL 1.5.3 (pysat `cd15`) default, DRAT logging;
  second proof source B1': Glucose 4 (`g4`), l <= 4 only.
- Treatment T1: backward core-marking trim of the same proof with own
  checker `rup_trim.c` (RUP only, deletions ignored, core-first). T2: trim
  iterated on its own output until no shrink. T3 (exploratory, l = 2, 3):
  re-solve only the core original clauses (with and without the core lemmas
  seeded) with cd15, g4, g3, lingeling, trim, keep the smallest, repeat.
- Count baselines B0 (not solved, reference slopes): tuples 2^(3l)/6
  (slope 3), MITM pairs 2^(2l) (slope 2), linear 2+1 scan 2^l leaf solves
  (slope 1; the baseline named in the task card for m = 3).
- Series A (primary): n = 17 fixed, l = 2, 3, 4, 5. Series B (secondary,
  cd15 only): (l, n) = (3, 11), (4, 13), (5, 17) [the (5,17) cell is the
  same data as series A].
- Seeds: cell seed = 20261114*1000003 + 1000n + 10l; targets drawn in order
  from that stream; each row stores seed, target index, b, field polynomial,
  x(R). Bootstrap seed 1.

Validation (`validate.py`, `validation.json`, all three passed before the
runs whose rows are kept, see "Deviations"): model set equals brute-force
solution set on 24 small instances (9 satisfiable); 18 planted targets are
SAT with the planted x fixed and lift to rational points summing to R;
trimmer: every tested proof VERIFIED, core originals UNSAT under an
independent solver, core lemmas re-checked by a pure-Python forward RUP
checker at l = 2, empty-clause-only proofs and a bogus proof of a
satisfiable formula are rejected. Trim failures in the measured rows: 0.

## Results (geometric means per cell) [pcc:unverified]

UNSAT (non-decomposable) targets, series A, n = 17:

| l | solver | rows | conflicts | proof lemmas | core lemmas T1 | core T2 (rows) | T1/conflicts | 2^l scan | 2^(2l) MITM | 2^(3l)/6 |
|---|---|---|---|---|---|---|---|---|---|---|
| 2 | cd15 | 8 | 290 | 526 | 296 | 296 (8) | 1.02 | 4 | 16 | 11 |
| 3 | cd15 | 8 | 2,677 | 3,018 | 1,802 | 1,801 (8) | 0.67 | 8 | 64 | 85 |
| 4 | cd15 | 6 | 25,534 | 31,975 | 22,063 | 22,027 (6) | 0.86 | 16 | 256 | 683 |
| 5 | cd15 | 7 | 304,669 | 325,871 | 246,904 | 220,192 (1) | 0.81 | 32 | 1,024 | 5,461 |
| 2 | g4 | 8 | 486 | 486 | 454 | 454 (8) | 0.93 | | | |
| 3 | g4 | 8 | 19,763 | 19,763 | 17,749 | 17,736 (8) | 0.90 | | | |
| 4 | g4 | 1 | 245,980 | 245,979 | 231,529 | 230,701 (1) | 0.94 | | | |

Series B cells (cd15, UNSAT): (11,3): conflicts 2,205, core T1 2,194
(ratio 0.995); (13,4): 21,134 / 19,930 (0.94); (17,5) as above. [pcc:unverified]

SAT targets (cd15, conflicts only, mostly planted): n = 17: l = 2: 139
(4 rows), l = 3: 754 (4), l = 4: 3,471 (2), l = 5: 50,408 (2). [pcc:unverified]

Fitted slopes, bits per l, least squares on per-instance log2 values, 95%
bootstrap interval in brackets [pcc:unverified]:

| series | solver | metric | l = 2..5 | l = 3..5 |
|---|---|---|---|---|
| A | cd15 | conflicts (B1, baseline SAT arm) | 3.34 [3.24, 3.42] | 3.41 [3.23, 3.57] |
| A | cd15 | decisions | 3.12 [3.05, 3.19] | 3.37 [3.20, 3.52] |
| A | cd15 | proof lemmas | 3.12 [3.00, 3.23] | 3.38 [3.16, 3.57] |
| A | cd15 | core lemmas T1 | 3.27 [3.14, 3.38] | 3.55 [3.31, 3.77] |
| A | cd15 | core lemmas T2 (1 row at l = 5) | 3.13 [2.98, 3.28] | 3.54 [3.17, 3.87] |
| A | cd15 | core literals T1 | 3.79 [3.62, 3.94] | 3.60 [3.42, 3.78] |
| A | g4 | conflicts, l = 2..4 (1 row at l = 4) | 4.95 [4.29, 5.71] | |
| A | g4 | core lemmas T1, l = 2..4 | 4.92 [4.24, 5.71] | |
| B | cd15 | conflicts, l = 3..5, n = 11, 13, 17 | | 3.55 [3.35, 3.73] |
| B | cd15 | core lemmas T1 | | 3.40 [3.19, 3.58] |
| A | cd15 | SAT targets, conflicts | 2.73 [2.44, 3.02] | 2.96 [2.12, 3.79] |

Reference count slopes: linear 2+1 scan 1, MITM 2, tuple enumeration 3.

T3 exploratory minimisation (n = 17) [pcc:unverified]:
- l = 2, 4 targets: best core over all solvers and rounds 249, 245, 163,
  224 lemmas, against the cd15 iterated-trim core 343, 628, 213, 224
  (ratios 0.73, 0.39, 0.77, 1.00).
- l = 3, 3 targets: best 1,293, 1,205, 1,219 against cd15 cores 1,347,
  1,205, 1,219 (ratios 0.96, 1.00, 1.00). Other solvers' cores at l = 3 were
  2 to 40 times larger than the cd15 core.
- Two sizes only, so no slope claim; geometric means 217 (l = 2) and 1,238
  (l = 3) differ by 2.5 bits.

Secondary (wall time, contended machine, indicative only): cd15 solve at
l = 5 took 17 to 35 s; one trim pass 100 to 170 s.

## Verdict against the pre-written kill criterion

Kill criterion: trimmed slope within 0.3 bits/l of the conflict slope AND
above 1.7 bits/l, on the primary series.

- Series A, cd15, l = 2..5 (4 sizes, 29 UNSAT rows): conflict slope 3.34,
  trimmed (T1) slope 3.27, difference 0.07; T2 3.13, difference 0.21 (T2
  has one row at l = 5). Both above 1.7. For l = 3..5 the trimmed slope
  (3.55) is above the conflict slope (3.41). [pcc:unverified]
- The trimmed core keeps 67% to 102% of the conflict count in every cd15
  cell and 90% to 94% for Glucose; the ratio does not fall with l.
- Stronger minimisation (T3) changed the core by a factor between 0.39 and
  1.00 at l = 2 and between 0.96 and 1.00 at l = 3.

**Kill criterion met** for this encoding and these solvers: the trimmed
DRAT core grows at the same rate as the conflict count, about 3.1 to 3.5
bits per l, which is above tuple enumeration (3), far above the MITM count
(2) and the linear 2+1 scan (1). Distance remaining to the named baseline:
at l = 5 the trimmed core has 2.5e5 lemmas against 32 leaf solves of the
linear 2+1 scan (ratio about 7.7e3), and the slope gap is about 2.3 bits
per l. [pcc:unverified]

What this does and does not show: it shows that backward trimming of
CDCL-found proofs does not expose short refutations here. It does not show
that no short resolution/DRAT refutation exists: trimming only removes
unused lemmas from the proof the solver happened to find, and the encoding
has long XOR chains where resolution is known to be weak.

## Deviations from PROTOCOL.md (all of them)

1. Trimmer: own `rup_trim.c` instead of drat-trim (running a build of the
   fetched drat-trim source was refused by the session's permission layer;
   the fetched file was not used). Amended in PROTOCOL.md before measuring.
2. Sizes: the protocol planned n = 23, l = 3..7. The first probe showed
   2.8e4 conflicts at (17,4) and 3.3e5 at (19,4) on this encoding, so
   n = 23 was unaffordable. Series A was run at n = 17, l = 2..5, series B
   at (3,11), (4,13), (5,17). Chosen from the cost probe, before any
   proof-size data existed. The probe rows (seed 1) are not in the results.
3. Order: the two first l = 5 processes were launched about one minute
   before the planted-lift validation passed (its first run failed on
   planted targets with P1 = -P2, a generator defect: the chain cannot
   express X at infinity; fixed by drawing distinct base x). UNSAT targets
   do not use the planted generator; the planted SAT rows at l = 5 were
   produced after the fix.
4. Time censoring, not result-based: (a) l = 5 target index 0 (the largest
   proof) is missing from the fits because its iterated trim did not finish;
   from its scratch files: 532,934 proof lemmas, 405,370 T1 core lemmas
   (ratio 0.76), T2 after 4 rounds 400,675. Because the censored row is the
   largest one, the l = 5 means are biased slightly low for conflicts and
   core alike. (b) l = 4: 6 of 8 UNSAT targets for cd15 and 1 for g4;
   2 of 4 SAT. (c) T2 at l = 5 was run for one target only (rounds = 1 for
   the others; T2 is null in those rows). (d) g4 not run at l = 5.
5. A first launch of l = 2..4 was killed and restarted after the generator
   fix; target index 3 at l = 5 was started twice (duplicate killed). Only
   first-written rows are kept (`analyze.py` de-duplicates).
6. SAT targets at n = 17 are mostly planted (random targets are almost
   never decomposable at n = 17, l <= 5).
7. Up to 4 measured processes ran concurrently (one core each) on a machine
   with load 5 to 6.5; wall times are not comparable across rows.
8. No docs/session-logs entry, benchmark record, Conductor task, ecdb or
   research-kb query: the task card restricts writes to this directory and
   those tools are not installed here. The session-log entry is below.

## What a full study would need

- drat-trim/dpr-trim and an LRAT check as the reference trimmer; kissat and
  CryptoMiniSat (BVA/Gauss, FRAT) as further proof sources.
- The repository's own encodings (ecc2k130/codegen cnf/wdsat, symmetry
  breaking, the (s,p)-linear encoding) instead of this pilot CNF, whose
  conflict counts (3.3 to 3.4 bits/l, n-dependent) are above 2^(3l)/6.
- More sizes (l up to 7 or 8 with a cheaper encoding), n scaled as in
  ECC2K-130-style parameters, >= 30 targets per cell, EKS route.
- Proof systems beyond resolution: the question "do short refutations
  exist" should be asked of XOR-aware (BDD/Gauss with DRAT), polynomial
  calculus/Nullstellensatz degree, and of proofs derived from the linear
  2+1 scan itself (a 2^l * poly certificate exists by construction; whether
  it translates to a 2^l * poly DRAT proof is the real go/no-go test and
  was not attempted).
- Real proof minimisation (MUS of lemma sets, proof-guided re-solving at
  l >= 4), which was only touched at l = 2, 3.

## Files

- `PROTOCOL.md`, `pdp.py` (field, curve, S3, CNF), `rup_trim.c` (trimmer),
  `validate.py`, `validation.json`, `run.py`, `t3.py`, `analyze.py`.
- Raw rows: `A234.jsonl`, `A4a.jsonl`, `A4b.jsonl`, `A5b..A5h.jsonl`,
  `B.jsonl`, `T3.jsonl`; merged: `results.json`; tables: `tables.md`.

Rerun of the smallest cell (about 1 s):
`S=$(mktemp -d); gcc -O2 -o $S/rup_trim rup_trim.c && python3 run.py $S A 17 2 8 4 /tmp/a2.jsonl`

## Session log entry (2026-11-14, kept here because writes are restricted to this directory)

### What was done
Protocol, generator, trimmer, validation, runs and analysis listed above.

### Decisions made
n = 17 instead of 23 (cost probe); own RUP trimmer (drat-trim build refused);
T2 limited to one target at l = 5 (each extra trim round cost 1 to 3 minutes
and shrank the core by under 2%).

### Open threads
Translate the linear 2+1 scan certificate into a clausal proof and measure
its size; repeat with the repository encodings and a reference trimmer.

### Errors
Planted generator allowed P1 = -P2 (fixed); a pkill pattern missed an old
process chain (found, killed, rows re-generated); l = 5 target 0 censored.
