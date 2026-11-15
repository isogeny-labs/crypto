# Lead E protocol: size of scan-guided refutations (written before the measured runs)

Date 2026-11-15. Local pilot, toy sizes, one core. Every number this pilot produces is
`[pcc:unverified]`. No benchmark record, leaderboard row or pcc claim. The directory is not under
version control, so file times are the only evidence of ordering.

## Question

Pilot 01 (2026-11-14) measured that backward-trimmed DRAT cores of refutations found by CDCL grow
at 3.27 bits per l (n = 17, l = 2..5). It did not test whether a short refutation exists. The
linear 2+1 scan decides the instance in 2^l leaf solves. This pilot writes the scan down as a
proof and measures its checked size.

## Claim under test and prediction

Claim: a refutation that follows the scan (one leaf refutation per x1 in V, resolved to the empty
clause over the l coordinates of x1) has size about 2^l times a per-leaf factor, so its slope in
l at fixed n is about 1 bit per l.

Prediction before data: at fixed n the trimmed scan-guided clausal proof has slope between 1.0
and 1.5 bits per l. The per-leaf factor of the clausal form is not expected to be polynomial in
n: a CDCL solver without XOR reasoning has to redo Gaussian elimination by resolution, so with n
growing with l (secondary series) the slope is expected to be well above 1. The XOR-aware step
count is expected at about 1 bit per l in both series.

## Instances

- `sub.py` is a verbatim copy of `../../sat_smt_leads_20261114/followup-subcube/sub.py`
  (field, curve y^2 + xy = x^3 + x^2 + B, V = span(1, t, .., t^(l-1)), S3 chain with variables
  x1 (c), u = x(R - P1), s, p, z).
- Added: the clause u != 0 (the earlier encoding had a false SAT at u = 0 through non-rational
  x2, x3). Targets with R -+ P1 equal to the 2-torsion point for some base point are rejected,
  so ground truth and encoding exclude the same case. x = 0 is not in the factor base (clauses
  x1 != 0 and p != 0). Targets have x(R) outside V, so no intermediate sum is the point at
  infinity.
- CNF: dual-rail gates (t = a & b, r = ~a & b, t + r + b = 0) and every XOR row cut into
  3-variable XORs with auxiliaries, variables in index order (4 clauses of width 3 each). No
  native XOR constraints: the clausal arm is pure CNF.
- Cell seed = 20261115 * 1000003 + 1000 n + 10 l. Targets are drawn in order from the cell's
  stream; the first 6 non-decomposable ones (by the pair-table ground truth) are used.
- Development and cost probes used seed 999 only (`probe.py`); no probe row is reported.

Series:

- **Primary (fixed n, as in pilot 01): n = 17, l = 3, 4, 5.** 6 targets per cell.
- Extension of the primary series: n = 17, l = 6, as many of the 6 targets as fit the time
  budget, in target order, one trimming pass only. Censoring here is by time, never by result.
- Secondary (n grows with l): (l, n) = (3, 11), (4, 13), (5, 17) [(5, 17) is the primary cell],
  and (6, 19) only if time allows.

## Arms (every arm on the same targets)

1. `cold`: scan-guided clausal proof, CaDiCaL 1.5.3 (`cd15`) with DRAT logging, a fresh solver
   per leaf, x1 given as assumptions (so every lemma is implied by the formula alone and no
   literal has to be added to it). The 2^l lemma lists are concatenated in increasing x1; after
   each list the leaf clause (the negated x1 assignment) is appended; then the 2^l - 1 clauses of
   the binary resolution tree over the coordinates of x1, ending in the empty clause.
2. `warm`: the same with one incremental solver for all leaves (lemmas of earlier leaves stay
   available).
3. `xor`: own DPLL + Gauss engine (`sub.Eng`: Gauss-Jordan over the XOR rows plus gate
   substitution), x1 first. Counted: row additions, gate substitutions (the unit-propagation
   steps), search nodes; root elimination counted once. Reported as steps and as the clause count
   of a standard XOR-to-DRAT translation: 8 clauses per variable in the union support of each row
   addition (4 definition clauses for the fresh prefix-parity variable of the result, 4 lemmas
   for the prefix invariant) plus 1, 2 clauses per gate substitution, 1 per search node. This arm
   is a count. **It is not emitted as a proof and not machine-checked**; it is reported as
   unchecked. The engine's verdict is compared with ground truth on every target.
4. `free` (reference on this encoding): Glucose 4 on the same CNF with no guidance, conflict
   budget 300 000, proof trimmed the same way. Rows over budget are reported as censored. l = 3
   and 4; l = 5 only if the l = 4 rows finish under the budget.
