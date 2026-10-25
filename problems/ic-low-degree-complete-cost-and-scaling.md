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
  construction and cubic-split work are incompletely counted. Legacy raw S
  includes a wall-priced solver term and is inadmissible as a counted L0
  comparison. [Pricing disclosure](../research/ic_algebraic_crossover_20261023/PIPELINE.md).
- **Derived or proved:** summand exchange has an exact polynomial/permutation
  certificate, IDC1ha27a77e72057f9cb, scoped to degree verdicts, not costs.
- **Suspected explanation:** additional closure work can outweigh smaller
  product spaces. The fixed finite observations do not determine its growth.
- **Missing capability:** complete native construction/split accounting,
  validated larger-cell closure and guess minima, same-input external CAS
  comparison, and measured hardware calibration. Local matrices have a
  12-GiB/900-second envelope; paid launcher access is not yet verified.
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
