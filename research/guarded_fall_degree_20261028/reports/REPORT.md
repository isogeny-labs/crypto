# Guarded fall-degree proof and extensions — 2026-10-28

This is a mathematical result report and development-verification record.
It is not an experiment timing report or a claimed ECDLP improvement.

![Proof dependencies and the open encoding bridge](proof-dependencies.svg)

The solid arrows are dependencies in the written derivations. The dashed
bridge to growing Boolean lower bounds is unproved. The Lean box covers only
the identity declarations in the proof map, not the complete degree theorem.

## Result

The ordinary coefficient-field model uses three membership equations, two
third summation polynomials, and an inverse guard for distinct first-pair
abscissas. For three abscissas in characteristic greater than three, every
unsatisfiable instance has exact first/last pair `(4,5)` under the stated
ordinary-top/Koszul first-fall definition.

In characteristic two the curve model is
`Y^2+XY=X^3+a2*X^2+a6`, `a6 != 0`. The first fall is four. Last fall is four
for split cube-orbit membership `g(T)=T^3+c`, and five otherwise. The exception
cannot occur over `GF(2^n)` with odd `n`.

For growing cardinality `B`, the odd-characteristic bounds are
`max(5,B+1) <= d_F <= 3B-4`, with first fall `B+1`. The binary extension has
`B+1 <= d_F <= 3B-4` for `B>=4`, with its separate three-abscissa classification.
These bounds are derived, not fitted to measurements. They involve a degree-`B`
membership input and do not prove analogous growth in a fixed-degree Boolean
presentation.

For the fully Boolean binary two-S3 chain with a single shared auxiliary
`u`, three linear `ell`-bit base embeddings, and explicit equations for every
bit, a separate direct argument proves `d_F <= 3*ell+1` for `ell>=1`,
independently of `n`. It holds with or without the inverse guard and needs
no rational-lift assumption. After fixing the base bits, the remaining
equations are affine in all auxiliary bits. Support-aware staged assignment
indicators localize each affine contradiction within the stated cap.
This may require exponentially many proof rows; it gives no growing lower
bound. See [the complete proof](../extensions/characteristic_two/boolean_upper_bound.txt).

## Method and verification

The local upper proof reduces the first-pair state to the missing third
abscissa, normalizes quadratic fibres by successive multiplication and monic
division, and derives a scalar resultant without exceeding degree five.
The scalar grid has normal forms of degree at most four, so staged inversion
derives one. Characteristic two uses a Frobenius cancellation in place of
division by two. Closed separating superspaces give the lower bounds.

Growing-base upper bounds lift local three-element proofs using an exact
triple-selector partition of unity. The lower bound uses the regular
leading-form core and a degree-`B` closed separating space. It assumes no
first-fall/solving-degree heuristic.

Native replay checks each historical DAG operation against its separately
supplied input. The pinned GF16 cube-orbit unit fixture checks the exceptional
four-degree case and serialized proof replay. No new field/base/target sweep
was run in this integration. The full library test and native/Lean checks are
recorded in `../validation/` and the PR.

The partial Lean project checks resultant, guard, missing-root, and related
identities and a common-root implication. The complete filtered-closure,
finite-grid, and curve bridges remain unformalized. See
[proof map](../formal/PROOF_MAP.json).

The repository identity records bind these four checked identities to the
Lean source hash: odd resultant `IDC1h56e099b3201f45e4`, binary integer lift
`IDC1h1fcce05743df34d3`, guard cancellation `IDC1h0808f60fce83a05e`, and
missing root `IDC1h304bc867d3d73267`. See [replay](../identities/replay.json)
and [serialized recheck](../identities/recheck.json). Their native checker
does not execute Lean or certify the closure theorem.

## Relation to existing work and open scope

[Sources](../SOURCES.md) records the primary literature checked and links to
the existing fall-degree ledger and E12 continuation. The latter's mixed
labels and independent-replay limitation are unchanged. There is no transfer
theorem here that converts coefficient-field cardinality lower bounds into
Boolean Weil-descent lower bounds. The direct Boolean upper theorem applies
only after the actual input encoding is checked against its hypotheses.

No performance dashboard, leaderboard, or benchmark graph changed: this work
has no measured cost ratio, speed improvement, or full-DLP result to add to
those axes. The study-local graph records dependencies rather than invented
measurements. The historical Python sources and raw parameter/certificate
files are provenance evidence; the new execution paths are native.

The remaining research obligation is a growing Boolean lower bound or exact
degree result, together with an encoding audit before applying the new upper
theorem to any existing ledger.
Partial Lean identity verification does not discharge it. Larger-cardinality
exact last-fall values also remain open between the proved bounds.
