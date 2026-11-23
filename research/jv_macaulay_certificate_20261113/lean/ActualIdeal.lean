import LiteralQuadratics

namespace JVMacaulayAudit
open MvPolynomial
noncomputable section

theorem combination_mem {R : Type*} [CommRing R] (I : Ideal R)
    {u v w : R} (hu : u∈I) (hv : v∈I) (hw : w∈I) (a b c : R) :
    a*u+b*v+c*w∈I :=
  I.add_mem (I.add_mem (I.mul_mem_left a hu) (I.mul_mem_left b hv)) (I.mul_mem_left c hw)

theorem quartic_mem_of_source_mem {R : Type*} [CommRing R] [CharP R 101]
    (I : Ideal R) (x y h : R) (h0 : f0 x y h∈I)
    (h1 : f1 x y h∈I) (h2 : f2 x y h∈I) : h^4∈I := by
  have hgy : gy x y h∈I := by
    rw [(quadratic_certificates x y h).1]
    simpa using combination_mem I h0 h1 h2 1 83 14
  have hgxy : gxy x y h∈I := by
    rw [(quadratic_certificates x y h).2.1]
    exact I.add_mem (I.mul_mem_left _ h1) (I.mul_mem_left _ h2)
  have hgx : gx x y h∈I := by
    rw [(quadratic_certificates x y h).2.2]
    exact I.add_mem (I.mul_mem_left _ h1) (I.mul_mem_left _ h2)
  have hr1 : r1 x y h∈I := by
    unfold r1
    exact I.sub_mem (I.add_mem (I.mul_mem_left _ hgy) (I.mul_mem_left _ hgxy))
      (I.mul_mem_left _ hgx)
  have hr2 : r2 x y h∈I := by
    unfold r2
    exact I.sub_mem (I.add_mem (I.mul_mem_left _ hgy) (I.mul_mem_left _ hgxy))
      (I.mul_mem_left _ hgx)
  have hcy : cy x y h∈I := by
    rw [(cubic_certificates x y h).1]
    exact I.add_mem (I.mul_mem_left _ hr1) (I.mul_mem_left _ hr2)
  have hcx : cx x y h∈I := by
    rw [(cubic_certificates x y h).2]
    exact I.add_mem (I.mul_mem_left _ hr1) (I.mul_mem_left _ hr2)
  rw [quartic_certificate x y h]
  exact I.mul_mem_left _ (I.sub_mem (I.sub_mem (I.sub_mem
    (I.mul_mem_left _ hgx) (I.mul_mem_left _ hcx)) (I.mul_mem_left _ hcx))
    (I.mul_mem_left _ hcy))

private instance prime101 : Fact (Nat.Prime 101) := ⟨by decide⟩
abbrev F := ZMod 101
abbrev Poly := MvPolynomial (Fin 3) F
def sourceIdeal : Ideal Poly :=
  Ideal.span ({f0 (X 0) (X 1) (X 2),f1 (X 0) (X 1) (X 2),
    f2 (X 0) (X 1) (X 2)} : Set Poly)

theorem source_quartic_mem : (X 2^4 : Poly)∈sourceIdeal := by
  apply quartic_mem_of_source_mem sourceIdeal (X 0) (X 1) (X 2)
  all_goals exact Ideal.subset_span (by simp)

abbrev AffinePoly := MvPolynomial (Fin 2) F
def affineIdeal : Ideal AffinePoly :=
  Ideal.span ({f0 (X 0) (X 1) 1,f1 (X 0) (X 1) 1,
    f2 (X 0) (X 1) 1} : Set AffinePoly)

theorem actual_affine_ideal_is_unit : affineIdeal=⊤ := by
  rw [Ideal.eq_top_iff_one]
  have h := quartic_mem_of_source_mem affineIdeal (X 0) (X 1) 1
    (Ideal.subset_span (by simp))
    (Ideal.subset_span (by simp))
    (Ideal.subset_span (by simp))
  simpa using h

theorem actual_affine_no_geometric_solution {L : Type*} [Field L] [CharP L 101]
    (x y : L) : ¬ (f0 x y 1=0 ∧ f1 x y 1=0 ∧ f2 x y 1=0) := by
  rintro ⟨h0,h1,h2⟩
  have hz := quartic_mem_of_source_mem (⊥ : Ideal L) x y 1
    (by simpa using h0) (by simpa using h1) (by simpa using h2)
  simp at hz

end
end JVMacaulayAudit
