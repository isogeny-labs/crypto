# H116 formal-scalar low-jet experiment

Status: excluded-fixture construction checkpoint. One deterministic run was
completed on 2026-11-18. It is not an admitted H116 target run and does not
establish a generic ECDLP algorithm. [pcc:unverified]

## Outcome

The fixed-precision candidate consumed only
`(p,A,B,t)=(31,1,19,-5)` and constructed the first eleven coefficients of the
normalized x-map from the elliptic formal logarithm and the scalar
`a=t-1=25`. The full degree-37 map was loaded only after candidate
construction. The candidate returned

```text
[1, 0, 20, 0, 13, 7, 2, 4, 8, 14, 5].
```

That prefix equals the quotient of the independently retained full Vélu map.
Its first nine coefficients also equal the Bostan recurrence, and an
independent verifier recomputed both comparisons and the transformed
first-order curve identity through degree nine. [pcc:unverified]

| Check | Observed result |
| --- | --- |
| candidate constructed before full-map load | pass |
| candidate equals full-map quotient, coefficients 0--10 | pass |
| candidate equals recurrence, coefficients 0--8 | pass |
| transformed first-order identity, coefficients 0--9 | pass |
| lambda=2 coordinate-isomorphism control | pass |
| wrong scalar `a+1` | rejected |
| omitted target scaling | rejected |
| independent-verifier mutation of candidate coefficient 10 | rejected |
| synthetic p=23 leading Frobenius perturbation | first difference at coefficient 11 |
| excluded-boundary p=19 perturbation | first difference at coefficient 9 |

The p=23 and p=19 controls were frozen before execution. They exercise the
predicted conversion of the leading normalized local-parameter correction
from degree `z^(p-1)` to coefficient `w^((p-1)/2)`. They do not prove that the
actual elliptic Frobenius correction has no additional lower terms; that
identification is a separate geometric obligation. [pcc:unverified]

## Construction

For `z=-x/y`, the implementation computes the series defined by

```text
q = 1 + A z^4 q^2 + B z^6 q^3,
x = z^-2 q^-1,
L(z) = integral dx/(2y).
```

It obtains `[a](z)` coefficient by coefficient from
`L([a](z))=aL(z)`, applies the normalized target coefficients
`(a^4 A,a^6 B)`, forms `x_target([a](z)/a)/x_source(z)`, and reverts
`w=1/x_source(z)`. No numerator or denominator from the full degree-37 map is
accepted by this candidate path. [pcc:unverified]

## Exact operation accounting

These are deterministic counts from the dense modular-series implementation,
not wall-time measurements. Each row has a different workload and is therefore
not an end-to-end speed comparison. [pcc:unverified]

| Stage | base-field additions | base-field multiplications | base-field inversions |
| --- | ---: | ---: | ---: |
| formal-scalar candidate construction | 64,426 | 67,584 | 16 |
| candidate internal consistency checks | 7,961 | 7,926 | 0 |
| full-map quotient after the map is supplied | 55 | 55 | 0 |
| nine-coefficient recurrence control | 27 | 57 | 12 |
| synthetic p=23 boundary control | 39,712 | 45,722 | 4 |
| synthetic p=19 boundary control | 23,442 | 26,602 | 4 |
| omitted-target-scaling negative control | 15,413 | 17,630 | 2 |
| wrong-scalar negative control | 72,387 | 75,510 | 16 |
| coordinate-isomorphism control | 72,387 | 75,510 | 16 |

The candidate currently uses naive dense composition and coefficient solving.
Its construction is fixed at eleven output coefficients in this checkpoint,
but no asymptotic cost theorem has yet been proved for growing precision or
for the missing semantic bridge. [pcc:unverified]

## Lean checkpoint

The new Lean endpoint packages Mathlib's
`PowerSeries.trunc_X_pow_self_mul` in the existing H116 prefix relation. It
proves [pcc:unverified] that, for every prime `p>19`, `X^(p-1)` is zero below degree 22 and its
even coefficients at degrees `0,2,...,20` vanish. This is the exact formal
precision cutoff used by the first eleven `w` coefficients. The proof gate
reports:

```text
PASS  64 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

The proof uses the default limits and only `propext`, `Classical.choice`, and
`Quot.sound`. It deliberately does not assert `V+pi=[t]`, identify the local
elliptic Frobenius with `z^p`, prove formal-group subtraction, or transfer the
parameter cutoff through the x-coordinate pole. [pcc:unverified]

## Reproduction and retained evidence

Run from repository parent commit
`4340f2aa4d00f91c8d3cc93fdd573e7f5ee1b384` on x86-64 Linux with an
AMD EPYC 9V45 processor:

```text
python3 h116_formal_scalar_jet.py MAP_CERTIFICATE_CONTROL.json > /tmp/formal.json
diff -u FORMAL_SCALAR_JET_CONTROL.json /tmp/formal.json
python3 h116_formal_scalar_verify.py FORMAL_SCALAR_JET_CONTROL.json MAP_CERTIFICATE_CONTROL.json
```

Retained source/result hashes are:

```text
914c1909ffdd5e6b87a01f4c2da9de8faca0187da490cb56af323e105e64b6b4  h116_formal_scalar_jet.py
949c646348d4d55d64befa6f23e66f54784f48d8d2f592411c7e967590e0a2d5  h116_formal_scalar_verify.py
b0afc3ee9ddc9e1b6b7de5ebcd9eb2e8feddd79c8ec499ea258aba5fee7c5739  FORMAL_SCALAR_JET_CONTROL.json
decee05b15443105a82e29bbb684ae458583a34ea14944a84fb85a6ce24e3763  lean/H116FiberTrace.lean
```

The descriptive run report, editable diagram source, SVG and inspected PDF are
under [`reports/run-20261118T0429Z/`](reports/run-20261118T0429Z/).

## Evidence boundary and next test

This run supports a fixed-size construction on one excluded fixture and a
formal precision lemma. Map semantics are independently checked only through
comparison with the already constructed full map. Computational advantage is
not established: construction, semantic certification, digit-predicate and
decoder costs remain separate. [pcc:unverified]

The next falsifiable step is to derive the local identities
`V+pi=[t]` and `pi(z)=z^p` inside an elliptic formal-group interface, prove the
x-coordinate pole-order transfer, and then test the same constructor on
additional excluded curves without modifying its formulas or precision.
[pcc:unverified]
