import Mathlib.Tactic

/-!
# No Möbius coordinates make two consecutive Vélu 2-isogeny x-maps monomial

Study: `research/isogeny_vs_mu_factor_base_20261102/` (PROTOCOL_Q4_SEARCH.md,
"Structural expectation").

The μ_k chain is cheap because, in its natural coordinate, every step is the
monomial map `u ↦ λ u²`. The study's Q4 search tried per-level Möbius
coordinates on Vélu 2-isogeny x-chains, and none reached that shape. This file
proves why. Over any field of characteristic ≠ 2, there are no Möbius
coordinates `M₀, M₁, M₂` on three consecutive levels for which both Vélu steps
become monomial (`u ↦ λ u²` or `u ↦ λ / u²`).

Everything is stated with binary quadratic forms, so points at infinity are
handled uniformly:

* a degree-2 map of `ℙ¹` is a pair of forms `(N, D)`, `(X : Z) ↦ (N : D)`;
* the fibre over a target `(T : W)` is the form `W•N - T•D`;
* a *branch value* is a target whose fibre form has zero discriminant;
* a point is *ramified* when both partial derivatives of a fibre form vanish
  there (a double root of the fibre).

The Vélu x-map with kernel abscissa `x₀` and `v = 3x₀² + a` is
`x ↦ x + v/(x - x₀)`, i.e. the forms `(X² - x₀XZ + vZ², XZ - x₀Z²)`. These are
Vélu's formulas for a 2-isogeny of `y² = x³ + ax + b` with kernel `(x₀, 0)`.
That they define the isogeny's x-map is Vélu's theorem. It is not formalised
here (Mathlib has no isogenies) and is taken as the definition of the step. The
study's harness checks it numerically: every image point lies on the codomain
curve.
-/

namespace VeluObstruction

/-- A binary quadratic form `A X² + B X Z + C Z²`. -/
@[ext] structure BQF (K : Type*) where
  A : K
  B : K
  C : K

/-- A `2 × 2` matrix `[[a, b], [c, d]]`, acting by `(X, Z) ↦ (aX + bZ, cX + dZ)`. -/
structure Mob (K : Type*) where
  a : K
  b : K
  c : K
  d : K

/-- A degree-2 rational map of `ℙ¹` given by two binary quadratic forms. -/
structure DMap (K : Type*) where
  N : BQF K
  D : BQF K

variable {K : Type*} [Field K]

namespace BQF

def disc (q : BQF K) : K := q.B ^ 2 - 4 * q.A * q.C

/-- `∂/∂X` of the form at `(X, Z)`. -/
def dX (q : BQF K) (X Z : K) : K := 2 * q.A * X + q.B * Z

/-- `∂/∂Z` of the form at `(X, Z)`. -/
def dZ (q : BQF K) (X Z : K) : K := q.B * X + 2 * q.C * Z

def smul (q : BQF K) (c : K) : BQF K := ⟨c * q.A, c * q.B, c * q.C⟩

def add (p q : BQF K) : BQF K := ⟨p.A + q.A, p.B + q.B, p.C + q.C⟩

/-- Substitution `X ↦ aX + bZ`, `Z ↦ cX + dZ`. -/
def comp (q : BQF K) (a b c d : K) : BQF K :=
  ⟨q.A * a ^ 2 + q.B * a * c + q.C * c ^ 2,
   2 * q.A * a * b + q.B * (a * d + b * c) + 2 * q.C * c * d,
   q.A * b ^ 2 + q.B * b * d + q.C * d ^ 2⟩

theorem disc_comp (q : BQF K) (a b c d : K) :
    (q.comp a b c d).disc = (a * d - b * c) ^ 2 * q.disc := by
  simp only [disc, comp]; ring

theorem disc_smul (q : BQF K) (c : K) : (q.smul c).disc = c ^ 2 * q.disc := by
  simp only [disc, smul]; ring

theorem dX_smul (q : BQF K) (c X Z : K) : (q.smul c).dX X Z = c * q.dX X Z := by
  simp only [dX, smul]; ring

theorem dZ_smul (q : BQF K) (c X Z : K) : (q.smul c).dZ X Z = c * q.dZ X Z := by
  simp only [dZ, smul]; ring

/-- Chain rule for `∂/∂X` under a linear substitution. -/
theorem dX_comp (q : BQF K) (a b c d X Z : K) :
    (q.comp a b c d).dX X Z
      = a * q.dX (a * X + b * Z) (c * X + d * Z) + c * q.dZ (a * X + b * Z) (c * X + d * Z) := by
  simp only [dX, dZ, comp]; ring

