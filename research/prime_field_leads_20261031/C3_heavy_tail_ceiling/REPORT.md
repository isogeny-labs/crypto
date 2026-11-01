# C3: the self-reducibility ceiling on every heavy-tail mechanism (scoring table)

**Lead.** C3 of `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`,
thresholds in `../PLAN.md`. **Kind.** Derivation and scoring; no experiment, no
counted measurement in this directory. **Decision.** writeup (the plan names no
threshold for C3; the null prediction "all mechanisms below the ceiling" is what
the table shows).

This file was written by the synthesis step (2026-11-01) from the two tables the
lead agent left (`ceiling_table.csv`, `mechanism_scoring.csv`) and the graph it
plotted from them (`ceiling_graph.svg`, script `plot_ceiling.py`, recorded values
only). The agent's run was cut off by host load before it wrote this report; no
number below is new.

## 1. The ceiling

Randomising a target is one group operation, R ↦ R + rP, and the discrete
logarithm is random self-reducible. If a mechanism is "easy" on a class of
targets of measure f, then

- the **mass gain** on the decomposition phase is at most 1/f: a sieve that
  re-randomises until the target falls in the class already prices the class at
  1/f attempts, so no selection rule can buy more than that factor; and
- the **screen budget** is what §2 of the note leaves for finding such a target:
  the relation phase is allowed p^{1/2 − 1/m} work per relation before it stops
  beating rho at the MITM baseline (p^{1/2 + 1/m} total over p^{1/m} relations),
  so a screen that tests one candidate per O(1) operations may test at most
  p^{1/2 − 1/m} candidates, i.e. it needs f ≥ p^{−(1/2 − 1/m)}.

For a mechanism to move the exponent it needs its per-attempt cost p^{e/m} to
drop below the threshold e < m/2 − 1 (note §2); the table converts that into the
log₂(1/f) the class must supply (`attempt_gain_log2_needed`), the mass
requirement (`mass_log2_inv_f_min`, which equals the gain needed since the gain
is at most 1/f) and the screen budget (`screen_log2_inv_f_max`).

## 2. Table (from `ceiling_table.csv`, n = log₂ p from 32 to 512; every row scales linearly in n)

| m | baseline | e_base | e needed | log₂(1/f) needed (mass) | log₂(1/f) allowed (screen) | window |
|---|---|---|---|---|---|---|
| 3 | MITM | 1.5 | < 0.5 | n/3 | n/6 | empty |
| 3 | ENUM | 2.0 | < 0.5 | n/2 | n/6 | empty |
| 4 | MITM | 2.0 | < 1.0 | n/4 | n/4 | one boundary point |
| 4 | ENUM | 3.0 | < 1.0 | n/2 | n/4 | empty |

At n = 256: m = 3 needs f ≤ 2^{−85.3} but may screen only 2^{42.7} candidates;
m = 4 needs f ≤ 2^{−64} and may screen exactly 2^{64}. So at m = 3 no
target-conditioned mechanism with an O(1) screen can reach the rho threshold,
whatever its tail, and at m = 4 only a mechanism whose class has measure exactly
p^{−1/4} and whose screen costs nothing beyond the test sits on the boundary
(rho_gain_per_bit column: 1.0 at that point, 2.0 to 3.0 elsewhere, meaning the
tail must supply two to three bits of gain per bit of screening it is allowed).

Graph: `ceiling_graph.svg` (mass requirement against screen budget, m = 3 and
m = 4, MITM baseline, against n).

## 3. Scoring of the catalogue (from `mechanism_scoring.csv`, 18 entries)

| Disposition | Mechanisms | Reason recorded |
|---|---|---|
| close | M012, M023, M044, M009, M035, M048 | locus or class measure is bounded (univariate hypersurface: f ≤ D/p; one solving-degree drop is a constant; a split-node chain of about n/m nodes replaces the m-ary relation; the stated c < 1/2 lies outside the screen budget) |
| sieve-pricing only | M014, M054, M077, M050, M076, M067 | gain ≤ 1/f with f constant or poly-log; M067 is undecided by this ceiling under the R8 normalisation and is left to its own Poisson-tail falsifier |
| unaffected by the R ceiling | M016, M053, M091 | not a target-class mechanism (membership design, matrix columns, a distinguisher feature) |
| not scored | M085, M080, M029 | distinguishers on input pairs, or a screen that is not O(1) (BKZ per candidate) |

Six closures are recommended; none of the eighteen is placed above the ceiling.
M023's own `revisit_when` clause on the atlas card
`LEAD-ICB-multi-target-and-portfolio` is the one card edit the scoring touches
(the two windows it needs, c < 1/2 − 1/m and c ≥ 1/m, are disjoint at m = 3 and
m = 4).

## 4. Limits

- Derivation only. The ceiling is an inequality on measure; it does not say
  what any mechanism's actual f is, and a mechanism's own falsifier still
  decides it where the table says "sieve-pricing only".
- The screen budget assumes an O(1) screen. A screen that is cheaper than one
  group operation does not exist in the counted unit; one that is dearer only
  shrinks the window.
- No PDF renderer was available to the lead agent; the synthesis step rendered
  `REPORT.pdf` from this file with Typst in a scratch virtualenv.

## 5. Reading

Within the §2 arithmetic, target conditioning cannot carry an index-calculus
attack over F_p past rho at m = 3, and at m = 4 it has exactly one boundary
point to stand on. Every heavy-tail mechanism in the catalogue is either
already priced by a sieve or bounded by the measure of its own locus.
