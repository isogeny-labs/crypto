import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic

/-!
# H113: binary/trinary section-curl structural obligations

The declarations below are generic algebraic and reconstruction statements.
They do not assert that a measured winding predicate is exact.
-/

namespace H113BinaryTrinaryWinding

section CarryDiamond

variable {Q A : Type*}
variable [AddCommGroup A]

/-- The section carry for multiplication by `n`. The section is deliberately
only a set-theoretic function. -/
def carry (n : ℕ) (liftSection : Q → A) (root target : Q) : A :=
  n • liftSection root - liftSection target

/-- The `[2] ∘ [3]` carry path telescopes to the direct `[6]` defect. -/
theorem carry_two_three_path
    (liftSection : Q → A) (h6 h2 q : Q) :
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
      carry 6 liftSection h6 q := by
  simp only [carry, nsmul_sub]
  abel

/-- The `[3] ∘ [2]` carry path telescopes to the direct `[6]` defect. -/
theorem carry_three_two_path
    (liftSection : Q → A) (h6 h3 q : Q) :
    3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q =
      carry 6 liftSection h6 q := by
  simp only [carry, nsmul_sub]
  abel

/-- The two binary/trinary section-carry paths commute in every additive
commutative group. -/
theorem commuting_carry_paths
    (liftSection : Q → A) (h6 h2 h3 q : Q) :
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
      3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q := by
  calc
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
        carry 6 liftSection h6 q := carry_two_three_path liftSection h6 h2 q
    _ = 3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q :=
      (carry_three_two_path liftSection h6 h3 q).symm

end CarryDiamond

section Representatives

/-- Four canonical representatives of one formal-layer equality yield an
integer winding in exactly the six-value interval frozen by H113. -/
theorem representative_winding_range
    (p a b c d numerator : ℤ)
    (hp : 0 < p)
    (ha0 : 0 ≤ a) (hap : a < p)
    (hb0 : 0 ≤ b) (hbp : b < p)
    (hc0 : 0 ≤ c) (hcp : c < p)
    (hd0 : 0 ≤ d) (hdp : d < p)
    (hnumerator : numerator = 2 * a + b - 3 * c - d)
    (hdiv : p ∣ numerator) :
    ∃ winding : ℤ,
      numerator = p * winding ∧ -3 ≤ winding ∧ winding ≤ 2 := by
  rcases hdiv with ⟨winding, hwinding⟩
  have hlower : -4 * p < numerator := by omega
  have hupper : numerator < 3 * p := by omega
  have hwlower : -4 < winding := by nlinarith [hwinding]
  have hwupper : winding < 3 := by nlinarith [hwinding]
  exact ⟨winding, hwinding, by omega, by omega⟩

end Representatives

section Oracle

variable {Point Value : Type*}

/-- Exactness of a public predicate supplies the canonical base-six digit.
Exactness is a hypothesis, not a conclusion of the formalization. -/
theorem exact_predicate_supplies_baseSix
    (observable : Point → Value) (predicate : Value → ℕ)
    (scalar : Point → ℕ)
    (hexact : ∀ q, predicate (observable q) = scalar q % 6) (q : Point) :
    predicate (observable q) = scalar q % 6 := by
  exact hexact q

/-- The native exact-ID score: the number of finite-cohort points on which a
candidate and its frozen label agree. -/
def agreementScore {Cohort Label : Type*}
    [Fintype Cohort] [DecidableEq Cohort] [DecidableEq Label]
    (candidate label : Cohort → Label) : ℕ :=
  (Finset.univ.filter fun q => candidate q = label q).card

/-- A full finite-cohort score is a pointwise exactness certificate on that
cohort. -/
theorem full_agreement_score_implies_exact
    {Cohort Label : Type*}
    [Fintype Cohort] [DecidableEq Cohort] [DecidableEq Label]
    (candidate label : Cohort → Label)
    (hfull : agreementScore candidate label = Fintype.card Cohort) :
    ∀ q, candidate q = label q := by
  let good : Finset Cohort :=
    Finset.univ.filter fun q => candidate q = label q
  have hcard : good.card = (Finset.univ : Finset Cohort).card := by
    simpa [good, agreementScore] using hfull
  have hgood : good = (Finset.univ : Finset Cohort) :=
    Finset.eq_of_subset_of_card_le (Finset.filter_subset _ _)
      (Nat.le_of_eq hcard.symm)
  intro q
  have hmem : q ∈ good := by rw [hgood]; simp
  exact (Finset.mem_filter.mp hmem).2

