# Adaptive S3: checked matrix degree and actual scalar recovery

2026-10-25 · PR2381 · run `adaptive-s3-20261025` ·
[protocol](../../ADAPTIVE_S3_20261025_PROTOCOL.md) and
[pre-execution correction](../../ADAPTIVE_S3_20261025_V2.md) ·
[PDF](report.pdf) · [graph](degrees.svg) and [data](coverage-data.json) ·
[canonical native cases](../../native/adaptive-s3-20261025/) ·
[Lean](../../lean-adaptive/AdaptiveS3.lean) ·
[validation and attempts](../../validation/adaptive-s3-20261025/README.md).

The named certificate-seeking solver was executed on all 98 prior size-two
cases, 188 main/twist factor-base comparisons and 1,384 target comparisons.
C++ generates polynomial rows with exact witnesses in the supplied S3 or
reduced-S3 inputs. Independent Rust reconstructs every matrix space, promotion,
witness expansion, point lift, useful-rank trace, stopping decision and degree.
The fresh replay matches every canonical result byte. Five independently
rehashed false payloads are rejected without a receipt.

The result has **24 joint cases of larger useful rank and smaller operational
matrix degree** for the reduced input; the original S3 input has **zero**.
The 24 are target comparisons, with repeated F43 base policies, rather than
independent random curves or a success probability. This is exact finite
algebra, with no timing claim. All losses and ties are in the canonical index.

## Concrete F43 first target

The main curve is `E/F43: y²=x³+x`, generator `P=(31,18)` of order 11. The
monic twist is `T/F43: Y²=X³+4X`, generator `U=(24,26)` of order 11. Main
base points are `(4,5),(13,19)` and twist base points `(15,9),(17,6)`.
The registered models remain `icv1-fp6-t0-74779950` and
`icv1-fp6-t0-c6a8a47c`; no new representation or generator alias is inferred.
The main and twist first target use `n=1` in their respective subgroups.

| Audited quantity | Main original S3 | Main reduced S3 | Twist original S3 | Twist reduced S3 |
| --- | ---: | ---: | ---: | ---: |
| Actual independent useful rows | 0 | 0 | 2 | 2 |
| Constrained ordered roots | 0 | 0 | 3 | 3 |
| Named solver matrix degree | 4 | 3 | 4 | 2 |
| Highest expanded input-witness degree encountered | 6 | 3 | 4 | 2 |
| Terminal certificate expanded degree | 6 | 3 | 4 | 2 |

For the reduced inputs the main remainder is
`hm=41xy+39(x+y)+27`, while the twist remainder is
`ht=17xy+3(x+y)+41=17(x−15)(y−15)`. Both use their respective
quadratic factor-base membership polynomials. Lean proves that the main D=2
input space excludes 1, and checks the D=3 unit witness

```
(37+16x+16y)hm +(21+32y)(x²+26x+9) +(21+32x)(y²+26y+9)=1 mod 43.
```

Lean also proves that the twist system's exact roots are `(15,15)`, `(15,17)`
and `(17,15)`, and that every quadratic vanishing at these three roots is a
linear combination of the three supplied quadratic inputs. Thus its D=2
row space is the full quadratic vanishing space. This binds the finite solver
certificate to the actual input equations. The universal integer-cleared unit
polynomial has source-bound certificate `IDC1h9eea191ebc3a8def` with 32+16
native evaluations and false-identity/source controls. That certificate covers
the polynomial, while Lean supplies the field and ideal statements.

The original input's main matrix degree is 4 but its largest expanded witness
degree is 6. Reinsertion therefore lowers the degree of matrices processed by
this named algorithm without establishing a degree-4 refutation in the
original S3 inputs. The previous homogeneous cutoff was 2 for **both**
reduced arms. This round does not show lower intrinsic regularity on the twist,
nor does it measure F4/F5, wall time or complete attack cost.

## Verified main-curve transfer

For this specific F43 pair the previously proved actual point map
`phi(O)=O`, `phi(x,y)=(41x,11y)` is injective and additive, sends `P` to
`U`, and preserves integer scalar relations. It is an explicit base-field
isomorphism for this exceptional fixture, not a generic twist map. This round
passes all 11 separate main input points through `phi`, derives the twist base
logs `5,7` from actual generator relations, and uses certified reduced-S3
root sets at recorded target shifts to recover and verify the source scalars
`0…10`. The identity case has no solver attempt; the ten nonidentity cases
have their complete attempted solver certificates in the index. The recovery
function receives each point, never its known scalar.

The factor bases differ between main and twist. Matched and pulled-back-base
controls preserve the relevant degree/status profiles and prevent treating a
coordinate relabeling as an intrinsic gain. This fixture demonstrates a valid
benefit to solving selected main-curve targets **under this chosen factor
base and named solver contract**. It does not establish a practical advantage
or a general ECDLP reduction for other twists.

## Proof map and next discriminating work

| Report claim | Lean declaration or checker |
| --- | --- |
| Checked row insertion preserves input ideal | `AdaptiveS3.adjoining_checked_row` |
| Main D=2 cannot derive 1 | `F43.mainDual_rows`, `main_no_degree2_unit` |
| Main D=3 derives 1 | `F43.main_degree3_unit`, `cleared_main_degree3_unit` |
| Three twist roots and their full quadratic vanishing space | `F43.twist_exact_roots`, `twist_quadratic_vanishing_space` |
| Actual scalar-preserving F43 map | existing `S3System.F43.toTwist_injective` and homomorphism theorems |
| Every operational trace, witness and recovered scalar | independent native Rust audit; no universal Lean solver theorem claimed |

The full scoped proof gate checks this addition alongside the unchanged prior
projects. It uses only standard Lean foundations (`propext`,
`Classical.choice`, `Quot.sound`) and has no extra trusted axiom. The full
library test and native replay receipts are in validation.

The next lead is to compare source-matched base policies on non-isomorphic
twists and to account for factor-base construction, relation acquisition,
linear solve, transfer and verification for multiple unknown targets. For any
claim of lower *intrinsic* regularity, compute a defined invariant of the
same homogenized ideals under a fixed term order; an input-leading cutoff or
one solver trace cannot substitute for it. The remaining general-model
normalization, exceptional S3/S4 lifting, generic pairing and trace-zero
bridges, shared Conductor access, CI/review and merge conflicts retain their
separate completion gates. Earlier canonical charts describe other metrics and
are unchanged; this run's graph records the new operational metric.
