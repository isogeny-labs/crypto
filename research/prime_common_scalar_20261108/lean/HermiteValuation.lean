module
public import HermiteGrid
public section
namespace HermiteValuation
open Polynomial NodeMultiplicity HermiteUniqueness HermiteBasis HermiteGrid

@[expose] def activeIndices (a : ℕ) (t : Fin (2^(a+1))) : Finset (Fin (2^(a+1))) :=
  (Finset.univ.erase t).erase (zeroIndex a)

@[expose] noncomputable def basisValuationSum (a M u : ℕ) (t : Fin (2^(a+1))) : ℤ :=
  ∑ s ∈ activeIndices a t, (sourceOrder a M u s:ℤ)*
    ((padicValInt 2 (gridNode a s):ℤ)-(padicValInt 2 (gridNode a t-gridNode a s):ℤ))

theorem valuationProduct {ι : Type*} (S : Finset ι) (f : ι→ℚ) (hf : ∀ s∈S, f s≠0) :
    padicValRat 2 (∏ s∈S, f s)=∑ s∈S, padicValRat 2 (f s) := by
  classical
  induction S using Finset.induction_on with
  | empty => simp
  | @insert x S hx ih =>
    rw [Finset.prod_insert hx,Finset.sum_insert hx,
      padicValRat.mul (hf x (Finset.mem_insert_self _ _))
        (Finset.prod_ne_zero_iff.mpr (fun s hs => hf s (Finset.mem_insert_of_mem hs))),
      ih (fun s hs => hf s (Finset.mem_insert_of_mem hs))]