/-- A source-independently verified full score supplies the base-six oracle
premise on the frozen finite cohort. -/
theorem full_score_supplies_baseSix
    {Cohort : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalar : Cohort → ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalar q % 6) = Fintype.card Cohort) :
    ∀ q, predicate (observable q) = scalar q % 6 := by
  exact full_agreement_score_implies_exact _ _ hfull

/-- Reconstruct a natural number from successive exact remainder queries in
an arbitrary radix at least two. -/
def radixReconstruct (base : ℕ) (hbase : 2 ≤ base)
    (oracle : ℕ → ℕ) : ℕ → ℕ
  | 0 => 0
  | k + 1 =>
      oracle (k + 1) +
        base * radixReconstruct base hbase oracle ((k + 1) / base)
termination_by k => k
decreasing_by
  exact Nat.div_lt_self (by omega) (by omega)

/-- An exact remainder oracle makes repeated radix division recover every
natural number. -/
theorem radixReconstruct_eq
    (base : ℕ) (hbase : 2 ≤ base) (oracle : ℕ → ℕ)
    (hexact : ∀ k, oracle k = k % base) :
    ∀ k, radixReconstruct base hbase oracle k = k := by
  intro k
  induction k using Nat.strong_induction_on with
  | h k ih =>
      cases k with
      | zero => simp [radixReconstruct]
      | succ n =>
          rw [radixReconstruct, hexact,
            ih ((n + 1) / base)
              (Nat.div_lt_self (by omega) (by omega))]
          exact Nat.mod_add_div (n + 1) base

/-- H113's integer decoder is the radix-six instance of the generic theorem. -/
theorem baseSixReconstruct_eq
    (oracle : ℕ → ℕ) (hexact : ∀ k, oracle k = k % 6) :
    ∀ k, radixReconstruct 6 (by omega) oracle k = k := by
  exact radixReconstruct_eq 6 (by omega) oracle hexact

/-- The canonical scalar remaining after `rounds` exact radix divisions. -/
def radixRoundState (base scalar rounds : ℕ) : ℕ :=
  scalar / base ^ rounds

/-- One more round divides the canonical remaining scalar by the radix. -/
theorem radixRoundState_succ (base scalar rounds : ℕ) :
    radixRoundState base scalar (rounds + 1) =
      radixRoundState base scalar rounds / base := by
  simp [radixRoundState, Nat.pow_succ, Nat.div_div_eq_div_mul]

/-- Exact radix division reaches zero after at most `log_base(scalar) + 1`
rounds for every radix at least two. -/
theorem radixRoundState_log_succ_zero
    (base scalar : ℕ) (hbase : 2 ≤ base) :
    radixRoundState base scalar (Nat.log base scalar + 1) = 0 := by
  apply Nat.div_eq_of_lt
  simpa [radixRoundState] using
    (Nat.lt_pow_succ_log_self (by omega : 1 < base) scalar)

/-- The frozen decoder's exact digit-query bound. -/
theorem baseSix_round_bound (scalar : ℕ) :
    radixRoundState 6 scalar (Nat.log 6 scalar + 1) = 0 := by
  exact radixRoundState_log_succ_zero 6 scalar (by omega)

/-- A complete bounded radix run. The first component is the recovered low
digits and the second is the scalar quotient remaining after `rounds`. -/
def radixRun (base : ℕ) : ℕ → ℕ → ℕ × ℕ
  | scalar, 0 => (0, scalar)
  | scalar, rounds + 1 =>
      let tail := radixRun base (scalar / base) rounds
      (scalar % base + base * tail.1, tail.2)

/-- Every bounded radix run partitions the original scalar into its recovered
prefix and its radix-weighted remaining quotient. -/
theorem radixRun_invariant (base scalar rounds : ℕ) :
    (radixRun base scalar rounds).1 +
      base ^ rounds * (radixRun base scalar rounds).2 = scalar := by
  induction rounds generalizing scalar with
  | zero => simp [radixRun]
  | succ rounds ih =>
      let tail := radixRun base (scalar / base) rounds
      have htail : tail.1 + base ^ rounds * tail.2 = scalar / base := by
        simpa [tail] using ih (scalar / base)
      calc
        (radixRun base scalar (rounds + 1)).1 +
            base ^ (rounds + 1) *
              (radixRun base scalar (rounds + 1)).2 =
            scalar % base + base * tail.1 +
              (base ^ rounds * base) * tail.2 := by
          simp [radixRun, tail, Nat.pow_succ]
        _ = scalar % base +
              base * (tail.1 + base ^ rounds * tail.2) := by
          ring
        _ = scalar % base + base * (scalar / base) := by rw [htail]
        _ = scalar := Nat.mod_add_div scalar base

