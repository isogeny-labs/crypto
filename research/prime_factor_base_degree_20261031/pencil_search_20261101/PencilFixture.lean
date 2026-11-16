import PencilSelector
import SecantFixture
import PencilCanonical

noncomputable section
namespace PencilFixture
open SecantFixture
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

/-- A hand-derived finite fixture, not a prospective construction trial. -/
def shiftedGraph (x : F) (u v : Bool) : F :=
  graph x+(if u then 2 else 0)+(if v then 10 else 0)

def labelMember (x y : F) : Prop :=
  ∃ u v : Bool, y = shiftedGraph x u v ∨ y = -(shiftedGraph x u v)

instance (x y : F) : Decidable (labelMember x y) := by
  unfold labelMember
  infer_instance

def baseX : Finset F := {0, 3, 5, 6, 7, 9, 10, 16}

theorem base_cardinality : baseX.card = 8 := by decide

/-- Exact membership for every rational ordinate, with both signs retained. -/
theorem exact_membership : ∀ x y : F,
    y^2 = curve x → (labelMember x y ↔ x ∈ baseX) := by decide

def signedPoints : Finset (F × F) :=
  Finset.univ.filter (fun p => p.2^2 = curve p.1 ∧ labelMember p.1 p.2)

theorem signed_cardinality : signedPoints.card = 16 := by decide

def positiveFibre (u v : Bool) : Finset (F × F) :=
  Finset.univ.filter (fun p => p.2^2 = curve p.1 ∧ p.2 = shiftedGraph p.1 u v)

/-- Empty/non-full fibres are accepted; the old complete-split assumption
    would have excluded this concrete larger base. -/
theorem fibre_cardinalities :
    (positiveFibre false false).card = 4 ∧
    (positiveFibre true false).card = 1 ∧
    (positiveFibre false true).card = 2 ∧
    (positiveFibre true true).card = 2 := by decide

def code (u v : Bool) : Fin 2 → F := ![if u then 1 else 0, if v then 1 else 0]
theorem code_boolean (u v : Bool) : PencilSelector.Boolean (code u v) := by
  cases u <;> cases v <;> simp [code, PencilSelector.Boolean, Fin.forall_fin_succ]
theorem graph_code (x : F) (u v : Bool) :
    PencilSelector.graph 4 9 9 0 ![2,10] (code u v) x = shiftedGraph x u v := by
  cases u <;> cases v <;>
    simp [code, shiftedGraph, graph, PencilSelector.graph, PencilSelector.offset,
      Fin.sum_univ_succ, add_assoc]

theorem inBase_iff_label (x y : F) :
    PencilSelector.InBase 4 9 9 0 ![2,10] x y ↔ labelMember x y := by
  constructor
  · rintro ⟨e, u, he, hu, hy⟩
    obtain ⟨bits, hb⟩ := (PencilSelector.boolean_iff_bool u).mp hu
    have hueq : u = code (bits 0) (bits 1) := by
      funext i
      fin_cases i <;> simpa [code] using hb _
    rw [hueq, graph_code] at hy
    refine ⟨bits 0, bits 1, ?_⟩
    rcases he with rfl | rfl
    · exact Or.inl (by simpa using hy)
    · exact Or.inr (by simpa using hy)
  · rintro ⟨u, v, hy | hy⟩
    · exact ⟨1, code u v, Or.inl rfl, code_boolean u v, by simpa [graph_code] using hy⟩
    · exact ⟨-1, code u v, Or.inr rfl, code_boolean u v, by simpa [graph_code] using hy⟩

def leftWeights (e : F) : Fin 4 → F := ![e*2, e*10, 0, 0]
def rightWeights (e : F) : Fin 4 → F := ![0, 0, e*2, e*10]

def equations (r s e f : F) : Set (PencilTop.P F 4) :=
  PencilCanonical.generators 4 r s 2 (e*4) (f*4) (e*9) (f*9) (e*9) (f*9)
    (leftWeights e) (rightWeights f)

/-- The complete eight-column fixture has canonical degree seven, including
    all four independent field-valued Boolean selectors. -/
theorem canonical_degree_seven (r s e f : F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) :
    SecantIdeal.FirstVanishing (DegreeBridge.topIdeal (equations r s e f)) 7 := by
  exact PencilCanonical.canonical_first_vanishing 4 r s 2 (e*4) (f*4)
    (e*9) (f*9) (e*9) (f*9) (sign_nonzero e he) (sign_nonzero f hf)
    (by decide) (leftWeights e) (rightWeights f)

def leftLabel (u : Fin 4 → F) : Fin 2 → F := ![u 0, u 1]
def rightLabel (u : Fin 4 → F) : Fin 2 → F := ![u 2, u 3]
def joined (u v : Fin 2 → F) : Fin 4 → F := ![u 0, u 1, v 0, v 1]

