# Orientation and y/x hybrid: binary S3 correctness

Six native development tests pass on the ordinary binary-curve models below, including all 24,782 affine triples on 18 curves. The comparison uses direct chord/tangent addition and independently written polynomial constraints. Claim `PCC-20261115-a65a43` has **unverified** standing: these are producer observations, without a signed independent verdict. [pcc:unverified]

This directory defines and checks explicit models because the resumed objective named no branch or formula. Scope is `E: y² + xy = x³ + a x² + b`, characteristic 2, `b ≠ 0`, and the third summation polynomial. It does not validate a separate proposed implementation, prime-field formulas, a higher summation eliminant or a solver cost. The library is unchanged.

## Equations and orientation

For three finite points `P_i=(x_i,y_i)` on E, define

```
s1 = x1 + x2 + x3
mu = x1*x2 + x1*x3 + x2*x3
S3 = mu² + x1*x2*x3 + b
```

The oriented model is

```
S3 = 0
lambda² + lambda + a = s1
yi = lambda*xi + mu                 (i=1,2,3).
```

A written derivation: substituting `y=lambda*X+mu` into E gives the monic cubic
`X³+(lambda²+lambda+a)X²+mu*X+mu²+b`. The constraints identify its coefficients with `∏(X+xi)`, including intersection multiplicities. Thus its intersection divisor is `P1+P2+P3`, and its group sum is O. Conversely a finite triple summing to O has this line and these coefficients. If two x values coincide, the incidence equations require the same point and `mu=x²` enforces its tangent; a repeated x=0 would contradict `S3=0` because b is nonzero. For a triple repetition the same coefficient comparison enforces a flex. This derivation assumes the standard line-divisor group law. It is not a Lean proof. [pcc:unverified]

`S3=0` alone neither chooses signs nor guarantees rational lifts. Rational lifts exist exactly when `Tr(s1+a)=0`: the slope equation is Artin–Schreier, and its two roots supply opposite line lifts. The tests compare this criterion to exhaustive rational-point addition, including repeated x and x=0. [pcc:unverified]

**Target convention:** equations here mean `P1+P2+P3=O`. For a requested decomposition `P1+P2=T`, supply `P3=-T=(xT,yT+xT)`, hence `t3=tT+1` when xT is nonzero. Supplying T instead reverses the requested relation.

## Anchor elimination and hybrid chart

For a supplied finite target point with **x3 nonzero**, the slope can be eliminated:

```
S3 = 0
x3*yi + y3*xi + mu*(xi+x3) = 0       (i=1,2).
```

Recover `lambda=(y3+mu)/x3`. If `C3=y3²+x3*y3+x3³+a*x3²+b`, then the exact characteristic-two polynomial identity is

```
(y3+mu)² + x3*(y3+mu) + (s1+a)*x3² = C3 + S3.
```

Thus on a valid target the omitted slope equation follows from S3. A native sparse-polynomial test checks this identity over `F2[X1,X2,X3,Y3,A,B]`, without finite-field reduction. [pcc:unverified]

For `ti=yi/xi`, **every xi must be nonzero**. Retain the chart equation and the two anchor equations:

```
Ci = xi²*(ti²+ti+xi+a)+b = 0         (i=1,2,3)
S3 = 0
x3*xi*(ti+t3)+mu*(xi+x3) = 0         (i=1,2).
```

The map `(xi,ti) -> (xi,xi*ti)` identifies this system with the anchored one. Negation is `ti -> ti+1`. The tests check both chart directions over every field element and compare hybrid acceptance to the full group relation. The t coordinate alone does not specify x: its chart equation is cubic in x. Restricting t also changes an x-defined factor base unless the original membership constraint is transported explicitly. [pcc:unverified]

## Exceptions and failing shortcuts

