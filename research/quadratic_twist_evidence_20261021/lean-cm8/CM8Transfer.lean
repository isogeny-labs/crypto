import SignedCache
/-! Actual F13 elliptic points and full order-seven scalar transport.
No generic geometric isogeny theorem is assumed by this finite bridge. -/
namespace CM8Transfer
set_option maxRecDepth 1000000
set_option maxHeartbeats 0
open WeierstrassCurve.Affine
abbrev k := ZMod 13
instance : Fact (Nat.Prime 13) := ⟨by decide⟩
abbrev E : WeierstrassCurve.Affine k := TwistEmbedding.short 1 4
abbrev T : WeierstrassCurve.Affine k := TwistEmbedding.short 4 6
instance : E.IsElliptic := by
  constructor
  exact isUnit_iff_ne_zero.mpr (by decide)
instance : T.IsElliptic := by
  constructor
  exact isUnit_iff_ne_zero.mpr (by decide)
def ep (x y : k) (h : y^2=x^3+x+4) : E.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ E).ne_zero).mp (by simpa [equation_iff,E,TwistEmbedding.short] using h))
def tp (x y : k) (h : y^2=x^3+4*x+6) : T.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ T).ne_zero).mp (by simpa [equation_iff,T,TwistEmbedding.short] using h))
def fx (x : k) : k := 12*(x+2/(x-10))
def fy (x y : k) : k := 8*y*(1-2/(x-10)^2)
theorem image_equation : ∀ x y : k, y^2=x^3+x+4 → x≠10 →
    (fy x y)^2=(fx x)^3+4*fx x+6 := by decide +kernel

def phi : E.Point → T.Point
  | .zero => 0
  | .some x y h => if hx : x=10 then 0 else
      tp (fx x) (fy x y) (image_equation x y (by
        simpa [equation_iff,E,TwistEmbedding.short] using h.left) hx)
def P : E.Point := ep 0 2 (by decide)
def U : T.Point := tp 8 11 (by decide)
def K : E.Point := ep 10 0 (by decide)
theorem zero_image : phi 0=0 := rfl
theorem kernel_image : phi K=0 := by decide +kernel
theorem generator_image : phi P=U := by decide +kernel
theorem source_order : (7:ℕ) • P=0 ∧ P≠0 := by decide +kernel
theorem target_order : (7:ℕ) • U=0 ∧ U≠0 := by decide +kernel
theorem scalar_transport : ∀ n : Fin 7, phi (n.val • P)=n.val • U := by decide +kernel
theorem subgroup_additive : ∀ i j : Fin 7,
    phi (i.val • P+j.val • P)=phi (i.val • P)+phi (j.val • P) := by decide +kernel
theorem target_scalar_injective : ∀ i j : Fin 7,
    i.val • U=j.val • U ↔ i=j := by decide +kernel
theorem actual_source_reduction (Q : E.Point) (hQ : ∃ i : Fin 7, Q=i.val • P)
    (n : Fin 7) : phi Q=n.val • U ↔ Q=n.val • P := by
  obtain ⟨i,rfl⟩ := hQ
  rw [scalar_transport]
  constructor
  · intro h
    have hi := (target_scalar_injective i n).mp h
    rw [hi]
  · intro h
    rw [←scalar_transport,h,scalar_transport]
theorem no_short_scaling : ∀ c : k, ¬(c^4=4 ∧ c^6=8) := by decide +kernel
/-- Check the necessary coefficient equations for every admissible short-
Weierstrass coordinate change over the actual field. -/
theorem no_admissible_short_change : ∀ u r s t : k, u≠0 →
    ¬(2*u^5*s=0 ∧ 2*u^3*t=0 ∧ u^4*s^2=3*u^4*r ∧
      u^6+2*u^2*s*t=3*u^2*r^2+4*u^2 ∧ 4*u^6+t^2=r^3+4*r+6) := by
  decide +kernel
#print axioms no_admissible_short_change
/-- Polynomial only; no field denominator or group hypothesis is hidden. -/
theorem cleared_map_identity {R : Type*} [CommRing R] (u A B : R) :
    (u^3+A*u^2+B*u)*(u^2-B)^2*u^2 =
      (u^2+A*u+B)^3*u^3 - 2*A*(u^2+A*u+B)^2*u^4 +
      (A^2-4*B)*(u^2+A*u+B)*u^5 := by ring
def mainBase : Fin 2 → E.Point := ![ep 0 2 (by decide),ep 7 4 (by decide)]
def twistBase : Fin 2 → T.Point := ![tp 6 5 (by decide),tp 11 4 (by decide)]
def signed {G : Type*} [AddCommGroup G] (s : Fin 2) (q : G) : G :=
  if s=0 then q else -q
theorem main_first_exact_lifts : ∀ i j s t : Fin 2,
    signed s (mainBase i)+signed t (mainBase j)=P ↔
      (i=1 ∧ j=1 ∧ s=0 ∧ t=0) := by decide +kernel
theorem twist_first_exact_lifts : ∀ i j s t : Fin 2,
    signed s (twistBase i)+signed t (twistBase j)=U ↔
      (i=0 ∧ j=1 ∧ s=0 ∧ t=0) ∨
      (i=1 ∧ j=0 ∧ s=0 ∧ t=0) ∨
      (i=1 ∧ j=1 ∧ s=1 ∧ t=1) := by decide +kernel
theorem twist_rows_independent : (0:ZMod 7)*1-(-2)*1≠0 := by decide
theorem twist_base_log_solution : ∀ a b : ZMod 7,
    (-2*b=1 ∧ a+b=1) ↔ (a=5 ∧ b=3) := by decide +kernel
#print axioms main_first_exact_lifts
#print axioms twist_first_exact_lifts
#print axioms twist_rows_independent
#print axioms twist_base_log_solution
#print axioms image_equation
#print axioms zero_image
#print axioms kernel_image
#print axioms generator_image
#print axioms source_order
#print axioms target_order
#print axioms scalar_transport
#print axioms subgroup_additive
#print axioms target_scalar_injective
#print axioms actual_source_reduction
#print axioms no_short_scaling
#print axioms cleared_map_identity
end CM8Transfer
