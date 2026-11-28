import ActualDescent
import Mathlib.RingTheory.AdjoinRoot

namespace EdwardsDescentAudit
open MvPolynomial
noncomputable section

def descentIdeal : Ideal Poly := Ideal.span ({f0,f1} : Set Poly)
def triangularIdeal : Ideal Poly := Ideal.span ({X 0^2,X 1-1} : Set Poly)

theorem linear_certificate : (X 1-1 : Poly)=3*(f0-f1) := by
  unfold f0 f1
  linear_combination (1-X 1)*(CharP.cast_eq_zero Poly 5)

theorem square_certificate : (X 0^2 : Poly)=3*f1-(X 1+1)*(X 1-1) := by
  unfold f1
  linear_combination -(X 0^2+X 1^2+2)*(CharP.cast_eq_zero Poly 5)

theorem reverse_certificates :
    f0=2*X 0^2+(2*X 1+4)*(X 1-1) ∧
      f1=2*X 0^2+2*(X 1+1)*(X 1-1) := by
  constructor
  · unfold f0
    linear_combination (CharP.cast_eq_zero Poly 5)
  · unfold f1
    linear_combination (CharP.cast_eq_zero Poly 5)

theorem ideals_equal : descentIdeal=triangularIdeal := by
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro p hp
    have hx : (X 0^2 : Poly) ∈ triangularIdeal := Ideal.subset_span (by simp)
    have hy : (X 1-1 : Poly) ∈ triangularIdeal := Ideal.subset_span (by simp)
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl
    · rw [reverse_certificates.1]
      exact Ideal.add_mem _ (Ideal.mul_mem_left _ _ hx) (Ideal.mul_mem_left _ _ hy)
    · rw [reverse_certificates.2]
      exact Ideal.add_mem _ (Ideal.mul_mem_left _ _ hx) (Ideal.mul_mem_left _ _ hy)
  · apply Ideal.span_le.mpr
    intro p hp
    have h0 : f0 ∈ descentIdeal := Ideal.subset_span (by simp)
    have h1 : f1 ∈ descentIdeal := Ideal.subset_span (by simp)
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl
    · rw [square_certificate,linear_certificate]
      exact Ideal.sub_mem _ (Ideal.mul_mem_left _ _ h1)
        (Ideal.mul_mem_left _ _ (Ideal.mul_mem_left _ _ (Ideal.sub_mem _ h0 h1)))
    · rw [linear_certificate]
      exact Ideal.mul_mem_left _ _ (Ideal.sub_mem _ h0 h1)

abbrev A := Poly ⧸ descentIdeal
def nilpotentPolynomial : Polynomial F := Polynomial.X^2
abbrev Dual := AdjoinRoot nilpotentPolynomial

theorem root_square : (AdjoinRoot.root nilpotentPolynomial)^2=0 := by
  have h := AdjoinRoot.mk_self (f:=nilpotentPolynomial)
  change AdjoinRoot.mk nilpotentPolynomial (Polynomial.X^2)=0 at h
  simpa only [map_pow,AdjoinRoot.mk_X] using h

def evaluate : Poly →ₐ[F] Dual :=
  MvPolynomial.aeval (fun i : Fin 2 => if i=0 then AdjoinRoot.root nilpotentPolynomial else 1)

theorem evaluate_kills (p : Poly) (hp : p ∈ descentIdeal) : evaluate p=0 := by
  have hsub : descentIdeal ≤ RingHom.ker evaluate.toRingHom := by
    rw [ideals_equal]
    apply Ideal.span_le.mpr
    intro q hq
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hq
    rcases hq with rfl | rfl
    · change evaluate (X 0^2)=0
      simpa [evaluate] using root_square
    · change evaluate (X 1-1)=0
      simp [evaluate]
  exact hsub hp

def forward : A →ₐ[F] Dual := Ideal.Quotient.liftₐ descentIdeal evaluate evaluate_kills
def coordinate (i : Fin 2) : A := Ideal.Quotient.mk descentIdeal (X i)

theorem quotient_square : coordinate 0^2=0 := by
  rw [coordinate,←map_pow,Ideal.Quotient.eq_zero_iff_mem,ideals_equal]
  exact Ideal.subset_span (by simp)

theorem quotient_second : coordinate 1=1 := by
  apply sub_eq_zero.mp
  change Ideal.Quotient.mk descentIdeal (X 1-1)=0
  rw [Ideal.Quotient.eq_zero_iff_mem,ideals_equal]
  exact Ideal.subset_span (by simp)

def backward : Dual →ₐ[F] A :=
  AdjoinRoot.liftAlgHom nilpotentPolynomial (Algebra.ofId F A) (coordinate 0) (by
    simpa [nilpotentPolynomial] using quotient_square)

@[simp] theorem forward_coordinate (i : Fin 2) :
    forward (coordinate i)=if i=0 then AdjoinRoot.root nilpotentPolynomial else 1 := by
  change evaluate (X i)=_
  simp [evaluate]

@[simp] theorem backward_root :
    backward (AdjoinRoot.root nilpotentPolynomial)=coordinate 0 := by simp [backward]

def quotientEquiv : A ≃ₐ[F] Dual :=
  AlgEquiv.ofAlgHom forward backward
    (by apply AdjoinRoot.algHom_ext; simp)
    (by
      apply Ideal.Quotient.algHom_ext F
      apply MvPolynomial.algHom_ext
      intro i
      change backward (forward (coordinate i))=coordinate i
      fin_cases i
      · simp
      · simp [quotient_second])

theorem quotient_dimension_two : Module.finrank F A=2 := by
  rw [quotientEquiv.toLinearEquiv.finrank_eq]
  change Module.finrank F (Polynomial F ⧸ Ideal.span {nilpotentPolynomial})=2
  rw [finrank_quotient_span_eq_natDegree]
  simp [nilpotentPolynomial]

theorem quotient_finite : Module.Finite F A := by
  have hd : nilpotentPolynomial.Monic := Polynomial.monic_X_pow 2
  have := hd.finite_adjoinRoot
  exact Module.Finite.equiv quotientEquiv.toLinearEquiv.symm

/-- A finite-dimensional coordinate algebra over a field is Artinian;
this records actual zero-dimensionality, without treating a finite set of
rational zeros as a substitute for scheme dimension. -/
theorem quotient_artinian : IsArtinianRing A := by
  have := quotient_finite
  exact IsArtinianRing.of_finite F A

theorem unique_solution (x y : F) :
    (MvPolynomial.eval ![x,y] f0=0 ∧ MvPolynomial.eval ![x,y] f1=0) ↔
      (x=0 ∧ y=1) := by
  constructor
  · intro h
    have hy : y-1=0 := by
      simpa [h.1,h.2] using congrArg (MvPolynomial.eval ![x,y]) linear_certificate
    have hx : x^2=0 := by
      simpa [h.2,hy] using congrArg (MvPolynomial.eval ![x,y]) square_certificate
    have hm : x*x=0 := by simpa [pow_two] using hx
    exact ⟨(mul_eq_zero.mp hm).elim id id,sub_eq_zero.mp hy⟩
  · rintro ⟨rfl,rfl⟩
    norm_num [f0,f1]
    decide +kernel

end
end EdwardsDescentAudit
