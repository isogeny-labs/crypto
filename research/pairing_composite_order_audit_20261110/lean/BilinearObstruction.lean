import TorsionBoundaries
import Mathlib.GroupTheory.QuotientGroup.Basic
import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots

namespace CompositeTateObstructionAudit

def LeftBilinear {S R T : Type*} [Add S] [Mul T] (e : S→R→T) : Prop :=
  ∀ P Q Z,e (P+Q) Z=e P Z*e Q Z

def LeftNondegenerate {S R T : Type*} [Zero S] [One T] (e : S→R→T) : Prop :=
  ∀ P,(∀ Q,e P Q=1)→P=0

/-- An exponent-two target cannot detect a nonzero double in its left domain.
The actual Tate quotient domains are instantiated in the other modules. -/
theorem nonzero_double_obstructs {S R T : Type*} [AddZeroClass S] [Monoid T]
    (hsq : ∀ z : T,z^2=1) (P : S) (hne : P+P≠0) :
    ¬∃ e : S→R→T,LeftBilinear e ∧ LeftNondegenerate e := by
  rintro ⟨e,hb,hn⟩
  apply hne
  apply hn
  intro Q
  rw [hb]
  simpa only [pow_two] using hsq (e P Q)

#print axioms nonzero_double_obstructs
end CompositeTateObstructionAudit

namespace CompositeTateObstructionAudit

theorem kernel_card_of_nsmul_all_zero {G : Type*} [AddCommGroup G] (n : ℕ)
    (hn : ∀ P : G,n • P=0) :
    Nat.card ↥(nsmulAddMonoidHom n : G→+G).ker=Nat.card G := by
  refine Nat.card_congr {
    toFun := Subtype.val
    invFun := fun P => ⟨P,hn P⟩
    left_inv := ?_
    right_inv := ?_ }
  · intro P
    apply Subtype.ext
    rfl
  · intro P
    rfl

theorem nsmul_range_eq_bot_of_all_zero {G : Type*} [AddCommGroup G] (n : ℕ)
    (hn : ∀ P : G,n • P=0) :
    (nsmulAddMonoidHom n : G→+G).range=⊥ := by
  ext P
  constructor
  · rintro ⟨Q,hQ⟩
    change P=0
    exact hQ.symm.trans (hn Q)
  · intro hP
    have hp : P=0 := hP
    subst P
    exact ⟨0,by simp⟩

theorem right_card_of_nsmul_all_zero {G : Type*} [AddCommGroup G] (n : ℕ)
    (hn : ∀ P : G,n • P=0) :
    Nat.card (G ⧸ (nsmulAddMonoidHom n : G→+G).range)=Nat.card G := by
  rw [nsmul_range_eq_bot_of_all_zero n hn]
  exact Nat.card_congr (QuotientAddGroup.quotientBot (G:=G)).toEquiv
end CompositeTateObstructionAudit

namespace CompositeTateObstructionAudit
variable {K : Type*} [Field K] [DecidableEq K]

def TateLeft (E : WeierstrassCurve.Affine K) (n : ℕ) : AddSubgroup E.Point :=
  (nsmulAddMonoidHom n : E.Point→+E.Point).ker
abbrev TateRight (E : WeierstrassCurve.Affine K) (n : ℕ) :=
  E.Point ⧸ (nsmulAddMonoidHom n : E.Point→+E.Point).range
abbrev TateTarget (K : Type*) [Field K] (n : ℕ) :=
  Kˣ ⧸ (powMonoidHom n : Kˣ→*Kˣ).range

def PairingExists (E : WeierstrassCurve.Affine K) (n : ℕ) : Prop :=
  ∃ e : ↥(TateLeft E n)→TateRight E n→TateTarget K n,
    LeftBilinear e ∧ LeftNondegenerate e
end CompositeTateObstructionAudit
