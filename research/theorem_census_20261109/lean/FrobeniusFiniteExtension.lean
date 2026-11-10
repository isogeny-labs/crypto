import FrobeniusPointDescent

/-! Actual Frobenius iteration and points over a chosen finite extension.
No abstract rationality predicate or torsion basis is assumed. -/
namespace FrobeniusFiniteExtensionAudit
noncomputable section
open FrobeniusPointDescentAudit

section Descent
variable {R A B : Type*} [CommRing R] [Field A] [Fintype A] [Field B]
  [Algebra R A] [Algebra R B] [Algebra A B] [IsScalarTower R A B]
  [DecidableEq A] [DecidableEq B]

def frobeniusOver (W : WeierstrassCurve.Affine R) :
    (W.baseChange B).Point→+(W.baseChange B).Point :=
  WeierstrassCurve.Affine.Point.map (W':=W) (FiniteField.frobeniusAlgHom A B)

theorem frobeniusOver_fixed_iff (W : WeierstrassCurve.Affine R)
    (P : (W.baseChange B).Point) :
    frobeniusOver (A:=A) W P=P ↔
      ∃ Q : (W.baseChange A).Point,
        WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId A B) Q=P := by
  cases P with
  | zero =>
    constructor
    · intro _
      exact ⟨0,rfl⟩
    · intro _
      rfl
  | some x y h =>
    constructor
    · intro hfix
      have hx : FiniteField.frobeniusAlgHom A B x=x :=
        (WeierstrassCurve.Affine.Point.some.inj hfix).1
      have hy : FiniteField.frobeniusAlgHom A B y=y :=
        (WeierstrassCurve.Affine.Point.some.inj hfix).2
      obtain ⟨a,ha⟩ := (scalar_frobenius_fixed_iff (F:=A) x).mp hx
      obtain ⟨b,hb⟩ := (scalar_frobenius_fixed_iff (F:=A) y).mp hy
      have hab : (W.baseChange A).Nonsingular a b := by
        apply (W.baseChange_nonsingular (Algebra.ofId A B).injective a b).mp
        change (W.baseChange B).Nonsingular (algebraMap A B a) (algebraMap A B b)
        rw [ha,hb]
        exact h
      refine ⟨.some a b hab,?_⟩
      change WeierstrassCurve.Affine.Point.some ((Algebra.ofId A B) a) ((Algebra.ofId A B) b) _=
        WeierstrassCurve.Affine.Point.some x y h
      rw [WeierstrassCurve.Affine.Point.some.injEq]
      exact ⟨ha,hb⟩
    · rintro ⟨Q,hQ⟩
      rw [←hQ]
      cases Q with
      | zero => rfl
      | some a b hab =>
        change WeierstrassCurve.Affine.Point.some _ _ _=
          WeierstrassCurve.Affine.Point.some _ _ _
        rw [WeierstrassCurve.Affine.Point.some.injEq]
        exact ⟨(FiniteField.frobeniusAlgHom A B).commutes a,
          (FiniteField.frobeniusAlgHom A B).commutes b⟩

end Descent

section Powers
variable {R A B : Type*} [CommRing R] [CommRing A] [Field B]
  [Algebra R A] [Algebra R B] [Algebra A B] [IsScalarTower R A B] [DecidableEq B]

