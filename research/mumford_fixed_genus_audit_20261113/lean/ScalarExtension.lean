import MumfordCantorGroup

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section
abbrev Base := MumfordSmoothnessAudit.K
abbrev BasePair := MumfordSmoothnessAudit.ReducedPair
variable {F : Type*} [Field F]

def curve (F : Type*) [Field F] : F[X] := X^5-X

/-- All reduced Mumford pairs of the actual fixed quintic, not an image list. -/
@[ext] structure Pair (F : Type*) [Field F] where
  u : F[X]
  v : F[X]
  monic_u : u.Monic
  degree_u : u.natDegree≤2
  degree_v : v.degree<u.degree
  equation : u∣v^2-curve F

theorem curve_map (ι : Base →+* F) :
    MumfordSmoothnessAudit.curvePolynomial.map ι=curve F := by
  simp [MumfordSmoothnessAudit.curvePolynomial,curve]

theorem curve_properties (ι : Base →+* F) :
    (curve F).Monic ∧ (curve F).natDegree=5 ∧ (curve F).Separable := by
  rw [← curve_map ι]
  exact ⟨MumfordSmoothnessAudit.curve_monic.map ι,
    (natDegree_map_eq_of_injective ι.injective _).trans MumfordSmoothnessAudit.curve_degree,
    MumfordSmoothnessAudit.curve_separable.map⟩

def extend (ι : Base →+* F) (P : BasePair) : Pair F where
  u := P.u.map ι
  v := P.v.map ι
  monic_u := P.monic_u.map ι
  degree_u := by rw [natDegree_map_eq_of_injective ι.injective]; exact P.degree_u
  degree_v := by
    rw [degree_map_eq_of_injective ι.injective,degree_map_eq_of_injective ι.injective]
    exact P.degree_v
  equation := by
    obtain ⟨q,hq⟩ := P.equation
    refine ⟨q.map ι,?_⟩
    rw [← Polynomial.map_mul,← hq]
    simp only [Polynomial.map_sub,Polynomial.map_pow,curve_map]

theorem extend_injective (ι : Base →+* F) : Function.Injective (extend ι) := by
  intro P Q h
  apply MumfordSmoothnessAudit.ReducedPair.ext
  · exact Polynomial.map_injective ι ι.injective (congrArg Pair.u h)
  · exact Polynomial.map_injective ι ι.injective (congrArg Pair.v h)

theorem extend_v_zero (ι : Base →+* F) (P : BasePair) : (extend ι P).v=0 := by
  change P.v.map ι=0
  rw [MumfordCantorAudit.every_v_zero,Polynomial.map_zero]

theorem extend_splits (ι : Base →+* F) (P : BasePair) (h : P.u.Splits) :
    (extend ι P).u.Splits := h.map ι

theorem v_natDegree_le (P : Pair F) : P.v.natDegree≤2 := by
  by_cases hv : P.v=0
  · simp [hv]
  · have hd : P.v.degree<(P.u.natDegree:WithBot ℕ) := by
      simpa only [Polynomial.degree_eq_natDegree P.monic_u.ne_zero] using P.degree_v
    exact (Nat.le_of_lt ((Polynomial.natDegree_lt_iff_degree_lt hv).mpr hd)).trans P.degree_u

def encode (P : Pair F) : (Fin 3→F)×(Fin 3→F) :=
  (fun i=>P.u.coeff i,fun i=>P.v.coeff i)

theorem encode_injective : Function.Injective (encode (F:=F)) := by
  intro P Q h
  apply Pair.ext
  · apply Polynomial.ext
    intro i
    by_cases hi : i<3
    · exact congrFun (congrArg Prod.fst h) ⟨i,hi⟩
    · have hp : P.u.natDegree < i := by have hh:=P.degree_u;omega
      have hq : Q.u.natDegree < i := by have hh:=Q.degree_u;omega
      rw [coeff_eq_zero_of_natDegree_lt hp,coeff_eq_zero_of_natDegree_lt hq]
  · apply Polynomial.ext
    intro i
    by_cases hi : i<3
    · exact congrFun (congrArg Prod.snd h) ⟨i,hi⟩
    · have hp : P.v.natDegree < i := by have hh:=v_natDegree_le P;omega
      have hq : Q.v.natDegree < i := by have hh:=v_natDegree_le Q;omega
      rw [coeff_eq_zero_of_natDegree_lt hp,coeff_eq_zero_of_natDegree_lt hq]

instance pairFinite [Finite F] : Finite (Pair F) :=
  Finite.of_injective encode encode_injective

noncomputable instance pairFintype [Finite F] : Fintype (Pair F) := Fintype.ofFinite _

def Smooth (P : Pair F) : Prop := P.u.Splits

theorem ambient_strict_count [Finite F] (B : Pair F) (hB : ¬Smooth B) :
    Nat.card {P : Pair F // Smooth P}<Nat.card (Pair F) := by
  classical
  rw [Nat.card_eq_fintype_card,Nat.card_eq_fintype_card]
  apply Fintype.card_lt_of_injective_not_surjective Subtype.val Subtype.val_injective
  intro hs
  obtain ⟨P,hP⟩ := hs B
  exact hB (hP ▸ P.property)

#print axioms curve_properties
#print axioms extend_injective
end
end MumfordFixedGenusAudit
