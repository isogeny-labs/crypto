# Prime31 B8 public-only D9 positive extraction protocol

**Status:** prospective source and correctness gate, [pcc:unverified]. This does not alter the registered negative D9 results or supply a complete index-calculus solve.

## Fixed input and comparison

Use the existing ordinary cofactor-one curve over `p=2147483647`, its first eight admitted base abscissas `(0,1,2,3,5,6,8,9)`, and the registered public positive residual `R=(1911579686,548820096)`. The test-only signed support and previously saved coefficient vector are excluded from the extractor's inputs. The two previously registered public negatives and their degree-nine unit identities remain independent controls; do not rerun or retune them to choose a favorable seed. The generic B8 signed-support control is a comparison after extraction, never a discovery oracle.

The public RR9 equations are the eight coefficients below degree eight of `D_R(X)V(X)^2-(X-r)A(X)^2-2sA(X)V(X)-H_B(X)`, with monic cubic `V`, degree-at-most-three `A`, and seven free coefficients. Reuse the existing exact sparse D9 Macaulay constructor: 27,456 rows, 11,440 columns, affine degree at most nine. Keep coefficient and monomial order identical to the registered negative-certificate source.

## One bounded positive attempt

Use deterministic seed `0x0d990003` and nonzero diagonal `D`. Compute `2N=22880` scalar moments for `M^T D M` and Berlekamp–Massey. If the returned degree is `N` and the final coefficient is zero, form the degree-`N-1` quotient polynomial of the connection polynomial by `t`; apply it to the same seeded Krylov starting vector. Admit the resulting vector only if direct multiplication by **every original sparse Macaulay row** gives zero and its constant-monomial coordinate is nonzero. Normalize it, read the seven degree-one coordinates, then require all eight original RR9 equations and the independently implemented direct polynomial identity to vanish.

The final correctness check reconstructs the eight signed full points from the recovered coefficient vector and checks their sum equals the public residual on the original curve. This checker may use point arithmetic after extraction. It must not pass signs, support, known target scalar or saved coefficients into the Krylov computation. A scalar BM degree below `N`, nonzero final coefficient, failed null-vector check, zero constant coordinate, or failed original-equation/full-point check is an **inconclusive or rejected attempt**, retained with its counters. Do not infer a positive root from a singular Gram matrix alone.

No automatic seed search or cap increase. Record wall time, memory, matrix dimensions, sparse nonzeros, moment and projection applications, field-operation accounting, output coefficients if any, and all verification flags. Run source-pinned focused tests before any claimed result. The full task needs a held-out natural-target yield/rank panel and a complete single-target IC/rho comparison; this one positive control only determines whether the public-only extraction mechanism works on its registered input.

## Sources checked

- Existing original source: `native/examples/prime31_rr9_public_macaulay.rs`, `prime31_rr9_d9_blackbox_core.rs`, `RR9_D9_BLACKBOX_PROTOCOL_20261117.md`, `RR9_D9_BLACKBOX_RESULT_20261117.md`, and `native/M8_NATURAL_F4_CI_OUTCOME_20261115.md` at `ce396efc71`.
- The scalar Wiedemann/Berlekamp–Massey operation and the prior one-sided certificate are the existing method, not a new theorem. Prime-field index-calculus context: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Yokoyama et al., *J. Math. Cryptol.* 14 (2020), DOI 10.1515/jmc-2019-0029, whose naive-method lower bound has statistical assumptions.
