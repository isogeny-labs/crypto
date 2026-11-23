/-
Bridge from the abstract monotonicity theorem of `GrowthLaw.lean` to the concrete tower algebras.

Pinned: Mathlib tag v4.35.0 (commit 56a005855ea17e0b0958f73ec801ae140b0a1679),
toolchain leanprover/lean4:v4.35.0. `GrowthLaw.lean` in this directory is the file
`../GrowthLaw.lean` verbatim, wrapped in `namespace GrowthLaw ... end GrowthLaw` (two added lines)
so that its root-level names cannot clash with Mathlib.

Status: written 2026-11-15, NOT YET BUILT against Mathlib (see receipts/ for the build state).
Nothing in this file may be cited as checked until a successful `lake build` log and the
`#print axioms` output are recorded under `receipts/`.

Part `Uni` (one variable, R_t = F_p[X]/(X^{2^t} - 1)):
* `I_le`, `red`            : the reduction ring hom R_{t+1} → R_t, X ↦ X (from (X^{2^t}-1) ∣ (X^{2^{t+1}}-1)).
* `mk_X_pow_mod`, `red_X_pow` : X^e ↦ X^(e mod 2^t).
* `pop_red_le`             : pop (e mod 2^t) ≤ pop e (GrowthLaw.pop_mod_le).
* `deg`                    : degree of a ∈ R_t := least c such that some representative has all
                             exponents of popcount ≤ c; `deg_red_le` : deg (red a) ≤ deg a.
* `deg_eq_nfDeg`           : (needs p > 1) `deg` equals the max popcount over the support of the
                             normal form (the representative of degree < 2^t), i.e. the degree of
                             PREREGISTRATION §2 on elements; `deg_le` : deg a ≤ t.
* `level`, `levelMap`      : `GrowthLaw.Level` / `GrowthLaw.LevelMap` instances, variables
                             y_j = X^{2^j} (j < t), generators = images of a fixed polynomial set S.