| Case | Required handling / concrete negative control |
| --- | --- |
| Repeated points | Retain Vieta/multiplicity constraints. A determinant alone accepts `P=Q=R=(1,0)` on `GF(8), a=0,b=1`, although `3P != O`. |
| Target x3=0 | Use the original lambda model or a separate full-point branch. The denominator-cleared anchor admits `P=(1,1), Q=(1,0), P3=(0,1)` on the same curve, although P+Q=O and the triple sum is P3. |
| Any xi=0 in the t chart | Excluded: `Ci(0,ti)=b != 0`; the chart does not represent the finite two-torsion point `(0,sqrt(b))`. |
| Target T=O | Outside the finite-triple model. Handle `P2=-P1`, including the unique finite two-torsion point. Identity/inverse controls are included in the native tests. |
| Chaining links | Carry compatible full-point orientations between links, and explicitly represent intermediate O and chart-boundary branches. This directory does not establish higher-arity completeness. |

All counterexamples and stated test outcomes are producer observations. [pcc:unverified]

## Reproduction and coverage

```
cargo test --release --manifest-path research/orientation_yx_validation_20261115/Cargo.toml -- --nocapture
```

No external crate dependency. Rust 1.100.0; tests run locally as development checks, not a timing experiment. Fields are `F2[z]/(z²+z+1)`, `F2[z]/(z³+z+1)`, and `F2[z]/(z⁴+z+1)`. On each, coefficients `a in {0,1,z}`, `b in {1,z}` give six curves, 18 total. Arithmetic validates inverses for every nonzero element. Point triples are ordered and include repetition; counts below overlap. [pcc:unverified]

| Control | Recorded coverage |
| --- | ---: |
| All affine triples | 24,782 |
| Nonzero-x hybrid triples | 19,664 |
| Triples with repeated x | 9,518 |
| Triples containing x=0 | 5,118 |
| Wrong-sign S3 roots rejected by oriented model | 1,560 |
| S3 roots with no rational lifts | 1,112 |
| Native tests passing | 6 / 6 |

Raw output: [unit-tests.log](unit-tests.log). Test source: [validation.rs](validation.rs). These counts have not received independent signed verification. [pcc:unverified]

The report and explanatory diagram are [report.pdf](report.pdf), [report.tex](report.tex), and [model.svg](model.svg). Existing IC scoreboards and performance charts are unchanged: this work adds correctness models and counterexamples, with no new cost measurement or promoted result.

## Sources checked

- `src/cryptanalysis/binary_semaev.rs`, pinned base `f651965dba321339f0b7615769bfa3e46230df67`: binary S3 Vieta derivation, trace caveat, and sign ambiguity. This directory tests the mathematical formula rather than importing that implementation.
- `research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md`, same base: distinguish point coordinates, tuple invariants and retained membership equations; avoid assigning a solver cost to a coordinate identity.
- Kosters–Yeo, *Notes on summation polynomials*, [primary full text](https://arxiv.org/html/1503.08001v2), sections 2, 4, 5: algebraic-closure root semantics, binary trace constraints, first-fall degree versus degree of regularity. Used to separate algebraic identities, rationality and solver claims.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, [2004/031](https://eprint.iacr.org/2004/031), cited by the repo and Kosters–Yeo. Its PDF was not independently read in this session; no claim here relies on an uninspected theorem beyond the Vieta derivation written above.
- Sources checked (KB): unavailable. No KB MCP was loaded; `bash scripts/research-kb status` exited 127 because no executable exists. Fallback scope was the above repository files, branch names, code searches for orientation / summation / y/x / hybrid and the primary Kosters–Yeo text. The search establishes no absence or novelty claim.
- Follow-up prior-art read: `research/toy_binary_formula_audit_20260924/{README.md,PROTOCOL.md,audit.py}` distinguishes polynomial zeros from rational relations and uses a fixed coefficient/conjugacy audit. Historical Python evidence was read but not executed or extended. The new controls test orientation and rational lifts in native Rust; they do not reissue that audit's counts.
