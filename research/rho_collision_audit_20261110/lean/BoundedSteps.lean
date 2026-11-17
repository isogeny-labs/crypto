import FirstCollision

namespace RhoBoundedStepsAudit
noncomputable section
open RhoFirstCollisionAudit
variable (p r : ℕ) [NeZero p]

def increments (i : Fin r) : ZMod p := ((i.val+1:ℕ):ZMod p)
def transition (h : ZMod p→Fin r) (x : ZMod p) : ZMod p := x+increments p r (h x)

def liftPath (h : ZMod p→Fin r) : ℕ→ℕ
  | 0 => 0
  | n+1 => liftPath h n+(h ((transition p r h)^[n] 0)).val+1

omit [NeZero p] in
theorem lift_cast (h : ZMod p→Fin r) (n : ℕ) :
    (liftPath p r h n:ZMod p)=(transition p r h)^[n] 0 := by
  induction n with
  | zero => simp [liftPath]
  | succ n ih =>
    rw [Function.iterate_succ_apply']
    simp only [liftPath,Nat.cast_add,Nat.cast_one,ih,transition,increments]
    ring

omit [NeZero p] in
theorem lift_bound (h : ZMod p→Fin r) (n : ℕ) : liftPath p r h n≤r*n := by
  induction n with
  | zero => simp [liftPath]
  | succ n ih =>
    have hi:=(h ((transition p r h)^[n] 0)).isLt
    rw [liftPath,Nat.mul_succ]
    omega

omit [NeZero p] in
theorem lift_strictMono (h : ZMod p→Fin r) : StrictMono (liftPath p r h) := by
  apply strictMono_nat_of_lt_succ
  intro n
  rw [liftPath]
  omega

omit [NeZero p] in
theorem repeat_lower_bound (h : ZMod p→Fin r) (i k : ℕ) (hik : i<k)
    (he : (transition p r h)^[i] 0=(transition p r h)^[k] 0) : p≤r*k := by
  have hs:=lift_strictMono p r h hik
  have hc : ((liftPath p r h k-liftPath p r h i:ℕ):ZMod p)=0 := by
    rw [Nat.cast_sub (Nat.le_of_lt hs),lift_cast,lift_cast,he,sub_self]
  have hd: p∣liftPath p r h k-liftPath p r h i := (ZMod.natCast_eq_zero_iff _ _).mp hc
  exact (Nat.le_of_dvd (Nat.sub_pos_of_lt hs) hd).trans
    ((Nat.sub_le _ _).trans (lift_bound p r h k))

theorem firstCollision_bounds (h : ZMod p→Fin r) :
    p≤r*firstCollision p (transition p r h) 0 ∧ firstCollision p (transition p r h) 0≤p := by
  obtain ⟨i,hi⟩:=firstCollision_spec p (transition p r h) 0
  exact ⟨repeat_lower_bound p r h i.val _ i.isLt hi,firstCollision_le p _ _⟩

omit [NeZero p] in
theorem increments_injective (hr : r<p) : Function.Injective (increments p r) := by
  intro i j h
  have hi:=i.isLt
  have hj:=j.isLt
  have hv:=congrArg ZMod.val h
  change (((i.val+1:ℕ):ZMod p)).val=(((j.val+1:ℕ):ZMod p)).val at hv
  rw [ZMod.val_natCast_of_lt (by omega),ZMod.val_natCast_of_lt (by omega)] at hv
  apply Fin.ext
  omega

omit [NeZero p] in
theorem increments_nonzero (hr : r<p) (i : Fin r) : increments p r i≠0 := by
  intro h
  have hi:=i.isLt
  have hd:p∣i.val+1 := (ZMod.natCast_eq_zero_iff _ _).mp h
  have hh : p ≤ i.val+1 := Nat.le_of_dvd (Nat.succ_pos i.val) hd
  omega

#print axioms firstCollision_bounds
#print axioms increments_injective
end
end RhoBoundedStepsAudit
