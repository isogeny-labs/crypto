import Std
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace N83HilbertArithmetic
-- Arithmetic only: the link to polynomial quotient ranks is a separate written proof.
def choose (n k : Nat) : Nat :=
  if k > n then 0 else (List.range k).foldl (fun v j => v * (n-j) / (j+1)) 1
def rank (i : Nat) : Nat :=
  if i < 2 then 0 else min (choose 27 (i-2)) (choose 27 i)
def hilbert (d : Nat) : Nat :=
  ((List.range 28).map (fun i =>
    if i <= d && d-i <= 27 then
      choose 27 i * choose 27 (d-i) - rank i * rank (d-i)
    else 0)).sum
-- Exhausts the support: squarefree monomials on 54 variables have degree at most 54.
theorem finite_first_zero :
    ((List.range 42).all (fun d => hilbert d > 0)) = true ∧
    ((List.range 14).all (fun k => hilbert (42+k) == 0)) = true := by decide
#print axioms finite_first_zero
end N83HilbertArithmetic
