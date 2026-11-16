module
public import HermiteValuation
public section
namespace HermiteMoment
open NodeMultiplicity HermiteUniqueness HermiteBasis HermiteGrid HermiteValuation

/-- The top nonzero binary level consists of the left endpoint only. -/
theorem topLevelIff (a : ℕ) (m : Fin (2^(a+1))) :
    gridNode a m % (2:ℤ)^(a+1)=(2:ℤ)^a ↔ m=0 := by
  let b : ℤ := (2:ℤ)^a
  have hb : 0<b := by dsimp [b]; positivity
  have hN : (2:ℤ)^(a+1)=b*2 := pow_succ _ _
  have hm0 : (0:ℤ)≤m.val := by positivity
  have hm : (m.val:ℤ)<b*2 := by
    have he : (m.val:ℤ)<(2:ℤ)^(a+1) := by exact_mod_cast m.isLt
    simpa only [hN] using he
  change ((m.val:ℤ)-b) % (2:ℤ)^(a+1)=b ↔ m=0
  rw [hN,Fin.ext_iff]
  change ((m.val:ℤ)-b) % (b*2)=b ↔ m.val=0
  by_cases hs : (m.val:ℤ)<b
  · have hmod : ((m.val:ℤ)-b) % (b*2)=((m.val:ℤ)+b) % (b*2) := by
      have he : (m.val:ℤ)+b=((m.val:ℤ)-b)+(b*2) := by ring
      rw [he]
      simp only [Int.add_emod,Int.emod_self,add_zero,Int.emod_emod]
    rw [hmod,Int.emod_eq_of_lt (by omega) (by omega)]
    omega
  · rw [Int.emod_eq_of_lt (by omega) (by omega)]
    omega

