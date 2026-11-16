import Mathlib.Algebra.Polynomial.Div
import Mathlib.Algebra.Polynomial.Splits
import Mathlib.FieldTheory.Separable
import Mathlib.Tactic

/-! Actual polynomial pairs at arbitrary genus bounds.
No geometric Picard/Jacobian identification or arbitrary group is assumed. -/
namespace MumfordFamilyPairsAudit
noncomputable section
open Polynomial
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev K := ZMod 3

structure ReducedPair (g : ℕ) (f : K[X]) where
  u : K[X]
  v : K[X]
  monic_u : u.Monic
  degree_u : u.natDegree≤g
  degree_v : v.degree<u.degree
  divides : u∣f-v^2

@[ext] theorem pair_ext {g : ℕ} {f : K[X]} {P Q : ReducedPair g f}
    (hu : P.u=Q.u) (hv : P.v=Q.v) : P=Q := by
  cases P
  cases Q
  cases hu
  cases hv
  rfl

def identity (g : ℕ) (f : K[X]) : ReducedPair g f :=
  ⟨1,0,monic_one,by simp,by simp,one_dvd _⟩

def Smooth {g : ℕ} {f : K[X]} (P : ReducedPair g f) : Prop := P.u.Splits

theorem v_natDegree_le {g : ℕ} {f : K[X]} (P : ReducedPair g f) :
    P.v.natDegree≤g := by
  by_cases hv : P.v=0
  · simp [hv]
  · have hd : P.v.degree<(P.u.natDegree:WithBot ℕ) := by
      simpa only [Polynomial.degree_eq_natDegree P.monic_u.ne_zero] using P.degree_v
    have hn := (Polynomial.natDegree_lt_iff_degree_lt hv).mpr hd
    exact (Nat.le_of_lt hn).trans P.degree_u

def encode (g : ℕ) (f : K[X]) (P : ReducedPair g f) :
    (Fin (g+1)→K)×(Fin (g+1)→K) :=
  (fun i=>P.u.coeff i,fun i=>P.v.coeff i)

theorem encode_injective (g : ℕ) (f : K[X]) : Function.Injective (encode g f) := by
  intro P Q h
  apply pair_ext
  · apply Polynomial.ext
    intro i
    by_cases hi : i<g+1
    · exact congrFun (congrArg Prod.fst h) ⟨i,hi⟩
    · have hp : P.u.natDegree < i := by have hh:=P.degree_u;omega
      have hq : Q.u.natDegree < i := by have hh:=Q.degree_u;omega
      rw [Polynomial.coeff_eq_zero_of_natDegree_lt hp,
        Polynomial.coeff_eq_zero_of_natDegree_lt hq]
  · apply Polynomial.ext
    intro i
    by_cases hi : i<g+1
    · exact congrFun (congrArg Prod.snd h) ⟨i,hi⟩
    · have hp : P.v.natDegree < i := by have hh:=v_natDegree_le P;omega
      have hq : Q.v.natDegree < i := by have hh:=v_natDegree_le Q;omega
      rw [Polynomial.coeff_eq_zero_of_natDegree_lt hp,
        Polynomial.coeff_eq_zero_of_natDegree_lt hq]

instance reducedPairFinite (g : ℕ) (f : K[X]) : Finite (ReducedPair g f) :=
  Finite.of_injective (encode g f) (encode_injective g f)

noncomputable instance reducedPairFintype (g : ℕ) (f : K[X]) : Fintype (ReducedPair g f) :=
  Fintype.ofFinite _

theorem nonsmooth_strict_count {g : ℕ} {f : K[X]} (B : ReducedPair g f)
    (hB : ¬Smooth B) : Nat.card {P : ReducedPair g f // Smooth P}<Nat.card (ReducedPair g f) := by
  classical
  rw [Nat.card_eq_fintype_card,Nat.card_eq_fintype_card]
  apply Fintype.card_lt_of_injective_not_surjective Subtype.val Subtype.val_injective
  intro hs
  obtain ⟨P,hP⟩ := hs B
  exact hB (hP ▸ P.property)

#print axioms reducedPairFinite
#print axioms nonsmooth_strict_count
end
end MumfordFamilyPairsAudit
