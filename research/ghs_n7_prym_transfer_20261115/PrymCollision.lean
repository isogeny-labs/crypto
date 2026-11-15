import Init

/-!
Algebraic boundary of the symmetric W1/Prym proposal. This file proves
the formal implication from kernel closure to classical membership,
and the abstract injectivity consequence of an adjoint identity.
It does not formalize finite-field kernel dimensions, the curve
geometry, the Hess map, or an attack-cost claim.
-/

namespace GHSN7Prym

variable {K C A B : Type}

theorem symmetric_coefficient_in_kernel
    (V : K → Prop) (add : K → K → K) (sq : K → K)
    (scale : C → K → K) (c d : C) (gamma : K)
    (sq_closed : ∀ x, V x → V (sq x))
    (scale_closed : ∀ e x, V x → V (scale e x))
    (add_closed : ∀ x y, V x → V y → V (add x y))
    (hgamma : V gamma) :
    V (sq (add (scale c (sq gamma)) (scale d gamma))) := by
  apply sq_closed
  apply add_closed
  · exact scale_closed c (sq gamma) (sq_closed gamma hgamma)
  · exact scale_closed d gamma hgamma

theorem injective_on_of_adjoint_double
    (f : A → B) (adj : B → A) (double : A → A) (H : A → Prop)
    (adjoint_identity : ∀ x, H x → adj (f x) = double x)
    (double_injective : ∀ x y, H x → H y → double x = double y → x = y)
    (x y : A) (hx : H x) (hy : H y) (hxy : f x = f y) : x = y := by
  apply double_injective x y hx hy
  calc
    double x = adj (f x) := (adjoint_identity x hx).symm
    _ = adj (f y) := congrArg adj hxy
    _ = double y := adjoint_identity y hy

end GHSN7Prym