* `lastFall_mono`          : d_F(t) ≤ d_F(t') for t ≤ t' (GrowthLaw.tower_lastFall_mono instantiated).

Part `Kummer` (n variables, R_t = F_p[z_1..z_n]/(z_i^{2^t} - 1); n = 2 is the PKM Kummer ring):
* `I_le`, `red`, `mk_X_pow_mod`, `mk_prod_mod`, `red_prod` (monomial ∏ z_i^{m_i} ↦ ∏ z_i^{m_i mod 2^t}),
  `pop_prod_red_le`, `deg`, `deg_red_le`, `level`, `levelMap`, `lastFall_mono`.

Not covered here (open): the identity of the inductive `GrowthLaw.Closure` with the subspace
definition of PREREGISTRATION §2 (linear span formulation); the normal-form identity of `deg` in the
multivariate part; the HKY system itself (S is an arbitrary polynomial set).
-/
import Mathlib
import GrowthLaw

noncomputable section

namespace Bridge

/-- `y - 1 ∣ y^k - 1` in any commutative ring. -/
theorem sub_one_dvd_pow_sub_one {A : Type*} [CommRing A] (y : A) (k : ℕ) : y - 1 ∣ y ^ k - 1 := by
  induction k with
  | zero => simp
  | succ k ih =>
    have h : y ^ (k + 1) - 1 = y ^ k * (y - 1) + (y ^ k - 1) := by ring
    rw [h]
    exact dvd_add (dvd_mul_left _ _) ih

/-! ## One variable -/

namespace Uni

open Polynomial

variable (p : ℕ)

/-- The modulus `X^(2^t) - 1`. -/
abbrev q (t : ℕ) : (ZMod p)[X] := X ^ (2 ^ t) - 1

/-- The ideal `(X^(2^t) - 1)`. -/
abbrev I (t : ℕ) : Ideal (ZMod p)[X] := Ideal.span {q p t}

/-- `R_t = F_p[X] / (X^(2^t) - 1)`. -/
abbrev R (t : ℕ) : Type := (ZMod p)[X] ⧸ I p t

theorem q_dvd (t : ℕ) : q p t ∣ q p (t + 1) := by
  refine ⟨X ^ (2 ^ t) + 1, ?_⟩
  show (X : (ZMod p)[X]) ^ (2 ^ (t + 1)) - 1 = (X ^ (2 ^ t) - 1) * (X ^ (2 ^ t) + 1)
  rw [Nat.pow_succ, pow_mul]
  ring

theorem I_le (t : ℕ) : I p (t + 1) ≤ I p t :=
  Ideal.span_singleton_le_span_singleton.mpr (q_dvd p t)

/-- The reduction map `R_{t+1} → R_t`, `X ↦ X`. -/
def red (t : ℕ) : R p (t + 1) →+* R p t := Ideal.Quotient.factor (I_le p t)

theorem red_mk (t : ℕ) (f : (ZMod p)[X]) :
    red p t (Ideal.Quotient.mk (I p (t + 1)) f) = Ideal.Quotient.mk (I p t) f :=
  Ideal.Quotient.factor_mk (I_le p t) f

theorem mk_X_pow_eq_one (t : ℕ) : Ideal.Quotient.mk (I p t) ((X : (ZMod p)[X]) ^ (2 ^ t)) = 1 := by
  rw [← map_one (Ideal.Quotient.mk (I p t)), Ideal.Quotient.eq]
  exact Ideal.mem_span_singleton_self _

/-- In `R_t`, `X^e = X^(e mod 2^t)`. -/
theorem mk_X_pow_mod (t e : ℕ) :
    Ideal.Quotient.mk (I p t) ((X : (ZMod p)[X]) ^ e)
      = Ideal.Quotient.mk (I p t) (X ^ (e % 2 ^ t)) := by
  have hsplit : (X : (ZMod p)[X]) ^ e = X ^ (e % 2 ^ t) * (X ^ (2 ^ t)) ^ (e / 2 ^ t) := by
    rw [← pow_mul, ← pow_add, Nat.mod_add_div]
  rw [hsplit, map_mul, map_pow (Ideal.Quotient.mk (I p t)) (X ^ (2 ^ t)), mk_X_pow_eq_one,
    one_pow, mul_one]

/-- The reduction map sends the monomial `X^e` to `X^(e mod 2^t)`. -/
theorem red_X_pow (t e : ℕ) :
    red p t (Ideal.Quotient.mk (I p (t + 1)) ((X : (ZMod p)[X]) ^ e))
      = Ideal.Quotient.mk (I p t) (X ^ (e % 2 ^ t)) := by
  rw [red_mk, mk_X_pow_mod]

/-- Degree bound on monomials: `pop (e mod 2^t) ≤ pop e`. -/
theorem pop_red_le (t e : ℕ) : GrowthLaw.pop (e % 2 ^ t) ≤ GrowthLaw.pop e :=
  GrowthLaw.pop_mod_le t e

/-- `c` bounds the degree of `a`: some representative has every exponent of popcount `≤ c`. -/
def DegBound (t : ℕ) (a : R p t) (c : ℕ) : Prop :=
  ∃ f : (ZMod p)[X], Ideal.Quotient.mk (I p t) f = a ∧ ∀ e ∈ f.support, GrowthLaw.pop e ≤ c

/-- The popcount degree of `a ∈ R_t` (least bound over representatives). -/
def deg (t : ℕ) (a : R p t) : ℕ := sInf {c | DegBound p t a c}

theorem degBound_nonempty (t : ℕ) (a : R p t) : {c | DegBound p t a c}.Nonempty := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective a
  exact ⟨f.support.sup GrowthLaw.pop, f, rfl, fun e he => Finset.le_sup (f := GrowthLaw.pop) he⟩

theorem deg_spec (t : ℕ) (a : R p t) : DegBound p t a (deg p t a) :=
  Nat.sInf_mem (degBound_nonempty p t a)

theorem deg_le_of_rep (t : ℕ) (f : (ZMod p)[X]) (c : ℕ) (hc : ∀ e ∈ f.support, GrowthLaw.pop e ≤ c) :
    deg p t (Ideal.Quotient.mk (I p t) f) ≤ c := by
  unfold deg
  exact Nat.sInf_le ⟨f, rfl, hc⟩

/-- The reduction map does not raise the degree (hypothesis `hdeg` of `GrowthLaw.LevelMap`). -/
theorem deg_red_le (t : ℕ) (a : R p (t + 1)) : deg p t (red p t a) ≤ deg p (t + 1) a := by
  obtain ⟨f, hf, hc⟩ := deg_spec p (t + 1) a
  have h := deg_le_of_rep p t f _ hc
  rw [← red_mk p t f, hf] at h
  exact h

/-- The degree of a monomial of `R_t` is at most `pop (e mod 2^t)`. -/
theorem deg_X_pow_le (t e : ℕ) :
    deg p t (Ideal.Quotient.mk (I p t) ((X : (ZMod p)[X]) ^ e)) ≤ GrowthLaw.pop (e % 2 ^ t) := by
  rw [mk_X_pow_mod]
  apply deg_le_of_rep
  intro e' he'
  rw [mem_support_iff, coeff_X_pow] at he'
  by_cases h : e' = e % 2 ^ t
  · rw [h]
  · exact absurd (if_neg h) he'

/-! ### Normal forms (needs `p > 1`, so that `F_p` is nontrivial) -/

section NormalForm

variable [Fact (1 < p)]

theorem q_monic (t : ℕ) : (q p t).Monic :=
  monic_X_pow_sub_one (Nat.two_pow_pos t).ne'

theorem degree_q (t : ℕ) : (q p t).degree = ((2 ^ t : ℕ) : WithBot ℕ) := by
  have h := degree_X_pow_sub_C (R := ZMod p) (Nat.two_pow_pos t) 1
  rw [C_1] at h
  exact h

/-- Folding exponents modulo `2^t`. -/
def foldExp (t : ℕ) (f : (ZMod p)[X]) : (ZMod p)[X] :=
  ∑ e ∈ f.support, C (f.coeff e) * X ^ (e % 2 ^ t)

theorem mk_fold (t : ℕ) (f : (ZMod p)[X]) :
    Ideal.Quotient.mk (I p t) (foldExp p t f) = Ideal.Quotient.mk (I p t) f := by
  conv_rhs => rw [f.as_sum_support_C_mul_X_pow]
  unfold foldExp
  rw [map_sum, map_sum]
  refine Finset.sum_congr rfl (fun e _ => ?_)
  rw [map_mul, map_mul, mk_X_pow_mod p t e]

theorem coeff_fold (t : ℕ) (f : (ZMod p)[X]) (m : ℕ) :
    (foldExp p t f).coeff m = ∑ e ∈ f.support, if m = e % 2 ^ t then f.coeff e else 0 := by
  unfold foldExp
  rw [finsetSum_coeff]
  refine Finset.sum_congr rfl (fun e _ => ?_)
  rw [coeff_C_mul_X_pow]

theorem degree_fold_lt (t : ℕ) (f : (ZMod p)[X]) : (foldExp p t f).degree < (q p t).degree := by
  rw [degree_q, degree_lt_iff_coeff_zero]
  intro m hm
  rw [coeff_fold]
  apply Finset.sum_eq_zero
  intro e _
  have hlt : e % 2 ^ t < 2 ^ t := Nat.mod_lt _ (Nat.two_pow_pos t)
  exact if_neg (Nat.ne_of_gt (lt_of_lt_of_le hlt hm))

theorem modByMonic_eq_fold (t : ℕ) (f : (ZMod p)[X]) : f %ₘ q p t = foldExp p t f := by
  have h1 : f %ₘ q p t = foldExp p t f %ₘ q p t := by
    apply modByMonic_eq_of_dvd_sub (q_monic p t)
    have h := (mk_fold p t f).symm
    rw [Ideal.Quotient.eq, Ideal.mem_span_singleton] at h
    exact h
  rw [h1, (modByMonic_eq_self_iff (q_monic p t)).mpr (degree_fold_lt p t f)]

theorem support_fold (t : ℕ) (f : (ZMod p)[X]) (e' : ℕ) (he : e' ∈ (foldExp p t f).support) :
    ∃ e ∈ f.support, e' = e % 2 ^ t := by
  rw [mem_support_iff, coeff_fold] at he
  obtain ⟨e, hes, hne⟩ := Finset.exists_ne_zero_of_sum_ne_zero he
  refine ⟨e, hes, ?_⟩
  by_contra h
  exact hne (if_neg h)

theorem mk_modByMonic (t : ℕ) (g : (ZMod p)[X]) :
    Ideal.Quotient.mk (I p t) (g %ₘ q p t) = Ideal.Quotient.mk (I p t) g := by
  have hq0 : Ideal.Quotient.mk (I p t) (q p t) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.mem_span_singleton_self _)
  conv_rhs => rw [← modByMonic_add_div g (q p t)]
  rw [map_add, map_mul, hq0, zero_mul, add_zero]

