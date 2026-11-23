import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic

/-!
# H114: multiradix carry-cohomology structural obligations

The declarations below are generic algebraic and reconstruction statements.
They do not assert that a measured winding predicate is exact.
-/

namespace H114MultiradixCarryCohomology

section CarryPermutohedron

variable {Q A : Type*}
variable [AddCommGroup A]

/-- The section carry for multiplication by `n`. The section is deliberately
only a set-theoretic function. -/
def carry (n : ℕ) (liftSection : Q → A) (root target : Q) : A :=
  n • liftSection root - liftSection target

/-- Every three-step section-carry path telescopes to its direct product
defect. This is the generic law behind all six H114 paths. -/
theorem three_step_carry_path
    (a b c : ℕ) (liftSection : Q → A) (habc hab ha q : Q) :
    (a * b) • carry c liftSection habc hab +
        a • carry b liftSection hab ha + carry a liftSection ha q =
      carry (a * b * c) liftSection habc q := by
  simp only [carry, nsmul_sub, ← mul_nsmul]
  simp only [Nat.mul_comm, Nat.mul_left_comm]
  abel

/-- The order `2,3,5` telescopes to the direct `[30]` defect. -/
theorem path_235_telescope
    (liftSection : Q → A) (h30 h6 h2 q : Q) :
    6 • carry 5 liftSection h30 h6 +
        2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 2 3 5 liftSection h30 h6 h2 q

/-- The order `2,5,3` telescopes to the direct `[30]` defect. -/
theorem path_253_telescope
    (liftSection : Q → A) (h30 h10 h2 q : Q) :
    10 • carry 3 liftSection h30 h10 +
        2 • carry 5 liftSection h10 h2 + carry 2 liftSection h2 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 2 5 3 liftSection h30 h10 h2 q

/-- The order `3,2,5` telescopes to the direct `[30]` defect. -/
theorem path_325_telescope
    (liftSection : Q → A) (h30 h6 h3 q : Q) :
    6 • carry 5 liftSection h30 h6 +
        3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 3 2 5 liftSection h30 h6 h3 q

/-- The order `3,5,2` telescopes to the direct `[30]` defect. -/
theorem path_352_telescope
    (liftSection : Q → A) (h30 h15 h3 q : Q) :
    15 • carry 2 liftSection h30 h15 +
        3 • carry 5 liftSection h15 h3 + carry 3 liftSection h3 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 3 5 2 liftSection h30 h15 h3 q

/-- The order `5,2,3` telescopes to the direct `[30]` defect. -/
theorem path_523_telescope
    (liftSection : Q → A) (h30 h10 h5 q : Q) :
    10 • carry 3 liftSection h30 h10 +
        5 • carry 2 liftSection h10 h5 + carry 5 liftSection h5 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 5 2 3 liftSection h30 h10 h5 q

/-- The order `5,3,2` telescopes to the direct `[30]` defect. -/
theorem path_532_telescope
    (liftSection : Q → A) (h30 h15 h5 q : Q) :
    15 • carry 2 liftSection h30 h15 +
        5 • carry 3 liftSection h15 h5 + carry 5 liftSection h5 q =
      carry 30 liftSection h30 q := by
  simpa using three_step_carry_path 5 3 2 liftSection h30 h15 h5 q

end CarryPermutohedron

section Representatives

/-- The `(2,3)` numerator and canonical representatives force H114's frozen
range `[-3,2]`. -/
theorem w23_representative_range
    (p x y z t winding : ℤ)
    (hp : 0 < p)
    (hx0 : 0 ≤ x) (hxp : x < p)
    (hy0 : 0 ≤ y) (hyp : y < p)
    (hz0 : 0 ≤ z) (hzp : z < p)
    (ht0 : 0 ≤ t) (htp : t < p)
    (heq : 2 * x + y - 3 * z - t = p * winding) :
    -3 ≤ winding ∧ winding ≤ 2 := by
  have hlower : -4 * p < 2 * x + y - 3 * z - t := by omega
  have hupper : 2 * x + y - 3 * z - t < 3 * p := by omega
  constructor <;> nlinarith

/-- The `(2,5)` numerator and canonical representatives force H114's frozen
range `[-5,2]`. -/
theorem w25_representative_range
    (p x y z t winding : ℤ)
    (hp : 0 < p)
    (hx0 : 0 ≤ x) (hxp : x < p)
    (hy0 : 0 ≤ y) (hyp : y < p)
    (hz0 : 0 ≤ z) (hzp : z < p)
    (ht0 : 0 ≤ t) (htp : t < p)
    (heq : 2 * x + y - 5 * z - t = p * winding) :
    -5 ≤ winding ∧ winding ≤ 2 := by
  have hlower : -6 * p < 2 * x + y - 5 * z - t := by omega
  have hupper : 2 * x + y - 5 * z - t < 3 * p := by omega
  constructor <;> nlinarith

