# Rectangular matrix unit rescaling

Above structures:Mathlib Matrix over arbitrary Semiring R,finite intermediate type beta,Units R. Objects are rectangular L:Matrix alpha beta R and P:Matrix beta gamma R plus a family of actual scalar units. No custom matrix multiplication or syntax encoding.

Existing laws as stated:Matrix.diagonal_mul_diagonal converts diagonal product to pointwise product;Units.val_inv gives each value times inverse1;Matrix.diagonal_one closes identity;Matrix.mul_assoc and Matrix.one_mul then cancel the inserted middle unit pair. Bounded Data/Matrix/Mul source review found these required component laws;no claim that a direct wrapper is absent from all Mathlib. KB unavailable fallback explicit.

Missing composed law:generic rectangular product invariance under L*diagonal(values) and diagonal(inverses)*P. Its owner is CryptoLean,first consumer current actual retained-node rescaling. No group/field law or finite-sum induction is re-derived.

Assembly probe:compile named generic module and audit actual theorem once at end. Finite intermediate beta may be empty;no finiteness of outer alpha/gamma or commutativity of R is required. Unit inverse is bundled;there is no assumption2 invertible in a finite target.

Inheritance table:matrix product/identity/diagonal from Matrix;inverse/value contracts from Units;product coherence from associativity. Only component laws above;no unfolding of matrix finite sums,no new limits or compiled trust.

Actual instance later:R=Q[X],beta=MomentLabels u k,units are C-images of rational2^q on odd labels and1 on even. This is NOT an integralRationals[X] or characteristic2^p unit construction. Actual right integrality uses proved Contentq separately,while admission/creation/mode/public cost remain open. [pcc:unverified]
