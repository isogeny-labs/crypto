# Low-degree decomposition has no complete cost or scaling qualification

- **Status:** `investigating`
- **Kind:** empirical bottleneck, incomplete measurement, resource and tool limits
- **Affected work:** [algebraic IC pursuit](../research/ic_algebraic_crossover_20261023/README.md), requirements A1--B3, C3 and D1
- **Next action owner:** current IC campaign
- **Last updated:** 2026-10-25

## Problem

The observed small solving degrees and symmetric product spaces do not yet
give a qualified complete-decomposition cost or a scaling law: extraction,
guessing, larger systems and all native costs have not been validated together.

The frozen campaign covers the original `(m,n,ell)=(3,22,8)` fixtures,
ell=9 and four-summand continuations, the actual HKY N=58/60 cells, and
approximately N=100. Complete cold one-target IC, with the strongest matched
rho and native-price band at 0, 0.1 and 1 GAE, remains the downstream test.

## Evidence and uncertainty

- **Observed:** three native structural panels each contain 1,152 rows and
  304 completed direct/symmetric pairs. Symmetric solver words are lower in
  263 pairs at closure degree 2, zero at degrees 3 and 4. Independent point
  enumeration reports no mismatch. [Raw-derived report data](../research/ic_algebraic_crossover_20261023/report-data.json)
  and [visual report](../research/ic_algebraic_crossover_20261023/reports/local-structural-20261025/report.pdf)
  retain incomplete cells and the direct solver's input-degree floor.
- **Observed:** all eight balanced three-guess branches on original draw 0
  saturate degree four without refuting; 28 k=0/1/2 attempts hit the local
  column cap. Original fixtures' exact counts and SAT witnesses replay.
- **Observed:** native complete-DLP smoke uses source-pinned v1 adapters;
  construction and cubic-split work are incompletely counted. Completed
  n=13/n=23 S uses pinned word rates; an earlier wall-pricing explanation
  was wrong and has a native reconstruction and explicit correction.
  [Pricing correction](../research/ic_algebraic_crossover_20261023/PIPELINE.md).
- **Derived or proved:** summand exchange has an exact polynomial/permutation
  certificate, IDC1ha27a77e72057f9cb, scoped to degree verdicts, not costs.
- **Observed relevance limit:** original UNSAT draw 5 and SAT draw 4 have no
  y lift on E0; draw 0 does. The original lift audit did not check subgroups. The
  [finite curve audit](../research/ic_algebraic_crossover_20261023/reports/run-fixture-group-audit-20261025/report.pdf)
  qualifies these Boolean fixtures without changing their recorded counts.
- **Completed relevance audit:** exact generated-subgroup enumeration
  resolves every earlier null: 105 of 1,159 target x-values are in the
  chosen reference/twist subgroups, 1,054 outside. All seven original E0
  bases have no points in its largest prime subgroup; draw 6 has none on
  either chosen model. This limits the original ECDLP interpretation.
- **Additional qualified controls:** 2,048 explicit a0/full-degree-b public
  subgroup queries retain every planned draw. Native finite-chain replay
  and independent point arithmetic agree on all rows: 1,591 have Boolean
  roots, 161 have actual signed subgroup decompositions. The frozen
  selection's 55 negatives and 24 positives all saturate without refuting
  at D2/D3; all 158 ranks/verdicts independently replay with HKY. The
  [new reports](../research/ic_algebraic_crossover_20261023/reports/run-qualified-closures-20261025/report.pdf)
  and [native reproduction](../research/ic_algebraic_crossover_20261023/verify-qualified.sh)
  keep quota shortages, empty bases and contract differences explicit.
- **Suspected explanation:** additional closure work can outweigh smaller
  product spaces. The fixed finite observations do not determine its growth.
- **Missing capability:** complete native construction/split accounting,
  validated larger-cell closure and guess minima, same-input external CAS
  comparison, and measured hardware calibration. Local matrices have a
  12-GiB/900-second envelope. The 116 additional balanced ell=8 attempts are
  actively executing at a 200k outer column cap under that envelope. All 44
  same-input Singular correctness checks agree on 22 small cases; equal-output
  fastest-tool performance comparisons remain open. The prescribed paid launcher
  is blocked by an explicit identity-policy denial; no research VM launched.
- **What this evidence does not establish:** a global minimum over all
  variable subsets, bounded last fall degree, a full IC speedup, or a
  security conclusion at a larger curve.

## What would unblock it

- [ ] Replay every branch of the frozen guess families, retaining caps as
  unknown, and complete the ell=9/four-summand and N=58/60/100 panels with
  exact SAT/UNSAT controls and independent fixed-point checks.
- [ ] Count construction, extraction, splitting, lifting and every pipeline
  phase in matched native units, verify every recovered scalar, and provide
  an independent replay, raw-data validator and strongest matched-rho band.
- [ ] Compare the same equations and outputs with the fastest accessible
  external CAS before a fastest-solver claim; qualify hardware separately.

These conditions establish finite comparisons on their recorded inputs.
An asymptotic statement would still need a separate argument.

## Next discriminating step

- **Action:** execute the frozen larger closure and ell=9 shards, then test
  whether a bounded small guess count continues to refute all branches.
- **Inputs and controls:** [PROTOCOL.md](../research/ic_algebraic_crossover_20261023/PROTOCOL.md),
  `inputs/fresh/selection.json`, all four basis families, exact SAT witnesses.
- **Execution limits:** local 12 GiB/900 s; registered remote caps and the
  owner's total additional USD 100 ceiling, including replay. No paid launch
  until launcher access, current price and aggregate commitment are checked.
- **Outcomes:** complete closure yields a finite degree observation; a
  saturated nonrefuting branch rejects that set; a cap or timeout stays
  inconclusive and informs the next storage implementation.
- **Recheck trigger:** new native run receipts or accounting/launcher support.

## Resolution and history

2026-10-25: opened with independent stage panels and complete draw-0
three-guess replay. Unresolved; documentation does not discharge the campaign.