/-- The `(3,5)` numerator and canonical representatives force H114's frozen
range `[-5,3]`. -/
theorem w35_representative_range
    (p x y z t winding : ℤ)
    (hp : 0 < p)
    (hx0 : 0 ≤ x) (hxp : x < p)
    (hy0 : 0 ≤ y) (hyp : y < p)
    (hz0 : 0 ≤ z) (hzp : z < p)
    (ht0 : 0 ≤ t) (htp : t < p)
    (heq : 3 * x + y - 5 * z - t = p * winding) :
    -5 ≤ winding ∧ winding ≤ 3 := by
  have hlower : -6 * p < 3 * x + y - 5 * z - t := by omega
  have hupper : 3 * x + y - 5 * z - t < 4 * p := by omega
  constructor <;> nlinarith

end Representatives

section Braid

/-- The two three-edge routes from `235` to `532` give the same integer
winding sum. The six edge equations are the explicit native-arithmetic
handoff; the nonzero formal-layer modulus permits cancellation. -/
theorem braid_winding_coherence
    (p k235 k253 k325 k352 k523 k532
      w23q w25q3 w35q w35q2 w25q w23q5 : ℤ)
    (hp : p ≠ 0)
    (h23 : k235 - k325 = p * w23q)
    (h25q3 : k325 - k352 = 3 * p * w25q3)
    (h35 : k352 - k532 = p * w35q)
    (h35q2 : k235 - k253 = 2 * p * w35q2)
    (h25 : k253 - k523 = p * w25q)
    (h23q5 : k523 - k532 = 5 * p * w23q5) :
    w23q + 3 * w25q3 + w35q =
      2 * w35q2 + w25q + 5 * w23q5 := by
  have hzero :
      p * ((w23q + 3 * w25q3 + w35q) -
        (2 * w35q2 + w25q + 5 * w23q5)) = 0 := by
    linear_combination -h23 - h25q3 - h35 + h35q2 + h25 + h23q5
  have hdiff :
      (w23q + 3 * w25q3 + w35q) -
        (2 * w35q2 + w25q + 5 * w23q5) = 0 :=
    (mul_eq_zero.mp hzero).resolve_left hp
  omega

end Braid

section CRT

/-- H114's fixed Chinese-remainder recombination. -/
def crt30 (d2 d3 d5 : ℕ) : ℕ :=
  (15 * d2 + 10 * d3 + 6 * d5) % 30

/-- The CRT digit recovers its canonical mod-2 limb. -/
theorem crt30_mod_two (d2 d3 d5 : ℕ) (hd2 : d2 < 2) :
    crt30 d2 d3 d5 % 2 = d2 := by
  simp [crt30, Nat.add_mod, Nat.mul_mod, Nat.mod_eq_of_lt hd2]

/-- The CRT digit recovers its canonical mod-3 limb. -/
theorem crt30_mod_three (d2 d3 d5 : ℕ) (hd3 : d3 < 3) :
    crt30 d2 d3 d5 % 3 = d3 := by
  simp [crt30, Nat.add_mod, Nat.mul_mod, Nat.mod_eq_of_lt hd3]

/-- The CRT digit recovers its canonical mod-5 limb. -/
theorem crt30_mod_five (d2 d3 d5 : ℕ) (hd5 : d5 < 5) :
    crt30 d2 d3 d5 % 5 = d5 := by
  simp [crt30, Nat.add_mod, Nat.mul_mod, Nat.mod_eq_of_lt hd5]

/-- Below 30, the three canonical limb residues uniquely determine the CRT
digit. -/
theorem crt30_unique_below
    (x d2 d3 d5 : ℕ)
    (hx : x < 30) (hd2 : d2 < 2) (hd3 : d3 < 3) (hd5 : d5 < 5)
    (hx2 : x % 2 = d2) (hx3 : x % 3 = d3) (hx5 : x % 5 = d5) :
    x = crt30 d2 d3 d5 := by
  have hc2 := crt30_mod_two d2 d3 d5 hd2
  have hc3 := crt30_mod_three d2 d3 d5 hd3
  have hc5 := crt30_mod_five d2 d3 d5 hd5
  have hc30 : crt30 d2 d3 d5 < 30 := Nat.mod_lt _ (by omega)
  omega

end CRT

section Oracle

variable {Point Value : Type*}

