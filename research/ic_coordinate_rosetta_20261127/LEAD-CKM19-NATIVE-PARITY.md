# LEAD-CKM19-NATIVE-PARITY — native model parity under complete IC cost

**Status:** `open_next_step` (proposal only; no run has been made).
**Lane:** S59 coordinate Rosetta, draft PR #5260.
**Evidence status:** the literature assessment and every predicted advantage in
this file are `[pcc:unverified]`. This file reports no new measurement and no
security reduction.

## Decision question

On one elliptic curve and one prime-order subgroup at a time, does the
even/odd support structure of a native twisted-Edwards `y` summation system,
combined with a symmetric presentation and a parity-aware monomial order,
reduce **complete single-target index-calculus cost** by more than 10% against
the best matched native Montgomery-`x` or short-Weierstrass-`x` arm?

This is a constant-factor lead. It is not an exponent claim. Every result must
also be reported as a ratio to matched `rho.negation`; a win among
index-calculus arms is not a win over rho.

## Why the lead is open

Cheng, Kodera and Miyaji (CKM19) compare summation-polynomial systems for
isomorphic Hessian, Montgomery, twisted-Edwards and Weierstrass forms over
optimal extension fields. Their Section 5.2 and Theorem 1 identify a unified
total-degree parity in the fixed-target twisted-Edwards polynomial: depending
on the number of unknown points, one total-degree parity is absent. Their
experiments report smaller polynomial supports and lower PDP solving cost for
the twisted-Edwards form in the tested cells. `[pcc:unverified]`

That is a real algebraic mechanism, but not a complete attack comparison. The
reported experiment is an extension-field PDP/F4 study, not a complete
ordinary-prime-field ECDLP run with matched relation collection, independent
rank, linear algebra, recovery and rho. CKM19 also warns that a highly
symmetric factor base can reduce decomposition probability or independent
rank. A missing-parity support count alone therefore cannot decide the lead.

The repository's adjacent evidence narrows the expected effect:

- the completed S59 Rosetta run found no significant relation-rate difference
  among its registered coordinate predicates, but it did not build or solve
  native summation-polynomial systems;
- the model/coordinate lead atlas records equal small-arity solving degree for
  Weierstrass and twisted Edwards and classifies model changes as constants;
- the coefficient/model Semaev study found no measured yield change and only
  a 1.3% Montgomery root-finder cost difference on its side curve;
- the existing symmetrization note records a genuine small-system Macaulay
  reduction, while explicitly leaving complete cost versus `rho.negation`
  open; and
- the torsion-symmetry study shows why a smaller quotient or support can be
  erased by lower yield and rank.

Those results do not test the joint mechanism here: native Edwards `y`, the
CKM19 parity restriction, symmetric variables, and a parity-aware F4 order on
the same admitted prime-field instances.

## Competing hypotheses

- **H0 — presentation-only:** the absent parity reduces symbolic support, but
  the complete cost ratio is at least 0.90 at one or more rungs.
- **H1 — solver constant:** the parity-aware twisted-Edwards arm reduces F4
  rows/columns and complete cost by more than 10% at all three rungs, without
  losing yield or independent rank.
- **H2 — rank/yield repayment:** the solver gets cheaper, but extra candidates,
  dependent rows, exceptional-map handling or recovery erase the gain.
- **H3 — broader symmetry, not parity:** symmetrization helps every native
  model similarly; the Edwards parity-aware arm is not more than 10% cheaper
  than the identical Edwards system under ordinary grevlex.

H1 is the only promoting outcome. H0, H2 and H3 leave useful diagnostics but
do not count as success.

## Frozen comparison core

The eventual experiment protocol must preserve all five requirements below.
They may be made more specific before the first run, but not weakened after
outcomes are visible.

1. **Same mathematical instance.** Within each rung, every arm uses the same
   curve, prime-order subgroup, generator, ordered target list, candidate
   relation stream, stopping rule and factor-base cardinality. Exact
   birational maps and inverses bind all coordinates to one registered curve.
2. **Native systems.** The three model coordinates are twisted-Edwards `y`,
   Montgomery `x`, and short-Weierstrass `x`. Each polynomial is derived from
   that model's native group law; converting all arms to the Weierstrass
   polynomial and merely renaming variables is disallowed.
