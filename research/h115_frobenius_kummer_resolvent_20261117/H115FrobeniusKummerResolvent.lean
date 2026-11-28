import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic

/-!
# H115: Frobenius--Kummer resolvent structural obligations

These theorems separate exact generic algebra, finite-score implications and
conditional radix-six decoding. They do not assert that an H115 moment
predicate is exact or efficiently computable.
-/

namespace H115FrobeniusKummerResolvent

section FrobeniusVerschiebung

variable {A : Type*} [AddCommGroup A]

/-- The characteristic-polynomial identities imply both orders of the
factorization `(V - 1) * (pi - 1) = [r]`.  The hypotheses deliberately expose
all elliptic-curve input used by this generic group calculation. -/
theorem verschiebung_frobenius_factor
    (pi V : A →+ A) (p t r : ℕ)
    (hpv : ∀ x, V (pi x) = p • x)
    (hcomm : Function.Commute pi V)
    (htrace : ∀ x, pi x + V x = t • x)
    (hcard : p + 1 = r + t) (x : A) :
    (V (pi x - x) - (pi x - x) = r • x) ∧
      (pi (V x - x) - (V x - x) = r • x) := by
  have hpiv : pi (V x) = p • x := by
    rw [hcomm x]
    exact hpv x
  constructor
  · calc
      V (pi x - x) - (pi x - x) =
          p • x - V x - (pi x - x) := by rw [map_sub, hpv]
      _ = (p + 1) • x - (pi x + V x) := by
        rw [add_nsmul, one_nsmul]
        abel
      _ = (p + 1) • x - t • x := by rw [htrace]
      _ = (r + t) • x - t • x := by rw [hcard]
      _ = r • x := by
        rw [add_nsmul]
        abel
  · calc
      pi (V x - x) - (V x - x) =
          p • x - pi x - (V x - x) := by rw [map_sub, hpiv]
      _ = (p + 1) • x - (pi x + V x) := by
        rw [add_nsmul, one_nsmul]
        abel
      _ = (p + 1) • x - t • x := by rw [htrace]
      _ = (r + t) • x - t • x := by rw [hcard]
      _ = r • x := by
        rw [add_nsmul]
        abel

/-- An `[r]`-division point maps to a point in the frozen `(V-1)` fiber. -/
theorem cocycle_maps_to_target
    (pi V : A →+ A) (p t r : ℕ)
    (hpv : ∀ x, V (pi x) = p • x)
    (hcomm : Function.Commute pi V)
    (htrace : ∀ x, pi x + V x = t • x)
    (hcard : p + 1 = r + t) (R Q : A)
    (hdivision : r • R = Q) :
    V (pi R - R) - (pi R - R) = Q := by
  rw [verschiebung_frobenius_factor pi V p t r hpv hcomm htrace hcard R |>.1]
  exact hdivision

/-- Changing a division point by torsion changes its cocycle by the image of
`pi-1`; no coordinates or field model enter this identity. -/
theorem cocycle_change (pi : A →+ A) (R T : A) :
    pi (R + T) - (R + T) = (pi R - R) + (pi T - T) := by
  rw [map_add]
  abel

/-- The complete fiber of `V-1` over `Q` is the coset of its kernel through
any one section point `c0`. -/
theorem fiber_coset (V : A →+ A) (c0 Q c : A)
    (hc0 : V c0 - c0 = Q) :
    V c - c = Q ↔
      ∃ h, V h - h = 0 ∧ c = c0 + h := by
  constructor
  · intro hc
    refine ⟨c - c0, ?_, ?_⟩
    · calc
        V (c - c0) - (c - c0) =
            (V c - c) - (V c0 - c0) := by
          rw [map_sub]
          abel
        _ = Q - Q := by rw [hc, hc0]
        _ = 0 := sub_self Q
    · abel
  · rintro ⟨h, hh, rfl⟩
    calc
      V (c0 + h) - (c0 + h) =
          (V c0 - c0) + (V h - h) := by
        rw [map_add]
        abel
      _ = Q + 0 := by rw [hc0, hh]
      _ = Q := add_zero Q