/-- Exactness of a public predicate supplies the canonical base-thirty digit.
Exactness is a hypothesis, not a conclusion of the formalization. -/
theorem exact_predicate_supplies_baseThirty
    (observable : Point → Value) (predicate : Value → ℕ)
    (scalar : Point → ℕ)
    (hexact : ∀ q, predicate (observable q) = scalar q % 30) (q : Point) :
    predicate (observable q) = scalar q % 30 := by
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

/-- A source-independently verified full score supplies the base-thirty oracle
premise on the frozen finite cohort. -/
theorem full_score_supplies_baseThirty
    {Cohort : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalar : Cohort → ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalar q % 30) = Fintype.card Cohort) :
    ∀ q, predicate (observable q) = scalar q % 30 := by
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

/-- H114's integer decoder is the radix-thirty instance of the generic theorem. -/
theorem baseThirtyReconstruct_eq
    (oracle : ℕ → ℕ) (hexact : ∀ k, oracle k = k % 30) :
    ∀ k, radixReconstruct 30 (by omega) oracle k = k := by
  exact radixReconstruct_eq 30 (by omega) oracle hexact

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
theorem baseThirty_round_bound (scalar : ℕ) :
    radixRoundState 30 scalar (Nat.log 30 scalar + 1) = 0 := by
  exact radixRoundState_log_succ_zero 30 scalar (by omega)

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

/-- End-to-end arithmetic reconstruction for H114's frozen radix thirty. -/
theorem baseThirty_run_recovers (scalar : ℕ) :
    radixRun 30 scalar (Nat.log 30 scalar + 1) = (scalar, 0) := by
  exact radixRun_log_succ_eq 30 scalar (by omega)

end Oracle

section GroupDecoder

variable {A : Type*}
variable [AddCommGroup A]

/-- One conditional H114 decoder update. The inverse-thirty action is exposed as
an explicit subgroup obligation. -/
theorem baseThirty_group_step
    (generator current : A) (scalar digit inverseThirty : ℕ)
    (hcurrent : current = scalar • generator)
    (hdigit : digit = scalar % 30)
    (hinverse : ∀ point : A, inverseThirty • (30 • point) = point) :
    inverseThirty • (current - digit • generator) =
      (scalar / 30) • generator := by
  rw [hcurrent, hdigit]
  have hdecompose : scalar % 30 + 30 * (scalar / 30) = scalar := by
    exact Nat.mod_add_div scalar 30
  calc
    inverseThirty • (scalar • generator - (scalar % 30) • generator) =
        inverseThirty • ((30 * (scalar / 30)) • generator) := by
      have hsubtract :
          scalar • generator - (scalar % 30) • generator =
            (30 * (scalar / 30)) • generator := by
        have hscalar :
            scalar • generator =
              (scalar % 30) • generator +
                (30 * (scalar / 30)) • generator := by
          rw [← add_nsmul]
          exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
        rw [hscalar]
        abel
      rw [hsubtract]
    _ = inverseThirty • (30 • ((scalar / 30) • generator)) := by
      rw [Nat.mul_comm 30 (scalar / 30), mul_nsmul]
    _ = (scalar / 30) • generator := hinverse _

/-- A concrete inverse-thirty certificate on the cyclic subgroup generated by
`generator`.  Unlike `baseThirty_group_step`'s generic hypothesis, this requires
no inverse action on unrelated points of the ambient group. -/
theorem inverse_thirty_on_cyclic
    (generator : A) (order inverseThirty quotient scalar : ℕ)
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order) :
    inverseThirty • (30 • (scalar • generator)) = scalar • generator := by
  calc
    inverseThirty • (30 • (scalar • generator)) =
        ((inverseThirty * 30) * scalar) • generator := by
      have haction :
          ((scalar * 30) * inverseThirty) • generator =
            inverseThirty • (30 • (scalar • generator)) := by
        calc
          ((scalar * 30) * inverseThirty) • generator =
              inverseThirty • ((scalar * 30) • generator) :=
            mul_nsmul generator (scalar * 30) inverseThirty
          _ = inverseThirty • (30 • (scalar • generator)) := by
            rw [mul_nsmul generator scalar 30]
      have hreorder :
          (scalar * 30) * inverseThirty =
            (inverseThirty * 30) * scalar := by
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

