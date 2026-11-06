import GenusSpan

/- Mixed-radix coordinates only. The proper-class-group bijection is a
separate native certificate plus standard group/CM argument, not assumed here. -/
namespace Prime61Coordinates
def encode (a : Fin 68162026) (b : Fin 8) : Fin 545296208 :=
  ⟨a.val + 68162026 * b.val, by omega⟩
def decode (i : Fin 545296208) : Fin 68162026 × Fin 8 :=
  (⟨i.val % 68162026, Nat.mod_lt _ (by decide)⟩,
   ⟨i.val / 68162026, by omega⟩)

theorem decode_encode (a : Fin 68162026) (b : Fin 8) :
    decode (encode a b) = (a,b) := by
  apply Prod.ext <;> apply Fin.ext <;> simp [decode, encode] <;> omega
theorem encode_decode (i : Fin 545296208) :
    encode (decode i).1 (decode i).2 = i := by
  apply Fin.ext
  simp [decode, encode]
  omega
theorem component_coordinate_count : 68162026 * 8 = 545296208 := by decide

#print axioms decode_encode
#print axioms encode_decode
#print axioms component_coordinate_count
end Prime61Coordinates
