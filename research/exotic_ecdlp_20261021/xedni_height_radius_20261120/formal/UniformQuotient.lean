import Mathlib

/-! An exact finite quotient replaces an asymptotic height-ball count in the
conditional bounded-relation step of the original xedni argument. -/

namespace XedniUniformQuotient

variable {H F : Type*} [AddCommGroup H] [AddCommGroup F] [Fintype F]
variable {r K : ℕ}

def coefficientBox (r K : ℕ) : Finset (Fin r → ℤ) :=
  Fintype.piFinset fun _ => Finset.Icc (-(K : ℤ)) (K : ℤ)

def eval (points : Fin r → H) (coeff : Fin r → ℤ) : H :=
  ∑ i, coeff i • points i

def Bounded (bound : ℕ) (coeff : Fin r → ℤ) : Prop :=
  ∀ i, (coeff i).natAbs ≤ bound

theorem coefficientBox_card (r K : ℕ) :
    (coefficientBox r K).card = (2 * K + 1) ^ r := by
  classical
  rw [coefficientBox, Fintype.card_piFinset_const]
  simp only [Int.card_Icc]
  congr 1
  omega

theorem coefficientBox_bounded {coeff : Fin r → ℤ}
    (h : coeff ∈ coefficientBox r K) : Bounded K coeff := by
  classical
  intro i
  have hi := (Fintype.mem_piFinset.mp h) i
  simp only [Finset.mem_Icc] at hi
  omega

/-- A finite quotient separates all short values when every nonzero kernel
element lies beyond twice the radius. A coefficient box larger than that
quotient therefore contains two coefficients with the same actual sum. -/
theorem bounded_relation_of_quotient_gap
    (points : Fin r → H) (ρ : H →+ F) (size gap : ℝ) (norm : H → ℝ)
    (htriangle : ∀ x y, norm (x - y) ≤ norm x + norm y)
    (hshort : ∀ coeff ∈ coefficientBox r K, norm (eval points coeff) ≤ size)
    (hkernel : ∀ z, z ≠ 0 → ρ z = 0 → gap ≤ norm z)
    (hgap : size + size < gap)
    (hcard : Fintype.card F < (2 * K + 1) ^ r) :
    ∃ coeff, (∃ i, coeff i ≠ 0) ∧ Bounded (2 * K) coeff ∧
      eval points coeff = 0 := by
  classical
  let box := coefficientBox r K
  let mapToF : (Fin r → ℤ) → F := fun coeff => ρ (eval points coeff)
  have hcard' : (Finset.univ : Finset F).card < box.card := by
    simpa [box, coefficientBox_card] using hcard
  have hmaps : Set.MapsTo mapToF (box : Set (Fin r → ℤ))
      ((Finset.univ : Finset F) : Set F) := by
    intro coeff _
    simp
  obtain ⟨a, ha, b, hb, hab, heq⟩ :=
    Finset.exists_ne_map_eq_of_card_lt_of_maps_to hcard' hmaps
  have hsum : eval points a = eval points b := by
    by_contra hne
    have hρsub : ρ (eval points a - eval points b) = 0 := by
      rw [map_sub]
      exact sub_eq_zero.mpr heq
    have hlower := hkernel (eval points a - eval points b)
      (sub_ne_zero.mpr hne) hρsub
    have hupper := htriangle (eval points a) (eval points b)
    have haR := hshort a (by simpa [box] using ha)
    have hbR := hshort b (by simpa [box] using hb)
    linarith
  have hcoord : ∃ i, a i ≠ b i := by
    by_contra h
    push Not at h
    apply hab
    funext i
    exact h i
  obtain ⟨i, hi⟩ := hcoord
  refine ⟨fun j => a j - b j, ⟨i, sub_ne_zero.mpr hi⟩, ?_, ?_⟩
  · intro j
    calc
      (a j - b j).natAbs ≤ (a j).natAbs + (b j).natAbs :=
        Int.natAbs_sub_le _ _
      _ ≤ K + K := Nat.add_le_add
        (coefficientBox_bounded (by simpa [box] using ha) j)
        (coefficientBox_bounded (by simpa [box] using hb) j)
      _ = 2 * K := by omega
  · unfold eval
    simp_rw [sub_smul]
    rw [Finset.sum_sub_distrib]
    exact sub_eq_zero.mpr hsum