/-- One base-thirty decoder update from the concrete cyclic-subgroup order and
inverse certificate used by the frozen prime-order cohort. -/
theorem baseThirty_cyclic_group_step
    (generator current : A)
    (scalar digit order inverseThirty quotient : ℕ)
    (hcurrent : current = scalar • generator)
    (hdigit : digit = scalar % 30)
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order) :
    inverseThirty • (current - digit • generator) =
      (scalar / 30) • generator := by
  rw [hcurrent, hdigit]
  have hdecompose : scalar % 30 + 30 * (scalar / 30) = scalar := by
    exact Nat.mod_add_div scalar 30
  calc
    inverseThirty • (scalar • generator - (scalar % 30) • generator) =
        inverseThirty • ((30 * (scalar / 30)) • generator) := by
      have hsubtract :
          scalar • generator - (scalar % 30) • generator =
            (30 * (scalar / 30)) • generator := by
        have hscalar :
            scalar • generator =
              (scalar % 30) • generator +
                (30 * (scalar / 30)) • generator := by
          rw [← add_nsmul]
          exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
        rw [hscalar]
        abel
      rw [hsubtract]
    _ = inverseThirty • (30 • ((scalar / 30) • generator)) := by
      rw [Nat.mul_comm 30 (scalar / 30), mul_nsmul]
    _ = (scalar / 30) • generator :=
      inverse_thirty_on_cyclic generator order inverseThirty quotient
        (scalar / 30) horder hcertificate

/-- A full native score, bound to the current finite-cohort point, supplies the
digit premise for the concrete cyclic decoder update. -/
theorem full_score_cyclic_group_step
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (point : Cohort)
    (generator current : A)
    (scalar digit order inverseThirty quotient : ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 30) = Fintype.card Cohort)
    (hcurrent : current = scalar • generator)
    (hpointScalar : scalarOf point = scalar)
    (hdigit : digit = predicate (observable point))
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order) :
    inverseThirty • (current - digit • generator) =
      (scalar / 30) • generator := by
  have hexact : predicate (observable point) = scalarOf point % 30 :=
    full_score_supplies_baseThirty observable predicate scalarOf hfull point
  have hdigitScalar : digit = scalar % 30 := by
    calc
      digit = predicate (observable point) := hdigit
      _ = scalarOf point % 30 := hexact
      _ = scalar % 30 := by rw [hpointScalar]
  exact baseThirty_cyclic_group_step generator current scalar digit order
    inverseThirty quotient hcurrent hdigitScalar horder hcertificate

/-- A full-score row drives every bound query in the canonical group trace,
provided each frozen cohort record is bound to the corresponding quotient
scalar and public group point. -/
theorem full_score_group_trace_step
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseThirty quotient round : ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 30) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 30 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 30 initialScalar i • generator)
    (hround : round < Nat.log 30 initialScalar + 1)
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order) :
    inverseThirty •
        (groupAt (pointAt round) -
          predicate (observable (pointAt round)) • generator) =
      radixRoundState 30 initialScalar (round + 1) • generator := by
  have hstep := full_score_cyclic_group_step
    observable predicate scalarOf (pointAt round)
    generator (groupAt (pointAt round))
    (radixRoundState 30 initialScalar round)
    (predicate (observable (pointAt round))) order inverseThirty quotient
    hfull (hgroupAt round hround) (hscalarAt round hround) rfl
    horder hcertificate
  calc
    inverseThirty •
        (groupAt (pointAt round) -
          predicate (observable (pointAt round)) • generator) =
        (radixRoundState 30 initialScalar round / 30) • generator := hstep
    _ = radixRoundState 30 initialScalar (round + 1) • generator := by
      rw [radixRoundState_succ]

/-- The last positive-scalar query is nonidentity in scalar space and its
verified update lands at the group identity. -/
theorem full_score_group_trace_last_step_zero
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseThirty quotient : ℕ)
    (hinitial : initialScalar ≠ 0)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 30) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 30 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 30 initialScalar i • generator)
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order) :
    radixRoundState 30 initialScalar (Nat.log 30 initialScalar) ≠ 0 ∧
      inverseThirty •
          (groupAt (pointAt (Nat.log 30 initialScalar)) -
            predicate (observable (pointAt (Nat.log 30 initialScalar))) •
              generator) = 0 := by
  have hqueryPositive :
      0 < radixRoundState 30 initialScalar (Nat.log 30 initialScalar) := by
    exact Nat.div_pos (Nat.pow_log_le_self 30 hinitial) (by positivity)
  have hstep := full_score_group_trace_step
    observable predicate scalarOf pointAt groupAt generator
    initialScalar order inverseThirty quotient (Nat.log 30 initialScalar)
    hfull hscalarAt hgroupAt (Nat.lt_succ_self _) horder hcertificate
  refine ⟨hqueryPositive.ne', ?_⟩
  calc
    inverseThirty •
        (groupAt (pointAt (Nat.log 30 initialScalar)) -
          predicate (observable (pointAt (Nat.log 30 initialScalar))) •
            generator) =
        radixRoundState 30 initialScalar
          (Nat.log 30 initialScalar + 1) • generator := hstep
    _ = 0 := by rw [baseThirty_round_bound, zero_nsmul]

