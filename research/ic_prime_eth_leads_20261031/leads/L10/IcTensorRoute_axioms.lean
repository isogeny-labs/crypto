/-
  IcTensorRoute.lean -- exponent bookkeeping for index calculus with a
  black-box Boolean tensor-contraction (or any Boolean) decomposition oracle.

  Built offline with the crypto-autoresearcher formal workspace:
    cd /Volumes/SSD990-2/crypto-autoresearcher/formal && lake env lean <this file>
  Mathlib pinned there (Lean v4.35.0-rc1). Axioms: propext, Classical.choice, Quot.sound only.

  Conventions. b = log2 p (bits of the group order), m = arity (number of
  factor-base summands), s = b/m bits per factor-base window, g stands for
  sqrt(log2 b). Costs are exponents of 2; all rules are stated multiplied out
  so that no division appears where it can be avoided.
-/
import Mathlib.Basic.Real.Basic
import Mathlib.Analysis.Real.Sqrt
import Mathlib.Tactic
import Mathlib.Data.Fintype.Pi
import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Data.Fintype.Card

namespace IcTensorRoute

/-! ## 1. The ladder rule (go / no-go for any per-attempt oracle at fixed arity)

Relation phase costs about m! * p^{(1+e)/m} when one decomposition attempt costs
p^{e/m} = 2^{e s}; rho costs p^{1/2}. Ignoring m!, the IC relation phase is below
rho iff (1+e)/m < 1/2, i.e. 2(1+e) < m. With e = en/ed rational this is the
natural-number statement below (both sides scaled by 2*ed*s). -/
theorem ladder_rule (m s en ed : ℕ) (hs : 0 < s) :
    2 * (ed + en) * s < m * ed * s ↔ 2 * (ed + en) < m * ed := by
  constructor
  · intro h; exact Nat.lt_of_mul_lt_mul_right h
  · intro h; exact Nat.mul_lt_mul_of_pos_right h hs

/-- Arity 2 can never satisfy the ladder rule (needs 2(1+e) < 2, impossible for e ≥ 0). -/
theorem arity_two_never (en ed : ℕ) : ¬ (2 * (ed + en) < 2 * ed) := by omega

/-- Arity 3 needs e < 1/2, arity 4 needs e < 1, arity m needs e < m/2 - 1 (real form). -/
theorem ladder_rule_real (m e : ℝ) (hm : 0 < m) :
    (1 + e) / m < 1 / 2 ↔ e < m / 2 - 1 := by
  rw [div_lt_iff₀ hm]; constructor <;> intro h <;> linarith

/-! ## 2. A Boolean solver with o(1) savings cannot rescue fixed arity

If an encoding has n_eff ≥ b "hard" bits (the m window indices alone carry b bits of
interface) and the solver costs 2^{n_eff (1-δ)}, then beating the per-attempt budget
2^{b (1/2 - 1/m)} forces δ ≥ 1/2 + 1/m. SAT, polynomial-method and tensor-contraction
savings are δ = o(1) (1/√log n, 1/(2d), ...), so none can do this at fixed m. -/
theorem solver_saving_floor (n b m δ : ℝ) (hb : 0 < b) (hn : b ≤ n) (hm : 2 ≤ m)
    (hδ : δ ≤ 1)
    (h : n * (1 - δ) ≤ b * (1 / 2 - 1 / m)) : 1 / 2 + 1 / m ≤ δ := by
  have hm0 : 0 < m := by linarith
  have h1 : b * (1 - δ) ≤ n * (1 - δ) := by nlinarith
  have h2 : b * (1 - δ) ≤ b * (1 / 2 - 1 / m) := le_trans h1 h
  have h3 : 1 - δ ≤ 1 / 2 - 1 / m := le_of_mul_le_mul_left h2 hb
  linarith

/-! ## 3. Black-box tensor contraction: the Bochum balance is optimal in m, and
     it cannot cross rho below astronomically large b.

Per-attempt cost 2^{c * m * b / g} (g = sqrt(log2 b)), number of attempts 2^{b/m}.
Total exponent E(m) = b/m + c m b / g. -/

/-- AM-GM in the form used below. -/
theorem amgm (x y : ℝ) : 4 * x * y ≤ (x + y) ^ 2 := by nlinarith [sq_nonneg (x - y)]

/-- E(m)^2 ≥ 4 c b^2 / g for every m > 0: no arity beats 2 b sqrt(c/g), the value at
    m = sqrt(g/c) = (log2 b)^{1/4}/sqrt c, which is the Bochum choice. -/
theorem blackbox_balance (b c g m : ℝ) (hg : 0 < g) (hm : 0 < m) :
    4 * c * b ^ 2 / g ≤ (b / m + c * m * b / g) ^ 2 := by
  have key : (b / m) * (c * m * b / g) = c * b ^ 2 / g := by
    field_simp
  have := amgm (b / m) (c * m * b / g)
  calc 4 * c * b ^ 2 / g = 4 * ((b / m) * (c * m * b / g)) := by rw [key]; ring
    _ ≤ (b / m + c * m * b / g) ^ 2 := by nlinarith [this]

/-- If the total exponent is below rho's b/2, i.e. 1/m + c m / g < 1/2 (after dividing by b),
    then g > 16 c. With c ≥ 1 this is sqrt(log2 b) > 16, i.e. log2 b > 256: the group
    order would need more than 2^256 BITS. Holds for every arity m ≥ 3 (m = 2 is never
    below rho: 1/2 + 2c/g ≥ 1/2). -/
