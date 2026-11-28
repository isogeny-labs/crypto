# Tensor padding over existing finite sums

Structure: arbitrary CommSemiring R, finite real indices I and dummy indices D, finite spin type Sigma; explicit decidable equality instances. Tensors are functions (I->Sigma)->R, not a new graph/ring formalism.

Law reused as stated: Finset.prod_boole converts product of actual dummy unary indicators to if assignment equals d0 then1else0; Equiv.sumArrowEquivProdArrow splits extended assignments; Fintype.sum_equiv and sum_prod_type move the finite sum; sum_ite_eq closes the unique fixed assignment. Exact Mathlib source read before edit; KB unavailable. No sum/product induction.

Missing composed law: generic contraction invariant under these concrete row/column padding operations, for first consumer actual unused packet slots. Owner CryptoLean, not study specialization. Includes empty I/D and arbitrary fixed dummy spins d0; no positive-cardinality or field hypothesis.

Assembly probe001: dummyWeight_eq -> expand actual padded tensors -> change finite sum indices by sumArrow equivalence -> sum over product -> collapse one dummy assignment -> original contraction. Every line names existing law. No new resource/trust settings.

Instance later: I=subtype occupied slots, D=subtype unused slots, Sigma=Bool, d0=false, same coefficient ring as actual target. Separate graph endpoint/degree/symbol/size/creation/bit-cost and original Boolean deformed-kernel interface remain unproved. [pcc:unverified]