/-- Chain rule for `∂/∂Z` under a linear substitution. -/
theorem dZ_comp (q : BQF K) (a b c d X Z : K) :
    (q.comp a b c d).dZ X Z
      = b * q.dX (a * X + b * Z) (c * X + d * Z) + d * q.dZ (a * X + b * Z) (c * X + d * Z) := by
  simp only [dX, dZ, comp]; ring

/-- If `q ∘ A` has a double root at `P` and `A` is invertible, then `q` has a
double root at `A P`. -/
theorem grad_zero_of_comp (q : BQF K) (a b c d X Z : K) (hdet : a * d - b * c ≠ 0)
    (h1 : (q.comp a b c d).dX X Z = 0) (h2 : (q.comp a b c d).dZ X Z = 0) :
    q.dX (a * X + b * Z) (c * X + d * Z) = 0 ∧ q.dZ (a * X + b * Z) (c * X + d * Z) = 0 := by
  rw [dX_comp] at h1
  rw [dZ_comp] at h2
  have hu : (a * d - b * c) * q.dX (a * X + b * Z) (c * X + d * Z) = 0 := by
    linear_combination d * h1 - c * h2
  have hw : (a * d - b * c) * q.dZ (a * X + b * Z) (c * X + d * Z) = 0 := by
    linear_combination a * h2 - b * h1
  exact ⟨(mul_eq_zero.mp hu).resolve_left hdet, (mul_eq_zero.mp hw).resolve_left hdet⟩

end BQF

def Mob.det (m : Mob K) : K := m.a * m.d - m.b * m.c

/-- The identity matrix. -/
def Mob.id : Mob K := ⟨1, 0, 0, 1⟩

/-- Fibre form of `F` over the target `(T : W)`: `W • N - T • D`. -/
def fibre (F : DMap K) (T W : K) : BQF K := (F.N.smul W).add (F.D.smul (-T))

