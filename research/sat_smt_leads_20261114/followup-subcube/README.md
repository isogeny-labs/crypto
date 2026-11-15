# Follow-up pilot: can a learning solver refute x1-subcubes below the linear 2+1 scan? (2026-11-14)

Pilot at toy sizes in a shared 4-core container. Every number here is an unverified pilot
assertion, tagged `[pcc:unverified]`. No benchmark record, leaderboard row or pcc claim.
Protocol written before the measured runs: [`PROTOCOL.md`](PROTOCOL.md). Code: `sub.py` (field,
curve, generator, encodings, CMS wrapper, own engine), `validate.py`, `run.py`, `analyze.py`.
Raw rows: `results.json` (1 808 rows, merged from `raw/*.jsonl`). All tables: `tables.md`.

**Decision by the pre-registered rule: KILL.** At the largest size (l = 8) every CryptoMiniSat
budget arm has rho >= 2.48 (rule: >= 1), and alpha = 1.81 (rule: >= 0.9). The alpha value comes
from l = 4 only, see "Alpha" below. [pcc:unverified]

## Setup

- F_{2^n}, (l, n) = (4, 13), (5, 17), (6, 19), (7, 23), (8, 29): n is the smallest prime >= 3l + 1,
  not fixed. Curve y^2 + xy = x^3 + x^2 + B, B from the cell seed 2026111400 + l.
  V = span(1, .., t^(l-1)), factor base = rational points with x in V \ {0}.
- 8 non-decomposable random targets and 4 planted (decomposable) targets per cell, the same for
  every arm. Baseline named throughout: the linear 2+1 scan, counted as 2^l leaf solves.
- CryptoMiniSat 5.18.0 (pycryptosat), one thread, options `autodisablegauss=0, sls=0`.
- Validation (`validation.json`): l = 3 and l = 4, 32 targets each (17 decomposable in total).
  Pair-table truth equals enumeration of all signed triples; CMS free solves (plain and dual
  encodings, two bases), CMS DFS (3 budgets, keep and discard) and the engine DFS agree with it;
  102 SAT models lift to curve points with signs summing to R; 3 072 per-x1 leaf answers (CMS
  dual and engine) equal the enumeration of decompositions using that x1. x = 0 (2-torsion) is
  excluded on both sides; targets have x(R) outside V, so no pair sums to infinity.

## Part A: conflicts per leaf (x1 fully assumed), non-decomposable targets [pcc:unverified]

| l | n | plain AND gates, incremental | plain, fresh solver per leaf | dual-rail, incremental | dual-rail, fresh solver per leaf |
|---|---|---|---|---|---|
| 4 | 13 | 151 | 248 | 3.36 | 18.0 |
| 5 | 17 | 551 | 728 | 1.64 | 23.4 |
| 6 | 19 | 1 949 | 2 735 | 1.75 | 19.1 |
| 7 | 23 | >= 8 390 (15 of 256 leaves at the 20 000 cap) | >= 10 016 (25 capped) | 1.68 | 29.7 |
| 8 | 29 | >= 19 570 (239 of 256 capped) | >= 19 594 (240 capped) | 1.60 | 26.4 |
| slope, bits per l | | 1.80 (lower bound) | 1.64 (lower bound) | -0.21 | +0.15 |

Plain arms use all leaves for l <= 5 and 32 sampled leaves per target for l >= 6; dual arms all
leaves (fresh: 64 sampled for l >= 7). 22 to 32 % of dual-rail incremental leaves need 0 conflicts;
the largest single leaf was 73 conflicts. Own engine: exactly 2 sub-branches per reached leaf
(the two roots u), 0 conflicts by construction, 1 760 to 12 177 row XORs per leaf.

**Diagnosis.** With x1 assigned, a gate t = c_i & u_j with c_i = 1 becomes the equivalence
t = u_j, but it lives in two binary clauses, not in the XOR set. The Gauss-Jordan matrix keeps
t and u_j as unrelated columns, so the n rows of S3(x1, xR, u) = 0 stay underdetermined and the
solver searches the n bits of u by decisions and learning (and again the bits of (s, p) through
the gates u_j & p_k, u_j & s_i). The dual-rail gate adds r = ~c_i & u_j and the XOR
t + r + u_j = 0: whichever way c_i is set, one rail is a unit and the XOR ties the other rail to
u_j inside the matrix. The system in u is then linear for Gauss, leaves a one-dimensional
solution set (the two roots u = x(R + P1), x(R - P1)), and after one decision on u the systems
in (s, p) and then in z are linear in the same way. That is the 1.6 conflicts per leaf. The
earlier pilot's fixed n = 23 added to its level at small l. Not diagnosed: why a fresh solver
needs 18 to 30 conflicts for the same leaf.

Part A target (O(1) conflicts per leaf) is met by the dual-rail incremental arm and by the engine.

## Part B: DFS over x1's coordinates with a conflict budget per subcube [pcc:unverified]

