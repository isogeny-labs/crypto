import CanonicalFixture
import SplitFactorBase

/-! One concrete polynomial presentation, actual group semantics, and its
    canonical homogeneous first-vanishing degree. -/
noncomputable section
open scoped BigOperators
open WeierstrassCurve
namespace FixtureGroupDegree
open SecantFixture
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

def graphPolynomial : Polynomial F :=
  Polynomial.C 4*Polynomial.X^2+Polynomial.C 9*Polynomial.X+Polynomial.C 9

theorem graph_eval (t : F) : graphPolynomial.eval t = graph t := by
  simp [graphPolynomial, graph]

theorem complete_split : SplitFactorBase.CompleteSplit 2 2 graphPolynomial baseX 4 := by
  classical
  unfold SplitFactorBase.CompleteSplit graphPolynomial SplitFactorBase.cubic baseX
  have h5 : (5 : F) ∉ ({6, 10, 16} : Finset F) := by decide
  have h6 : (6 : F) ∉ ({10, 16} : Finset F) := by decide
  have h10 : (10 : F) ∉ ({16} : Finset F) := by decide
  rw [Finset.prod_insert h5, Finset.prod_insert h6,
    Finset.prod_insert h10, Finset.prod_singleton]
  norm_num only [map_pow, map_ofNat]
  unfold F at *
  ring_nf
  reduce_mod_char!
  ring_nf
  reduce_mod_char!

abbrev W : WeierstrassCurve.Affine F :=
  WeierstrassCurve.toAffine (SecantGroupLaw.shortCurve 2 2)

theorem discriminant_ne_zero : W.Δ ≠ 0 := by decide

/-- Every finite point on this concrete curve is a valid nonsingular target. -/
theorem target_nonsingular (r s : F) (hcurve : s^2 = r^3+2*r+2) :
    W.Nonsingular r s :=
  SplitFactorBase.nonsingular_of_equation 2 2 discriminant_ne_zero hcurve

open MvPolynomial
local notation "P" => MvPolynomial (Fin 3) F

/-- Exactly the generators whose canonical top ideal is compared. -/
def generators (r s e f : F) : Set P :=
  {SecantTopForms.g1 r, SecantTopForms.g2 r s 2,
    SecantTopForms.q 0 2 (e*4) r s (graphLower 0 e),
    SecantTopForms.q 1 2 (f*4) r s (graphLower 1 f)}

def coordinates (x z m : F) : Fin 3 → F := ![x, z, m]

theorem eval_g1 (r x z m : F) :
    eval (coordinates x z m) (SecantTopForms.g1 r) = m^2-x-z-r := by
  simp [SecantTopForms.g1, coordinates]

theorem eval_g2 (r s x z m : F) :
    eval (coordinates x z m) (SecantTopForms.g2 r s 2) =
      x*z-r^2-r*m^2-2-2*m*s := by
  simp [SecantTopForms.g2, coordinates]
  ring

theorem eval_q (i : Fin 3) (r s e : F) (v : Fin 3 → F) :
    eval v (SecantTopForms.q i 2 (e*4) r s (graphLower i e)) =
      e*graph (v i)-v 2*(v i-r)+s := by
  simp [SecantTopForms.q, graphLower, graph]
  ring

theorem zeroes_iff_system (r s e f x z m : F) :
    (∀ p ∈ generators r s e f, eval (coordinates x z m) p = 0) ↔
      SecantGroupLaw.System 2 r s x z (e*graph x) (f*graph z) m := by
  simp only [generators, Set.forall_mem_insert, Set.mem_singleton_iff, forall_eq]
  rw [eval_g1, eval_g2, eval_q, eval_q]
  rfl

theorem canonical_top_ideal (r s e f : F) :
    DegreeBridge.topIdeal (generators r s e f) =
      DegreeBridge.canonicalSecant 2 (e*4) (f*4) r s 2
        (graphLower 0 e) (graphLower 1 f) := rfl

