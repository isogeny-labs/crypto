import Obstruction

namespace DiemChartAudit

variable {K : Type*} [Field K]

def horizontal (z : K) : Line K :=
  Projectivization.mk K (1,z) (by intro h; have := congrArg Prod.fst h; simp at this)

def vertical : Line K :=
  Projectivization.mk K (0,1) (by intro h; have := congrArg Prod.snd h; simp at this)

def third (z : K) : Line K :=
  Projectivization.mk K (z-1,1) (by intro h; have := congrArg Prod.snd h; simp at this)

theorem line_normal_form (p : Line K) : (∃ z, horizontal z = p) ∨ vertical = p := by
  induction p using Projectivization.ind with
  | h v hv =>
    by_cases hx : v.1 = 0
    · right
      have hy : v.2 ≠ 0 := by intro hy; exact hv (Prod.ext hx hy)
      apply (Projectivization.mk_eq_mk_iff' K _ _ _ _).mpr
      refine ⟨v.2⁻¹, ?_⟩
      apply Prod.ext
      · simp [hx]
      · simp [hy]
    · left
      refine ⟨v.2/v.1, (Projectivization.mk_eq_mk_iff' K _ _ _ _).mpr ?_⟩
      refine ⟨v.1⁻¹, ?_⟩
      apply Prod.ext
      · simp [hx]
      · simp [div_eq_mul_inv, mul_comm]

theorem horizontal_injective : Function.Injective (horizontal : K → Line K) := by
  intro x y h
  obtain ⟨a, ha⟩ := (Projectivization.mk_eq_mk_iff' K _ _ _ _).mp h
  have ha1 : a = 1 := by simpa using congrArg Prod.fst ha
  simpa [ha1] using (congrArg Prod.snd ha).symm

def threeChart (i : Fin 3) (u : Kˣ) : Line K :=
  if i = 0 then horizontal (u : K)
  else if i = 1 then horizontal ((u : K)-1)
  else third (u : K)

/-- The repair uses the actual three projective coordinate formulas. -/
theorem three_chart_cover (p : Line K) : ∃ i : Fin 3, ∃ u : Kˣ, threeChart i u = p := by
  rcases line_normal_form p with ⟨z, hz⟩ | hv
  · by_cases h : z = 0
    · refine ⟨1, 1, ?_⟩
      simpa [threeChart, h] using hz
    · exact ⟨0, Units.mk0 z h, by simpa [threeChart] using hz⟩
  · exact ⟨2, 1, by simpa [threeChart, third, vertical] using hv⟩

theorem three_chart_surjective :
    Function.Surjective (fun z : Fin 3 × Kˣ => threeChart z.1 z.2) := by
  intro p
  obtain ⟨i,u,h⟩ := three_chart_cover p
  exact ⟨(i,u),h⟩

/-- Every displayed chart is injective as a map of rational point sets. -/
theorem three_chart_injective (i : Fin 3) :
    Function.Injective (threeChart (K := K) i) := by
  intro u v h
  apply Units.ext
  by_cases h0 : i = 0
  · exact horizontal_injective (by simpa [threeChart,h0] using h)
  by_cases h1 : i = 1
  · have he := horizontal_injective (by simpa [threeChart,h0,h1] using h)
    exact sub_left_injective he
  · have he : third (u : K) = third (v : K) := by simpa [threeChart,h0,h1] using h
    obtain ⟨a,ha⟩ := (Projectivization.mk_eq_mk_iff' K _ _ _ _).mp he
    have ha1 : a = 1 := by simpa using congrArg Prod.snd ha
    have : (v : K)-1 = (u : K)-1 := by simpa [ha1] using congrArg Prod.fst ha
    exact (sub_left_injective this).symm

end DiemChartAudit