rho = (leaves visited + conflicts) / 2^l, mean of 8 non-decomposable targets; 1 = the scan count.
Trend = slope of log2(rho) over l = 4..8. "own scan" = rho divided by the same solver's B = 0 arm.

Keep learnt clauses (one incremental solver), standard coordinate order:

| budget per subcube of dimension j | l=4 | l=5 | l=6 | l=7 | l=8 | trend, bits per l | rho / own scan at l=8 | leaves visited at l=8 |
|---|---|---|---|---|---|---|---|---|
| 0 (the scan, done by CMS) | 3.88 | 3.41 | 3.40 | 2.49 | 2.48 | -0.17 | 1.00 | 256 |
| 10 | 5.93 | 4.71 | 4.44 | 3.37 | 3.75 | -0.18 | 1.51 | 3.8 |
| 100 | 20.3 | 15.1 | 12.4 | 11.7 | 7.95 | -0.31 | 3.21 | 1.8 |
| 1000 | 109 | 96.9 | 59.6 | 42.9 | 43.3 | -0.38 | 17.5 | 0 |
| 0.25 * 2^j | 4.35 | 3.41 | 3.95 | 3.30 | 3.52 | -0.07 | 1.42 | 148 |
| 0.5 * 2^j | 4.60 | 3.79 | 4.21 | 4.03 | 4.39 | 0.00 | 1.77 | 16.2 |
| 1 * 2^j | 5.73 | 4.74 | 6.01 | 5.50 | 6.01 | +0.04 | 2.43 | 9.0 |

Control, discard learnt clauses (fresh solver per node):

| budget | l=4 | l=5 | l=6 | l=7 | l=8 | trend |
|---|---|---|---|---|---|---|
| 0 | 19.0 | 24.4 | 20.1 | 29.7 | 26.5 | +0.12 |
| 10 | 28.8 | 35.8 | 31.8 | 41.3 | not run (protocol) | +0.14 |
| 100 | 58.1 | 76.7 | 75.4 | 88.8 | 108 | +0.20 |
| 1000 | 114 | 234 | 264 | 312 | not run (protocol) | +0.45 |
| 0.25 * 2^j | 20.4 | 26.1 | 22.0 | 32.0 | not run (protocol) | +0.17 |
| 0.5 * 2^j | 22.5 | 28.6 | 24.8 | 34.9 | not run (protocol) | +0.17 |
| 1 * 2^j | 24.7 | 31.2 | 27.9 | 38.5 | 36.3 | +0.14 |

Readings:

- No arm is under 1 at any size. The lowest arm at every size is B = 0, which is the scan
  performed by CMS at 1.5 to 2.9 conflicts per leaf. Every arm that lets the solver attempt
  subcubes costs more than that scan: 1.3x to 1.5x for B = 10, and the proportional budgets
  drift upward against it (1 * 2^j: 1.48, 1.39, 1.77, 2.21, 2.43 for l = 4..8).
- The negative trends of the absolute-budget arms follow the B = 0 arm's own fall from 3.88 to
  2.48, i.e. the per-leaf conflict count settling, plus a fixed budget shrinking relative to
  2^l. Relative to the solver's own scan no arm falls with l except B = 100 and B = 1000,
  which stay 3x and 17x above it. rho' (solver calls + conflicts) gives the same ordering.
- With learnt clauses kept, subcubes are pruned above the leaves: at l = 8 and B = 10 the mean
  prune counts at depths 5, 6, 7, 8 are 16.5, 23.8, 12.6, 3.8 (99 % of leaves covered by
  subcubes of dimension 1 to 3), but the DFS spends 957 conflicts for 256 leaves, 3.7 per
  covered leaf, against 1.5 per leaf in the B = 0 scan. Pruning happens; it is not cheaper
  than the leaves it removes.
- Keep against discard at l = 8: 2.48 against 26.5 (B = 0), 7.95 against 108 (B = 100),
  6.01 against 36.3 (1 * 2^j). The discard arm also pays the fresh-solver leaf cost seen in
  Part A, so this ratio is not a clean measure of clause reuse.
- Controls, keep arm, l = 8 (random coordinate order / second basis of V): B = 0 2.61 / 2.47,
  B = 10 3.77 / 4.17, B = 100 7.86 / 8.16, 1 * 2^j 5.90 / 6.13. Same picture.
- Own engine E (Gauss + gate substitution, no learning): 0 prunes above depth l in all 120
  runs (5 sizes, 3 orders, 8 targets). 47 to 56 % of the x1 values reach a consistent u (the
  rest fail the lift condition on arrival at depth l); each of those costs 2 sub-branches.
- Decomposable targets (4 per cell, keep, reported separately): the DFS stops at the first
  model. rho at l = 8: 0.94 (B = 0), 1.65 (B = 10), 5.27 (B = 100), 33.3 (B = 1000), 1.71, 2.30,
  3.81 (proportional). The scan with early stop is the matching reference, not 2^l.

### Alpha: cost of refuting one subcube of dimension j, fresh solver, cap 20 000 conflicts

Mean conflicts (multiple of 2^j):

