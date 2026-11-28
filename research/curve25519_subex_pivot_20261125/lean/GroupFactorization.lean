module

public import Mathlib

@[expose] public section

/-!
# Finite mode-to-group factorization identities

This module kernel-checks the algebraic steps used by the producer-side
factorization argument: row-wise expansion, finite Fubini across disjoint
group assignments, precision truncation of terms carrying a sufficiently
large power of two, and the Rust-shaped scalar Cauchy mode series with its
explicit geometric remainder. It does not connect the formal ring values,
modes, or group functions to concrete Rust objects.
-/

open scoped BigOperators

namespace Curve25519GroupFactorization

/-- Expanding a finite product of finite row sums enumerates each row-choice
function exactly once. -/
theorem row_choice_expansion
    {Row : Type*} [Fintype Row] [DecidableEq Row]
    {Choice : Row → Type*} [∀ row, Fintype (Choice row)]
    {R : Type*} [CommSemiring R]
    (factor : ∀ row, Choice row → R) :
    (∏ row, ∑ choice, factor row choice) =
      ∑ selection : ∀ row, Choice row, ∏ row, factor row (selection row) := by
  exact Fintype.prod_sum factor

/-- The packet row factor with `none` selecting the local affine term and
`some mode` selecting one crossing-mode contribution. -/
def packetRowFactor
    {Row Mode R : Type*} [Mul R]
    (localTerm : Row → R)
    (coefficient : Row → Mode → R)
    (sourceForm : Mode → R)
    (row : Row) : Option Mode → R
  | none => localTerm row
  | some mode => coefficient row mode * sourceForm mode

/-- The direct packet row product expands into the choice functions that are
later regrouped by mode exponent. -/
theorem packet_row_choice_expansion
    {Row Mode R : Type*}
    [Fintype Row] [DecidableEq Row] [Fintype Mode] [CommSemiring R]
    (localTerm : Row → R)
    (coefficient : Row → Mode → R)
    (sourceForm : Mode → R) :
    (∏ row, (localTerm row + ∑ mode, coefficient row mode * sourceForm mode)) =
      ∑ selection : Row → Option Mode,
        ∏ row, packetRowFactor localTerm coefficient sourceForm row (selection row) := by
  rw [← row_choice_expansion]
  congr with row
  simp [packetRowFactor]

/-- The exponent of one mode in a row-choice function. The `Fin` bound records
that no mode can be selected more often than there are packet rows. -/
def selectionExponent
    {Row Mode : Type*} [Fintype Row] [DecidableEq Row] [DecidableEq Mode]
    (selection : Row → Option Mode) (mode : Mode) :
    Fin (Fintype.card Row + 1) :=
  ⟨(Finset.univ.filter fun row ↦ selection row = some mode).card,
    lt_of_le_of_lt (Finset.card_le_univ _)
      (Nat.lt_succ_self (Fintype.card Row))⟩

