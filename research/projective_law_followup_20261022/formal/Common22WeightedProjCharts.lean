import Common22WeightedProjHead
set_option autoImplicit false
noncomputable section
namespace Common22WeightedProjCharts
open MvPolynomial Common22WeightedProjHead
variable {K : Type*} [Field K]
def affineValues (x y : K) : Fin 3 → K := ![x,y,1]
def infinityValues (z w : K) : Fin 3 → K := ![1,w,z]
theorem affine_equation (a3 a2 a1 a0 x y : K) :
    eval₂Hom (RingHom.id K) (affineValues x y) (equation a3 a2 a1 a0) =
      y^2 - (x^5 + a3*x^3 + a2*x^2 + a1*x + a0) := by
  simp [equation, affineValues, eval₂Hom]
theorem infinity_equation (a3 a2 a1 a0 z w : K) :
    eval₂Hom (RingHom.id K) (infinityValues z w) (equation a3 a2 a1 a0) =
      w^2 - z*(1 + a3*z^2 + a2*z^3 + a1*z^4 + a0*z^5) := by
  simp [equation, infinityValues, eval₂Hom]
  ring
#print axioms affine_equation
#print axioms infinity_equation
end Common22WeightedProjCharts
