module

public import GroupFactorization

@[expose] public section

/-!
# Formal model of the native extension coefficient ring

The native implementation uses a power-basis presentation
`(Z/2^precision)[U]/f`, where `f` is monic with binary coefficients. This
module defines that quotient, decodes fixed-length serialized coordinate
vectors into it, proves the coefficientwise additive operations, and closes
the `2^precision = 0` premise needed by the exact Cauchy series theorem.

It does not yet prove that the Rust convolution/reduction loop implements the
quotient multiplication or that its deterministic irreducibility search
selects a particular formal bit vector.
-/

open scoped BigOperators
open Polynomial
open Curve25519GroupFactorization

namespace Curve25519ExtensionRingModel

/-- The monic binary-coefficient polynomial represented natively by a leading
degree bit and one Boolean for every lower coefficient. -/
noncomputable def binaryPolynomial
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    Polynomial (ZMod (2 ^ precision)) :=
  X ^ degree +
    ∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0

/-- The formal quotient ring matching the native coefficient-ring shape. -/
abbrev NativeExtension
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :=
  AdjoinRoot (binaryPolynomial precision degree lowerCoefficient)

/-- Decode one fixed-length power-basis coordinate vector into the quotient. -/
noncomputable def decodePowerBasis
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    NativeExtension precision degree lowerCoefficient :=
  ∑ index,
    AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
      (coordinates index) *
      AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
        (index : ℕ)

/-- Decode canonical natural-number representatives from a serialized native
element. The native verifier separately checks vector length and the strict
`coordinate < 2^precision` bounds. -/
noncomputable def decodeSerialized
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ℕ) :
    NativeExtension precision degree lowerCoefficient :=
  decodePowerBasis precision degree lowerCoefficient
    (fun index => coordinates index)

/-- Polynomial whose low power-basis coefficients are one serialized
coordinate vector. -/
noncomputable def coordinatePolynomial
    (precision degree : ℕ)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    Polynomial (ZMod (2 ^ precision)) :=
  ∑ index : Fin degree, monomial (index : ℕ) (coordinates index)

/-- Power-basis decoding is exactly projection of the coordinate polynomial
into the quotient. -/
theorem coordinatePolynomial_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (coordinatePolynomial precision degree coordinates) =
      decodePowerBasis precision degree lowerCoefficient coordinates := by
  rw [coordinatePolynomial, map_sum]
  apply Finset.sum_congr rfl
  intro index _
  rw [← C_mul_X_pow_eq_monomial, map_mul, map_pow]
  rfl

/-- Multiplication of decoded coordinate vectors is projection of the ordinary
coordinate-polynomial convolution product. -/
theorem decodePowerBasis_mul_eq_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient left *
        decodePowerBasis precision degree lowerCoefficient right =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (coordinatePolynomial precision degree left *
          coordinatePolynomial precision degree right) := by
  rw [← coordinatePolynomial_mk, ← coordinatePolynomial_mk, map_mul]

