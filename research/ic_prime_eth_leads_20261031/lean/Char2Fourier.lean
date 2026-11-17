/-
  Char2Fourier.lean -- core Lean (no Mathlib). Computational check of the ingredient a
  characteristic-2 Fourier multiplication gadget needs (lead L1): in the Schönhage ring
  B_N = F_2[x]/(x^{2N} + x^N + 1) with N = 3^ν, the element x is a PRINCIPAL 3N-th root of
  unity (x^{3N} = 1 and Σ_{i<3N} x^{j i} = 0 for 0 < j < 3N), and 3N is odd, hence a unit.
  This is the char-2 replacement for ω = 2^{2J/L} in Z/(2^J+1) used by the Bochum gadget
  (their Lemma 5.3). Checked here for N = 3 (ring F_2[x]/(x^6+x^3+1), L = 9) and N = 9
  (ring F_2[x]/(x^18+x^9+1), L = 27) by kernel evaluation (`decide`), no `native_decide`.
  Polynomials over F_2 are bitmasks; XOR is addition.
  Build: lean Char2Fourier.lean   (toolchain leanprover/lean4:v4.35.0)
-/
namespace Char2Fourier

/-- multiply by x in F_2[x]/(x^{2N}+x^N+1): shift, reduce the overflow bit 2N by x^N + 1. -/
def mulx (N : Nat) (a : Nat) : Nat :=
  let a' := a <<< 1
  if a' &&& (1 <<< (2*N)) != 0 then a' ^^^ ((1 <<< (2*N)) ||| (1 <<< N) ||| 1) else a'

def xpow (N : Nat) : Nat → Nat
  | 0 => 1
  | n+1 => mulx N (xpow N n)

/-- Σ_{i < 3N} x^{j i} in the ring (XOR of the powers). -/
def psum (N j : Nat) : Nat :=
  (List.range (3*N)).foldl (fun acc i => acc ^^^ xpow N (j*i)) 0

/-- principal root check for one ring: x^{3N} = 1 and all nontrivial character sums vanish. -/
def principal (N : Nat) : Bool :=
  xpow N (3*N) == 1 && (List.range (3*N - 1)).all (fun j => psum N (j+1) == 0)

theorem x_is_principal_root_N3 : principal 3 = true := by decide
set_option maxRecDepth 20000 in
theorem x_is_principal_root_N9 : principal 9 = true := by decide
/-- the transform length 3N is odd, so it is a unit in characteristic 2. -/
theorem length_odd (ν : Nat) : (3 * 3^ν) % 2 = 1 := by
  induction ν with
  | zero => rfl
  | succ k ih => rw [Nat.pow_succ, ← Nat.mul_assoc, Nat.mul_comm (3 * 3^k) 3, Nat.mul_mod, ih]

end Char2Fourier

#print axioms Char2Fourier.x_is_principal_root_N3
#print axioms Char2Fourier.x_is_principal_root_N9
#print axioms Char2Fourier.length_odd