5. References, not measured: pilot 01's stored slopes (CDCL-found, other encoding, n = 17,
   l = 2..5: conflicts 3.34, trimmed core 3.27) and the count 2^l.

## Checking

Every clausal proof is checked by `rup_dep.c` (a copy of pilot 01's `rup_trim.c` that also writes
core indices and, per checked lemma, the lemmas met in its conflict analysis): backward RUP
check from the empty clause against the full CNF, deletions ignored. Trimmed proofs (the marked
clauses, in order) are checked again; the reported trimmed size is the size of a proof that the
checker verified as given. Where the trimmed proof has at most 60 000 clauses it is also checked
forward by an independent code path (MiniSat 2.2 unit propagation through python-sat, lemma by
lemma). A proof that fails or is not checked is reported as such and not used in a fit.

Metrics per proof: raw clause count; T1 = clauses marked by one backward pass; T2 = fixpoint of
re-trimming (at most 8 passes; l = 6: T1 only). **Trimmed proof size = number of proof clauses
(solver lemmas, leaf clauses, tree clauses, empty clause) in the T2 proof (T1 where T2 is not
run).** Literal counts are reported as well.

## Validation before the measured runs (`validate.py`, cell (l, n) = (3, 11), the smallest)

- CNF satisfiability equals plain enumeration of all signed triples (`Cell.brute`) on random and
  planted targets; every model lifts to rational points with x in V \ {0} summing to R.
- Per leaf: the CNF under x1 is SAT exactly when enumeration finds a decomposition using that x1.
- u = 0: on crafted targets R = P1 + T the CNF with the u != 0 clause agrees with enumeration
  restricted to decompositions whose partial sum P2 + P3 is not T for some ordering; the count of
  false SAT answers without the clause is recorded.
- The engine's verdict equals ground truth.
- Checker: scan proofs verify; a proof stripped of its solver lemmas and a proof for a
  decomposable target with a forged leaf clause must be rejected; core clauses of the formula are
  UNSAT under another solver; forward check agrees.

## Analysis and decision rules (fixed now)

- Slope: least squares of per-target log2(size) on l, 95 % interval by bootstrap over targets
  within cells (2 000 draws, seed 1).
- **Step 2 rule.** "short proofs exist" if, on the primary series (>= 3 sizes, 6 targets each),
  every scan-guided clausal proof used is verified by the checker and the trimmed-size slope of
  the better scan-guided arm is <= 1.5 bits per l, with the upper end of its interval reported.
  "not shown" otherwise (slope above 1.5, any unverified proof, or fewer than 3 complete sizes).
  The secondary series is reported next to it and cannot rescue or overturn the primary decision;
  if its slope is above 1.5 the README says so in the same paragraph.
- Fixed n makes the per-leaf linear system the same size at every l, so a slope near 1 on the
  primary series tests only that leaf proofs compose without blow-up; it says nothing about the
  n-dependence of the per-leaf factor. The secondary series and the xor arm are there for that.
- **Step 3 (lower-bound probe, n = 17, l = 3 and 4, all 6 targets).** Proof sources: cold and
  warm, `cd15` and `g4`. Minimisation: iterated trimming; re-solving the scan on the core
  original clauses only (up to 3 rounds, with and without the previous core's lemmas seeded as
  proof lemmas); removal of lemmas subsumed by an earlier clause followed by re-trimming. The
  smallest verified proof per target is kept. Reference without sharing: the sum over leaves of
  the trimmed size of that leaf's own cold `cd15` proof checked alone.
- Sharing statistic. Closing clause: a proof clause over x1-coordinate variables only; its cube
  is the set of leaves falsifying it. Reasoning lemma: any other proof clause. A reasoning lemma
  serves the union of the cubes of the closing clauses whose RUP check reaches it through
  reasoning lemmas only. Per proof: the median number of leaves served. (The probe on seed 999
  showed that solvers emit copies of leaf clauses and that tree clauses can be RUP without their
  children, so leaf usage has to be defined through cubes, not through the appended leaf
  clauses.)
- **Step 3 rule.** "cross-leaf sharing" only if the median over targets of the per-proof median
  is >= 4 leaves at the largest l of the primary series with 6 complete targets, on the smallest
  verified proof per target among the arms run at that l. Otherwise "no cross-leaf sharing
  shown". Also reported, without a rule: mean, 90th percentile, share of lemmas serving >= 4
  leaves, and minimised size divided by the no-sharing reference.

## Limits fixed in advance

One process, one core, `nice`; single runs under 10 minutes; about 75 minutes in total. No
tuning: solver options are defaults, and no arm is re-run with other settings after its numbers
are seen. Words "improvement", "speedup", "beats" are not used as claims.
