import Mathlib.Algebra.Module.Submodule.Basic
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.NumberTheory.Padics.PadicVal.Basic
import Mathlib.Tactic

/-!
# Conductor orthogonality for quadratic orders

Model the order `O_f = ℤ + f O_K` in an integral basis as the submodule
`quadraticOrderSubmodule f ≤ ℤ × ℤ` of pairs whose second coordinate is
divisible by `f`. Under this identification:

* lattice containment is conductor divisibility;
* a degree-`d` isogeny pushforward corresponds to the scaled containment
  `d • O_{f₂} ≤ O_{f₁}`;
* a mutual (forward/dual) step forces `|v_ℓ(f₁) − v_ℓ(f₂)| ≤ v_ℓ(d)`, so
  steps coprime to `ℓ` (in particular of degree `< ℓ`) leave `v_ℓ(f)` fixed.

A conductor-valuation gap therefore forces an `ℓ`-power into the product of
step degrees, and in particular a single step with `ℓ ∣ d` and `ℓ ≤ d`.

**Not formalized here (documented, not axiomatized):** the identification
`End(E) ≃ O_f` for ordinary elliptic curves over finite fields is not in
Mathlib; the lattice statements apply once that bridge is supplied. Robert's
Remark 4.5 (poly(log ℓ) volcano navigation) remains open.
-/

set_option autoImplicit false

namespace ConductorOrthogonality

/-- Quadratic order of conductor `f` as a `ℤ`-lattice in `ℤ × ℤ`
(basis `1, ω`): elements `a + f b ω`. -/
def quadraticOrderSubmodule (f : ℕ) : Submodule ℤ (ℤ × ℤ) where
  carrier := { p | (f : ℤ) ∣ p.2 }
  add_mem' := fun {_ _} ha hb => dvd_add ha hb
  zero_mem' := dvd_zero _
  smul_mem' := fun _ _ hp => dvd_mul_of_dvd_right hp _

theorem mem_quadraticOrderSubmodule {f : ℕ} {p : ℤ × ℤ} :
    p ∈ quadraticOrderSubmodule f ↔ (f : ℤ) ∣ p.2 := Iff.rfl

theorem quadraticOrderSubmodule_mono {f₁ f₂ : ℕ} (h : f₁ ∣ f₂) :
    quadraticOrderSubmodule f₂ ≤ quadraticOrderSubmodule f₁ := by
  intro p hp
  exact dvd_trans (Int.natCast_dvd_natCast.mpr h) hp

/-- Lattice containment is conductor divisibility. -/
theorem quadraticOrderSubmodule_le_iff {f₁ f₂ : ℕ} :
    quadraticOrderSubmodule f₂ ≤ quadraticOrderSubmodule f₁ ↔ f₁ ∣ f₂ := by
  constructor
  · intro h
    have : ((0 : ℤ), (f₂ : ℤ)) ∈ quadraticOrderSubmodule f₁ :=
      h (by simp [mem_quadraticOrderSubmodule])
    exact Int.natCast_dvd_natCast.mp this
  · exact quadraticOrderSubmodule_mono

/-- Image of `O_f` under coordinatewise multiplication by `d`. -/
def smulQuadraticOrder (d f : ℕ) : Submodule ℤ (ℤ × ℤ) where
  carrier := { p | ∃ q : ℤ × ℤ, (f : ℤ) ∣ q.2 ∧ p = ((d : ℤ) * q.1, (d : ℤ) * q.2) }
  zero_mem' := by
    refine ⟨(0 : ℤ × ℤ), dvd_zero (f : ℤ), ?_⟩
    simp [Prod.ext_iff]
  add_mem' := by
    rintro p₁ p₂ ⟨q₁, hq₁, rfl⟩ ⟨q₂, hq₂, rfl⟩
    refine ⟨q₁ + q₂, dvd_add hq₁ hq₂, ?_⟩
    simp [mul_add]
  smul_mem' := by
    rintro c p ⟨q, hq, rfl⟩
    refine ⟨(c * q.1, c * q.2), dvd_mul_of_dvd_right hq c, ?_⟩
    simp [mul_left_comm]

theorem mem_smulQuadraticOrder {d f : ℕ} {p : ℤ × ℤ} :
    p ∈ smulQuadraticOrder d f ↔
      ∃ q : ℤ × ℤ, (f : ℤ) ∣ q.2 ∧ p = ((d : ℤ) * q.1, (d : ℤ) * q.2) :=
  Iff.rfl

