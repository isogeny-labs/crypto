# N83 balanced index-calculus regularity remains unresolved

- **Status:** open
- **Kind:** resource limit; presentation limitation; instrumentation gap
- **Affected work:** [N83 m2–m9 study](../research/n83_ic_degree_sweep_20261027/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-27

## Problem

The requested finite degree-of-regularity measurements are not established by the bounded n83 sweep, and the chosen prime top-form presentation is positive-dimensional for m>=3.

The scope is all Koblitz, general binary and prime cells at m=2..9 with ell=floor(83/m), one planted and one natural full-curve target per cell. Top-form regularity and affine final-basis degree are different computations; maximum intermediate solving degree is not instrumented. Retain the original requested measurements as unresolved where no finite result is certified.

## Evidence and uncertainty

- **Observed:** All 96 attempts ran: 53 time-limit exits, 43 explicit memory-limit exits, no completed degree measurements. The [frozen protocol](../research/n83_ic_degree_sweep_20261027/protocol-v2.json) uses 40-second whole-process limits and 3 GiB virtual memory per worker. Every raw status, target, script and trace is retained in the [evidence bundle](../research/n83_ic_degree_sweep_20261027/evidence/storage.json); all 533 manifest entries passed SHA-256 verification. Passing two-variable controls do not establish tractability of n83 systems.
- **Derived:** [The prime top ideal](../research/n83_ic_degree_sweep_20261027/ALGEBRA.md) has dimension ceil((m-2)/2) for m=3..9. This is a representation-specific statement, not an IC impossibility theorem.
- **Missing capability:** A suitable finite-dimensional prime regularity presentation, enough solver resources or a more efficient verified backend for finite-degree cells, and exact instrumentation of intermediate solving degree.
- **Not established:** No finite degree is inferred from a timeout, first-fall behavior or a final basis. No full DLP result, speedup, asymptotic claim or comparison of field families follows.

## What would unblock it

- [ ] Complete and independently verify the intended degree quantity on every finite-degree n83/m2–m9 cell without downscaling or substituting estimates.
- [ ] Specify a prime presentation with the intended regularity semantics, or explicitly accept positive-dimensional top-form quotients as a distinct result.
- [ ] If practical solving degree is intended, instrument the solver's intermediate degrees and validate that instrument on exact controls.

These conditions would resolve the measurement question for the stated presentation; complete relation collection, rank, descent and matched rho accounting would still be separate work.

## Next discriminating step

Freeze a follow-up protocol that separates input expansion from standard-basis work and checks a suitable native/F4 backend on the exact preserved inputs. Keep the current complete failure grid and the same balanced domains. Use explicit solver-work limits and checkpoint behavior. A successful certified degree fills only its own cell; a further timeout remains unresolved. Recheck when the revised encoding or instrumentation is available. This is a planned step, not a completed follow-up run.

## Resolution and history

Unresolved. The v1 launch failed before target data; v2 corrects permissions and preserves the original mathematical inputs and limits. Merging the runner and evidence does not resolve the outstanding measurements.

The user's follow-up permits first or last fall identification, retaining all families and m=2..9 and allowing n51/53. R3 completed on the retained n83 inputs: 32 explicit binary/Koblitz witnesses certify first fall 2, while 16 prime coefficient-minor certificates exclude falls through degree 4 only. All 48 fresh-process replays passed; [R3 evidence](../research/n83_ic_degree_sweep_20261027/fall/experiment.export.json). This progress does not resolve regularity or last fall. Exact prime first/last values are the remaining active-goal gap. The next step is a higher-degree prime certificate with an explicit lower-degree exclusion, or the authorized smaller-n fallback. A leading-form nontrivial syzygy and an earliest new affine degree fall must be identified separately if their definitions differ.
