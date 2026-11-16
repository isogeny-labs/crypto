# Matched trace-line C1 pipeline measurement is unavailable

- **Status:** open
- **Kind:** implementation gap; access/tool limit; unpriced-work gap
- **Affected work:** [study](../research/trace_c1_2lp_20261027/README.md), PR #2686 follow-up
- **Next action owner:** native IC research / authorised Iso Lab runner
- **Last updated:** 2026-10-27

## Problem

We cannot establish a fully priced C1 2LP improvement for the trace-selected
base because the algebraic x-line oracle, its complete pricing and an
authorised execution/checkpoint route are missing from this implementation.
The required experiment remains open; the bounded MITM reference is not a
substitute for algebraic C1 trial-cost measurement.

## Evidence and uncertainty

- Observed: native trace-line construction and an explicit-basis extension of
  the existing four-summand 2LP collector are implemented. Its GAE ledger marks
  native solver/lookup/LP/LA work uncharged. Prospective specs compare the same
  public targets; no experiment session was launched.
- Observed: `conductor check` returns command not found; `.conductor/WORKFLOW.md`
  is absent from the fetched tree. No reservation was obtained. No conflict
  bypass or task metadata was fabricated; the change uses an isolated feature
  branch and records the coordination gap.
- Observed: no local AWS CLI or Iso Lab MCP; GitHub connector exposes no new
  workflow-dispatch operation. The checkpoint gate fails; no upload token is
  configured. No credentials were copied or permissions expanded.
- Derived/proved: no new mathematical result. Upstream trace-line applicability
  is conditional on its stated field/trace hypotheses. No performance or
  security conclusion follows from unavailable measurements.
- Missing capability: algebraic C1 x=u*v coefficient path, full pricing,
  exclusive target-phase accounting, frozen admitted image, L2 receipts and
  independent-host replay. External fastest-tool comparison is also pending.
- Unknown: whether trial cost changes enough to cancel the relation gain,
  the cost-ratio variance, and the eventual end-to-end result.

## What would unblock it

- [ ] Both algebraic C1 arms reproduce the exact toy decomposition reference.
- [ ] Every requested setup/online phase has counted and calibrated costs;
  complete total/S and matched rho bands are independently recomputable.
- [ ] Native protocol validation/freeze, exact curve registry/EC1 identities,
  source/binary/image digests, blinded analysis and positive controls are pinned.
- [ ] Existing authorised Iso Lab submission and S3 checkpoint access work;
  bounded job receives L2/A/A admission and payload hashes verify.
- [ ] Full transcript and scalar replay pass on a second host class; report,
  export, independent review and validation receipts accompany any claim.

These conditions enable measurement; they do not guarantee a gain or imply
that the named c2pnb curves can be solved practically.

## Next discriminating step

Implement the shifted x-line coefficient path for the C1 algebraic oracle and
cross-check its complete toy solution sets against exact enumeration. Use the
same standard/selected bases, public targets and bounds in the prospective
[protocol](../research/trace_c1_2lp_20261027/PROTOCOL.md). If the sets disagree,
fix the oracle and preserve the failing fixture. If equal, finish the native
pricing and freeze an authorised measurement image. Infrastructure failure
leaves the experiment blocked; no local benchmark substitute is authorised.
Recheck when either implementation or authenticated execution access changes.

## Resolution and history

2026-10-27: opened with the implementation PR. Unresolved. No C1 result or
end-to-end saving is established by completing or merging this record.
