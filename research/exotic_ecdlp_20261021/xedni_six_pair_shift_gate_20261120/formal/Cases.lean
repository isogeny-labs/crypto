import Shift

namespace XedniSixPairShift

set_option maxRecDepth 20000
set_option maxHeartbeats 0

structure LocalRow where
  x1 : ℤ
  x2 : ℤ
  y1 : ℤ
  y2 : ℤ
  k1 : ℤ
  k2 : ℤ

def localTable : List LocalRow := [
  ⟨8, -4, 19, 19, -7, -26⟩, -- source 201, local [0, 0]
  ⟨8, -4, 19, 19, -28, 37⟩, -- source 201, local [0, 1]
  ⟨8, -4, 19, 19, -28, -19⟩, -- source 201, local [0, 2]
  ⟨8, -4, 19, 19, -35, 9⟩, -- source 201, local [0, 3]
  ⟨8, -4, 19, 19, 35, 9⟩, -- source 201, local [0, 4]
  ⟨8, -4, 19, 19, 28, 37⟩, -- source 201, local [0, 5]
  ⟨8, -4, 19, 19, 28, -19⟩, -- source 201, local [0, 6]
  ⟨8, -4, 19, 19, 7, -26⟩, -- source 201, local [0, 7]
  ⟨8, -4, 19, 19, 37, -15⟩, -- source 201, local [1, 0]
  ⟨8, -4, 19, 19, 16, -29⟩, -- source 201, local [1, 1]
  ⟨8, -4, 19, 19, 16, -8⟩, -- source 201, local [1, 2]
  ⟨8, -4, 19, 19, 9, 20⟩, -- source 201, local [1, 3]
  ⟨8, -4, 19, 19, 2, 20⟩, -- source 201, local [1, 4]
  ⟨8, -4, 19, 19, -5, -29⟩, -- source 201, local [1, 5]
  ⟨8, -4, 19, 19, -5, -8⟩, -- source 201, local [1, 6]
  ⟨8, -4, 19, 19, -26, -15⟩, -- source 201, local [1, 7]
  ⟨8, -4, 19, 19, 26, -15⟩, -- source 201, local [2, 0]
  ⟨8, -4, 19, 19, 5, -29⟩, -- source 201, local [2, 1]
  ⟨8, -4, 19, 19, 5, -8⟩, -- source 201, local [2, 2]
  ⟨8, -4, 19, 19, -2, 20⟩, -- source 201, local [2, 3]
  ⟨8, -4, 19, 19, -9, 20⟩, -- source 201, local [2, 4]
  ⟨8, -4, 19, 19, -16, -29⟩, -- source 201, local [2, 5]
  ⟨8, -4, 19, 19, -16, -8⟩, -- source 201, local [2, 6]
  ⟨8, -4, 19, 19, -37, -15⟩, -- source 201, local [2, 7]
  ⟨8, -4, -19, -19, -7, -26⟩, -- source 243, local [0, 0]
  ⟨8, -4, -19, -19, -28, 37⟩, -- source 243, local [0, 1]
  ⟨8, -4, -19, -19, -28, -19⟩, -- source 243, local [0, 2]
  ⟨8, -4, -19, -19, -35, 9⟩, -- source 243, local [0, 3]
  ⟨8, -4, -19, -19, 35, 9⟩, -- source 243, local [0, 4]
  ⟨8, -4, -19, -19, 28, 37⟩, -- source 243, local [0, 5]
  ⟨8, -4, -19, -19, 28, -19⟩, -- source 243, local [0, 6]
  ⟨8, -4, -19, -19, 7, -26⟩, -- source 243, local [0, 7]
  ⟨8, -4, -19, -19, 37, -15⟩, -- source 243, local [1, 0]
  ⟨8, -4, -19, -19, 16, -29⟩, -- source 243, local [1, 1]
  ⟨8, -4, -19, -19, 16, -8⟩, -- source 243, local [1, 2]
  ⟨8, -4, -19, -19, 9, 20⟩, -- source 243, local [1, 3]
  ⟨8, -4, -19, -19, 2, 20⟩, -- source 243, local [1, 4]
  ⟨8, -4, -19, -19, -5, -29⟩, -- source 243, local [1, 5]
  ⟨8, -4, -19, -19, -5, -8⟩, -- source 243, local [1, 6]
  ⟨8, -4, -19, -19, -26, -15⟩, -- source 243, local [1, 7]
  ⟨8, -4, -19, -19, 26, -15⟩, -- source 243, local [2, 0]
  ⟨8, -4, -19, -19, 5, -29⟩, -- source 243, local [2, 1]
  ⟨8, -4, -19, -19, 5, -8⟩, -- source 243, local [2, 2]
  ⟨8, -4, -19, -19, -2, 20⟩, -- source 243, local [2, 3]
  ⟨8, -4, -19, -19, -9, 20⟩, -- source 243, local [2, 4]
  ⟨8, -4, -19, -19, -16, -29⟩, -- source 243, local [2, 5]
  ⟨8, -4, -19, -19, -16, -8⟩, -- source 243, local [2, 6]
  ⟨8, -4, -19, -19, -37, -15⟩, -- source 243, local [2, 7]
  ⟨14, 17, 7, 10, 0, 34⟩, -- source 425, local [0, 0]
  ⟨14, 17, 7, 10, 0, 20⟩, -- source 425, local [0, 1]
  ⟨14, 17, 7, 10, -7, 20⟩, -- source 425, local [0, 2]
  ⟨14, 17, 7, 10, -21, -22⟩, -- source 425, local [0, 3]
  ⟨14, 17, 7, 10, -28, -29⟩, -- source 425, local [0, 4]
  ⟨14, 17, 7, 10, 35, -15⟩, -- source 425, local [0, 5]
  ⟨14, 17, 7, 10, 28, -29⟩, -- source 425, local [0, 6]
  ⟨14, 17, 7, 10, 28, -15⟩, -- source 425, local [0, 7]
  ⟨14, 17, 7, 10, -33, 1⟩, -- source 425, local [1, 0]
  ⟨14, 17, 7, 10, -33, -13⟩, -- source 425, local [1, 1]
  ⟨14, 17, 7, 10, 37, -13⟩, -- source 425, local [1, 2]
  ⟨14, 17, 7, 10, 23, 22⟩, -- source 425, local [1, 3]
  ⟨14, 17, 7, 10, 16, 15⟩, -- source 425, local [1, 4]
  ⟨14, 17, 7, 10, 2, 29⟩, -- source 425, local [1, 5]
  ⟨14, 17, 7, 10, -5, 15⟩, -- source 425, local [1, 6]
  ⟨14, 17, 7, 10, -5, 29⟩, -- source 425, local [1, 7]
  ⟨14, 17, 7, 10, 22, 34⟩, -- source 425, local [2, 0]
  ⟨14, 17, 7, 10, 22, 20⟩, -- source 425, local [2, 1]
  ⟨14, 17, 7, 10, 15, 20⟩, -- source 425, local [2, 2]
  ⟨14, 17, 7, 10, 1, -22⟩, -- source 425, local [2, 3]
  ⟨14, 17, 7, 10, -6, -29⟩, -- source 425, local [2, 4]
  ⟨14, 17, 7, 10, -20, -15⟩, -- source 425, local [2, 5]
  ⟨14, 17, 7, 10, -27, -29⟩, -- source 425, local [2, 6]
  ⟨14, 17, 7, 10, -27, -15⟩, -- source 425, local [2, 7]
  ⟨14, 17, -7, -10, 0, 34⟩, -- source 467, local [0, 0]
  ⟨14, 17, -7, -10, 0, 20⟩, -- source 467, local [0, 1]
  ⟨14, 17, -7, -10, -28, -29⟩, -- source 467, local [0, 2]
  ⟨14, 17, -7, -10, -28, -15⟩, -- source 467, local [0, 3]
  ⟨14, 17, -7, -10, -35, -15⟩, -- source 467, local [0, 4]
  ⟨14, 17, -7, -10, 28, -29⟩, -- source 467, local [0, 5]
  ⟨14, 17, -7, -10, 21, -22⟩, -- source 467, local [0, 6]
  ⟨14, 17, -7, -10, 7, 20⟩, -- source 467, local [0, 7]
  ⟨14, 17, -7, -10, -22, 34⟩, -- source 467, local [1, 0]
  ⟨14, 17, -7, -10, -22, 20⟩, -- source 467, local [1, 1]
  ⟨14, 17, -7, -10, 27, -29⟩, -- source 467, local [1, 2]
  ⟨14, 17, -7, -10, 27, -15⟩, -- source 467, local [1, 3]
  ⟨14, 17, -7, -10, 20, -15⟩, -- source 467, local [1, 4]
  ⟨14, 17, -7, -10, 6, -29⟩, -- source 467, local [1, 5]
  ⟨14, 17, -7, -10, -1, -22⟩, -- source 467, local [1, 6]
  ⟨14, 17, -7, -10, -15, 20⟩, -- source 467, local [1, 7]
  ⟨14, 17, -7, -10, 33, 1⟩, -- source 467, local [2, 0]
  ⟨14, 17, -7, -10, 33, -13⟩, -- source 467, local [2, 1]
  ⟨14, 17, -7, -10, 5, 15⟩, -- source 467, local [2, 2]
  ⟨14, 17, -7, -10, 5, 29⟩, -- source 467, local [2, 3]
  ⟨14, 17, -7, -10, -2, 29⟩, -- source 467, local [2, 4]
  ⟨14, 17, -7, -10, -16, 15⟩, -- source 467, local [2, 5]
  ⟨14, 17, -7, -10, -23, 22⟩, -- source 467, local [2, 6]
  ⟨14, 17, -7, -10, -37, -13⟩, -- source 467, local [2, 7]
  ⟨15, 23, 16, 4, -22, 12⟩, -- source 511, local [0, 0]
  ⟨15, 23, 16, 4, -29, -9⟩, -- source 511, local [0, 1]
  ⟨15, 23, 16, 4, -29, -23⟩, -- source 511, local [0, 2]
  ⟨15, 23, 16, 4, 20, 33⟩, -- source 511, local [0, 3]
  ⟨15, 23, 16, 4, 20, 19⟩, -- source 511, local [0, 4]
  ⟨15, 23, 16, 4, 13, -2⟩, -- source 511, local [0, 5]
  ⟨15, 23, 16, 4, -1, -9⟩, -- source 511, local [0, 6]
  ⟨15, 23, 16, 4, -8, -37⟩, -- source 511, local [0, 7]
  ⟨15, 23, 16, 4, -33, 23⟩, -- source 511, local [1, 0]
  ⟨15, 23, 16, 4, 37, 2⟩, -- source 511, local [1, 1]
  ⟨15, 23, 16, 4, 37, -12⟩, -- source 511, local [1, 2]
  ⟨15, 23, 16, 4, 9, -33⟩, -- source 511, local [1, 3]
  ⟨15, 23, 16, 4, 9, 30⟩, -- source 511, local [1, 4]
  ⟨15, 23, 16, 4, 2, 9⟩, -- source 511, local [1, 5]
  ⟨15, 23, 16, 4, -12, 2⟩, -- source 511, local [1, 6]
  ⟨15, 23, 16, 4, -19, -26⟩, -- source 511, local [1, 7]
  ⟨15, 23, 16, 4, 33, -32⟩, -- source 511, local [2, 0]
  ⟨15, 23, 16, 4, 26, 24⟩, -- source 511, local [2, 1]
  ⟨15, 23, 16, 4, 26, 10⟩, -- source 511, local [2, 2]
  ⟨15, 23, 16, 4, -2, -11⟩, -- source 511, local [2, 3]
  ⟨15, 23, 16, 4, -2, -25⟩, -- source 511, local [2, 4]
  ⟨15, 23, 16, 4, -9, 31⟩, -- source 511, local [2, 5]
  ⟨15, 23, 16, 4, -23, 24⟩, -- source 511, local [2, 6]
  ⟨15, 23, 16, 4, -30, -4⟩, -- source 511, local [2, 7]
  ⟨15, 23, -16, -4, -33, -32⟩, -- source 553, local [0, 0]
  ⟨15, 23, -16, -4, 30, -4⟩, -- source 553, local [0, 1]
  ⟨15, 23, -16, -4, 23, 24⟩, -- source 553, local [0, 2]
  ⟨15, 23, -16, -4, 9, 31⟩, -- source 553, local [0, 3]
  ⟨15, 23, -16, -4, 2, -11⟩, -- source 553, local [0, 4]
  ⟨15, 23, -16, -4, 2, -25⟩, -- source 553, local [0, 5]
  ⟨15, 23, -16, -4, -26, 24⟩, -- source 553, local [0, 6]
  ⟨15, 23, -16, -4, -26, 10⟩, -- source 553, local [0, 7]
  ⟨15, 23, -16, -4, 33, 23⟩, -- source 553, local [1, 0]
  ⟨15, 23, -16, -4, 19, -26⟩, -- source 553, local [1, 1]
  ⟨15, 23, -16, -4, 12, 2⟩, -- source 553, local [1, 2]
  ⟨15, 23, -16, -4, -2, 9⟩, -- source 553, local [1, 3]
  ⟨15, 23, -16, -4, -9, -33⟩, -- source 553, local [1, 4]
  ⟨15, 23, -16, -4, -9, 30⟩, -- source 553, local [1, 5]
  ⟨15, 23, -16, -4, -37, 2⟩, -- source 553, local [1, 6]
  ⟨15, 23, -16, -4, -37, -12⟩, -- source 553, local [1, 7]
  ⟨15, 23, -16, -4, 22, 12⟩, -- source 553, local [2, 0]
  ⟨15, 23, -16, -4, 8, -37⟩, -- source 553, local [2, 1]
  ⟨15, 23, -16, -4, 1, -9⟩, -- source 553, local [2, 2]
  ⟨15, 23, -16, -4, -13, -2⟩, -- source 553, local [2, 3]
  ⟨15, 23, -16, -4, -20, 33⟩, -- source 553, local [2, 4]
  ⟨15, 23, -16, -4, -20, 19⟩, -- source 553, local [2, 5]
  ⟨15, 23, -16, -4, 29, -9⟩, -- source 553, local [2, 6]
  ⟨15, 23, -16, -4, 29, -23⟩, -- source 553, local [2, 7]
]

def admissible (r : LocalRow) : Prop :=
  residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 = 0 ∨
  residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 % (47*77) ≠ 0
theorem exhaustive : ∀ r ∈ localTable, admissible r := by
  dsimp only [admissible]
  decide
theorem localTable_length : localTable.length = 144 := by decide
theorem centered_zero_count :
    (localTable.filter (fun r => decide (residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 = 0))).length = 8 := by
  decide
theorem noncentered_shift_excluded (r : LocalRow) (hr : r ∈ localTable)
    (hnz : residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 ≠ 0) :
    residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 % (47*77) ≠ 0 := by
  rcases exhaustive r hr with hzero | hmod
  · exact False.elim (hnz hzero)
  · exact hmod
#print axioms exhaustive
#print axioms centered_zero_count
#print axioms noncentered_shift_excluded

end XedniSixPairShift
