import Init

namespace PowerFold
-- Only the homomorphism/product bridge is formalized here. The finite-field
-- root-of-unity factorization and the two-center fiber theorem remain written proofs.
def product {A : Type} (mul : A → A → A) (one : A) : List A → A
  | [] => one
  | x :: xs => mul x (product mul one xs)

theorem map_product {A B : Type} (mulA : A → A → A) (oneA : A)
    (mulB : B → B → B) (oneB : B) (f : A → B)
    (hOne : f oneA = oneB)
    (hMul : ∀ x y, f (mulA x y) = mulB (f x) (f y))
    (xs : List A) :
    f (product mulA oneA xs) = product mulB oneB (xs.map f) := by
  induction xs with
  | nil => exact hOne
  | cons x xs ih =>
    change f (mulA x (product mulA oneA xs)) =
      mulB (f x) (product mulB oneB (xs.map f))
    rw [hMul, ih]

#print axioms map_product
end PowerFold
