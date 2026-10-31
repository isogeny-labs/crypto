import ReducedS3

/-! Complete membership roots, rather than merely sound zero tests. -/
namespace KnownRoots

def membership {k : Type*} [CommRing k] (roots : List k) (x : k) : k :=
  (roots.map (fun a => x-a)).prod

theorem product_root_iff {k : Type*} [CommRing k] [IsDomain k]
    (roots : List k) (x : k) : membership roots x=0 ↔ x ∈ roots := by
  induction roots with
  | nil => simp [membership]
  | cons a roots ih =>
    change (x-a)*membership roots x=0 ↔ x ∈ a::roots
    rw [mul_eq_zero,ih]
    simp only [sub_eq_zero,List.mem_cons]

theorem expanded_two_factors {k : Type*} [CommRing k] (a b x : k) :
    (x-a)*(x-b)=x^2-(a+b)*x+a*b := by ring

theorem quadratic_roots_iff {k : Type*} [CommRing k] [IsDomain k]
    (a b x : k) : x^2-(a+b)*x+a*b=0 ↔ x=a ∨ x=b := by
  rw [← expanded_two_factors]
  simp only [mul_eq_zero,sub_eq_zero]

theorem constrained_pairs_iff {k : Type*} [CommRing k] [IsDomain k]
    (a b x y : k) (h : k → k → k) :
    (x^2-(a+b)*x+a*b=0 ∧ y^2-(a+b)*y+a*b=0 ∧ h x y=0) ↔
    ((x=a ∨ x=b) ∧ (y=a ∨ y=b) ∧ h x y=0) := by
  rw [quadratic_roots_iff,quadratic_roots_iff]

theorem main_actual_input (x y z : ZMod 43) :
    (x^2-17*x+9=0 ∧ y^2-17*y+9=0 ∧
      ReducedS3.reduced 1 0 z 17 9 x y=0) ↔
    ((x=4 ∨ x=13) ∧ (y=4 ∨ y=13) ∧
      ReducedS3.reduced 1 0 z 17 9 x y=0) := by
  have h := constrained_pairs_iff (4:ZMod 43) 13 x y
    (ReducedS3.reduced 1 0 z 17 9)
  norm_num at h ⊢
  exact h

theorem twist_actual_input (x y z : ZMod 43) :
    (x^2-32*x+40=0 ∧ y^2-32*y+40=0 ∧
      ReducedS3.reduced 4 0 z 32 40 x y=0) ↔
    ((x=15 ∨ x=17) ∧ (y=15 ∨ y=17) ∧
      ReducedS3.reduced 4 0 z 32 40 x y=0) := by
  have h := constrained_pairs_iff (15:ZMod 43) 17 x y
    (ReducedS3.reduced 4 0 z 32 40)
  norm_num at h ⊢
  exact h

#print axioms product_root_iff
#print axioms expanded_two_factors
#print axioms quadratic_roots_iff
#print axioms constrained_pairs_iff
#print axioms main_actual_input
#print axioms twist_actual_input
end KnownRoots
