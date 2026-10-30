# N83 index-calculus regularity remains unresolved

- **Status:** open
- **Kind:** resource limit; presentation limitation; instrumentation gap
- **Affected work:** [N83 m2–m9 study](../research/n83_ic_degree_sweep_20261027/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-29

## Problem

For the current fixed-base n83 request, m2 highest-form regularities are certified exactly, while m3..9 binary/Koblitz values now have matching original-top certificates under the written overlap proof (full Lean algebraic formalization and independent review pending) and the prime top-form presentation is positive-dimensional for m>=3. Intermediate solving degree and an elite-base improvement remain unestablished.

The current scope is all Koblitz, general binary and prime cells at m=2..9 with one fixed ell27 factor base per curve, one planted and one natural full-curve target per cell. The earlier ell=floor(83/m) sweep below remains historical evidence. Top-form regularity and affine final-basis degree are different computations; maximum intermediate solving degree is not instrumented. Retain the original requested measurements as unresolved where no finite result is certified.

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

The user's follow-up permits first or last fall identification, retaining all families and m=2..9 and allowing n51/53. R3 completed on the retained n83 inputs: 32 explicit binary/Koblitz witnesses certify first fall 2, while 16 prime coefficient-minor certificates exclude falls through degree 4 only. All 48 fresh-process replays passed; [R3 evidence](../research/n83_ic_degree_sweep_20261027/fall/experiment.export.json).

R4 subsequently validated all 16 prime standard leading-syzygy first-fall certificates: floor(ell/2)+4, giving 24,17,14,12,10,9,9,8 for m=2..9. The lower bound is a written Lefschetz/filtered-Koszul derivation, supported by five exact small-matrix controls; target witnesses, hypotheses and original presentations are checked natively. These are not full n83 matrix-rank measurements. [Combined report](../research/n83_ic_degree_sweep_20261027/fall/prime/reports/report.md), [R4 evidence](../research/n83_ic_degree_sweep_20261027/fall/prime/experiment.export.json), and [formal proof map](../research/n83_ic_degree_sweep_20261027/fall/prime/PROOF_MAP.md). The complete Lean theorem and independent scientific review remain open. Earliest NEW affine prime fall is also unresolved: a leading-form nontrivial syzygy and a nonzero affine degree-lowering product do not by themselves prove novelty in the lower closed space. This completes the standard first-fall grid but does not resolve this regularity problem or identify last fall.

## Fixed ell27 follow-up, 2026-10-28

The user subsequently requested one shared 2^27-candidate factor base per curve
for every m2..9. Complete point datasets were generated and pinned by manifest.
The [fixed-base regularity rerun](../research/n83_fixed_degree_sweep_20261028/README.md)
executed all 96 attempts against this new domain, checking every planted point
against actual stored records. Observed:4 completed affine computations, 63
whole-process time limits, 29 memory exits, zero completed top-form regularity
values. Koblitz and general binary m2 have final affine basis degrees 2/0 for
planted/natural targets; these are not regularity measurements. See the
[export](../research/n83_fixed_degree_sweep_20261028/experiment.export.json)
and [report](../research/n83_fixed_degree_sweep_20261028/reports/run-20261028T0325Z/report.md).

The latest requested factor-base size is fixed ell27. Future work on this
request must retain those manifests and all m2..9 cells. The next discriminating
step above applies to these retained fixed-domain inputs as well: distinguish
input expansion from solver work and validate an appropriate finite-degree
presentation/backend. No first-fall substitution or timeout-derived bound
resolves the remaining regularity measurements. The earlier balanced-domain
results and protocol remain historical evidence, unchanged.


## Fixed-base certificates and continuation, 2026-10-29

Later native certificates resolve original highest-form regularity at m=2:
Koblitz/general binary exactly 28 and prime exactly 42, on both retained
original target kinds and the paired baseline/spike40 panels. For binary and
Koblitz m=3..9, certified intervals are respectively [111,138], [194,221],
[277,331], [360,414], [443,524], [526,607] and [609,717]. Exact higher-m
values remain unresolved. Prime m>=3 retains the positive-dimensional
highest-form quotient in this presentation. This is not a statement of
infinite Castelnuovo–Mumford regularity.

The [48-row original fixed-base view](../research/n83_candidate_pdp_20261028/FIXED_BASE_REGULARITY.json)
joins those existing certificates only after matching exact system and point
manifest hashes. [DATA_INDEX](../research/n83_candidate_pdp_20261028/DATA_INDEX.json)
preserves the original attempts, all later certificates, formats and storage
records. The earlier zero-completion statement above describes the bounded
solver attempts, not these later exact certificates.

The twelve retained m3 V3 prefixes have now been continued to complete V3
spaces, with producer, fresh and S3-restored replay checks. Their LFD>=3
witnesses were already in the parent prefixes. No global LFD upper bound or
exact LFD follows from V3 completion. All earlier data and capped traces remain.
See the [continuation publication record](../research/n83_candidate_pdp_20261028/evidence/degree3-continuation-publication.json).

The next regularity obligation is an exact certificate for the overlapping
binary m3 chain on the unchanged polynomial presentation. Current disjoint
component bounds do not resolve that overlap. For higher LFD witnesses, the
complete V3 spaces can now support strict nonmembership tests; further closure
work needs a new frozen protocol and preserved parent/suffix data. A global
upper certificate is still a separate obligation. Neither the equal m2
regularities nor the higher-m bounds establish an elite base; held-out matched
targets and unchanged preprocessing remain required. This problem stays open.


## Fixed ell27 overlap certificates, 2026-10-29

All84 retained original/baseline/spike40 Koblitz/binary m3..9 systems passed
producer, fresh-process and S3-restored replay. Exact original-top degrees are
111,194,277,360,443,526,609. The new source-bound overlap proof and full small
Hilbert controls are in [CHAIN_EXACT_DESIGN.md](../research/n83_candidate_pdp_20261028/CHAIN_EXACT_DESIGN.md);
[publication checks](../research/n83_candidate_pdp_20261028/evidence/chain-exact-publication.json)
and [the48-row fixed-base view](../research/n83_candidate_pdp_20261028/FIXED_BASE_REGULARITY_V2.json)
retain sources and manifests. Prior interval and failed solver records remain.

The binary regularity interval gap is closed in the written/native certificate
method; independent scientific review and the full Lean bridge remain open.
Prime positive-dimensional presentations, intermediate solving degree, global
FFD/LFD and elite-base improvement are still unresolved. A change only between
the tested linear selector bases does not reduce original-top regularity. Next
use the saved completeV3 for strict higher-fall tests, or freeze a genuinely
different selector/presentation arm with explicit semantics and matched inputs.


## Fixed ell27 strict degree-four falls, 2026-10-29

The twelve retained m3 Koblitz/binary original/baseline/spike40 systems now
have LFD>=4 certificates. All 249 candidates per case are degree<=3 consequences
with degree<=4 derivations outside the complete saved V3. Production, fresh
replay and S3-restored replay pass for all twelve cases. See the
[construction](../research/n83_candidate_pdp_20261028/DEGREE4_WITNESS_DESIGN.md),
[native results](../research/n83_candidate_pdp_20261028/evidence/degree4-witness/summary.json)
and [publication checks](../research/n83_candidate_pdp_20261028/evidence/degree4-witness-publication.json).

This strengthens the previous lower3 evidence without altering targets, points,
input equations, parent traces or regularity values. Exact LFD, its global upper
bound and complete V4 remain unknown. A proposed higher-fall witness must prove
nonmembership in full V4 or furnish a valid separating certificate; failure of
membership in a restricted V4 closure is insufficient. Prime presentations,
held-out elite-base selection and independent scientific review remain open.


## Fixed ell27 V4 consequence propagation, 2026-10-29

12/12 producer/fresh replays and 12/12 S3-restored replays passed. The native results record 2988 new independent rows and 2988 operations. Global upper certificates: 0; exact LFD certificates: 0.

Lower4 is inherited from the strict witness dataset. A restricted queue stall or cap gives no higher-fall bound and no global upper. Unrestricted V4 was not computed. Counts do not rank factor-base quality; independent scientific review remains pending.

See the [native results](../research/n83_candidate_pdp_20261028/evidence/degree4-propagation/summary.json),
[upper-certificate argument](../research/n83_candidate_pdp_20261028/DEGREE4_PROPAGATION_DESIGN.md)
and [restored checks](../research/n83_candidate_pdp_20261028/evidence/degree4-propagation-publication.json).
Only a verified unit or exhaustive affine ideal equality can certify global
upper4 under this protocol. Prime/higher-m global LFD, independent review and
held-out matched elite-base selection remain separate unresolved requirements.


## Fixed ell27 hybrid quartic closure, 2026-10-29

12/12 producer/fresh replays and 12/12 S3-restored replays passed. The native results record 50122 new independent rows and 50134 product attempts; 0 cases completed the V4 normal-form closure. Global upper certificates: 0; exact LFD certificates: 0.

Lower4 is inherited from the strict witness dataset. Complete V4 normal forms and a global LFD upper certificate are distinct claims. A cap gives no full-closure conclusion or higher-fall bound. Counts do not rank factor-base quality; independent scientific review remains pending.

See the [native results](../research/n83_candidate_pdp_20261028/evidence/degree4-quartic/summary.json),
[upper-certificate argument](../research/n83_candidate_pdp_20261028/DEGREE4_QUARTIC_DESIGN.md)
and [restored checks](../research/n83_candidate_pdp_20261028/evidence/degree4-quartic-publication.json).
Only a verified unit or exhaustive affine ideal equality can certify global
upper4 under this protocol. Prime/higher-m global LFD, independent review and
held-out matched elite-base selection remain separate unresolved requirements.


## Prime m2 first-fall certificates,2026-10-30

All six retained prime m2 original/baseline/spike40 planted/natural systems have highest-form **FFD = 17** and a strict filtered witness giving **LFD >= 17**. All six producer, fresh-process and S3-restored checks passed. Global upper and exact LFD remain unknown.

These are source-bound certificates using native exact arithmetic and a written rank/Koszul/filtered-space argument; independent scientific review and full algebraic formalization remain pending. The original-top regularity is still42. No elite-base or performance gain is established.

See [native results](../research/n83_candidate_pdp_20261028/evidence/prime-first-fall/summary.json)
and [the source-bound proof argument](../research/n83_candidate_pdp_20261028/PRIME_FIRST_FALL_DESIGN.md).
Prime global LFD upper/exact,higher-m FFD/LFD,the pending binary quartic closure,
and independent held-out elite-base selection remain unresolved.