/-- Scaled containment `d • O_{f₂} ≤ O_{f₁}` is `f₁ ∣ d · f₂`. -/
theorem smul_quadraticOrderSubmodule_le_iff {f₁ f₂ d : ℕ} :
    smulQuadraticOrder d f₂ ≤ quadraticOrderSubmodule f₁ ↔ f₁ ∣ d * f₂ := by
  constructor
  · intro h
    have hmem : ((0 : ℤ), (d : ℤ) * (f₂ : ℤ)) ∈ quadraticOrderSubmodule f₁ := by
      refine h ⟨(0, (f₂ : ℤ)), dvd_refl _, ?_⟩
      simp
    have : (f₁ : ℤ) ∣ (d : ℤ) * (f₂ : ℤ) := mem_quadraticOrderSubmodule.mp hmem
    exact Int.natCast_dvd_natCast.mp (by simpa [Nat.cast_mul] using this)
  · intro hdvd p hp
    rcases hp with ⟨q, hq, rfl⟩
    change (f₁ : ℤ) ∣ (d : ℤ) * q.2
    obtain ⟨k, hk⟩ := hq
    rw [hk, ← mul_assoc]
    exact dvd_mul_of_dvd_left
      (by simpa [Nat.cast_mul] using Int.natCast_dvd_natCast.mpr hdvd) _

/-- A mutual conductor step of degree `d` between orders `O_{f₁}` and `O_{f₂}`. -/
structure ValidConductorStep (f₁ f₂ d : ℕ) : Prop where
  forward : f₁ ∣ d * f₂
  backward : f₂ ∣ d * f₁

theorem ValidConductorStep.of_smul {f₁ f₂ d : ℕ}
    (hf : smulQuadraticOrder d f₂ ≤ quadraticOrderSubmodule f₁)
    (hb : smulQuadraticOrder d f₁ ≤ quadraticOrderSubmodule f₂) :
    ValidConductorStep f₁ f₂ d :=
  ⟨(smul_quadraticOrderSubmodule_le_iff).1 hf,
   (smul_quadraticOrderSubmodule_le_iff).1 hb⟩

theorem ValidConductorStep.to_smul_forward {f₁ f₂ d : ℕ}
    (h : ValidConductorStep f₁ f₂ d) :
    smulQuadraticOrder d f₂ ≤ quadraticOrderSubmodule f₁ :=
  (smul_quadraticOrderSubmodule_le_iff).2 h.forward

theorem ValidConductorStep.to_smul_backward {f₁ f₂ d : ℕ}
    (h : ValidConductorStep f₁ f₂ d) :
    smulQuadraticOrder d f₁ ≤ quadraticOrderSubmodule f₂ :=
  (smul_quadraticOrderSubmodule_le_iff).2 h.backward

private theorem padicValNat_le_of_dvd {ℓ a b : ℕ} [Fact ℓ.Prime]
    (hb : b ≠ 0) (h : a ∣ b) : padicValNat ℓ a ≤ padicValNat ℓ b := by
  have ha : a ≠ 0 := fun ha => by subst ha; exact hb (Nat.eq_zero_of_zero_dvd h)
  have : ℓ ^ padicValNat ℓ a ∣ b := dvd_trans pow_padicValNat_dvd h
  exact (padicValNat_dvd_iff_le hb).1 this

/-- Valuation change across one mutual step is bounded by `v_ℓ(d)`. -/
theorem step_padicValNat_bounds {f₁ f₂ d ℓ : ℕ} [Fact ℓ.Prime]
    (h : ValidConductorStep f₁ f₂ d) (hf₁ : f₁ ≠ 0) (hf₂ : f₂ ≠ 0) (hd : d ≠ 0) :
    padicValNat ℓ f₁ ≤ padicValNat ℓ d + padicValNat ℓ f₂ ∧
      padicValNat ℓ f₂ ≤ padicValNat ℓ d + padicValNat ℓ f₁ := by
  constructor
  · have := padicValNat_le_of_dvd (ℓ := ℓ) (Nat.mul_ne_zero hd hf₂) h.forward
    rwa [padicValNat.mul (p := ℓ) hd hf₂] at this
  · have := padicValNat_le_of_dvd (ℓ := ℓ) (Nat.mul_ne_zero hd hf₁) h.backward
    rwa [padicValNat.mul (p := ℓ) hd hf₁] at this