/-- A natural-number inverse certificate acts as an inverse on the specified
cyclic subgroup. This is reused by the rational section and decoder. -/
theorem certified_inverse_on_cyclic
    (generator : A) (order factor inverse quotient scalar : ℕ)
    (horder : order • generator = 0)
    (hcertificate : inverse * factor = 1 + quotient * order) :
    inverse • (factor • (scalar • generator)) = scalar • generator := by
  calc
    inverse • (factor • (scalar • generator)) =
        ((inverse * factor) * scalar) • generator := by
      have haction :
          ((scalar * factor) * inverse) • generator =
            inverse • (factor • (scalar • generator)) := by
        calc
          ((scalar * factor) * inverse) • generator =
              inverse • ((scalar * factor) • generator) :=
            mul_nsmul generator (scalar * factor) inverse
          _ = inverse • (factor • (scalar • generator)) := by
            rw [mul_nsmul generator scalar factor]
      have hreorder :
          (scalar * factor) * inverse =
            (inverse * factor) * scalar := by
        ac_rfl
      rw [← haction, hreorder]
    _ = ((1 + quotient * order) * scalar) • generator := by
      rw [hcertificate]
    _ = scalar • generator := by
      rw [add_mul, one_mul, add_nsmul]
      have hzero : ((quotient * order) * scalar) • generator = 0 := by
        have hreorder : (quotient * order) * scalar =
            order * (quotient * scalar) := by ac_rfl
        rw [hreorder, mul_nsmul, horder, nsmul_zero]
      rw [hzero, add_zero]

/-- If `Q` is Frobenius-rational and `inverse` certifies the inverse of
`p-1` modulo its order, then `[inverse]Q` is a public point in the `(V-1)`
fiber over `Q`. -/
theorem rational_section
    (pi V : A →+ A) (p pMinusOne r inverse quotient : ℕ)
    (hpv : ∀ x, V (pi x) = p • x)
    (hp : p = pMinusOne + 1)
    (Q : A) (hpiQ : pi Q = Q) (hrQ : r • Q = 0)
    (hcertificate : inverse * pMinusOne = 1 + quotient * r) :
    V (inverse • Q) - inverse • Q = Q := by
  have hVQ : V Q = p • Q := by
    calc
      V Q = V (pi Q) := by rw [hpiQ]
      _ = p • Q := hpv Q
  have hdiff : V Q - Q = pMinusOne • Q := by
    rw [hVQ, hp, add_nsmul, one_nsmul]
    abel
  calc
    V (inverse • Q) - inverse • Q = inverse • (V Q - Q) := by
      rw [map_nsmul, nsmul_sub]
    _ = inverse • (pMinusOne • Q) := by rw [hdiff]
    _ = Q := by
      simpa using certified_inverse_on_cyclic Q r pMinusOne inverse quotient 1
        hrQ hcertificate

/-- The `[r]`-torsion subset, represented without importing an elliptic-curve
model. -/
def torsionSet (r : ℕ) : Set A := {x | r • x = 0}

/-- The image of `pi-1` restricted to `[r]`-torsion. -/
def cocycleImage (pi : A →+ A) (r : ℕ) : Set A :=
  {c | ∃ T, T ∈ torsionSet r ∧ c = pi T - T}

/-- The kernel of `V-1` restricted to `[r]`-torsion. -/
def fiberKernel (V : A →+ A) (r : ℕ) : Set A :=
  {c | c ∈ torsionSet r ∧ V c - c = 0}

/-- The factorization always gives image containment. Equality is obtained
only from the explicit reverse-lifting (rank/cardinality) premise. -/
theorem image_kernel_handoff
    (pi V : A →+ A) (p t r : ℕ)
    (hpv : ∀ x, V (pi x) = p • x)
    (hcomm : Function.Commute pi V)
    (htrace : ∀ x, pi x + V x = t • x)
    (hcard : p + 1 = r + t)
    (hreverse : ∀ c, c ∈ fiberKernel V r →
      ∃ T, T ∈ torsionSet r ∧ c = pi T - T) :
    cocycleImage pi r = fiberKernel V r := by
  apply Set.Subset.antisymm
  · rintro c ⟨T, hT, rfl⟩
    constructor
    · change r • (pi T - T) = 0
      rw [nsmul_sub, ← map_nsmul, hT, map_zero, sub_zero]
    · exact verschiebung_frobenius_factor pi V p t r hpv hcomm htrace hcard T |>.1.trans hT
  · intro c hc
    exact hreverse c hc

