import XedniDependentLift
import UniformConstant

/-! Count adaptive lift relations when the selected group varies by sample. -/

namespace XedniVaryingFamily

variable {r q : ℕ} [Fact q.Prime]

/-- Each admitted offset tuple may choose its own additive lift group and
reduction map. A uniform coefficient bound still gives the same exact
reduced-point sample count. -/
theorem varying_lift_sample_count
    (H : (Fin r → ZMod q) → Type*)
    [∀ t, AddCommGroup (H t)]
    (red : ∀ t, H t →+ ZMod q)
    (lifts : ∀ t, Fin r → H t)
    (s : Fin r → ZMod q) (w : ZMod q)
    (admissible : Finset (Fin r → ZMod q))
    (hcompat : ∀ t ∈ admissible, ∀ i,
      red t (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (B : ℕ) (hB : B < q)
    (hwitness : ∀ t ∈ admissible,
      ∃ coeff, XedniObstruction.Nontrivial coeff ∧
        XedniObstruction.Bounded B coeff ∧
        (∑ i, coeff i • lifts t i) = 0) :
    admissible.card * q ≤
      ((2 * B + 1) ^ r - 1) * Nat.card (Fin r → ZMod q) := by
  classical
  let candidates := XedniObstruction.boundedCandidates (r := r) B
  let event := (XedniObstruction.reducedCandidates (q := q) candidates).biUnion
    (XedniObstruction.zeroFiber (F := ZMod q))
  have hsubset : admissible ⊆
      event.preimage (XedniObstruction.xedniSampleMap s w)
        (XedniObstruction.xedniSampleMap_bijective s w).1.injOn := by
    intro t ht
    obtain ⟨coeff, hnon, hsmall, hrel⟩ := hwitness t ht
    have hcoeff : coeff ∈ candidates :=
      (XedniObstruction.mem_boundedCandidates B coeff).2 ⟨hnon, hsmall⟩
    have hred := XedniObstruction.relation_reduces (red t) (lifts t) coeff
      (show XedniObstruction.Relation (lifts t) coeff from hrel)
    have hdot : XedniObstruction.dotHom (fun i => (coeff i : ZMod q))
        (XedniObstruction.xedniSampleMap s w t) = 0 := by
      simpa [XedniObstruction.Relation, XedniObstruction.dotHom,
        hcompat t ht, zsmul_eq_mul] using hred
    apply Finset.mem_preimage.mpr
    apply Finset.mem_biUnion.mpr
    refine ⟨fun i => (coeff i : ZMod q), ?_, ?_⟩
    · exact Finset.mem_image.mpr ⟨coeff, hcoeff, rfl⟩
    · simpa [XedniObstruction.zeroFiber] using hdot
  have hvalid : ∀ coeff ∈ candidates,
      XedniObstruction.Nontrivial coeff ∧
        XedniObstruction.Bounded B coeff := by
    intro coeff hc
    exact (XedniObstruction.mem_boundedCandidates B coeff).1 hc
  have hcount := XedniObstruction.sampled_integer_candidate_union_count
    B hB candidates hvalid s w
  have hcard : candidates.card = (2 * B + 1) ^ r - 1 :=
    XedniObstruction.boundedCandidates_card B
  have hq : Nat.card (ZMod q) = q := by simp
  calc
    admissible.card * q ≤
        (event.preimage (XedniObstruction.xedniSampleMap s w)
          (XedniObstruction.xedniSampleMap_bijective s w).1.injOn).card * q :=
      Nat.mul_le_mul_right q (Finset.card_le_card hsubset)
    _ ≤ candidates.card * Nat.card (Fin r → ZMod q) := by
      simpa [event, hq] using hcount
    _ = ((2 * B + 1) ^ r - 1) * Nat.card (Fin r → ZMod q) := by
      rw [hcard]

/-- If the complete coefficient box is smaller than `q`, a compatible
bounded relation cannot occur for every offset tuple, even with different
selected lift groups and reduction maps. -/
theorem no_uniform_varying_lift_rule
    (H : (Fin r → ZMod q) → Type*)
    [∀ t, AddCommGroup (H t)]
    (red : ∀ t, H t →+ ZMod q)
    (lifts : ∀ t, Fin r → H t)
    (s : Fin r → ZMod q) (w : ZMod q)
    (hcompat : ∀ t i,
      red t (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (B : ℕ) (hB : B < q)
    (hwitness : ∀ t,
      ∃ coeff, XedniObstruction.Nontrivial coeff ∧
        XedniObstruction.Bounded B coeff ∧
        (∑ i, coeff i • lifts t i) = 0)
    (hbudget : (2 * B + 1) ^ r - 1 < q) : False := by
  have hcount := varying_lift_sample_count H red lifts s w Finset.univ
    (by intro t _ i; exact hcompat t i) B hB
    (by intro t _; exact hwitness t)
  have hsample : (Finset.univ : Finset (Fin r → ZMod q)).card =
      Nat.card (Fin r → ZMod q) := Fintype.card_eq_nat_card
  rw [hsample] at hcount
  have hpos : 0 < Nat.card (Fin r → ZMod q) := Nat.card_pos
  have hstrict := Nat.mul_lt_mul_of_pos_left hbudget hpos
  exact (lt_irrefl _ (lt_of_le_of_lt (by simpa [mul_comm] using hcount) hstrict))

/-- The varying-family contradiction follows from uniform rank, torsion,
point-norm, and minimum-norm envelopes. Each selected group may differ. -/
theorem no_uniform_varying_height_lifts
    (A : (Fin r → ZMod q) → Type*)
    [∀ t, AddCommGroup (A t)] [∀ t, Fintype (A t)]
    (d : (Fin r → ZMod q) → ℕ)
    (D T c : ℕ)
    (red : ∀ t, XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t) →+ ZMod q)
    (lifts : ∀ t, Fin r → XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t))
    (s : Fin r → ZMod q) (w : ZMod q)
    (freeNorm : ∀ t, (Fin (d t) → ℤ) → ℝ) (m U : ℝ)
    (hcompat : ∀ t i,
      red t (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (hdegree : ∀ t, d t < r) (hrank : ∀ t, d t ≤ D)
    (htorsion : ∀ t, Fintype.card (A t) ≤ T)
    (hcpos : 0 < c)
    (hzero : ∀ t, freeNorm t 0 = 0)
    (hadd : ∀ t x y,
      freeNorm t (x + y) ≤ freeNorm t x + freeNorm t y)
    (hsmul : ∀ t (a : ℤ) x,
      freeNorm t (a • x) ≤ (a.natAbs : ℝ) * freeNorm t x)
    (hpoint : ∀ t i, freeNorm t (lifts t i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ t (N : ℕ) u,
      freeNorm t (N • u) = (N : ℝ) * freeNorm t u)
    (hkillc : ∀ t (a : A t), c • a = 0)
    (hmin : ∀ t u, u ≠ 0 → m ≤ freeNorm t u)
    (hratio : (2 : ℝ) * r * U < (c : ℝ) * m)
    (hbudget : (2 * (2 * (T * c ^ D + 1)) + 1) ^ r - 1 < q) : False := by
  let B := 2 * (T * c ^ D + 1)
  have hpow : 2 * B + 1 ≤ (2 * B + 1) ^ r := by
    calc
      2 * B + 1 = (2 * B + 1) ^ 1 := by simp
      _ ≤ (2 * B + 1) ^ r := pow_le_pow_right₀ (by omega) (by
        have := hdegree (fun _ => 0)
        omega)
  have hB : B < q := by dsimp [B] at *; omega
  have hwitness (t : Fin r → ZMod q) :
      ∃ coeff, XedniObstruction.Nontrivial coeff ∧
        XedniObstruction.Bounded B coeff ∧
        (∑ i, coeff i • lifts t i) = 0 := by
    obtain ⟨coeff, hn, hb, hr⟩ :=
      XedniUniformConstant.bounded_relation_with_constant (d t) r c
        (lifts t) (freeNorm t) m U (hdegree t) hcpos (hzero t)
        (hadd t) (hsmul t) (hpoint t) hU (hscale t)
        (hkillc t) (hmin t) hratio
    have hpow_le : c ^ d t ≤ c ^ D :=
      pow_le_pow_right₀ (by omega) (hrank t)
    have hproduct : Fintype.card (A t) * c ^ d t ≤ T * c ^ D :=
      Nat.mul_le_mul (htorsion t) hpow_le
    have hsmall : 2 * (Fintype.card (A t) * c ^ d t + 1) ≤ B := by
      dsimp [B]
      omega
    refine ⟨coeff, hn, ?_, ?_⟩
    · intro i
      exact le_trans (hb i) hsmall
    · simpa [XedniUniformQuotient.eval] using hr
  exact no_uniform_varying_lift_rule
    (fun t => XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t))
    red lifts s w hcompat B hB hwitness hbudget

#print axioms XedniVaryingFamily.varying_lift_sample_count
#print axioms XedniVaryingFamily.no_uniform_varying_lift_rule
#print axioms XedniVaryingFamily.no_uniform_varying_height_lifts

end XedniVaryingFamily
