# Idea 5 — conflict-directed polynomial lemmas (pilot, 2026-11-14)

Pilot only. Toy sizes, one core of a shared container. Every number below is
an unverified pilot assertion `[pcc:unverified]`. Protocol and kill criterion
were written before the measured run: `PROTOCOL.md` (addendum = parameters
fixed on calibration seeds idx 900-902, which are not reported as results).

## What was run

- m = 3 binary point decomposition, F_{2^13} (x^13+x^4+x^3+x+1), curve
  y^2+xy = x^3+x^2+b (b per seed), V random l-dimensional, l = 2, 3, 4.
  System: p = x1 x2, S3(x1,x2,t) = 0, S3(x3,X_R,t) = 0 in bits; 3n+l XOR
  equations over variables and AND-gate monomial variables (`pdp5.py`).
- Solver: Glucose 4 (python-sat), XORs cut to arity <= 4 in CNF, identical for
  all arms. Outer loop of conflict-budget rounds (B = 470 / 1600 / 9500).
  **Deviation from the idea text:** not pycryptosat. The installed pycryptosat
  5.18 exposes no counters and no learnt clauses; Glucose gives exact counters
  and learnt clauses through its proof log. No native-XOR solver arm was run.
- Arms, same instances, interleaved per instance:
  A0 untreated; A1 static full degree-2 Macaulay lemmas (all variables as
  multipliers) added up front; A2 conflict-directed (multipliers = top-k
  variables in the last round's learnt clauses, <= 64 new lemmas per round);
  A3 random supports of size k with lemma count matched to A2 per round.
  k = 8 is the protocol arm; k = 12 is a secondary pair declared in the
  addendum before measurement.
- Lemma generator: rows {f} and {v*f}, multilinear ANF, Gaussian elimination
  with cubic and non-existing quadratic monomials eliminated first; kept rows
  are XORs over existing monomial variables, independent of the equations and
  of earlier lemmas. Cost counted in 64-bit word XORs.
- 8 targets per cell planned (4 planted, 4 random points). l = 2 and l = 3:
  all 8 x 6 arms. **l = 4 is incomplete:** the run was stopped at the time
  budget; idx 0,4,1,5,2 have all six arms, idx 6 has A0/A1/A2k8 only, idx 3
  and 7 were not run. 129 rows in `rows.jsonl` / `results.json`.

## Validation [pcc:unverified]

- Status of every row equals brute-force enumeration of V^3 for the same
  algebraic system (all three cells): 129/129.
- All planted targets SAT; all 66 SAT models satisfy the S3 chain and lift to
  curve points P1 ± P2 ± P3 with x-coordinate x(R): 66/66.
- All lemmas of all arms vanish on the planted solutions (soundness check on
  planted instances only).

## Results [pcc:unverified]

Full tables: `tables.md`. Conflicts are Glucose totals over all rounds.

Median conflicts (all targets; n = instances with that arm):

| l | A0 untreated | A1 static | A2 directed k=8 | A3 random k=8 | A2 k=12 | A3 k=12 | enum 2^(3l) | MITM 2^(2l) | linear 2+1 scan 2^l |
|---|---|---|---|---|---|---|---|---|---|
| 2 (n=8) | 2324 | 992 | 1508 | 2064 | 1826 | 2252 | 64 | 16 | 4 |
| 3 (n=8) | 13431 | 9523 | 10866 | 11684 | 10543 | 15162 | 512 | 64 | 8 |
| 4 (n=6/6/6/5/5/5) | 76090 | 60616 | 275282 | 309730 | 324344 | 72816 | 4096 | 256 | 16 |

Paired geometric-mean conflict ratios (bootstrap 95% over instances):

| l | random/directed k=8 (A3/A2) | A3/A2 k=12 | untreated/directed k=8 (A0/A2) | A0/A2 k=12 | untreated/static (A0/A1) |
|---|---|---|---|---|---|
| 2 | 1.19 (1.03..1.43), n=8 | 0.80 (0.55..1.09) | 1.31 (1.00..1.88) | 0.89 (0.61..1.31) | 2.29 (1.09..4.51) |
| 3 | 0.65 (0.38..1.05), n=8 | 0.99 (0.58..1.49) | 0.68 (0.41..1.07) | 0.78 (0.42..1.25) | 1.02 (0.35..2.51) |
| 4 | 1.24 (0.71..2.51), n=5 | 0.38 (0.19..0.84), n=5 | 0.39 (0.22..0.66), n=6 | 0.30 (0.12..0.83), n=5 | 1.06 (0.28..3.49), n=6 |

Fitted slope, log2(median conflicts) against l, l = 2..4 (three points):

| arm | all targets | UNSAT only | SAT only |
|---|---|---|---|
| A0 untreated | 2.52 | 2.96 | 2.59 |
| A1 static | 2.97 | 3.11 | 2.71 |
| A2 directed k=8 | 3.76 | 3.96 | 3.15 |
| A3 random k=8 | 3.61 | 3.42 | 3.90 |
| A2 directed k=12 | 3.74 | 2.78 | 3.85 |
| A3 random k=12 | 2.51 | 3.21 | 3.17 |

UNSAT/SAT split is by brute-force truth; UNSAT cells have 4, 4 and 2-3
instances, so those slopes are indicative only.

Lemma generation: A2 k=8 added on average 24 / 38 / 32 lemmas per instance
(l = 2 / 3 / 4), mean length 2.4 / 10.7 / 29.9 monomials, at 1.0e4 / 6.1e4 /
3.2e5 word XORs; A1 static 143 / 101 / 50 lemmas at 3.0e5 / 4.4e5 / 6.5e5
word XORs. Generation cost is small next to 1e7-1e8 propagations at l = 4.

## Verdict against the pre-written kill criterion

**Kill criterion met** [pcc:unverified], on both clauses, for the k = 8
protocol arm:

- (a) random/directed conflict ratio at the largest cell (l = 4, 5 paired
  instances) is 1.24 (0.71..2.51), below the required 2.0; at l = 3 it is 0.65.
  For k = 12 the ratio at l = 4 is 0.38, i.e. the directed arm used more
  conflicts than its random-support control.
- (b) the directed arm's slope (3.76 bits/l) is not lower than the untreated
  arm's (2.52); it is higher. At l = 4 the directed arm used 1/0.39 = 2.6x the
  untreated arm's conflicts (geometric mean, n = 6).

Other observations, not claims: the static control A1 had fewer conflicts than
untreated at l = 2 (ratio 2.29) and was indistinguishable from it at l = 3 and
l = 4 (1.02, 1.06, intervals spanning 0.3..3.5), consistent with the
repository's earlier negative result for static degree-2 rows. Per-instance
spread is large (single ratios from 0.05 to 10.8), so nothing below a factor
of about 3 is resolvable at these sample sizes. All SAT arms remain far above
the linear 2+1 scan count 2^l; the untreated UNSAT slope (2.96) sits near
enumeration (3.0).

## Limits of this pilot

- l = 4 cell incomplete (5-6 of 8 instances); l = 5 not run. Slope from three
  points, the largest with 5-6 instances.
- Glucose honours the conflict budget only at restarts: rounds are longer than
  B, and at l = 2 five of eight instances (all four SAT ones) finished before
  any lemma was added, so A2/A3 equal A0 there (2 of 8 at l = 3, 0 at l = 4).
- "Conflict signal" = variable frequency in the round's learnt clauses, not a
  per-conflict implication-graph cut; multipliers are single variables only
  (degree-3 Macaulay closure restricted to existing degree-2 monomials); every
  equation is multiplied, not only "participating" ones, because the
  equations are dense in t.
- Lemmas are long XORs encoded in CNF with cut variables; a native-XOR solver
  (CryptoMiniSat Gauss) could treat them differently. Not tested.
- One n, one curve family, one encoding (S3 chain with p and t auxiliaries),
  one solver, one solver seed.
- The repository session log, research-kb lookup, benchmark record and
  Conductor task required by the repository rules were not done: this task was
  restricted to writing inside this directory and those tools are not installed.

## What a full study would need

Native-XOR arm (CryptoMiniSat with exposed statistics, or a patched solver
that exports conflict clauses and takes XOR lemmas at restart), exact per-round
budgets, l up to 6-7 with >= 16 targets per cell and several solver seeds, a
per-conflict support definition, a higher multiplier degree, and the
repository's own encoding (`ecc2k130/codegen`) next to this one.

## Files

`PROTOCOL.md`, `pdp5.py` (field, curve, S3 system, lemma generator, outer
loop), `run.py`, `analyze.py`, `rows.jsonl`, `results.json`, `tables.md`,
`run_l23.log`, `run_l4.log`, `explore.py` and `calib.py` (validation and
calibration helpers).

Re-run the smallest cell: `python3 run.py 2` (appends to `rows.jsonl`; about
5 s), then `python3 analyze.py`.
