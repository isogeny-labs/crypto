# H116 pointwise Frobenius characteristic-polynomial experiment

- Protocol: `H116-POINTWISE-CHARPOLY-001`
- Protocol SHA-256:
  `36dbefbbd7d7efba055e94d1deaa427a63dc13a94ed6561bc38e1784a3efba4a`
- Source gate: `POINTWISE_CHARPOLY_SOURCE_GATE.md`
- Result SHA-256:
  `5c8f506a81280c650470dac6d2b32deca78c9e63a45282b1a579a6ace66368eb`
- Independent verification SHA-256:
  `8fa541743cee166a6a66ea58db95d8c21e465909d50f4af04c5703e251a269e6`

## Outcome

All eight positive cells pass. On all 5,066 enumerated points, coordinate
Frobenius has the correct extension-field order, satisfies
`pi^2-[t]pi+[p]=0`, and gives
`[p]pi^-1=[t]-pi`. Every enumerated extension point count also equals the
independent trace recurrence. This is exhaustive finite evidence, not a generic
proof. [pcc:unverified]

The protocol as a whole does not pass: seven of eight cells reject every
required mutation, while `fp5-a2-b1-d2` does not reject the mutation that
replaces `pi^-1` by the identity in `[p]pi^-1`. The failed control is retained.
It does not falsify either positive equation, but it shows that this mutation
is not a universal orientation control on finite extension-point groups.
[pcc:unverified]

| Cell | `#E(F_p)` | `t` | Enumerated/predicted `#E(F_(p^d))` | Characteristic failures | `V` failures | Wrong trace rejected | Wrong norm rejected | Missing inverse rejected | Protocol cell |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `fp5-a2-b1-d2` | 7 | -1 | 35/35 | 0 | 0 | 34 | 34 | 0 | fail: degenerate control |
| `fp5-a2-b1-d3` | 7 | -1 | 112/112 | 0 | 0 | 111 | 111 | 105 | pass |
| `fp5-a2-b1-d4` | 7 | -1 | 595/595 | 0 | 0 | 594 | 594 | 560 | pass |
| `fp7-a1-b1-d2` | 5 | 3 | 55/55 | 0 | 0 | 54 | 54 | 50 | pass |
| `fp7-a1-b1-d3` | 5 | 3 | 380/380 | 0 | 0 | 379 | 379 | 375 | pass |
| `fp7-a1-b1-d4` | 5 | 3 | 2475/2475 | 0 | 0 | 2474 | 2474 | 2470 | pass |
| `fp11-a1-b1-d2` | 14 | -2 | 140/140 | 0 | 0 | 139 | 139 | 126 | pass |
| `fp11-a1-b1-d3` | 14 | -2 | 1274/1274 | 0 | 0 | 1273 | 1273 | 1260 | pass |

The wrong-trace and wrong-norm mutations reject every nonidentity point in all
eight cells: 5,058 rejections each. The missing-inverse mutation rejects 4,946
points in total. Its invisible set normally has the size of the base-field
point group in this panel; the exceptional degree-two `p=5` cell makes the
entire 35-point extension group invisible to that mutation. [pcc:unverified]

## Why the frozen control degenerates

For `fp5-a2-b1-d2`, `t=-1` and every point over `F_25` satisfies `pi^2=1`.
The verified characteristic equation becomes [pcc:unverified]

```text
1 + pi + [5] = 0,
```

so `pi=[-6]` on this finite group. Its enumerated order is 35, hence `[35]=0`
on every point and

```text
[5]pi = [-30] = [5].
```

Because degree two also gives `pi^-1=pi`, the correct expression
`[5]pi^-1` and the mutated expression `[5]` are identical on the whole group.
The verifier independently reproduces the zero rejection count. This is an
algebraic collision in the frozen control, not a producer mismatch.
[pcc:unverified]

## Measurements

The Rust producer reports 0.336159932 seconds summed across the eight cells;
the independent Python replay reports 8.685026739 seconds. These are single-run
diagnostics, not benchmarks. The producer emitted 185,760 point additions,
40,528 scalar multiplications, 138,438 Frobenius maps and 139,438 extension
inversions. [pcc:unverified]

The producer additionally reports 4,604,473 extension-field multiplications,
1,140,943 extension-field additions, 91,464,268 base-field multiplications and
95,510,068 base-field additions. These are exact instrumented counts for this
implementation, not an optimized cost model. No speed or asymptotic claim uses
them. [pcc:unverified]

## Independent verification

The Python verifier recomputes the first irreducible polynomial by exhaustive
monic trial division, enumerates every square pair, and reimplements the affine
group law and both Frobenius equations. It matches every producer correctness
field exactly and reports no mismatches. It returns failure only because the
same frozen control fails in the same cell. [pcc:unverified]

## Interpretation and next gate

The finite panel supports the characteristic equation and the pointwise
construction used by Lean, including negative trace and degrees two through
four. It does not prove the equation for generic prime-field curves, identify
the pointwise map with a rational or scheme Verschiebung, or establish any
ECDLP advantage. The next proof gate remains a generic elliptic theorem or a
proof-carrying interface for the characteristic equation. A future control may
measure the kernel of `[p](pi^-1-1)` explicitly, but this frozen panel will not
be amended or rerun. [pcc:unverified]
