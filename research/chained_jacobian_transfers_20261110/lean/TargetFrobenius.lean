/-! Finite Frobenius certificates and a conditional conjugacy obstruction.
No finite-field or Jacobian geometry is constructed. -/
set_option autoImplicit false
namespace TargetFrobenius

abbrev Label := Fin 6
abbrev Pair := Label × Label
abbrev Matching := List Pair

def cycle (i : Label) : Label :=
  ⟨(i.val+1)%6, Nat.mod_lt _ (by decide)⟩

def sourceCycle (i : Label) : Label :=
  ⟨(if i.val < 3 then (i.val+1)%3 else 3+(i.val-2)%3)%6,
   Nat.mod_lt _ (by decide)⟩

def cube (f : Label → Label) (i : Label) : Label := f (f (f i))

def samePair (p q : Pair) : Bool :=
  (p.1 == q.1 && p.2 == q.2) || (p.1 == q.2 && p.2 == q.1)

def imagePair (f : Label → Label) (p : Pair) : Pair := (f p.1, f p.2)

def branchPairs : List Pair :=
  [(0,1),(0,2),(0,3),(0,4),(0,5),
   (1,2),(1,3),(1,4),(1,5),(2,3),(2,4),(2,5),(3,4),(3,5),(4,5)]

def fixedPairs (f : Label → Label) : List Pair :=
  branchPairs.filter fun p => samePair (imagePair f p) p

def matchings : List Matching :=
  [[(0,1),(2,3),(4,5)],[(0,1),(2,4),(3,5)],[(0,1),(2,5),(3,4)],
   [(0,2),(1,3),(4,5)],[(0,2),(1,4),(3,5)],[(0,2),(1,5),(3,4)],
   [(0,3),(1,2),(4,5)],[(0,3),(1,4),(2,5)],[(0,3),(1,5),(2,4)],
   [(0,4),(1,2),(3,5)],[(0,4),(1,3),(2,5)],[(0,4),(1,5),(2,3)],
   [(0,5),(1,2),(3,4)],[(0,5),(1,3),(2,4)],[(0,5),(1,4),(2,3)]]

def stable (f : Label → Label) (m : Matching) : Bool :=
  m.all fun p => m.any fun q => samePair (imagePair f p) q

theorem source_cube_identity : ∀ i, cube sourceCycle i = i := by decide

theorem source_fixed_pairs : (fixedPairs (cube sourceCycle)).length = 15 := by decide

theorem target_fixed_pairs : fixedPairs (cube cycle) = [(0,3),(1,4),(2,5)] := by decide

theorem unique_target_matching :
    matchings.filter (stable cycle) = [[(0,3),(1,4),(2,5)]] := by decide

theorem target_cube_moves : cube cycle 0 ≠ 0 := by decide

theorem cube_intertwines {A B : Type} (s : A → A) (t : B → B) (f : A → B)
    (h : ∀ a, f (s a) = t (f a)) (a : A) :
    f (s (s (s a))) = t (t (t (f a))) := by rw [h, h, h]

theorem cube_obstruction {A B : Type} (s : A → A) (t : B → B)
    (hs : ∀ a, s (s (s a)) = a) (b : B) (ht : t (t (t b)) ≠ b)
    (f : A → B) (g : B → A) (hr : ∀ y, f (g y) = y)
    (he : ∀ a, f (s a) = t (f a)) : False := by
  have h := cube_intertwines s t f he (g b)
  rw [hs, hr] at h
  exact ht h.symm

end TargetFrobenius
#print axioms TargetFrobenius.source_cube_identity
#print axioms TargetFrobenius.source_fixed_pairs
#print axioms TargetFrobenius.target_fixed_pairs
#print axioms TargetFrobenius.unique_target_matching
#print axioms TargetFrobenius.target_cube_moves
#print axioms TargetFrobenius.cube_intertwines
#print axioms TargetFrobenius.cube_obstruction