/-- The second run component is exactly division by the corresponding radix
power. -/
theorem radixRun_remaining_eq (base scalar rounds : ℕ) :
    (radixRun base scalar rounds).2 = scalar / base ^ rounds := by
  induction rounds generalizing scalar with
  | zero => simp [radixRun]
  | succ rounds ih =>
      simp only [radixRun]
      rw [ih]
      rw [Nat.pow_succ, Nat.div_div_eq_div_mul]
      rw [Nat.mul_comm base (base ^ rounds)]

/-- A complete exact radix run returns the scalar and zero remaining quotient
within the logarithmic round bound. -/
theorem radixRun_log_succ_eq
    (base scalar : ℕ) (hbase : 2 ≤ base) :
    radixRun base scalar (Nat.log base scalar + 1) = (scalar, 0) := by
  have hremaining :
      (radixRun base scalar (Nat.log base scalar + 1)).2 = 0 := by
    rw [radixRun_remaining_eq]
    exact radixRoundState_log_succ_zero base scalar hbase
  apply Prod.ext
  · have hinvariant :=
      radixRun_invariant base scalar (Nat.log base scalar + 1)
    simpa [hremaining] using hinvariant
  · exact hremaining

/-- End-to-end arithmetic reconstruction for H113's frozen radix six. -/
theorem baseSix_run_recovers (scalar : ℕ) :
    radixRun 6 scalar (Nat.log 6 scalar + 1) = (scalar, 0) := by
  exact radixRun_log_succ_eq 6 scalar (by omega)

end Oracle

section GroupDecoder

variable {A : Type*}
variable [AddCommGroup A]

/-- One conditional H113 decoder update. The inverse-six action is exposed as
an explicit subgroup obligation. -/
theorem baseSix_group_step
    (generator current : A) (scalar digit inverseSix : ℕ)
    (hcurrent : current = scalar • generator)
    (hdigit : digit = scalar % 6)
    (hinverse : ∀ point : A, inverseSix • (6 • point) = point) :
    inverseSix • (current - digit • generator) =
      (scalar / 6) • generator := by
  rw [hcurrent, hdigit]
  have hdecompose : scalar % 6 + 6 * (scalar / 6) = scalar := by
    exact Nat.mod_add_div scalar 6
  calc
    inverseSix • (scalar • generator - (scalar % 6) • generator) =
        inverseSix • ((6 * (scalar / 6)) • generator) := by
      have hsubtract :
          scalar • generator - (scalar % 6) • generator =
            (6 * (scalar / 6)) • generator := by
        have hscalar :
            scalar • generator =
              (scalar % 6) • generator +
                (6 * (scalar / 6)) • generator := by
          rw [← add_nsmul]
          exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
        rw [hscalar]
        abel
      rw [hsubtract]
    _ = inverseSix • (6 • ((scalar / 6) • generator)) := by
      rw [Nat.mul_comm 6 (scalar / 6), mul_nsmul]
    _ = (scalar / 6) • generator := hinverse _

/-- A concrete inverse-six certificate on the cyclic subgroup generated by
`generator`.  Unlike `baseSix_group_step`'s generic hypothesis, this requires
no inverse action on unrelated points of the ambient group. -/
theorem inverse_six_on_cyclic
    (generator : A) (order inverseSix quotient scalar : ℕ)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    inverseSix • (6 • (scalar • generator)) = scalar • generator := by
  calc
    inverseSix • (6 • (scalar • generator)) =
        ((inverseSix * 6) * scalar) • generator := by
      have haction :
          ((scalar * 6) * inverseSix) • generator =
            inverseSix • (6 • (scalar • generator)) := by
        calc
          ((scalar * 6) * inverseSix) • generator =
              inverseSix • ((scalar * 6) • generator) :=
            mul_nsmul generator (scalar * 6) inverseSix
          _ = inverseSix • (6 • (scalar • generator)) := by
            rw [mul_nsmul generator scalar 6]
      have hreorder :
          (scalar * 6) * inverseSix =
            (inverseSix * 6) * scalar := by
        ac_rfl
      rw [← haction, hreorder]
    _ = ((1 + quotient * order) * scalar) • generator := by
      rw [hcertificate]
    _ = scalar • generator := by
      calc
        ((1 + quotient * order) * scalar) • generator =
            scalar • generator +
              ((quotient * order) * scalar) • generator := by
          rw [add_mul, add_nsmul, one_mul]
        _ = scalar • generator := by
          have hreorder :
              (quotient * order) * scalar =
                (quotient * scalar) * order := by
            ac_rfl
          have hmultiple_zero :
              ((quotient * scalar) * order) • generator = 0 := by
            have hreorder_multiple :
                (quotient * scalar) * order =
                  order * (quotient * scalar) := by
              ac_rfl
            calc
              ((quotient * scalar) * order) • generator =
                  (order * (quotient * scalar)) • generator := by
                rw [hreorder_multiple]
              _ = (quotient * scalar) • (order • generator) :=
                mul_nsmul generator order (quotient * scalar)
              _ = 0 := by rw [horder, nsmul_zero]
          rw [hreorder, hmultiple_zero, add_zero]

