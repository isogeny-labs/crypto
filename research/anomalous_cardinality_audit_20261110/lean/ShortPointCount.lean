import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Data.Fintype.Card
import Mathlib.Tactic
import PrimeFieldControls

namespace ShortPointCountAudit
noncomputable section

def shortCurve {K : Type*} [Field K] (a b : K) : WeierstrassCurve.Affine K :=
  ⟨0,0,0,a,b⟩

theorem square_equation {K : Type*} [Field K] (a b x y : K)
    (h : (shortCurve a b).Nonsingular x y) : y^2=x^3+a*x+b := by
  have he : (shortCurve a b).Equation x y := h.1
  rw [WeierstrassCurve.Affine.equation_iff] at he
  simpa [shortCurve] using he

private def chosenRoot {K : Type*} [Field K] (a b x : K) : K := by
  classical
  exact if h : ∃ y : K, y^2=x^3+a*x+b then h.choose else 0

private theorem chosenRoot_square {K : Type*} [Field K] (a b x : K)
    (h : ∃ y : K, y^2=x^3+a*x+b) : (chosenRoot a b x)^2=x^3+a*x+b := by
  classical
  simp only [chosenRoot,dite_eq_left h]
  exact h.choose_spec

/-- An empty affine x-fiber improves the elementary bound to strictly less
than twice the field size. No Hasse estimate or curve-cardinality premise. -/
theorem cardinality_lt_twice_field {K : Type*} [Field K] [Fintype K]
    (a b x₀ : K) (hgap : ¬∃ y : K, y^2=x₀^3+a*x₀+b) :
    Nat.card (shortCurve a b).Point<2*Fintype.card K := by
  classical
  let A := {xy : K × K // (shortCurve a b).Nonsingular xy.1 xy.2}
  let X := {x : K // x≠x₀}
  let f : A → X × Bool := fun P =>
    (⟨P.1.1,by
      intro h
      apply hgap
      refine ⟨P.1.2,?_⟩
      simpa only [h] using square_equation a b P.1.1 P.1.2 P.2⟩,
      decide (P.1.2=chosenRoot a b P.1.1))
  have hf : Function.Injective f := by
    rintro ⟨⟨x,y⟩,hP⟩ ⟨⟨u,v⟩,hQ⟩ h
    have hx : x=u := congrArg (fun z : X × Bool => z.1.1) h
    subst u
    have hb : decide (y=chosenRoot a b x)=decide (v=chosenRoot a b x) :=
      congrArg Prod.snd h
    apply Subtype.ext
    change (x,y)=(x,v)
    congr 1
    by_cases hy : y=chosenRoot a b x
    · have hv : v=chosenRoot a b x := of_decide_eq_true (by simpa [hy] using hb.symm)
      exact hy.trans hv.symm
    · have hv : v≠chosenRoot a b x := by
        intro hv
        exact hy (of_decide_eq_true (by simpa [hv] using hb))
      have hr := chosenRoot_square a b x ⟨y,square_equation a b x y hP⟩
      have hy' : y= -chosenRoot a b x :=
        (sq_eq_sq_iff_eq_or_eq_neg.mp ((square_equation a b x y hP).trans hr.symm)).resolve_left hy
      have hv' : v= -chosenRoot a b x :=
        (sq_eq_sq_iff_eq_or_eq_neg.mp ((square_equation a b x v hQ).trans hr.symm)).resolve_left hv
      exact hy'.trans hv'.symm
  have ha : Fintype.card A≤Fintype.card X*2 := by
    simpa using Fintype.card_le_of_injective f hf
  have hx : Fintype.card X<Fintype.card K :=
    Fintype.card_subtype_lt (x:=x₀) (by simp)
  have hp : Nat.card (shortCurve a b).Point=Fintype.card A+1 := by
    rw [Nat.card_congr (shortCurve a b).nonsingularPointEquiv]
    change Nat.card (Option A)=Fintype.card A+1
    rw [Nat.card_eq_fintype_card,Fintype.card_option]
  omega

/-- The missing-fiber hypothesis cannot be omitted, even on an actual
nonsingular short Weierstrass curve. -/
theorem unrestricted_twice_field_bound_false :
    ¬ ∀ (K : Type) [Field K] [Fintype K] (a b : K),
      (shortCurve a b).Δ≠0 → Nat.card (shortCurve a b).Point<2*Fintype.card K := by
  intro h
  let : Fact (Nat.Prime 3) := ⟨by decide⟩
  have hh := h (ZMod 3) (-1) 1 SmallCharacteristicPrimeAudit.E3_nonsingular
  change Nat.card SmallCharacteristicPrimeAudit.E3.Point<2*Fintype.card (ZMod 3) at hh
  rw [SmallCharacteristicPrimeAudit.E3_cardinality] at hh
  norm_num at hh

theorem multiple_below_twice_eq (p n : ℕ) (hn : 0<n)
    (hd : p∣n) (hb : n<2*p) : n=p := by
  obtain ⟨k,rfl⟩ := hd
  have hk : k=1 := by
    have hpos : 0<k := by nlinarith
    have hlt : k<2 := by nlinarith
    omega
  simp [hk]

#print axioms cardinality_lt_twice_field
#print axioms unrestricted_twice_field_bound_false
#print axioms multiple_below_twice_eq
end
end ShortPointCountAudit
