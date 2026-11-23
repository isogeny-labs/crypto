# Projective Edwards transfer adapter

This directory specifies the bounded implementation slice tracked by stacked
draft PR #4835. It adds a total, typed target-model adapter for the existing
odd-prime degree-two cover. It does not supply the separate Jacobian
index-calculus solver, catalog-wide subgroup preservation, computational
advantage, or a scaling result. [pcc:unverified]

## Requirement-to-evidence ledger

| Requirement | Initial state | Acceptance evidence |
| --- | --- | --- |
| Exact supported models and fields | specified | Deterministically proved prime `p > 3`; twisted and scaled Edwards parameters satisfying the nonsingularity identities below |
| Total projective representation | specified | Both defining quadrics, canonical projective normalization, identity, two-torsion, and both `Z=0` boundary charts |
| Forward and inverse maps | specified | Branch-complete formulas, curve-equation checks, and both round trips up to projective scale |
| Composed cover maps | not implemented | Typed pullback, arbitrary-class pushforward, target arithmetic, and both degree-two composition certificates |
| Independent oracle | designed, not yet durable | Exhaustive independent enumeration over every admitted parameter tuple at `p=5,7,11`, known-false mutations, and source-bound receipt |
| CLI and sweeper integration | not implemented | Original-coordinate point input/output, structured malformed/unsupported results, and deterministic replay |
| Live catalog classification | audited | 24 Edwards maps and 15 named bindings; all 15 remain proof-gated by field/order evidence after the backend exists |
| Subgroup preservation | unknown for live Edwards bindings | Requires admitted field/order evidence, checked generated-subgroup path, and independent replay |
| Advantage, scaling, full solver | unknown / separate | No promotion from this adapter or its bounded oracle |

## Sources checked

The repository knowledge-base wrapper was unavailable in this runtime:
`scripts/research-kb status` exited 127 with `no executable found`. The audit
therefore used the committed evidence and primary/official sources below; this
is not a novelty claim. [pcc:unverified]

- Bernstein, Birkner, Joye, Lange, and Peters, *Twisted Edwards Curves*,
  [IACR ePrint 2008/013](https://eprint.iacr.org/2008/013), DOI
  `10.1007/978-3-540-68164-9_26`. The official page identifies the twisted
  Edwards/Montgomery equivalence and projective coordinates. The PDF endpoint
  returned an HTML denial in this runtime, so no inaccessible page is cited as
  having been read.
- Bernstein and Lange, *Faster addition and doubling on elliptic curves*,
  [IACR ePrint 2007/286](https://eprint.iacr.org/2007/286). The official
  abstract states the birational Edwards model scope and distinguishes
  complete addition hypotheses from arbitrary model conversion.
  [pcc:unverified]
- [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748), Sections 3 and 4.1,
  gives concrete Montgomery/twisted-Edwards equations, restrictions, and
  affine maps. The fetched text had SHA-256
  `279ca0ecc5e92e2962e27b846986aeb74729d9dd34bd4a04a362f80dcb596ad3`.
- The [Explicit-Formulas Database twisted-Edwards entry](https://www.hyperelliptic.org/EFD/g1p/auto-twisted.html)
  and [projective-coordinate entry](https://www.hyperelliptic.org/EFD/g1p/auto-twisted-projective.html)
  record the equation, coordinate conventions, and source-linked formulas.
  The generic-coordinate data fetched with SHA-256
  `14a962a51db3d4b8e02775735cd46c1a788df9b61945cc82941655580d51d2fd`.
- [`docs/curves/COVERS.md`](../../docs/curves/COVERS.md),
  [`src/bin/curve_cover_check/models.rs`](../../src/bin/curve_cover_check/models.rs),
  and [`src/prime_hyperelliptic/model_transfer.rs`](../../src/prime_hyperelliptic/model_transfer.rs)
  provide the repository's coefficient normalization, catalog formulas,
  prime-proof boundary, and existing projective Montgomery adapter.
- [`research/efd_coordinate_charts_20261108/Charts.lean`](../efd_coordinate_charts_20261108/Charts.lean)
  proves only denominator-qualified affine chart round trips. It does not
  establish the global completion or exceptional branches.

## Exact model and parameter boundary

For `a*x^2+y^2=1+d*x^2*y^2`, set `c=1`, `a0=a`, and `d0=d`. For
`x^2+y^2=c^2*(1+d*x^2*y^2)`, set `a0=1` and `d0=d*c^4`. Require a
deterministically proved prime `p>3`, canonical coefficients, `c != 0`,
`a0 != 0`, `d0 != 0`, and `a0 != d0`. The Montgomery parameters are
[pcc:unverified]

```text
A = 2*(a0+d0)/(a0-d0),       B = 4/(a0-d0),
B*w^2 = u^3 + A*u^2 + u.
```

The adapter must reconstruct the existing short-Weierstrass intermediate and
check every coefficient identity rather than trusting catalog metadata.

Use homogeneous Edwards coordinates `[X:Y:Z:T]`, modulo nonzero common scale.
The twisted model is

```text
a*X^2 + Y^2 = Z^2 + d*T^2,   X*Y = Z*T,
```

and the scaled model is

```text
X^2 + Y^2 = c^2*Z^2 + c^2*d*T^2,   X*Y = Z*T.
```

The affine chart is `x=X/Z`, `y=Y/Z`, `T/Z=x*y`. The four-coordinate
completion is the graph closure of the affine model; unlike the singular
plane-quartic closure, its two `Z=0` families remain distinct typed points.

## Total branch table

Write a Montgomery projective point as `[U:W:V]`. Away from the two base
points, map it to

```text
[c*U*(U+V) : c*W*(U-V) : W*(U+V) : c^2*U*(U-V)].
```

The forward base-point branches are

```text
[0:1:0] -> [0:c:1:0]       (identity)
[0:0:1] -> [0:-c:1:0]      (rational two-torsion).
```

The inverse atlas is

```text
Z=0, X=0: [X:Y:Z:T] -> [-T : -c*Y : T]
Z=0, Y=0: [X:Y:Z:T] -> [c*X+T : 0 : c*X-T]
X=0, Y=-c*Z:          -> [0:0:1]
otherwise:            -> [X*(c*Z+Y) : c*Z*(c*Z+Y) : X*(c*Z-Y)].
```

The identity is accepted by the final branch and maps to `[0:1:0]`; an
explicit identity branch may be retained for diagnostics. Every branch must
validate its output and both compositions by projective equality, not raw
coordinate equality. Omitting either base point or either boundary chart is a
required known-false control.

## Audited live impact and claim boundary

The current graph has 30 normalized prime-field maps: 6 Montgomery and 24
Edwards (20 twisted and 4 scaled). Fifteen Edwards maps carry named EC1
bindings (11 twisted and 4 scaled). Every one of those 15 bindings has both
its field and declared annihilator outside the accepted deterministic proof
range. Completing the adapter therefore changes their primary disposition
from `unsupported_normalized_target_transfer_backend` to
`unsupported_without_primality_certificate`; it does not make any binding
supported. Historical enum values and corpora remain valid and immutable.

Bounded diagnostics over small fields are implementation-development evidence
only until their independent source, exact output, mutations, and receipt are
committed. A failed finite diagnostic is never a nonexistence result.
