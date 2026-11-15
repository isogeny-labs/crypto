module
public import NormPairingRadical
public import NormWronskian
public import Mathlib.Algebra.Polynomial.RingDivision
public import Mathlib.Algebra.Polynomial.Degree.Support
public section
namespace NormPencilFunctional
open Polynomial
variable {K : Type*} [Field K]

/-- The actual quotient moment as a linear functional. -/
noncomputable def moment (F V : Polynomial K) : Polynomial K →ₗ[K] K where
  toFun P := ((P*V) %ₘ F).coeff (F.natDegree-1)
  map_add' P Q := by rw [add_mul,add_modByMonic,coeff_add]
  map_smul' c P := by
    simp only [smul_mul_assoc,smul_modByMonic,coeff_smul,RingHom.id_apply]

theorem momentApply (F V P : Polynomial K) :
    moment F V P=((P*V) %ₘ F).coeff (F.natDegree-1) := by simp [moment]

/-- The supplied actual norm witness forces annihilation of G; it is not assumed. -/
theorem annihilatesTarget (F H f G U V : Polynomial K) (alpha beta c : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hc : c≠0)
    (hnorm : (C alpha*F)^2-f*(C beta*H)^2=C c*G) :
    moment F V G=0 := by
  let mu := moment F V
  have hupper : mu ((C alpha*F)^2)=0 := by
    have hd : F ∣ (C alpha*F)^2*V := ⟨C alpha*C alpha*F*V,by ring⟩
    change (((C alpha*F)^2*V) %ₘ F).coeff (F.natDegree-1)=0
    rw [(modByMonic_eq_zero_iff_dvd hf).mpr hd,coeff_zero]
  have hlower0 : mu (f*(H*H))=0 := by
    exact (NormPairingRadical.lowerFullRadical F H f H U V hf hh hhp hgap hbez
      (le_refl _)).mpr ⟨1,by simp⟩ H (le_refl _)
  have heq : f*(C beta*H)^2=(beta^2) • (f*(H*H)) := by
    simp only [smul_eq_C_mul,map_pow]
    ring
  have hlower : mu (f*(C beta*H)^2)=0 := by
    rw [heq,map_smul,hlower0,smul_zero]
  have hcG : mu (c • G)=0 := by
    rw [smul_eq_C_mul,← hnorm,map_sub,hupper,hlower,sub_self]
  rw [map_smul,smul_eq_mul] at hcG
  exact (mul_eq_zero.mp hcG).resolve_left hc

/-- A linear functional is given by its moments on every bounded polynomial. -/
theorem coefficientExpansion (mu : Polynomial K →ₗ[K] K) (P : Polynomial K)
    (d : ℕ) (hp : P.natDegree≤d) :
    mu P=∑ j ∈ Finset.range (d+1), P.coeff j*mu (X^j) := by
  conv_lhs => rw [P.as_sum_range_C_mul_X_pow' (by omega : P.natDegree<d+1)]
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro j _
  rw [← smul_eq_C_mul,map_smul,smul_eq_mul]

/-- Annihilating a monic target gives the actual leading-eliminated coefficient pencil. -/
theorem coefficientPencil (mu : Polynomial K →ₗ[K] K) (G N : Polynomial K)
    (hg : G.Monic) (hn : N.natDegree≤G.natDegree) (hzero : mu G=0) :
    mu N=∑ j ∈ Finset.range G.natDegree,
      (N.coeff j-N.coeff G.natDegree*G.coeff j)*mu (X^j) := by
  have hexp := coefficientExpansion mu N G.natDegree hn
  have hgexp := coefficientExpansion mu G G.natDegree (le_refl _)
  rw [Finset.sum_range_succ] at hexp hgexp
  rw [hg.coeff_natDegree,one_mul,hzero] at hgexp
  simp_rw [sub_mul,mul_assoc]
  rw [Finset.sum_sub_distrib,← Finset.mul_sum]
  linear_combination hexp-N.coeff G.natDegree*hgexp

/-- The actual trial norm lies in the coefficient pencil determined by the witness moments. -/
theorem normCoefficientPencil (F H f G U V A B : Polynomial K)
    (alpha beta c : K) (hf : F.Monic) (hh : H.Monic) (hg : G.Monic)
    (hhp : 0<H.natDegree) (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hc : c≠0)
    (hnorm : (C alpha*F)^2-f*(C beta*H)^2=C c*G)
    (ha : 2*A.natDegree≤G.natDegree)
    (hb : f.natDegree+2*B.natDegree≤G.natDegree) :
    moment F V (A^2-f*B^2)=∑ j ∈ Finset.range G.natDegree,
      ((A^2-f*B^2).coeff j-
        (A^2-f*B^2).coeff G.natDegree*G.coeff j)*moment F V (X^j) := by
  exact coefficientPencil (moment F V) G (A^2-f*B^2) hg
    (NormWronskian.normDegreeBound f A B G.natDegree ha hb)
    (annihilatesTarget F H f G U V alpha beta c hf hh hhp hgap hbez hc hnorm)

/-- Universal norm polarization; a separate native record checks this exact identity. -/
theorem normPolarization {R : Type*} [CommRing R] (A B Q T f : Polynomial R) :
    (A+Q)^2-f*(B+T)^2-(A^2-f*B^2)-(Q^2-f*T^2)=
      2*(A*Q-f*(B*T)) := by ring
end NormPencilFunctional
#print axioms NormPencilFunctional.momentApply
#print axioms NormPencilFunctional.annihilatesTarget
#print axioms NormPencilFunctional.coefficientExpansion
#print axioms NormPencilFunctional.coefficientPencil
#print axioms NormPencilFunctional.normCoefficientPencil
#print axioms NormPencilFunctional.normPolarization
