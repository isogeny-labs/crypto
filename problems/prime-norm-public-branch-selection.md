# Prime-field norm certificates: public branch selection and total cost

- **Status:** investigating
- **Kind:** construction/proof gap; separate unavailable KB/compute-admission capabilities
- **Affected work:** [norm derivation](../research/prime_norm_rank_20261112/DERIVATION.md), [next finder](../research/prime_norm_rank_20261112/NEXT_FINDER.md), [public-jet report](../research/prime_norm_rank_20261112/public-jet-report/README.md), [actual group bridge](../research/prime_norm_rank_20261112/group-bridge-report/README.md), [membership projection](../research/prime_norm_rank_20261112/membership-elimination-report/README.md)
- **Next action owner:** ongoing research session
- **Last updated:** 2026-11-13

## Problem

A public-input procedure must jointly choose square-root branches over valid
support and recover a bounded nonzero norm pair, with exp(o(log r)) complete
expected bit cost; no such procedure or bound has been established here.

Original scope: general nonsingular E/Fp with p>3, prime subgroup order r,
log p=O(log r), public P,Q, no special weak-curve hypothesis, no precomputation
claim. Conditional polynomial certificates and a Gaussian solve for supplied
branches do not discharge selection. This record preserves that acceptance
criterion; it does not claim nonexistence.

## Evidence and uncertainty

- **Observed, unsigned:** [original finite native artifact](../research/prime_norm_rank_20261112/public-jet-evidence/native-control.json), sourcea5b04b3e5, has289 fixed jet inputs; target-c1 modular lift retains a nonzero exact residual. For the supplied six-point branch assignment, reconstruction has dimension1; flipping one supplied branch gives dimension0. No sign search or timing panel [pcc:unverified].
- **Derived, source-bound:** NormPublicJets.firstJet gives explicit modular reachability; poleWindowNoNewNorm excludes new norms by componentwise support-multiple corrections under both old and new pole budgets; publicResidueCertificate handles coupled branch congruences without moment inputs. Exact hypotheses and unsigned claim pins are in the report [pcc:unverified].
- **Unknown:** whether a structured public branch-selection procedure can exploit the cubic curve constraint to achieve the required bound. No hardness or global impossibility inference from these controls.
- **Capabilities:** KB MCP absent and canonical wrapper has no executable. No current paid-solver allowance/admission was established; local Lean/Rust development checks do not require a solver campaign.
- **Not established:** novelty, independent signed scientific standing, an efficient joint finder producing the required point labels, full input/exception/subgroup coverage, or total sub-exponential bit cost. The actual norm-to-point and supplied-label scalar bridge is now separately formalized at its exact hypotheses; componentwise rigidity does not exclude coupled/rational corrections or the earlier W-family without a map [pcc:unverified].

## What would unblock it

- [ ] Explicit public procedure taking curve, subgroup, base and target inputs, selecting valid support/root branches and returning a bounded nonzero A/B without supplied successful witnesses or secret sign labels.
- [ ] Complete input-to-scalar correctness with exact field, degree, subgroup and exceptional-case hypotheses. Partial: the actual public Hankel norm now implies the actual nonsingular point sum, and other supplied scalar labels give the target integer; see NormGroupBridge.publicHankelTargetScalar, unsigned claim PCC-20261113-3f21d1. Producing those inputs/labels efficiently remains open [pcc:unverified].
- [ ] Uniform exp(o(log r)) complete expected bit-cost proof including setup, failed candidates, root/linear algebra work and final verification. A fixed power of p fails this criterion.
- [ ] Source-pinned executable controls and independent replay at the scope claimed. Admit any scaling campaign prospectively with live finite resource bounds.

These conditions would resolve the certificate-finder gap at the stated scope;
formalizing a certificate or completing this problem record alone does not.

## Next discriminating step

- **Action:** derive a constrained discrete branch or support-selection rule with a counted reduction in visits to a singular actual public matrix. The matrix map, exact corank and norm-to-group bridge are available under their stated hypotheses. Compare discovered-sign work with supplied-sign reconstruction; do not assume a useful gradient from rank or determinant alone [pcc:unverified].
- **Inputs/controls:** public curve/support/branches, positive fixture, changed branch, exact full norm and degree checks. Distinguish supplied-sign and discovered-sign inputs.
- **Limits:** local proof/development work first; no paid attack/scaling run authorized by this document. A runtime panel requires a separate frozen protocol and live admission under repository rules.
- **Outcomes:** a proved structural search advantage leads to an actual public candidate generator and full accounting; an equivalence or a failed rule redirects the finder without a global impossibility claim. Infrastructure errors stay infrastructure errors.
- **Recheck trigger:** an explicit structural branch-selection construction, new theorem, or an admitted executable generator.

## Resolution and history

2026-11-13: initial evidence-linked record after exact jet/coupled-chart test.
Unresolved. Earlier artifacts and canonical claims/ledger entries are retained.

