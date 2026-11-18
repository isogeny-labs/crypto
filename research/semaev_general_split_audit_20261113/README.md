# Every source-ordered Semaev resultant split

Lean proves the literal canonical recursion for every permitted split:
if `N=(m+2)+(n+2)`, the resultant of the actual source polynomials
of arities `m+3` and `n+3` equals the canonical polynomial of arity `N`.
The elimination variable is last, and the two actual factor degrees are
proved to be `2^(m+1)` and `2^(n+1)`. Every source index
`N≥4, 1≤k≤N−3` is covered by `k=n+1`. [pcc:unverified]

The main declaration is
`SemaevGeneralSplitAudit.actual_all_source_splits_certificate`
in [SplitCertificate.lean](lean/SplitCertificate.lean).
It binds the literal multivariate polynomial, rather than inferring a
polynomial identity from values over finite fields. It also proves the
specialization identity over every commutative domain under arbitrary
coefficient maps, retaining the prescribed dimensions when degrees drop.
Repeated coordinates are allowed. [pcc:unverified]

The proof uses fully prescribed resultant interchange, the actual S3
convolution, actual canonical recurrence and full input symmetry. List
permutations move a peeled input between blocks. Universal evaluation
in the multivariate polynomial ring proves literal equality. Injectivity
of the coefficient maps for independent variables proves the natural
degrees. A separate partial-evaluation bridge proves the source's
final-variable ordering. See [proof map](proof-map.json) and
[source review](evidence/source-review.json). [pcc:unverified]

Two inherited negative controls remain explicit in the certificate:
resetting intermediate natural degrees can make general resultant
interchange give −1 on one side and 2 on the other; treating the curve's
b-invariants as independent can break S4 symmetry. Lean also proves
that the latter control's invariant values cannot come from a rational
Weierstrass curve. Neither control refutes the qualified source
recursion. [pcc:unverified]

The whole-project gate and exact PCC replay are recorded in the evidence
packet. Exact local **PCC-20261113-31f9cd** replay passed at 2026-11-13T22:34:15Z.
Signed standing remains unverified; see [receipt](verification.json).
All failed sources, build logs and exits are retained losslessly.
No holes, custom axioms, native evaluator, raised limits, global
elaboration flags or waivers were introduced. [pcc:unverified]

This confirms the general recursion in Semaev2004 Theorem1 equation2,
with the general invariant model documented by Kosters–Yeo2015. The
formal polynomial identity holds in every characteristic for actual
Weierstrass coefficient models; no new group interpretation for
singular models is asserted. Higher-arity absolute irreducibility
remains open. No novelty or qualified-source refutation is claimed.
This is formal work, with no new native experiment, paid compute or
agent upload. [pcc:unverified]

The [all-fourteen requirement map](../theorem_census_20261109/README.md#original-fourteen-requirements)
retains the other outstanding proofs. This packet does not complete all
fourteen items. [pcc:unverified]

Rebuild with `lake build` in `lean/`; from the repository root run
`tools/lean/proof-gate.sh research/semaev_general_split_audit_20261113/lean`.
The exact Lean/Mathlib versions, complete custom import closure,
declaration binding and artifact hashes are in [sources.json](sources.json).
