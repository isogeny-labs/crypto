-- Exact polynomial identities over GF(16) with a^4+a+1=0.
-- Elements are 4-bit polynomial-basis codes. The field and quotient geometry
-- are described in REPORT.md; these proofs check the finite identities.

def gfMulAux : Nat → Nat → Nat → Nat → Nat
  | 0, _, _, acc => acc
  | n + 1, a, b, acc =>
      let acc' := if b % 2 == 1 then Nat.xor acc a else acc
      let shifted := a * 2
      let a' := if shifted ≥ 16 then Nat.xor shifted 19 else shifted
      gfMulAux n a' (b / 2) acc'

def gfMul (a b : Nat) : Nat := gfMulAux 4 a b 0

def gf16FiniteCheck : Bool :=
  (List.range 16).all fun a =>
    ((List.range 16).all fun b =>
      gfMul a b < 16 && gfMul a b == gfMul b a) &&
    (a == 0 || (List.range 16).any fun b => gfMul a b == 1)

theorem gf16_finite_check : gf16FiniteCheck = true := by decide

def coeff (p : List Nat) (i : Nat) : Nat := (p[i]?).getD 0

def polyMulCoeff (p q : List Nat) (n : Nat) : Nat :=
  (List.range (n + 1)).foldl
    (fun acc i => Nat.xor acc (gfMul (coeff p i) (coeff q (n - i)))) 0

def polyMul (p q : List Nat) : List Nat :=
  (List.range 13).map (polyMulCoeff p q)

def polyAdd (p q : List Nat) : List Nat :=
  (List.range 13).map (fun i => Nat.xor (coeff p i) (coeff q i))

def polyScale (s : Nat) (p : List Nat) : List Nat :=
  (List.range 13).map (fun i => gfMul s (coeff p i))

def polyPow (p : List Nat) (n : Nat) : List Nat :=
  (List.range n).foldl (fun acc _ => polyMul acc p) [1]

def polySubstitute (F L : List Nat) : List Nat :=
  (List.range 4).foldl
    (fun acc i => polyAdd acc (polyScale (coeff F i) (polyPow L i))) []

def quotientIdentity (h f B F L : List Nat) : Bool :=
  polyAdd (polyAdd f (polyMul B B)) (polyMul h B) == polySubstitute F L

theorem quotient_4006 :
    quotientIdentity [3,14,7,0,5] [3,9,4,14,1,10,0,0,3,6]
      [5,6,11,3,8,11,14] [14,0,8,13] [0,14,7,0,5] = true := by decide

theorem quotient_4054 :
    quotientIdentity [10,13,0,0,5] [7,14,11,6,7,0,0,0,15,7]
      [14,5,15,11,10,6,4] [10,0,8,7] [0,13,0,0,5] = true := by decide

theorem quotient_4078 :
    quotientIdentity [10,1,9,0,5] [6,9,5,13,14,14,0,0,1,9]
      [6,8,1,12,10,7,3] [8,0,8,9] [0,1,9,0,5] = true := by decide

theorem quotient_4126 :
    quotientIdentity [3,1,8,0,5] [15,15,2,7,0,2,0,0,13,14]
      [8,12,3,0,1,11,7] [8,0,8,14] [0,1,8,0,5] = true := by decide

-- Numerical contradiction after applying Riemann--Hurwitz to a hypothetical
-- separable degree >= 2 morphism genus 4 -> genus 3. The theorem itself is
-- an external mathematical input, not formalized here.
theorem genus_four_to_three_degree_two_bound : ¬ (8 ≤ 6) := by omega