/-- The normal form of `a ∈ R_t`: its representative of degree `< 2^t`. -/
def nf (t : ℕ) (a : R p t) : (ZMod p)[X] :=
  Classical.choose (Ideal.Quotient.mk_surjective a) %ₘ q p t

theorem mk_nf (t : ℕ) (a : R p t) : Ideal.Quotient.mk (I p t) (nf p t a) = a := by
  unfold nf
  rw [mk_modByMonic]
  exact Classical.choose_spec (Ideal.Quotient.mk_surjective a)

theorem nf_mk (t : ℕ) (f : (ZMod p)[X]) : nf p t (Ideal.Quotient.mk (I p t) f) = foldExp p t f := by
  unfold nf
  rw [← modByMonic_eq_fold]
  apply modByMonic_eq_of_dvd_sub (q_monic p t)
  have h := Classical.choose_spec (Ideal.Quotient.mk_surjective (Ideal.Quotient.mk (I p t) f))
  rw [Ideal.Quotient.eq, Ideal.mem_span_singleton] at h
  exact h

/-- Degree of the normal form: the maximum popcount over its support (PREREGISTRATION §2). -/
def nfDeg (t : ℕ) (a : R p t) : ℕ := (nf p t a).support.sup GrowthLaw.pop

/-- The representative-minimal degree equals the normal-form degree. -/
theorem deg_eq_nfDeg (t : ℕ) (a : R p t) : deg p t a = nfDeg p t a := by
  apply le_antisymm
  · have h := deg_le_of_rep p t (nf p t a) (nfDeg p t a)
      (fun e he => Finset.le_sup (f := GrowthLaw.pop) he)
    rw [mk_nf] at h
    exact h
  · obtain ⟨f, hf, hc⟩ := deg_spec p t a
    unfold nfDeg
    rw [← hf, nf_mk]
    apply Finset.sup_le
    intro e' he'
    obtain ⟨e, hes, rfl⟩ := support_fold p t f e' he'
    exact le_trans (GrowthLaw.pop_mod_le t e) (by rw [hf]; exact hc e hes)

