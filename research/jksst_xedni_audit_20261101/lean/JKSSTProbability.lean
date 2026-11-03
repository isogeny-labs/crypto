import Mathlib

/-!
# Exact finite probability bounds for the JKSST relation heuristic

This file replaces the paper's expression “approximately `(2 C₂)^r / p`” by
finite cardinality theorems.  The sample space is uniform and finite.  A fixed
nonzero coefficient vector defines a surjective linear functional, so each
fiber has exactly the same cardinality.  We then count the ordinary relation
event and the strictly useful event, and apply a union bound over *distinct
events* (not merely over a list of coefficient vectors).
-/

open scoped BigOperators

namespace JKSSTProbability

open Filter Asymptotics

variable {F ι : Type*} [Field F] [Fintype F] [Fintype ι]

/-! ## A nonzero dot product is uniform -/

/-- The linear functional attached to a coefficient vector. -/
noncomputable def dotLinear (m : ι → F) : (ι → F) →ₗ[F] F :=
  Fintype.linearCombination F m

omit [Fintype F] in
@[simp] theorem dotLinear_apply (m a : ι → F) :
    dotLinear m a = ∑ i, a i * m i :=
  rfl

omit [Fintype F] in
theorem dotLinear_ne_zero {m : ι → F} (hm : m ≠ 0) :
    dotLinear m ≠ 0 := by
  classical
  obtain ⟨i, hi⟩ : ∃ i, m i ≠ 0 := by
    simpa [funext_iff] using hm
  intro h
  have happ := LinearMap.congr_fun h (Pi.single i 1)
  have : m i = 0 := by
    simpa [dotLinear] using happ
  exact hi this

omit [Fintype F] in
theorem dotLinear_surjective {m : ι → F} (hm : m ≠ 0) :
    Function.Surjective (dotLinear m) := by
  exact LinearMap.surjective (dotLinear_ne_zero hm)

/-- A fiber of a dot product, represented as a finset in the uniform sample
space `ι → F`. -/
noncomputable def dotFiber (m : ι → F) (y : F) : Finset (ι → F) :=
  by
    classical
    exact Finset.univ.filter fun a ↦ dotLinear m a = y

theorem mem_dotFiber_iff (m a : ι → F) (y : F) :
    a ∈ dotFiber m y ↔ dotLinear m a = y := by
  classical
  change a ∈ Finset.univ.filter (fun z ↦ dotLinear m z = y) ↔ _
  constructor
  · exact fun h ↦ (Finset.mem_filter.mp h).2
  · exact fun h ↦ Finset.mem_filter.mpr ⟨Finset.mem_univ _, h⟩

omit [Fintype F] in
theorem finrank_ker_dotLinear {m : ι → F} (hm : m ≠ 0) :
    Module.finrank F (LinearMap.ker (dotLinear m)) + 1 = Fintype.card ι := by
  rw [Module.Dual.finrank_ker_add_one_of_ne_zero (dotLinear_ne_zero hm)]
  simp

theorem card_ker_dotLinear {m : ι → F} (hm : m ≠ 0) :
    Nat.card (LinearMap.ker (dotLinear m)) =
      Fintype.card F ^ (Fintype.card ι - 1) := by
  rw [Module.natCard_eq_pow_finrank
    (K := F) (V := LinearMap.ker (dotLinear m))]
  rw [Nat.card_eq_fintype_card]
  congr 1
  have h := finrank_ker_dotLinear hm
  omega

