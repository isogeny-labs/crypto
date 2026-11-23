# S4: actual resultant recursion and a degree-drop control

Lean proves the first resultant-recursion step on the general Weierstrass
model, over any algebraically closed field and in every characteristic.
With nonzero discriminant, the actual fixed degree 2/2 Sylvester resultant
of S3(x1,x2,T) and S3(x3,x4,T) vanishes exactly when four actual affine
points with those x-coordinates sum to zero. [pcc:unverified]

The definition calls Mathlib's `Polynomial.resultant` directly, with explicit
degree arguments 2 and 2. It does not replace the determinant with an assumed
formula. `quadratic_resultant` computes its actual matrix and proves
Res=(af−cd)²−(ae−bd)(bf−ce), for aT²+bT+c and dT²+eT+f.
`generalS4_factor` then uses the already checked signed S3 roots for a
distinct first pair. A finite common root glues actual points equal up to
negation; the reverse direction extracts the coordinate of the nonzero
pair sum. A distinct second pair is handled by pair symmetry.
[pcc:unverified]

When both pairs repeat, both leading coefficients vanish. The fixed-size
resultant is zero, while actual points P,−P,Q,−Q sum to zero, with the
partial pair sums at infinity. This branch is part of the proof, not an
exception discarded by a genericity assumption. [pcc:unverified]

The negative control is the actual smooth F5 curve y²=x³−x, base changed to
its algebraic closure:

| Quantity | Lean conclusion |
|---|---|
| S3(0,0,t) | 1 for every geometric t |
| S3(1,1,t) | 4 for every geometric t |
| Fixed degree 2/2 resultant at (0,0,1,1) | 0 |
| Common finite root | None |
| Four actual affine points with these x-coordinates | A zero-sum quartet exists |

Thus fixed-size resultant zero does not imply a common finite root after
simultaneous degree loss. Recomputing a resultant with the specialized
polynomials' new actual degrees changes the operation. The control refutes
that proof shortcut; the qualified S4 point-sum theorem is proved.
[pcc:unverified]

Source: [GeneralS4.lean](lean/GeneralS4.lean). Main declarations:

- `quadratic_resultant`: actual Sylvester determinant coefficient identity.
- `generalS4_common_root_iff`: the finite-root criterion with a distinct pair.
- `glue_common_root`, `fourPoints_to_common_root`: actual group-law bridges.
- `generalS4_zero_iff_exists_points`: the complete first-step equivalence.
- `fixed_degree_resultant_common_root_counterexample`: faithful geometric control.

The whole census project gate passes197 declarations,0 failing and0 trusted
native exceptions. Generated declarations are included; this is not 197 new
results. The theorem/control packets are PCC-20261109-21757f and
PCC-20261109-ce6722; their producer checks remain unsigned. [pcc:unverified]

Both full local packet checks pass; exact source/gate/receipt digests and
the native supplementary record are bound in
[general-s4-verification.json](general-s4-verification.json).
[pcc:unverified]

Supplement IDC1hbf1f44625ee306dc checks the determinant coefficient rewrite
using the unchanged repository native checker:6 variables,degree 4,
32 derived plus11 extra points over 2^61−1. Source was committed in e6bcfc3a
before issue, and the proof-file digest matches. These one-host finite
checks do not replace the universal Lean theorem or establish independent
host/signed standing. [pcc:unverified]

Primary source: [Kosters–Yeo2015](https://arxiv.org/pdf/1503.08001),
§2/p3recursive definition and Proposition 2.1, including visual PDF review.
Exact source/code hashes and read scopes are in
[general-s4-sources.json](general-s4-sources.json).
Original Semaev 2004remains unread: ePrint/CiteSeer 403and cited RHUL 404.
The [proof diagram/report](reports/proof-20261109-general-s4/report.pdf)
records the actual dependency and exceptional branch. [pcc:unverified]

General recursion for every m, irreducibility and individual symbolic
degrees remain open. The earlier bounded S4/S5census and frozen native
inputs/protocols/raw data are unchanged. No new curve population,
statistical experiment or paid compute was run. Reproduce the formal gate
with `tools/lean/proof-gate.sh research/theorem_census_20261109/lean`.
[pcc:unverified]
