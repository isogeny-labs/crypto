import WalkWeightedGeneral

namespace RhoFirstCollisionAudit
noncomputable section
open WalkBoundaryAudit
variable (p : ℕ) [NeZero p]

def CollisionAt (f : ZMod p→ZMod p) (x : ZMod p) (n : ℕ) : Prop :=
  ∃ i : Fin n,f^[i.val] x=f^[n] x

instance collisionAtDecidable (f : ZMod p→ZMod p) (x : ZMod p) (n : ℕ) :
    Decidable (CollisionAt p f x n) := inferInstanceAs (Decidable (∃ i : Fin n,f^[i.val] x=f^[n] x))

theorem collision_exists_bounded (f : ZMod p→ZMod p) (x : ZMod p) :
    ∃ n : ℕ,n≤p ∧ CollisionAt p f x n := by
  obtain ⟨i,j,hij,he⟩:=iterates_repeat p f x
  have hv : i.val≠j.val := fun h=>hij (Fin.ext h)
  rcases lt_or_gt_of_ne hv with h|h
  · exact ⟨j.val,by have hj:=j.isLt;omega,⟨⟨i.val,h⟩,he⟩⟩
  · exact ⟨i.val,by have hi:=i.isLt;omega,⟨⟨j.val,h⟩,he.symm⟩⟩

theorem collision_exists (f : ZMod p→ZMod p) (x : ZMod p) :
    ∃ n : ℕ,CollisionAt p f x n := by
  obtain ⟨n,_,h⟩:=collision_exists_bounded p f x
  exact ⟨n,h⟩

/-- Least index of a repeated state of the actual deterministic iteration. -/
def firstCollision (f : ZMod p→ZMod p) (x : ZMod p) : ℕ :=
  Nat.find (collision_exists p f x)

theorem firstCollision_spec (f : ZMod p→ZMod p) (x : ZMod p) :
    CollisionAt p f x (firstCollision p f x) := Nat.find_spec (collision_exists p f x)

theorem firstCollision_positive (f : ZMod p→ZMod p) (x : ZMod p) :
    0<firstCollision p f x := by
  obtain ⟨i,_⟩:=firstCollision_spec p f x
  have hi:=i.isLt
  omega

theorem firstCollision_le (f : ZMod p→ZMod p) (x : ZMod p) : firstCollision p f x≤p := by
  obtain ⟨n,hn,h⟩:=collision_exists_bounded p f x
  exact (Nat.find_min' (collision_exists p f x) h).trans hn

theorem no_earlier_collision (f : ZMod p→ZMod p) (x : ZMod p) (n : ℕ)
    (hn : n<firstCollision p f x) : ¬CollisionAt p f x n :=
  Nat.find_min (collision_exists p f x) hn

#print axioms firstCollision_spec
#print axioms firstCollision_le
end
end RhoFirstCollisionAudit
