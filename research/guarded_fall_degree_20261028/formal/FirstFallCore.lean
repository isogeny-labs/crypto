import GuardedModel

/-!+The low-degree top-map injectivity argument and explicit degree-four
syzygy used by the ordinary first-fall proof. These statements do not yet
define the weighted syzygy module or its Koszul quotient, so they are not
advertised as a complete Lean definition/proof of first fall = 4.+-/
namespace FirstFallCore
noncomputable section
open MvPolynomial GridLower

variable {K : Type*} [Field K]

def guardTop : P K := X 4*(X 1-X 2)
def pairTop : P K := X 1^2+X 1*X 2+X 2^2

theorem guardTop_ne_zero : (guardTop : P K) ≠ 0 := by
  intro h
  have he := congrArg (eval (fun j : Fin 5 => if j = 1 ∨ j = 4 then (1 : K) else 0)) h
  simpa [guardTop] using he

/-- There is no nonzero relation involving scalar multiples of the cubic
tops and any polynomial multiple of the quadratic guard top. This is
stronger than the degree-at-most-three top-map injectivity needed. -/
theorem cubic_guard_top_injective (a b c : K) (L : P K)
    (h : C a*X 1^3+C b*X 2^3+C c*X 3^3+L*guardTop = 0) :
    a = 0 ∧ b = 0 ∧ c = 0 ∧ L = 0 := by
  have ha : a = 0 := by
    have he := congrArg (eval (fun j : Fin 5 => if j = 1 then (1 : K) else 0)) h
    simpa [guardTop] using he
  have hb : b = 0 := by
    have he := congrArg (eval (fun j : Fin 5 => if j = 2 then (1 : K) else 0)) h
    simpa [guardTop] using he
  have hc : c = 0 := by
    have he := congrArg (eval (fun j : Fin 5 => if j = 3 then (1 : K) else 0)) h
    simpa [guardTop] using he
  have hL : L*guardTop = 0 := by simpa [ha,hb,hc] using h
  exact ⟨ha,hb,hc,(mul_eq_zero.mp hL).resolve_right guardTop_ne_zero⟩

/-- The explicit homogeneous top relation of degree four. -/
theorem degree_four_top_relation :
    (X 4 : P K)*X 1^3-X 4*X 2^3-pairTop*guardTop = 0 := by
  unfold pairTop guardTop
  ring

/-- The lifted relation has the displayed quadratic consequence for a
general monic membership cubic. -/
theorem lifted_guard_relation (s₁ s₂ s₃ : K) :
    let x : P K := X 1
    let y : P K := X 2
    let v : P K := X 4
    let g := fun t : P K => t^3-C s₁*t^2+C s₂*t-C s₃
    let Q := x^2+x*y+y^2-C s₁*(x+y)+C s₂
    v*(g x-g y)-Q*(v*(x-y)-1) = Q := by
  dsimp
  ring

end
end FirstFallCore

#print axioms FirstFallCore.guardTop_ne_zero
#print axioms FirstFallCore.cubic_guard_top_injective
#print axioms FirstFallCore.degree_four_top_relation
#print axioms FirstFallCore.lifted_guard_relation
