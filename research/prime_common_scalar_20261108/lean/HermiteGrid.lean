module
public import HermiteBasis
public import Mathlib.Data.Int.ModEq
public import Mathlib.Algebra.Group.Fin.Basic
public section
namespace HermiteGrid
open NodeMultiplicity HermiteUniqueness LocalIntegral

@[expose] def zeroIndex (a : ℕ) : Fin (2^(a+1)) :=
  ⟨2^a,by
    rw [pow_succ]
    have h : 0<2^a := by positivity
    omega⟩

@[expose] def mirror (a : ℕ) (t s : Fin (2^(a+1))) : Fin (2^(a+1)) := t+zeroIndex a-s

theorem mirrorInvolution (a : ℕ) (t : Fin (2^(a+1))) : Function.Involutive (mirror a t) := by
  intro s
  unfold mirror
  abel

@[expose] def mirrorEquiv (a : ℕ) (t : Fin (2^(a+1))) : Fin (2^(a+1)) ≃ Fin (2^(a+1)) :=
  (mirrorInvolution a t).toPerm

theorem nodeZeroIff (a : ℕ) (s : Fin (2^(a+1))) : gridNode a s=0 ↔ s=zeroIndex a := by
  rw [Fin.ext_iff]
  change ((s.val:ℤ)-(2:ℤ)^a=0) ↔ s.val=2^a
  rw [sub_eq_zero]
  norm_cast

theorem mirrorZero (a : ℕ) (t : Fin (2^(a+1))) : mirror a t (zeroIndex a)=t := by
  simp [mirror]

theorem mirrorTarget (a : ℕ) (t : Fin (2^(a+1))) : mirror a t t=zeroIndex a := by
  unfold mirror
  abel

theorem mirrorExclusions (a : ℕ) (t s : Fin (2^(a+1))) (hs0 : gridNode a s≠0) (hst : s≠t) :
    gridNode a (mirror a t s)≠0 ∧ mirror a t s≠t := by
  have hinj := (mirrorEquiv a t).injective
  constructor
  · intro h
    have he : mirror a t s=mirror a t t := by rw [mirrorTarget]; exact (nodeZeroIff a _).mp h
    exact hst (hinj he)
  · intro h
    have he : mirror a t s=mirror a t (zeroIndex a) := by rw [mirrorZero]; exact h
    exact hs0 ((nodeZeroIff a s).mpr (hinj he))

theorem finAddCongruence {n : ℕ} (x y : Fin n) :
    Int.ModEq (n:ℤ) ((x+y).val:ℤ) ((x.val:ℤ)+(y.val:ℤ)) := by
  change (((x+y).val:ℤ) % (n:ℤ)) = ((x.val:ℤ)+(y.val:ℤ))%(n:ℤ)
  rw [Fin.val_add,Int.natCast_emod,Int.natCast_add,Int.emod_emod]

theorem finSubCongruence {n : ℕ} (x y : Fin n) :
    Int.ModEq (n:ℤ) ((x-y).val:ℤ) ((x.val:ℤ)-(y.val:ℤ)) := by
  rw [Int.modEq_iff_dvd,Fin.intCast_val_sub_eq_sub_add_ite]
  split_ifs
  · simp
  · simpa only [sub_add_cancel_left,neg_sub] using (dvd_neg.mpr (dvd_refl (n:ℤ)))

/-- The actual centered representative is congruent to the actual integer difference. -/
theorem actualMirrorCongruence (a : ℕ) (t s : Fin (2^(a+1))) :
    Int.ModEq ((2^(a+1):ℕ):ℤ) (gridNode a (mirror a t s)) (gridNode a t-gridNode a s) := by
  have he := (finSubCongruence (t+zeroIndex a) s).trans
    ((finAddCongruence t (zeroIndex a)).sub (Int.ModEq.refl (s.val:ℤ)))
  have hh := he.sub (Int.ModEq.refl (2^a:ℤ))
  convert hh using 1 <;> simp only [gridNode,mirror,zeroIndex,Nat.cast_pow,Nat.cast_ofNat] <;> ring

/-- Congruence modulo the full binary grid preserves every bounded nonzero valuation. -/
theorem binaryValuationCongruence (a : ℕ) (x y : ℤ) (hx : x≠0) (hbound : x.natAbs<2^(a+1))
    (hmod : Int.ModEq ((2^(a+1):ℕ):ℤ) x y) : padicValInt 2 y=padicValInt 2 x := by
  have hi : padicValInt 2 x<a+1 := bounded_valuation a x.natAbs (Int.natAbs_ne_zero.mpr hx) hbound
  have hd : (2:ℤ)^(padicValInt 2 x+1) ∣ ((2^(a+1):ℕ):ℤ) := by
    rw [Nat.cast_pow,Nat.cast_ofNat]
    exact pow_dvd_pow 2 (by omega)
  have he := hmod.of_dvd hd
  change x%(2:ℤ)^(padicValInt 2 x+1)=y%(2:ℤ)^(padicValInt 2 x+1) at he
  have hxmod := (binaryResidueIff x (padicValInt 2 x)).mpr ⟨hx,rfl⟩
  have hymod : y%(2:ℤ)^(padicValInt 2 x+1)=(2:ℤ)^(padicValInt 2 x) := he.symm.trans hxmod
  exact ((binaryResidueIff y (padicValInt 2 x)).mp hymod).2

theorem actualMirrorValuation (a : ℕ) (t s : Fin (2^(a+1))) (hst : s≠t) :
    padicValInt 2 (gridNode a (mirror a t s))=padicValInt 2 (gridNode a t-gridNode a s) := by
  have hn : gridNode a t-gridNode a s≠0 := sub_ne_zero.mpr ((sourceGridInjective a).ne hst.symm)
  have hb : (gridNode a t-gridNode a s).natAbs<2^(a+1) := by
    have ht := t.isLt
    have hs := s.isLt
    unfold gridNode
    omega
  exact binaryValuationCongruence a _ _ hn hb (actualMirrorCongruence a t s).symm

end HermiteGrid
#print axioms HermiteGrid.actualMirrorCongruence
#print axioms HermiteGrid.actualMirrorValuation