theorem actualTopLevelCard (a : ℕ) : Fintype.card (activeLevel a a)=1 := by
  have he : activeLevel a a ≃ {m : Fin (2^(a+1)) // m=0} :=
    Equiv.subtypeEquivRight (fun m => topLevelIff a m)
  rw [Fintype.card_congr he]
  simp

theorem actualValuationCardThroughTop (a i : ℕ) (hi : i≤a) :
    Fintype.card {m : Fin (2^(a+1)) // gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=i}=2^(a-i) := by
  by_cases hl : i<a
  · exact actualValuationCard a i hl
  · have hei : i=a := by omega
    subst i
    have he : activeLevel a a ≃
        {m : Fin (2^(a+1)) // gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=a} :=
      Equiv.subtypeEquivRight (fun m => binaryResidueIff (gridNode a m) a)
    rw [←Fintype.card_congr he,actualTopLevelCard]
    simp

theorem actualNodeValuationBound (a : ℕ) (m : Fin (2^(a+1))) : padicValInt 2 (gridNode a m)≤a := by
  by_cases hz : gridNode a m=0
  · simp [hz]
  · have hn : (gridNode a m).natAbs≠0 := Int.natAbs_ne_zero.mpr hz
    have hb : (gridNode a m).natAbs<2^(a+1) := by
      have hm := m.isLt
      have hp : 2^(a+1)=2^a*2 := pow_succ _ _
      have hpos : 0<2^a := by positivity
      have he : gridNode a m=(m.val:ℤ)-((2^a:ℕ):ℤ) := by simp [gridNode]
      rw [he]
      omega
    exact Nat.le_of_lt_succ (LocalIntegral.bounded_valuation a _ hn hb)

/-- Finite algebraic second-moment sum; no real convergence assumption. -/
theorem finiteSquareGeometric (n : ℕ) :
    (∑ j ∈ Finset.range (n+1), (j:ℚ)^2/(2:ℚ)^j) =
      6-((n:ℚ)^2+4*(n:ℚ)+6)/(2:ℚ)^n := by
  induction n with
  | zero => norm_num
  | succ n ih =>
    rw [Finset.sum_range_succ,ih]
    push_cast
    rw [pow_succ]
    field_simp
    ring

theorem finiteSquareGeometricBound (n : ℕ) :
    (∑ j ∈ Finset.range (n+1), (j:ℚ)^2/(2:ℚ)^j)≤6 := by
  rw [finiteSquareGeometric]
  have hn : (0:ℚ)≤((n:ℚ)^2+4*(n:ℚ)+6)/(2:ℚ)^n := by positivity
  linarith

theorem excessGapSquare (i v w : ℕ) (hmin : min v w ≤ i)
    (hneq : v≠w → i=min v w) :
    ((v:ℤ)-(w:ℤ))^2=((v-i:ℕ):ℤ)^2+((w-i:ℕ):ℤ)^2 := by
  by_cases heq : v=w
  · subst w
    have hv : v ≤ i := by simpa using hmin
    simp [Nat.sub_eq_zero_of_le hv]
  · have hi := hneq heq
    rcases le_total v w with hvw | hwv
    · rw [min_eq_left hvw] at hi
      subst i
      rw [Nat.sub_self,Nat.cast_zero,zero_pow (by decide),zero_add,Nat.cast_sub hvw]
      ring
    · rw [min_eq_right hwv] at hi
      subst i
      rw [Nat.sub_self,Nat.cast_zero,zero_pow (by decide),add_zero,Nat.cast_sub hwv]

theorem actualSourceGapSquare (a : ℕ) (t s : Fin (2^(a+1))) (ht0 : gridNode a t≠0)
    (hs : s∈activeIndices a t) :
    ((padicValInt 2 (gridNode a s):ℤ)-(padicValInt 2 (gridNode a (mirror a t s)):ℤ))^2 =
      ((padicValInt 2 (gridNode a s)-padicValInt 2 (gridNode a t):ℕ):ℤ)^2+
      ((padicValInt 2 (gridNode a (mirror a t s))-padicValInt 2 (gridNode a t):ℕ):ℤ)^2 := by
  have hs0 : gridNode a s≠0 := by
    intro h
    exact (Finset.mem_erase.mp hs).1 ((nodeZeroIff a s).mp h)
  have hst : s≠t := (Finset.mem_erase.mp ((Finset.mem_erase.mp hs).2)).1
  have hd0 : gridNode a t-gridNode a s≠0 := sub_ne_zero.mpr ((sourceGridInjective a).ne hst.symm)
  let x : ℚ := gridNode a s
  let y : ℚ := (gridNode a t:ℚ)-(gridNode a s:ℚ)
  have hx : x≠0 := by dsimp [x]; exact_mod_cast hs0
  have hy : y≠0 := by dsimp [y]; exact_mod_cast hd0
  have he : x+y=(gridNode a t:ℚ) := by dsimp [x,y]; ring
  have hxy : x+y≠0 := by rw [he]; exact_mod_cast ht0
  have hvx : padicValRat 2 x=(padicValInt 2 (gridNode a s):ℤ) := padicValRat.of_int
  have hvy : padicValRat 2 y=(padicValInt 2 (gridNode a t-gridNode a s):ℤ) := by
    dsimp [y]
    rw [←Int.cast_sub,padicValRat.of_int]
  have hvt : padicValRat 2 (x+y)=(padicValInt 2 (gridNode a t):ℤ) := by rw [he,padicValRat.of_int]
  have hmin : min (padicValInt 2 (gridNode a s)) (padicValInt 2 (gridNode a t-gridNode a s))≤
      padicValInt 2 (gridNode a t) := by
    have hh := padicValRat.min_le_padicValRat_add (p:=2) hxy
    rw [hvx,hvy,hvt] at hh
    exact_mod_cast hh
  have hneq : padicValInt 2 (gridNode a s)≠padicValInt 2 (gridNode a t-gridNode a s) →
      padicValInt 2 (gridNode a t)=min (padicValInt 2 (gridNode a s))
        (padicValInt 2 (gridNode a t-gridNode a s)) := by
    intro hh
    have hrat : padicValRat 2 x≠padicValRat 2 y := by rw [hvx,hvy]; exact_mod_cast hh
    have hv := padicValRat.add_eq_min (p:=2) hxy hx hy hrat
    rw [hvt,hvx,hvy] at hv
    exact_mod_cast hv
  have hresult := excessGapSquare _ _ _ hmin hneq
  rw [←actualMirrorValuation a t s hst] at hresult
  exact hresult

@[expose] noncomputable def sourceMoment (a i : ℕ) : ℤ :=
  ∑ s∈Finset.univ.erase (zeroIndex a), ((padicValInt 2 (gridNode a s)-i:ℕ):ℤ)^2

theorem actualGapMomentEq (a : ℕ) (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) :
    gapMoment a t=2*sourceMoment a (padicValInt 2 (gridNode a t)) := by
  classical
  let f (s : Fin (2^(a+1))) : ℤ :=
    ((padicValInt 2 (gridNode a s)-padicValInt 2 (gridNode a t):ℕ):ℤ)^2
  have he : gapMoment a t=∑ s∈activeIndices a t, (f s+f (mirror a t s)) := by
    apply Finset.sum_congr rfl
    intro s hs
    exact actualSourceGapSquare a t s ht0 hs
  have hmirror : (∑ s∈activeIndices a t, f (mirror a t s))=∑ s∈activeIndices a t, f s := by
    symm
    apply Finset.sum_equiv (mirrorEquiv a t) (activeMirrorIff a t)
    intro s hs
    change f s=f (mirror a t (mirror a t s))
    rw [mirrorInvolution a t s]
  have hsum : (∑ s∈activeIndices a t, f s)=sourceMoment a (padicValInt 2 (gridNode a t)) := by
    apply Finset.sum_subset
    · intro s hs
      exact Finset.mem_erase.mpr ⟨(Finset.mem_erase.mp hs).1,Finset.mem_univ s⟩
    · intro s hs hnot
      have hst : s=t := by
        by_contra hne
        apply hnot
        simp only [activeIndices,Finset.mem_erase,Finset.mem_univ,and_true]
        exact ⟨(Finset.mem_erase.mp hs).1,hne⟩
      subst s
      simp [f]
  rw [he,Finset.sum_add_distrib,hmirror,hsum]
  ring

theorem sourceMomentLevelSum (a i : ℕ) :
    sourceMoment a i = ∑ j∈Finset.range (a+1), ((2^(a-j):ℕ):ℤ)*((j-i:ℕ):ℤ)^2 := by
  classical
  let S : Finset (Fin (2^(a+1))) := Finset.univ.erase (zeroIndex a)
  let v (s : Fin (2^(a+1))) : ℕ := padicValInt 2 (gridNode a s)
  have hmap : ∀ s∈S, v s∈Finset.range (a+1) := by
    intro s hs
    exact Finset.mem_range.mpr (Nat.lt_succ_of_le (actualNodeValuationBound a s))
  have he := Finset.sum_fiberwise_of_maps_to hmap (fun s => ((v s-i:ℕ):ℤ)^2)
  change (∑ s∈S, ((v s-i:ℕ):ℤ)^2)=_
  rw [←he]
  apply Finset.sum_congr rfl
  intro j hj
  have hc : (S.filter (fun s => v s=j)).card=2^(a-j) := by
    calc
      _ = Fintype.card {s : Fin (2^(a+1)) // gridNode a s≠0 ∧ padicValInt 2 (gridNode a s)=j} := by
        rw [Fintype.card_subtype]
        congr 1
        ext s
        simp only [S,v,Finset.mem_filter,Finset.mem_erase,Finset.mem_univ,and_true,true_and,ne_eq,nodeZeroIff]
      _ = _ := actualValuationCardThroughTop a j (Nat.le_of_lt_succ (Finset.mem_range.mp hj))
  calc
    _ = ∑ s∈S.filter (fun s => v s=j), ((j-i:ℕ):ℤ)^2 := by
      apply Finset.sum_congr rfl
      intro s hs
      rw [(Finset.mem_filter.mp hs).2]
    _ = _ := by simp only [Finset.sum_const,nsmul_eq_mul,hc]

theorem weightedSquareMoment (a i : ℕ) (hi : i≤a) :
    (∑ j∈Finset.range (a+1), ((2^(a-j):ℕ):ℚ)*((j-i:ℕ):ℚ)^2)≤6*((2^(a-i):ℕ):ℚ) := by
  let n := a-i
  have hlen : a+1=i+(n+1) := by dsimp [n]; omega
  have hfirst : (∑ j∈Finset.range i, ((2^(a-j):ℕ):ℚ)*((j-i:ℕ):ℚ)^2)=0 := by
    apply Finset.sum_eq_zero
    intro j hj
    have hz : j-i=0 := Nat.sub_eq_zero_of_le (Nat.le_of_lt (Finset.mem_range.mp hj))
    simp [hz]
  have he : (∑ j∈Finset.range (a+1), ((2^(a-j):ℕ):ℚ)*((j-i:ℕ):ℚ)^2) =
      ((2^n:ℕ):ℚ)*(∑ k∈Finset.range (n+1), (k:ℚ)^2/(2:ℚ)^k) := by
    rw [hlen,Finset.sum_range_add,hfirst,zero_add,Finset.mul_sum]
    apply Finset.sum_congr rfl
    intro k hk
    have hkn : k≤n := Nat.le_of_lt_succ (Finset.mem_range.mp hk)
    have hs : a-(i+k)=n-k := by dsimp [n]; omega
    have hp : (2^(n-k):ℕ)*2^k=2^n := by
      rw [←pow_add,Nat.sub_add_cancel hkn]
    have hq : ((2^(n-k):ℕ):ℚ)=((2^n:ℕ):ℚ)/(2:ℚ)^k := by
      apply (eq_div_iff (by positivity : (2:ℚ)^k≠0)).mpr
      exact_mod_cast hp
    rw [hs,Nat.add_sub_cancel_left,hq]
    ring
  rw [he]
  have hb := mul_le_mul_of_nonneg_left (finiteSquareGeometricBound n)
    (by positivity : (0:ℚ)≤((2^n:ℕ):ℚ))
  simpa only [n,mul_comm] using hb

theorem actualSourceMomentBound (a i : ℕ) (hi : i≤a) : sourceMoment a i≤6*(2^(a-i):ℕ) := by
  have he : (sourceMoment a i:ℚ)=
      ∑ j∈Finset.range (a+1), ((2^(a-j):ℕ):ℚ)*((j-i:ℕ):ℚ)^2 := by
    rw [sourceMomentLevelSum]
    push_cast
    rfl
  have hb := weightedSquareMoment a i hi
  rw [←he] at hb
  exact_mod_cast hb

theorem actualHermiteMultiplierLoss (a M u : ℕ) (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) :
    -(6:ℤ)*(sourceCeiling M u:ℤ)*(2^(a-padicValInt 2 (gridNode a t)):ℕ) ≤
      padicValRat 2 ((basis a M u t).eval 0) := by
  have hb := actualBasisMomentBound a M u t
  rw [actualGapMomentEq a t ht0] at hb
  have hm := actualSourceMomentBound a (padicValInt 2 (gridNode a t)) (actualNodeValuationBound a t)
  have hs := mul_le_mul_of_nonneg_left hm (show (0:ℤ)≤2*(sourceCeiling M u:ℤ) by positivity)
  nlinarith only [hb,hs]

end HermiteMoment
#print axioms HermiteMoment.actualValuationCardThroughTop
#print axioms HermiteMoment.finiteSquareGeometricBound
#print axioms HermiteMoment.actualHermiteMultiplierLoss
