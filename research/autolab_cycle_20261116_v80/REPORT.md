# C80: exact field-aware polynomial squaring

The candidate computes diagonal squares and only one of each symmetric cross product, doubled by field addition in odd characteristic. Binary cross terms vanish. This replaces three modular self-products in a fresh copy of the guarded sparse-power kernel; inverse guards and generic fallback remain frozen. Monic reduction is unchanged, including repeated-root quotients and zero/nonunit bases. The construction is classical; no novelty claim is made.

Independent checks passed560 arbitrary square remainders across14 field configurations,336 modular powers,120 Taylor/Hasse repeated-root cases at actual NIST exponents,5 complete large-field factor-product root cases and5 full conditional S3 output regressions against the frozen C78 backend. Fields include characteristic3, GF9, even binary extensions, GF(2^83) and P192/P224/P256/P384/P521. Every remainder, root vector, shift candidate and degree12 polynomial matched. Empty/sparse inputs and zero roots were retained.

Scalar operation probes include the same remainder work in both methods. For12-coefficient inputs modulo a degree12 polynomial, P521 changed from287 scalar multiplications to209 multiplications plus12 squares (221 total); GF(2^83) changed from287 to143 multiplications plus12 squares (155 total). GF9 changed from274 to196 multiplications plus12 squares (208 total). The full28 operation probes remain in raw evidence. A scalar square and multiplication need not cost the same, and these counts are not runtime or end-to-end ratios.

No noisy speed panel was rerun. C79 still has one failed identical-code A/A pair, so this cycle is correctness/work reduction only. The helper remains experimental; no generic production promotion or qualified performance claim follows. Full independent-source search, matched genuine F4/F5/F6,20 hard completed instances and50x remain open.

Next: test the fixed startup-amortization ladder without deleting failing sizes. Then freeze a new cost panel comparing the copied C78 kernel and square-specialized kernel, with every field, setup and memory retained. Binary semilinear trace jumps remain an independent algebraic route. Do not multiply old observed gains by operation-count savings to claim a composite speedup.

One frozen90-second,4-GiB,one-core experiment passed through resume/run-ready. All previous sources, inputs, failures and raw evidence are preserved. Scientific goal active.
