# Exact formal scope

Lean source: `ExteriorWitness.lean`. The actual finite source grid is
`node(j)=j-4` for j=0,...,7. `grid` is the full degree8 rational polynomial;
`normalizer` is its derivative evaluated at0. `chart(delta)` is the product
with the delta factor omitted, normalized by this actual derivative and
substituted at T=1+8X. No expected coefficient is an axiom or hypothesis.

Seven named theorems:

| Name | Checked statement |
| --- | --- |
| source_normalizer | The full finite-grid derivative at0 is -144 |
| witness_slots_allowed | (0,0),(2,1) belong to the actual even total two-bit code with coordinate bounds <4 |
| witness_has_passive_axis | Axis1 low bits agree; the signed axis2 difference is1 |
| local_node_value | Actual chart1 value at0 is -5/3 |
| crossing_linear_coefficient | Actual chart2 coefficient1 is40/3, via derivative coefficient theorem |
| passive_product_coefficient | Their exact product is -200/9 |
| passive_product_nonzero | That product is nonzero |

This certifies the finite rational Cauchy/code witness. There is no theorem
here about general a, finite-field rank2(L-1), Gamma composition, fermionic
ordering, mode counts, complete network recursion, ECDLP or asymptotic runtime.
The general analysis is written/unformalized. The code contains ordinary
kernel-checked `decide` for the literal selected-bit arithmetic; it uses no
native_decide, native-trusted gate, custom axiom, sorry, or admitted premise.

Final validation completed: default build and explicit source build,
seven-declaration transitive axiom audit, shared pinned CryptoLean proof
gate and import-inclusive fresh leanchecker all exited0. The fresh receipt
explicitly names ExteriorWitness; no inferred checker result is substituted.

Build failures001 and002 arose from confusing Polynomial.eval_zero (the zero
polynomial) with evaluation at0; the correct bridge is
coeff_zero_eq_eval_zero. Failure004 was missing decidable instance unfolding
for the named finite code predicate; norm_num[evenSlot4] fixed it. Failed
elaborations print compiler-generated sorryAx in their error logs and are
not certificates. Successful final source has seven declarations with only
propext/Classical.choice/Quot.sound, or no axioms. Original failed sources
and logs are preserved under receipts/ with .lean.txt archival extensions.
