import Init

/-! Finite arithmetic for the saved degree-two isogeny transfer.
Curve-map identities are checked in the Sage receipts, not formalized here. -/
namespace Isogenous2Arithmetic

def r : Nat := 185302017921931
theorem r_is_odd : r % 2 = 1 := by decide
theorem r_coprime_two : Nat.Coprime r 2 := by decide
theorem inverse_two : (2 * ((r + 1) / 2)) % r = 1 := by decide
theorem source_order : 270 * r = 50031544838921370 := by decide
theorem scalar_nonzero : 12345 % r = 12345 := by decide
theorem positive_order_mod_four : 218 % 4 = 2 := by decide
theorem negative_order_mod_four : 270 % 4 = 2 := by decide
theorem positive_prime_order : 218 = 2 * 109 := by decide
theorem orbit_bridge_cells : 5 * 2 = 10 := by decide

theorem composite_transfer_injective
    {A B C : Type} (φ : A → B) (ψ : B → A) (T : A → C)
    (dbl : A → A)
    (hdual : ∀ x, ψ (φ x) = dbl x)
    (hdouble : Function.Injective dbl)
    (hT : Function.Injective T) :
    Function.Injective (fun x => T (ψ (φ x))) := by
  intro x y h
  apply hdouble
  calc
    dbl x = ψ (φ x) := (hdual x).symm
    _ = ψ (φ y) := hT h
    _ = dbl y := hdual y

end Isogenous2Arithmetic
