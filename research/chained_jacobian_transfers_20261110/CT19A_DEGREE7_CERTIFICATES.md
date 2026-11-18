# CT19a — fixed F101 degree-7 elliptic kernel certificates (development candidate)

Date: 2026-11-11. Branch:
`research/ct19-degree7-polynomial-controls-20261111`, stacked on
`crypto` PR #3877. All mathematics/outputs here have **producer-only,
unsigned standing**: [pcc:unverified]. This is not CT20's genus-2
correspondence, a signed theorem, an end-to-end solver, or a cost result.

## Sources checked and anti-duplication

- Existing queue and current head: `research/chained_jacobian_transfers_20261110/{QUEUE.md,CHECKPOINT.md,CT19_PLAN.md}`.
  CT17 complete in `cryptanalysis` PR #1481; CT18 complete in PR #1482.
- Existing independent kernel verifier: `src/cryptanalysis/isogeny_walk/kernel.rs`;
  existing division and rational maps:
  `isogeny_algos/src/find/divpoly.rs`,
  `isogeny_algos/src/kernel/kohel.rs`,
  `isogeny_algos/src/curve.rs`. No new map algorithm is proposed.
- Standard background: Howe–Leprévost–Poonen, *Large torsion subgroups
  of split Jacobians of curves of genus two or three* (2000),
  https://arxiv.org/abs/math/9809210; Frey–Kani gluing surveyed by
  Sijsling, https://jrsijsling.eu/notes/gluing.pdf.
  Reuses standard 2-gluing and odd-degree isogeny facts; no novelty claim.
- Shared research-KB unavailable to this connector; encrypted lead vault not
  opened. This is **not** a complete prior-art/atlas audit.
- The current `isogeny_algos::find::divpoly::kernel_polys` accepts candidates
  after only four sampled x checks. The fixed fixture checks below are
  deliberately independent of that weak acceptance heuristic.

## Concrete public inputs

Over F101, the two already accepted shifted-109 quotient models are

```
E1 : Y^2 = X^3 + X + 46                  #E1(F101) = 109
E2 : Y^2 = X^3 + 44 X + 21              #E2(F101) = 113
```

The second comes from `Y²=u³+13u²+33u+5` by `X=u+38`;
since 13/3 = 38 in F101, this is an exact coordinate isomorphism.
The separate test enumerates all 101 x-coordinates for the long/short
equation bridge.

The Frobenius characteristic polynomials on E[7] are:

```
E1: T² - (-7)T + 101 ≡ (T-2)(T-5) (mod 7)
E2: T² - (-11)T + 101 ≡ (T-4)(T-6) (mod 7)
```

Both have distinct roots. Each eigenline determines one F101-rational
degree-seven cyclic kernel. Eigenline *does not* mean pointwise-rational
7-torsion. The fixed degree-three kernels and normalized Kohel targets
below are **candidate certificates** that the committed Rust test is
required to validate coefficientwise and on all rational points:

| Source short (a,b) | monic kernel h(x), coefficients low-to-high | normalized target (a',b') | raw normalized dual kernel low-to-high |
|---|---|---|---|
| (1,46) | (9,76,20,1) | (5,39) | (49,69,62,1) |
| (1,46) | (72,100,35,1) | (26,58) | (58,26,58,1) |
| (44,21) | (77,86,71,1) | (13,42) | (42,13,8,1) |
| (44,21) | (59,46,8,1) | (69,50) | (29,88,45,1) |

A normalized raw dual `psi` targets the model obtained from the
original source by scaling `(a,b)→(7^4 a,7^6 b)`.
The actual dual `psi_dual(P)` is obtained by scaling its output
`(x,y)→(7^-2 x,7^-3 y)`. Require both compositions to be
`[7]` on all 109/113 rational points.

The exact polynomial identity for `x'=N(x)/D(x)` is checked without any
probability or testing only on rational x:

```
S := N' D - N D'
f_source S² = D (N³ + a' N D² + b' D³).
```

Also require `h|psi_7`, `deg h=3`, squarefree, coprime to source cubic;
`gcd(N,D)=1`, correct codomain and both full point-group composites,
additivity over *all* ordered rational source pairs, and complete
enumeration of the two rational degree-seven kernels. This is stronger
than the existing four-random-x sampler. No unknown logarithms are
used in the certification.

Run (requires Rust toolchain, not present in this connector environment):

```sh
cargo test --manifest-path isogeny_algos/Cargo.toml --test ct19_degree7 -- --nocapture
cargo clippy --manifest-path isogeny_algos/Cargo.toml --test ct19_degree7 -- -D warnings
cargo fmt --manifest-path isogeny_algos/Cargo.toml -- --check
```

These Rust tests have **not been compiled or run here**. The four data
rows and consequences are unverified assertions until those runs are
captured and an independent PCC verdict exists. Do not infer that an
imported producer's point checks count as independent verification.

## CT20 implication: 2-torsion compatibility is automatic, not a search

Let `psi:E1[2]→E2[2]` be the original F101-defined anti-isometry
underlying the *existing* degree-(2,2) gluing and suppose both listed
degree-seven maps `phi_i:E_i→E_i'` are genuinely defined over F101.
Because gcd(7,2)=1, the restrictions `phi_i|E_i[2]` are isomorphisms.
Define

```
psi' = phi_2|[2] ◦ psi ◦ (phi_1|[2])^-1.
```

Weil-pairing functoriality gives `e_2(phi_i(P),phi_i(Q)) =
e_2(P,Q)^7 = e_2(P,Q)`. Thus the transported `psi'` is again a
Galois-equivariant anti-isometry. Its graph is maximally isotropic,
and the product `phi_1 × phi_2` carries the original graph **onto**
the transported graph. By functoriality of quotient, it induces a
well-defined F101-rational degree-49 isogeny

```
(E1 × E2)/graph(psi) ---> (E1' × E2')/graph(psi').
```

With the induced principal polarizations this has type (7,7).
This group-theoretic fact does not depend on a genus-2 equation.
For these candidate targets the j-invariant pairs are
`(91,89), (91,92), (87,89), (87,92)`, pairwise unequal,
so the *generic* irreducible 2-gluing condition is satisfied.
This last irreducibility/genus-two identification, exact coordinate
gluing, and equality to a specific native Richelot target must still
be checked by a separate geometric implementation. In particular
the theorem **does not** give an executable Mumford-coordinate map
without the missing conversions, nor an ECDLP speedup.

**Sharper next step:** use the four frozen kernel/codomain pairs to
construct four explicit target genus-2 equations and proper normalizations
using the existing `isogeny_algos::genus2::glue` implementation, then
verify the `(7,7)` dual identity on all 12,317 rational Jacobian classes
via CT17's existing decomposition and the independent
`isogeny_walk::kernel::verify_kernel` for each elliptic factor.
The main remaining uncertainty is actual **computable divisor transport
and model normalization**, not abstract 2-torsion compatibility.

## Limits / failed or unattempted gates

- Current repository gate requires PCC claims and independent signed CI
  standing. This branch has no new signed PCC verdict, no local Rust
  execution receipt and no completed Lean/Picard formalization.
- The parent #3877 retains a frozen append-only ledger-prefix conflict and
  missing archive/kernel-clock gates: do not merge, rebase away evidence,
  bypass proof gate, or force-update it.
- No research KB, unlocked atlas, S3 checkpoint token or live spending
  allowance here. No remote compute, benchmark or solver attempted.
- If the test rejects any fixture, retain the failure and update this note;
  the chart and gluing inference remain conditional on the exact identities.
