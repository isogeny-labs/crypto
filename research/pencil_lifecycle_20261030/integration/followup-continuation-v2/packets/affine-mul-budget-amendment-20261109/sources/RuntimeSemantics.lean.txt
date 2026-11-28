import ProjectiveLawBoundary

/-!
# Executable Lean abstractions for pinned projective-law fragments

The definitions in this file manually abstract a decision order and selected
proof-side intermediates associated with pinned Rust functions. Their theorems
are conditional on the hypotheses stated below. Function-body hashes in
`runtime-bindings.json` identify precise Rust source fragments, but establish
no translation theorem: the Rust compiler and Lean kernel do not share an
intermediate representation.
-/

set_option autoImplicit false
set_option maxRecDepth 10000

namespace ProjectiveLawRuntime

section Branch12

variable {K : Type*} [Field K] [DecidableEq K]

inductive Branch12 where
  | common | cancellation | coincidentYMismatch | tripling
  | coincidentDoublePlusOther
  deriving DecidableEq, Repr

/-- Manual abstraction of the decision order of `add12_weighted_complete`,
after the caller supplies its cleared `k`, ordinate-comparison, and
support-comparison values. -/
def classify12 (k y v x x₄ : K) : Branch12 :=
  if k ≠ 0 then .common
  else if y = -v then .cancellation
  else if y ≠ v then .coincidentYMismatch
  else if x = x₄ then .tripling
  else .coincidentDoublePlusOther

/-- Valid ordinates cannot select Rust's `coincident_y_mismatch` return.
The theorem follows the executable decision tree, rather than proving only a
detached equal-squares lemma. -/
theorem classify12_ne_mismatch_of_valid
    (k y v x x₄ : K) (hvalid : y ^ 2 = v ^ 2) :
    classify12 k y v x x₄ ≠ .coincidentYMismatch := by
  rcases ProjectiveLawBoundary.equal_or_opposite_of_sq_eq_sq y v hvalid with hy | hy
  · subst y
    unfold classify12
    by_cases hk : k ≠ 0
    · rw [if_pos hk]
      intro h
      cases h
    · rw [if_neg hk]
      by_cases hv : v = -v
      · rw [if_pos hv]
        intro h
        cases h
      · rw [if_neg hv]
        have hvv : ¬v ≠ v := not_ne_iff.mpr rfl
        rw [if_neg hvv]
        by_cases hx : x = x₄
        · rw [if_pos hx]
          intro h
          cases h
        · rw [if_neg hx]
          intro h
          cases h
  · unfold classify12
    by_cases hk : k ≠ 0
    · rw [if_pos hk]
      intro h
      cases h
    · rw [if_neg hk, if_pos hy]
      intro h
      cases h

end Branch12

section Branch22Prelude

variable {K : Type*} [Field K] [DecidableEq K]

/-- Manual abstraction of the initial, cross-scaled `add22_weighted` exits.
The `shared` and `nonshared` results leave the later ordinate and C tests
unclassified. -/
inductive Branch22Prelude where
  | sameUInverse | sameUDouble | sameUMixedZeroSlope | sameUMixed
  | sharedZeroQ | shared | nonshared
  deriving DecidableEq, Repr

def classifySameU22 (s₁ t₁ s₂ t₂ : K) : Branch22Prelude :=
  if s₁ = -s₂ ∧ t₁ = -t₂ then .sameUInverse
  else if s₁ = s₂ ∧ t₁ = t₂ then .sameUDouble
  else if s₁ - s₂ = 0 then .sameUMixedZeroSlope
  else .sameUMixed

def classifyDistinctU22 (q₁ r₁ q₂ r₂ : K) : Branch22Prelude :=
  let dq22 := q₁ - q₂
  let dr22 := r₁ - r₂
  let B22 := dq22 * (q₂ * r₁ - q₁ * r₂) - dr22 ^ 2
  if B22 = 0 then
    if dq22 = 0 then .sharedZeroQ else .shared
  else .nonshared

def classify22Prelude (q₁ r₁ s₁ t₁ q₂ r₂ s₂ t₂ : K) : Branch22Prelude :=
  if q₁ = q₂ ∧ r₁ = r₂ then classifySameU22 s₁ t₁ s₂ t₂
  else classifyDistinctU22 q₁ r₁ q₂ r₂

