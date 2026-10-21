import KoblitzRound14
open scoped Matrix
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound14Relations
abbrev Point := Option (Nat × Nat)
def step (a : Nat) := if a.testBit 4 then Nat.xor ((a<<<1)%32) 5 else (a<<<1)%32
def mulAux : Nat → Nat → Nat → Nat → Nat
 | 0,_,_,z => z
 | n+1,a,b,z => mulAux n (step a) (b>>>1) (if b.testBit 0 then Nat.xor z a else z)
def mul (a b : Nat) := mulAux 5 a b 0
def pow (a : Nat) (n : Nat) := (List.range n).foldl (fun z _ => mul z a) 1
def inv (a : Nat) := pow a 30
def add (a : Nat) (p q : Point) : Point :=
 match p,q with
 | none,q => q
 | p,none => p
 | some (x,y),some (u,v) =>
   if x=u ∧ (y≠v ∨ x=0) then none else
    let slope := if x=u then Nat.xor x (mul y (inv x)) else mul (Nat.xor y v) (inv (Nat.xor x u))
    let nx := Nat.xor (Nat.xor (Nat.xor (Nat.xor (mul slope slope) slope) x) u) a
    some (nx,Nat.xor (Nat.xor (mul slope (Nat.xor x nx)) y) nx)
def scalar (a n : Nat) (p : Point) := (List.range n).foldl (fun z _ => add a z p) none
def valid (a : Nat) : Point → Bool
 | none => true
 | some (x,y) => (x<32 && y<32) && (Nat.xor (mul y y) (mul x y)==Nat.xor (Nat.xor (mul (mul x x) x) (mul a (mul x x))) 1)
