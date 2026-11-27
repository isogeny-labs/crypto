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