/-- The concrete signed base relation in the actual nonsingular-point group. -/
def BaseRelation (r s e f x z : F) (hR : W.Nonsingular r s) : Prop :=
  x ∈ baseX ∧ z ∈ baseX ∧
    ∃ (hP : W.Nonsingular x (e*graph x)) (hQ : W.Nonsingular z (f*graph z)),
      Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR

theorem polynomial_roots_iff_relation (r s e f x z : F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) (hR : W.Nonsingular r s) :
    (∃ m, ∀ p ∈ generators r s e f, eval (coordinates x z m) p = 0) ↔
      BaseRelation r s e f x z hR := by
  simp only [zeroes_iff_system]
  simpa only [graph_eval, BaseRelation] using
    (SplitFactorBase.selector_iff_group_relation (2 : F) 2 graphPolynomial baseX 4
      (by decide) complete_split discriminant_ne_zero hR x z e f he hf)

/-- One actual generator set has exact group semantics and canonical degree3;
    the inherited dense comparator has canonical degree5. Every sign is kept. -/
theorem exact_relations_and_degree (r s e f : F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) (hR : W.Nonsingular r s) :
    (∀ x z, (∃ m, ∀ p ∈ generators r s e f, eval (coordinates x z m) p = 0) ↔
      BaseRelation r s e f x z hR) ∧
    SecantIdeal.FirstVanishing (DegreeBridge.topIdeal (generators r s e f)) 3 ∧
    SecantIdeal.FirstVanishing
      (DegreeBridge.canonicalDense 4 2 2 r (denseLower 0) (denseLower 1)) 5 := by
  refine ⟨fun x z => polynomial_roots_iff_relation r s e f x z he hf hR, ?_⟩
  rw [canonical_top_ideal]
  exact CanonicalFixture.degree_five_to_three r s e f he hf

/-- Every ordinate lift is included by the original four polynomial branches. -/
theorem all_lifts_polynomial_complete {r s x z y v : F}
    (hx : x ∈ baseX) (hz : z ∈ baseX)
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular z v) (hR : W.Nonsingular r s) :
    (Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR) ↔
      ∃ e f m : F, (e = 1 ∨ e = -1) ∧ (f = 1 ∨ f = -1) ∧
        y = e*graph x ∧ v = f*graph z ∧
        ∀ p ∈ generators r s e f, eval (coordinates x z m) p = 0 := by
  have hc := SplitFactorBase.all_four_branches_complete (2 : F) 2
    graphPolynomial baseX 4 (by decide) complete_split hx hz hP hQ hR
  simp only [graph_eval] at hc
  constructor
  · intro hadd
    obtain ⟨e, f, m, he, hf, hy, hv, hm⟩ := hc.mp hadd
    exact ⟨e, f, m, he, hf, hy, hv, (zeroes_iff_system r s e f x z m).mpr hm⟩
  · rintro ⟨e, f, m, he, hf, hy, hv, hp⟩
    exact hc.mpr ⟨e, f, m, he, hf, hy, hv,
      (zeroes_iff_system r s e f x z m).mp hp⟩

/-- Infinity is covered by an exact inverse-pair theorem, not a finite slope. -/
theorem identity_relations_iff (x z e f : F)
    (hP : W.Nonsingular x (e*graph x)) (hQ : W.Nonsingular z (f*graph z)) :
    Affine.Point.some hP + Affine.Point.some hQ = 0 ↔
      x = z ∧ e*graph x = -(f*graph z) :=
  SecantGroupLaw.identity_target_iff 2 2 hP hQ

end FixtureGroupDegree

#print axioms FixtureGroupDegree.complete_split
#print axioms FixtureGroupDegree.discriminant_ne_zero
#print axioms FixtureGroupDegree.target_nonsingular
#print axioms FixtureGroupDegree.zeroes_iff_system
#print axioms FixtureGroupDegree.polynomial_roots_iff_relation
#print axioms FixtureGroupDegree.exact_relations_and_degree
#print axioms FixtureGroupDegree.all_lifts_polynomial_complete
#print axioms FixtureGroupDegree.identity_relations_iff