/-- The valuation is attached to the ACTUAL normalized Hermite basis at zero. -/
theorem actualBasisAtZeroValuation (a M u : ℕ) (t : Fin (2^(a+1))) :
    padicValRat 2 ((basis a M u t).eval 0)=basisValuationSum a M u t := by
  classical
  let S : Finset (Fin (2^(a+1))) := Finset.univ.erase t
  let f (s : Fin (2^(a+1))) : ℚ :=
    (((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹*(-(gridNode a s:ℚ)))^(sourceOrder a M u s)
  have hf (s : Fin (2^(a+1))) (hs : s∈S) : f s≠0 := by
    by_cases h0 : gridNode a s=0
    · simp [f,sourceOrder,h0]
    · have hn : (gridNode a s:ℚ)≠0 := by exact_mod_cast h0
      exact pow_ne_zero _ (mul_ne_zero (inv_ne_zero (gridDifferenceNonzero a t s (Finset.ne_of_mem_erase hs)))
        (neg_ne_zero.mpr hn))
  have he : (basis a M u t).eval 0=∏ s∈S, f s := by
    simp only [basis,eval_prod,eval_pow,eval_mul,eval_C,eval_sub,eval_X,zero_sub]
    rfl
  rw [he,valuationProduct S f hf]
  have hterm (s : Fin (2^(a+1))) (hs : s∈S) : padicValRat 2 (f s)=
      (sourceOrder a M u s:ℤ)*((padicValInt 2 (gridNode a s):ℤ)-
        (padicValInt 2 (gridNode a t-gridNode a s):ℤ)) := by
    by_cases h0 : gridNode a s=0
    · simp [f,sourceOrder,h0]
    · have hn : (gridNode a s:ℚ)≠0 := by exact_mod_cast h0
      dsimp [f]
      rw [padicValRat.pow,padicValRat.mul
        (inv_ne_zero (gridDifferenceNonzero a t s (Finset.ne_of_mem_erase hs))) (neg_ne_zero.mpr hn),
        padicValRat.inv,padicValRat.neg,←Int.cast_sub]
      simp only [padicValRat.of_int]
      ring
  calc
    _ = ∑ s∈S, (sourceOrder a M u s:ℤ)*((padicValInt 2 (gridNode a s):ℤ)-
          (padicValInt 2 (gridNode a t-gridNode a s):ℤ)) := Finset.sum_congr rfl hterm
    _ = _ := by
      symm
      apply Finset.sum_subset (Finset.erase_subset _ S)
      intro s hs hnot
      have hz : s=zeroIndex a := by simpa only [activeIndices,Finset.mem_erase,hs,and_true,not_not] using hnot
      subst s
      have h0 : gridNode a (zeroIndex a)=0 := (nodeZeroIff a _).mpr rfl
      simp [sourceOrder,h0]

theorem activeMirrorIff (a : ℕ) (t s : Fin (2^(a+1))) :
    s∈activeIndices a t ↔ mirror a t s∈activeIndices a t := by
  have forward (s : Fin (2^(a+1))) (hs : s∈activeIndices a t) : mirror a t s∈activeIndices a t := by
    have hs' : s≠zeroIndex a ∧ s≠t := by simpa [activeIndices] using hs
    have hn : gridNode a s≠0 := by simpa [nodeZeroIff] using hs'.1
    have he := mirrorExclusions a t s hn hs'.2
    simpa [activeIndices,nodeZeroIff] using he
  constructor
  · exact forward s
  · intro h
    have he := forward (mirror a t s) h
    simpa only [mirrorInvolution a t s] using he

/-- Exact finite pairing; no asymptotic or independent-randomness premise. -/
theorem actualPairedValuationIdentity (a M u : ℕ) (t : Fin (2^(a+1))) :
    2*basisValuationSum a M u t =
      ∑ s∈activeIndices a t,
        ((sourceOrder a M u s:ℤ)-(sourceOrder a M u (mirror a t s):ℤ))*
          ((padicValInt 2 (gridNode a s):ℤ)-(padicValInt 2 (gridNode a (mirror a t s)):ℤ)) := by
  classical
  let v (s : Fin (2^(a+1))) : ℤ := padicValInt 2 (gridNode a s)
  let q (s : Fin (2^(a+1))) : ℤ := sourceOrder a M u s
  have hs : basisValuationSum a M u t=∑ s∈activeIndices a t, q s*(v s-v (mirror a t s)) := by
    apply Finset.sum_congr rfl
    intro s hs
    have hst : s≠t := (Finset.mem_erase.mp ((Finset.mem_erase.mp hs).2)).1
    dsimp [q,v]
    rw [actualMirrorValuation a t s hst]
  have hp : (∑ s∈activeIndices a t, q s*(v s-v (mirror a t s))) =
      ∑ s∈activeIndices a t, q (mirror a t s)*(v (mirror a t s)-v s) := by
    apply Finset.sum_equiv (mirrorEquiv a t) (activeMirrorIff a t)
    intro s hs
    change q s*(v s-v (mirror a t s)) =
      q (mirror a t (mirror a t s))*(v (mirror a t (mirror a t s))-v (mirror a t s))
    rw [mirrorInvolution a t s]
  rw [hs]
  calc
    _ = (∑ s∈activeIndices a t, q s*(v s-v (mirror a t s)))+
        ∑ s∈activeIndices a t, q (mirror a t s)*(v (mirror a t s)-v s) := by rw [←hp]; ring
    _ = _ := by
      rw [←Finset.sum_add_distrib]
      apply Finset.sum_congr rfl
      intro s hs
      dsimp [q,v]
      ring

theorem taperProductOrdered (h u v w : ℕ) (hvw : v≤w) :
    -(h:ℤ)*((v:ℤ)-(w:ℤ))^2 ≤
      (((h*(u-v):ℕ):ℤ)-((h*(u-w):ℕ):ℤ))*((v:ℤ)-(w:ℤ)) := by
  let δ : ℕ := (u-v)-(u-w)
  let D : ℕ := w-v
  have hmono : u-w≤u-v := Nat.sub_le_sub_left hvw u
  have hd : δ≤D := by dsimp [δ,D]; omega
  have he : (((h*(u-v):ℕ):ℤ)-((h*(u-w):ℕ):ℤ))=(h:ℤ)*(δ:ℤ) := by
    simp only [Nat.cast_mul]
    dsimp [δ]
    rw [Nat.cast_sub hmono]
    ring
  have hgap : (v:ℤ)-(w:ℤ)=-(D:ℤ) := by
    dsimp [D]
    rw [Nat.cast_sub hvw]
    ring
  rw [he,hgap]
  have hm := mul_le_mul_of_nonneg_left (show (δ:ℤ)≤(D:ℤ) by exact_mod_cast hd)
    (show 0≤(h:ℤ)*(D:ℤ) by positivity)
  nlinarith only [hm]

theorem taperLipschitzProduct (h u v w : ℕ) :
    -(h:ℤ)*((v:ℤ)-(w:ℤ))^2 ≤
      (((h*(u-v):ℕ):ℤ)-((h*(u-w):ℕ):ℤ))*((v:ℤ)-(w:ℤ)) := by
  rcases le_total v w with hvw | hwv
  · exact taperProductOrdered h u v w hvw
  · have he := taperProductOrdered h u w v hwv
    convert he using 1 <;> ring

@[expose] noncomputable def gapMoment (a : ℕ) (t : Fin (2^(a+1))) : ℤ :=
  ∑ s∈activeIndices a t,
    ((padicValInt 2 (gridNode a s):ℤ)-(padicValInt 2 (gridNode a (mirror a t s)):ℤ))^2

/-- Reduces the actual Hermite multiplier loss to the actual grid's squared
valuation-gap moment. No unproved moment estimate is inserted. -/
theorem actualBasisMomentBound (a M u : ℕ) (t : Fin (2^(a+1))) :
    -(sourceCeiling M u:ℤ)*gapMoment a t ≤ 2*padicValRat 2 ((basis a M u t).eval 0) := by
  rw [actualBasisAtZeroValuation,actualPairedValuationIdentity]
  unfold gapMoment
  rw [Finset.mul_sum]
  apply Finset.sum_le_sum
  intro s hs
  have hs0 : gridNode a s≠0 := by
    intro hz
    exact (Finset.mem_erase.mp hs).1 ((nodeZeroIff a s).mp hz)
  have hm : mirror a t s∈activeIndices a t := (activeMirrorIff a t s).mp hs
  have hm0 : gridNode a (mirror a t s)≠0 := by
    intro hz
    exact (Finset.mem_erase.mp hm).1 ((nodeZeroIff a _).mp hz)
  simpa only [sourceOrder,hs0,hm0,ite_false] using
    taperLipschitzProduct (sourceCeiling M u) u
      (padicValInt 2 (gridNode a s)) (padicValInt 2 (gridNode a (mirror a t s)))

end HermiteValuation
#print axioms HermiteValuation.actualBasisAtZeroValuation
#print axioms HermiteValuation.actualPairedValuationIdentity
#print axioms HermiteValuation.actualBasisMomentBound