/-- A global Boolean incidence assignment is equivalent to one independent
assignment on every group fiber. -/
def assignmentByGroupEquiv
    {Incidence Group : Type*}
    (group : Incidence → Group) :
    (Incidence → Bool) ≃
      (∀ target : Group, {incidence // group incidence = target} → Bool) where
  toFun assignment _ incidence := assignment incidence.1
  invFun assignments incidence :=
    assignments (group incidence) ⟨incidence, rfl⟩
  left_inv _ := rfl
  right_inv assignments := by
    funext target incidence
    obtain ⟨incidence, hgroup⟩ := incidence
    subst target
    rfl

/-- Once every factor depends only on one group fiber, summing over all global
incidence assignments factors as the product of the group contractions. -/
theorem partitioned_assignment_factorization
    {Incidence Group R : Type*}
    [Fintype Incidence] [DecidableEq Incidence]
    [Fintype Group] [DecidableEq Group] [CommSemiring R]
    (group : Incidence → Group)
    (groupTerm : ∀ target : Group,
      ({incidence // group incidence = target} → Bool) → R) :
    (∑ assignment : Incidence → Bool,
        ∏ target, groupTerm target (fun incidence ↦ assignment incidence.1)) =
      ∏ target, ∑ assignment, groupTerm target assignment := by
  rw [Fintype.prod_sum]
  exact Fintype.sum_equiv (assignmentByGroupEquiv group) _ _ (fun _ ↦ rfl)

/-- One endpoint incidence for each side of every parent edge. `false` selects
the row endpoint and `true` selects the column endpoint. -/
abbrev EndpointIncidence (Edge : Type*) := Edge × Bool

/-- Assign every parent endpoint incidence to its independently certified
row or column group. -/
def endpointGroup
    {Edge Group : Type*}
    (rowGroup columnGroup : Edge → Group) : EndpointIncidence Edge → Group
  | (edge, false) => rowGroup edge
  | (edge, true) => columnGroup edge

/-- Exact endpoint mass across the complete group-child family: every parent
edge occurs once through its row endpoint and once through its column endpoint,
independently of how those endpoints are partitioned into groups. -/
theorem complete_group_child_incidence_count
    {Edge Group : Type*} [Fintype Edge] [Fintype Group] [DecidableEq Group]
    (rowGroup columnGroup : Edge → Group) :
    (∑ target, Fintype.card
      {incidence : EndpointIncidence Edge //
        endpointGroup rowGroup columnGroup incidence = target}) =
      2 * Fintype.card Edge := by
  have hfiber := Fintype.sum_fiberwise
    (endpointGroup rowGroup columnGroup)
    (fun _ : EndpointIncidence Edge ↦ 1)
  simpa [Fintype.card_prod, Fintype.card_bool, Nat.mul_comm] using hfiber

/-- Restrict one group-child assignment to the incidences owned by a single
packet. This is the typed counterpart of the packet-table mask assembled by
the standalone verifier. -/
def restrictPacketAssignment
    {Incidence Group Packet : Type*}
    (group : Incidence → Group) (packet : Incidence → Packet)
    (target : Group) (selectedPacket : Packet)
    (assignment : {incidence // group incidence = target} → Bool) :
    {incidence // group incidence = target ∧ packet incidence = selectedPacket} → Bool :=
  fun incidence ↦ assignment ⟨incidence.1, incidence.2.1⟩

/-- Semantic value of one complete certified child: the retained old tensors
see the full group fiber, while each packet tensor sees exactly its ordered
packet/group subfiber. -/
def groupChildTerm
    {Incidence Group Packet R : Type*}
    [Fintype Packet] [CommSemiring R]
    (group : Incidence → Group) (packet : Incidence → Packet)
    (oldTerm : ∀ target : Group,
      ({incidence // group incidence = target} → Bool) → R)
    (packetEntry : ∀ (target : Group) (selectedPacket : Packet),
      ({incidence // group incidence = target ∧
        packet incidence = selectedPacket} → Bool) → R)
    (target : Group)
    (assignment : {incidence // group incidence = target} → Bool) : R :=
  oldTerm target assignment *
    ∏ selectedPacket,
      packetEntry target selectedPacket
        (restrictPacketAssignment group packet target selectedPacket assignment)

/-- Instantiation of finite Fubini for the exact child interface checked by
the native certificates: a complete group fiber of old tensors and one total
Boolean entry table for every packet/group subfiber. -/
theorem complete_child_family_factorization
    {Incidence Group Packet R : Type*}
    [Fintype Incidence] [DecidableEq Incidence]
    [Fintype Group] [DecidableEq Group]
    [Fintype Packet] [CommSemiring R]
    (group : Incidence → Group) (packet : Incidence → Packet)
    (oldTerm : ∀ target : Group,
      ({incidence // group incidence = target} → Bool) → R)
    (packetEntry : ∀ (target : Group) (selectedPacket : Packet),
      ({incidence // group incidence = target ∧
        packet incidence = selectedPacket} → Bool) → R) :
    (∑ assignment : Incidence → Bool,
        ∏ target,
          groupChildTerm group packet oldTerm packetEntry target
            (fun incidence ↦ assignment incidence.1)) =
      ∏ target,
        ∑ assignment,
          groupChildTerm group packet oldTerm packetEntry target assignment := by
  exact partitioned_assignment_factorization group
    (groupChildTerm group packet oldTerm packetEntry)

/-- Any finite sum can be regrouped by the fibers of a finite key map. Applied
to row choices, the key is the resulting mode-exponent vector. -/
theorem sum_regroup_by_fibers
    {Source Target R : Type*}
    [Fintype Source] [Fintype Target] [DecidableEq Target]
    [AddCommMonoid R]
    (key : Source → Target) (term : Source → R) :
    (∑ source, term source) =
      ∑ target, ∑ source, if key source = target then term source else 0 := by
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro source _
  simp

/-- The packet row expansion can be regrouped by the complete vector of mode
multiplicities. This is the finite partition used by the native packet-entry
verifier when it extracts one requested exponent vector. -/
theorem packet_row_choice_expansion_by_exponent
    {Row Mode R : Type*}
    [Fintype Row] [DecidableEq Row] [Fintype Mode] [DecidableEq Mode]
    [CommSemiring R]
    (localTerm : Row → R)
    (coefficient : Row → Mode → R)
    (sourceForm : Mode → R) :
    (∏ row, (localTerm row + ∑ mode, coefficient row mode * sourceForm mode)) =
      ∑ beta : Mode → Fin (Fintype.card Row + 1),
        ∑ selection : Row → Option Mode,
          if selectionExponent selection = beta then
            ∏ row, packetRowFactor localTerm coefficient sourceForm row (selection row)
          else 0 := by
  rw [packet_row_choice_expansion]
  exact sum_regroup_by_fibers selectionExponent
    (fun selection ↦
      ∏ row, packetRowFactor localTerm coefficient sourceForm row (selection row))

/-- Multiplying terms with known powers of two adds their weights exactly. -/
theorem product_pow_two_weight
    {Term : Type*} [Fintype Term]
    (weight coefficient : Term → ℕ) :
    (∏ term, 2 ^ weight term * coefficient term) =
      2 ^ (∑ term, weight term) * ∏ term, coefficient term := by
  rw [Finset.prod_mul_distrib, Finset.prod_pow_eq_pow_sum]

/-- A term carrying at least `2^p` is zero in the coefficient ring modulo
`2^p`. -/
theorem pow_two_multiple_vanishes
    (p weight coefficient : ℕ)
    (hweight : p ≤ weight) :
    ((2 ^ weight * coefficient : ℕ) : ZMod (2 ^ p)) = 0 := by
  obtain ⟨extra, rfl⟩ := Nat.exists_eq_add_of_le hweight
  rw [pow_add]
  simp [mul_assoc]

/-- A finite collection of terms all above the precision cutoff contributes
zero modulo `2^p`. -/
theorem high_weight_sum_vanishes
    {Term : Type*} [Fintype Term]
    (p : ℕ) (weight coefficient : Term → ℕ)
    (hweight : ∀ term, p ≤ weight term) :
    (∑ term, ((2 ^ weight term * coefficient term : ℕ) : ZMod (2 ^ p))) = 0 := by
  apply Finset.sum_eq_zero
  intro term _
  exact pow_two_multiple_vanishes p (weight term) (coefficient term) (hweight term)

/-- A powers-of-two shifted element becomes nilpotent at the coefficient
precision once its accumulated valuation reaches the modulus exponent. -/
theorem shifted_power_nilpotent
    (precision shift exponent coefficient : ℕ)
    (hvaluation : precision ≤ shift * exponent) :
    (((2 ^ shift * coefficient : ℕ) : ZMod (2 ^ precision)) ^ exponent) = 0 := by
  simpa [mul_pow, pow_mul] using
    pow_two_multiple_vanishes precision (shift * exponent)
      (coefficient ^ exponent) hvaluation

/-- The same valuation argument works in every coefficient ring in which
`2^precision` is zero, including finite extensions of `ZMod (2^precision)`. -/
theorem shifted_power_nilpotent_of_pow_eq_zero
    {R : Type*} [CommRing R]
    (precision shift exponent : ℕ) (coefficient : R)
    (hmodulus : (2 : R) ^ precision = 0)
    (hvaluation : precision ≤ shift * exponent) :
    (((2 : R) ^ shift * coefficient) ^ exponent) = 0 := by
  obtain ⟨extra, hexponent⟩ := Nat.exists_eq_add_of_le hvaluation
  rw [mul_pow, ← pow_mul, hexponent, pow_add, hmodulus]
  simp

/-- An outer power-of-two factor can kill a truncated geometric remainder
even when the ratio itself is not yet nilpotent. This is the exact cutoff
used by the Rust series, whose outer shift contributes to total valuation. -/
theorem shifted_power_with_base_vanishes_of_pow_eq_zero
    {R : Type*} [CommRing R]
    (precision baseShift modeShift exponent : ℕ) (coefficient : R)
    (hmodulus : (2 : R) ^ precision = 0)
    (hvaluation : precision ≤ baseShift + modeShift * exponent) :
    (2 : R) ^ baseShift *
        (((2 : R) ^ modeShift * coefficient) ^ exponent) = 0 := by
  obtain ⟨extra, hexponent⟩ := Nat.exists_eq_add_of_le hvaluation
  rw [mul_pow, ← pow_mul, ← mul_assoc, ← pow_add, hexponent, pow_add, hmodulus]
  simp

/-- Dividing a tile displacement by its known `2^v` content leaves the
normalized shift `2^(i+1-v) * sample`. This is the exact integer/ring algebra
behind `y = y0 + 2^(i+1)j` in the Rust mode expansion. -/
theorem cauchy_tile_denominator_identity
    {R : Type*} [CommRing R]
    (base sample : R) (v i : ℕ) (hvi : v ≤ i) :
    (2 : R) ^ v * (base - (2 : R) ^ (i + 1 - v) * sample) =
      (2 : R) ^ v * base - (2 : R) ^ (i + 1) * sample := by
  have hv : v ≤ i + 1 := le_trans hvi (Nat.le_succ i)
  rw [mul_sub, ← mul_assoc, ← pow_add, Nat.add_sub_of_le hv]

/-- Multiplying a truncated normalized geometric series by its shifted
denominator leaves exactly the next-power remainder. -/
theorem geometric_inverse_partial_product
    {R : Type*} [CommRing R]
    (denominator denominatorInv step : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1) :
    (denominator - step) *
        (denominatorInv * ∑ index ∈ Finset.range terms,
          (step * denominatorInv) ^ index) =
      1 - (step * denominatorInv) ^ terms := by
  let ratio := step * denominatorInv
  calc
    (denominator - step) *
          (denominatorInv * ∑ index ∈ Finset.range terms, ratio ^ index) =
        (1 - ratio) * ∑ index ∈ Finset.range terms, ratio ^ index := by
          dsimp [ratio]
          rw [sub_mul, ← mul_assoc denominator denominatorInv, hinverse, one_mul,
            ← mul_assoc step denominatorInv]
          ring
    _ = 1 - ratio ^ terms := mul_neg_geom_sum ratio terms

/-- The finite geometric series is an inverse of `denominator-step` whenever
the normalized step is nilpotent at the truncation length. -/
theorem nilpotent_geometric_inverse
    {R : Type*} [CommRing R]
    (denominator denominatorInv step : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hnilpotent : (step * denominatorInv) ^ terms = 0) :
    (denominator - step) *
        (denominatorInv * ∑ index ∈ Finset.range terms,
          (step * denominatorInv) ^ index) = 1 := by
  rw [geometric_inverse_partial_product denominator denominatorInv step terms
    hinverse, hnilpotent]
  simp

/-- An actual inverse of the shifted denominator equals the truncated
geometric expression; inverses are unique in a commutative ring. -/
theorem inverse_eq_nilpotent_geometric
    {R : Type*} [CommRing R]
    (denominator denominatorInv step shiftedInv : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse : (denominator - step) * shiftedInv = 1)
    (hnilpotent : (step * denominatorInv) ^ terms = 0) :
    shiftedInv = denominatorInv *
      ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index := by
  let candidate := denominatorInv *
    ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index
  have hcandidate : (denominator - step) * candidate = 1 := by
    exact nilpotent_geometric_inverse denominator denominatorInv step terms
      hinverse hnilpotent
  calc
    shiftedInv = shiftedInv * 1 := by simp
    _ = shiftedInv * ((denominator - step) * candidate) := by rw [hcandidate]
    _ = candidate * ((denominator - step) * shiftedInv) := by ring
    _ = candidate := by rw [hshiftedInverse]; simp

/-- Without a nilpotence premise, the exact inverse is the finite geometric
sum plus its explicit next-power remainder. -/
theorem inverse_eq_geometric_add_remainder
    {R : Type*} [CommRing R]
    (denominator denominatorInv step shiftedInv : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse : (denominator - step) * shiftedInv = 1) :
    shiftedInv = denominatorInv *
        ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index +
      shiftedInv * (step * denominatorInv) ^ terms := by
  let candidate := denominatorInv *
    ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index
  have hcandidate :
      (denominator - step) * candidate =
        1 - (step * denominatorInv) ^ terms := by
    exact geometric_inverse_partial_product denominator denominatorInv step terms
      hinverse
  have hother : shiftedInv * (denominator - step) = 1 := by
    rw [mul_comm]
    exact hshiftedInverse
  calc
    shiftedInv = shiftedInv * 1 := by simp
    _ = shiftedInv *
        ((denominator - step) * candidate +
          (step * denominatorInv) ^ terms) := by rw [hcandidate]; ring
    _ = candidate + shiftedInv * (step * denominatorInv) ^ terms := by
      rw [mul_add, ← mul_assoc, hother, one_mul]

/-- The normalized Cauchy inverse series, rewritten in the exact coefficient
shape emitted by `mode_coefficient`: coefficient `b` contains
`denominatorInv^(b+1)` and the shift `baseShift + modeShift*b`, then is
evaluated at `sample^b`. -/
theorem cauchy_weight_mode_series
    {R : Type*} [CommRing R]
    (numerator denominator denominatorInv shiftedInv sample : R)
    (baseShift modeShift terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse :
      (denominator - (2 : R) ^ modeShift * sample) * shiftedInv = 1)
    (htruncation :
      (2 : R) ^ baseShift *
        (((2 : R) ^ modeShift * sample) * denominatorInv) ^ terms = 0) :
    numerator * shiftedInv * (2 : R) ^ baseShift =
      ∑ index ∈ Finset.range terms,
        (numerator * denominatorInv ^ (index + 1) *
          (2 : R) ^ (baseShift + modeShift * index)) * sample ^ index := by
  rw [inverse_eq_geometric_add_remainder denominator denominatorInv
    ((2 : R) ^ modeShift * sample) shiftedInv terms hinverse
    hshiftedInverse]
  have hremainder :
      numerator *
          (shiftedInv *
            (((2 : R) ^ modeShift * sample) * denominatorInv) ^ terms) *
          (2 : R) ^ baseShift = 0 := by
    calc
      numerator *
            (shiftedInv *
              (((2 : R) ^ modeShift * sample) * denominatorInv) ^ terms) *
            (2 : R) ^ baseShift =
          numerator * shiftedInv *
            ((2 : R) ^ baseShift *
              (((2 : R) ^ modeShift * sample) * denominatorInv) ^ terms) := by
                ring
      _ = 0 := by rw [htruncation]; simp
  rw [mul_add, add_mul, hremainder, add_zero]
  simp_rw [Finset.mul_sum, Finset.sum_mul]
  apply Finset.sum_congr rfl
  intro index hindex
  rw [mul_pow, mul_pow, ← pow_mul, pow_succ', pow_add]
  ring

/-- Exact Rust cutoff specialization. Under `v <= i <= a`, both shifts used by
`mode_coefficient` are positive. Therefore the omitted `b = precision-1`
remainder has total valuation at least `precision`, including the outer
`a+1-v` shift, and the loop over `0 .. precision-2` is exact. -/
theorem cauchy_weight_mode_series_at_precision
    {R : Type*} [CommRing R]
    (numerator denominator denominatorInv shiftedInv sample : R)
    (a i v precision : ℕ)
    (hvi : v ≤ i) (hia : i ≤ a) (hprecision : 0 < precision)
    (hmodulus : (2 : R) ^ precision = 0)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse :
      (denominator - (2 : R) ^ (i + 1 - v) * sample) * shiftedInv = 1) :
    numerator * shiftedInv * (2 : R) ^ (a + 1 - v) =
      ∑ index ∈ Finset.range (precision - 1),
        (numerator * denominatorInv ^ (index + 1) *
          (2 : R) ^ ((a + 1 - v) + (i + 1 - v) * index)) *
            sample ^ index := by
  have hbase : 0 < a + 1 - v := by omega
  have hmode : 0 < i + 1 - v := by omega
  have hmul : precision - 1 ≤ (i + 1 - v) * (precision - 1) := by
    simpa using Nat.mul_le_mul_right (precision - 1) hmode
  have hvaluation :
      precision ≤ (a + 1 - v) + (i + 1 - v) * (precision - 1) := by
    omega
  have htruncation :
      (2 : R) ^ (a + 1 - v) *
        (((2 : R) ^ (i + 1 - v) * sample) * denominatorInv) ^
          (precision - 1) = 0 := by
    simpa [mul_assoc] using
      shifted_power_with_base_vanishes_of_pow_eq_zero precision
        (a + 1 - v) (i + 1 - v) (precision - 1)
        (sample * denominatorInv) hmodulus hvaluation
  exact cauchy_weight_mode_series numerator denominator denominatorInv shiftedInv
    sample (a + 1 - v) (i + 1 - v) (precision - 1) hinverse
    hshiftedInverse htruncation

end Curve25519GroupFactorization