3. **Arity and presentation.** Use `m = 3` and `m = 4`, where `m` is the number
   of unknown factor-base points in `P_1 + ... + P_m = T`. Run direct and
   elementary-symmetric presentations. Run a parity-aware,
   grevlex-preserving monomial layout and an identical-polynomial ordinary
   layout control; the latter is mandatory for attributing a gain to layout
   rather than to the polynomial generator.
4. **Measurements.** Record polynomial support, solving degree, peak and
   cumulative Macaulay columns, counted F4 work, checked relation yield,
   independent rank, and complete cost. Report every complete arm against the
   same matched `rho.negation` record.
5. **Promotion gate.** Missing terms, lower degree, fewer columns or a shorter
   isolated solve never promote the lead. The candidate's complete cost must
   be more than 10% below the matched reference on each of three increasing
   rungs under the simultaneous uncertainty rule below.

## Instance and factor-base admission

The planned public toy ladder is `fp16`, `fp20`, `fp24`. Before any affected
measurement, a fixture manifest must freeze one exact eligible curve per rung,
including `p`, all three equations, map formulae, exceptional sets, group
order, subgroup order `r`, cofactor, generator, target scalars and canonical
IDs. If an exact eligible registered fixture is unavailable, a deterministic
search rule and seed must be frozen and the first admissible result retained.

Each curve must admit all three forms over the same base field. The subgroup
must avoid map exceptions, and every exceptional point encountered in a
candidate relation must be counted rather than silently resampled. A map
round-trip, subgroup-membership and group-law commutation certificate is
required for every retained point.

The primary factor base is a single registered set of `+-` point orbits in the
subgroup, transported through all three maps. Its represented point
cardinality—not the number of raw coordinate keys—is fixed across arms. A
secondary native-window panel may select model-native bases of the same
represented cardinality, but it is descriptive and cannot replace the
transported-base primary gate.

Targets, candidate scalars and seeds are paired across arms. Include A/A
repeats, planted decomposable targets, certified unsatisfiable systems where
available, map-exception fixtures, and a random invertible coordinate-change
null.

## Arm matrix

For each rung and each `m in {3,4}`, run the following core matrix:

| dimension | values |
| --- | --- |
| native model | Edwards `y`; Montgomery `x`; short-Weierstrass `x` |
| presentation | direct; elementary-symmetric |
| monomial layout | ordinary grevlex; parity-aware, grevlex-preserving F4 layout |

The requested parity-aware monomial ordering is implemented at the F4 layout
layer. The parity-aware arm keeps grevlex as the mathematical term order and
preserves the same leading-monomial decisions as the control. It changes
physical monomial enumeration, F4 row/column layout and batch scheduling by
the exact `Z/2` grading. This avoids calling a parity-first comparator a
monomial order: such a comparator is not automatically compatible with
multiplication. The arm may omit a parity block only when an exact symbolic
audit establishes that the block is zero after target specialization and
factor-base constraints. Applying the layout to Montgomery and Weierstrass is
a null/control, not a license to delete their nonzero terms. If a different
mathematical term order is proposed, an admissibility checker and an
identical-order control become blocking gates.

Both presentations must describe the same relation set under exact checks. The
symmetric arm must reconstruct the unordered coordinate multiset, lift every
coordinate to subgroup points, enumerate any sign/fibre ambiguity, and check
the sum against `T`. Duplicates and non-rational lifts are charged.

## Required outputs

### Algebra and solver

For the native generator and final factor-base system, record:

- total degree, per-variable multidegree, equation/unknown count, nonzero
  monomials, coefficient bytes and even/odd total-degree histograms;
- the support before and after target specialization, factor-base constraints
  and symmetrization;
- first-fall degree where available, raw maximum step degree, corrected
  solving degree (steps with pair reductions), and the exact convention used;
- peak Macaulay columns, cumulative generated columns, nonzero entries, peak
  rows, pivot count and peak resident bytes; and
- F4 pairs selected/reduced, reducers, row reductions, field
  multiply/add/invert counts, rebuilds, wall time and termination reason.

The same solver version, optimization budget, timeout and memory cap apply to
all arms. Retain machine-readable traces. An external engine replay (for
example msolve, Magma when licensed, or Singular/OpenF4) is required before a
solver-cost claim is promoted.

