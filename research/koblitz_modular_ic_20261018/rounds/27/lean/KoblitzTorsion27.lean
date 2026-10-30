import Mathlib.Tactic.Abel
import Mathlib.Algebra.Group.Basic
import Lean.Elab.Tactic.Omega

/-! Integral identities in an arbitrary additive group. The actual principal
divisors and complete RR nonvanishing certificate are bound separately. -/
namespace KoblitzTorsion27
variable {A : Type*} [AddCommGroup A]

def r0 (a : Fin 5 → A) : A :=
  (11:ℤ) • a 0 + (7:ℤ) • a 2 + (11:ℤ) • a 3 + (5:ℤ) • a 4
def r1 (a : Fin 5 → A) : A :=
  (-5:ℤ) • a 0 + (11:ℤ) • a 1 + (7:ℤ) • a 3 + (11:ℤ) • a 4
def r2 (a : Fin 5 → A) : A :=
  (-11:ℤ) • a 0 + (-5:ℤ) • a 1 + (11:ℤ) • a 2 + (7:ℤ) • a 4
def r3 (a : Fin 5 → A) : A :=
  (-7:ℤ) • a 0 + (-11:ℤ) • a 1 + (-5:ℤ) • a 2 + (11:ℤ) • a 3
def r4 (a : Fin 5 → A) : A :=
  (-7:ℤ) • a 1 + (-11:ℤ) • a 2 + (-5:ℤ) • a 3 + (11:ℤ) • a 4
def torsion (a : Fin 5 → A) : A :=
  (-9:ℤ) • a 0 + (-11:ℤ) • a 1 + (-4:ℤ) • a 2 + (8:ℤ) • a 3 + (10:ℤ) • a 4
def shifted (a : Fin 5 → A) : A :=
  (-10:ℤ) • a 0 + (-9:ℤ) • a 1 + (-11:ℤ) • a 2 + (-4:ℤ) • a 3 + (8:ℤ) • a 4
def moved (a : Fin 5 → A) : A :=
  a 0 + (-3:ℤ) • a 1 + (10:ℤ) • a 2 + (2:ℤ) • a 3

theorem actual_eleven_identity (a : Fin 5 → A) :
    (11:ℤ) • torsion a = r0 a + (2:ℤ) • r1 a + (4:ℤ) • r2 a +
      (8:ℤ) • r3 a + (5:ℤ) • r4 a := by
  dsimp [torsion,r0,r1,r2,r3,r4]
  abel

theorem actual_diamond_identity (a : Fin 5 → A) :
    shifted a - (6:ℤ) • torsion a =
      -(r0 a + r1 a + (2:ℤ) • r2 a + (4:ℤ) • r3 a + (2:ℤ) • r4 a) := by
  dsimp [shifted,torsion,r0,r1,r2,r3,r4]
  abel

theorem actual_moving_identity (a : Fin 5 → A) :
    moved a - (4:ℤ) • torsion a =
      -(r1 a + r2 a + (3:ℤ) • r3 a + (2:ℤ) • r4 a) := by
  dsimp [moved,torsion,r1,r2,r3,r4]
  abel

theorem eleven_kills (a : Fin 5 → A)
    (h0 : r0 a=0) (h1 : r1 a=0) (h2 : r2 a=0) (h3 : r3 a=0) (h4 : r4 a=0) :
    (11:ℤ) • torsion a = 0 := by
  rw [actual_eleven_identity,h0,h1,h2,h3,h4]
  simp only [zsmul_zero, add_zero, zero_add, neg_zero]

theorem diamond_is_six (a : Fin 5 → A)
    (h0 : r0 a=0) (h1 : r1 a=0) (h2 : r2 a=0) (h3 : r3 a=0) (h4 : r4 a=0) :
    shifted a = (6:ℤ) • torsion a := by
  apply sub_eq_zero.mp
  rw [actual_diamond_identity,h0,h1,h2,h3,h4]
  simp only [zsmul_zero, add_zero, zero_add, neg_zero]

theorem moved_is_four (a : Fin 5 → A)
    (h1 : r1 a=0) (h2 : r2 a=0) (h3 : r3 a=0) (h4 : r4 a=0) :
    moved a = (4:ℤ) • torsion a := by
  apply sub_eq_zero.mp
  rw [actual_moving_identity,h1,h2,h3,h4]
  simp only [zsmul_zero, add_zero, zero_add, neg_zero]

theorem recover_from_moved (a : Fin 5 → A)
    (h0 : r0 a=0) (h1 : r1 a=0) (h2 : r2 a=0) (h3 : r3 a=0) (h4 : r4 a=0) :
    (3:ℤ) • moved a = torsion a := by
  apply sub_eq_zero.mp
  calc
    (3:ℤ) • moved a - torsion a = (11:ℤ) • torsion a := by
      rw [moved_is_four a h1 h2 h3 h4]
      abel
    _ = 0 := eleven_kills a h0 h1 h2 h3 h4

theorem moved_nonzero_implies_original_nonzero (a : Fin 5 → A)
    (h1 : r1 a=0) (h2 : r2 a=0) (h3 : r3 a=0) (h4 : r4 a=0)
    (hne : moved a ≠ 0) : torsion a ≠ 0 := by
  intro h
  apply hne
  rw [moved_is_four a h1 h2 h3 h4,h,zsmul_zero]

theorem mixed_ten_recovers {A : Type*} [AddCommGroup A] (d : A)
    (h : (11:ℤ) • d = 0) : (10:ℤ) • ((10:ℤ) • d) = d := by
  apply sub_eq_zero.mp
  calc
    (10:ℤ) • ((10:ℤ) • d) - d = (9:ℤ) • ((11:ℤ) • d) := by abel
    _ = 0 := by rw [h,zsmul_zero]

theorem trace_coordinate_degree_obstructions :
    (¬ 3 ∣ 34) ∧ (¬ 3 ∣ 80) ∧ (¬ 3 ∣ 110) := by decide

theorem degree_three_tower_impossible (d : ℕ)
    (h : 3*d=34 ∨ 3*d=80 ∨ 3*d=110) : False := by omega

theorem proper_model_degree_bounds :
    (128 : ℕ) ≥ 2*61+2 ∧ (64 : ℕ) ≥ 2*31+2 ∧ 2*129>256 := by decide

theorem quadratic_kernel_dimensions :
    (68*69/2 : ℕ)=2346 ∧ 2346-(256+1-61)=2150 ∧
    (34*35/2 : ℕ)=595 ∧ 595-(128+1-31)=497 := by decide

theorem free_quotient_dimensions :
    (61-1)/5+1=13 ∧ (61-1)/10+1=7 ∧ (31-1)/5+1=7 ∧
    (61-31-13+7 : ℕ)=24 := by decide

theorem coprime_two_five_kernel (x : A)
    (h2 : (2:ℤ) • x = 0) (h5 : (5:ℤ) • x = 0) : x = 0 := by
  calc
    x = (3:ℤ) • ((2:ℤ) • x) - (5:ℤ) • x := by abel
    _ = 0 := by rw [h2,h5]; simp only [zsmul_zero, sub_self]

theorem mixed_polarization_type_counts :
    (18+6 : ℕ)=24 ∧ 2*6=12 ∧ 30-6=24 ∧ 5^12=244140625 := by decide

theorem mixed_polarization_prime_to_eleven :
    (244140625 : ℕ) % 11 = 3 ∧ Nat.Coprime 244140625 11 := by decide

end KoblitzTorsion27
