# R76 correction: b has an infinity zero

R76 correctly computes the **finite** zero divisors: degree 96 for
`b` and degree 80 for `c`. Its `CM.md`, `report.pdf`, `STATUS.md`, and
initial PR description incorrectly infer that the total pole degree
of `b` is 96. That inference omitted a zero at infinity. This additive
erratum supersedes only that pole-degree claim; the sealed R76 native
and Lean runs and their source-byte manifests are unchanged.

Set `t=1/z` and `U=t^2 V`. The quadratic equation becomes
`U^2+U=t+t^3`, which has two unramified rational places at `t=0`,
with `U=0` and `U=1`. At the first, `v(R)=-3` because
`R=t^-3(1+(1+t)U)` has unit bracket. The already proved identity
`Norm(R)=z=t^-1` then gives `v(R)=2` at the other place. Consequently

| Quadratic infinity place | `v(z)` | `v(R)` | `v(R+1)` | `v(b)` | `v(c)` |
|---|---:|---:|---:|---:|---:|
| `U=0` | -1 | -3 | -3 | -7 | -4 |
| `U=1` | -1 | 2 | 0 | 1 | -1 |

The degree-32 source has degree 16 over this quadratic field. The
fundamental equality for places above each unramified quadratic
infinity place gives weighted degree 16. Thus the proper pole degrees
are **112 for `b` and 80 for `c`**. The total zero degree of `b` is
96 finite plus 16 at infinity, also 112; `c` has no infinity zero.
The individual degree-32 infinity branches remain to be determined.

R77 will bind the infinity substitution and weighted arithmetic to
new exact checks. This correction is additive because changing sealed
R76 source files would invalidate the existing R76 custody pins.