/-- Independent EC verification makes a reconstructed scalar unique whenever
the admitted scalar-action map is injective. -/
theorem verified_scalar_unique
    {Scalar Point : Type*} (mul : Scalar → Point) (target : Point)
    (candidate trueScalar : Scalar) (hcandidate : mul candidate = target)
    (htrue : mul trueScalar = target) (hinjective : Function.Injective mul) :
    candidate = trueScalar := by
  apply hinjective
  rw [hcandidate, htrue]

/-- Final H114 certificate. A source-independently verified full-score row and
its explicit trace bindings imply complete arithmetic recovery, a nonidentity
last query, an identity terminal group update, and uniqueness under the
independent final scalar verification map. The full-score premise remains an
input; this theorem does not assert that the winding predicate has it. -/
theorem h114_conditional_end_to_end_certificate
    {Cohort Value Scalar VerifyPoint : Type*}
    [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (pointAt : ℕ → Cohort)
    (groupAt : Cohort → A) (generator : A)
    (initialScalar order inverseThirty quotient : ℕ)
    (hinitial : initialScalar ≠ 0)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 30) = Fintype.card Cohort)
    (hscalarAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      scalarOf (pointAt i) = radixRoundState 30 initialScalar i)
    (hgroupAt : ∀ i, i < Nat.log 30 initialScalar + 1 →
      groupAt (pointAt i) =
        radixRoundState 30 initialScalar i • generator)
    (horder : order • generator = 0)
    (hcertificate : inverseThirty * 30 = 1 + quotient * order)
    (verifyMul : Scalar → VerifyPoint) (target : VerifyPoint)
    (candidate trueScalar : Scalar)
    (hcandidate : verifyMul candidate = target)
    (htrue : verifyMul trueScalar = target)
    (hinjective : Function.Injective verifyMul) :
    radixRun 30 initialScalar (Nat.log 30 initialScalar + 1) =
        (initialScalar, 0) ∧
      radixRoundState 30 initialScalar (Nat.log 30 initialScalar) ≠ 0 ∧
      inverseThirty •
          (groupAt (pointAt (Nat.log 30 initialScalar)) -
            predicate (observable (pointAt (Nat.log 30 initialScalar))) •
              generator) = 0 ∧
      candidate = trueScalar := by
  have hrun := baseThirty_run_recovers initialScalar
  have hlast := full_score_group_trace_last_step_zero
    observable predicate scalarOf pointAt groupAt generator
    initialScalar order inverseThirty quotient hinitial hfull
    hscalarAt hgroupAt horder hcertificate
  have hunique := verified_scalar_unique verifyMul target candidate trueScalar
    hcandidate htrue hinjective
  exact ⟨hrun, hlast.1, hlast.2, hunique⟩

end GroupDecoder

#print axioms three_step_carry_path
#print axioms path_235_telescope
#print axioms path_253_telescope
#print axioms path_325_telescope
#print axioms path_352_telescope
#print axioms path_523_telescope
#print axioms path_532_telescope
#print axioms w23_representative_range
#print axioms w25_representative_range
#print axioms w35_representative_range
#print axioms braid_winding_coherence
#print axioms crt30
#print axioms crt30_mod_two
#print axioms crt30_mod_three
#print axioms crt30_mod_five
#print axioms crt30_unique_below
#print axioms exact_predicate_supplies_baseThirty
#print axioms agreementScore
#print axioms full_agreement_score_implies_exact
#print axioms full_score_supplies_baseThirty
#print axioms radixReconstruct_eq
#print axioms baseThirtyReconstruct_eq
#print axioms radixRoundState
#print axioms radixRoundState_succ
#print axioms radixRoundState_log_succ_zero
#print axioms baseThirty_round_bound
#print axioms radixRun
#print axioms radixRun_invariant
#print axioms radixRun_remaining_eq
#print axioms radixRun_log_succ_eq
#print axioms baseThirty_run_recovers
#print axioms baseThirty_group_step
#print axioms inverse_thirty_on_cyclic
#print axioms baseThirty_cyclic_group_step
#print axioms full_score_cyclic_group_step
#print axioms full_score_group_trace_step
#print axioms full_score_group_trace_last_step_zero
#print axioms verified_scalar_unique
#print axioms h114_conditional_end_to_end_certificate

end H114MultiradixCarryCohomology
