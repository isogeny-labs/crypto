import Std

/- A structural dependency of the upper-bound witness only.
   The square-zero algebra, pair-product identity, nontriviality and lower-bound
   filtered Koszul argument are NOT formalized by this theorem. -/
namespace PrimeFirstFall

theorem square_zero_remainder_annihilates
    {R : Type} (mul : R → R → R) (zero L g r : R)
    (assoc : ∀ a b c, mul (mul a b) c = mul a (mul b c))
    (mul_zero : ∀ a, mul a zero = zero)
    (remainder : mul L g = mul g r)
    (square_zero : mul r r = zero) :
    mul (mul L L) g = zero := by
  calc
    mul (mul L L) g = mul L (mul L g) := assoc L L g
    _ = mul L (mul g r) := by rw [remainder]
    _ = mul (mul L g) r := (assoc L g r).symm
    _ = mul (mul g r) r := by rw [remainder]
    _ = mul g (mul r r) := assoc g r r
    _ = mul g zero := by rw [square_zero]
    _ = zero := mul_zero g

#print axioms square_zero_remainder_annihilates
end PrimeFirstFall
