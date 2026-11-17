/-
  L01Index.lean -- core Lean, no Mathlib.  Index-arithmetic obligations of the char-2 Schonhage gadget (lead L01).
  (1) carry-less no-wraparound: operands of n <= N bits give a product of degree 2n-2 < 2N  (all n, by omega);
  (2) four-step (Cooley-Tukey) exponent identity for L = P*Q, i = Q*i1 + i2, k = k1 + P*k2:
        i*k = Q*i1*k1 + i2*k1 + P*i2*k2 + (P*Q)*i1*k2  hence  i*k = Q*i1*k1 + i2*k1 + P*i2*k2  (mod L)
      proved for ALL natural numbers by ring-style rewriting;
  (3) the block read-off of the ternary inverse: for 0<=j<N2 the two cyclic shifts  k-2N2, k-N2 (mod 3N2) of c (deg<2N2)
      pick c_{k+N2} for k<N2 -- checked exhaustively for N2 = 3 (low block) and N2 = 9 (low, mid, high blocks) by decide.
  Not covered in Lean: parsimony (the char-2 reduction mod f is a deterministic linear map, so no quotient guess exists);
  the end-to-end multiplication correctness is checked numerically in src/test_gadget.py, not in Lean.
-/
namespace L01Index
theorem no_wrap (n N : Nat) (h : n ≤ N) : 2*n - 1 ≤ 2*N := by omega
theorem prod_deg_lt (n N : Nat) (hn : 1 ≤ n) (h : n ≤ N) : (n-1) + (n-1) < 2*N := by omega

theorem fourstep (P Q i1 i2 k1 k2 : Nat) :
    (Q*i1 + i2) * (k1 + P*k2) = Q*i1*k1 + i2*k1 + P*i2*k2 + (P*Q)*(i1*k2) := by
  simp only [Nat.mul_add, Nat.add_mul, Nat.mul_assoc, Nat.mul_left_comm, Nat.mul_comm, Nat.add_assoc, Nat.add_left_comm]
  omega

theorem fourstep_mod (P Q i1 i2 k1 k2 : Nat) :
    ((Q*i1 + i2) * (k1 + P*k2)) % (P*Q) = (Q*i1*k1 + i2*k1 + P*i2*k2) % (P*Q) := by
  rw [fourstep, Nat.add_mul_mod_self_left]

/-- read-off of c from r = c*(y^{2M}+y^M) mod y^{3M}-1 for deg c < 2M, positions as a function of the index. -/
def cyc (M : Nat) (j s : Nat) : Nat := (j + 3*M - s) % (3*M)   -- index j-s mod 3M
def r_src (M k : Nat) : List Nat :=   -- indices t<2M of c that land on position k: t+2M = k or t+M = k (mod 3M)
  (List.range (2*M)).filter (fun t => (t + 2*M) % (3*M) == k || (t + M) % (3*M) == k)

theorem readoff3 : (List.range 3).all (fun k => if k < 3 then r_src 3 k == [k+3] else true) = true := by decide
theorem readoff9 : (List.range 9).all (fun k => r_src 9 k == [k+9]) = true := by decide
theorem readoff_mid9 : ((List.range 9).map (fun k => r_src 9 (k+9))) = (List.range 9).map (fun k => [k]) := by decide
theorem readoff_hi9 : ((List.range 9).map (fun k => r_src 9 (k+18))) = (List.range 9).map (fun k => [k, k+9]) := by decide
end L01Index
#print axioms L01Index.no_wrap
#print axioms L01Index.fourstep_mod
