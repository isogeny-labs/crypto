# Prime-field norm certificates: public branch selection and total cost

- **Status:** investigating
- **Kind:** construction/proof gap; separate unavailable KB/compute-admission capabilities
- **Affected work:** [norm derivation](../research/prime_norm_rank_20261112/DERIVATION.md), [next finder](../research/prime_norm_rank_20261112/NEXT_FINDER.md), [public-jet report](../research/prime_norm_rank_20261112/public-jet-report/README.md)
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
- **Not established:** novelty, independent signed scientific standing, an efficient joint finder, the entire divisor/scalar bridge in Lean, or total sub-exponential bit cost. Componentwise rigidity does not exclude coupled/rational corrections or the earlier W-family without a map.

## What would unblock it

- [ ] Explicit public procedure taking curve, subgroup, base and target inputs, selecting valid support/root branches and returning a bounded nonzero A/B without supplied successful witnesses or secret sign labels.
- [ ] Correctness linked through actual divisor/group relation and scalar verification; exact field, degree, subgroup and exceptional-case hypotheses.
- [ ] Uniform exp(o(log r)) complete expected bit-cost proof including setup, failed candidates, root/linear algebra work and final verification. A fixed power of p fails this criterion.
- [ ] Source-pinned executable controls and independent replay at the scope claimed. Admit any scaling campaign prospectively with live finite resource bounds.

These conditions would resolve the certificate-finder gap at the stated scope;
formalizing a certificate or completing this problem record alone does not.

## Next discriminating step

- **Action:** derive the actual signed public evaluation-matrix family and a branch-update rule that changes the probability or counted cost of finding a nonzero bounded kernel. Compare with supplied-sign reconstruction; do not assume a useful gradient from rank or determinant alone [pcc:unverified].
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