theorem left_joined (u v : Fin 2 → F) : leftLabel (joined u v) = u := by
  funext i
  fin_cases i <;> rfl
theorem right_joined (u v : Fin 2 → F) : rightLabel (joined u v) = v := by
  funext i
  fin_cases i <;> rfl

theorem zeroes_iff_system (r s e f x z m : F) (u : Fin 4 → F) :
    (∀ p ∈ equations r s e f,
      MvPolynomial.eval (PencilCanonical.coordinates 4 x z m u) p = 0) ↔
      PencilSelector.System 2 4 9 9 0 r s e f x z m ![2,10]
        (leftLabel u) (rightLabel u) := by
  rw [equations, PencilCanonical.zeroes_iff]
  have hb : (∀ i, u i^2-u i = 0) ↔
      PencilSelector.Boolean (leftLabel u) ∧ PencilSelector.Boolean (rightLabel u) := by
    rw [PencilSelector.boolean_iff_equations, PencilSelector.boolean_iff_equations]
    simp [leftLabel, rightLabel, Fin.forall_fin_succ, and_assoc]
  rw [hb]
  have hl : e*4*x^2+(e*9*x+e*9+∑ i, leftWeights e i*u i)-m*(x-r)+s =
      e*PencilSelector.graph 4 9 9 0 ![2,10] (leftLabel u) x-m*(x-r)+s := by
    simp [leftWeights, leftLabel, PencilSelector.graph, PencilSelector.offset,
      Fin.sum_univ_succ]
    ring
  have hr : f*4*z^2+(f*9*z+f*9+∑ i, rightWeights f i*u i)-m*(z-r)+s =
      f*PencilSelector.graph 4 9 9 0 ![2,10] (rightLabel u) z-m*(z-r)+s := by
    simp [rightWeights, rightLabel, PencilSelector.graph, PencilSelector.offset,
      Fin.sum_univ_succ]
    ring
  rw [hl, hr]
  simp only [PencilSelector.System, SecantGroupLaw.System, and_assoc]

abbrev W : WeierstrassCurve.Affine F :=
  WeierstrassCurve.toAffine (SecantGroupLaw.shortCurve 2 2)

theorem discriminant_ne_zero : W.Δ ≠ 0 := by decide

/-- The same actual polynomial generators whose degree is seven give precisely
    the finite-target relations on the eight-column base, keeping every lift. -/
theorem polynomial_complete {r s x z y v : F}
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular z v) (hR : W.Nonsingular r s) :
    (x ∈ baseX ∧ z ∈ baseX ∧
      WeierstrassCurve.Affine.Point.some hP + WeierstrassCurve.Affine.Point.some hQ =
        WeierstrassCurve.Affine.Point.some hR) ↔
      ∃ e f m : F, ∃ u : Fin 4 → F,
        (e = 1 ∨ e = -1) ∧ (f = 1 ∨ f = -1) ∧
        y = e*PencilSelector.graph 4 9 9 0 ![2,10] (leftLabel u) x ∧
        v = f*PencilSelector.graph 4 9 9 0 ![2,10] (rightLabel u) z ∧
        ∀ p ∈ equations r s e f,
          MvPolynomial.eval (PencilCanonical.coordinates 4 x z m u) p = 0 := by
  have hpy : y^2 = curve x := by
    simpa [curve] using (SecantGroupLaw.equation_iff (2 : F) 2 x y).mp hP.1
  have hqv : v^2 = curve z := by
    simpa [curve] using (SecantGroupLaw.equation_iff (2 : F) 2 z v).mp hQ.1
  rw [← exact_membership x y hpy, ← exact_membership z v hqv,
    ← inBase_iff_label, ← inBase_iff_label]
  rw [PencilSelector.complete_selector_iff (2 : F) 2 4 9 9 0 ![2,10] hP hQ hR]
  constructor
  · rintro ⟨e, f, m, u, v, he, hf, hy, hv, hs⟩
    refine ⟨e, f, m, joined u v, he, hf, ?_, ?_, ?_⟩
    · simpa only [left_joined] using hy
    · simpa only [right_joined] using hv
    · apply (zeroes_iff_system r s e f x z m _).mpr
      simpa only [left_joined, right_joined] using hs
  · rintro ⟨e, f, m, u, he, hf, hy, hv, hp⟩
    exact ⟨e, f, m, leftLabel u, rightLabel u, he, hf, hy, hv,
      (zeroes_iff_system r s e f x z m u).mp hp⟩

