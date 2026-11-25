import Std

/- Integer label identities only. These statements do not formalize elliptic
curve arithmetic, a sampler, relation yield, or measured running time. -/
namespace WalkLabels

/-- Four terms from one affine sequence cancel if the two index sums agree. -/
theorem affine_collision (a d i j k l : Int) (h : i + j = k + l) :
    (a + i*d) + (a + j*d) - (a + k*d) - (a + l*d) = 0 := by
  have hd : (i+j)*d = (k+l)*d := congrArg (fun z => z*d) h
  simp only [Int.add_mul] at hd
  omega

/-- The same index condition cancels both label coordinates. -/
theorem affine_pair_collision (a b da db i j k l : Int) (h : i + j = k + l) :
    ((a + i*da) + (a + j*da) - (a + k*da) - (a + l*da) = 0) ∧
    ((b + i*db) + (b + j*db) - (b + k*db) - (b + l*db) = 0) := by
  exact ⟨affine_collision a da i j k l h, affine_collision b db i j k l h⟩

/-- Four identical labels cancel under two positive and two negative signs. -/
theorem balanced_constant (b : Int) : b + b - b - b = 0 := by omega

#print axioms affine_collision
#print axioms affine_pair_collision
#print axioms balanced_constant
end WalkLabels