2026-11-13 signed-Hankel update: [new report](../research/prime_norm_rank_20261112/signed-hankel-report/README.md), source17fd598fd,
supplies the actual public branch-to-Hankel map and publicInputCertificate.
Singularity of that actual matrix produces the nonzero bounded pair without
supplied A/B or interpolation/moment oracles. The fixed native artifact
compares full6x6 and compressed2x2 kernels, checks literal branch updates,
and retains wrong-weight/changed-branch controls [pcc:unverified].

Next obligation refined: efficient branch selection in that public family,
keeping the target branch fixed. Investigate constrained determinant repair
and exact corank/uniqueness geometry; continuous field-weight repair need not
stay on the curve. Polynomial-size matrices and per-update work alone do not
bound total visits. The original exit criteria remain open [pcc:unverified].

2026-11-13 exact geometry/repair update: [report](../research/prime_norm_rank_20261112/branch-repair-report/README.md),
source4b6ed788d. Actual public kernel is one scalar line; singular numerical
corank1 follows, without a numerical-rank hypothesis. The continuous point
correction preserves the square iff it is zero or the opposite-branch change.
The one frozen native control records four off-curve/full-norm failures among
five continuous determinant repairs, one valid restoration and one zero
repair direction; fixing target0 excludes one of those repairs. No statistical
or asymptotic inference from those cases [pcc:unverified].

Next action refined: a constrained discrete/multi-coordinate branch rule or
a different public factor-base mechanism with fewer counted attempts. A
larger-corank target is excluded only in the stated actual public family;
continuous field-weight repair does not discharge branch selection. Original
exit criteria remain open [pcc:unverified].

2026-11-13 actual group-bridge update: [report](../research/prime_norm_rank_20261112/group-bridge-report/README.md),
source6ba718256. NormGroupBridge identifies the product of actual distinct
nonsingular point ideals with the actual norm function's principal ideal by
CRT and equality of finite quotient dimensions. The existing actual Point.toClass
injection then gives the zero point sum; the public Hankel certificate supplies
the norm without a divisor/group-relation oracle. The target corollary needs
only the other supplied integer labels, which remain explicit premises.
Unsigned theorem claimPCC-20261113-3f21d1 [pcc:unverified].

The fixed native control checks the positive sum as infinity, the flipped
sum as(10,6), and target integer-31 by multiplication from the stated public
base. This closes the actual certificate-to-group/supplied-label scalar
obligation for that stated family; it does not create those successful
branches or their labels. Next action remains an actual discrete finder
and a complete expected bit-cost bound [pcc:unverified].

2026-11-13 actual membership-input projection: [report](../research/prime_norm_rank_20261112/membership-elimination-report/README.md),
sourceccd9e23f1. A bounded raw-source audit keeps the prior RR m5/t4 cell
unfinished: its timeout/interrupted runs do not give a completed negative
result. The new construction recovers19 auxiliary variables from four actual
function coefficients and checks all25 original equations symbolically.
The remaining five constraints have degree24; expanded input grows from1554
to191962 bytes. Sixteen universal Lean statements cover target/norm division,
unique actual tower recovery and degree guards; the concrete compiler binding
is a separate unsigned native check, claimPCC-20261113-b465d8 [pcc:unverified].

This is an exact representation trade, with no solver or cost comparison yet.
Next discriminating candidate is a module-aware/circuit-preserving solver
that uses the projection without paying for expansion, compared on identical
inputs with the strongest original formulation. It must still produce actual
successful public inputs and meet the original complete expected bit-cost
criterion. No new paid or solver campaign was launched [pcc:unverified].

2026-11-13 explicit special-fiber update: [report](../research/prime_norm_rank_20261112/weighted-leading-report/README.md),
source5e342cafb. The actual target-completed pole family and Dickson tower
have Lean endpoint identities; at the special fiber, its four nonconstant
remainder coefficient equations force the zero retained tuple over every
field. The support is monic degree5; a vanishing pure-power remainder makes
it a monic divisor of X^16, hence X^5, and triangular nilpotence forces all
four coefficients to zero. Eighteen new statements, whole-study gate304/0,
import-inclusive fresh replay0; centralPCC-20261113-7b7707 and receipt
PCC-20261113-582ee7 are unsigned [pcc:unverified].

The special-fiber origin is not a recovered parameter-one solution. Exact
weighted initial-form binding to the concrete reduced.ms input, regularity,
flatness, a weighted closure/normal-form bound and complete expected bit
cost remain separate. The prospectively frozen fixed native binding
experiment was not launched: checkpoint upload transport is unavailable
after the resume was supplied (PCC-20261113-cefb51). Continue the universal
pole-normalization/filtration proof independently; use an admitted symbolic
fixture check when its infrastructure is available. Original W-jet/public
finder exit criteria remain open, without identifying the families or
inferring a non-generic impossibility result [pcc:unverified].
