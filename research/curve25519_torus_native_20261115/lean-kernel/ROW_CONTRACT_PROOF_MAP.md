# Same-query concrete instance gate

1. Structure: existing Mathlib `ZMod 5`, `Matrix`, vector/dot products and
   finite sums on `Fin 2` and `Fin 1`.
2. Laws used as stated: `add_neg_cancel` in the assembly probe; existing
   finite-decidability and vector/dot-product operations in the closed witness. This is a
   concrete witness, not a new generic dependence/probability law.
3. Instance: two identical rows (1), vector (1,-1), and tags (b,b) over F5.
   The endpoint quantifies b but fixes the whole matrix/witness instance.
4. Assembly probe: anonymous use of `add_neg_cancel` over ZMod5. The named
   certificate checks a nonzero vector and the two concrete cancellations
   for every element of the finite field by kernel-checked `decide`.
5. Missing: arbitrary query-group aggregation, probability/retention laws,
   actual curve rows and any complete batch-solver cost. The example only
   demonstrates why a shared tag is not an independent row tag.
6. Inheritance: ZMod ring operations and finite-index/matrix structure from
   Mathlib. Finite arithmetic uses its existing decidable operations; no custom axiom or limit
   override, no recreation of a field or generic vector law.

The new file extends the project without modifying the frozen
KernelInstance.lean or dependency manifest. Gate-v1 remains its historical
receipt; the new complete-project receipt is gate-v2. No theorem about an
actual curve oracle's output contract follows from this concrete witness.
