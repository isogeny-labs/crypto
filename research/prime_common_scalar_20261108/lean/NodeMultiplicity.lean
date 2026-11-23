module
public import PackedContraction
public import Mathlib.Algebra.Order.Floor.Div
public section
namespace NodeMultiplicity

/-- A residue class in a complete interval has exactly one index per block. -/
@[expose] def residueEquiv (n d r : ℕ) (hd : 0<d) (hr : r<d) :
    {m : Fin (n*d) // m.val % d = r} ≃ Fin n where
  toFun m := ⟨m.val.val/d, (Nat.div_lt_iff_lt_mul hd).mpr m.val.isLt⟩
  invFun q := ⟨⟨q.val*d+r, by
    have hq := Nat.mul_le_mul_right d q.isLt
    nlinarith⟩, by simp [Nat.add_mod,Nat.mul_mod,Nat.mod_eq_of_lt hr]⟩
  left_inv m := by
    apply Subtype.ext
    apply Fin.ext
    change m.val.val/d*d+r=m.val.val
    simpa only [m.property,Nat.mul_comm] using Nat.div_add_mod m.val.val d
  right_inv q := by
    apply Fin.ext
    change (q.val*d+r)/d=q.val
    rw [Nat.add_comm,Nat.add_mul_div_right _ _ hd,Nat.div_eq_of_lt hr,zero_add]

theorem residueCard (n d r : ℕ) (hd : 0<d) (hr : r<d) :
    Fintype.card {m : Fin (n*d) // m.val % d = r} = n := by
  exact (Fintype.card_congr (residueEquiv n d r hd hr)).trans (Fintype.card_fin n)

@[expose] def gridNode (a : ℕ) (m : Fin (2^(a+1))) : ℤ := (m.val:ℤ)-(2^a:ℤ)

/-- Active levels have i<a, so shifting by L preserves this residue. -/
theorem shiftedNodeResidue (a i : ℕ) (hi : i<a) (m : Fin (2^(a+1))) :
    gridNode a m % (2:ℤ)^(i+1) = (m.val % 2^(i+1):ℕ) := by
  have hd : (2:ℤ)^(i+1) ∣ (2:ℤ)^a := pow_dvd_pow 2 (by omega)
  simp only [gridNode,Int.sub_emod,Int.emod_eq_zero_of_dvd hd,sub_zero,Int.emod_emod]
  norm_cast

/-- The actual shifted source-grid level, expressed by its exact binary residue. -/
@[expose] def activeLevel (a i : ℕ) :=
  {m : Fin (2^(a+1)) // gridNode a m % (2:ℤ)^(i+1) = (2:ℤ)^i}

instance (a i : ℕ) : Fintype (activeLevel a i) := inferInstanceAs (Fintype {m : Fin (2^(a+1)) // _})

theorem actualLevelCard (a i : ℕ) (hi : i<a) :
    Fintype.card (activeLevel a i) = 2^(a-i) := by
  have hp : 2^(a+1)=2^(a-i)*2^(i+1) := by rw [←pow_add]; congr 1; omega
  have he : activeLevel a i ≃ {m : Fin (2^(a+1)) // m.val % 2^(i+1)=2^i} :=
    Equiv.subtypeEquivRight (fun m => by
      rw [shiftedNodeResidue a i hi m]
      norm_cast)
  rw [Fintype.card_congr he]
  rw [hp]
  exact residueCard (2^(a-i)) (2^(i+1)) (2^i)
    (by positivity) (by
      rw [pow_succ]
      have h : 0<2^i := by positivity
      omega)

/-- Exact residue characterization of a nonzero integer's two-adic level. -/
theorem binaryResidueIff (t : ℤ) (i : ℕ) :
    t % (2:ℤ)^(i+1) = (2:ℤ)^i ↔ t≠0 ∧ padicValInt 2 t=i := by
  have hb : 0<(2:ℤ)^i := by positivity
  have hc : (2:ℤ)^(i+1) = (2:ℤ)^i*2 := pow_succ _ _
  have hdiv : (2:ℤ)^i ∣ (2:ℤ)^(i+1) := pow_dvd_pow 2 (by omega)
  constructor
  · intro h
    have hn : t≠0 := by
      intro ht
      simp only [ht,Int.zero_emod] at h
      omega
    have hd : (2:ℤ)^i ∣ t := by
      have he := Int.emod_add_mul_ediv t ((2:ℤ)^(i+1))
      rw [h] at he
      refine ⟨1+2*(t/((2:ℤ)^(i+1))),?_⟩
      calc
        t = (2:ℤ)^i+(2:ℤ)^(i+1)*(t/((2:ℤ)^(i+1))) := he.symm
        _ = _ := by rw [hc]; ring
    have hnd : ¬(2:ℤ)^(i+1) ∣ t := by
      intro hdt
      rw [Int.emod_eq_zero_of_dvd hdt] at h
      omega
    have hl := (padicValInt_dvd_iff_of_ne_one (p:=2) (by decide) i t).mp hd
    have hu : ¬i+1≤padicValInt 2 t := by
      intro hv
      exact hnd ((padicValInt_dvd_iff_of_ne_one (p:=2) (by decide) (i+1) t).mpr (Or.inr hv))
    exact ⟨hn,by omega⟩
  · rintro ⟨hn,hv⟩
    have hd : (2:ℤ)^i ∣ t := (padicValInt_dvd_iff_of_ne_one (p:=2) (by decide) i t).mpr (Or.inr (by omega))
    have hnd : ¬(2:ℤ)^(i+1) ∣ t := by
      intro hdt
      have hx := (padicValInt_dvd_iff_of_ne_one (p:=2) (by decide) (i+1) t).mp hdt
      omega
    have hr0 := Int.emod_nonneg t (by positivity : (2:ℤ)^(i+1)≠0)
    have hrlt := Int.emod_lt_of_pos t (by positivity : 0<(2:ℤ)^(i+1))
    have hrdiv : (2:ℤ)^i ∣ t % (2:ℤ)^(i+1) := by
      have hsub : (2:ℤ)^i ∣ t % (2:ℤ)^(i+1)-t :=
        dvd_trans hdiv Int.dvd_emod_sub_self
      simpa only [sub_add_cancel] using dvd_add hsub hd
    have hrne : t % (2:ℤ)^(i+1) ≠ 0 := by
      exact fun h => hnd (Int.dvd_of_emod_eq_zero h)
    obtain ⟨k,hk⟩ := hrdiv
    have hrpos : 0<t % (2:ℤ)^(i+1) := by omega
    have hrlt2 : t % (2:ℤ)^(i+1)<(2:ℤ)^i*2 := by simpa only [hc] using hrlt
    have hkpos : 0<k := by nlinarith
    have hklt : k<2 := by nlinarith
    have hk1 : k=1 := by omega
    simpa [hk1] using hk

theorem actualValuationCard (a i : ℕ) (hi : i<a) :
    Fintype.card {m : Fin (2^(a+1)) // gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=i} =
      2^(a-i) := by
  have he : activeLevel a i ≃
      {m : Fin (2^(a+1)) // gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=i} :=
    Equiv.subtypeEquivRight (fun m => binaryResidueIff (gridNode a m) i)
  rw [←Fintype.card_congr he]
  exact actualLevelCard a i hi

@[expose] noncomputable def taperTotal (a h u : ℕ) : ℕ :=
  ∑ i ∈ Finset.range u, Fintype.card (activeLevel a i)*h*(u-i)

/-- An all-level count, connected to the actual grid, exceeds the repaired
degree. The first three levels certify a lower bound on the full sum. -/
theorem actualTaperMargin (a h u M : ℕ) (hu : 3≤u) (hua : u≤a) (hh : 0<h)
    (hbudget : 3*M≤h*u) : 2*M*(2^(a+1)-1) < taperTotal a h u := by
  have hc0 := actualLevelCard a 0 (by omega)
  have hc1 := actualLevelCard a 1 (by omega)
  have hc2 := actualLevelCard a 2 (by omega)
  have hp : 2^a=4*2^(a-2) := by
    calc
      _ = 2^((a-2)+2) := by congr 1; omega
      _ = _ := by rw [pow_add]; norm_num; ring
  have hp1 : 2^(a-1)=2*2^(a-2) := by
    calc
      _ = 2^((a-2)+1) := by congr 1; omega
      _ = _ := by rw [pow_add]; norm_num; ring
  have hs : (∑ i ∈ Finset.range 3, Fintype.card (activeLevel a i)*h*(u-i)) ≤ taperTotal a h u := by
    unfold taperTotal
    exact Finset.sum_le_sum_of_subset_of_nonneg (Finset.range_mono hu) (fun _ _ _ => Nat.zero_le _)
  have hm := RationalScalar.three_level_margin (h:ℚ) (u:ℚ) (M:ℚ)
    (by exact_mod_cast hu) (by exact_mod_cast hh) (by exact_mod_cast hbudget)
  have hfirst : ((∑ i ∈ Finset.range 3, Fintype.card (activeLevel a i)*h*(u-i) : ℕ):ℚ) =
      (h:ℚ)*((u:ℚ)+((u:ℚ)-1)/2+((u:ℚ)-2)/4)*(2^a:ℕ) := by
    simp only [Finset.sum_range_succ,Finset.sum_range_zero,zero_add,hc0,hc1,hc2,Nat.sub_zero]
    rw [hp,hp1]
    simp only [Nat.cast_add,Nat.cast_mul,Nat.cast_ofNat]
    rw [Nat.cast_sub (by omega : 1≤u),Nat.cast_sub (by omega : 2≤u)]
    norm_num
    ring
  have hpositive : (0:ℚ)<(2^a:ℕ) := by positivity
  have hscaled := mul_lt_mul_of_pos_right hm hpositive
  rw [←hfirst] at hscaled
  have hstrict : 4*M*2^a < taperTotal a h u := by
    have hsQ : ((∑ i ∈ Finset.range 3, Fintype.card (activeLevel a i)*h*(u-i) : ℕ):ℚ) ≤ taperTotal a h u := by
      exact_mod_cast hs
    have hx := hscaled.trans_le hsQ
    exact_mod_cast hx
  have hdegree : 2*M*(2^(a+1)-1) ≤ 4*M*2^a := by
    rw [pow_succ]
    have hpow : 0<2^a := by positivity
    nlinarith [Nat.sub_le (2^a*2) 1]
  exact hdegree.trans_lt hstrict

@[expose] def sourceCeiling (M u : ℕ) : ℕ := (3*M) ⌈/⌉ u

theorem sourceCeilingBudget (M u : ℕ) (hu : 0<u) :
    3*M≤sourceCeiling M u*u := by
  simpa only [sourceCeiling,nsmul_eq_mul,Nat.cast_id,Nat.mul_comm] using
    (le_smul_ceilDiv (α:=ℕ) (β:=ℕ) (b:=3*M) hu)

@[expose] noncomputable def sourceJetTotal (a h u : ℕ) : ℕ :=
  ∑ m : Fin (2^(a+1)), if gridNode a m=0 then 0 else h*(u-padicValInt 2 (gridNode a m))

/-- The level sum is the actual D.5 multiplicity sum over individual grid nodes. -/
theorem sourceJetTotalEq (a h u : ℕ) : sourceJetTotal a h u=taperTotal a h u := by
  classical
  have hw (i : ℕ) : Fintype.card (activeLevel a i)*h*(u-i) =
      ∑ m : Fin (2^(a+1)), if gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=i then h*(u-i) else 0 := by
    have he : activeLevel a i ≃
        {m : Fin (2^(a+1)) // gridNode a m≠0 ∧ padicValInt 2 (gridNode a m)=i} :=
      Equiv.subtypeEquivRight (fun m => binaryResidueIff (gridNode a m) i)
    rw [Fintype.card_congr he,Fintype.card_subtype]
    simp [Finset.sum_ite,Finset.sum_const,nsmul_eq_mul,mul_assoc]
  unfold taperTotal sourceJetTotal
  simp_rw [hw]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro m hm
  by_cases hz : gridNode a m=0
  · simp [hz]
  · simp only [hz,ite_false,ne_eq,not_false_eq_true,true_and]
    have hp (i : ℕ) : (padicValInt 2 (gridNode a m)=i) ↔ i=padicValInt 2 (gridNode a m) := eq_comm
    simp_rw [hp]
    rw [Finset.sum_ite_eq']
    by_cases hv : padicValInt 2 (gridNode a m)<u
    · simp [Finset.mem_range,hv]
    · have hs : u-padicValInt 2 (gridNode a m)=0 := Nat.sub_eq_zero_of_le (by omega)
      simp [Finset.mem_range,hv,hs]

theorem sourceJetDegreeMargin (a u M : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) :
    2*M*(2^(a+1)-1) < sourceJetTotal a (sourceCeiling M u) u := by
  rw [sourceJetTotalEq]
  have hb := sourceCeilingBudget M u (by omega)
  have hh : 0<sourceCeiling M u := by nlinarith
  exact actualTaperMargin a _ u M hu hua hh hb

end NodeMultiplicity
#print axioms NodeMultiplicity.actualLevelCard
#print axioms NodeMultiplicity.actualValuationCard
#print axioms NodeMultiplicity.actualTaperMargin
#print axioms NodeMultiplicity.sourceJetTotalEq
#print axioms NodeMultiplicity.sourceJetDegreeMargin