/-- Every element of `R_t` has degree at most `t`. -/
theorem deg_le (t : ℕ) (a : R p t) : deg p t a ≤ t := by
  have h := deg_le_of_rep p t (nf p t a) t (fun e he => by
    apply GrowthLaw.pop_le_of_lt_two_pow
    by_contra hge
    have hdeg : (nf p t a).degree < (e : WithBot ℕ) := by
      have h1 : (nf p t a).degree < ((2 ^ t : ℕ) : WithBot ℕ) := by
        rw [← degree_q p t]
        exact degree_modByMonic_lt _ (q_monic p t)
      exact lt_of_lt_of_le h1 (by exact_mod_cast (Nat.not_lt.mp hge))
    exact (mem_support_iff.mp he) (coeff_eq_zero_of_degree_lt hdeg))
  rw [mk_nf] at h
  exact h

end NormalForm

/-! ### The tower as `GrowthLaw.Level`s and the instantiated monotonicity theorem -/

/-- Filtered data of `R_t`: variables `y_j = X^(2^j)` for `j < t`. -/
def filtered (t : ℕ) : GrowthLaw.FilteredData (ZMod p) (R p t) (Fin t) where
  zero := 0
  add := (· + ·)
  smul k a := Ideal.Quotient.mk (I p t) (C k) * a
  mulv j a := Ideal.Quotient.mk (I p t) (X ^ (2 ^ (j : ℕ))) * a
  deg := deg p t

/-- Level `t` of the tower for the polynomial system `S` (its images in `R_t` are the generators). -/
def level (S : Set (ZMod p)[X]) (t : ℕ) : GrowthLaw.Level (ZMod p) where
  R := R p t
  X := Fin t
  D := filtered p t
  G := fun r => ∃ f ∈ S, Ideal.Quotient.mk (I p t) f = r
  one := 1

