import Mathlib.Tactic.Ring

namespace Inverse88

variable {R : Type*} [CommRing R]

def equation (p q : R) : R := q ^ 2 + p ^ 2 * q + p + p ^ 3
def k (p q : R) : R := p + q + p ^ 2
def G (p q : R) : R := p * (k p q) ^ 2 + p + (k p q) ^ 2
def qResidual (p q : R) : R :=
  p * ((p + k p q) * (k p q + 1) + p) + q * (k p q) ^ 2

theorem target_base_product (p q : R) (hchar : (2 : R) = 0) :
    qResidual p q = k p q * equation p q := by
  unfold qResidual k equation
  calc
    p * ((p + (p + q + p ^ 2)) * ((p + q + p ^ 2) + 1) + p) +
        q * (p + q + p ^ 2) ^ 2 =
      (p + q + p ^ 2) * (q ^ 2 + p ^ 2 * q + p + p ^ 3) +
        2 * (p ^ 4 + p ^ 3 * q + p ^ 3 + 2 * p ^ 2 * q +
             p * q ^ 2 + p ^ 2) := by ring
    _ = (p + q + p ^ 2) * (q ^ 2 + p ^ 2 * q + p + p ^ 3) := by
      rw [hchar]
      ring

theorem target_base_return (p q : R) (hchar : (2 : R) = 0)
    (hE : equation p q = 0) : qResidual p q = 0 := by
  rw [target_base_product p q hchar, hE, mul_zero]

theorem lambda_return (p q t : R) (hchar : (2 : R) = 0) :
    G p q * (t + (k p q + 1) ^ 2) +
        G p q * (k p q + 1) ^ 2 = G p q * t := by
  calc
    G p q * (t + (k p q + 1) ^ 2) + G p q * (k p q + 1) ^ 2 =
      G p q * t + 2 * G p q * (k p q + 1) ^ 2 := by ring
    _ = G p q * t := by rw [hchar]; ring

end Inverse88
