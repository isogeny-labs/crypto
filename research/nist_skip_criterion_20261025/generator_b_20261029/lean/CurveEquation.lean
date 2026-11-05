import Std

namespace GeneratorCoefficient

/-- Integer-representative form of the supplied curve equation modulo p. -/
def OnCurve (p b x y : Int) : Prop :=
  (y * y) % p = (x * x * x - 3 * x + b) % p

def SeedEquation (p c b : Int) : Prop :=
  (c * (b * b)) % p = (-27) % p

theorem at_zero (p b y : Int) (h : OnCurve p b 0 y) :
    (y * y) % p = b % p := by
  simpa [OnCurve] using h

theorem at_one (p b y : Int) (h : OnCurve p b 1 y) :
    (y * y) % p = (b - 2) % p := by
  have e : (1 : Int) * 1 * 1 - 3 * 1 + b = b - 2 := by omega
  simpa only [OnCurve, e] using h

theorem transfer_seed_square (p c b z : Int) (hz : z % p = b % p)
    (hs : SeedEquation p c b) :
    (c * (z * z)) % p = (-27) % p := by
  calc
    (c * (z * z)) % p = ((c % p) * (((z % p) * (z % p)) % p)) % p := by
      rw [Int.mul_emod c (z * z) p, Int.mul_emod z z p]
    _ = ((c % p) * (((b % p) * (b % p)) % p)) % p := by rw [hz]
    _ = (c * (b * b)) % p := by
      rw [Int.mul_emod c (b * b) p, Int.mul_emod b b p]
    _ = (-27) % p := hs

theorem seed_at_zero (p c b y : Int) (h : OnCurve p b 0 y)
    (hs : SeedEquation p c b) :
    (c * ((y * y) * (y * y))) % p = (-27) % p := by
  exact transfer_seed_square p c b (y * y) (at_zero p b y h) hs

theorem seed_at_one (p c b y : Int) (h : OnCurve p b 1 y)
    (hs : SeedEquation p c b) :
    (c * ((y * y + 2) * (y * y + 2))) % p = (-27) % p := by
  have hy := at_one p b y h
  have hz : (y * y + 2) % p = b % p := by
    calc
      (y * y + 2) % p = (((y * y) % p) + (2 % p)) % p := Int.add_emod _ _ _
      _ = (((b - 2) % p) + (2 % p)) % p := by rw [hy]
      _ = (b - 2 + 2) % p := (Int.add_emod _ _ _).symm
      _ = b % p := by congr 1; omega
  exact transfer_seed_square p c b (y * y + 2) hz hs

#print axioms at_zero
#print axioms at_one
#print axioms transfer_seed_square
#print axioms seed_at_zero
#print axioms seed_at_one

end GeneratorCoefficient