/-- The reduction map satisfies every hypothesis of Theorem 1 (`GrowthLaw.LevelMap`). -/
def levelMap (S : Set (ZMod p)[X]) (t : ℕ) : GrowthLaw.LevelMap (level p S (t + 1)) (level p S t) where
  φ := red p t
  hone := map_one (red p t)
  hzero := map_zero (red p t)
  hadd := fun (a b : R p (t + 1)) => map_add (red p t) a b
  hsmul := fun k (a : R p (t + 1)) => by
    show red p t (Ideal.Quotient.mk (I p (t + 1)) (C k) * a)
      = Ideal.Quotient.mk (I p t) (C k) * red p t a
    rw [map_mul, red_mk]
  hmul := fun (x : Fin (t + 1)) => by
    by_cases hx : (x : ℕ) < t
    · left
      refine ⟨(⟨x, hx⟩ : Fin t), fun (a : R p (t + 1)) => ?_⟩
      show red p t (Ideal.Quotient.mk (I p (t + 1)) (X ^ (2 ^ (x : ℕ))) * a)
        = Ideal.Quotient.mk (I p t) (X ^ (2 ^ (x : ℕ))) * red p t a
      rw [map_mul, red_mk]
    · right
      intro (a : R p (t + 1))
      have hxt : (x : ℕ) = t := by have := x.isLt; omega
      show red p t (Ideal.Quotient.mk (I p (t + 1)) (X ^ (2 ^ (x : ℕ))) * a) = red p t a
      rw [map_mul, red_mk, hxt, mk_X_pow_eq_one, one_mul]
  hdeg := deg_red_le p t
  hgen := fun r (hr : ∃ f ∈ S, Ideal.Quotient.mk (I p (t + 1)) f = r) => by
    obtain ⟨f, hfS, hf⟩ := hr
    show ∃ g ∈ S, Ideal.Quotient.mk (I p t) g = red p t r
    exact ⟨f, hfS, by rw [← hf, red_mk]⟩

