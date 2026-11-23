# Symbolic union of colored interpolation kernels

Prospective extension after source f51eb1fbfe80a4243a38115da8c3bd8e010590b9
build001 passes. The first source implements the original finite state-count
endpoint without statement or proof-limit changes. Its floor002 probe passes;
floor001 was a PATH-only127 failure. Final all-source/axiom audit is pending.

Before any symbolic-union declaration, use this exact contract. Structure
above: the same Field F, Polynomial F, bounded-kernel Set and Fin t x Fin b
records as the previous prospective plan. Existing theorem used as stated:
Finset.prod_eq_zero_iff, finite function choice and mem_kernel. Object:
for each color i multiply the actual point-evaluation linear forms
u(x_ij)+y_ij*v(x_ij). Preserve the existential choice within each color.

Assembly probe: over an arbitrary field and an arbitrary function
f : Fin t -> Fin b -> F, (forall i, product_j f i j = 0) iff there exists
a choice c with forall i, f i (c i)=0. Close it with the product-zero law and
Classical.choose, then specialize the factors to polynomial evaluations.
The final actual-coordinate probe uses the existing Mathlib curve predicate.

Missing generic components: definition/interface of the per-color product;
product-zero iff there exists a record in the color; equivalence of all
product constraints with existence of a colored prefix whose genuine
kernel contains (u,v). These laws belong in the shared polynomial module.

| Operation | Inherited law | New glue |
|---|---|---|
| Evaluation linear forms | Polynomial.eval over Field F | reuse kernel expression |
| Product over color | Finset.univ.prod | definition/interface |
| Product vanishes | Finset.prod_eq_zero_iff | specialize |
| One choice per color | Classical.choose | combine existential factors |
| Selected support | mem_prefixSupport | recover kernel membership |
| Bounded coefficients | mem_kernel | retain both original degree caps |

Proof chain: color product zero iff one factor is zero; forall colors this
holds iff there is a choice function; selected-support membership identifies
each chosen record; append the unchanged u/v bounds. This proves a compact
symbolic membership representation, not an algorithm for extracting a
nonzero pair. It does not imply that a union is a linear module.
Handle b=0/t=0 with the actual empty-product/function conventions.

No new benchmark or numerical run. Keep the previous source/build/floor
receipts intact. One lake process at a time; unchanged standard-axiom gate
after the final source. No custom axioms, proof-limit overrides or native
evaluation. The new theorem does not change or weaken prefix_kernel_count.