/-- Steps with `ℓ ∤ d` preserve `v_ℓ(f)`. -/
theorem coprime_step_preserves_padicValNat {f₁ f₂ d ℓ : ℕ} [Fact ℓ.Prime]
    (h : ValidConductorStep f₁ f₂ d) (hf₁ : f₁ ≠ 0) (hf₂ : f₂ ≠ 0) (hd : d ≠ 0)
    (hcop : ¬ ℓ ∣ d) :
    padicValNat ℓ f₁ = padicValNat ℓ f₂ := by
  have hbounds := step_padicValNat_bounds (ℓ := ℓ) h hf₁ hf₂ hd
  have hv : padicValNat ℓ d = 0 := padicValNat.eq_zero_of_not_dvd (p := ℓ) hcop
  omega

/-- An inductive walk of mutual conductor steps. -/
inductive ConductorWalk : ℕ → List ℕ → Type where
  | singleton (f : ℕ) (hf : f ≠ 0) : ConductorWalk f []
  | cons {f₁ f₂ : ℕ} {d : ℕ} {ds : List ℕ}
      (hstep : ValidConductorStep f₁ f₂ d) (hd : d ≠ 0) (hf₁ : f₁ ≠ 0) (hf₂ : f₂ ≠ 0)
      (w : ConductorWalk f₂ ds) : ConductorWalk f₁ (d :: ds)

/-- Every degree on a walk participates in some mutual conductor step. -/
theorem ConductorWalk.toValidStep {f : ℕ} {ds : List ℕ} :
    ∀ (_w : ConductorWalk f ds) (d : ℕ), d ∈ ds →
      ∃ f₁ f₂, ValidConductorStep f₁ f₂ d ∧ f₁ ≠ 0 ∧ f₂ ≠ 0 ∧ d ≠ 0
  | singleton _ _, d, hd => by cases hd
  | cons hstep hd hf₁ hf₂ w, d, hmem => by
    rcases List.mem_cons.1 hmem with rfl | htail
    · exact ⟨_, _, hstep, hf₁, hf₂, hd⟩
    · exact ConductorWalk.toValidStep w d htail

/-- Product of step degrees along a walk. -/
def ConductorWalk.degreeProd {f : ℕ} {ds : List ℕ} (_ : ConductorWalk f ds) : ℕ := ds.prod

/-- Endpoint conductor of a walk. -/
def ConductorWalk.endpoint : {f : ℕ} → {ds : List ℕ} → ConductorWalk f ds → ℕ
  | f, [], singleton _ _ => f
  | _, _ :: _, cons _ _ _ _ w => w.endpoint

theorem ConductorWalk.endpoint_ne_zero {f : ℕ} {ds : List ℕ} :
    ∀ w : ConductorWalk f ds, w.endpoint ≠ 0
  | singleton _ hf => hf
  | cons _ _ _ _ w => w.endpoint_ne_zero

private theorem degrees_ne_zero {f : ℕ} {ds : List ℕ} :
    ∀ _w : ConductorWalk f ds, 0 ∉ ds
  | ConductorWalk.singleton _ _ => by intro h; cases h
  | ConductorWalk.cons _ hd _ _ w => by
      intro h
      rcases List.mem_cons.1 h with rfl | htail
      · exact hd rfl
      · exact degrees_ne_zero w htail

/-- Bound relating start/end valuations to the degree-product valuation. -/
theorem walk_padicValNat_bound {ℓ f : ℕ} [Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds) :
    padicValNat ℓ f ≤ padicValNat ℓ ds.prod + padicValNat ℓ w.endpoint ∧
      padicValNat ℓ w.endpoint ≤ padicValNat ℓ ds.prod + padicValNat ℓ f := by
  induction w with
  | singleton f hf =>
      simp [ConductorWalk.endpoint]
  | @cons f₁ f₂ d ds hstep hd hf₁ hf₂ w ih =>
      have hb := step_padicValNat_bounds (ℓ := ℓ) hstep hf₁ hf₂ hd
      have hds0 : ds.prod ≠ 0 ∨ ds = [] := by
        by_cases h : ds = []
        · exact Or.inr h
        · exact Or.inl (List.prod_ne_zero (degrees_ne_zero w))
      have hmul :
          padicValNat ℓ (d * ds.prod) = padicValNat ℓ d + padicValNat ℓ ds.prod := by
        rcases hds0 with hne | hempty
        · exact padicValNat.mul (p := ℓ) hd hne
        · subst hempty; simp
      change
        padicValNat ℓ f₁ ≤ padicValNat ℓ (d * ds.prod) + padicValNat ℓ w.endpoint ∧
          padicValNat ℓ w.endpoint ≤ padicValNat ℓ (d * ds.prod) + padicValNat ℓ f₁
      rw [hmul]
      omega

