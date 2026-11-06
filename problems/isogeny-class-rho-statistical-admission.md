# Isogeny-class rho statistical and admission gap

- **Status:** investigating
- **Kind:** implementation gap; empirical inference gap; proof gap
- **Affected work:** crypto #2174, #2186 and #2187;
  [paper review](../papers/isogeny-class-rho/REVIEW-20261019.md)
- **Next action owner:** source/evidence PR maintainers; review work in progress
- **Last updated:** 2026-10-19

## Problem

The historical isogeny-class rho study cannot yet establish its blanket
empirical null because its companion statistics mishandle ties, shared-seed
covariance is unassessed, ancillary flags were omitted from the summary,
and native admission and concrete proof bridges are incomplete.

The workload is the public synthetic four-class study at
`research/isogeny-class-rho/runs/run-20261018T0857Z/`: 200 measured curves,
85,425 recorded solves, small prime/binary/Koblitz instances. The exact
class/curve identities remain in its frozen records. This record neither
changes those identities nor extrapolates to deployed curves.

## Evidence and uncertainty

- **Observed:** immutable companion source
  `a277c5f06469bdb61b3af4a7e6c7df5cde7019d0` returns KS D=1 for identical
  tied samples `[1,1]`; D must be 0. The parent report includes one B1
  generator flag, two P1 and one P0 shift flags, and B1 generator KW p=0.039.
  The paper formerly said no test fired. See the dated review and proof map.
- **Derived or proved:** conditional Lean scalar transport and answer
  reflection do not construct the concrete curve isogeny or couple native
  hash walks. The checked scope and pins are in the proof map.
- **Suspected explanation:** the KS implementation advances only one tied
  ECDF at a time. The historical magnitude of statistical changes is unknown
  until exact producer lineage and corrected native replay are established.
- **Missing capability:** sealed native experiment/control/answer/audit
  records and a qualified host for any admitted performance comparison.
- **What this evidence does not establish:** a replacement p-value, a
  corrected null, detector sensitivity, a wall-time win or a new attack.

## What would unblock it

- [ ] Correct native order/field arithmetic and tie-aware statistics pass
  meaningful independent known-answer and adversarial controls.
- [ ] Bind each historical result to its actual producer revision and raw
  rows; add versioned corrected analysis and shared-seed covariance assessment
  without silently replacing the frozen protocol or historical outputs.
- [ ] Refresh all affected canonical graphs, report PDFs and benchmark
  mirrors from that corrected evidence in the same PR.
- [ ] Complete sealed native experiment admission and concrete proof/
  identity-certificate bridges required by the claimed scope.
- [ ] Obtain all applicable successful CI at the final reviewed head.

These are necessary gates; resolving a source defect alone does not qualify
the historical experiment or prove the broader generic-model claim.

## Next discriminating step

- **Action:** validate the companion source fixes, then perform versioned
  native reanalysis against the exact frozen rows and producer receipts.
- **Inputs and controls:** identical/disjoint tied KS samples, known-answer
  rank-test controls, actual historical per-solve rows, unchanged protocol,
  and explicit shared-seed covariance calculations.
- **Execution limits and stop condition:** source/correctness validation
  first; stop if lineage or required inputs cannot be established. No new
  timing campaign or weakening of isolation tiers is authorized by this record.
- **Outcomes:** corrected analysis may support a bounded null, expose a
  difference or remain inconclusive; each outcome gets its own versioned
  report and matching graphs. Infrastructure failure is not a null result.
- **Recheck trigger:** validated source repair plus exact producer lineage.

## Resolution and history

2026-10-19: paper corrections and formal dependency audit started. Historical
raw evidence and statistics retained. Unresolved.