/-- The adjoined power-basis generator satisfies the exact binary-polynomial
relation used by the native high-degree reduction loop. -/
theorem nativeExtension_root_relation
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^ degree =
      -∑ index,
        AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
            (if lowerCoefficient index then 1 else 0) *
          AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
            (index : ℕ) := by
  let f := binaryPolynomial precision degree lowerCoefficient
  let root := AdjoinRoot.root f
  let of := AdjoinRoot.of f
  have hlow :
      Polynomial.eval₂ of root
          (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) =
        ∑ index, of (if lowerCoefficient index then 1 else 0) *
          root ^ (index : ℕ) := by
    change (Polynomial.eval₂RingHom of root)
        (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = _
    rw [map_sum]
    apply Finset.sum_congr rfl
    intro index _
    by_cases h : lowerCoefficient index
    · simp [h, Polynomial.eval₂RingHom]
    · simp [h, Polynomial.eval₂RingHom]
  have hroot := AdjoinRoot.eval₂_root f
  change Polynomial.eval₂ of root
      (X ^ degree +
        ∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 0 at hroot
  rw [eval₂_add, eval₂_pow, eval₂_X, hlow] at hroot
  exact eq_neg_of_add_eq_zero_left hroot

/-- One native high-degree reduction step is sound in the formal quotient:
subtracting the current coefficient at every set lower-polynomial bit replaces
`coefficient * U^(degree+offset)` exactly. -/
theorem nativeExtension_reduce_high_power
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coefficient : ZMod (2 ^ precision)) (offset : ℕ) :
    AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient) coefficient *
        AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
          (degree + offset) =
      ∑ index,
        AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
            (if lowerCoefficient index then -coefficient else 0) *
          AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
            (offset + (index : ℕ)) := by
  let f := binaryPolynomial precision degree lowerCoefficient
  let root := AdjoinRoot.root f
  let of := AdjoinRoot.of f
  calc
    of coefficient * root ^ (degree + offset) =
        of coefficient * root ^ degree * root ^ offset := by
          simp [pow_add, mul_assoc]
    _ = of coefficient *
          (-∑ index, of (if lowerCoefficient index then 1 else 0) *
            root ^ (index : ℕ)) * root ^ offset := by
          rw [nativeExtension_root_relation]
    _ = ∑ index,
          of (if lowerCoefficient index then -coefficient else 0) *
            root ^ (offset + (index : ℕ)) := by
      rw [mul_neg, Finset.mul_sum, neg_mul, Finset.sum_mul,
        ← Finset.sum_neg_distrib]
      apply Finset.sum_congr rfl
      intro index _
      by_cases h : lowerCoefficient index
      · simp [h, pow_add]
        ring
      · simp [h]

/-- The leading degree coefficient of the binary polynomial is one, including
in degenerate coefficient rings. -/
theorem binaryPolynomial_coeff_degree
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    (binaryPolynomial precision degree lowerCoefficient).coeff degree = 1 := by
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun polynomial : Polynomial (ZMod (2 ^ precision)) =>
        polynomial.coeff degree
      map_zero' := coeff_zero degree
      map_add' := fun left right => coeff_add left right degree }
  rw [binaryPolynomial, coeff_add, coeff_X_pow_self]
  change 1 + coefficient
    (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 1
  rw [map_sum]
  have hsum :
      ∑ index, coefficient
        (if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 0 := by
    apply Finset.sum_eq_zero
    intro index _
    by_cases h : lowerCoefficient index
    · simp [coefficient, h, coeff_monomial, ne_of_lt index.isLt]
    · simp [coefficient, h]
  rw [hsum, add_zero]

/-- Logical polynomial form of one descending native reduction update. It
subtracts the targeted coefficient times a shifted copy of the modulus. -/
noncomputable def binaryReductionStep
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    Polynomial (ZMod (2 ^ precision)) :=
  polynomial -
    monomial offset (polynomial.coeff (degree + offset)) *
      binaryPolynomial precision degree lowerCoefficient

/-- One logical reduction step clears its targeted high coefficient. -/
theorem binaryReductionStep_coeff
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
      (degree + offset) = 0 := by
  rw [binaryReductionStep, coeff_sub, coeff_monomial_mul,
    binaryPolynomial_coeff_degree]
  simp

/-- One logical reduction step preserves the represented quotient value. -/
theorem binaryReductionStep_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReductionStep precision degree lowerCoefficient polynomial offset) =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient) polynomial := by
  simp [binaryReductionStep]

/-- Apply a caller-supplied finite schedule of logical reduction offsets. The
native loop uses the descending offsets of its high convolution buffer. -/
noncomputable def binaryReduceOffsets
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    List ℕ → Polynomial (ZMod (2 ^ precision)) →
      Polynomial (ZMod (2 ^ precision))
  | [], polynomial => polynomial
  | offset :: offsets, polynomial =>
      binaryReduceOffsets precision degree lowerCoefficient offsets
        (binaryReductionStep precision degree lowerCoefficient polynomial offset)

