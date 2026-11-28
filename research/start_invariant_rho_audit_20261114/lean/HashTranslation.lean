import SidonCounterfamily

namespace StartInvariantRhoAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit RhoFirstCollisionAudit
variable {G I : Type*} [AddCommGroup G]

def shift (a : G) (h : Hash G I) : Hash G I := fun x => h (a + x)

def shiftEquiv (a : G) : Hash G I ≃ Hash G I where
  toFun := shift a
  invFun := shift (-a)
  left_inv h := by funext x; simp [shift]
  right_inv h := by funext x; simp [shift]

section Finite
variable [Fintype G]

theorem weight_shift (w : I → ℝ) (a : G) (h : Hash G I) :
    weight w (shift a h) = weight w h := by
  exact Equiv.prod_comp (Equiv.addLeft a) (fun x => w (h x))

variable [Fintype I] [DecidableEq G]

theorem expectation_shift (w : I → ℝ) (a : G) (F : Hash G I → ℝ) :
    expectation w (fun h => F (shift a h)) = expectation w F := by
  unfold expectation
  calc
    _ = ∑ h : Hash G I, weight w (shift a h) * F (shift a h) := by
      simp_rw [weight_shift]
    _ = _ := Equiv.sum_comp (shiftEquiv (I := I) a) (fun h => weight w h * F h)

end Finite

def step (g : I → G) (h : Hash G I) (x : G) : G := x + g (h x)

theorem step_translate (g : I → G) (h : Hash G I) (a x : G) :
    step g h (a + x) = a + step g (shift a h) x := by
  simp only [step, shift, add_assoc]

theorem iterate_translate (g : I → G) (h : Hash G I) (a x : G) (n : ℕ) :
    (step g h)^[n] (a + x) = a + (step g (shift a h))^[n] x := by
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [Function.iterate_succ_apply', Function.iterate_succ_apply', ih, step_translate]

section Cyclic
variable (p : ℕ) [NeZero p] (g : I → ZMod p) (h : Hash (ZMod p) I)

omit [NeZero p] in
theorem collision_translate (a x : ZMod p) (n : ℕ) :
    CollisionAt p (step g h) (a + x) n ↔
      CollisionAt p (step g (shift a h)) x n := by
  unfold CollisionAt
  simp_rw [iterate_translate, add_left_cancel_iff]

theorem firstCollision_translate (a x : ZMod p) :
    firstCollision p (step g h) (a + x) = firstCollision p (step g (shift a h)) x := by
  unfold firstCollision
  exact Nat.find_congr' (fun {n} => collision_translate p g h a x n)

theorem firstCollision_from_zero (a : ZMod p) :
    firstCollision p (step g h) a = firstCollision p (step g (shift a h)) 0 := by
  simpa only [add_zero] using firstCollision_translate p g h a 0

variable [Fintype I]

theorem expected_firstCollision_start (w : I → ℝ) (a : ZMod p) :
    expectation w (fun h : Hash (ZMod p) I => (firstCollision p (step g h) a : ℝ)) =
      expectation w (fun h : Hash (ZMod p) I => (firstCollision p (step g h) 0 : ℝ)) := by
  have hf : (fun h : Hash (ZMod p) I => (firstCollision p (step g h) a : ℝ)) =
      (fun h : Hash (ZMod p) I => (firstCollision p (step g (shift a h)) 0 : ℝ)) := by
    funext h
    exact congrArg (fun n : ℕ => (n : ℝ)) (firstCollision_from_zero p g h a)
  rw [hf]
  exact expectation_shift w a (fun h => (firstCollision p (step g h) 0 : ℝ))

theorem firstCollision_statistic_start (w : I → ℝ) (a : ZMod p) (φ : ℕ → ℝ) :
    expectation w (fun h : Hash (ZMod p) I => φ (firstCollision p (step g h) a)) =
      expectation w (fun h : Hash (ZMod p) I => φ (firstCollision p (step g h) 0)) := by
  have hf : (fun h : Hash (ZMod p) I => φ (firstCollision p (step g h) a)) =
      (fun h : Hash (ZMod p) I => φ (firstCollision p (step g (shift a h)) 0)) := by
    funext h
    exact congrArg φ (firstCollision_from_zero p g h a)
  rw [hf]
  exact expectation_shift w a (fun h => φ (firstCollision p (step g h) 0))

#print axioms weight_shift
#print axioms expectation_shift
#print axioms firstCollision_translate
#print axioms expected_firstCollision_start
#print axioms firstCollision_statistic_start
end Cyclic
end
end StartInvariantRhoAudit
