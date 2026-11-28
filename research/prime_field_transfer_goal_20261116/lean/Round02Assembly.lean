import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.Degree
import Mathlib.Algebra.CharP.Algebra
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic.NormNum

-- Assembly probes only: no new generic law is exported.
example (W : WeierstrassCurve ℤ) :
    (W.preΨ' 19).natDegree = (19 ^ 2 - if Even 19 then 4 else 1) / 2 :=
  W.natDegree_preΨ' (by norm_num)

example (f : ZMod (17 ^ 64) →+* ZMod 2) : 2 ∣ 17 ^ 64 :=
  CharP.dvd_of_ringHom f (17 ^ 64) 2