/-- The literal distinct-support decision tree cannot emit the zero-q-delta
error: `B=0`, `dq=0` forces `dr=0`, contrary to the earlier support test. -/
theorem classifyDistinctU22_ne_sharedZeroQ
    (q₁ r₁ q₂ r₂ : K) (hU : ¬ (q₁ = q₂ ∧ r₁ = r₂)) :
    classifyDistinctU22 q₁ r₁ q₂ r₂ ≠ .sharedZeroQ := by
  have hdistinct : q₁ - q₂ ≠ 0 ∨ r₁ - r₂ ≠ 0 := by
    by_cases hq : q₁ = q₂
    · right
      apply sub_ne_zero.mpr
      intro hr
      exact hU ⟨hq, hr⟩
    · left
      exact sub_ne_zero.mpr hq
  dsimp only [classifyDistinctU22]
  split_ifs with hB hdq
  · exact False.elim ((ProjectiveLawBoundary.shared_zero_q_delta_unreachable
        (q₁ - q₂) (r₁ - r₂) q₁ q₂ r₁ r₂ hdistinct hB) hdq)
  · intro h; cases h
  · intro h; cases h

/-- The same-support error exit is ruled out by the two validity
coefficients of `v₂²-v₁² mod u`, under the odd-characteristic precondition.
These hypotheses are supplied; this theorem does not prove that arbitrary
Rust inputs satisfy the Mumford validity invariant. -/
theorem classifySameU22_ne_zeroSlope
    (s₁ t₁ s₂ t₂ : K) (hodd : (2 : K) ≠ 0)
    (hx : (t₂ - t₁) * (s₁ + s₂) = 0)
    (hc : (t₂ - t₁) * (t₂ + t₁) = 0) :
    classifySameU22 s₁ t₁ s₂ t₂ ≠ .sameUMixedZeroSlope := by
  unfold classifySameU22
  split_ifs with hinv heq hk
  · intro h; cases h
  · intro h; cases h
  · have hs : s₁ = s₂ := sub_eq_zero.mp hk
    by_cases ht : t₁ = t₂
    · exact False.elim (heq ⟨hs, ht⟩)
    · have hsum : s₁ + s₂ = 2 * s₁ := by rw [← hs]; ring
      have hx' : (t₂ - t₁) * (2 * s₁) = 0 := by rw [← hsum]; exact hx
      have h : s₁ = 0 ∧ t₂ = -t₁ :=
        ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse
          s₁ t₁ t₂ hodd (Ne.symm ht) hx' hc
      apply False.elim
      apply hinv
      constructor
      · rw [← hs, h.1]; ring
      · rw [h.2]; simp
  · intro h; cases h

end Branch22Prelude

section Generic12

variable {R : Type*} [CommRing R]

structure Generic12Input (R : Type*) where
  X : R
  q : R
  r : R
  s : R
  t : R
  K : R
  A : R
  B : R
  D : R
  Q : R
  R₀ : R

structure Generic12Output (R : Type*) where
  S : R
  T : R
  N : R
  U : R

/-- Manual abstraction of selected cleared generic 1+2 intermediates
associated with `add12_weighted_generic`. No source-equivalence theorem is
claimed. -/
def generic12Runtime (i : Generic12Input R) : Generic12Output R :=
  {
    S := i.A * i.Q - i.B * i.K ^ 2
    T := i.A * i.R₀ * i.K ^ 2 - i.D * i.K ^ 4
    N := i.K * (i.s * i.X + i.t) + i.A * (i.X ^ 2 + i.q * i.X + i.r)
    U := i.K ^ 2 * i.X ^ 2 + i.Q * i.X + i.R₀
  }

/-- The abstraction satisfies the exact cleared ordinate identity conditional
on the supplied equations defining its `B` and `D` intermediates. -/
theorem generic12Runtime_cleared
    (i : Generic12Input R)
    (hB : i.B = i.s * i.K + i.A * i.q)
    (hD : i.D = i.t * i.K + i.A * i.r) :
    let o := generic12Runtime i
    i.K ^ 2 * (i.K ^ 2 * o.N - i.A * o.U) +
      i.K ^ 2 * o.S * i.X + o.T = 0 := by
  simpa [generic12Runtime] using
    ProjectiveLawBoundary.generic12_output_v_clearing
      i.X i.q i.r i.s i.t i.K i.A i.B i.D i.Q i.R₀ hB hD