/-- If every step degree is coprime to `ℓ`, the walk preserves `v_ℓ`. -/
theorem smooth_walk_preserves_padicValNat {ℓ f : ℕ} [Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds) (hcop : ∀ d ∈ ds, ¬ ℓ ∣ d) :
    padicValNat ℓ f = padicValNat ℓ w.endpoint := by
  induction w with
  | singleton f hf => rfl
  | cons hstep hd hf₁ hf₂ w ih =>
      have hhead : ¬ ℓ ∣ _ := hcop _ (List.mem_cons_self)
      have htail : ∀ d ∈ _, ¬ ℓ ∣ d := fun d hd' => hcop d (List.mem_cons_of_mem _ hd')
      have hstep' := coprime_step_preserves_padicValNat hstep hf₁ hf₂ hd hhead
      rw [hstep', ih htail]
      rfl

/-- Degrees strictly less than a prime `ℓ` leave `v_ℓ(f)` unchanged. -/
theorem small_degree_walk_preserves_padicValNat {ℓ f : ℕ} [hp : Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds) (hsmall : ∀ d ∈ ds, d < ℓ) :
    padicValNat ℓ f = padicValNat ℓ w.endpoint := by
  refine smooth_walk_preserves_padicValNat w ?_
  intro d hdmem
  have hd0 : d ≠ 0 := fun h => degrees_ne_zero w (h ▸ hdmem)
  exact Nat.not_dvd_of_pos_of_lt (Nat.pos_of_ne_zero hd0) (hsmall d hdmem)

/-- A valuation gap between the walk endpoints forces an `ℓ`-power into the
product of step degrees. -/
theorem conductor_gap_forces_prime_power_dvd_prod {ℓ f : ℕ} [Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds)
    (hgap : padicValNat ℓ f ≠ padicValNat ℓ w.endpoint) :
    ∃ k > 0, ℓ ^ k ∣ ds.prod := by
  have bound := walk_padicValNat_bound (ℓ := ℓ) w
  have vpos : 0 < padicValNat ℓ ds.prod := by omega
  exact ⟨padicValNat ℓ ds.prod, vpos, pow_padicValNat_dvd⟩

/-- Ascending gap (`v_ℓ` strictly increases) likewise forces an `ℓ`-power. -/
theorem ascending_gap_forces_prime_power_dvd_prod {ℓ f : ℕ} [Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds)
    (hasc : padicValNat ℓ f < padicValNat ℓ w.endpoint) :
    ∃ k > 0, ℓ ^ k ∣ ds.prod :=
  conductor_gap_forces_prime_power_dvd_prod w (ne_of_lt hasc)

private theorem exists_dvd_of_prime_dvd_prod {ℓ : ℕ} (hℓ : Nat.Prime ℓ) :
    ∀ ds : List ℕ, ℓ ∣ ds.prod → ∃ d ∈ ds, ℓ ∣ d
  | [], h => absurd (by simpa using h) hℓ.ne_one
  | d :: ds, h => by
      rw [List.prod_cons] at h
      rcases hℓ.dvd_mul.1 h with hd | hds
      · exact ⟨d, List.mem_cons_self, hd⟩
      · obtain ⟨e, he, hdvd⟩ := exists_dvd_of_prime_dvd_prod hℓ ds hds
        exact ⟨e, List.mem_cons_of_mem _ he, hdvd⟩

/-- A valuation gap forces some step with `ℓ ∣ d` and hence `ℓ ≤ d`. -/
theorem conductor_gap_forces_large_step {ℓ f : ℕ} [hp : Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorWalk f ds)
    (hgap : padicValNat ℓ f ≠ padicValNat ℓ w.endpoint) :
    ∃ d ∈ ds, ℓ ∣ d ∧ ℓ ≤ d := by
  obtain ⟨k, hk, hdiv⟩ := conductor_gap_forces_prime_power_dvd_prod w hgap
  have hℓd : ℓ ∣ ds.prod := dvd_trans (dvd_pow_self ℓ (ne_of_gt hk)) hdiv
  obtain ⟨d, hdmem, hdvd⟩ := exists_dvd_of_prime_dvd_prod hp.out ds hℓd
  have hdpos : 0 < d := by
    obtain ⟨f₁, f₂, _, _, _, hd0⟩ := w.toValidStep d hdmem
    exact Nat.pos_of_ne_zero hd0
  exact ⟨d, hdmem, hdvd, Nat.le_of_dvd hdpos hdvd⟩

end ConductorOrthogonality
