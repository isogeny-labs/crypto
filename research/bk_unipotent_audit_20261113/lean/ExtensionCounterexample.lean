import FrobeniusPeriod
import Mathlib.FieldTheory.Finite.GaloisField

namespace BKUnipotentAudit
noncomputable section
open FrobeniusPointDescentAudit FrobeniusFiniteExtensionAudit
attribute [local instance] closureDecidableEq

def fullTorsionImage {A : Type*} [Field A] [Algebra F A] (ι : A →ₐ[F] K) : Prop := by
  classical
  exact ∀ P : geometricCurve.Point, (3:ℕ) • P=0 →
    P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) ι)

theorem chosen_field_criterion {A : Type*} [Field A] [Fintype A] [Algebra F A]
    (ι : A →ₐ[F] K) (k : ℕ) (hc : Fintype.card A=7^k) :
    fullTorsionImage ι ↔ 3 ∣ k := by
  classical
  let : Algebra A K := ι.toRingHom.toAlgebra
  let : IsScalarTower F A K := IsScalarTower.of_algHom ι
  have hc' : Fintype.card A=Fintype.card F^k := by simpa only [ZMod.card] using hc
  have h := finite_extension_torsion_iff_of_card (F:=F) (K:=A) (B:=K) curve 3 k hc'
  have hm : (fun Q : (curve.baseChange A).Point =>
      WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId A K) Q)=
      (fun Q => WeierstrassCurve.Affine.Point.map (W':=curve) ι Q) := by
    funext Q
    cases Q <;> rfl
  simpa only [hm,fullTorsionImage] using h.trans (all_torsion_fixed_iff_three_dvd k)

theorem galois_embedding_exists (k : ℕ) : Nonempty (GaloisField 7 k →ₐ[F] K) := by
  exact ⟨IsAlgClosed.lift (R:=F) (S:=GaloisField 7 k) (M:=K)⟩

theorem galois_field_criterion (k : ℕ) (hk : 0<k) (ι : GaloisField 7 k →ₐ[F] K) :
    fullTorsionImage ι ↔ 3 ∣ k := by
  classical
  let : Fintype (GaloisField 7 k) := Fintype.ofFinite _
  have hc : Fintype.card (GaloisField 7 k)=7^k := by
    rw [← Nat.card_eq_fintype_card]
    exact GaloisField.card 7 k (Nat.ne_of_gt hk)
  exact chosen_field_criterion ι k hc

theorem every_nonmultiple_counter (k : ℕ) (hk : ¬3 ∣ k) :
    3 ∣ 7^k-1 ∧ ¬∀ P : geometricCurve.Point, (3:ℕ) • P=0 → sigma^[k] P=P := by
  exact ⟨cyclotomic_divisibility k,fun h => hk ((all_torsion_fixed_iff_three_dvd k).mp h)⟩

theorem infinite_image_counterexamples (n : ℕ) :
    Nonempty (GaloisField 7 (3*n+1) →ₐ[F] K) ∧
    3 ∣ 7^(3*n+1)-1 ∧
    ∀ ι : GaloisField 7 (3*n+1) →ₐ[F] K, ¬fullTorsionImage ι := by
  refine ⟨galois_embedding_exists _,cyclotomic_divisibility _,?_⟩
  intro ι h
  have hd := (galois_field_criterion (3*n+1) (by omega) ι).mp h
  omega

theorem all_extension_boundary_certificate :
    curve.Δ≠0 ∧ Nat.card curve.Point=9 ∧
    Nat.card {P : geometricCurve.Point // (3:ℕ) • P=0}=9 ∧
    Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧ 3^2 ∣ Nat.card curve.Point ∧
    TorsionBoundaryAudit.embeddingDegree 7 3=1 ∧
    (∀ k : ℕ, (∀ P : geometricCurve.Point, (3:ℕ) • P=0 → sigma^[k] P=P) ↔ 3 ∣ k) ∧
    (∀ k : ℕ, 3 ∣ 7^k-1) ∧
    (∀ (A : Type) [Field A] [Fintype A] [Algebra F A] (ι : A →ₐ[F] K) (k : ℕ),
      Fintype.card A=7^k → (fullTorsionImage ι ↔ 3 ∣ k)) ∧
    (∀ k : ℕ, 0<k → Nat.card (GaloisField 7 k)=7^k ∧
      Nonempty (GaloisField 7 k →ₐ[F] K) ∧
      ∀ ι : GaloisField 7 k →ₐ[F] K, fullTorsionImage ι ↔ 3 ∣ k) ∧
    (∀ n : ℕ, Nonempty (GaloisField 7 (3*n+1) →ₐ[F] K) ∧
      3 ∣ 7^(3*n+1)-1 ∧
      ∀ ι : GaloisField 7 (3*n+1) →ₐ[F] K, ¬fullTorsionImage ι) := by
  refine ⟨TorsionBoundaryAudit.bkCurve_nonsingular,TorsionBoundaryAudit.bkCurve_cardinality,
    actual_geometric_cardinality,by decide,by decide,by decide,?_,embedding_degree_one,
    all_torsion_fixed_iff_three_dvd,cyclotomic_divisibility,?_,?_,infinite_image_counterexamples⟩
  · rw [TorsionBoundaryAudit.bkCurve_cardinality]
    decide
  · intro A _ _ _ ι k hc
    exact chosen_field_criterion ι k hc
  · intro k hk
    exact ⟨GaloisField.card 7 k (Nat.ne_of_gt hk),galois_embedding_exists k,
      galois_field_criterion k hk⟩

#print axioms chosen_field_criterion
#print axioms galois_field_criterion
#print axioms infinite_image_counterexamples
#print axioms all_extension_boundary_certificate

end
end BKUnipotentAudit
