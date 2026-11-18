# H116 regular-unit substitution control

Status: complete finite-series control under frozen protocol
`H116-REGULAR-SUBSTITUTION-001`. [pcc:unverified]

## Frozen question

Does a nonconstant regular outer unit preserve the leading-parameter precision
boundary proved by `PowerSeriesPrefixEq.leading_X_inv_sq_mul_subst_inv`? The
protocol fixed 20 prime/cutoff cases and the three boundary formulas before
execution. Producer and verifier sources were committed at `8490ba95b0`; no
case was replaced. [pcc:unverified]

## Result

| measured boundary | predicted first difference | matches |
| --- | ---: | ---: |
| `q(Xf)` substitution | `m+1` | 20/20 |
| `q(Xf)^-1` | `m+1` | 20/20 |
| `f^-2 q(Xf)^-1` | `m` | 20/20 |

All substitution deltas equal `q_1 c`, all inverse-substitution deltas equal
`-q_1 c q_0^-2`, and all full normalized-coordinate deltas equal
`-2 c f_0^-3 q_0^-1`. The independent verifier confirms all 60 boundary
equalities and all 20 control triplets. [pcc:unverified]

For `(p,m)=(23,1)`, substitution and inverse substitution first differ at
degree two, while the normalized coordinate first differs at degree one; the
three observed/predicted deltas are `2`, `10`, and `22`. For `(587,22)`, the
indices are 23, 23, and 22, with deltas `468`, `535`, and `137`. A retained
result with the first coordinate delta incremented is rejected.
[pcc:unverified]

The control triplet in every case checks that zero correction leaves the
coordinate unchanged, a constant outer series ignores parameter differences,
and an outer series with zero constant is rejected before inversion.
[pcc:unverified]

## Interpretation boundary

The result supports the exact generic composition theorem and shows that the
regular unit neither consumes another coefficient nor hides the sharp pole
boundary on this panel. It does not prove that the formal scalar parameters are
the actual `V-1` parameters, nor identify the synthetic `q` with a curve's
coordinate series. [pcc:unverified]

## Integrity

```text
1af64543e8df2fa9b8338c5f0c3673ca4522eb52a2c209712fc0194a381e43bb  REGULAR_SUBSTITUTION_PROTOCOL.json
484f839b802f6a6b6b5687d11d53649b5d54dfefcebcd82f3555cde7e20da11b  h116_regular_substitution_control.py
2b123a2af2b0c9e4adb8e55b6a12e4412ee99fb875798d7c602b41f178a3cb0c  h116_regular_substitution_verify.py
becf9bd0a89b78c81549db6dcf1be1f0d12c720cc185b7a523ce7b42c54a1d89  REGULAR_SUBSTITUTION_RESULT.json
```
