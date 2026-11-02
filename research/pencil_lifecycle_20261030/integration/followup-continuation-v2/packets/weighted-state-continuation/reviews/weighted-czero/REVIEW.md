# Independent C0 weighted field review

Fresh Lean replay passed all eight named modules. The axiom readback covers 14 main declarations, seven producer controls and six independent controls (27 total), using only propext, Classical.choice and Quot.sound. Scoped field-formula correspondence holds with the assumptions below. This packet records technical evidence; publication/workflow completion remains separate.

## Source correspondence

The bound native snapshot comes from crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs. normalize, lines 533–566, decodes DegreeOne by x/z² and y/(z⁵w). add22_weighted, lines 899–1023, crosses the other input's q/r/s/t weights with powers 2/4/3/5 and includes the opposite w on s/t. The C=0 branch's literal B, A, x, y, z and w=1 associations agree with WeightedCZero.payload. This is field-value correspondence, not a word-level or execution-order theorem.

Write Z=z1*z2 and W=w1*w2. Source-first scaling gives Bcross=Z⁸B, Across=Z⁹WA and Ccross=Z⁷WC. The output is xnum=Z¹⁸W²*xn, ynum=Z⁴⁵W⁵*yn, z=Z⁹WB, w=1. Division by z² and z⁵w cancels the weights. The proof modules establish those identities and the correct ordinate sign.

WeightedPoint.reconstruct covers arbitrary stored D2 coordinates with nonzero z and w, by reconstructing them from their normalized coordinates. Thus composition is not restricted to inputs originally created by weight. WeightedGuards proves the normalized/crossed B-nonzero and C-zero equivalences using their stated nonzero factors. These algebraic guards do not establish caller control flow.

WeightedPoint.weighted_on_curve composes decoding with CZeroOutput.czero_output. It requires a field, four nonzero weights, B nonzero, C zero, monic f of degree five with coefficient four zero, and both input Mumford residual divisibilities. Its conclusion is nonzero output weights and y²=f(x). Divisor-class addition is a separate unproved bridge here.

## Replay and adversarial controls

Exact copied producer modules were built in snapshot with the pinned Lean toolchain by run.sh. before.sha256/after.txt bind inputs and selected retained dependencies before/after. Dependency binaries remain trusted; neither all Mathlib dependencies nor the kernel implementation were independently rebuilt/rechecked. Per-module native exits and complete output are retained.

Independent rational controls cover nonunit mixed-sign weights yielding decoded (10,291), the literal output z=20*21⁹ and w=1, omitted-w failure, zero-w singular output, reconstruction of arbitrary raw coordinates, and a nonzero-C witness. Controls use kernel tactics, not native_decide.

The first reviewer output-z control expected 10*21⁹ and failed. Inspection found B=-2 and W=-10, giving 20*21⁹. The first source and failure receipt remain in reviewer-attempt1; only the reviewer expectation was corrected. Producer inputs were unchanged. Final six reviewer controls pass.

## Review independence and boundaries

The reviewer inspected native source and producer status/plan, wrote a local source-first plan, and then inspected proof bodies and replayed them. This was not a fully blind derivation and the plan was not precommitted. Dispatch preceded producer publication; root is recording that sequencing deviation and publishing through T-1225. No claim of a completed prescribed review workflow is made. Exact reviewed bytes are in reviewed-source.sha256; root must bind them to the published archive.

Unproved transitions are native caller validity, divisor-class sum, word arithmetic/counters/scheduling and compiler correctness. No scientific experiment or new measurement was performed. No cost ratio is inferred. The source-linked status JSON, receipts, controls and diagram support independent assessment of these scoped results.