/-- One base-six decoder update from the concrete cyclic-subgroup order and
inverse certificate used by the frozen prime-order cohort. -/
theorem baseSix_cyclic_group_step
    (generator current : A)
    (scalar digit order inverseSix quotient : ℕ)
    (hcurrent : current = scalar • generator)
    (hdigit : digit = scalar % 6)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    inverseSix • (current - digit • generator) =
      (scalar / 6) • generator := by
  rw [hcurrent, hdigit]
  have hdecompose : scalar % 6 + 6 * (scalar / 6) = scalar := by
    exact Nat.mod_add_div scalar 6
  calc
    inverseSix • (scalar • generator - (scalar % 6) • generator) =
        inverseSix • ((6 * (scalar / 6)) • generator) := by
      have hsubtract :
          scalar • generator - (scalar % 6) • generator =
            (6 * (scalar / 6)) • generator := by
        have hscalar :
            scalar • generator =
              (scalar % 6) • generator +
                (6 * (scalar / 6)) • generator := by
          rw [← add_nsmul]
          exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
        rw [hscalar]
        abel
      rw [hsubtract]
    _ = inverseSix • (6 • ((scalar / 6) • generator)) := by
      rw [Nat.mul_comm 6 (scalar / 6), mul_nsmul]
    _ = (scalar / 6) • generator :=
      inverse_six_on_cyclic generator order inverseSix quotient
        (scalar / 6) horder hcertificate

/-- A full native score, bound to the current finite-cohort point, supplies the
digit premise for the concrete cyclic decoder update. -/
theorem full_score_cyclic_group_step
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (point : Cohort)
    (generator current : A)
    (scalar digit order inverseSix quotient : ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (hcurrent : current = scalar • generator)
    (hpointScalar : scalarOf point = scalar)
    (hdigit : digit = predicate (observable point))
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    inverseSix • (current - digit • generator) =
      (scalar / 6) • generator := by
  have hexact : predicate (observable point) = scalarOf point % 6 :=
    full_score_supplies_baseSix observable predicate scalarOf hfull point
  have hdigitScalar : digit = scalar % 6 := by
    calc
      digit = predicate (observable point) := hdigit
      _ = scalarOf point % 6 := hexact
      _ = scalar % 6 := by rw [hpointScalar]
  exact baseSix_cyclic_group_step generator current scalar digit order
    inverseSix quotient hcurrent hdigitScalar horder hcertificate

/-- A full-score row drives every bound query in the canonical group trace,
provided each frozen cohort record is bound to the corresponding quotient
scalar and public group point. -/
theorem full_score_group_trace_step
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseSix quotient round : ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 6 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 6 initialScalar i • generator)
    (hround : round < Nat.log 6 initialScalar + 1)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    inverseSix •
        (groupAt (pointAt round) -
          predicate (observable (pointAt round)) • generator) =
      radixRoundState 6 initialScalar (round + 1) • generator := by
  have hstep := full_score_cyclic_group_step
    observable predicate scalarOf (pointAt round)
    generator (groupAt (pointAt round))
    (radixRoundState 6 initialScalar round)
    (predicate (observable (pointAt round))) order inverseSix quotient
    hfull (hgroupAt round hround) (hscalarAt round hround) rfl
    horder hcertificate
  calc
    inverseSix •
        (groupAt (pointAt round) -
          predicate (observable (pointAt round)) • generator) =
        (radixRoundState 6 initialScalar round / 6) • generator := hstep
    _ = radixRoundState 6 initialScalar (round + 1) • generator := by
      rw [radixRoundState_succ]