### Relation and complete attack

Record candidates attempted, systems completed, checked decompositions,
duplicate/sign-equivalent rows, accepted relations, rank increments, rank
profile, rows needed for full rank and overshoot. Relation yield is measured
per candidate and per counted group/field work, not just per successful PDP.

Complete cost includes fixture/map setup, factor-base construction, relation
generation, every failed solver call, relation verification, sparse matrix
construction, rank completion, linear algebra, target descent/recovery and
final discrete-log verification. Report counted cost `S`, peak memory and
admitted isolated wall time. The matched rho arm uses the identical subgroup,
target set, worker count, memory envelope and negation folding.

## Statistical decision rule

For each rung, let `C*` be the complete cost of the candidate
twisted-Edwards-`y` + symmetric + parity-aware arm. Let `C_best` be the lowest
complete cost among all preregistered non-candidate model/presentation/order
arms that passed every correctness and completion gate.

H1 passes only if all of the following hold:

- at `fp16`, `fp20` and `fp24`, the simultaneous 95% upper confidence bound
  for `C* / C_best` is strictly below `0.90`;
- the corresponding Edwards identical-system parity-aware-layout/ordinary-
  layout comparison also has an upper bound below `0.90`, so the ordering
  lever is attributable;
- every candidate target completes and passes the final identity check, with
  no worse relation yield or independent-rank completion outside the
  preregistered equivalence margin; and
- the result survives independent replay from retained records.

Multiplicity across rungs and the model/presentation family must be controlled
by a frozen simultaneous-bootstrap or max-statistic procedure. A timeout,
missing complete row, rank failure, post-hoc rung substitution or confidence
interval crossing `0.90` is not a pass. The report must show `C* /
C_rho.negation` at every rung; a value above one is stated plainly.

## Execution stages and stop rules

1. **Symbolic qualification:** derive all six native `m` generators, check
   group-law identities on exhaustive small fields, reproduce CKM19's parity
   pattern for Edwards, and establish direct/symmetric relation-set equality on
   the controls.
2. **Solver qualification:** run paired planted and unsatisfiable systems,
   validate traces, ordering controls and external-engine replay.
3. **Relation qualification:** collect until the frozen row target, checking
   every relation and rank increment.
4. **Complete ladder:** run all complete arms and matched rho at the three
   frozen rungs under admitted timing.

Stop before the paid/isolated stage if the Edwards parity pattern does not
survive final-system construction, if any native generator disagrees with the
group law, if direct and symmetric arms produce different checked relation
sets, or if the instrument cannot count exceptions and failed solver calls. A
stage stop is a falsification or an instrumentation blocker, never a successful
constant-factor result.

## Sources and prior evidence

- Chen-Mou Cheng, Kenta Kodera and Atsuko Miyaji, [“Differences among
  Summation Polynomials over Various Forms of Elliptic
  Curves”](https://doi.org/10.1587/transfun.E102.A.1061), IEICE Transactions on
  Fundamentals E102-A(9), 1061–1071 (2019). Assessment used the official
  metadata and the user-supplied screenshots of pages 1061–1069; author
  biographies were not needed.
- `research/ic_coordinate_rosetta_20261127/{README.md,REPORT.md}` — completed
  S59 coordinate predicate and EFD chart comparison.
- `research/lead_sweep_20261110/LEAD-EXO-curve-model-representation/PROTOCOL.md`
  — degree-two coordinate obstruction and limits of the earlier toy test.
- `research/coeff_model_semaev_20261109/report/REPORT.md` — prior
  coefficient/model yield and counted-cost study.
- `research/pdp_literature_leads_20261022/leads/LEAD-03.md` — torsion symmetry,
  yield and rank costs.
- Workspace `docs/notes/summation-symmetrization-20261123/README.md` and
  `docs/plans/2026-11-20-ten-ecdlp-blind-spots.md`, Workstream 6 — the existing
  symmetric-system measurement and same-instance model-comparison rules.

The shared research-KB connector and wrapper were unavailable when this lead
was opened. The repository/lead-atlas read and official-source lookup are a
bounded fallback; no novelty or absence claim is inferred from them.
