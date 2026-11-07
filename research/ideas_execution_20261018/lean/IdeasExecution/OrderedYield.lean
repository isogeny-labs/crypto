import FactorBaseDesigns.Yield

/-!
Instantiate the existing finite-fibre theorem with the ordered Cartesian
power of a factor-base list. This states an exact total multiplicity identity,
not a coverage, independence, solving-cost, or fixed-target yield theorem.
-/
namespace IdeasExecution
open FactorBaseDesigns
variable {α G : Type} [DecidableEq G]

def tuples (B : List α) : Nat → List (List α)
  | 0 => [[]]
  | m + 1 => B.flatMap (fun b => (tuples B m).map (fun t => b :: t))

omit [DecidableEq G] in
theorem length_slot (B : List α) (T : List (List α)) :
    (B.flatMap (fun b => T.map (fun t => b :: t))).length = B.length * T.length := by
  induction B with
  | nil => simp
  | cons b B ih =>
      simp only [List.flatMap_cons, List.length_append, List.length_map, List.length_cons]
      rw [ih, Nat.add_mul]
      omega

omit [DecidableEq G] in
theorem tuples_length (B : List α) (m : Nat) : (tuples B m).length = B.length ^ m := by
  induction m with
  | zero => simp [tuples]
  | succ m ih =>
      rw [tuples, length_slot, ih, Nat.pow_succ]
      exact Nat.mul_comm _ _

/-- All-target ordered m-tuple multiplicity equals the mth power of the base
cardinality, for any decomposition pattern whose image is covered. -/
theorem ordered_total_yield (B : List α) (m : Nat) (φ : List α → G)
    (Gs : List G) (hnd : Gs.Nodup)
    (hcov : ∀ t ∈ tuples B m, φ t ∈ Gs) :
    (Gs.map (count (tuples B m) φ)).sum = B.length ^ m := by
  rw [sum_count (tuples B m) φ hnd hcov, tuples_length]

/-- Different presentations with equally large base lists have equal total
ordered-tuple multiplicity. Nothing here bounds an oracle's runtime. -/
theorem ordered_patterns_neutral (B C : List α) (m : Nat)
    (φ ψ : List α → G) (Gs : List G) (hnd : Gs.Nodup)
    (hφ : ∀ t ∈ tuples B m, φ t ∈ Gs)
    (hψ : ∀ t ∈ tuples C m, ψ t ∈ Gs) (hsize : B.length = C.length) :
    (Gs.map (count (tuples B m) φ)).sum = (Gs.map (count (tuples C m) ψ)).sum := by
  rw [ordered_total_yield B m φ Gs hnd hφ,
      ordered_total_yield C m ψ Gs hnd hψ, hsize]
end IdeasExecution