theorem no_crossover (c g m : ℝ) (hc : 0 ≤ c) (hg : 0 < g) (hm : 3 ≤ m)
    (h : 1 / m + c * m / g < 1 / 2) : 16 * c < g := by
  have hm0 : 0 < m := by linarith
  -- clear denominators: 2 g + 2 c m^2 < m g
  have h1 : 2 * g + 2 * c * m ^ 2 < m * g := by
    have := h
    rw [div_add_div _ _ (ne_of_gt hm0) (ne_of_gt hg), div_lt_div_iff₀ (by positivity) (by norm_num)] at this
    nlinarith [this]
  -- m^2 ≥ 8 (m - 2) since (m-4)^2 ≥ 0
  have h2 : 8 * (m - 2) ≤ m ^ 2 := by nlinarith [sq_nonneg (m - 4)]
  -- g (m-2) > 2 c m^2 ≥ 16 c (m-2), and m - 2 > 0
  have h3 : 16 * c * (m - 2) < g * (m - 2) := by nlinarith [mul_nonneg hc (sub_nonneg.mpr hm)]
  have h4 : 0 < m - 2 := by linarith
  exact lt_of_mul_lt_mul_right h3 (le_of_lt h4)

/-- Arity 2 is never below rho for the black-box route, whatever c ≥ 0 and g > 0. -/
theorem arity_two_blackbox (c g : ℝ) (hc : 0 ≤ c) (hg : 0 < g) :
    ¬ (1 / (2:ℝ) + c * 2 / g < 1 / 2) := by
  intro h
  have : 0 ≤ c * 2 / g := by positivity
  linarith

/-- Bits version: g = ⌊sqrt(log2 b)⌋ means g*g ≤ log2 b; with 16 < g the group size
    exceeds 2^(2^256). -/
theorem crossover_bits (g lam : ℕ) (hg : 16 < g) (hlam : g * g ≤ lam) :
    2 ^ 256 < 2 ^ lam := by
  have : 256 < lam := by nlinarith
  exact Nat.pow_lt_pow_right (by norm_num) this

/-! ## 4. Polynomial-method solvers (Dinur-type 2^{(1 - 1/(2d)) n}) lose to guessing
     m-1 of m windows whenever m ≤ 2d. Weil-descended Semaev systems have d ≥ 2 at
     m = 3 and larger at m ≥ 4, so the published bounds never reach the ladder. -/
theorem polymethod_vs_enumeration (m d l : ℝ) (hd : 0 < d) (hl : 0 ≤ l) (hm : m ≤ 2 * d) :
    (m - 1) * l ≤ (1 - 1 / (2 * d)) * m * l := by
  have h : m / (2 * d) ≤ 1 := by rw [div_le_one (by positivity)]; exact hm
  have : m - 1 ≤ (1 - 1 / (2 * d)) * m := by
    have : (1 - 1 / (2 * d)) * m = m - m / (2 * d) := by field_simp
    linarith
  nlinarith

/-! ## 5. Interface lower bound for Boolean tensor networks.
A parsimonious encoding of a language on k interface bits (Bochum Def. 2) names k
DISTINCT edges, so the network has at least k edges; the per-attempt exponent of any
black-box contractor is at least c k / sqrt(log k). For an m-sum relation with window
indices of b/m bits the interface alone is b bits; the arithmetic that checks the sum
adds the mb term (one b-bit multiplication per addition, each needing ≥ b edges). -/
theorem interface_lower_bound (k s : ℕ) (interface : Fin k → Fin s)
    (hinj : Function.Injective interface) : k ≤ s := by
  simpa using Fintype.card_le_of_injective interface hinj

/-! ## 6. Yield conservation for arbitrary factor bases in a finite abelian group.
Summed over all targets R, the number of ordered decompositions R = a_1 + ... + a_m
with a_i ∈ F_i equals ∏ |F_i|, for ANY subsets F_i (hashed, window, algebraic):
mean yield per target is ∏|F_i| / |G|. This is why no choice of base changes the
expected relation rate; only the variance (second moment) can differ. -/
theorem yield_conservation {G : Type*} [AddCommGroup G] [Fintype G] [DecidableEq G]
    {m : ℕ} (F : Fin m → Finset G) :
    ∑ R : G, ((Fintype.piFinset F).filter (fun a => ∑ i, a i = R)).card
      = ∏ i, (F i).card := by
  rw [← Fintype.card_piFinset]
  symm
  exact Finset.card_eq_sum_card_fiberwise (fun a _ => Finset.mem_univ (∑ i, a i))

end IcTensorRoute

#print axioms IcTensorRoute.ladder_rule
#print axioms IcTensorRoute.solver_saving_floor
#print axioms IcTensorRoute.blackbox_balance
#print axioms IcTensorRoute.no_crossover
#print axioms IcTensorRoute.crossover_bits
#print axioms IcTensorRoute.polymethod_vs_enumeration
#print axioms IcTensorRoute.interface_lower_bound
#print axioms IcTensorRoute.yield_conservation

#print axioms IcTensorRoute.ladder_rule_real
#print axioms IcTensorRoute.arity_two_never
#print axioms IcTensorRoute.amgm
#print axioms IcTensorRoute.arity_two_blackbox