open MvPolynomial
def denseRemainder (i : Fin 2) : MvPolynomial (Fin 2) F :=
  C 12*X i^7+C 15*X i^6+C 15*X i^5+C 9*X i^4+C 10*X i^3+C 5*X i

/-- A polynomial identity in the ring, not just equality of F17 functions. -/
theorem dense_membership_polynomial (i : Fin 2) :
    (X i : MvPolynomial (Fin 2) F)*(X i-C 3)*(X i-C 5)*(X i-C 6)*
      (X i-C 7)*(X i-C 9)*(X i-C 10)*(X i-C 16) = X i^8+denseRemainder i := by
  unfold denseRemainder
  simp only [map_ofNat]
  ring_nf
  have : CharP (MvPolynomial (Fin 2) F) 17 := inferInstance
  reduce_mod_char!
  ring_nf
  reduce_mod_char!

theorem denseRemainder_degree (i : Fin 2) : (denseRemainder i).totalDegree < 8 := by
  apply lt_of_le_of_lt ?_ (show (7 : ℕ) < 8 by decide)
  apply DegreeBridge.degree_le_of_above_zero (denseRemainder i) 7
  intro d hd
  have h7 : d ≠ 7 := by omega
  have h6 : d ≠ 6 := by omega
  have h5 : d ≠ 5 := by omega
  have h4 : d ≠ 4 := by omega
  have h3 : d ≠ 3 := by omega
  have h1 : d ≠ 1 := by omega
  have hX : homogeneousComponent d (X i : MvPolynomial (Fin 2) F) = 0 := by
    simpa [h1] using DenseTopForms.hc_pow (K := F) d 1 i
  simp [denseRemainder, _root_.map_add, homogeneousComponent_C_mul,
    DenseTopForms.hc_pow, h7, h6, h5, h4, h3, h1, hX]

theorem dense_degree_nine (r : F) : SecantIdeal.FirstVanishing
    (DegreeBridge.canonicalDense 8 2 2 r (denseRemainder 0) (denseRemainder 1)) 9 := by
  rw [DegreeBridge.canonicalDense_eq 8 2 2 r _ _
    (denseRemainder_degree 0) (denseRemainder_degree 1)]
  exact DenseTopForms.dense_equations_first_vanishing 8 (by decide) 2 2 r _ _
    (denseRemainder_degree 0) (denseRemainder_degree 1)

/-- Complete generator roots imply on-curve base relations even when the
    solver supplied no membership or ordinate certificates beforehand. -/
theorem roots_imply_relation {r s e f x z m : F} (u : Fin 4 → F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) (hR : W.Nonsingular r s)
    (hp : ∀ p ∈ equations r s e f,
      eval (PencilCanonical.coordinates 4 x z m u) p = 0) :
    x ∈ baseX ∧ z ∈ baseX ∧ ∃ y v : F,
      ∃ (hP : W.Nonsingular x y) (hQ : W.Nonsingular z v),
        WeierstrassCurve.Affine.Point.some hP + WeierstrassCurve.Affine.Point.some hQ =
          WeierstrassCurve.Affine.Point.some hR := by
  have hs := (zeroes_iff_system r s e f x z m u).mp hp
  obtain ⟨y, v, hy, hv, hP, hQ, hadd⟩ :=
    PencilSelector.system_implies_base_relation (2 : F) 2 4 9 9 0 ![2,10]
      (leftLabel u) (rightLabel u) he hf discriminant_ne_zero hR hs
  have hpy : y^2 = curve x := by
    simpa [curve] using (SecantGroupLaw.equation_iff (2 : F) 2 x y).mp hP.1
  have hqv : v^2 = curve z := by
    simpa [curve] using (SecantGroupLaw.equation_iff (2 : F) 2 z v).mp hQ.1
  exact ⟨(exact_membership x y hpy).mp ((inBase_iff_label x y).mp hy),
    (exact_membership z v hqv).mp ((inBase_iff_label z v).mp hv), y, v, hP, hQ, hadd⟩

/-- Arithmetic for full affine monomial envelopes, not observed F4 widths. -/
theorem ambient_budget_arithmetic : Nat.choose 14 7 = 3432 ∧ Nat.choose 11 2 = 55 := by
  decide

end PencilFixture

#print axioms PencilFixture.exact_membership
#print axioms PencilFixture.signed_cardinality
#print axioms PencilFixture.fibre_cardinalities
#print axioms PencilFixture.inBase_iff_label
#print axioms PencilFixture.canonical_degree_seven
#print axioms PencilFixture.zeroes_iff_system
#print axioms PencilFixture.polynomial_complete
#print axioms PencilFixture.dense_membership_polynomial
#print axioms PencilFixture.dense_degree_nine
#print axioms PencilFixture.roots_imply_relation
