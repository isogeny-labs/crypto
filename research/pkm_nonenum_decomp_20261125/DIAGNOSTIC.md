# Prefix-sieve implementation diagnostic — 2026-11-25

The frozen protocol is `PROTOCOL.md`. This diagnostic uses the explicit
curve `y²=x³+2x+3` over `F_97`, `g=1`, and Kummer roots of order `2^t`.
The curve and targets are a correctness and bottleneck probe, outside the
protocol's independent prime-order-curve panel. No qualifying performance
ratio is inferred from them.

`prefix_sieve.py` represents each exponent prefix by one polynomial equation,
adds the two Semaev `S3` equations, and asks SymPy for a Groebner basis over
`F_97`. A basis containing `1` certifies that the prefix has no solution even
over an algebraic closure. At surviving leaves, it lifts actual curve points
and checks their sum against the target. Its wall cap yields a `censored`
record; there is no success inferred from such a record. The implementation
does not yet count field operations or enforce the frozen 2-GiB memory limit,
so this remains a pilot of the mechanism.

| `t` | target | signed base | independent ordered witnesses | candidate status | nodes | certified prunes | checked leaves | candidate witnesses | Groebner wall |
|---:|:--|---:|---:|:--|---:|---:|---:|---:|---:|
| 1 | `(0,87)` | 3 | 1 | complete | 7 | 3 | 1 | 1 | 0.022 s |
| 2 | `(0,87)` | 5 | 10 | complete | 67 | 24 | 10 | 10 | 1.966 s |
| 3 | `(92,16)` | 9 | 18 | censored at 10 s | 1 | 0 | 0 | 0 | 10.005 s |
| 3 | `(92,16)` | 9 | 18 | censored at 60 s | 1 | 0 | 0 | 0 | 60.002 s |

An independent exhaustive point-sum loop produced the witness counts. For
`t=1,2`, the prefix solver returned exactly those counts. For `t=3`, a
nontrivial witness is `(1,54)+(22,92)+(50,19)=(92,16)`; both Semaev equations
were separately checked. The 60-second root call did not finish, before any
prefix could be split. This falsifies the generic Groebner root solver as a
practical first implementation on this diagnostic. It does not falsify the
prefix-sieve idea with a specialized certificate.

The next candidate implementation needs a cheaper, checkable prefix
inconsistency test. Until that exists, the three-curve timed panel would be
censored at its first root system. The exact program and controls remain in
this directory; the broader route-1 and route-2 controls are under
`research/subexp_routes_20261125/`.
