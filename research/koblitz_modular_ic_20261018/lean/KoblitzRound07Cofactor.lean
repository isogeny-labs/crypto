import KoblitzRound07
namespace KoblitzRound07Cofactor
set_option maxRecDepth 200000
set_option maxHeartbeats 0
def pmul (a b : ℕ) : ℕ :=
  (List.range (b.log2+1)).foldl (fun z i => if b.testBit i then Nat.xor z (a*2^i) else z) 0
def nx : ℕ := 4797999389564132366833871562796371339641637035212194096591054750544150139020754201096402344244584407431192578
def dx : ℕ := 2394412697275611706273898607890474378087272784919535017996654055917313235471374517855834335677921641714946049
def nr : ℕ := 4274088270573570896558460598219537716495268876222893227035538220207515686802175727904679384485347779204606704888290515532545874401235878713144784374056560625292865
def dr : ℕ := 4356698302086176848183744910904214929807886590469789439981218964885271726100989694444622538935651897847197848792128293450941247367658233671947009659120149396756033
/-- Exact cleared polynomial identity, encoded by binary coefficient masks. -/
theorem binary_coordinate_identity :
  Nat.xor (Nat.xor (Nat.xor
    (pmul (pmul (pmul nx nx) nx) (pmul dr dr))
    (pmul (pmul nx (pmul dx dx)) (pmul dr dr)))
    (pmul (pmul 10 (pmul (pmul dx dx) dx)) (pmul dr dr)))
    (pmul (Nat.xor (pmul nr nr) (pmul nr dr)) (pmul (pmul dx dx) dx))=0 := by
  decide +kernel
end KoblitzRound07Cofactor
#print axioms KoblitzRound07Cofactor.binary_coordinate_identity