/-- The map `M' ∘ F ∘ M⁻¹`, using the adjugate of `M` for `M⁻¹`. The adjugate
gives the same projective map, since the two differ by the nonzero scalar
`det M`. -/
def conj (F : DMap K) (m m' : Mob K) : DMap K :=
  { N := ((F.N.comp m.d (-m.b) (-m.c) m.a).smul m'.a).add ((F.D.comp m.d (-m.b) (-m.c) m.a).smul m'.b)
    D := ((F.N.comp m.d (-m.b) (-m.c) m.a).smul m'.c).add ((F.D.comp m.d (-m.b) (-m.c) m.a).smul m'.d) }

/-- `G` is monomial: up to a nonzero scalar, `u ↦ λ u²` or `u ↦ λ / u²`. -/
def IsMonomial (G : DMap K) : Prop :=
  ∃ κ l : K, κ ≠ 0 ∧ l ≠ 0 ∧
    ((G.N = BQF.mk (κ * l) 0 0 ∧ G.D = BQF.mk 0 0 κ) ∨
     (G.N = BQF.mk 0 0 (κ * l) ∧ G.D = BQF.mk κ 0 0))

/-- Vélu's 2-isogeny x-map `x ↦ x + v / (x - x₀)`. -/
def velu (x₀ v : K) : DMap K := ⟨⟨1, -x₀, v⟩, ⟨0, 1, -x₀⟩⟩

/-- The squaring map `u ↦ u²` of the μ_k chain. -/
def squareMap : DMap K := ⟨⟨1, 0, 0⟩, ⟨0, 0, 1⟩⟩

theorem fibre_smul (F : DMap K) (c T W : K) :
    fibre F (c * T) (c * W) = (fibre F T W).smul c := by
  ext <;> simp only [fibre, BQF.smul, BQF.add] <;> ring

/-- Fibres of a conjugated map are the original fibres, pulled back by
the adjugate of the source matrix and scaled by the determinant of the target
matrix. -/
theorem fibre_conj (F : DMap K) (m m' : Mob K) (T₀ W₀ : K) :
    fibre (conj F m m') (m'.a * T₀ + m'.b * W₀) (m'.c * T₀ + m'.d * W₀)
      = ((fibre F T₀ W₀).comp m.d (-m.b) (-m.c) m.a).smul m'.det := by
  ext <;> simp only [fibre, conj, BQF.smul, BQF.add, BQF.comp, Mob.det] <;> ring

theorem velu_fibre_disc (x v T W : K) :
    (fibre (velu x v) T W).disc = (T - x * W) ^ 2 - 4 * v * W ^ 2 := by
  simp only [fibre, velu, BQF.disc, BQF.add, BQF.smul]; ring

theorem velu_fibre_dX (x v T W X Z : K) :
    (fibre (velu x v) T W).dX X Z = 2 * W * X + (-W * x - T) * Z := by
  simp only [fibre, velu, BQF.dX, BQF.add, BQF.smul]; ring

theorem velu_fibre_dZ (x v T W X Z : K) :
    (fibre (velu x v) T W).dZ X Z = (-W * x - T) * X + 2 * (W * v + T * x) * Z := by
  simp only [fibre, velu, BQF.dZ, BQF.add, BQF.smul]; ring

theorem four_ne_zero' (h2 : (2 : K) ≠ 0) : (4 : K) ≠ 0 := by
  rw [show (4 : K) = 2 * 2 by norm_num]
  exact mul_ne_zero h2 h2

/-- Positive control: the squaring map, in the identity coordinates, is monomial.
So the predicate is not vacuous. -/
theorem squareMap_isMonomial : IsMonomial (conj (squareMap : DMap K) Mob.id Mob.id) := by
  refine ⟨1, 1, one_ne_zero, one_ne_zero, Or.inl ⟨?_, ?_⟩⟩ <;>
    ext <;> simp only [conj, squareMap, Mob.id, BQF.comp, BQF.smul, BQF.add] <;> ring

/-- A monomial map has branch values only at `0 = (0 : 1)` and `∞ = (1 : 0)`. -/
theorem monomial_branch (h2 : (2 : K) ≠ 0) {G : DMap K} (hG : IsMonomial G) {T W : K}
    (hd : (fibre G T W).disc = 0) : T * W = 0 := by
  obtain ⟨κ, l, hκ, hl, hcase⟩ := hG
  have h4 : (4 : K) * κ ^ 2 * l ≠ 0 :=
    mul_ne_zero (mul_ne_zero (four_ne_zero' h2) (pow_ne_zero 2 hκ)) hl
  have e : (fibre G T W).disc = (4 * κ ^ 2 * l) * (T * W) := by
    rcases hcase with ⟨hN, hD⟩ | ⟨hN, hD⟩ <;>
      simp only [fibre, BQF.disc, BQF.add, BQF.smul, hN, hD] <;> ring
  rw [e] at hd
  exact (mul_eq_zero.mp hd).resolve_left h4

/-- At `0` and at `∞`, a monomial map has a fibre with a double root. -/
theorem monomial_ramif {G : DMap K} (hG : IsMonomial G) {q₁ q₂ : K} (hq : q₁ * q₂ = 0) :
    ∃ T W : K, (T ≠ 0 ∨ W ≠ 0) ∧ (fibre G T W).dX q₁ q₂ = 0 ∧ (fibre G T W).dZ q₁ q₂ = 0 := by
  obtain ⟨κ, l, -, -, hcase⟩ := hG
  rcases hcase with ⟨hN, hD⟩ | ⟨hN, hD⟩ <;> rcases mul_eq_zero.mp hq with h | h
  · refine ⟨0, 1, Or.inr one_ne_zero, ?_, ?_⟩ <;>
      simp only [fibre, BQF.dX, BQF.dZ, BQF.add, BQF.smul, hN, hD, h] <;> ring
  · refine ⟨1, 0, Or.inl one_ne_zero, ?_, ?_⟩ <;>
      simp only [fibre, BQF.dX, BQF.dZ, BQF.add, BQF.smul, hN, hD, h] <;> ring
  · refine ⟨1, 0, Or.inl one_ne_zero, ?_, ?_⟩ <;>
      simp only [fibre, BQF.dX, BQF.dZ, BQF.add, BQF.smul, hN, hD, h] <;> ring
  · refine ⟨0, 1, Or.inr one_ne_zero, ?_, ?_⟩ <;>
      simp only [fibre, BQF.dX, BQF.dZ, BQF.add, BQF.smul, hN, hD, h] <;> ring

/-- **Core obstruction.** Suppose the next kernel abscissa `x₁` is a branch value of the
first Vélu step (`(x₁ - x₀)² = 4v`) and the second step is non-degenerate
(`v₁ ≠ 0`). Then no Möbius coordinates on the three levels make both steps
monomial. -/
theorem no_two_monomial_steps (h2 : (2 : K) ≠ 0) (x₀ v x₁ v₁ : K)
    (hb : (x₁ - x₀) ^ 2 = 4 * v) (hv₁ : v₁ ≠ 0)
    (m₀ m₁ m₂ : Mob K) (hd₀ : m₀.det ≠ 0) (hd₁ : m₁.det ≠ 0) (hd₂ : m₂.det ≠ 0) :
    ¬ (IsMonomial (conj (velu x₀ v) m₀ m₁) ∧ IsMonomial (conj (velu x₁ v₁) m₁ m₂)) := by
  rintro ⟨hG₁, hG₂⟩
  -- (1) the next kernel abscissa `(x₁ : 1)` is a branch value of step 1
  have hbr : (fibre (velu x₀ v) x₁ 1).disc = 0 := by
    rw [velu_fibre_disc]; linear_combination hb
  -- (2) its level-1 image `Q = M₁ (x₁, 1)` is a branch value of the monomial step 1
  have hQ : (fibre (conj (velu x₀ v) m₀ m₁) (m₁.a * x₁ + m₁.b * 1) (m₁.c * x₁ + m₁.d * 1)).disc
      = 0 := by
    rw [fibre_conj, BQF.disc_smul, BQF.disc_comp, hbr]; ring
  -- (3) so `Q ∈ {0, ∞}`, and there step 2 (monomial) has a double root of some fibre
  have hprod := monomial_branch h2 hG₁ hQ
  obtain ⟨T, W, hTW, hX, hZ⟩ := monomial_ramif hG₂ hprod
  -- (4) pull that fibre back through `M₂` and the point back through `M₁`
  obtain ⟨T₀, hT₀⟩ : ∃ T₀ : K, T₀ = m₂.d * T - m₂.b * W := ⟨_, rfl⟩
  obtain ⟨W₀, hW₀⟩ : ∃ W₀ : K, W₀ = -m₂.c * T + m₂.a * W := ⟨_, rfl⟩
  have e₁ : m₂.a * T₀ + m₂.b * W₀ = m₂.det * T := by rw [hT₀, hW₀]; simp only [Mob.det]; ring
  have e₂ : m₂.c * T₀ + m₂.d * W₀ = m₂.det * W := by rw [hT₀, hW₀]; simp only [Mob.det]; ring
  have hfc := fibre_conj (velu x₁ v₁) m₁ m₂ T₀ W₀
  rw [e₁, e₂, fibre_smul] at hfc
  have hX' : ((fibre (velu x₁ v₁) T₀ W₀).comp m₁.d (-m₁.b) (-m₁.c) m₁.a).dX
      (m₁.a * x₁ + m₁.b * 1) (m₁.c * x₁ + m₁.d * 1) = 0 := by
    have h := congrArg (fun q => BQF.dX q (m₁.a * x₁ + m₁.b * 1) (m₁.c * x₁ + m₁.d * 1)) hfc
    simp only [BQF.dX_smul] at h
    rw [hX, mul_zero] at h
    exact (mul_eq_zero.mp h.symm).resolve_left hd₂
  have hZ' : ((fibre (velu x₁ v₁) T₀ W₀).comp m₁.d (-m₁.b) (-m₁.c) m₁.a).dZ
      (m₁.a * x₁ + m₁.b * 1) (m₁.c * x₁ + m₁.d * 1) = 0 := by
    have h := congrArg (fun q => BQF.dZ q (m₁.a * x₁ + m₁.b * 1) (m₁.c * x₁ + m₁.d * 1)) hfc
    simp only [BQF.dZ_smul] at h
    rw [hZ, mul_zero] at h
    exact (mul_eq_zero.mp h.symm).resolve_left hd₂
  have hadj : m₁.d * m₁.a - -m₁.b * -m₁.c ≠ 0 := by
    intro h; apply hd₁; simp only [Mob.det]; linear_combination h
  obtain ⟨g₁, g₂⟩ := BQF.grad_zero_of_comp _ _ _ _ _ _ _ hadj hX' hZ'
  rw [velu_fibre_dX] at g₁
  rw [velu_fibre_dZ] at g₂
  -- (5) a Vélu step never ramifies at its own kernel abscissa: this forces `T₀ = W₀ = 0`
  have hδ : m₁.a * m₁.d - m₁.b * m₁.c ≠ 0 := hd₁
  have hA : (m₁.a * m₁.d - m₁.b * m₁.c) * (W₀ * x₁ - T₀) = 0 := by
    linear_combination g₁
  have hB : (m₁.a * m₁.d - m₁.b * m₁.c) * (2 * W₀ * v₁) = 0 := by
    linear_combination g₂ + x₁ * g₁
  have hW0 : W₀ = 0 := by
    have h := (mul_eq_zero.mp hB).resolve_left hδ
    rcases mul_eq_zero.mp h with h | h
    · exact (mul_eq_zero.mp h).resolve_left h2
    · exact absurd h hv₁
  have hT0 : T₀ = 0 := by
    have h := (mul_eq_zero.mp hA).resolve_left hδ
    rw [hW0] at h; linear_combination -h
  have hT : T = 0 := by
    have h : m₂.det * T = 0 := by linear_combination -e₁ + m₂.a * hT0 + m₂.b * hW0
    exact (mul_eq_zero.mp h).resolve_left hd₂
  have hW : W = 0 := by
    have h : m₂.det * W = 0 := by linear_combination -e₂ + m₂.c * hT0 + m₂.d * hW0
    exact (mul_eq_zero.mp h).resolve_left hd₂
  rcases hTW with h | h
  · exact h hT
  · exact h hW

/-- The codomain of the Vélu step with kernel `(x₀, 0)` on `y² = x³ + ax + b` is
`y² = x³ + a'x + b'`, with `a' = a - 5v`, `b' = b - 7x₀v` and `v = 3x₀² + a`.
Its 2-torsion cubic factors as `(x + 2x₀)((x - x₀)² - 4v)`. The root `-2x₀` is
the dual kernel; the other two are the branch values `x₀ ± 2√v`. -/
theorem codomain_cubic_factor (a b x₀ x : K) (hroot : x₀ ^ 3 + a * x₀ + b = 0) :
    x ^ 3 + (a - 5 * (3 * x₀ ^ 2 + a)) * x + (b - 7 * x₀ * (3 * x₀ ^ 2 + a))
      = (x + 2 * x₀) * ((x - x₀) ^ 2 - 4 * (3 * x₀ ^ 2 + a)) := by
  linear_combination hroot

/-- **Main theorem (Vélu chains).** Let `K` have characteristic ≠ 2, and let
`E : y² = x³ + ax + b` have a 2-torsion abscissa `x₀` with `v = 3x₀² + a ≠ 0`.
Let `x₁` be any 2-torsion abscissa of the Vélu codomain other than the dual
kernel `-2x₀` (a non-backtracking, i.e. cyclic, next step). Then no Möbius
coordinates `M₀, M₁, M₂` make both Vélu steps monomial. So no per-level
coordinate system turns a cyclic 2-isogeny x-chain of length ≥ 2 into a chain
of monomial (μ_k-type) steps. -/
theorem velu_chain_not_monomial (h2 : (2 : K) ≠ 0) (a b x₀ x₁ : K)
    (hroot₀ : x₀ ^ 3 + a * x₀ + b = 0) (hv : 3 * x₀ ^ 2 + a ≠ 0)
    (hroot₁ : x₁ ^ 3 + (a - 5 * (3 * x₀ ^ 2 + a)) * x₁ + (b - 7 * x₀ * (3 * x₀ ^ 2 + a)) = 0)
    (hnb : x₁ + 2 * x₀ ≠ 0)
    (m₀ m₁ m₂ : Mob K) (hd₀ : m₀.det ≠ 0) (hd₁ : m₁.det ≠ 0) (hd₂ : m₂.det ≠ 0) :
    ¬ (IsMonomial (conj (velu x₀ (3 * x₀ ^ 2 + a)) m₀ m₁) ∧
       IsMonomial (conj (velu x₁ (3 * x₁ ^ 2 + (a - 5 * (3 * x₀ ^ 2 + a)))) m₁ m₂)) := by
  have hfac : (x₁ + 2 * x₀) * ((x₁ - x₀) ^ 2 - 4 * (3 * x₀ ^ 2 + a)) = 0 := by
    rw [← codomain_cubic_factor a b x₀ x₁ hroot₀]; exact hroot₁
  have hb : (x₁ - x₀) ^ 2 = 4 * (3 * x₀ ^ 2 + a) := by
    have h := (mul_eq_zero.mp hfac).resolve_left hnb
    linear_combination h
  have hx : x₁ - x₀ ≠ 0 := by
    intro h
    apply hv
    have h4 : (4 : K) * (3 * x₀ ^ 2 + a) = 0 := by linear_combination (x₁ - x₀) * h - hb
    exact (mul_eq_zero.mp h4).resolve_left (four_ne_zero' h2)
  have hv₁ : 3 * x₁ ^ 2 + (a - 5 * (3 * x₀ ^ 2 + a)) ≠ 0 := by
    have e : 3 * x₁ ^ 2 + (a - 5 * (3 * x₀ ^ 2 + a)) = 2 * (x₁ - x₀) * (x₁ + 2 * x₀) := by
      linear_combination hb
    rw [e]
    exact mul_ne_zero (mul_ne_zero h2 hx) hnb
  exact no_two_monomial_steps h2 x₀ _ x₁ _ hb hv₁ m₀ m₁ m₂ hd₀ hd₁ hd₂

end VeluObstruction
