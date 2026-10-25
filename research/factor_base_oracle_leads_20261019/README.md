# Three factor-base leads implemented and tested natively

The standalone `factor-base-sweep compare-leads` command now compares
asymmetric chart branches, guarded selector membership and a checked
torsion quotient. The frozen six-case panel records 472 trial/arm rows,
143 original-curve point witnesses and 355 algebraic roots. Separate native
field/group arithmetic checks every returned witness and root, exact
finite-inventory support, cofactor-projected training rank and lifting.
It runs on the same host; it is not an independent-host certificate.

Executed source/protocol/input lock:
`aaf9e246a3af4be371d7089dc86fc32c61c2f5ac`. The exact plan, copied inputs,
model identities, source hashes, task card, receipt and director assessment
are retained. All six cases completed as designed; individual resource
stops remain in the results. No research retry or budget increase.

## What changed our view

- **Asymmetric branching is the strongest follow-up.** On the complete
  degree-13 B033+B027 graft it returned all four conditioned positive
  controls with 338,374 counted F2 reduction multiplications. Product
  membership and its A/A repeat, and selector membership, hit resource
  limits on all four. On the complete degree-7 union all three returned
  all four controls: asymmetric used 5,950 reduction multiplications,
  product 590,196 and selector 2,486,760. All attempted A+A, A+B and B+B
  branches before a witness are charged. These are component observations,
  not total-cost speedups.
- **The tested selector encoding did not improve successful reduction
  work.** In each of the three chart panels where product and selector
  completed all controls, selector used more reduction multiplications.
  Both were limited on the full degree-13 graft. This is a result about
  this exact guarded encoding, variable basis and solver, not a general
  impossibility statement about selector methods.
- **The quotient mechanism and lifting work on both exact toy models.**
  Quotient and direct arms return the same four positive controls. Counted
  reduction multiplications are 1,038 versus 32,430 on degree7 and 1,288
  versus 64,128 on degree13. Every lifted result is checked in the original
  eight-point inventory. The four-point quotient base C={±G,±[2]G} has
  deliberately known construction scalars and only eight supported targets;
  it tests the mechanism, not an optimized unknown-log factor base.

The small chart controls also improve the asymmetric component count:
954 versus product 3,580 on degree7 and 1,878 versus 7,890 on degree13.
Completed reference repeats match their original outcomes and counters;
the repeated full-graft reference is also resource-limited, with incomplete
counters.
The maximal *successful observed procedural degree* is 2 or 3 depending
on the case and arm. F4 includes substitution/root enumeration. This is
neither certified degree of regularity nor certified refutation degree.

## Fresh probes and independent relations

Each case has 12 training targets and 12 disjoint holdout targets, shared
across its arms. Controls are selected from supported targets and never
count as random yield or rank. Bases themselves came from prior studies.

| Case | Training verified / 12 | Holdout verified / 12 | Training rank | Qualification |
|:--|:--|:--|:--|:--|
| n7-small | 3 for every chart arm | 0 for every chart arm | 3 for every arm | Same outcomes, different counted work |
| n7-full | 9 for every chart arm | 7 for every chart arm | 6 for every arm | Asymmetric changes work, not observed yield or rank |
| n13-small | 0 for every chart arm | 0 for every chart arm | 0 | Positive controls only provide successful cases |
| n13-graft | 0 for every chart arm | 0 for every chart arm | 0 | Asymmetric completes all random absences; product and selector mostly resource-limited |
| n7-torsion | 2 for both arms | 2 for both arms | 2 for both arms | Same point outcomes on direct and quotient paths |
| n13-torsion | 0 for both arms | 1 for both arms | 0 | Very small support, no rank improvement |

On n13-graft, product has 12 limited training calls and 11 limited holdout
calls plus one completed absence. Selector has 12 limited calls in each
random phase. Asymmetric has 12 checked absences in each phase. No arm
found a random success in this case. A limit is not a refutation, and
missing reduction counters are not zero work. The full native
[analysis.json](analysis.json) and report preserve every arm/phase count.

Training columns are distinct nonzero signed [h] projections. Points that
differ by small torsion cannot create extra independent unknowns. Original
and projected inventory sizes are recorded separately. No scalar logarithm
is solved; a row rank does not establish completed linear algebra or a
whole index-calculus algorithm.

## Run the installed native CLI

```sh
factor-base-sweep compare-leads \
  --plan research/factor_base_oracle_leads_20261019/plan.json \
  --out runs/oracle-leads
factor-base-sweep verify --run runs/oracle-leads
```

The executable launches its own native workers and emits line-by-line
encoding, solver-matrix, witness and rank events. No Cargo launcher is used
at runtime. Outputs are write-once; use a new output directory. Preserve
old executable bytes for existing byte-bound sweep resumes.

The native root code is
[`leads.rs`](../../src/bin/factor_base_sweep/leads.rs) and
[`leads_algebra.rs`](../../src/bin/factor_base_sweep/leads_algebra.rs).
The [independent audit crate](audit/src/main.rs) uses separate polynomial
field/group arithmetic, inherited from the earlier chart audit. It checks
the emitted roots and actual points without rerunning F4. Its arithmetic
is independent of the production backend, not a newly independent host
or second solver implementation.

## Evidence and remaining obligations

- [Frozen protocol](PROTOCOL.md), [plan](plan.json), [source hashes](SOURCE_SHA256SUMS).
- [Results report](reports/run-20261019/report.pdf), [report source](report.typ),
  [rendering source](render.py), and SVG/150dpi PNG plots.
- [Validation](VALIDATION.md), [pricing coverage](PRICING.md),
  [formalization status](FORMALIZATION.md), [literature](LITERATURE.md),
  [setup observations](SETUP_OBSERVATIONS.md), and card/receipt/assessment.
- `runs.tar.xz` and its sidecar contain the sealed experiment, independent
  audit, exact commands, original failures and validation receipts.
  `checks.tar.xz` holds subsequent extraction/CI replay/publication checks.

All complete native cost totals and total-cost rankings remain unknown at
the symmetric 0/0.1/1 band. Symbolic assembly, root search, map/setup,
verification, rank and allocation costs are not completely emitted or
calibrated for any arm. Matched Magma/msolve/Singular comparison is unrun;
the executables were not found on PATH at setup. There is no fastest
implementation, accepted speedup, rho comparison, statistical significance,
certified regularity,83/130-bit transfer or end-to-end claim.

The next discriminating work is complete accounting and matched fresh
targets for the asymmetric path, and a larger useful quotient inventory
with algebraically cheap membership and fully priced lifting. The tested
selector form is a retained negative lead. These are recommendations,
not new executions or promotions of any draft XP backlog card.

Existing canonical scoreboard, leaderboard and browser records are
unchanged: this stage diagnostic has no admitted IC/rho ratio, newly
priced phase, cross-method session, curve registration or pipeline verdict.
The browsing report and graph copies are in
[`benchmarks/20261019-factor-base-oracle-leads`](../../benchmarks/20261019-factor-base-oracle-leads/).
The earlier chart/shape studies, frozen evidence, registry/aliases/store
and all three draft follow-up cards remain unchanged and unrun.
