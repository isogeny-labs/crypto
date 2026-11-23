import Init

/-! Explicit finite-group component only. Geometric kernel classification and
Galois/Picard realization are written dependencies, not hidden hypotheses.
Low/high bits encode the C4 coordinate; the other coordinates encode C2. -/
set_option autoImplicit false
namespace TwoHopDescent

structure Mixed where
  low : Bool
  high : Bool
  left : Bool
  right : Bool
deriving DecidableEq

def zeroM : Mixed := ⟨false, false, false, false⟩
def pointM : Mixed := ⟨false, true, false, false⟩
def generatorM : Mixed := ⟨true, false, false, false⟩
def addM (a b : Mixed) : Mixed :=
  ⟨Bool.xor a.low b.low,
   Bool.xor (Bool.xor a.high b.high) (a.low && b.low),
   Bool.xor a.left b.left, Bool.xor a.right b.right⟩
def twiceM (a : Mixed) : Mixed := addM a a

theorem doubled_range (x : Mixed) : twiceM x = zeroM ∨ twiceM x = pointM := by
  cases x with
  | mk a b c d => cases a <;> cases b <;> cases c <;> cases d <;> decide

theorem doubled_generator : twiceM generatorM = pointM := by decide

theorem doubled_point_nonzero : pointM ≠ zeroM := by decide

theorem additive_injection_fixes_doubled_point
    (F : Mixed → Mixed)
    (hzero : F zeroM = zeroM)
    (hinj : Function.Injective F)
    (hadd : ∀ x y, F (addM x y) = addM (F x) (F y)) :
    F pointM = pointM := by
  have hp : F pointM = twiceM (F generatorM) := by
    calc
      F pointM = F (twiceM generatorM) := congrArg F doubled_generator.symm
      _ = twiceM (F generatorM) := hadd generatorM generatorM
  cases doubled_range (F generatorM) with
  | inl h => exact False.elim (doubled_point_nonzero (hinj (hp.trans (h.trans hzero.symm))))
  | inr h => exact hp.trans h

theorem no_fixed_free_mixed_automorphism
    (F : Mixed → Mixed)
    (hzero : F zeroM = zeroM)
    (hinj : Function.Injective F)
    (hadd : ∀ x y, F (addM x y) = addM (F x) (F y))
    (hfree : ∀ x, F x = x → x = zeroM) : False := by
  exact doubled_point_nonzero
    (hfree pointM (additive_injection_fixes_doubled_point F hzero hinj hadd))

structure Good where
  lowA : Bool
  highA : Bool
  lowB : Bool
  highB : Bool
deriving DecidableEq

def addG (a b : Good) : Good :=
  ⟨Bool.xor a.lowA b.lowA,
   Bool.xor (Bool.xor a.highA b.highA) (a.lowA && b.lowA),
   Bool.xor a.lowB b.lowB,
   Bool.xor (Bool.xor a.highB b.highB) (a.lowB && b.lowB)⟩
def twiceG (a : Good) : Good := addG a a
def InDouble (a : Good) : Prop := ∃ b, twiceG b = a

theorem doubling_covariance (F : Good → Good)
    (hadd : ∀ x y, F (addG x y) = addG (F x) (F y)) (x : Good) :
    F (twiceG x) = twiceG (F x) := hadd x x

theorem doubled_image_preserved (F : Good → Good)
    (hadd : ∀ x y, F (addG x y) = addG (F x) (F y))
    (x : Good) (hx : InDouble x) : InDouble (F x) := by
  cases hx with
  | intro y hy => exact ⟨F y, (doubling_covariance F hadd y).symm.trans (congrArg F hy)⟩

structure V where
  a : Bool
  b : Bool
  c : Bool
  d : Bool
deriving DecidableEq

def zeroV : V := ⟨false, false, false, false⟩
def cubic (v : V) : V := ⟨v.b, Bool.xor v.a v.b, v.d, Bool.xor v.c v.d⟩

theorem cubic_fixed_only_zero (v : V) : cubic v = v → v = zeroV := by
  cases v with
  | mk a b c d => cases a <;> cases b <;> cases c <;> cases d <;> decide

theorem cubic_order_three (v : V) : cubic (cubic (cubic v)) = v := by
  cases v with
  | mk a b c d => cases a <;> cases b <;> cases c <;> cases d <;> decide

end TwoHopDescent
#print axioms TwoHopDescent.doubled_range
#print axioms TwoHopDescent.doubled_generator
#print axioms TwoHopDescent.doubled_point_nonzero
#print axioms TwoHopDescent.additive_injection_fixes_doubled_point
#print axioms TwoHopDescent.no_fixed_free_mixed_automorphism
#print axioms TwoHopDescent.doubling_covariance
#print axioms TwoHopDescent.doubled_image_preserved
#print axioms TwoHopDescent.cubic_fixed_only_zero
#print axioms TwoHopDescent.cubic_order_three