theorem card_dotFiber {m : ι → F} (hm : m ≠ 0) (y : F) :
    (dotFiber m y).card = Fintype.card F ^ (Fintype.card ι - 1) := by
  classical
  let e₁ : ↑(dotFiber m y) ≃ ((dotLinear m).toAddMonoidHom ⁻¹' {y} : Set (ι → F)) :=
    { toFun := fun a ↦ ⟨a, by
        change dotLinear m (a : ι → F) = y
        have ha := a.property
        change (a : ι → F) ∈
          Finset.univ.filter (fun z ↦ dotLinear m z = y) at ha
        exact (Finset.mem_filter.mp ha).2⟩
      invFun := fun a ↦ ⟨a, by
        change (a : ι → F) ∈ dotFiber m y
        simp only [dotFiber, Finset.mem_filter, Finset.mem_univ, true_and]
        have ha : dotLinear m (a : ι → F) = y := a.property
        exact ha⟩
      left_inv := fun _ ↦ rfl
      right_inv := fun _ ↦ rfl }
  let e := e₁.trans
    (AddMonoidHom.fiberEquivKerOfSurjective (dotLinear_surjective hm) y)
  calc
    (dotFiber m y).card = Nat.card ↑(dotFiber m y) := by simp
    _ = Nat.card (LinearMap.ker (dotLinear m)) := Nat.card_congr e
    _ = Fintype.card F ^ (Fintype.card ι - 1) := card_ker_dotLinear hm

/-! ## Exact ordinary-relation union bound -/

/-- The set of samples on which the coefficient vector `m` is a relation. -/
noncomputable def relationEvent (m : ι → F) : Finset (ι → F) :=
  dotFiber m 0

/-- Deduplicate coefficient vectors by the event they define.  In particular,
`m` and `c • m` for `c ≠ 0` contribute only one event. -/
noncomputable def distinctRelationEvents (R : Finset (ι → F)) :
    Finset (Finset (ι → F)) := by
  classical
  exact R.image relationEvent

/-- The union of all relation events in `R`, after event-level deduplication. -/
noncomputable def badRelationSamples (R : Finset (ι → F)) : Finset (ι → F) := by
  classical
  exact (distinctRelationEvents R).biUnion id

theorem card_distinctRelationEvents_le (R : Finset (ι → F)) :
    (distinctRelationEvents R).card ≤ R.card := by
  classical
  unfold distinctRelationEvents
  exact Finset.card_image_le

theorem card_relationEvent {m : ι → F} (hm : m ≠ 0) :
    (relationEvent m).card = Fintype.card F ^ (Fintype.card ι - 1) :=
  card_dotFiber hm 0

/-- If the zero coefficient vector were admitted, its ordinary relation event
would be the entire sample space.  This is why every probabilistic statement
below carries an explicit `m ≠ 0` hypothesis. -/
noncomputable def sampleSpace : Finset (ι → F) := by
  classical
  exact Finset.univ

theorem relationEvent_zero : relationEvent (0 : ι → F) = sampleSpace := by
  classical
  ext a
  simp [relationEvent, sampleSpace, mem_dotFiber_iff, dotLinear_apply]

omit [Fintype F] in
theorem dotLinear_smul (c : F) (m a : ι → F) :
    dotLinear (c • m) a = c * dotLinear m a := by
  classical
  simp only [dotLinear_apply, Pi.smul_apply, smul_eq_mul, Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i _hi
  ring

/-- Nonzero scalar multiples define exactly the same relation hyperplane. -/
theorem relationEvent_smul (c : F) (hc : c ≠ 0) (m : ι → F) :
    relationEvent (c • m) = relationEvent m := by
  classical
  ext a
  simp only [relationEvent, mem_dotFiber_iff, dotLinear_smul]
  exact mul_eq_zero.trans (or_iff_right hc)

/-- Exact finite union bound.  The factor is the number of *distinct event
sets*, and the zero coefficient vector is explicitly forbidden. -/
theorem card_badRelationSamples_le
    (R : Finset (ι → F)) (hR : ∀ m ∈ R, m ≠ 0) :
    (badRelationSamples R).card ≤
      (distinctRelationEvents R).card *
        Fintype.card F ^ (Fintype.card ι - 1) := by
  classical
  apply Finset.card_biUnion_le_card_mul
  intro E hE
  rw [distinctRelationEvents, Finset.mem_image] at hE
  obtain ⟨m, hmR, rfl⟩ := hE
  exact (card_relationEvent (hR m hmR)).le

/-- Cross-multiplied probability statement: under the uniform distribution,
`Pr[bad] ≤ (# distinct relation events) / q`, where `q = |F|`. -/
theorem card_badRelationSamples_mul_card_field_le
    (R : Finset (ι → F)) (hR : ∀ m ∈ R, m ≠ 0) :
    (badRelationSamples R).card * Fintype.card F ≤
      (distinctRelationEvents R).card *
        Fintype.card F ^ Fintype.card ι := by
  classical
  by_cases hEmpty : R = ∅
  · subst R
    simp [badRelationSamples, distinctRelationEvents]
  · obtain ⟨m, hmR⟩ := Finset.nonempty_iff_ne_empty.mpr hEmpty
    obtain ⟨i, _hi⟩ : ∃ i, m i ≠ 0 := by
      simpa [funext_iff] using hR m hmR
    have hι : 1 ≤ Fintype.card ι := Fintype.card_pos_iff.mpr ⟨i⟩
    calc
      (badRelationSamples R).card * Fintype.card F ≤
          ((distinctRelationEvents R).card *
            Fintype.card F ^ (Fintype.card ι - 1)) * Fintype.card F :=
        Nat.mul_le_mul_right _ (card_badRelationSamples_le R hR)
      _ = (distinctRelationEvents R).card *
            Fintype.card F ^ Fintype.card ι := by
        rw [mul_assoc, ← pow_succ]
        congr 2
        omega

/-! ## Exact useful-denominator event -/

/-- Outputs `(A,B)` that yield the scalar `x` and have useful (nonzero)
denominator. -/
noncomputable def usefulOutputs (x : F) : Finset (F × F) := by
  classical
  exact Finset.univ.filter fun z ↦ z.1 = x * z.2 ∧ z.2 ≠ 0

theorem mem_usefulOutputs_iff (x : F) (z : F × F) :
    z ∈ usefulOutputs x ↔ z.1 = x * z.2 ∧ z.2 ≠ 0 := by
  classical
  simp [usefulOutputs]

theorem card_usefulOutputs (x : F) :
    (usefulOutputs x).card = Fintype.card F - 1 := by
  classical
  let e : ↑(usefulOutputs x) ≃ {b : F // b ≠ 0} :=
    { toFun := fun z ↦ ⟨z.1.2, (mem_usefulOutputs_iff x z.1).mp z.2 |>.2⟩
      invFun := fun b ↦ ⟨(x * b.1, b.1),
        (mem_usefulOutputs_iff x _).mpr ⟨rfl, b.2⟩⟩
      left_inv := by
        rintro ⟨⟨A, B⟩, hz⟩
        apply Subtype.ext
        have hAB := (mem_usefulOutputs_iff x (A, B)).mp hz |>.1
        exact Prod.ext hAB.symm rfl
      right_inv := fun _ ↦ rfl }
  calc
    (usefulOutputs x).card = Nat.card ↑(usefulOutputs x) := by simp
    _ = Nat.card {b : F // b ≠ 0} := Nat.card_congr e
    _ = Fintype.card F - 1 := by
      rw [Nat.card_eq_fintype_card]
      rw [Fintype.card_subtype_compl]
      simp

/-- Apply the same dot product independently to both random vectors. -/
noncomputable def pairDot (m : ι → F) : ((ι → F) × (ι → F)) → (F × F) :=
  fun z ↦ (dotLinear m z.1, dotLinear m z.2)

noncomputable def pairDotFiber (m : ι → F) (y : F × F) :
    Finset ((ι → F) × (ι → F)) := by
  classical
  exact Finset.univ.filter fun z ↦ pairDot m z = y

theorem mem_pairDotFiber_iff (m : ι → F) (y : F × F)
    (z : (ι → F) × (ι → F)) :
    z ∈ pairDotFiber m y ↔ pairDot m z = y := by
  classical
  change z ∈ Finset.univ.filter (fun w ↦ pairDot m w = y) ↔ _
  exact Finset.mem_filter.trans (and_iff_right (Finset.mem_univ z))

theorem card_pairDotFiber {m : ι → F} (hm : m ≠ 0) (y : F × F) :
    (pairDotFiber m y).card =
        (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2 := by
  classical
  have heq :
      pairDotFiber m y =
        dotFiber m y.1 ×ˢ dotFiber m y.2 := by
    ext z
    rw [mem_pairDotFiber_iff, Finset.mem_product,
      mem_dotFiber_iff, mem_dotFiber_iff]
    rw [pairDot]
    exact Prod.ext_iff
  rw [heq, Finset.card_product,
    card_dotFiber hm, card_dotFiber hm, pow_two]

theorem card_filter_preimage_of_constant_fiber
    {X Y : Type*} [Fintype X] [Fintype Y] [DecidableEq X] [DecidableEq Y]
    (f : X → Y) (S : Finset Y) (k : ℕ)
    (hfiber : ∀ y ∈ S, (Finset.univ.filter fun x ↦ f x = y).card = k) :
    (Finset.univ.filter fun x ↦ f x ∈ S).card = S.card * k := by
  classical
  rw [← Finset.sum_card_fiberwise_eq_card_filter Finset.univ S f]
  calc
    ∑ y ∈ S, (Finset.univ.filter fun x ↦ f x = y).card =
        ∑ _y ∈ S, k := by
      apply Finset.sum_congr rfl
      intro y hy
      exact hfiber y hy
    _ = S.card * k := by simp

/-- Samples giving a relation for `x` whose denominator is nonzero. -/
noncomputable def usefulEvent (x : F) (m : ι → F) :
    Finset ((ι → F) × (ι → F)) := by
  classical
  exact Finset.univ.filter fun z ↦ pairDot m z ∈ usefulOutputs x

theorem mem_usefulEvent_iff (x : F) (m : ι → F)
    (z : (ι → F) × (ι → F)) :
    z ∈ usefulEvent x m ↔
      dotLinear m z.1 = x * dotLinear m z.2 ∧ dotLinear m z.2 ≠ 0 := by
  classical
  unfold usefulEvent
  rw [Finset.mem_filter, mem_usefulOutputs_iff]
  simp [pairDot]

/-- For fixed nonzero `m`, the useful event has exact probability
`(q - 1) / q²`: its cardinality is `(q-1) q^(2(r-1))` inside a sample space
of size `q^(2r)`. -/
theorem card_usefulEvent {m : ι → F} (hm : m ≠ 0) (x : F) :
    (usefulEvent x m).card =
      (Fintype.card F - 1) *
        (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2 := by
  classical
  unfold usefulEvent
  rw [card_filter_preimage_of_constant_fiber (pairDot m) (usefulOutputs x)
    ((Fintype.card F ^ (Fintype.card ι - 1)) ^ 2)]
  · rw [card_usefulOutputs]
  · intro y _hy
    exact card_pairDotFiber hm y

/-- Nonzero scalar multiples also define exactly the same useful event.  Thus
counting a symmetric coefficient box counts `m` and `-m` twice (at least). -/
theorem usefulEvent_smul (x c : F) (hc : c ≠ 0) (m : ι → F) :
    usefulEvent x (c • m) = usefulEvent x m := by
  classical
  ext z
  rw [mem_usefulEvent_iff, mem_usefulEvent_iff]
  simp only [dotLinear_smul]
  constructor
  · rintro ⟨heq, hden⟩
    constructor
    · apply mul_left_cancel₀ hc
      calc
        c * dotLinear m z.1 = x * (c * dotLinear m z.2) := heq
        _ = c * (x * dotLinear m z.2) := by ring
    · intro hzero
      exact hden (by simp [hzero])
  · rintro ⟨heq, hden⟩
    constructor
    · calc
        c * dotLinear m z.1 = c * (x * dotLinear m z.2) := congrArg (c * ·) heq
        _ = x * (c * dotLinear m z.2) := by ring
    · exact mul_ne_zero hc hden

/-- Deduplicate candidates by their useful event.  This is the concrete
finite analogue of counting projective coefficient directions. -/
noncomputable def distinctUsefulEvents (x : F) (R : Finset (ι → F)) :
    Finset (Finset ((ι → F) × (ι → F))) := by
  classical
  exact R.image (usefulEvent x)

noncomputable def badUsefulSamples (x : F) (R : Finset (ι → F)) :
    Finset ((ι → F) × (ι → F)) := by
  classical
  exact (distinctUsefulEvents x R).biUnion id

/-- The event-level/projective count is never larger than the raw candidate
count.  It can be strictly smaller because scalar multiples coincide. -/
theorem card_distinctUsefulEvents_le (x : F) (R : Finset (ι → F)) :
    (distinctUsefulEvents x R).card ≤ R.card := by
  classical
  unfold distinctUsefulEvents
  exact Finset.card_image_le

/-- Union bound for useful relations.  For `N` distinct event sets, the exact
per-event factor is `(q-1) q^(2(r-1))`, not merely an unspecified
approximation. -/
theorem card_badUsefulSamples_le
    (x : F) (R : Finset (ι → F)) (hR : ∀ m ∈ R, m ≠ 0) :
    (badUsefulSamples x R).card ≤
      (distinctUsefulEvents x R).card *
        ((Fintype.card F - 1) *
          (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2) := by
  classical
  apply Finset.card_biUnion_le_card_mul
  intro E hE
  rw [distinctUsefulEvents, Finset.mem_image] at hE
  obtain ⟨m, hmR, rfl⟩ := hE
  exact (card_usefulEvent (hR m hmR) x).le

/-- End-to-end interface: an algorithm whose success set is contained in the
union of useful-relation events inherits the same exact bound.  No details of
the algorithm are assumed beyond this containment certificate. -/
theorem card_algorithmSuccess_le
    (x : F) (R : Finset (ι → F))
    (success : Finset ((ι → F) × (ι → F)))
    (hR : ∀ m ∈ R, m ≠ 0)
    (hsuccess : success ⊆ badUsefulSamples x R) :
    success.card ≤
      (distinctUsefulEvents x R).card *
        ((Fintype.card F - 1) *
          (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2) :=
  (Finset.card_le_card hsuccess).trans (card_badUsefulSamples_le x R hR)

/-- Cross-multiplied probability form of `card_algorithmSuccess_le`:
`Pr[success] ≤ N (q-1) / q²`, for `N` distinct useful-event sets. -/
theorem card_algorithmSuccess_mul_card_field_sq_le
    (x : F) (R : Finset (ι → F))
    (success : Finset ((ι → F) × (ι → F)))
    (hR : ∀ m ∈ R, m ≠ 0)
    (hsuccess : success ⊆ badUsefulSamples x R) :
    success.card * Fintype.card F ^ 2 ≤
      (distinctUsefulEvents x R).card * (Fintype.card F - 1) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  classical
  by_cases hEmpty : R = ∅
  · subst R
    have hs0 : success = ∅ := by
      apply Finset.eq_empty_iff_forall_notMem.mpr
      intro z hz
      have := hsuccess hz
      simp [badUsefulSamples, distinctUsefulEvents] at this
    subst success
    simp
  · obtain ⟨m, hmR⟩ := Finset.nonempty_iff_ne_empty.mpr hEmpty
    obtain ⟨i, _hi⟩ : ∃ i, m i ≠ 0 := by
      simpa [funext_iff] using hR m hmR
    have hι : 1 ≤ Fintype.card ι := Fintype.card_pos_iff.mpr ⟨i⟩
    have hk :
        Fintype.card F ^ (Fintype.card ι - 1) * Fintype.card F =
          Fintype.card F ^ Fintype.card ι := by
      rw [← pow_succ]
      congr 1
      omega
    calc
      success.card * Fintype.card F ^ 2 ≤
          ((distinctUsefulEvents x R).card *
            ((Fintype.card F - 1) *
              (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2)) *
                Fintype.card F ^ 2 :=
        Nat.mul_le_mul_right _ (card_algorithmSuccess_le x R success hR hsuccess)
      _ = (distinctUsefulEvents x R).card * (Fintype.card F - 1) *
            (Fintype.card F ^ Fintype.card ι) ^ 2 := by
        rw [pow_two, pow_two, ← hk]
        ring

/-! ## Characteristic and denominator caveats -/

/-- Coordinatewise reduction of an integral coefficient vector. -/
def reduceIntVector (m : ι → ℤ) : ι → F := fun i ↦ (m i : F)

omit [Fintype F] [Fintype ι] in
/-- A nonzero short integral vector remains nonzero after reduction provided
its coordinate bound is below the *characteristic*.  Merely assuming
`C < |F|` is not enough for a non-prime finite field. -/
theorem reduceIntVector_ne_zero_of_abs_le
    (m : ι → ℤ) (C : ℕ) (hm : m ≠ 0)
    (hbound : ∀ i, |m i| ≤ (C : ℤ))
    (hC : C < ringChar F) : reduceIntVector (F := F) m ≠ 0 := by
  classical
  obtain ⟨i, hi⟩ : ∃ i, m i ≠ 0 := by
    simpa [funext_iff] using hm
  intro hzero
  have hcast : (m i : F) = 0 := by
    simpa [reduceIntVector] using congrFun hzero i
  have hdvd : (ringChar F : ℤ) ∣ m i :=
    (CharP.intCast_eq_zero_iff F (ringChar F) (m i)).mp hcast
  have hlt : |m i| < (ringChar F : ℤ) := by
    exact lt_of_le_of_lt (hbound i) (by exact_mod_cast hC)
  exact hi (Int.eq_zero_of_abs_lt_dvd hdvd hlt)

/-- Conversion of a `/q` bound to a `/p` bound requires a quantitative
comparison `q ≥ κ p`.  This cross-multiplied form says that
`bad/total ≤ N/q` implies `bad/total ≤ N/(κ p)`. -/
theorem div_card_field_to_div_group_order
    {bad total N q κ p : ℕ}
    (hq : bad * q ≤ N * total) (hcompare : κ * p ≤ q) :
    bad * κ * p ≤ N * total := by
  calc
    bad * κ * p = bad * (κ * p) := by ring
    _ ≤ bad * q := Nat.mul_le_mul_left bad hcompare
    _ ≤ N * total := hq

/-- Size-comparison form of the `/q` to `/p` conversion.  If `p ≤ h*q`,
then a bound `bad/total ≤ N/q` gives `bad/total ≤ h*N/p`.
The premise is an explicit comparison between `p` and `q`; it is not derived
here from an elliptic-curve cofactor. -/
theorem div_card_field_to_base_with_size_factor
    {bad total N q p h : ℕ}
    (hq : bad * q ≤ N * total) (hcompare : p ≤ h * q) :
    bad * p ≤ (h * N) * total := by
  calc
    bad * p ≤ bad * (h * q) := Nat.mul_le_mul_left bad hcompare
    _ = h * (bad * q) := by ring
    _ ≤ h * (N * total) := Nat.mul_le_mul_left h hq
    _ = (h * N) * total := by ring

/-- A fixed positive probability cannot obey a `C₀ / p` upper bound for
all unbounded denominators `p`.  The inequality is written without division:
`success/total ≤ C₀/p` would imply `success*p ≤ C₀*total`. -/
theorem fixed_positive_probability_not_bigO_inverse
    {success total C₀ : ℕ} (hsuccess : 0 < success) :
    ¬ ∀ p : ℕ, success * p ≤ C₀ * total := by
  intro h
  have hp := h (C₀ * total + 1)
  have hlower : C₀ * total + 1 ≤ success * (C₀ * total + 1) :=
    le_mul_of_one_le_left' hsuccess
  omega

/-- A fixed positive real-valued function is not asymptotically bounded by
`p ↦ 1/p` along the natural numbers. Unlike the preceding exact theorem,
this is a literal Mathlib `atTop` Big-O statement. -/
theorem fixed_positive_real_not_bigO_inverse
    (c : ℝ) (hc : 0 < c) :
    ¬ ((fun _ : ℕ ↦ c) =O[atTop] (fun p : ℕ ↦ ((p : ℝ)⁻¹))) := by
  intro h
  have hzero : Tendsto (fun _ : ℕ ↦ c) atTop (nhds 0) :=
    h.trans_tendsto (tendsto_inv_atTop_nhds_zero_nat (𝕜 := ℝ))
  have hc_zero : c = 0 := tendsto_nhds_unique tendsto_const_nhds hzero
  exact (ne_of_gt hc) hc_zero

/-- Probability-specialized form of
`fixed_positive_real_not_bigO_inverse`. -/
theorem fixed_positive_probability_not_bigO_inverse_nat
    {success total : ℕ} (hsuccess : 0 < success) (htotal : 0 < total) :
    ¬ ((fun _ : ℕ ↦ (success : ℝ) / (total : ℝ)) =O[atTop]
      (fun p : ℕ ↦ ((p : ℝ)⁻¹))) := by
  apply fixed_positive_real_not_bigO_inverse
  positivity

/-! ## Waiting-time consequence -/

/-- If every trial has conditional success hazard at most `ε`, then the
survival probability satisfies `S(n+1) ≥ (1-ε) S(n)`.  Consequently the
tail-sum formula for expected waiting time is at least `1/ε`.

The theorem takes the survival sequence and its summability explicitly; this
is the exact analytic interface needed by an algorithmic model. -/
theorem expected_waiting_ge_inv_of_conditional_hazard
    (ε : ℝ) (survival : ℕ → ℝ)
    (hεpos : 0 < ε) (hεone : ε ≤ 1)
    (hzero : survival 0 = 1)
    (hstep : ∀ n, (1 - ε) * survival n ≤ survival (n + 1))
    (hsum : Summable survival) :
    1 / ε ≤ ∑' n, survival n := by
  have hnonneg : 0 ≤ 1 - ε := sub_nonneg.mpr hεone
  have hlt : 1 - ε < 1 := by linarith
  have hpoint : ∀ n : ℕ, (1 - ε) ^ n ≤ survival n := by
    intro n
    induction n with
    | zero => simp [hzero]
    | succ n ih =>
        calc
          (1 - ε) ^ (n + 1) = (1 - ε) * (1 - ε) ^ n := by
            rw [pow_succ, mul_comm]
          _ ≤ (1 - ε) * survival n :=
            mul_le_mul_of_nonneg_left ih hnonneg
          _ ≤ survival (n + 1) := hstep n
  have hgeom := hasSum_geometric_of_lt_one hnonneg hlt
  calc
    1 / ε = ∑' n : ℕ, (1 - ε) ^ n := by
      rw [hgeom.tsum_eq]
      rw [show 1 - (1 - ε) = ε by ring]
      simp [one_div]
    _ ≤ ∑' n, survival n :=
      hgeom.summable.tsum_le_tsum hpoint hsum

/-- Composite-modulus warning: over `ZMod 6`, a nonzero coefficient need not
define a surjective scalar functional.  Thus the field (or unit-pivot)
hypothesis in the exact `/q` theorem is essential. -/
theorem composite_modulus_nonzero_not_surjective :
    (2 : ZMod 6) ≠ 0 ∧
      ¬ Function.Surjective (fun a : ZMod 6 ↦ 2 * a) := by
  decide

end JKSSTProbability
