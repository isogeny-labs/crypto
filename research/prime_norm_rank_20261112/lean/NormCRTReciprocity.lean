module
public import Mathlib.Algebra.Polynomial.Div
public import Mathlib.Algebra.Polynomial.Degree.Domain
public import Mathlib.RingTheory.Coprime.Basic
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Ring
import Lean.Elab.Tactic.Omega
public section
namespace NormCRTReciprocity
open Polynomial

/-- Universal reconstruction residual, also used by the native expression certificate. -/
theorem reconstructionResidual {R : Type*} [CommRing R]
    (F H P U V Q T : Polynomial R) :
    (P*V-F*Q)*H+(P*U-H*T)*F-P=
      P*(U*F+V*H-1)-F*H*(Q+T) := by ring

variable {K : Type*} [Field K]

/-- Bounded CRT reconstruction from the two genuine congruence conditions. -/
theorem reconstruct (F H P S T : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hc : IsCoprime F H)
    (hs : S.natDegree<F.natDegree) (ht : T.natDegree<H.natDegree)
    (hp : P.natDegree<F.natDegree+H.natDegree)
    (hfs : F ∣ S*H-P) (hht : H ∣ T*F-P) : S*H+T*F=P := by
  let E := S*H+T*F-P
  have hfe : F ∣ E := by
    have he : E=(S*H-P)+T*F := by dsimp [E]; ring
    rw [he]
    exact dvd_add hfs (dvd_mul_left F T)
  have hhe : H ∣ E := by
    have he : E=(T*F-P)+S*H := by dsimp [E]; ring
    rw [he]
    exact dvd_add hht (dvd_mul_left H S)
  have hprod := hc.mul_dvd hfe hhe
  have hsh : (S*H).natDegree<F.natDegree+H.natDegree := by
    have hle : (S*H).natDegree≤S.natDegree+H.natDegree := natDegree_mul_le
    omega
  have htf : (T*F).natDegree<F.natDegree+H.natDegree := by
    have hle : (T*F).natDegree≤T.natDegree+F.natDegree := natDegree_mul_le
    omega
  have hsum : (S*H+T*F).natDegree<F.natDegree+H.natDegree :=
    lt_of_le_of_lt (natDegree_add_le _ _) (max_lt_iff.mpr ⟨hsh,htf⟩)
  have hedeg : E.natDegree<F.natDegree+H.natDegree :=
    lt_of_le_of_lt (natDegree_sub_le _ _) (max_lt_iff.mpr ⟨hsum,hp⟩)
  have hezero : E=0 := by
    by_contra hn
    have hle := natDegree_le_of_dvd hprod hn
    rw [hf.natDegree_mul hh] at hle
    omega
  exact sub_eq_zero.mp hezero

/-- Comparing the top coefficient gives the reciprocal quotient-pairing identity. -/
theorem coefficientReciprocity (F H P S T : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hc : IsCoprime F H)
    (hs : S.natDegree<F.natDegree) (ht : T.natDegree<H.natDegree)
    (hp : P.natDegree<F.natDegree+H.natDegree-1)
    (hfs : F ∣ S*H-P) (hht : H ∣ T*F-P) :
    S.coeff (F.natDegree-1)+T.coeff (H.natDegree-1)=0 := by
  have hrec := reconstruct F H P S T hf hh hc hs ht (by omega) hfs hht
  have hsf : S.natDegree≤F.natDegree-1 := by omega
  have hth : T.natDegree≤H.natDegree-1 := by omega
  have hsh : (S*H).coeff (F.natDegree+H.natDegree-1)=S.coeff (F.natDegree-1) := by
    rw [show F.natDegree+H.natDegree-1=(F.natDegree-1)+H.natDegree by omega]
    rw [coeff_mul_add_eq_of_natDegree_le hsf (le_refl H.natDegree),hh.coeff_natDegree,
      mul_one]
  have htf : (T*F).coeff (F.natDegree+H.natDegree-1)=T.coeff (H.natDegree-1) := by
    rw [show F.natDegree+H.natDegree-1=(H.natDegree-1)+F.natDegree by omega]
    rw [coeff_mul_add_eq_of_natDegree_le hth (le_refl F.natDegree),hf.coeff_natDegree,
      mul_one]
  have hzero : P.coeff (F.natDegree+H.natDegree-1)=0 :=
    coeff_eq_zero_of_natDegree_lt hp
  have hcoeff := congrArg (fun R : Polynomial K =>
    R.coeff (F.natDegree+H.natDegree-1)) hrec
  rw [coeff_add,hsh,htf,hzero] at hcoeff
  exact hcoeff

/-- Canonical remainder instance, with an explicit ordinary Bezout certificate. -/
theorem remainderReciprocity (F H P U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic)
    (hfp : 0<F.natDegree) (hhp : 0<H.natDegree)
    (hbez : U*F+V*H=1)
    (hp : P.natDegree<F.natDegree+H.natDegree-1) :
    ((P*V) %ₘ F).coeff (F.natDegree-1)+
      ((P*U) %ₘ H).coeff (H.natDegree-1)=0 := by
  have hfne : F≠1 := by
    intro h
    rw [h,natDegree_one] at hfp
    omega
  have hhne : H≠1 := by
    intro h
    rw [h,natDegree_one] at hhp
    omega
  have hs := natDegree_modByMonic_lt (P*V) hf hfne
  have ht := natDegree_modByMonic_lt (P*U) hh hhne
  have hfs : F ∣ ((P*V) %ₘ F)*H-P := by
    refine ⟨-P*U-((P*V) /ₘ F)*H, ?_⟩
    rw [modByMonic_eq_sub_mul_div]
    linear_combination P*hbez
  have hht : H ∣ ((P*U) %ₘ H)*F-P := by
    refine ⟨-P*V-((P*U) /ₘ H)*F, ?_⟩
    rw [modByMonic_eq_sub_mul_div]
    linear_combination P*hbez
  exact coefficientReciprocity F H P _ _ hf hh ⟨U,V,hbez⟩ hs ht hp hfs hht

/-- The weighted bridge needed by the lower norm-Hankel block.
Here V is an inverse of f*H modulo F and U is an inverse of F modulo H.
No separability or restriction on the characteristic is required. -/
theorem weightedReciprocity (F H f R U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic)
    (hfp : 0<F.natDegree) (hhp : 0<H.natDegree)
    (hbez : U*F+V*(f*H)=1)
    (hr : R.natDegree<F.natDegree+H.natDegree-1) :
    ((f*R*V) %ₘ F).coeff (F.natDegree-1)+
      ((R*U) %ₘ H).coeff (H.natDegree-1)=0 := by
  have hb : U*F+(f*V)*H=1 := by linear_combination hbez
  have h := remainderReciprocity F H R U (f*V) hf hh hfp hhp hb hr
  simpa only [mul_assoc,mul_comm,mul_left_comm] using h
end NormCRTReciprocity
#print axioms NormCRTReciprocity.reconstructionResidual
#print axioms NormCRTReciprocity.reconstruct
#print axioms NormCRTReciprocity.coefficientReciprocity
#print axioms NormCRTReciprocity.remainderReciprocity
#print axioms NormCRTReciprocity.weightedReciprocity
