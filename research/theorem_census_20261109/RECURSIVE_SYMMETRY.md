# Symmetry and sharp degree for the two newest Semaev inputs

Lean proves that each actual canonical recursive resultant is unchanged
when its two newest independent inputs are exchanged. Both have degree
$2^{m-2}$ at every recursive arity $m\ge4$. This extends the previous
all-arity last-input proof to the penultimate input. The earlier proofs
already cover S3 and all four S4 input degrees. [pcc:unverified]

The complete tracked census project gate passes **320 declarations**, zero
failures and zero trusted-native exceptions. Only propext, Classical.choice
and Quot.sound are accepted. No hole, custom mathematical axiom or native
decision exception. [Gate summary](evidence/recursive-symmetry-gate.log),
[lossless full audit](evidence/recursive-symmetry-gate.tar.gz), with
[unpack/hash receipt](evidence/recursive-symmetry-gate-archive.json).
Lean 4.35.0 and Mathlib 56a00585 are pinned. [pcc:unverified]

The proof uses Mathlib's actual bivariate algebra automorphism. With the
new variables $z=C(X)$ and $t=X$, it sends $z$ to $t$, $t$ to $z$, and fixes
old coefficients. The lifted old polynomial is fixed, and the three
coefficients of the lifted S3 quadratic are symmetric in $z,t$.
Resultant functoriality therefore fixes the actual declared-size Sylvester
determinant. No degree is reset after specialization. [pcc:unverified]

Applying this identity to the actual increasing coefficient-ring family
identifies its last-variable degree with the degree read after exchanging
the last two variables, which is the original penultimate-input degree.
The existing sharp last-input theorem supplies $2^{m-2}$ for both.
This is an identity of formal polynomials and an actual degree proof,
not agreement on a finite set of field evaluations. [pcc:unverified]

| Obligation | Declaration in `RecursiveSemaevSymmetryAudit` |
|:--|:--|
| Actual exchange of inner/outer variables | `exchange_inner`, `exchange_outer` |
| Old coefficients fixed | `exchange_scalar`, `firstLift_exchange` |
| Lifted S3 quadratic fixed | `exchange_qa`, `exchange_qb`, `exchange_qc`, `secondLift_exchange` |
| Actual resultant fixed | `resultantStep_exchange` |
| Canonical recursion fixed | `recursive_new_pair_exchange` |
| Sharp penultimate degree | `recursive_penultimate_degree` |
| Fully quantified combined binding | `newest_pair_audit_certificate` |

The proof does not establish full permutation symmetry or the degrees of
all older inputs at arity at least 5. Higher geometric zero-sum recursion,
irreducibility and the original fourteen-item audit remain separate.
There is no new native experiment, timing or paid compute. [pcc:unverified]

See [new source](lean/RecursiveSemaevSymmetry.lean),
[source plan](recursive-symmetry-sources.json), and retained failed
source snapshots/diagnostics in evidence/. Primary read scopes remain
recorded in sources.json and the earlier recursive-degree notes.
Original Semaev 2004 remains unread after 403; no source-status promotion
or novelty claim is made. [pcc:unverified]

Source commit `0c3de9b1` precedes packet `PCC-20261110-104604`; its full
local check passes at 01:54:16 UTC on producer host `762f5fc120c9`. Signed
standing remains unverified. See [actual bindings](recursive-symmetry-verification.json),
[inspected two-page PDF](reports/proof-20261110-recursive-symmetry/report.pdf),
and [editable report](reports/proof-20261110-recursive-symmetry/report.md).
[pcc:unverified]