def rowSum (a : Nat) (p : List Point) (r : List Nat) := r.foldl (fun z i => add a z (p[i]!)) none
instance : Fact (Nat.Prime 11) := ⟨by decide⟩
def evenPoints : List Point := [some (30,21),some (10,24),some (19,28),some (14,7),some (8,23),some (19,28),some (14,7),some (8,23),some (30,21),some (10,24),some (30,11),some (10,18),some (19,15),some (14,9),some (8,31),some (10,24),some (19,28),some (14,7),some (8,23),some (30,21),some (10,18),some (19,15),some (14,9),some (8,31),some (30,11),none,none,none,none,none,some (8,31),some (30,11),some (10,18),some (19,15),some (14,9),some (30,11),some (10,18),some (19,15),some (14,9),some (8,31)]
def oddPoints : List Point := [some (16,25),some (2,29),some (13,6),some (4,22),some (27,20),none,none,none,none,none,some (4,18),some (27,15),some (16,9),some (2,31),some (13,11),some (27,15),some (16,9),some (2,31),some (13,11),some (4,18),some (27,15),some (16,9),some (2,31),some (13,11),some (4,18),some (27,20),some (16,25),some (2,29),some (13,6),some (4,22),some (4,22),some (27,20),some (16,25),some (2,29),some (13,6),none,none,none,none,none]
def principalRows : List (List Nat) := [[],[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],[15,16,17,18,19],[20,21,22,23,24,25,26,27,28,29],[],[],[30,31,32,33,34,35,36,37,38,39],[]]
def deckRows : List (List Nat) := [[0,1,2,3,4],[5,6,7,8,9],[10,11,12,13,14],[15,16,17,18,19],[20,21,22,23,24],[25,26,27,28,29],[30,31,32,33,34],[35,36,37,38,39]]
theorem even_points_valid_order11 : (evenPoints.all fun p => valid 1 p && (scalar 1 11 p==none))=true := by decide +kernel
theorem odd_points_valid_order11 : (oddPoints.all fun p => valid 0 p && (scalar 0 11 p==none))=true := by decide +kernel
theorem principal_rows_zero : (principalRows.all fun r => (rowSum 1 evenPoints r==none) && (rowSum 0 oddPoints r==none))=true := by decide +kernel
theorem deck_rows_zero : (deckRows.all fun r => (rowSum 1 evenPoints r==none) && (rowSum 0 oddPoints r==none))=true := by decide +kernel
def deckMinor : Matrix (Fin 8) (Fin 8) (ZMod 11) := !![1,0,0,0,0,0,0,0;0,1,0,0,0,0,0,0;0,0,1,0,0,0,0,0;0,0,0,1,0,0,0,0;0,0,0,0,1,0,0,0;0,0,0,0,0,1,0,0;0,0,0,0,0,0,1,0;0,0,0,0,0,0,0,1]
theorem deck_minor_one : deckMinor=1 := by decide +kernel
theorem deck_minor_rank : Matrix.rank deckMinor=8 := by rw [deck_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 8))
def deck_evenMinor : Matrix (Fin 7) (Fin 7) (ZMod 11) := !![1,0,0,0,0,0,0;0,1,0,0,0,0,0;0,0,1,0,0,0,0;0,0,0,1,0,0,0;0,0,0,0,1,0,0;0,0,0,0,0,1,0;0,0,0,0,0,0,1]
theorem deck_even_minor_one : deck_evenMinor=1 := by decide +kernel
theorem deck_even_minor_rank : Matrix.rank deck_evenMinor=7 := by rw [deck_even_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 7))
def deck_oddMinor : Matrix (Fin 6) (Fin 6) (ZMod 11) := !![1,0,0,0,0,0;0,1,0,0,0,0;0,0,1,0,0,0;0,0,0,1,0,0;0,0,0,0,1,0;0,0,0,0,0,1]
theorem deck_odd_minor_one : deck_oddMinor=1 := by decide +kernel
theorem deck_odd_minor_rank : Matrix.rank deck_oddMinor=6 := by rw [deck_odd_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 6))
def principalMinor : Matrix (Fin 4) (Fin 4) (ZMod 11) := !![1,0,0,0;0,1,0,0;0,0,1,0;0,0,0,1]
theorem principal_minor_one : principalMinor=1 := by decide +kernel
theorem principal_minor_rank : Matrix.rank principalMinor=4 := by rw [principal_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 4))
def principal_evenMinor : Matrix (Fin 4) (Fin 4) (ZMod 11) := !![1,0,0,0;0,1,0,0;0,0,1,0;0,0,0,1]
theorem principal_even_minor_one : principal_evenMinor=1 := by decide +kernel
theorem principal_even_minor_rank : Matrix.rank principal_evenMinor=4 := by rw [principal_even_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 4))
def principal_oddMinor : Matrix (Fin 4) (Fin 4) (ZMod 11) := !![1,0,0,0;0,1,0,0;0,0,1,0;0,0,0,1]
theorem principal_odd_minor_one : principal_oddMinor=1 := by decide +kernel
theorem principal_odd_minor_rank : Matrix.rank principal_oddMinor=4 := by rw [principal_odd_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin 4))
theorem actual_degrees_disjoint : (([6,30,60,210,660] : List Nat).all fun d => d%6==0)=true ∧ (([7,11,28,33,50,85,280] : List Nat).all fun d => d%6!=0)=true := by decide +kernel
theorem fresh_block_coefficients {K ι : Type*} [Field K] [Fintype ι] [DecidableEq ι] (c : ι → K) (v : ι → ι → K) (hv : ∀ i j,v i j=if i=j then 1 else 0) (h : ∀ j,∑ i,c i*v i j=0) : ∀ j,c j=0 := by
 intro j
 simpa [hv] using h j
end KoblitzRound14Relations
#print axioms KoblitzRound14Relations.even_points_valid_order11
#print axioms KoblitzRound14Relations.odd_points_valid_order11
#print axioms KoblitzRound14Relations.principal_rows_zero
#print axioms KoblitzRound14Relations.deck_rows_zero
#print axioms KoblitzRound14Relations.deck_minor_one
#print axioms KoblitzRound14Relations.deck_minor_rank
#print axioms KoblitzRound14Relations.deck_even_minor_one
#print axioms KoblitzRound14Relations.deck_even_minor_rank
#print axioms KoblitzRound14Relations.deck_odd_minor_one
#print axioms KoblitzRound14Relations.deck_odd_minor_rank
#print axioms KoblitzRound14Relations.principal_minor_one
#print axioms KoblitzRound14Relations.principal_minor_rank
#print axioms KoblitzRound14Relations.principal_even_minor_one
#print axioms KoblitzRound14Relations.principal_even_minor_rank
#print axioms KoblitzRound14Relations.principal_odd_minor_one
#print axioms KoblitzRound14Relations.principal_odd_minor_rank
#print axioms KoblitzRound14Relations.actual_degrees_disjoint
#print axioms KoblitzRound14Relations.fresh_block_coefficients
