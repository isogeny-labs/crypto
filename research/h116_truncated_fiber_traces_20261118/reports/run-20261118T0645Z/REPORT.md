---
title: "H116 omitted-Frobenius boundary and formal-group filtration"
date: "2026-11-18"
geometry: margin=0.70in
fontsize: 9.5pt
---

# Outcome

The source-frozen p=31 experiment hit both preregistered values. The normalized
retained `V-1` and independently constructed scalar `[-6]` maps agree at
coefficients 0 through 14, first differ at coefficient 15, and have boundary
delta 10 modulo 31. Both rational-map identities pass and all three negative
controls reject. [pcc:unverified]

| quantity | frozen prediction | measured |
| --- | ---: | ---: |
| first different coefficient | `(31-1)/2 = 15` | 15 |
| `V-1 minus scalar` delta | `2/(-6) = 10 mod 31` | 10 |
| equal prefix coefficients | 15 | 15 |
| map identities | 2 | 2 |
| negative controls rejected | 3 | 3 |

![Retained coefficients and modular differences. The vertical line is the
precommitted boundary, not a fitted value.](boundary.png){width=92%}

The corrected producer recorded 96,757 additions, 96,881 multiplications and
four inversions at precision 18. These deterministic counts are diagnostics of
one fixed cell, not runtime scaling evidence. [pcc:unverified]

# Proof and experiment chain

![The generic filtration mechanism is proved; the actual elliptic
Verschiebung/Frobenius identification remains an endpoint.
[pcc:unverified]](frobenius_bridge.png){width=99%}

The Lean source imports Mathlib's actual one-dimensional `FormalGroup`. It
proves `Y | F(X,Y)-X`, transports that quotient through substitution, proves
that an `X^m` correction is invisible below degree `m`, and composes this with
the existing leading-parameter/double-pole bridge. The exact repository
auditor reports 84 theorems, zero failures and zero trusted native decisions;
the default-limit build passes. [pcc:unverified]

The producer constructs `[6]` as `[2]([3])`. The independent verifier instead
uses division-polynomial recurrences through `psi7`. The first frozen producer
had the wrong sign on `A^2D^4`; its failed curve identity and verifier rejection
are retained. Version two changed only that documented sign while preserving
the fixture, prediction, controls and verifier.

# Reproducibility and boundary

Protocol freeze commit is `ec89b5d977`; corrected-source freeze commit is
`61f650e0b9`. Result SHA-256 begins `d2efcf9d8eb20962`; verifier output SHA-256
begins `ce94c6fb0c81ba83`; producer source SHA-256 begins `9d24f17c29b44e46`.
Complete hashes, arrays and commands are in
`../../FROBENIUS_BOUNDARY_EXPERIMENT.md`. [pcc:unverified]

The result checks the first coefficient omitted by the earlier low-jet match
and supports the predicted filtration boundary in one excluded small curve.
It does not prove that the supplied rational `V-1` map instantiates the formal
sum on every ordinary prime-field curve, does not establish a growing-family
predicate, and does not establish a subexponential ECDLP algorithm. The next
proof obligation is the actual elliptic `V+pi=[t]`/local-Frobenius/sign/scaling
instance. [pcc:unverified]

# Sources

- Milne, *Elliptic Curves*, Remark II.4.3 and Theorems IV.9.1, IV.9.4.
- Mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`.
- Study prior-art receipt `../../PRIOR_ART.md` and proof map
  `../../PROOF_MAP.md`.