| l | j=0 | j=1 | j=2 | j=3 | j=4 | alpha per protocol |
|---|---|---|---|---|---|---|
| 4 | 16 (16x) | 43 (21x) | 206 (52x) | 1 422 (178x) | 2 531 (158x) | 1.81 over j = 2..4 |
| 5 | 31 | 48 | 353 (88x) | 2 021 (253x) | censored 8 of 16 | not defined (2 uncensored j) |
| 6 | 20 | 53 | 444 (111x) | 2 163 (270x) | censored 3 of 16 | not defined |
| 7 | 10 | 64 | 609 (152x) | 1 658 (207x) | censored 6 of 16 | not defined |
| 8 | 14 | 170 | 863 (216x) | 5 155 (644x) | censored 6 of 16 | not defined |

The protocol's alpha needs 3 uncensored dimensions j >= 2 and exists only at l = 4 (1.81). The
two-point slopes j = 2 -> 3 at l = 5..8 are 2.5, 2.3, 1.4, 2.6. No cell shows cost growing slower
than 2^j, and the level is 50x to 640x the leaf count. This is a cold solver; the warm
incremental solver of the keep arm refutes dimension 1 to 3 subcubes within about 10 conflicts,
after paying for failed attempts higher up (the 3.7 conflicts per covered leaf above). A
per-node record of warm refutation cost against j was not taken.

## Desk note: can a cheap relaxation refute a subcube?

Fix k of x1's coordinates, j = l - k free, and linearise every remaining monomial (what the
engine and a B = 0 XOR solver do):

- Stage 1, S3(x1, xR, u) = 0: n equations. Unknowns: n bits of u, the j free coordinates, and
  the j*n products c_i u_j. That is n + j(n + 1) unknowns for n equations, and the n equations
  restricted to u alone already have rank n - 1. One free coordinate adds n + 1 free columns.
- Stage 2, S3(x2, x3, u) = 0: n equations in 3l - 1 bits of (s, p) and n(3l - 1) products with
  the bits of u. Fixing coordinates of x1 removes none of these, because u is not determined.
- Stage 3, z^2 + sz + p = 0: 2l - 1 nontrivial equations, l^2 products s_i z_j.
- Total at l = 8, n = 29, j = 1: 73 equations against 821 linearised unknowns. A degree-2
  relaxation cannot be inconsistent before u is known, and u is known only when j = 0 and the
  root bit is chosen. Measured: 0 engine prunes above depth l.
- Raising the degree (multiplying by monomials) is XL / Groebner on the full system, the
  known route whose cost the scan already undercuts at these sizes; eliminating u gives S4,
  whose degree in (s, p) does not drop when coordinates of x1 are fixed.
- The one cheap test that removes about half the leaves, the lift condition
  Tr(x1 + 1 + B / x1^2) = 0, is a trace of an inverse and is not constant on subcubes of
  dimension >= 1, so it prunes leaves and not subcubes.

So the negative result has a counting explanation: a subcube refutation must reason through
u, and u carries n > 3l unknown bits that only a full x1 pins down.

## Deviations and limits

- Development of the dual-rail encoding and the two solver options used seed 999 at
  l = 4, 5, 6 before PROTOCOL.md; it showed CMS disabling its Gauss matrix after long searches
  (one leaf then cost 2 * 10^5 conflicts), hence `autodisablegauss=0`.
- `validate.py` ran on the l = 4 cell seed (different target draws) before the measured run.
- One PROTOCOL.md sentence (the choice of n) was reworded after validation and before the
  measured runs; no content changed.
- l = 8 ran as three processes (targets 0-3, 4-7, planted), l = 5 and 6 in one, l = 7 in one,
  up to 3 at once on a shared machine: conflict counts are unaffected, wall times are not
  comparable. No row is NOT_RUN; 64 alpha rows are skipped after censoring, per protocol.
- CMS conflict limits overshoot by a few conflicts and limit 0 returns without searching;
  conflicts actually spent are counted. pycryptosat exposes no per-call propagation or Gauss
  counts, so "propagation/Gauss work" is recorded for the engine only (row XORs, fixpoint
  calls) and as wall time for CMS.
- Not covered: m = 4; l > 8; fixed n; a second V (only a second basis of the same V); CMS
  option sweeps; decision-variable restriction inside CMS; a learning version of the own
  engine; warm per-node refutation cost against j; any proof-system lower bound. One conflict
  is counted as one leaf solve, which is a convention, not a measured cost equivalence.
- Not done because this pilot was confined to its directory: `docs/session-logs/` entry,
  Conductor task, research-kb receipt, visual report. This README is the handoff.

## Rerun

Smallest measured cell, about 10 s:
`python3 run.py /tmp/fsub_l4.jsonl 540 4 && python3 analyze.py /tmp/fsub_l4.jsonl`
Validation (about 45 s): `python3 validate.py 3 4`. Everything: `run.py raw/lN.jsonl 540 N`
for N = 4..8, then `python3 analyze.py > tables.md` (rewrites `results.json`).
