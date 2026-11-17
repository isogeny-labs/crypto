import KoblitzRound19
namespace KoblitzRound19Maps
def mul (a b:Nat) : Nat :=
 let raw:=(List.range 5).foldl (fun z i => if b.testBit i then Nat.xor z (a*2^i) else z) 0
 ((List.range 5).reverse).foldl (fun z i => if z.testBit (5+i) then Nat.xor z (37*2^i) else z) raw
def cases : List (Nat×Nat×Nat) := [(0,0,1),(0,1,0),(0,1,1),(0,2,29),(0,2,31),(0,3,12),(0,3,15),(0,4,18),(0,4,22),(0,5,26),(0,5,31),(0,7,24),(0,7,31),(0,9,20),(0,9,29),(0,11,22),(0,11,29),(0,12,5),(0,12,9),(0,13,6),(0,13,11),(0,15,22),(0,15,25),(0,16,9),(0,16,25),(0,17,3),(0,17,18),(0,18,6),(0,18,20),(0,21,7),(0,21,18),(0,23,11),(0,23,28),(0,24,15),(0,24,23),(0,26,11),(0,26,17),(0,27,15),(0,27,20),(0,28,9),(0,28,21),(0,31,6),(0,31,25),(1,0,1),(1,6,16),(1,6,22),(1,8,23),(1,8,31),(1,10,18),(1,10,24),(1,14,7),(1,14,9),(1,19,15),(1,19,28),(1,20,13),(1,20,25),(1,22,2),(1,22,20),(1,25,4),(1,25,29),(1,29,6),(1,29,27),(1,30,11),(1,30,21)]
def residual (v:Nat×Nat×Nat) : Nat := let (a,x,y):=v;Nat.xor (Nat.xor (Nat.xor (Nat.xor (mul y y) (mul x y)) (mul x (mul x x))) (mul a (mul x x))) 1
theorem all_constant_maps : cases.all (fun v => residual v==0)=true := by decide +kernel
theorem constant_count : cases.length=64 := by decide +kernel
theorem sign_counts : (cases.countP fun v => v.1=0)=43 ∧ (cases.countP fun v => v.1=1)=21 := by decide +kernel
end KoblitzRound19Maps
#print axioms KoblitzRound19Maps.all_constant_maps
#print axioms KoblitzRound19Maps.constant_count
#print axioms KoblitzRound19Maps.sign_counts