omit [Fintype F] in
/-- The kernel gap follows from divisibility by `N`, linear scaling of the
height norm, and a minimum norm when the `N`-multiple is nonzero. This form
allows the height norm to vanish on torsion annihilated by `N`. -/
theorem kernel_gap_of_divisible (ρ : H →+ F) (norm : H → ℝ)
    (N : ℕ) (m : ℝ)
    (hdiv : ∀ z, ρ z = 0 → ∃ u, z = N • u)
    (hscale : ∀ u, norm (N • u) = (N : ℝ) * norm u)
    (hmin : ∀ u, N • u ≠ 0 → m ≤ norm u) :
    ∀ z, z ≠ 0 → ρ z = 0 → (N : ℝ) * m ≤ norm z := by
  intro z hz hρz
  obtain ⟨u, hu⟩ := hdiv z hρz
  have hune : N • u ≠ 0 := by
    simpa [hu] using hz
  rw [hu, hscale]
  exact mul_le_mul_of_nonneg_left (hmin u hune) (by positivity)

/-- If the rank `d` is below the number `r` of sampled points, a linear
choice of quotient modulus gives a strict finite count for sufficiently
large coefficient radius. -/
theorem rank_quotient_count (T d r c K N : ℕ)
    (hdegree : d < r) (hN : N ≤ c * K)
    (hlarge : T * c ^ d < K) :
    T * N ^ d < (2 * K + 1) ^ r := by
  have hK : 0 < K := by omega
  have hpow : N ^ d ≤ (c * K) ^ d := by gcongr
  have hstrict : (T * c ^ d) * K ^ d < K * K ^ d :=
    Nat.mul_lt_mul_of_pos_right hlarge (pow_pos hK d)
  calc
    T * N ^ d ≤ T * (c * K) ^ d := Nat.mul_le_mul_left T hpow
    _ = (T * c ^ d) * K ^ d := by rw [mul_pow]; ac_rfl
    _ < K * K ^ d := hstrict
    _ = K ^ (d + 1) := by rw [pow_succ]; ac_rfl
    _ ≤ (2 * K + 1) ^ (d + 1) := by gcongr; omega
    _ ≤ (2 * K + 1) ^ r := pow_le_pow_right₀ (by omega) (by omega)

/-- A rank-`d` quotient-size bound and a uniform height gap give an explicit
bounded relation. The quotient map and height laws remain hypotheses. -/
theorem bounded_relation_of_rank_quotient
    (points : Fin r → H) (ρ : H →+ F) (norm : H → ℝ)
    (N T d K : ℕ) (m size : ℝ)
    (htriangle : ∀ x y, norm (x - y) ≤ norm x + norm y)
    (hshort : ∀ coeff ∈ coefficientBox r K, norm (eval points coeff) ≤ size)
    (hdiv : ∀ z, ρ z = 0 → ∃ u, z = N • u)
    (hscale : ∀ u, norm (N • u) = (N : ℝ) * norm u)
    (hmin : ∀ u, N • u ≠ 0 → m ≤ norm u)
    (hgap : size + size < (N : ℝ) * m)
    (hquot : Fintype.card F ≤ T * N ^ d)
    (hcount : T * N ^ d < (2 * K + 1) ^ r) :
    ∃ coeff, (∃ i, coeff i ≠ 0) ∧ Bounded (2 * K) coeff ∧
      eval points coeff = 0 := by
  apply bounded_relation_of_quotient_gap points ρ size ((N : ℝ) * m) norm
    htriangle hshort (kernel_gap_of_divisible ρ norm N m hdiv hscale hmin)
    hgap
  exact lt_of_le_of_lt hquot hcount

#print axioms XedniUniformQuotient.coefficientBox_card
#print axioms XedniUniformQuotient.coefficientBox_bounded
#print axioms XedniUniformQuotient.bounded_relation_of_quotient_gap
#print axioms XedniUniformQuotient.kernel_gap_of_divisible
#print axioms XedniUniformQuotient.rank_quotient_count
#print axioms XedniUniformQuotient.bounded_relation_of_rank_quotient

end XedniUniformQuotient