/-- Theorem 1 on the concrete one-variable tower: `t ≤ t'` implies `d_F(t) ≤ d_F(t')`. -/
theorem lastFall_mono (S : Set (ZMod p)[X]) {t t' c c' : ℕ} (htt : t ≤ t')
    (h : GrowthLaw.IsLastFall (level p S t) c) (h' : GrowthLaw.IsLastFall (level p S t') c') :
    c ≤ c' :=
  GrowthLaw.tower_lastFall_mono (level p S) (levelMap p S) htt h h'

/-- Theorem 1, refutation form: refuting at level `t + d` and degree `c` refutes at level `t`. -/
theorem refutes_descend (S : Set (ZMod p)[X]) (c t d : ℕ) :
    GrowthLaw.Refutes (level p S (t + d)) c → GrowthLaw.Refutes (level p S t) c :=
  GrowthLaw.tower_refutes_descend (level p S) (levelMap p S) c t d

end Uni

/-! ## Several variables: the Kummer ring `F_p[z_1..z_n] / (z_i^(2^t) - 1)` (`n = 2`: PKM) -/

namespace Kummer

open MvPolynomial

variable (p n : ℕ)

abbrev P : Type := MvPolynomial (Fin n) (ZMod p)

abbrev gens (t : ℕ) : Set (P p n) := Set.range fun i : Fin n => (X i : P p n) ^ (2 ^ t) - 1

abbrev I (t : ℕ) : Ideal (P p n) := Ideal.span (gens p n t)

abbrev R (t : ℕ) : Type := P p n ⧸ I p n t

theorem gen_mem (t : ℕ) (i : Fin n) : (X i : P p n) ^ (2 ^ t) - 1 ∈ I p n t :=
  Ideal.subset_span ⟨i, rfl⟩

theorem I_le (t : ℕ) : I p n (t + 1) ≤ I p n t := by
  apply Ideal.span_le.mpr
  rintro _ ⟨i, rfl⟩
  have h : (X i : P p n) ^ (2 ^ (t + 1)) - 1 = (X i ^ (2 ^ t) + 1) * (X i ^ (2 ^ t) - 1) := by
    rw [Nat.pow_succ, pow_mul]
    ring
  have hm := Submodule.smul_mem (I p n t) ((X i : P p n) ^ (2 ^ t) + 1) (gen_mem p n t i)
  rw [smul_eq_mul, ← h] at hm
  exact hm

/-- The reduction map `R_{t+1} → R_t`, `z_i ↦ z_i`. -/
def red (t : ℕ) : R p n (t + 1) →+* R p n t := Ideal.Quotient.factor (I_le p n t)

theorem red_mk (t : ℕ) (f : P p n) :
    red p n t (Ideal.Quotient.mk (I p n (t + 1)) f) = Ideal.Quotient.mk (I p n t) f :=
  Ideal.Quotient.factor_mk (I_le p n t) f

theorem mk_X_pow_eq_one (t : ℕ) (i : Fin n) :
    Ideal.Quotient.mk (I p n t) ((X i : P p n) ^ (2 ^ t)) = 1 := by
  rw [← map_one (Ideal.Quotient.mk (I p n t)), Ideal.Quotient.eq]
  exact gen_mem p n t i

theorem mk_X_pow_mod (t : ℕ) (i : Fin n) (e : ℕ) :
    Ideal.Quotient.mk (I p n t) ((X i : P p n) ^ e)
      = Ideal.Quotient.mk (I p n t) (X i ^ (e % 2 ^ t)) := by
  have hsplit : (X i : P p n) ^ e = X i ^ (e % 2 ^ t) * (X i ^ (2 ^ t)) ^ (e / 2 ^ t) := by
    rw [← pow_mul, ← pow_add, Nat.mod_add_div]
  rw [hsplit, map_mul, map_pow (Ideal.Quotient.mk (I p n t)) (X i ^ (2 ^ t)), mk_X_pow_eq_one,
    one_pow, mul_one]

/-- In `R_t`, `∏ z_i^(m_i) = ∏ z_i^(m_i mod 2^t)`. -/
theorem mk_prod_mod (t : ℕ) (m : Fin n → ℕ) :
    Ideal.Quotient.mk (I p n t) (∏ i, (X i : P p n) ^ m i)
      = Ideal.Quotient.mk (I p n t) (∏ i, (X i : P p n) ^ (m i % 2 ^ t)) := by
  rw [map_prod, map_prod]
  exact Finset.prod_congr rfl (fun i _ => mk_X_pow_mod p n t i (m i))

/-- The reduction map sends `∏ z_i^(m_i)` to `∏ z_i^(m_i mod 2^t)`. -/
theorem red_prod (t : ℕ) (m : Fin n → ℕ) :
    red p n t (Ideal.Quotient.mk (I p n (t + 1)) (∏ i, (X i : P p n) ^ m i))
      = Ideal.Quotient.mk (I p n t) (∏ i, (X i : P p n) ^ (m i % 2 ^ t)) := by
  rw [red_mk, mk_prod_mod]

/-- Monomial degree bound: `Σ pop (m_i mod 2^t) ≤ Σ pop m_i`. -/
theorem pop_prod_red_le (t : ℕ) (m : Fin n → ℕ) :
    ∑ i, GrowthLaw.pop (m i % 2 ^ t) ≤ ∑ i, GrowthLaw.pop (m i) :=
  Finset.sum_le_sum (fun i _ => GrowthLaw.pop_mod_le t (m i))

/-- The popcount degree of a monomial exponent vector. -/
def mdeg (m : Fin n →₀ ℕ) : ℕ := ∑ i, GrowthLaw.pop (m i)

def DegBound (t : ℕ) (a : R p n t) (c : ℕ) : Prop :=
  ∃ f : P p n, Ideal.Quotient.mk (I p n t) f = a ∧ ∀ m ∈ f.support, mdeg n m ≤ c

def deg (t : ℕ) (a : R p n t) : ℕ := sInf {c | DegBound p n t a c}

theorem degBound_nonempty (t : ℕ) (a : R p n t) : {c | DegBound p n t a c}.Nonempty := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective a
  exact ⟨f.support.sup (mdeg n), f, rfl, fun m hm => Finset.le_sup (f := mdeg n) hm⟩

theorem deg_spec (t : ℕ) (a : R p n t) : DegBound p n t a (deg p n t a) :=
  Nat.sInf_mem (degBound_nonempty p n t a)

theorem deg_le_of_rep (t : ℕ) (f : P p n) (c : ℕ) (hc : ∀ m ∈ f.support, mdeg n m ≤ c) :
    deg p n t (Ideal.Quotient.mk (I p n t) f) ≤ c := by
  unfold deg
  exact Nat.sInf_le ⟨f, rfl, hc⟩

theorem deg_red_le (t : ℕ) (a : R p n (t + 1)) : deg p n t (red p n t a) ≤ deg p n (t + 1) a := by
  obtain ⟨f, hf, hc⟩ := deg_spec p n (t + 1) a
  have h := deg_le_of_rep p n t f _ hc
  rw [← red_mk p n t f, hf] at h
  exact h

/-- Filtered data of `R_t`: variables `y_{i,j} = z_i^(2^j)` for `j < t`. -/
def filtered (t : ℕ) : GrowthLaw.FilteredData (ZMod p) (R p n t) (Fin n × Fin t) where
  zero := 0
  add := (· + ·)
  smul k a := Ideal.Quotient.mk (I p n t) (C k) * a
  mulv x a := Ideal.Quotient.mk (I p n t) ((X x.1 : P p n) ^ (2 ^ (x.2 : ℕ))) * a
  deg := deg p n t

def level (S : Set (P p n)) (t : ℕ) : GrowthLaw.Level (ZMod p) where
  R := R p n t
  X := Fin n × Fin t
  D := filtered p n t
  G := fun r => ∃ f ∈ S, Ideal.Quotient.mk (I p n t) f = r
  one := 1

def levelMap (S : Set (P p n)) (t : ℕ) :
    GrowthLaw.LevelMap (level p n S (t + 1)) (level p n S t) where
  φ := red p n t
  hone := map_one (red p n t)
  hzero := map_zero (red p n t)
  hadd := fun (a b : R p n (t + 1)) => map_add (red p n t) a b
  hsmul := fun k (a : R p n (t + 1)) => by
    show red p n t (Ideal.Quotient.mk (I p n (t + 1)) (C k) * a)
      = Ideal.Quotient.mk (I p n t) (C k) * red p n t a
    rw [map_mul, red_mk]
  hmul := fun (x : Fin n × Fin (t + 1)) => by
    by_cases hx : (x.2 : ℕ) < t
    · left
      refine ⟨((x.1, ⟨x.2, hx⟩) : Fin n × Fin t), fun (a : R p n (t + 1)) => ?_⟩
      show red p n t (Ideal.Quotient.mk (I p n (t + 1)) ((X x.1 : P p n) ^ (2 ^ (x.2 : ℕ))) * a)
        = Ideal.Quotient.mk (I p n t) ((X x.1 : P p n) ^ (2 ^ (x.2 : ℕ))) * red p n t a
      rw [map_mul, red_mk]
    · right
      intro (a : R p n (t + 1))
      have hxt : (x.2 : ℕ) = t := by have := x.2.isLt; omega
      show red p n t (Ideal.Quotient.mk (I p n (t + 1)) ((X x.1 : P p n) ^ (2 ^ (x.2 : ℕ))) * a)
        = red p n t a
      rw [map_mul, red_mk, hxt, mk_X_pow_eq_one, one_mul]
  hdeg := deg_red_le p n t
  hgen := fun r (hr : ∃ f ∈ S, Ideal.Quotient.mk (I p n (t + 1)) f = r) => by
    obtain ⟨f, hfS, hf⟩ := hr
    show ∃ g ∈ S, Ideal.Quotient.mk (I p n t) g = red p n t r
    exact ⟨f, hfS, by rw [← hf, red_mk]⟩

/-- Theorem 1 on the concrete Kummer tower: `t ≤ t'` implies `d_F(t) ≤ d_F(t')`. -/
theorem lastFall_mono (S : Set (P p n)) {t t' c c' : ℕ} (htt : t ≤ t')
    (h : GrowthLaw.IsLastFall (level p n S t) c) (h' : GrowthLaw.IsLastFall (level p n S t') c') :
    c ≤ c' :=
  GrowthLaw.tower_lastFall_mono (level p n S) (levelMap p n S) htt h h'

theorem refutes_descend (S : Set (P p n)) (c t d : ℕ) :
    GrowthLaw.Refutes (level p n S (t + d)) c → GrowthLaw.Refutes (level p n S t) c :=
  GrowthLaw.tower_refutes_descend (level p n S) (levelMap p n S) c t d

end Kummer

end Bridge

end

#print axioms Bridge.Uni.red_X_pow
#print axioms Bridge.Uni.pop_red_le
#print axioms Bridge.Uni.deg_red_le
#print axioms Bridge.Uni.deg_X_pow_le
#print axioms Bridge.Uni.deg_eq_nfDeg
#print axioms Bridge.Uni.deg_le
#print axioms Bridge.Uni.levelMap
#print axioms Bridge.Uni.lastFall_mono
#print axioms Bridge.Uni.refutes_descend
#print axioms Bridge.Kummer.red_prod
#print axioms Bridge.Kummer.pop_prod_red_le
#print axioms Bridge.Kummer.deg_red_le
#print axioms Bridge.Kummer.levelMap
#print axioms Bridge.Kummer.lastFall_mono
#print axioms Bridge.Kummer.refutes_descend