/-- Translation by one preimage identifies a nonempty homomorphism fiber with
the kernel. This is the exact multiplicity mechanism behind the Kummer
cocycle map; no cardinality is hidden in the construction. -/
def homFiberEquivKernel
    {B : Type*} [AddCommGroup B] (f : A →+ B) (y : B) (x : A)
    (hx : f x = y) :
    {z : A // f z = y} ≃ {k : A // f k = 0} where
  toFun z := ⟨z.1 - x, by rw [map_sub, z.2, hx, sub_self]⟩
  invFun k := ⟨k.1 + x, by rw [map_add, k.2, hx, zero_add]⟩
  left_inv z := by
    apply Subtype.ext
    simp
  right_inv k := by
    apply Subtype.ext
    simp

/-- If the kernel has the frozen cardinality `r`, then every nonempty
cocycle fiber has multiplicity exactly `r`. Finiteness and the kernel count
are explicit premises. -/
theorem cocycle_fiber_multiplicity
    {B : Type*} [AddCommGroup B] (f : A →+ B) (y : B) (x : A)
    (hx : f x = y)
    [Fintype {z : A // f z = y}] [Fintype {k : A // f k = 0}]
    (r : ℕ) (hkernel : Fintype.card {k : A // f k = 0} = r) :
    Fintype.card {z : A // f z = y} = r := by
  rw [Fintype.card_congr (homFiberEquivKernel f y x hx)]
  exact hkernel

end FrobeniusVerschiebung

section FiniteScore

variable {Cohort Label Value : Type*}

/-- Native exact-ID score used by the frozen producer and verifier. -/
def agreementScore [Fintype Cohort] [DecidableEq Cohort] [DecidableEq Label]
    (candidate label : Cohort → Label) : ℕ :=
  (Finset.univ.filter fun q => candidate q = label q).card

/-- A full finite score is exactly a pointwise certificate on that finite
cohort; it is not evidence outside the frozen cohort. -/
theorem full_agreement_score_implies_exact
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

/-- A source-independently verified full score supplies H115's radix-six digit
premise on the frozen finite cohort. -/
theorem full_score_supplies_baseSix
    [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalar : Cohort → ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalar q % 6) = Fintype.card Cohort) :
    ∀ q, predicate (observable q) = scalar q % 6 := by
  exact full_agreement_score_implies_exact _ _ hfull

/-- H115's fixed two-limb Chinese-remainder recombination. -/
def crt6 (d2 d3 : ℕ) : ℕ := (3 * d2 + 4 * d3) % 6

theorem crt6_mod_two (d2 d3 : ℕ) (hd2 : d2 < 2) :
    crt6 d2 d3 % 2 = d2 := by
  simp [crt6, Nat.add_mod, Nat.mul_mod, Nat.mod_eq_of_lt hd2]

theorem crt6_mod_three (d2 d3 : ℕ) (hd3 : d3 < 3) :
    crt6 d2 d3 % 3 = d3 := by
  simp [crt6, Nat.add_mod, Nat.mul_mod, Nat.mod_eq_of_lt hd3]

theorem crt6_unique_below
    (x d2 d3 : ℕ) (hx : x < 6) (hd2 : d2 < 2) (hd3 : d3 < 3)
    (hx2 : x % 2 = d2) (hx3 : x % 3 = d3) :
    x = crt6 d2 d3 := by
  have hc2 := crt6_mod_two d2 d3 hd2
  have hc3 := crt6_mod_three d2 d3 hd3
  have hc6 : crt6 d2 d3 < 6 := Nat.mod_lt _ (by omega)
  omega

end FiniteScore

section Decoder

/-- The canonical scalar remaining after `rounds` exact radix divisions. -/
def radixRoundState (base scalar rounds : ℕ) : ℕ :=
  scalar / base ^ rounds

theorem radixRoundState_succ (base scalar rounds : ℕ) :
    radixRoundState base scalar (rounds + 1) =
      radixRoundState base scalar rounds / base := by
  simp [radixRoundState, Nat.pow_succ, Nat.div_div_eq_div_mul]

theorem radixRoundState_log_succ_zero
    (base scalar : ℕ) (hbase : 2 ≤ base) :
    radixRoundState base scalar (Nat.log base scalar + 1) = 0 := by
  apply Nat.div_eq_of_lt
  simpa [radixRoundState] using
    (Nat.lt_pow_succ_log_self (by omega : 1 < base) scalar)

theorem baseSix_round_bound (scalar : ℕ) :
    radixRoundState 6 scalar (Nat.log 6 scalar + 1) = 0 := by
  exact radixRoundState_log_succ_zero 6 scalar (by omega)

/-- A bounded exact radix run: recovered low digits and remaining quotient. -/
def radixRun (base : ℕ) : ℕ → ℕ → ℕ × ℕ
  | scalar, 0 => (0, scalar)
  | scalar, rounds + 1 =>
      let tail := radixRun base (scalar / base) rounds
      (scalar % base + base * tail.1, tail.2)

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
            base ^ (rounds + 1) * (radixRun base scalar (rounds + 1)).2 =
            scalar % base + base * tail.1 +
              (base ^ rounds * base) * tail.2 := by
          simp [radixRun, tail, Nat.pow_succ]
        _ = scalar % base + base * (tail.1 + base ^ rounds * tail.2) := by
          ring
        _ = scalar % base + base * (scalar / base) := by rw [htail]
        _ = scalar := Nat.mod_add_div scalar base

theorem radixRun_remaining_eq (base scalar rounds : ℕ) :
    (radixRun base scalar rounds).2 = scalar / base ^ rounds := by
  induction rounds generalizing scalar with
  | zero => simp [radixRun]
  | succ rounds ih =>
      simp only [radixRun]
      rw [ih, Nat.pow_succ, Nat.div_div_eq_div_mul]
      rw [Nat.mul_comm base (base ^ rounds)]

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

theorem baseSix_run_recovers (scalar : ℕ) :
    radixRun 6 scalar (Nat.log 6 scalar + 1) = (scalar, 0) := by
  exact radixRun_log_succ_eq 6 scalar (by omega)

variable {A : Type*} [AddCommGroup A]

/-- One scalar-blind base-six group update, conditional on the digit and a
concrete inverse-six certificate for the cyclic subgroup. -/
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
  have hdecompose : scalar % 6 + 6 * (scalar / 6) = scalar :=
    Nat.mod_add_div scalar 6
  have hsubtract :
      scalar • generator - (scalar % 6) • generator =
        6 • ((scalar / 6) • generator) := by
    have hscalar : scalar • generator =
        (scalar % 6) • generator +
          (6 * (scalar / 6)) • generator := by
      rw [← add_nsmul]
      exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
    rw [hscalar]
    rw [Nat.mul_comm 6 (scalar / 6), mul_nsmul]
    abel
  rw [hsubtract]
  exact certified_inverse_on_cyclic generator order 6 inverseSix quotient
    (scalar / 6) horder hcertificate

/-- A full independently verified score supplies the digit used by one
concrete cyclic-group decoder update. -/
theorem full_score_baseSix_cyclic_group_step
    {Cohort Value : Type*} [Fintype Cohort] [DecidableEq Cohort]
    (observable : Cohort → Value) (predicate : Value → ℕ)
    (scalarOf : Cohort → ℕ) (point : Cohort)
    (generator current : A)
    (scalar order inverseSix quotient : ℕ)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (hcurrent : current = scalar • generator)
    (hpointScalar : scalarOf point = scalar)
    (horder : order • generator = 0)
    (hcertificate : inverseSix * 6 = 1 + quotient * order) :
    inverseSix •
        (current - predicate (observable point) • generator) =
      (scalar / 6) • generator := by
  have hexact : predicate (observable point) = scalarOf point % 6 :=
    full_score_supplies_baseSix observable predicate scalarOf hfull point
  have hdigit : predicate (observable point) = scalar % 6 := by
    rw [hexact, hpointScalar]
  exact baseSix_cyclic_group_step generator current scalar
    (predicate (observable point)) order inverseSix quotient hcurrent hdigit
    horder hcertificate

/-- Independent scalar verification makes a reconstructed scalar unique. -/
theorem verified_scalar_unique
    {Scalar Point : Type*} (mul : Scalar → Point) (target : Point)
    (candidate trueScalar : Scalar) (hcandidate : mul candidate = target)
    (htrue : mul trueScalar = target) (hinjective : Function.Injective mul) :
    candidate = trueScalar := by
  apply hinjective
  rw [hcandidate, htrue]

end Decoder

section CostBoundary

/-- Named cost terms required by the H115 protocol. No asymptotic property is
built into this accounting record. -/
structure CostTerms where
  extensionConstruction : ℕ
  torsionKernelConstruction : ℕ
  fiberTerms : ℕ
  momentScoring : ℕ
  decoderQueries : ℕ

def CostTerms.total (c : CostTerms) : ℕ :=
  c.extensionConstruction + c.torsionKernelConstruction + c.fiberTerms +
    c.momentScoring + c.decoderQueries

/-- Final conditional H115 certificate. The finite score, public
implementation match and subexponential-cost property are inputs, not
conclusions. -/
theorem h115_conditional_end_to_end_certificate
    {Cohort Value Scalar VerifyPoint GroupPoint : Type*}
    [Fintype Cohort] [DecidableEq Cohort]
    [AddCommGroup GroupPoint]
    (observable specification : Cohort → Value)
    (predicate : Value → ℕ) (scalarOf : Cohort → ℕ)
    (himplementation : observable = specification)
    (hfull : agreementScore (fun q => predicate (observable q))
        (fun q => scalarOf q % 6) = Fintype.card Cohort)
    (initialScalar : ℕ)
    (verifyMul : Scalar → VerifyPoint) (target : VerifyPoint)
    (candidate trueScalar : Scalar)
    (hcandidate : verifyMul candidate = target)
    (htrue : verifyMul trueScalar = target)
    (hinjective : Function.Injective verifyMul)
    (point : Cohort) (generator current : GroupPoint)
    (groupScalar order inverseSix quotient : ℕ)
    (hcurrent : current = groupScalar • generator)
    (hpointScalar : scalarOf point = groupScalar)
    (horder : order • generator = 0)
    (hinverse : inverseSix * 6 = 1 + quotient * order)
    (cost : ℕ → CostTerms) (Subexponential : (ℕ → ℕ) → Prop)
    (hsubexponential : Subexponential (fun r => (cost r).total)) :
    (∀ q, predicate (specification q) = scalarOf q % 6) ∧
      radixRun 6 initialScalar (Nat.log 6 initialScalar + 1) =
        (initialScalar, 0) ∧
      candidate = trueScalar ∧
      inverseSix •
          (current - predicate (specification point) • generator) =
        (groupScalar / 6) • generator ∧
      Subexponential (fun r => (cost r).total) := by
  have hexact := full_score_supplies_baseSix observable predicate scalarOf hfull
  have hspec : ∀ q, predicate (specification q) = scalarOf q % 6 := by
    intro q
    rw [← himplementation]
    exact hexact q
  have hstep :
      inverseSix •
          (current - predicate (specification point) • generator) =
        (groupScalar / 6) • generator := by
    apply baseSix_cyclic_group_step generator current groupScalar
      (predicate (specification point)) order inverseSix quotient hcurrent
    · rw [hspec point, hpointScalar]
    · exact horder
    · exact hinverse
  exact ⟨hspec, baseSix_run_recovers initialScalar,
    verified_scalar_unique verifyMul target candidate trueScalar
      hcandidate htrue hinjective,
    hstep,
    hsubexponential⟩

end CostBoundary

#print axioms verschiebung_frobenius_factor
#print axioms cocycle_maps_to_target
#print axioms cocycle_change
#print axioms fiber_coset
#print axioms certified_inverse_on_cyclic
#print axioms rational_section
#print axioms image_kernel_handoff
#print axioms homFiberEquivKernel
#print axioms cocycle_fiber_multiplicity
#print axioms agreementScore
#print axioms full_agreement_score_implies_exact
#print axioms full_score_supplies_baseSix
#print axioms crt6
#print axioms crt6_mod_two
#print axioms crt6_mod_three
#print axioms crt6_unique_below
#print axioms radixRoundState
#print axioms radixRoundState_succ
#print axioms radixRoundState_log_succ_zero
#print axioms baseSix_round_bound
#print axioms radixRun
#print axioms radixRun_invariant
#print axioms radixRun_remaining_eq
#print axioms radixRun_log_succ_eq
#print axioms baseSix_run_recovers
#print axioms baseSix_cyclic_group_step
#print axioms full_score_baseSix_cyclic_group_step
#print axioms verified_scalar_unique
#print axioms CostTerms.total
#print axioms h115_conditional_end_to_end_certificate

end H115FrobeniusKummerResolvent
