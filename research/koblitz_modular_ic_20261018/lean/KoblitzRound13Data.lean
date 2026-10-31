import KoblitzRound13
set_option maxHeartbeats 0
set_option maxRecDepth 100000
namespace KoblitzRound13Data
def deck : Fin 5 → Fin 5 → Fin 5 := ![![0,1,2,3,4],![3,4,0,1,2],![4,0,1,2,3],![1,2,3,4,0],![2,3,4,0,1]]
def lambda : Array Nat := #[1,2,4,8,5]
def inverseSquares : Array Nat := #[1,3,5,9,4]
def negative : Array (Array Bool) := #[#[false,false,false,false,false],#[true,true,false,false,false],#[false,true,true,true,true],#[true,true,true,true,false],#[false,false,false,true,true]]
def deckCheck : Bool := (List.finRange 5).all fun r =>
 (((r.val+1)^2*inverseSquares[r.val]!)%11==1) &&
 (List.finRange 5).all fun i =>
  ((lambda[i.val]!*inverseSquares[r.val]!)%11 ==
   if (negative[r.val]!)[i.val]! then 11-lambda[(deck r i).val]! else lambda[(deck r i).val]!) &&
  ((negative[r.val]!)[i.val]! == ((i.val+(deck r i).val)%2==1))
theorem actual_deck_table : deckCheck=true := by decide +kernel
theorem uniform_deck_norm {A : Type*} [AddCommGroup A] (v : Fin 5 → A) (i : Fin 5) :
 (∑ r,v (deck r i))=∑ j,v j := by
 fin_cases i <;> simp [deck,Fin.sum_univ_succ] <;> abel
theorem actual_deck_augmentation_zero {A : Type*} [AddCommGroup A] (v : Fin 5 → A) (i : Fin 5) :
 (∑ r,((5:ℕ) • v (deck r i)-(∑ j,v j)))=0 := by
 rw [Finset.sum_sub_distrib,←Finset.smul_sum,uniform_deck_norm]
 simp
theorem projected_deck_zero {A B : Type*} [AddCommGroup A] [AddCommGroup B]
 (f : A →+ B) (v : Fin 5 → A) (i : Fin 5) :
 f (∑ r,((5:ℕ) • v (deck r i)-(∑ j,v j)))=0 := by
 rw [actual_deck_augmentation_zero,map_zero]
def pmodAux : Nat → Nat → Nat → Nat
 | 0,a,_ => a
 | n+1,a,p => if a=0 ∨ a.log2<p.log2 then a else pmodAux n (Nat.xor a (p<<<(a.log2-p.log2))) p
def pmod (a p : Nat) := pmodAux (a.log2+1) a p
def pmul (a b : Nat) : Nat := (List.range (b.log2+1)).foldl (fun z i => if b.testBit i then Nat.xor z (a<<<i) else z) 0
def pgcdAux : Nat → Nat → Nat → Nat
 | 0,a,_ => a
 | n+1,a,b => if b=0 then a else pgcdAux n b (pmod a b)
def pgcd (a b : Nat) := pgcdAux (a.log2+b.log2+2) a b
def frob (p : Nat) : Nat → Nat
 | 0 => 2
 | n+1 => pmod (pmul (frob p n) (frob p n)) p
def rabin (p d : Nat) (primes : List Nat) : Bool :=
 (p.log2==d) && (frob p d==2) && (primes.all fun l => pgcd p (Nat.xor (frob p (d/l)) 2)==1)
theorem ordinary_base_polynomial_0_Rabin : rabin 623159635 29 [29]=true := by decide +kernel
theorem ordinary_base_polynomial_1_Rabin : rabin 849692969 29 [29]=true := by decide +kernel
theorem ordinary_base_polynomial_2_Rabin : rabin 2169518353982831 50 [2,5]=true := by decide +kernel
theorem source_degree_certificate : ([250,250] : List Nat).sum=500 ∧ ([145,145,145,145] : List Nat).sum=580 := by decide +kernel
end KoblitzRound13Data
#print axioms KoblitzRound13Data.actual_deck_table
#print axioms KoblitzRound13Data.uniform_deck_norm
#print axioms KoblitzRound13Data.actual_deck_augmentation_zero
#print axioms KoblitzRound13Data.projected_deck_zero
#print axioms KoblitzRound13Data.ordinary_base_polynomial_0_Rabin
#print axioms KoblitzRound13Data.ordinary_base_polynomial_1_Rabin
#print axioms KoblitzRound13Data.ordinary_base_polynomial_2_Rabin
#print axioms KoblitzRound13Data.source_degree_certificate
