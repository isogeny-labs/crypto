import CryptoLean.Polynomial.ProjectiveVelu
import Mathlib.RingTheory.AdjoinRoot
import Mathlib.Data.ZMod.Basic

namespace ProjectiveAuxInstances
open CryptoLean.Polynomial
abbrev K := ZMod 19

/-- The fixed three-step auxiliary map evaluated in the semantic support quotient. -/
noncomputable def towerPair (U : Polynomial K) : AdjoinRoot U × AdjoinRoot U :=
  veluProjectiveStep 17 3 (veluProjectiveStep 16 5
    (veluProjectiveStep 14 1 (AdjoinRoot.root U,1)))

/-- Actual public parameters instantiate the shared projective-step law. -/
theorem towerPair_isCoprime (U : Polynomial K) :
    IsCoprime (towerPair U).1 (towerPair U).2 := by
  have h5 : IsUnit (5 : AdjoinRoot U) := by
    simpa only [Nat.cast_ofNat,map_ofNat] using
      ((ZMod.isUnit_iff_coprime 5 19).mpr (by decide)).map (AdjoinRoot.of U)
  have h3 : IsUnit (3 : AdjoinRoot U) := by
    simpa only [Nat.cast_ofNat,map_ofNat] using
      ((ZMod.isUnit_iff_coprime 3 19).mpr (by decide)).map (AdjoinRoot.of U)
  exact isCoprime_veluProjectiveStep
    (isCoprime_veluProjectiveStep
      (isCoprime_veluProjectiveStep isCoprime_one_right isUnit_one) h5) h3

/-- A finite terminal value discharges the denominator-unit obligation. -/
theorem terminal_denominator_isUnit (U : Polynomial K)
    (hterminal : (towerPair U).1=15*(towerPair U).2) : IsUnit (towerPair U).2 := by
  apply (towerPair_isCoprime U).symm.isUnit_of_dvd
  rw [hterminal]
  exact dvd_mul_left _ _

end ProjectiveAuxInstances
