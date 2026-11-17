import Init.Grind.Module.Basic

/-! Proper-map recovery, wrong transpose sign and the canonical six-branch
two-torsion permutation counts. Actual Picard maps, group order, geometric
Weierstrass identification and branch relabeling are native/written inputs. -/
set_option autoImplicit false
namespace T20Whole
open Lean.Grind

theorem rational_half_recovery {M : Type} [IntModule M]
    (x : M) (order : (12317 : Int) • x=0) :
    (6159 : Int) • ((2 : Int) • x)=x := by grind

theorem wrong_transpose_only_identity {M : Type} [IntModule M]
    (x : M) (order : (12317 : Int) • x=0) :
    -((2 : Int) • x)=(2 : Int) • x ↔ x=0 := by
  constructor
  · intro h
    have h4 : (4 : Int) • x=0 := by grind
    have hm : (12316 : Int) • x=0 := by
      calc
        _ = (3079 : Int) • ((4 : Int) • x) := by grind
        _ = 0 := by rw [h4]; grind
    grind
  · intro h
    rw [h]
    grind

theorem recovery_and_chain_constants :
    (2 : Nat)*6159%12317=1 ∧ (98 : Nat)*99%109=1 ∧
    (2 : Nat)*57%113=1 ∧ (49 : Nat)*4=196 ∧
    (12317 : Nat)-1=12316 := by decide

def sourcePermutation (i : Nat) : Nat :=
  if i<3 then (i+1)%3 else 3+(i-3+1)%3
def targetPermutation (i : Nat) : Nat := (i+1)%6
def opposite (i : Nat) : Nat := (i+3)%6

theorem branch_permutation_cubes :
    ∀ i : Fin 6,
      sourcePermutation (sourcePermutation (sourcePermutation i.val))=i.val ∧
      targetPermutation (targetPermutation (targetPermutation i.val))=opposite i.val := by decide

def pairs : List (Nat × Nat) :=
  [(0,1),(0,2),(0,3),(0,4),(0,5),
   (1,2),(1,3),(1,4),(1,5),(2,3),(2,4),(2,5),(3,4),(3,5),(4,5)]
def fixedPair (p : Nat × Nat) : Bool :=
  (opposite p.1==p.1 && opposite p.2==p.2) ||
  (opposite p.1==p.2 && opposite p.2==p.1)

theorem actual_target_fixed_pairs :
    pairs.filter fixedPair=[(0,3),(1,4),(2,5)] := by decide

theorem fixed_two_torsion_counts :
    1+pairs.length=16 ∧ 1+(pairs.filter fixedPair).length=4 := by decide

end T20Whole
#print axioms T20Whole.rational_half_recovery
#print axioms T20Whole.wrong_transpose_only_identity
#print axioms T20Whole.recovery_and_chain_constants
#print axioms T20Whole.branch_permutation_cubes
#print axioms T20Whole.actual_target_fixed_pairs
#print axioms T20Whole.fixed_two_torsion_counts