end Generic12

section Generic22

variable {R : Type*} [CommRing R]

structure Generic22Input (R : Type*) where
  X : R
  q₁ : R
  r₁ : R
  s₁ : R
  t₁ : R
  q₂ : R
  r₂ : R
  s₂ : R
  t₂ : R

structure Generic22Output (R : Type*) where
  dq : R
  dr : R
  ds : R
  dt : R
  B : R
  A : R
  C : R
  u₁ : R
  u₂ : R
  v₁ : R
  v₂ : R
  N : R

/-- Proof-side CRT/interpolant abstraction associated with selected
dehomogenized intermediates of `add22_weighted_common`. In particular, `N` is
a constructed polynomial witness for the congruence proofs below; this
definition does not assert that Rust computes an `N` variable or establish a
derivation from the Rust function. -/
def generic22Runtime (i : Generic22Input R) : Generic22Output R :=
  let dq := i.q₁ - i.q₂
  let dr := i.r₁ - i.r₂
  let ds := i.s₁ - i.s₂
  let dt := i.t₁ - i.t₂
  let B := dq * (i.q₂ * i.r₁ - i.q₁ * i.r₂) - dr ^ 2
  let A := dq * (dt * i.q₂ - ds * i.r₂) - dr * dt
  let C := dq * dt - dr * ds
  let u₁ := i.X ^ 2 + i.q₁ * i.X + i.r₁
  let u₂ := i.X ^ 2 + i.q₂ * i.X + i.r₂
  let v₁ := i.s₁ * i.X + i.t₁
  let v₂ := i.s₂ * i.X + i.t₂
  {
    dq, dr, ds, dt, B, A, C, u₁, u₂, v₁, v₂,
    N := B * v₁ - (C * i.X + A) * u₁
  }

/-- The proof-side cubic witness is congruent to `B*v₁` modulo the first
support polynomial. -/
theorem generic22Runtime_mod_first (i : Generic22Input R) :
    let o := generic22Runtime i
    o.N - o.B * o.v₁ = -(o.C * i.X + o.A) * o.u₁ := by
  simp [generic22Runtime]
  ring

/-- The same numerator is congruent to `B*v₂` modulo the second support
polynomial, with the quotient made explicit. -/
theorem generic22Runtime_mod_second (i : Generic22Input R) :
    let o := generic22Runtime i
    o.N - o.B * o.v₂ =
      (-o.C * i.X - o.C * o.dq - o.A) * o.u₂ := by
  simpa [generic22Runtime] using
    ProjectiveLawBoundary.generic22_interpolates_second
      i.X i.q₁ i.r₁ i.s₁ i.t₁ i.q₂ i.r₂ i.s₂ i.t₂

end Generic22

section DivisorClassInterface

variable {Div Class : Type*} [AddCommGroup Div] [AddCommGroup Class]

/-- Minimal interface still needed from concrete smooth-curve divisor theory.
`Principal` must mean principal divisors for the intended curve; constructing
this interface for Mumford divisors is not done here. -/
structure PrincipalClassInterface where
  classMap : Div →+ Class
  Principal : Div → Prop
  principal_mem_kernel : ∀ d, Principal d → classMap d = 0

/-- The standard residual-divisor class step: if the interpolation function
has principal divisor `input + residual`, the class represented by the input
equals the inverse residual class returned by reduction. -/
theorem residual_represents_input
    (M : PrincipalClassInterface (Div := Div) (Class := Class))
    (input residual : Div)
    (hprincipal : M.Principal (input + residual)) :
    M.classMap input = -M.classMap residual := by
  have hzero : M.classMap (input + residual) = 0 :=
    M.principal_mem_kernel _ hprincipal
  rw [map_add] at hzero
  exact eq_neg_of_add_eq_zero_left hzero

end DivisorClassInterface

end ProjectiveLawRuntime