theorem point_map_pow (W : WeierstrassCurve.Affine R) (φ : B→ₐ[A]B) (n : ℕ)
    (P : (W.baseChange B).Point) :
    WeierstrassCurve.Affine.Point.map (W':=W) (φ^n) P=
      (WeierstrassCurve.Affine.Point.map (W':=W) φ)^[n] P := by
  induction n generalizing P with
  | zero => cases P <;> rfl
  | succ n ih =>
    calc
      WeierstrassCurve.Affine.Point.map (W':=W) (φ^(n+1)) P=
          WeierstrassCurve.Affine.Point.map (W':=W) (φ^n)
            (WeierstrassCurve.Affine.Point.map (W':=W) φ P) := by
        rw [pow_succ]
        exact (WeierstrassCurve.Affine.Point.map_map (W':=W) φ (φ^n) P).symm
      _=(WeierstrassCurve.Affine.Point.map (W':=W) φ)^[n]
          (WeierstrassCurve.Affine.Point.map (W':=W) φ P) := ih _
      _=(WeierstrassCurve.Affine.Point.map (W':=W) φ)^[n+1] P :=
        (Function.iterate_succ_apply _ _ _).symm

end Powers

section FiniteExtension
variable {F K B : Type*} [Field F] [Fintype F] [Field K] [Fintype K] [Field B]
  [Algebra F K] [Algebra F B] [Algebra K B] [IsScalarTower F K B]

theorem scalar_frobenius_power :
    (FiniteField.frobeniusAlgHom F B)^Module.finrank F K=
      (FiniteField.frobeniusAlgHom K B).restrictScalars F := by
  ext x
  simp only [AlgHom.coe_pow,FiniteField.coe_frobeniusAlgHom,pow_iterate,
    AlgHom.restrictScalars_apply]
  rw [Module.card_eq_pow_finrank (K:=F) (V:=K)]

theorem finrank_eq_of_card (k : ℕ) (h : Fintype.card K=Fintype.card F^k) :
    Module.finrank F K=k := by
  apply Nat.pow_right_injective (show 2≤Fintype.card F from Fintype.one_lt_card)
  exact (Module.card_eq_pow_finrank (K:=F) (V:=K)).symm.trans h

variable [DecidableEq B]

theorem frobeniusOver_eq_iterate (W : WeierstrassCurve.Affine F)
    (P : (W.baseChange B).Point) :
    frobeniusOver (A:=K) W P=(pointFrobenius W)^[Module.finrank F K] P := by
  calc
    frobeniusOver (A:=K) W P=
        WeierstrassCurve.Affine.Point.map (W':=W)
          ((FiniteField.frobeniusAlgHom K B).restrictScalars F) P := by
      cases P <;> rfl
    _=WeierstrassCurve.Affine.Point.map (W':=W)
        ((FiniteField.frobeniusAlgHom F B)^Module.finrank F K) P := by
      rw [scalar_frobenius_power (F:=F) (K:=K) (B:=B)]
    _=(pointFrobenius W)^[Module.finrank F K] P :=
      point_map_pow W (FiniteField.frobeniusAlgHom F B) _ P

variable [DecidableEq K]

theorem finite_extension_fixed_iff (W : WeierstrassCurve.Affine F)
    (P : (W.baseChange B).Point) :
    (pointFrobenius W)^[Module.finrank F K] P=P ↔
      ∃ Q : (W.baseChange K).Point,
        WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B) Q=P := by
  rw [←frobeniusOver_eq_iterate (F:=F) (K:=K) (B:=B) W P]
  exact frobeniusOver_fixed_iff W P

theorem finite_extension_torsion_iff (W : WeierstrassCurve.Affine F) (r : ℕ) :
    (∀ P : (W.baseChange B).Point,r • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B))) ↔
    (∀ P : (W.baseChange B).Point,r • P=0 →
      (pointFrobenius W)^[Module.finrank F K] P=P) := by
  constructor
  · intro h P hP
    exact (finite_extension_fixed_iff W P).mpr (h P hP)
  · intro h P hP
    exact (finite_extension_fixed_iff W P).mp (h P hP)

theorem finite_extension_fixed_iff_of_card (W : WeierstrassCurve.Affine F)
    (k : ℕ) (h : Fintype.card K=Fintype.card F^k) (P : (W.baseChange B).Point) :
    (pointFrobenius W)^[k] P=P ↔
      ∃ Q : (W.baseChange K).Point,
        WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B) Q=P := by
  have hk := finrank_eq_of_card (F:=F) (K:=K) k h
  simpa only [hk] using finite_extension_fixed_iff (K:=K) W P

theorem finite_extension_torsion_iff_of_card (W : WeierstrassCurve.Affine F)
    (r k : ℕ) (h : Fintype.card K=Fintype.card F^k) :
    (∀ P : (W.baseChange B).Point,r • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B))) ↔
    (∀ P : (W.baseChange B).Point,r • P=0 → (pointFrobenius W)^[k] P=P) := by
  have hk := finrank_eq_of_card (F:=F) (K:=K) k h
  simpa only [hk] using finite_extension_torsion_iff (K:=K) W r

end FiniteExtension

theorem finite_extension_audit_certificate :
    ∀ (F K B : Type*) [Field F] [Fintype F] [Field K] [Fintype K] [Field B]
      [Algebra F K] [Algebra F B] [Algebra K B] [IsScalarTower F K B]
      [DecidableEq K] [DecidableEq B],
      Fintype.card K=Fintype.card F^Module.finrank F K ∧
      ∀ W : WeierstrassCurve.Affine F,
        (∀ P : (W.baseChange B).Point,
          (pointFrobenius W)^[Module.finrank F K] P=P ↔
            ∃ Q : (W.baseChange K).Point,
              WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B) Q=P) ∧
        (∀ r : ℕ,
          (∀ P : (W.baseChange B).Point,r • P=0 →
            P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B))) ↔
          (∀ P : (W.baseChange B).Point,r • P=0 →
            (pointFrobenius W)^[Module.finrank F K] P=P)) ∧
        (∀ k : ℕ,Fintype.card K=Fintype.card F^k →
          (∀ P : (W.baseChange B).Point,(pointFrobenius W)^[k] P=P ↔
            ∃ Q : (W.baseChange K).Point,
              WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B) Q=P) ∧
          (∀ r : ℕ,
            (∀ P : (W.baseChange B).Point,r • P=0 →
              P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K B))) ↔
            (∀ P : (W.baseChange B).Point,r • P=0 → (pointFrobenius W)^[k] P=P))) := by
  intro F K B _ _ _ _ _ _ _ _ _ _ _
  refine ⟨Module.card_eq_pow_finrank,?_⟩
  intro W
  refine ⟨finite_extension_fixed_iff W,finite_extension_torsion_iff W,?_⟩
  intro k h
  exact ⟨finite_extension_fixed_iff_of_card W k h,
    fun r=>finite_extension_torsion_iff_of_card W r k h⟩

#print axioms frobeniusOver_fixed_iff
#print axioms point_map_pow
#print axioms scalar_frobenius_power
#print axioms finite_extension_fixed_iff
#print axioms finite_extension_torsion_iff
#print axioms finite_extension_fixed_iff_of_card
#print axioms finite_extension_torsion_iff_of_card
#print axioms finite_extension_audit_certificate
end
end FrobeniusFiniteExtensionAudit
