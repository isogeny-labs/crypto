The research objective is a faster relation-solving method for elliptic curves over prime fields, starting with p = 2^31 − 1. The active candidate combines a full Dickson fibre with rational auxiliary S3 equations. Its correctness and complete cost are being checked separately. No asymptotic speedup has yet been established.

# Current work

Iteration six has a written all-node quadratic representation and independent mathematical review. The frozen Rust verifier covers rational signed chains, a nonrational twist counterexample and the leaf-ordering boundary. Its first original-lock build stopped on malformed inherited repository fixtures before new tests ran; the failure is retained while the source is repaired and the same gate is rerun. [Build receipt](https://github.com/isogeny-labs/crypto/actions/runs/45096187034).

# Exact quadratic candidate

For every internal node, orient its two children from the known parent point using a slope. The root is the specified nonzero rational target; all auxiliary abscissae must belong to the prime field.

```math
\begin{aligned}
\lambda_v^2&=x_l+x_r+x_v,\\
2y_v\lambda_v&=(x_l-x_v)(x_r-x_v)-(3x_v^2+a),\\
y_w&=\lambda_v(x_w-x_v)-y_v.
\end{aligned}
```

Together with the Dickson squaring ladders, the system has mk + 3m − 5 variables and mk + 3m − 4 quadratic equations for m ≥ 2 and k ≥ 1. Written proofs establish rational-tree soundness and completeness for a freely ordered left chain with m ≤ n and nonzero total. A fixed balanced tree does not inherit that completeness. Explicit field guards are required if roots are sought over an algebraic closure. The elimination cost remains open.

[Mathematical note and proof](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/iteration6/MATHEMATICAL_NOTE.md) · [Illustrated report](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/iteration6/REPORT.pdf)

# Checkpoints and evidence

- [Main research PR 4928](https://github.com/isogeny-labs/crypto/pull/4928)

- [Companion symmetric pair oracle PR 4924](https://github.com/isogeny-labs/crypto/pull/4924)

- [Machine-readable state](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/STATE.json)

- [Continuation instructions](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/CONTINUATION.md)

- [Iteration six correctness scope](https://github.com/isogeny-labs/crypto/blob/4b29204b844e806932a6473b25a34bae5dcc12cc/research/prime31_summation_compression_20261122/iteration6/SCOPE.md)

- [Earlier original-lock correctness run](https://github.com/isogeny-labs/crypto/actions/runs/45089569480)

# Results retained

| Iteration | Result | Consequence |
| --- | --- | --- |
| 1 | Exact denominator-free Dickson S3 recurrence; source-bound correctness CI. | The measured field-multiplication cell loses to sparse product. |
| 2 | Written full-curve bidegree argument. | Bounded scalar bidegree is unavailable in that representation. |
| 3 | Conditional output-sensitive tree count. | The standard exact empty-node predicate still processes the fibre. |
| 4 | Exact zero-count trace and full signed sum dictionary. | Dense state or tuple preprocessing dominates the specified methods. |
| 5 | Exact bucket obstruction and conditional setup/query exponent budget. | A different, nonhomomorphic implicit representation is needed. |
| 6 | Written exact quadratic representation, rational-tree purity and free-order comb completeness; fixed Rust controls frozen. | Initial original-root build failed on inherited source syntax before new tests; repair and replay in progress. |

All new mathematical and computational claims retain their recorded unverified PCC standing. Written proofs, native arithmetic certificates and producer CI have distinct scopes; none supplies a signed asymptotic solver proof.

# Next decision

Determine whether the rational quadratic representation admits a provably cheaper exact empty-target solve, including preprocessing, failed queries, rational-root extraction and signed output. For a complete ECDLP advantage, include independent relation rank, linear algebra and an explicit growing prime-field family. A fixed 31-bit field cannot establish an asymptotic exponent.

# Ongoing continuation

The existing hourly research task continues the same PR lanes. Preserve prior results and adverse comparisons, advance one concrete mathematical or correctness gate per iteration, and update this Page and the repository state after confirmed saves.