/-- Every finite schedule of logical modulus-subtraction steps preserves the
quotient value. Schedule completeness and its in-place Rust array refinement
are separate obligations. -/
theorem binaryReduceOffsets_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (offsets : List ℕ) (polynomial : Polynomial (ZMod (2 ^ precision))) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReduceOffsets precision degree lowerCoefficient offsets polynomial) =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient) polynomial := by
  induction offsets generalizing polynomial with
  | nil => rfl
  | cons offset offsets ih =>
      rw [binaryReduceOffsets, ih, binaryReductionStep_mk]

/-- Coordinate-polynomial convolution followed by any finite logical reduction
schedule represents the product of the decoded inputs. -/
theorem decodePowerBasis_mul_eq_reduced_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) (offsets : List ℕ) :
    decodePowerBasis precision degree lowerCoefficient left *
        decodePowerBasis precision degree lowerCoefficient right =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReduceOffsets precision degree lowerCoefficient offsets
          (coordinatePolynomial precision degree left *
            coordinatePolynomial precision degree right)) := by
  rw [binaryReduceOffsets_mk]
  exact decodePowerBasis_mul_eq_mk precision degree lowerCoefficient left right

/-- The base characteristic relation survives in every polynomial quotient:
the formal native extension satisfies the modulus premise used by the Cauchy
series theorem. -/
theorem nativeExtension_pow_two_zero
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    (2 : NativeExtension precision degree lowerCoefficient) ^ precision = 0 := by
  have hbase : (2 : ZMod (2 ^ precision)) ^ precision = 0 := by
    change ((Nat.castRingHom (ZMod (2 ^ precision))) 2) ^ precision = 0
    rw [← map_pow]
    exact CharP.cast_eq_zero (ZMod (2 ^ precision)) (2 ^ precision)
  change
    (AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient) 2) ^
      precision = 0
  rw [← map_pow, hbase]
  simp

/-- Native coordinatewise addition decodes to quotient-ring addition. -/
theorem decodePowerBasis_add
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (a b : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient
        (fun index => a index + b index) =
      decodePowerBasis precision degree lowerCoefficient a +
        decodePowerBasis precision degree lowerCoefficient b := by
  simp [decodePowerBasis, add_mul, Finset.sum_add_distrib]

/-- Native coordinatewise negation decodes to quotient-ring negation. -/
theorem decodePowerBasis_neg
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (a : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient (fun index => -a index) =
      -decodePowerBasis precision degree lowerCoefficient a := by
  simp [decodePowerBasis]

/-- Exact Rust-shaped Cauchy cutoff in the formal native extension. The
extension's `2^precision = 0` premise is discharged by the quotient model. -/
theorem nativeExtension_cauchy_weight_mode_series_at_precision
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (numerator denominator denominatorInv shiftedInv sample :
      NativeExtension precision degree lowerCoefficient)
    (a i v : ℕ)
    (hvi : v ≤ i) (hia : i ≤ a) (hprecision : 0 < precision)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse :
      (denominator - (2 : NativeExtension precision degree lowerCoefficient) ^
        (i + 1 - v) * sample) * shiftedInv = 1) :
    numerator * shiftedInv *
        (2 : NativeExtension precision degree lowerCoefficient) ^ (a + 1 - v) =
      ∑ index ∈ Finset.range (precision - 1),
        (numerator * denominatorInv ^ (index + 1) *
          (2 : NativeExtension precision degree lowerCoefficient) ^
            ((a + 1 - v) + (i + 1 - v) * index)) * sample ^ index := by
  exact cauchy_weight_mode_series_at_precision
    numerator denominator denominatorInv shiftedInv sample a i v precision
    hvi hia hprecision
    (nativeExtension_pow_two_zero precision degree lowerCoefficient)
    hinverse hshiftedInverse

end Curve25519ExtensionRingModel