/-- The last positive-scalar query is nonidentity in scalar space and its
verified update lands at the group identity. -/
theorem full_score_group_trace_last_step_zero
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseSix quotient : ℕ)
    (hinitial : initialScalar ≠ 0)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 6 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 6 initialScalar i • generator)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    radixRoundState 6 initialScalar (Nat.log 6 initialScalar) ≠ 0 ∧
      inverseSix •
          (groupAt (pointAt (Nat.log 6 initialScalar)) -
            predicate (observable (pointAt (Nat.log 6 initialScalar))) •
              generator) = 0 := by
  have hqueryPositive :
      0 < radixRoundState 6 initialScalar (Nat.log 6 initialScalar) := by
    exact Nat.div_pos (Nat.pow_log_le_self 6 hinitial) (by positivity)
  have hstep := full_score_group_trace_step
    observable predicate scalarOf pointAt groupAt generator
    initialScalar order inverseSix quotient (Nat.log 6 initialScalar)
    hfull hscalarAt hgroupAt (Nat.lt_succ_self _) horder hcertificate
  refine ⟨hqueryPositive.ne', ?_⟩
  calc
    inverseSix •
        (groupAt (pointAt (Nat.log 6 initialScalar)) -
          predicate (observable (pointAt (Nat.log 6 initialScalar))) •
            generator) =
        radixRoundState 6 initialScalar
          (Nat.log 6 initialScalar + 1) • generator := hstep
    _ = 0 := by rw [baseSix_round_bound, zero_nsmul]

/-- Independent EC verification makes a reconstructed scalar unique whenever
the admitted scalar-action map is injective. -/
theorem verified_scalar_unique
    {Scalar Point : Type*} (mul : Scalar → Point) (target : Point)
    (candidate trueScalar : Scalar) (hcandidate : mul candidate = target)
    (htrue : mul trueScalar = target) (hinjective : Function.Injective mul) :
    candidate = trueScalar := by
  apply hinjective
  rw [hcandidate, htrue]

/-- Final H113 certificate. A source-independently verified full-score row and
its explicit trace bindings imply complete arithmetic recovery, a nonidentity
last query, an identity terminal group update, and uniqueness under the
independent final scalar verification map. The full-score premise remains an
input; this theorem does not assert that the winding predicate has it. -/
theorem h113_conditional_end_to_end_certificate
    {Cohort Value Scalar VerifyPoint : Type*}
    [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseSix quotient : ℕ)
    (hinitial : initialScalar ≠ 0)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 6 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 6 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 6 initialScalar i • generator)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order)
    (verifyMul : Scalar → VerifyPoint) (target : VerifyPoint)
    (candidate trueScalar : Scalar)
    (hcandidate : verifyMul candidate = target)
    (htrue : verifyMul trueScalar = target)
    (hinjective : Function.Injective verifyMul) :
    radixRun 6 initialScalar (Nat.log 6 initialScalar + 1) =
        (initialScalar, 0) ∧
      radixRoundState 6 initialScalar (Nat.log 6 initialScalar) ≠ 0 ∧
      inverseSix •
          (groupAt (pointAt (Nat.log 6 initialScalar)) -
            predicate (observable (pointAt (Nat.log 6 initialScalar))) •
              generator) = 0 ∧
      candidate = trueScalar := by
  have hrun := baseSix_run_recovers initialScalar
  have hlast := full_score_group_trace_last_step_zero
    observable predicate scalarOf pointAt groupAt generator
    initialScalar order inverseSix quotient hinitial hfull
    hscalarAt hgroupAt horder hcertificate
  have hunique := verified_scalar_unique verifyMul target candidate trueScalar
    hcandidate htrue hinjective
  exact ⟨hrun, hlast.1, hlast.2, hunique⟩

end GroupDecoder

#print axioms carry_two_three_path
#print axioms carry_three_two_path
#print axioms commuting_carry_paths
#print axioms representative_winding_range
#print axioms exact_predicate_supplies_baseSix
#print axioms agreementScore
#print axioms full_agreement_score_implies_exact
#print axioms full_score_supplies_baseSix
#print axioms radixReconstruct_eq
#print axioms baseSixReconstruct_eq
#print axioms radixRoundState
#print axioms radixRoundState_succ
#print axioms radixRoundState_log_succ_zero
#print axioms baseSix_round_bound
#print axioms radixRun
#print axioms radixRun_invariant
#print axioms radixRun_remaining_eq
#print axioms radixRun_log_succ_eq
#print axioms baseSix_run_recovers
#print axioms baseSix_group_step
#print axioms inverse_six_on_cyclic
#print axioms baseSix_cyclic_group_step
#print axioms full_score_cyclic_group_step
#print axioms full_score_group_trace_step
#print axioms full_score_group_trace_last_step_zero
#print axioms verified_scalar_unique
#print axioms h113_conditional_end_to_end_certificate

end H113BinaryTrinaryWinding
