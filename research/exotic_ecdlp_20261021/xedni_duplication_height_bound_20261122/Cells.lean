import Bounds

namespace XedniDuplicationCells

def F_real (t : ℝ) : ℝ := t ^ 4 - 1502 * t ^ 2 + 31976 * t - 187435
def G_real (t : ℝ) : ℝ := 4 * t ^ 3 - 188 * t ^ 2 + 3004 * t - 15988

private theorem from_cell_F (t K N : ℝ) (hN : max 1 |t| ≤ N)
    (hK : N ^ 4 / 1024 = K) (hF : K ≤ F_real t) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  calc
    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr
    _ = K := hK
    _ ≤ |F_real t| := hF.trans (le_abs_self _)
    _ ≤ max |F_real t| |G_real t| := le_max_left _ _

private theorem from_cell_negF (t K N : ℝ) (hN : max 1 |t| ≤ N)
    (hK : N ^ 4 / 1024 = K) (hF : K ≤ -F_real t) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  calc
    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr
    _ = K := hK
    _ ≤ |F_real t| := hF.trans (neg_le_abs _)
    _ ≤ max |F_real t| |G_real t| := le_max_left _ _

private theorem from_cell_G (t K N : ℝ) (hN : max 1 |t| ≤ N)
    (hK : N ^ 4 / 1024 = K) (hG : K ≤ G_real t) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  calc
    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr
    _ = K := hK
    _ ≤ |G_real t| := hG.trans (le_abs_self _)
    _ ≤ max |F_real t| |G_real t| := le_max_right _ _

private theorem from_cell_negG (t K N : ℝ) (hN : max 1 |t| ≤ N)
    (hK : N ^ 4 / 1024 = K) (hG : K ≤ -G_real t) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  calc
    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr
    _ = K := hK
    _ ≤ |G_real t| := hG.trans (neg_le_abs _)
    _ ≤ max |F_real t| |G_real t| := le_max_right _ _

private theorem cell_0 (t : ℝ)
    (hl : (-128 : ℝ) ≤ t) (hu : t ≤ (-64 : ℝ)) :
    (262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 64 : ℝ) * t + (2 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (239546325 : ℝ) (111992405 : ℝ) (151565951 / 3 : ℝ) (21580629 : ℝ) (8391125 : ℝ)
    (262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_0_global (t : ℝ)
    (hl : (-128 : ℝ) ≤ t) (hu : t ≤ (-64 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (128 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (128 : ℝ) ^ 4 / 1024 = (262144 : ℝ) := by norm_num
  exact from_cell_F t (262144 : ℝ) (128 : ℝ) hN hK (cell_0 t hl hu)

private theorem cell_1 (t : ℝ)
    (hl : (-64 : ℝ) ≤ t) (hu : t ≤ (-32 : ℝ)) :
    (16384 : ℝ) ≤ -G_real t := by
  let s : ℝ := (1 / 32 : ℝ) * t + (2 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2026868 : ℝ) (1417108 : ℝ) (2911516 / 3 : ℝ) (654292 : ℝ) (435700 : ℝ)
    (16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_1_global (t : ℝ)
    (hl : (-64 : ℝ) ≤ t) (hu : t ≤ (-32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (64 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (64 : ℝ) ^ 4 / 1024 = (16384 : ℝ) := by norm_num
  exact from_cell_negG t (16384 : ℝ) (64 : ℝ) hN hK (cell_1 t hl hu)

private theorem cell_2 (t : ℝ)
    (hl : (-32 : ℝ) ≤ t) (hu : t ≤ (0 : ℝ)) :
    (1024 : ℝ) ≤ -F_real t := by
  let s : ℝ := (1 / 32 : ℝ) * t + (1 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1700139 : ℝ) (1723883 : ℝ) (2866177 / 3 : ℝ) (443243 : ℝ) (187435 : ℝ)
    (1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_2_global (t : ℝ)
    (hl : (-32 : ℝ) ≤ t) (hu : t ≤ (0 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (32 : ℝ) ^ 4 / 1024 = (1024 : ℝ) := by norm_num
  exact from_cell_negF t (1024 : ℝ) (32 : ℝ) hN hK (cell_2 t hl hu)

private theorem cell_3 (t : ℝ)
    (hl : (0 : ℝ) ≤ t) (hu : t ≤ (4 : ℝ)) :
    (1 / 4 : ℝ) ≤ -F_real t := by
  let s : ℝ := (1 / 4 : ℝ) * t + (0 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (187435 : ℝ) (155459 : ℝ) (382465 / 3 : ℝ) (103523 : ℝ) (83307 : ℝ)
    (1 / 4 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_3_global (t : ℝ)
    (hl : (0 : ℝ) ≤ t) (hu : t ≤ (4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (4 : ℝ) ^ 4 / 1024 = (1 / 4 : ℝ) := by norm_num
  exact from_cell_negF t (1 / 4 : ℝ) (4 : ℝ) hN hK (cell_3 t hl hu)

private theorem cell_4 (t : ℝ)
    (hl : (4 : ℝ) ≤ t) (hu : t ≤ (6 : ℝ)) :
    (81 / 64 : ℝ) ≤ -F_real t := by
  let s : ℝ := (1 / 2 : ℝ) * t + (-2 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (83307 : ℝ) (73199 : ℝ) (192085 / 3 : ℝ) (55763 : ℝ) (48355 : ℝ)
    (81 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_4_global (t : ℝ)
    (hl : (4 : ℝ) ≤ t) (hu : t ≤ (6 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (6 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (6 : ℝ) ^ 4 / 1024 = (81 / 64 : ℝ) := by norm_num
  exact from_cell_negF t (81 / 64 : ℝ) (6 : ℝ) hN hK (cell_4 t hl hu)

private theorem cell_5 (t : ℝ)
    (hl : (6 : ℝ) ≤ t) (hu : t ≤ (8 : ℝ)) :
    (4 : ℝ) ≤ -F_real t := by
  let s : ℝ := (1 / 2 : ℝ) * t + (-3 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (48355 : ℝ) (40947 : ℝ) (103189 / 3 : ℝ) (28655 : ℝ) (23659 : ℝ)
    (4 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_5_global (t : ℝ)
    (hl : (6 : ℝ) ≤ t) (hu : t ≤ (8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (8 : ℝ) ^ 4 / 1024 = (4 : ℝ) := by norm_num
  exact from_cell_negF t (4 : ℝ) (8 : ℝ) hN hK (cell_5 t hl hu)

private theorem cell_6 (t : ℝ)
    (hl : (8 : ℝ) ≤ t) (hu : t ≤ (9 : ℝ)) :
    (6561 / 1024 : ℝ) ≤ -F_real t := by
  let s : ℝ := (1 : ℝ) * t + (-8 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (23659 : ℝ) (21161 : ℝ) (56548 / 3 : ℝ) (16716 : ℝ) (14752 : ℝ)
    (6561 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_6_global (t : ℝ)
    (hl : (8 : ℝ) ≤ t) (hu : t ≤ (9 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (9 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (9 : ℝ) ^ 4 / 1024 = (6561 / 1024 : ℝ) := by norm_num
  exact from_cell_negF t (6561 / 1024 : ℝ) (9 : ℝ) hN hK (cell_6 t hl hu)

private theorem cell_7 (t : ℝ)
    (hl : (9 : ℝ) ≤ t) (hu : t ≤ (19 / 2 : ℝ)) :
    (130321 / 16384 : ℝ) ≤ -F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-18 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (14752 : ℝ) (13770 : ℝ) (38491 / 3 : ℝ) (95455 / 8 : ℝ) (177175 / 16 : ℝ)
    (130321 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_7_global (t : ℝ)
    (hl : (9 : ℝ) ≤ t) (hu : t ≤ (19 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (19 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (19 / 2 : ℝ) ^ 4 / 1024 = (130321 / 16384 : ℝ) := by norm_num
  exact from_cell_negF t (130321 / 16384 : ℝ) (19 / 2 : ℝ) hN hK (cell_7 t hl hu)

private theorem cell_8 (t : ℝ)
    (hl : (19 / 2 : ℝ) ≤ t) (hu : t ≤ (10 : ℝ)) :
    (625 / 64 : ℝ) ≤ -F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-19 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (177175 / 16 : ℝ) (10215 : ℝ) (112759 / 12 : ℝ) (8617 : ℝ) (7875 : ℝ)
    (625 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_8_global (t : ℝ)
    (hl : (19 / 2 : ℝ) ≤ t) (hu : t ≤ (10 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (10 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (10 : ℝ) ^ 4 / 1024 = (625 / 64 : ℝ) := by norm_num
  exact from_cell_negF t (625 / 64 : ℝ) (10 : ℝ) hN hK (cell_8 t hl hu)

private theorem cell_9 (t : ℝ)
    (hl : (10 : ℝ) ≤ t) (hu : t ≤ (41 / 4 : ℝ)) :
    (2825761 / 262144 : ℝ) ≤ -F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-40 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (7875 : ℝ) (7504 : ℝ) (342835 / 48 : ℝ) (217281 / 32 : ℝ) (1650367 / 256 : ℝ)
    (2825761 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_9_global (t : ℝ)
    (hl : (10 : ℝ) ≤ t) (hu : t ≤ (41 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (41 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (41 / 4 : ℝ) ^ 4 / 1024 = (2825761 / 262144 : ℝ) := by norm_num
  exact from_cell_negF t (2825761 / 262144 : ℝ) (41 / 4 : ℝ) hN hK (cell_9 t hl hu)

private theorem cell_10 (t : ℝ)
    (hl : (41 / 4 : ℝ) ≤ t) (hu : t ≤ (21 / 2 : ℝ)) :
    (194481 / 16384 : ℝ) ≤ -F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-41 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1650367 / 256 : ℝ) (781243 / 128 : ℝ) (1107697 / 192 : ℝ) (174207 / 32 : ℝ) (82039 / 16 : ℝ)
    (194481 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_10_global (t : ℝ)
    (hl : (41 / 4 : ℝ) ≤ t) (hu : t ≤ (21 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (21 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (21 / 2 : ℝ) ^ 4 / 1024 = (194481 / 16384 : ℝ) := by norm_num
  exact from_cell_negF t (194481 / 16384 : ℝ) (21 / 2 : ℝ) hN hK (cell_10 t hl hu)

private theorem cell_11 (t : ℝ)
    (hl : (21 / 2 : ℝ) ≤ t) (hu : t ≤ (43 / 4 : ℝ)) :
    (3418801 / 262144 : ℝ) ≤ -F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-42 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (82039 / 16 : ℝ) (153949 / 32 : ℝ) (864601 / 192 : ℝ) (538105 / 128 : ℝ) (1001775 / 256 : ℝ)
    (3418801 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_11_global (t : ℝ)
    (hl : (21 / 2 : ℝ) ≤ t) (hu : t ≤ (43 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (43 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (43 / 4 : ℝ) ^ 4 / 1024 = (3418801 / 262144 : ℝ) := by norm_num
  exact from_cell_negF t (3418801 / 262144 : ℝ) (43 / 4 : ℝ) hN hK (cell_11 t hl hu)

private theorem cell_12 (t : ℝ)
    (hl : (43 / 4 : ℝ) ≤ t) (hu : t ≤ (87 / 8 : ℝ)) :
    (57289761 / 4194304 : ℝ) ≤ -F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-86 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1001775 / 256 : ℝ) (1929115 / 512 : ℝ) (11134549 / 3072 : ℝ) (7133875 / 2048 : ℝ) (13697887 / 4096 : ℝ)
    (57289761 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_12_global (t : ℝ)
    (hl : (43 / 4 : ℝ) ≤ t) (hu : t ≤ (87 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (87 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (87 / 8 : ℝ) ^ 4 / 1024 = (57289761 / 4194304 : ℝ) := by norm_num
  exact from_cell_negF t (57289761 / 4194304 : ℝ) (87 / 8 : ℝ) hN hK (cell_12 t hl hu)

private theorem cell_13 (t : ℝ)
    (hl : (87 / 8 : ℝ) ≤ t) (hu : t ≤ (11 : ℝ)) :
    (14641 / 1024 : ℝ) ≤ -F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-87 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (13697887 / 4096 : ℝ) (1641003 / 512 : ℝ) (147265 / 48 : ℝ) (2933 : ℝ) (2800 : ℝ)
    (14641 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_13_global (t : ℝ)
    (hl : (87 / 8 : ℝ) ≤ t) (hu : t ≤ (11 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (11 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (11 : ℝ) ^ 4 / 1024 = (14641 / 1024 : ℝ) := by norm_num
  exact from_cell_negF t (14641 / 1024 : ℝ) (11 : ℝ) hN hK (cell_13 t hl hu)

private theorem cell_14 (t : ℝ)
    (hl : (11 : ℝ) ≤ t) (hu : t ≤ (89 / 8 : ℝ)) :
    (62742241 / 4194304 : ℝ) ≤ -F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-88 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2800 : ℝ) (2667 : ℝ) (121729 / 48 : ℝ) (1232405 / 512 : ℝ) (9339039 / 4096 : ℝ)
    (62742241 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_14_global (t : ℝ)
    (hl : (11 : ℝ) ≤ t) (hu : t ≤ (89 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (89 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (89 / 8 : ℝ) ^ 4 / 1024 = (62742241 / 4194304 : ℝ) := by norm_num
  exact from_cell_negF t (62742241 / 4194304 : ℝ) (89 / 8 : ℝ) hN hK (cell_14 t hl hu)

private theorem cell_15 (t : ℝ)
    (hl : (89 / 8 : ℝ) ≤ t) (hu : t ≤ (45 / 4 : ℝ)) :
    (4100625 / 262144 : ℝ) ≤ -G_real t := by
  let s : ℝ := (8 : ℝ) * t + (-89 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (42095 / 128 : ℝ) (163481 / 512 : ℝ) (118991 / 384 : ℝ) (38475 / 128 : ℝ) (4663 / 16 : ℝ)
    (4100625 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_15_global (t : ℝ)
    (hl : (89 / 8 : ℝ) ≤ t) (hu : t ≤ (45 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (45 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (45 / 4 : ℝ) ^ 4 / 1024 = (4100625 / 262144 : ℝ) := by norm_num
  exact from_cell_negG t (4100625 / 262144 : ℝ) (45 / 4 : ℝ) hN hK (cell_15 t hl hu)

private theorem cell_16 (t : ℝ)
    (hl : (45 / 4 : ℝ) ≤ t) (hu : t ≤ (91 / 8 : ℝ)) :
    (68574961 / 4194304 : ℝ) ≤ -G_real t := by
  let s : ℝ := (8 : ℝ) * t + (-90 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (4663 / 16 : ℝ) (36133 / 128 : ℝ) (104939 / 384 : ℝ) (135375 / 512 : ℝ) (32725 / 128 : ℝ)
    (68574961 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_16_global (t : ℝ)
    (hl : (45 / 4 : ℝ) ≤ t) (hu : t ≤ (91 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (91 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (91 / 8 : ℝ) ^ 4 / 1024 = (68574961 / 4194304 : ℝ) := by norm_num
  exact from_cell_negG t (68574961 / 4194304 : ℝ) (91 / 8 : ℝ) hN hK (cell_16 t hl hu)

private theorem cell_17 (t : ℝ)
    (hl : (91 / 8 : ℝ) ≤ t) (hu : t ≤ (23 / 2 : ℝ)) :
    (279841 / 16384 : ℝ) ≤ -G_real t := by
  let s : ℝ := (8 : ℝ) * t + (-91 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (32725 / 128 : ℝ) (126425 / 512 : ℝ) (45757 / 192 : ℝ) (7355 / 32 : ℝ) (443 / 2 : ℝ)
    (279841 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_17_global (t : ℝ)
    (hl : (91 / 8 : ℝ) ≤ t) (hu : t ≤ (23 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (23 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (23 / 2 : ℝ) ^ 4 / 1024 = (279841 / 16384 : ℝ) := by norm_num
  exact from_cell_negG t (279841 / 16384 : ℝ) (23 / 2 : ℝ) hN hK (cell_17 t hl hu)

private theorem cell_18 (t : ℝ)
    (hl : (23 / 2 : ℝ) ≤ t) (hu : t ≤ (185 / 16 : ℝ)) :
    (1171350625 / 67108864 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-184 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (443 / 2 : ℝ) (13909 / 64 : ℝ) (163729 / 768 : ℝ) (856399 / 4096 : ℝ) (209927 / 1024 : ℝ)
    (1171350625 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_18_global (t : ℝ)
    (hl : (23 / 2 : ℝ) ≤ t) (hu : t ≤ (185 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (185 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (185 / 16 : ℝ) ^ 4 / 1024 = (1171350625 / 67108864 : ℝ) := by norm_num
  exact from_cell_negG t (1171350625 / 67108864 : ℝ) (185 / 16 : ℝ) hN hK (cell_18 t hl hu)

private theorem cell_19 (t : ℝ)
    (hl : (185 / 16 : ℝ) ≤ t) (hu : t ≤ (93 / 8 : ℝ)) :
    (74805201 / 4194304 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-185 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (209927 / 1024 : ℝ) (823017 / 4096 : ℝ) (604843 / 3072 : ℝ) (197507 / 1024 : ℝ) (24179 / 128 : ℝ)
    (74805201 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_19_global (t : ℝ)
    (hl : (185 / 16 : ℝ) ≤ t) (hu : t ≤ (93 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (93 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (93 / 8 : ℝ) ^ 4 / 1024 = (74805201 / 4194304 : ℝ) := by norm_num
  exact from_cell_negG t (74805201 / 4194304 : ℝ) (93 / 8 : ℝ) hN hK (cell_19 t hl hu)

private theorem cell_20 (t : ℝ)
    (hl : (93 / 8 : ℝ) ≤ t) (hu : t ≤ (187 / 16 : ℝ)) :
    (1222830961 / 67108864 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-186 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (24179 / 128 : ℝ) (189357 / 1024 : ℝ) (555943 / 3072 : ℝ) (725215 / 4096 : ℝ) (177325 / 1024 : ℝ)
    (1222830961 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_20_global (t : ℝ)
    (hl : (93 / 8 : ℝ) ≤ t) (hu : t ≤ (187 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (187 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (187 / 16 : ℝ) ^ 4 / 1024 = (1222830961 / 67108864 : ℝ) := by norm_num
  exact from_cell_negG t (1222830961 / 67108864 : ℝ) (187 / 16 : ℝ) hN hK (cell_20 t hl hu)

private theorem cell_21 (t : ℝ)
    (hl : (187 / 16 : ℝ) ≤ t) (hu : t ≤ (47 / 4 : ℝ)) :
    (4879681 / 262144 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-187 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (177325 / 1024 : ℝ) (693385 / 4096 : ℝ) (254099 / 1536 : ℝ) (41371 / 256 : ℝ) (2525 / 16 : ℝ)
    (4879681 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_21_global (t : ℝ)
    (hl : (187 / 16 : ℝ) ≤ t) (hu : t ≤ (47 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (47 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (47 / 4 : ℝ) ^ 4 / 1024 = (4879681 / 262144 : ℝ) := by norm_num
  exact from_cell_negG t (4879681 / 262144 : ℝ) (47 / 4 : ℝ) hN hK (cell_21 t hl hu)

private theorem cell_22 (t : ℝ)
    (hl : (47 / 4 : ℝ) ≤ t) (hu : t ≤ (189 / 16 : ℝ)) :
    (1275989841 / 67108864 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-188 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2525 / 16 : ℝ) (39429 / 256 : ℝ) (230795 / 1536 : ℝ) (600167 / 4096 : ℝ) (146251 / 1024 : ℝ)
    (1275989841 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_22_global (t : ℝ)
    (hl : (47 / 4 : ℝ) ≤ t) (hu : t ≤ (189 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (189 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (189 / 16 : ℝ) ^ 4 / 1024 = (1275989841 / 67108864 : ℝ) := by norm_num
  exact from_cell_negG t (1275989841 / 67108864 : ℝ) (189 / 16 : ℝ) hN hK (cell_22 t hl hu)

private theorem cell_23 (t : ℝ)
    (hl : (189 / 16 : ℝ) ≤ t) (hu : t ≤ (95 / 8 : ℝ)) :
    (81450625 / 4194304 : ℝ) ≤ -G_real t := by
  let s : ℝ := (16 : ℝ) * t + (-189 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (146251 / 1024 : ℝ) (569841 / 4096 : ℝ) (416101 / 3072 : ℝ) (134971 / 1024 : ℝ) (16409 / 128 : ℝ)
    (81450625 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_23_global (t : ℝ)
    (hl : (189 / 16 : ℝ) ≤ t) (hu : t ≤ (95 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (95 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (95 / 8 : ℝ) ^ 4 / 1024 = (81450625 / 4194304 : ℝ) := by norm_num
  exact from_cell_negG t (81450625 / 4194304 : ℝ) (95 / 8 : ℝ) hN hK (cell_23 t hl hu)

private theorem cell_24 (t : ℝ)
    (hl : (95 / 8 : ℝ) ≤ t) (hu : t ≤ (381 / 32 : ℝ)) :
    (21071715921 / 1073741824 : ℝ) ≤ -G_real t := by
  let s : ℝ := (32 : ℝ) * t + (-380 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (16409 / 128 : ℝ) (258845 / 2048 : ℝ) (1530967 / 12288 : ℝ) (4023879 / 32768 : ℝ) (991355 / 8192 : ℝ)
    (21071715921 / 1073741824 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_24_global (t : ℝ)
    (hl : (95 / 8 : ℝ) ≤ t) (hu : t ≤ (381 / 32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (381 / 32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (381 / 32 : ℝ) ^ 4 / 1024 = (21071715921 / 1073741824 : ℝ) := by norm_num
  exact from_cell_negG t (21071715921 / 1073741824 : ℝ) (381 / 32 : ℝ) hN hK (cell_24 t hl hu)

private theorem cell_25 (t : ℝ)
    (hl : (381 / 32 : ℝ) ≤ t) (hu : t ≤ (191 / 16 : ℝ)) :
    (1330863361 / 67108864 : ℝ) ≤ -G_real t := by
  let s : ℝ := (32 : ℝ) * t + (-381 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (991355 / 8192 : ℝ) (3906961 / 32768 : ℝ) (2886557 / 24576 : ℝ) (947691 / 8192 : ℝ) (116657 / 1024 : ℝ)
    (1330863361 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, G_real] <;> ring

private theorem cell_25_global (t : ℝ)
    (hl : (381 / 32 : ℝ) ≤ t) (hu : t ≤ (191 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (191 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (191 / 16 : ℝ) ^ 4 / 1024 = (1330863361 / 67108864 : ℝ) := by norm_num
  exact from_cell_negG t (1330863361 / 67108864 : ℝ) (191 / 16 : ℝ) hN hK (cell_25 t hl hu)

private theorem cell_26 (t : ℝ)
    (hl : (191 / 16 : ℝ) ≤ t) (hu : t ≤ (383 / 32 : ℝ)) :
    (21517662721 / 1073741824 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-382 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (35716865 / 65536 : ℝ) (74424129 / 131072 : ℝ) (464404355 / 786432 : ℝ) (321454273 / 524288 : ℝ) (666501633 / 1048576 : ℝ)
    (21517662721 / 1073741824 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_26_global (t : ℝ)
    (hl : (191 / 16 : ℝ) ≤ t) (hu : t ≤ (383 / 32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (383 / 32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (383 / 32 : ℝ) ^ 4 / 1024 = (21517662721 / 1073741824 : ℝ) := by norm_num
  exact from_cell_F t (21517662721 / 1073741824 : ℝ) (383 / 32 : ℝ) hN hK (cell_26 t hl hu)

private theorem cell_27 (t : ℝ)
    (hl : (383 / 32 : ℝ) ≤ t) (hu : t ≤ (12 : ℝ)) :
    (81 / 4 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-383 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (666501633 / 1048576 : ℝ) (5391365 / 8192 : ℝ) (2090561 / 3072 : ℝ) (11245 / 16 : ℝ) (725 : ℝ)
    (81 / 4 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_27_global (t : ℝ)
    (hl : (383 / 32 : ℝ) ≤ t) (hu : t ≤ (12 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (12 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (12 : ℝ) ^ 4 / 1024 = (81 / 4 : ℝ) := by norm_num
  exact from_cell_F t (81 / 4 : ℝ) (12 : ℝ) hN hK (cell_27 t hl hu)

private theorem cell_28 (t : ℝ)
    (hl : (12 : ℝ) ≤ t) (hu : t ≤ (385 / 32 : ℝ)) :
    (21970650625 / 1073741824 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-384 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (725 : ℝ) (11955 / 16 : ℝ) (2363201 / 3072 : ℝ) (6481931 / 8192 : ℝ) (852626945 / 1048576 : ℝ)
    (21970650625 / 1073741824 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_28_global (t : ℝ)
    (hl : (12 : ℝ) ≤ t) (hu : t ≤ (385 / 32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (385 / 32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (385 / 32 : ℝ) ^ 4 / 1024 = (21970650625 / 1073741824 : ℝ) := by norm_num
  exact from_cell_F t (21970650625 / 1073741824 : ℝ) (385 / 32 : ℝ) hN hK (cell_28 t hl hu)

private theorem cell_29 (t : ℝ)
    (hl : (385 / 32 : ℝ) ≤ t) (hu : t ≤ (193 / 16 : ℝ)) :
    (1387488001 / 67108864 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-385 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (852626945 / 1048576 : ℝ) (437783361 / 524288 : ℝ) (673798787 / 786432 : ℝ) (115140289 / 131072 : ℝ) (58983681 / 65536 : ℝ)
    (1387488001 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_29_global (t : ℝ)
    (hl : (385 / 32 : ℝ) ≤ t) (hu : t ≤ (193 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (193 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (193 / 16 : ℝ) ^ 4 / 1024 = (1387488001 / 67108864 : ℝ) := by norm_num
  exact from_cell_F t (1387488001 / 67108864 : ℝ) (193 / 16 : ℝ) hN hK (cell_29 t hl hu)

private theorem cell_30 (t : ℝ)
    (hl : (193 / 16 : ℝ) ≤ t) (hu : t ≤ (387 / 32 : ℝ)) :
    (22430753361 / 1073741824 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-386 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (58983681 / 65536 : ℝ) (120794435 / 131072 : ℝ) (741648539 / 786432 : ℝ) (505633499 / 524288 : ℝ) (1033562705 / 1048576 : ℝ)
    (22430753361 / 1073741824 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_30_global (t : ℝ)
    (hl : (193 / 16 : ℝ) ≤ t) (hu : t ≤ (387 / 32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (387 / 32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (387 / 32 : ℝ) ^ 4 / 1024 = (22430753361 / 1073741824 : ℝ) := by norm_num
  exact from_cell_F t (22430753361 / 1073741824 : ℝ) (387 / 32 : ℝ) hN hK (cell_30 t hl hu)

private theorem cell_31 (t : ℝ)
    (hl : (387 / 32 : ℝ) ≤ t) (hu : t ≤ (97 / 8 : ℝ)) :
    (88529281 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (32 : ℝ) * t + (-387 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1033562705 / 1048576 : ℝ) (263964603 / 262144 : ℝ) (202133915 / 196608 : ℝ) (17189539 / 16384 : ℝ) (4383233 / 4096 : ℝ)
    (88529281 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_31_global (t : ℝ)
    (hl : (387 / 32 : ℝ) ≤ t) (hu : t ≤ (97 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (97 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (97 / 8 : ℝ) ^ 4 / 1024 = (88529281 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (88529281 / 4194304 : ℝ) (97 / 8 : ℝ) hN hK (cell_31 t hl hu)

private theorem cell_32 (t : ℝ)
    (hl : (97 / 8 : ℝ) ≤ t) (hu : t ≤ (195 / 16 : ℝ)) :
    (1445900625 / 67108864 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-194 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (4383233 / 4096 : ℝ) (9109859 / 8192 : ℝ) (56699675 / 49152 : ℝ) (39147003 / 32768 : ℝ) (80962385 / 65536 : ℝ)
    (1445900625 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_32_global (t : ℝ)
    (hl : (97 / 8 : ℝ) ≤ t) (hu : t ≤ (195 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (195 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (195 / 16 : ℝ) ^ 4 / 1024 = (1445900625 / 67108864 : ℝ) := by norm_num
  exact from_cell_F t (1445900625 / 67108864 : ℝ) (195 / 16 : ℝ) hN hK (cell_32 t hl hu)

private theorem cell_33 (t : ℝ)
    (hl : (195 / 16 : ℝ) ≤ t) (hu : t ≤ (49 / 4 : ℝ)) :
    (5764801 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-195 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (80962385 / 65536 : ℝ) (20907691 / 16384 : ℝ) (16176203 / 12288 : ℝ) (1388899 / 1024 : ℝ) (357345 / 256 : ℝ)
    (5764801 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_33_global (t : ℝ)
    (hl : (195 / 16 : ℝ) ≤ t) (hu : t ≤ (49 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (49 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (49 / 4 : ℝ) ^ 4 / 1024 = (5764801 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (5764801 / 262144 : ℝ) (49 / 4 : ℝ) hN hK (cell_33 t hl hu)

private theorem cell_34 (t : ℝ)
    (hl : (49 / 4 : ℝ) ≤ t) (hu : t ≤ (197 / 16 : ℝ)) :
    (1506138481 / 67108864 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-196 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (357345 / 256 : ℝ) (1469861 / 1024 : ℝ) (18119291 / 12288 : ℝ) (24793965 / 16384 : ℝ) (101690225 / 65536 : ℝ)
    (1506138481 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_34_global (t : ℝ)
    (hl : (49 / 4 : ℝ) ≤ t) (hu : t ≤ (197 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (197 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (197 / 16 : ℝ) ^ 4 / 1024 = (1506138481 / 67108864 : ℝ) := by norm_num
  exact from_cell_F t (1506138481 / 67108864 : ℝ) (197 / 16 : ℝ) hN hK (cell_34 t hl hu)

private theorem cell_35 (t : ℝ)
    (hl : (197 / 16 : ℝ) ≤ t) (hu : t ≤ (99 / 8 : ℝ)) :
    (96059601 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-197 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (101690225 / 65536 : ℝ) (52102295 / 32768 : ℝ) (80020259 / 49152 : ℝ) (13644711 / 8192 : ℝ) (6974801 / 4096 : ℝ)
    (96059601 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_35_global (t : ℝ)
    (hl : (197 / 16 : ℝ) ≤ t) (hu : t ≤ (99 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (99 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (99 / 8 : ℝ) ^ 4 / 1024 = (96059601 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (96059601 / 4194304 : ℝ) (99 / 8 : ℝ) hN hK (cell_35 t hl hu)

private theorem cell_36 (t : ℝ)
    (hl : (99 / 8 : ℝ) ≤ t) (hu : t ≤ (199 / 16 : ℝ)) :
    (1568239201 / 67108864 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-198 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (6974801 / 4096 : ℝ) (14254493 / 8192 : ℝ) (87337643 / 49152 : ℝ) (59419877 / 32768 : ℝ) (121204833 / 65536 : ℝ)
    (1568239201 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_36_global (t : ℝ)
    (hl : (99 / 8 : ℝ) ≤ t) (hu : t ≤ (199 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (199 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (199 / 16 : ℝ) ^ 4 / 1024 = (1568239201 / 67108864 : ℝ) := by norm_num
  exact from_cell_F t (1568239201 / 67108864 : ℝ) (199 / 16 : ℝ) hN hK (cell_36 t hl hu)

private theorem cell_37 (t : ℝ)
    (hl : (199 / 16 : ℝ) ≤ t) (hu : t ≤ (25 / 2 : ℝ)) :
    (390625 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-199 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (121204833 / 65536 : ℝ) (15446239 / 8192 : ℝ) (5902055 / 3072 : ℝ) (250443 / 128 : ℝ) (31865 / 16 : ℝ)
    (390625 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_37_global (t : ℝ)
    (hl : (199 / 16 : ℝ) ≤ t) (hu : t ≤ (25 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (25 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (25 / 2 : ℝ) ^ 4 / 1024 = (390625 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (390625 / 16384 : ℝ) (25 / 2 : ℝ) hN hK (cell_37 t hl hu)

private theorem cell_38 (t : ℝ)
    (hl : (25 / 2 : ℝ) ≤ t) (hu : t ≤ (201 / 16 : ℝ)) :
    (1632240801 / 67108864 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-200 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (31865 / 16 : ℝ) (259397 / 128 : ℝ) (6331847 / 3072 : ℝ) (17165457 / 8192 : ℝ) (139544225 / 65536 : ℝ)
    (1632240801 / 67108864 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_38_global (t : ℝ)
    (hl : (25 / 2 : ℝ) ≤ t) (hu : t ≤ (201 / 16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (201 / 16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (201 / 16 : ℝ) ^ 4 / 1024 = (1632240801 / 67108864 : ℝ) := by norm_num
  exact from_cell_F t (1632240801 / 67108864 : ℝ) (201 / 16 : ℝ) hN hK (cell_38 t hl hu)

private theorem cell_39 (t : ℝ)
    (hl : (201 / 16 : ℝ) ≤ t) (hu : t ≤ (101 / 8 : ℝ)) :
    (104060401 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (16 : ℝ) * t + (-201 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (139544225 / 65536 : ℝ) (70882397 / 32768 : ℝ) (107971259 / 49152 : ℝ) (18266885 / 8192 : ℝ) (9267825 / 4096 : ℝ)
    (104060401 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_39_global (t : ℝ)
    (hl : (201 / 16 : ℝ) ≤ t) (hu : t ≤ (101 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (101 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (101 / 8 : ℝ) ^ 4 / 1024 = (104060401 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (104060401 / 4194304 : ℝ) (101 / 8 : ℝ) hN hK (cell_39 t hl hu)

private theorem cell_40 (t : ℝ)
    (hl : (101 / 8 : ℝ) ≤ t) (hu : t ≤ (51 / 4 : ℝ)) :
    (6765201 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-101 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (9267825 / 4096 : ℝ) (4768295 / 2048 : ℝ) (7349651 / 3072 : ℝ) (1257095 / 512 : ℝ) (644273 / 256 : ℝ)
    (6765201 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_40_global (t : ℝ)
    (hl : (101 / 8 : ℝ) ≤ t) (hu : t ≤ (51 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (51 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (51 / 4 : ℝ) ^ 4 / 1024 = (6765201 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (6765201 / 262144 : ℝ) (51 / 4 : ℝ) hN hK (cell_40 t hl hu)

private theorem cell_41 (t : ℝ)
    (hl : (51 / 4 : ℝ) ≤ t) (hu : t ≤ (103 / 8 : ℝ)) :
    (112550881 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-102 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (644273 / 256 : ℝ) (1319997 / 512 : ℝ) (8104475 / 3072 : ℝ) (5523221 / 2048 : ℝ) (11281505 / 4096 : ℝ)
    (112550881 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_41_global (t : ℝ)
    (hl : (51 / 4 : ℝ) ≤ t) (hu : t ≤ (103 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (103 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (103 / 8 : ℝ) ^ 4 / 1024 = (112550881 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (112550881 / 4194304 : ℝ) (103 / 8 : ℝ) hN hK (cell_41 t hl hu)

private theorem cell_42 (t : ℝ)
    (hl : (103 / 8 : ℝ) ≤ t) (hu : t ≤ (13 : ℝ)) :
    (28561 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-103 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (11281505 / 4096 : ℝ) (1439571 / 512 : ℝ) (137651 / 48 : ℝ) (5845 / 2 : ℝ) (2976 : ℝ)
    (28561 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_42_global (t : ℝ)
    (hl : (103 / 8 : ℝ) ≤ t) (hu : t ≤ (13 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (13 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (13 : ℝ) ^ 4 / 1024 = (28561 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (28561 / 1024 : ℝ) (13 : ℝ) hN hK (cell_42 t hl hu)

private theorem cell_43 (t : ℝ)
    (hl : (13 : ℝ) ≤ t) (hu : t ≤ (105 / 8 : ℝ)) :
    (121550625 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-104 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2976 : ℝ) (6059 / 2 : ℝ) (147923 / 48 : ℝ) (1603949 / 512 : ℝ) (13035425 / 4096 : ℝ)
    (121550625 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_43_global (t : ℝ)
    (hl : (13 : ℝ) ≤ t) (hu : t ≤ (105 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (105 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (105 / 8 : ℝ) ^ 4 / 1024 = (121550625 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (121550625 / 4194304 : ℝ) (105 / 8 : ℝ) hN hK (cell_43 t hl hu)

private theorem cell_44 (t : ℝ)
    (hl : (105 / 8 : ℝ) ≤ t) (hu : t ≤ (53 / 4 : ℝ)) :
    (7890481 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-105 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (13035425 / 4096 : ℝ) (6619629 / 2048 : ℝ) (10078571 / 3072 : ℝ) (1704005 / 512 : ℝ) (863825 / 256 : ℝ)
    (7890481 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_44_global (t : ℝ)
    (hl : (105 / 8 : ℝ) ≤ t) (hu : t ≤ (53 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (53 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (53 / 4 : ℝ) ^ 4 / 1024 = (7890481 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (7890481 / 262144 : ℝ) (53 / 4 : ℝ) hN hK (cell_44 t hl hu)

private theorem cell_45 (t : ℝ)
    (hl : (53 / 4 : ℝ) ≤ t) (hu : t ≤ (107 / 8 : ℝ)) :
    (131079601 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-106 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (863825 / 256 : ℝ) (1751295 / 512 : ℝ) (10646051 / 3072 : ℝ) (7187215 / 2048 : ℝ) (14549553 / 4096 : ℝ)
    (131079601 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_45_global (t : ℝ)
    (hl : (53 / 4 : ℝ) ≤ t) (hu : t ≤ (107 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (107 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (107 / 8 : ℝ) ^ 4 / 1024 = (131079601 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (131079601 / 4194304 : ℝ) (107 / 8 : ℝ) hN hK (cell_45 t hl hu)

private theorem cell_46 (t : ℝ)
    (hl : (107 / 8 : ℝ) ≤ t) (hu : t ≤ (27 / 2 : ℝ)) :
    (531441 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-107 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (14549553 / 4096 : ℝ) (3681169 / 1024 : ℝ) (2792855 / 768 : ℝ) (235333 / 64 : ℝ) (59465 / 16 : ℝ)
    (531441 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_46_global (t : ℝ)
    (hl : (107 / 8 : ℝ) ≤ t) (hu : t ≤ (27 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (27 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (27 / 2 : ℝ) ^ 4 / 1024 = (531441 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (531441 / 16384 : ℝ) (27 / 2 : ℝ) hN hK (cell_46 t hl hu)

private theorem cell_47 (t : ℝ)
    (hl : (27 / 2 : ℝ) ≤ t) (hu : t ≤ (109 / 8 : ℝ)) :
    (141158161 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-108 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (59465 / 16 : ℝ) (240387 / 64 : ℝ) (2914151 / 768 : ℝ) (3923815 / 1024 : ℝ) (15844241 / 4096 : ℝ)
    (141158161 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_47_global (t : ℝ)
    (hl : (27 / 2 : ℝ) ≤ t) (hu : t ≤ (109 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (109 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (109 / 8 : ℝ) ^ 4 / 1024 = (141158161 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (141158161 / 4194304 : ℝ) (109 / 8 : ℝ) hN hK (cell_47 t hl hu)

private theorem cell_48 (t : ℝ)
    (hl : (109 / 8 : ℝ) ≤ t) (hu : t ≤ (55 / 4 : ℝ)) :
    (9150625 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-109 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (15844241 / 4096 : ℝ) (7996611 / 2048 : ℝ) (12103547 / 3072 : ℝ) (2034859 / 512 : ℝ) (1025985 / 256 : ℝ)
    (9150625 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_48_global (t : ℝ)
    (hl : (109 / 8 : ℝ) ≤ t) (hu : t ≤ (55 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (55 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (55 / 4 : ℝ) ^ 4 / 1024 = (9150625 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (9150625 / 262144 : ℝ) (55 / 4 : ℝ) hN hK (cell_48 t hl hu)

private theorem cell_49 (t : ℝ)
    (hl : (55 / 4 : ℝ) ≤ t) (hu : t ≤ (111 / 8 : ℝ)) :
    (151807041 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-110 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1025985 / 256 : ℝ) (2069081 / 512 : ℝ) (12514211 / 3072 : ℝ) (8407385 / 2048 : ℝ) (16940225 / 4096 : ℝ)
    (151807041 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_49_global (t : ℝ)
    (hl : (55 / 4 : ℝ) ≤ t) (hu : t ≤ (111 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (111 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (111 / 8 : ℝ) ^ 4 / 1024 = (151807041 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (151807041 / 4194304 : ℝ) (111 / 8 : ℝ) hN hK (cell_49 t hl hu)

private theorem cell_50 (t : ℝ)
    (hl : (111 / 8 : ℝ) ≤ t) (hu : t ≤ (14 : ℝ)) :
    (2401 / 64 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-111 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (16940225 / 4096 : ℝ) (1066605 / 256 : ℝ) (805661 / 192 : ℝ) (4225 : ℝ) (4253 : ℝ)
    (2401 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_50_global (t : ℝ)
    (hl : (111 / 8 : ℝ) ≤ t) (hu : t ≤ (14 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (14 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (14 : ℝ) ^ 4 / 1024 = (2401 / 64 : ℝ) := by norm_num
  exact from_cell_F t (2401 / 64 : ℝ) (14 : ℝ) hN hK (cell_50 t hl hu)

private theorem cell_51 (t : ℝ)
    (hl : (14 : ℝ) ≤ t) (hu : t ≤ (113 / 8 : ℝ)) :
    (163047361 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-112 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (4253 : ℝ) (4281 : ℝ) (827165 / 192 : ℝ) (1109627 / 256 : ℝ) (17858625 / 4096 : ℝ)
    (163047361 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_51_global (t : ℝ)
    (hl : (14 : ℝ) ≤ t) (hu : t ≤ (113 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (113 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (113 / 8 : ℝ) ^ 4 / 1024 = (163047361 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (163047361 / 4194304 : ℝ) (113 / 8 : ℝ) hN hK (cell_51 t hl hu)

private theorem cell_52 (t : ℝ)
    (hl : (113 / 8 : ℝ) ≤ t) (hu : t ≤ (57 / 4 : ℝ)) :
    (10556001 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-113 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (17858625 / 4096 : ℝ) (8981609 / 2048 : ℝ) (13548419 / 3072 : ℝ) (2270345 / 512 : ℝ) (1141121 / 256 : ℝ)
    (10556001 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_52_global (t : ℝ)
    (hl : (113 / 8 : ℝ) ≤ t) (hu : t ≤ (57 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (57 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (57 / 4 : ℝ) ^ 4 / 1024 = (10556001 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (10556001 / 262144 : ℝ) (57 / 4 : ℝ) hN hK (cell_52 t hl hu)

private theorem cell_53 (t : ℝ)
    (hl : (57 / 4 : ℝ) ≤ t) (hu : t ≤ (115 / 8 : ℝ)) :
    (174900625 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-114 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1141121 / 256 : ℝ) (2294139 / 512 : ℝ) (13833947 / 3072 : ℝ) (9267251 / 2048 : ℝ) (18620945 / 4096 : ℝ)
    (174900625 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_53_global (t : ℝ)
    (hl : (57 / 4 : ℝ) ≤ t) (hu : t ≤ (115 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (115 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (115 / 8 : ℝ) ^ 4 / 1024 = (174900625 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (174900625 / 4194304 : ℝ) (115 / 8 : ℝ) hN hK (cell_53 t hl hu)

private theorem cell_54 (t : ℝ)
    (hl : (115 / 8 : ℝ) ≤ t) (hu : t ≤ (29 / 2 : ℝ)) :
    (707281 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-115 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (18620945 / 4096 : ℝ) (4676847 / 1024 : ℝ) (3523319 / 768 : ℝ) (294875 / 64 : ℝ) (74025 / 16 : ℝ)
    (707281 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_54_global (t : ℝ)
    (hl : (115 / 8 : ℝ) ≤ t) (hu : t ≤ (29 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (29 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (29 / 2 : ℝ) ^ 4 / 1024 = (707281 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (707281 / 16384 : ℝ) (29 / 2 : ℝ) hN hK (cell_54 t hl hu)

private theorem cell_55 (t : ℝ)
    (hl : (29 / 2 : ℝ) ≤ t) (hu : t ≤ (117 / 8 : ℝ)) :
    (187388721 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-116 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (74025 / 16 : ℝ) (297325 / 64 : ℝ) (3582119 / 768 : ℝ) (4794505 / 1024 : ℝ) (19249073 / 4096 : ℝ)
    (187388721 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_55_global (t : ℝ)
    (hl : (29 / 2 : ℝ) ≤ t) (hu : t ≤ (117 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (117 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (117 / 8 : ℝ) ^ 4 / 1024 = (187388721 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (187388721 / 4194304 : ℝ) (117 / 8 : ℝ) hN hK (cell_55 t hl hu)

private theorem cell_56 (t : ℝ)
    (hl : (117 / 8 : ℝ) ≤ t) (hu : t ≤ (59 / 4 : ℝ)) :
    (12117361 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-117 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (19249073 / 4096 : ℝ) (9660063 / 2048 : ℝ) (14541635 / 3072 : ℝ) (2431919 / 512 : ℝ) (1219985 / 256 : ℝ)
    (12117361 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_56_global (t : ℝ)
    (hl : (117 / 8 : ℝ) ≤ t) (hu : t ≤ (59 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (59 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (59 / 4 : ℝ) ^ 4 / 1024 = (12117361 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (12117361 / 262144 : ℝ) (59 / 4 : ℝ) hN hK (cell_56 t hl hu)

private theorem cell_57 (t : ℝ)
    (hl : (59 / 4 : ℝ) ≤ t) (hu : t ≤ (119 / 8 : ℝ)) :
    (200533921 / 4194304 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-118 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1219985 / 256 : ℝ) (2448021 / 512 : ℝ) (14734859 / 3072 : ℝ) (9853405 / 2048 : ℝ) (19765281 / 4096 : ℝ)
    (200533921 / 4194304 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_57_global (t : ℝ)
    (hl : (59 / 4 : ℝ) ≤ t) (hu : t ≤ (119 / 8 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (119 / 8 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (119 / 8 : ℝ) ^ 4 / 1024 = (200533921 / 4194304 : ℝ) := by norm_num
  exact from_cell_F t (200533921 / 4194304 : ℝ) (119 / 8 : ℝ) hN hK (cell_57 t hl hu)

private theorem cell_58 (t : ℝ)
    (hl : (119 / 8 : ℝ) ≤ t) (hu : t ≤ (15 : ℝ)) :
    (50625 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (8 : ℝ) * t + (-119 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (19765281 / 4096 : ℝ) (2477969 / 512 : ℝ) (232973 / 48 : ℝ) (4867 : ℝ) (4880 : ℝ)
    (50625 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_58_global (t : ℝ)
    (hl : (119 / 8 : ℝ) ≤ t) (hu : t ≤ (15 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (15 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (15 : ℝ) ^ 4 / 1024 = (50625 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (50625 / 1024 : ℝ) (15 : ℝ) hN hK (cell_58 t hl hu)

private theorem cell_59 (t : ℝ)
    (hl : (15 : ℝ) ≤ t) (hu : t ≤ (61 / 4 : ℝ)) :
    (13845841 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-60 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (4880 : ℝ) (4906 : ℝ) (59165 / 12 : ℝ) (317023 / 64 : ℝ) (1273713 / 256 : ℝ)
    (13845841 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_59_global (t : ℝ)
    (hl : (15 : ℝ) ≤ t) (hu : t ≤ (61 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (61 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (61 / 4 : ℝ) ^ 4 / 1024 = (13845841 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (13845841 / 262144 : ℝ) (61 / 4 : ℝ) hN hK (cell_59 t hl hu)

private theorem cell_60 (t : ℝ)
    (hl : (61 / 4 : ℝ) ≤ t) (hu : t ≤ (31 / 2 : ℝ)) :
    (923521 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-61 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1273713 / 256 : ℝ) (639667 / 128 : ℝ) (963503 / 192 : ℝ) (161223 / 32 : ℝ) (80921 / 16 : ℝ)
    (923521 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_60_global (t : ℝ)
    (hl : (61 / 4 : ℝ) ≤ t) (hu : t ≤ (31 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (31 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (31 / 2 : ℝ) ^ 4 / 1024 = (923521 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (923521 / 16384 : ℝ) (31 / 2 : ℝ) hN hK (cell_60 t hl hu)

private theorem cell_61 (t : ℝ)
    (hl : (31 / 2 : ℝ) ≤ t) (hu : t ≤ (63 / 4 : ℝ)) :
    (15752961 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-62 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (80921 / 16 : ℝ) (162461 / 32 : ℝ) (978359 / 192 : ℝ) (654585 / 128 : ℝ) (1313825 / 256 : ℝ)
    (15752961 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_61_global (t : ℝ)
    (hl : (31 / 2 : ℝ) ≤ t) (hu : t ≤ (63 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (63 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (63 / 4 : ℝ) ^ 4 / 1024 = (15752961 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (15752961 / 262144 : ℝ) (63 / 4 : ℝ) hN hK (cell_61 t hl hu)

private theorem cell_62 (t : ℝ)
    (hl : (63 / 4 : ℝ) ≤ t) (hu : t ≤ (16 : ℝ)) :
    (64 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-63 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1313825 / 256 : ℝ) (82405 / 16 : ℝ) (248081 / 48 : ℝ) (10373 / 2 : ℝ) (5205 : ℝ)
    (64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_62_global (t : ℝ)
    (hl : (63 / 4 : ℝ) ≤ t) (hu : t ≤ (16 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (16 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (16 : ℝ) ^ 4 / 1024 = (64 : ℝ) := by norm_num
  exact from_cell_F t (64 : ℝ) (16 : ℝ) hN hK (cell_62 t hl hu)

private theorem cell_63 (t : ℝ)
    (hl : (16 : ℝ) ≤ t) (hu : t ≤ (65 / 4 : ℝ)) :
    (17850625 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-64 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (5205 : ℝ) (10447 / 2 : ℝ) (251633 / 48 : ℝ) (84189 / 16 : ℝ) (1352225 / 256 : ℝ)
    (17850625 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_63_global (t : ℝ)
    (hl : (16 : ℝ) ≤ t) (hu : t ≤ (65 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (65 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (65 / 4 : ℝ) ^ 4 / 1024 = (17850625 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (17850625 / 262144 : ℝ) (65 / 4 : ℝ) hN hK (cell_63 t hl hu)

private theorem cell_64 (t : ℝ)
    (hl : (65 / 4 : ℝ) ≤ t) (hu : t ≤ (33 / 2 : ℝ)) :
    (1185921 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-65 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1352225 / 256 : ℝ) (678713 / 128 : ℝ) (1022135 / 192 : ℝ) (171069 / 32 : ℝ) (85913 / 16 : ℝ)
    (1185921 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_64_global (t : ℝ)
    (hl : (65 / 4 : ℝ) ≤ t) (hu : t ≤ (33 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (33 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (33 / 2 : ℝ) ^ 4 / 1024 = (1185921 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (1185921 / 16384 : ℝ) (33 / 2 : ℝ) hN hK (cell_64 t hl hu)

private theorem cell_65 (t : ℝ)
    (hl : (33 / 2 : ℝ) ≤ t) (hu : t ≤ (67 / 4 : ℝ)) :
    (20151121 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-66 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (85913 / 16 : ℝ) (172583 / 32 : ℝ) (1040303 / 192 : ℝ) (696947 / 128 : ℝ) (1401201 / 256 : ℝ)
    (20151121 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_65_global (t : ℝ)
    (hl : (33 / 2 : ℝ) ≤ t) (hu : t ≤ (67 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (67 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (67 / 4 : ℝ) ^ 4 / 1024 = (20151121 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (20151121 / 262144 : ℝ) (67 / 4 : ℝ) hN hK (cell_65 t hl hu)

private theorem cell_66 (t : ℝ)
    (hl : (67 / 4 : ℝ) ≤ t) (hu : t ≤ (17 : ℝ)) :
    (83521 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-67 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1401201 / 256 : ℝ) (352127 / 64 : ℝ) (66389 / 12 : ℝ) (5565 : ℝ) (5600 : ℝ)
    (83521 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_66_global (t : ℝ)
    (hl : (67 / 4 : ℝ) ≤ t) (hu : t ≤ (17 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (17 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (17 : ℝ) ^ 4 / 1024 = (83521 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (83521 / 1024 : ℝ) (17 : ℝ) hN hK (cell_66 t hl hu)

private theorem cell_67 (t : ℝ)
    (hl : (17 : ℝ) ≤ t) (hu : t ≤ (69 / 4 : ℝ)) :
    (22667121 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-68 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (5600 : ℝ) (5635 : ℝ) (68069 / 12 : ℝ) (365601 / 64 : ℝ) (1473425 / 256 : ℝ)
    (22667121 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_67_global (t : ℝ)
    (hl : (17 : ℝ) ≤ t) (hu : t ≤ (69 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (69 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (69 / 4 : ℝ) ^ 4 / 1024 = (22667121 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (22667121 / 262144 : ℝ) (69 / 4 : ℝ) hN hK (cell_67 t hl hu)

private theorem cell_68 (t : ℝ)
    (hl : (69 / 4 : ℝ) ≤ t) (hu : t ≤ (35 / 2 : ℝ)) :
    (1500625 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-69 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1473425 / 256 : ℝ) (742223 / 128 : ℝ) (1122167 / 192 : ℝ) (188603 / 32 : ℝ) (95145 / 16 : ℝ)
    (1500625 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_68_global (t : ℝ)
    (hl : (69 / 4 : ℝ) ≤ t) (hu : t ≤ (35 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (35 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (35 / 2 : ℝ) ^ 4 / 1024 = (1500625 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (1500625 / 16384 : ℝ) (35 / 2 : ℝ) hN hK (cell_68 t hl hu)

private theorem cell_69 (t : ℝ)
    (hl : (35 / 2 : ℝ) ≤ t) (hu : t ≤ (71 / 4 : ℝ)) :
    (25411681 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-70 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (95145 / 16 : ℝ) (191977 / 32 : ℝ) (1162655 / 192 : ℝ) (782781 / 128 : ℝ) (1581953 / 256 : ℝ)
    (25411681 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_69_global (t : ℝ)
    (hl : (35 / 2 : ℝ) ≤ t) (hu : t ≤ (71 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (71 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (71 / 4 : ℝ) ^ 4 / 1024 = (25411681 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (25411681 / 262144 : ℝ) (71 / 4 : ℝ) hN hK (cell_69 t hl hu)

private theorem cell_70 (t : ℝ)
    (hl : (71 / 4 : ℝ) ≤ t) (hu : t ≤ (18 : ℝ)) :
    (6561 / 64 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-71 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1581953 / 256 : ℝ) (199793 / 32 : ℝ) (302957 / 48 : ℝ) (6384 : ℝ) (6461 : ℝ)
    (6561 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_70_global (t : ℝ)
    (hl : (71 / 4 : ℝ) ≤ t) (hu : t ≤ (18 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (18 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (18 : ℝ) ^ 4 / 1024 = (6561 / 64 : ℝ) := by norm_num
  exact from_cell_F t (6561 / 64 : ℝ) (18 : ℝ) hN hK (cell_70 t hl hu)

private theorem cell_71 (t : ℝ)
    (hl : (18 : ℝ) ≤ t) (hu : t ≤ (73 / 4 : ℝ)) :
    (28398241 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-72 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (6461 : ℝ) (6538 : ℝ) (317741 / 48 : ℝ) (214595 / 32 : ℝ) (1740225 / 256 : ℝ)
    (28398241 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_71_global (t : ℝ)
    (hl : (18 : ℝ) ≤ t) (hu : t ≤ (73 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (73 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (73 / 4 : ℝ) ^ 4 / 1024 = (28398241 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (28398241 / 262144 : ℝ) (73 / 4 : ℝ) hN hK (cell_71 t hl hu)

private theorem cell_72 (t : ℝ)
    (hl : (73 / 4 : ℝ) ≤ t) (hu : t ≤ (37 / 2 : ℝ)) :
    (1874161 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-73 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1740225 / 256 : ℝ) (881845 / 128 : ℝ) (1341359 / 192 : ℝ) (226833 / 32 : ℝ) (115145 / 16 : ℝ)
    (1874161 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_72_global (t : ℝ)
    (hl : (73 / 4 : ℝ) ≤ t) (hu : t ≤ (37 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (37 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (37 / 2 : ℝ) ^ 4 / 1024 = (1874161 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (1874161 / 16384 : ℝ) (37 / 2 : ℝ) hN hK (cell_72 t hl hu)

private theorem cell_73 (t : ℝ)
    (hl : (37 / 2 : ℝ) ≤ t) (hu : t ≤ (75 / 4 : ℝ)) :
    (31640625 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-74 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (115145 / 16 : ℝ) (233747 / 32 : ℝ) (1424327 / 192 : ℝ) (964887 / 128 : ℝ) (1962065 / 256 : ℝ)
    (31640625 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_73_global (t : ℝ)
    (hl : (37 / 2 : ℝ) ≤ t) (hu : t ≤ (75 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (75 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (75 / 4 : ℝ) ^ 4 / 1024 = (31640625 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (31640625 / 262144 : ℝ) (75 / 4 : ℝ) hN hK (cell_73 t hl hu)

private theorem cell_74 (t : ℝ)
    (hl : (75 / 4 : ℝ) ≤ t) (hu : t ≤ (19 : ℝ)) :
    (130321 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-75 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (1962065 / 256 : ℝ) (498589 / 64 : ℝ) (95075 / 12 : ℝ) (8062 : ℝ) (8208 : ℝ)
    (130321 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_74_global (t : ℝ)
    (hl : (75 / 4 : ℝ) ≤ t) (hu : t ≤ (19 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (19 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (19 : ℝ) ^ 4 / 1024 = (130321 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (130321 / 1024 : ℝ) (19 : ℝ) hN hK (cell_74 t hl hu)

private theorem cell_75 (t : ℝ)
    (hl : (19 : ℝ) ≤ t) (hu : t ≤ (77 / 4 : ℝ)) :
    (35153041 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-76 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (8208 : ℝ) (8354 : ℝ) (102083 / 12 : ℝ) (554691 / 64 : ℝ) (2261681 / 256 : ℝ)
    (35153041 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_75_global (t : ℝ)
    (hl : (19 : ℝ) ≤ t) (hu : t ≤ (77 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (77 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (77 / 4 : ℝ) ^ 4 / 1024 = (35153041 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (35153041 / 262144 : ℝ) (77 / 4 : ℝ) hN hK (cell_75 t hl hu)

private theorem cell_76 (t : ℝ)
    (hl : (77 / 4 : ℝ) ≤ t) (hu : t ≤ (39 / 2 : ℝ)) :
    (2313441 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-77 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2261681 / 256 : ℝ) (1152299 / 128 : ℝ) (1762079 / 192 : ℝ) (299535 / 32 : ℝ) (152825 / 16 : ℝ)
    (2313441 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_76_global (t : ℝ)
    (hl : (77 / 4 : ℝ) ≤ t) (hu : t ≤ (39 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (39 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (39 / 2 : ℝ) ^ 4 / 1024 = (2313441 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (2313441 / 16384 : ℝ) (39 / 2 : ℝ) hN hK (cell_76 t hl hu)

private theorem cell_77 (t : ℝ)
    (hl : (39 / 2 : ℝ) ≤ t) (hu : t ≤ (79 / 4 : ℝ)) :
    (38950081 / 262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-78 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (152825 / 16 : ℝ) (311765 / 32 : ℝ) (1908839 / 192 : ℝ) (1299137 / 128 : ℝ) (2653665 / 256 : ℝ)
    (38950081 / 262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_77_global (t : ℝ)
    (hl : (39 / 2 : ℝ) ≤ t) (hu : t ≤ (79 / 4 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (79 / 4 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (79 / 4 : ℝ) ^ 4 / 1024 = (38950081 / 262144 : ℝ) := by norm_num
  exact from_cell_F t (38950081 / 262144 : ℝ) (79 / 4 : ℝ) hN hK (cell_77 t hl hu)

private theorem cell_78 (t : ℝ)
    (hl : (79 / 4 : ℝ) ≤ t) (hu : t ≤ (20 : ℝ)) :
    (625 / 4 : ℝ) ≤ F_real t := by
  let s : ℝ := (4 : ℝ) * t + (-79 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (2653665 / 256 : ℝ) (42329 / 4 : ℝ) (518753 / 48 : ℝ) (22083 / 2 : ℝ) (11285 : ℝ)
    (625 / 4 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_78_global (t : ℝ)
    (hl : (79 / 4 : ℝ) ≤ t) (hu : t ≤ (20 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (20 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (20 : ℝ) ^ 4 / 1024 = (625 / 4 : ℝ) := by norm_num
  exact from_cell_F t (625 / 4 : ℝ) (20 : ℝ) hN hK (cell_78 t hl hu)

private theorem cell_79 (t : ℝ)
    (hl : (20 : ℝ) ≤ t) (hu : t ≤ (41 / 2 : ℝ)) :
    (2825761 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-40 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (11285 : ℝ) (11772 : ℝ) (147557 / 12 : ℝ) (51443 / 4 : ℝ) (215481 / 16 : ℝ)
    (2825761 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_79_global (t : ℝ)
    (hl : (20 : ℝ) ≤ t) (hu : t ≤ (41 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (41 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (41 / 2 : ℝ) ^ 4 / 1024 = (2825761 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (2825761 / 16384 : ℝ) (41 / 2 : ℝ) hN hK (cell_79 t hl hu)

private theorem cell_80 (t : ℝ)
    (hl : (41 / 2 : ℝ) ≤ t) (hu : t ≤ (21 : ℝ)) :
    (194481 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-41 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (215481 / 16 : ℝ) (112595 / 8 : ℝ) (44171 / 3 : ℝ) (15418 : ℝ) (16160 : ℝ)
    (194481 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_80_global (t : ℝ)
    (hl : (41 / 2 : ℝ) ≤ t) (hu : t ≤ (21 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (21 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (21 : ℝ) ^ 4 / 1024 = (194481 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (194481 / 1024 : ℝ) (21 : ℝ) hN hK (cell_80 t hl hu)

private theorem cell_81 (t : ℝ)
    (hl : (21 : ℝ) ≤ t) (hu : t ≤ (43 / 2 : ℝ)) :
    (3418801 / 16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-42 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (16160 : ℝ) (16902 : ℝ) (53075 / 3 : ℝ) (148253 / 8 : ℝ) (310793 / 16 : ℝ)
    (3418801 / 16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_81_global (t : ℝ)
    (hl : (21 : ℝ) ≤ t) (hu : t ≤ (43 / 2 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (43 / 2 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (43 / 2 : ℝ) ^ 4 / 1024 = (3418801 / 16384 : ℝ) := by norm_num
  exact from_cell_F t (3418801 / 16384 : ℝ) (43 / 2 : ℝ) hN hK (cell_81 t hl hu)

private theorem cell_82 (t : ℝ)
    (hl : (43 / 2 : ℝ) ≤ t) (hu : t ≤ (22 : ℝ)) :
    (14641 / 64 : ℝ) ≤ F_real t := by
  let s : ℝ := (2 : ℝ) * t + (-43 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (310793 / 16 : ℝ) (40635 / 2 : ℝ) (255161 / 12 : ℝ) (22265 : ℝ) (23325 : ℝ)
    (14641 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_82_global (t : ℝ)
    (hl : (43 / 2 : ℝ) ≤ t) (hu : t ≤ (22 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (22 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (22 : ℝ) ^ 4 / 1024 = (14641 / 64 : ℝ) := by norm_num
  exact from_cell_F t (14641 / 64 : ℝ) (22 : ℝ) hN hK (cell_82 t hl hu)

private theorem cell_83 (t : ℝ)
    (hl : (22 : ℝ) ≤ t) (hu : t ≤ (23 : ℝ)) :
    (279841 / 1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 : ℝ) * t + (-22 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (23325 : ℝ) (25445 : ℝ) (83396 / 3 : ℝ) (30408 : ℝ) (33296 : ℝ)
    (279841 / 1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_83_global (t : ℝ)
    (hl : (22 : ℝ) ≤ t) (hu : t ≤ (23 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (23 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (23 : ℝ) ^ 4 / 1024 = (279841 / 1024 : ℝ) := by norm_num
  exact from_cell_F t (279841 / 1024 : ℝ) (23 : ℝ) hN hK (cell_83 t hl hu)

private theorem cell_84 (t : ℝ)
    (hl : (23 : ℝ) ≤ t) (hu : t ≤ (24 : ℝ)) :
    (324 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 : ℝ) * t + (-23 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (33296 : ℝ) (36184 : ℝ) (118052 / 3 : ℝ) (42819 : ℝ) (46613 : ℝ)
    (324 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_84_global (t : ℝ)
    (hl : (23 : ℝ) ≤ t) (hu : t ≤ (24 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (24 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (24 : ℝ) ^ 4 / 1024 = (324 : ℝ) := by norm_num
  exact from_cell_F t (324 : ℝ) (24 : ℝ) hN hK (cell_84 t hl hu)

private theorem cell_85 (t : ℝ)
    (hl : (24 : ℝ) ≤ t) (hu : t ≤ (26 : ℝ)) :
    (28561 / 64 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 2 : ℝ) * t + (-12 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (46613 : ℝ) (54201 : ℝ) (189275 / 3 : ℝ) (73477 : ℝ) (85565 : ℝ)
    (28561 / 64 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_85_global (t : ℝ)
    (hl : (24 : ℝ) ≤ t) (hu : t ≤ (26 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (26 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (26 : ℝ) ^ 4 / 1024 = (28561 / 64 : ℝ) := by norm_num
  exact from_cell_F t (28561 / 64 : ℝ) (26 : ℝ) hN hK (cell_85 t hl hu)

private theorem cell_86 (t : ℝ)
    (hl : (26 : ℝ) ≤ t) (hu : t ≤ (28 : ℝ)) :
    (2401 / 4 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 2 : ℝ) * t + (-13 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (85565 : ℝ) (97653 : ℝ) (334331 / 3 : ℝ) (127145 : ℝ) (144981 : ℝ)
    (2401 / 4 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_86_global (t : ℝ)
    (hl : (26 : ℝ) ≤ t) (hu : t ≤ (28 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (28 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (28 : ℝ) ^ 4 / 1024 = (2401 / 4 : ℝ) := by norm_num
  exact from_cell_F t (2401 / 4 : ℝ) (28 : ℝ) hN hK (cell_86 t hl hu)

private theorem cell_87 (t : ℝ)
    (hl : (28 : ℝ) ≤ t) (hu : t ≤ (32 : ℝ)) :
    (1024 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 4 : ℝ) * t + (-7 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (144981 : ℝ) (180653 : ℝ) (674591 / 3 : ℝ) (279405 : ℝ) (346325 : ℝ)
    (1024 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_87_global (t : ℝ)
    (hl : (28 : ℝ) ≤ t) (hu : t ≤ (32 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (32 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (32 : ℝ) ^ 4 / 1024 = (1024 : ℝ) := by norm_num
  exact from_cell_F t (1024 : ℝ) (32 : ℝ) hN hK (cell_87 t hl hu)

private theorem cell_88 (t : ℝ)
    (hl : (32 : ℝ) ≤ t) (hu : t ≤ (48 : ℝ)) :
    (5184 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 16 : ℝ) * t + (-2 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (346325 : ℝ) (614005 : ℝ) (3239231 / 3 : ℝ) (1874613 : ℝ) (3195221 : ℝ)
    (5184 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_88_global (t : ℝ)
    (hl : (32 : ℝ) ≤ t) (hu : t ≤ (48 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (48 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (48 : ℝ) ^ 4 / 1024 = (5184 : ℝ) := by norm_num
  exact from_cell_F t (5184 : ℝ) (48 : ℝ) hN hK (cell_88 t hl hu)

private theorem cell_89 (t : ℝ)
    (hl : (48 : ℝ) ≤ t) (hu : t ≤ (64 : ℝ)) :
    (16384 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 16 : ℝ) * t + (-3 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (3195221 : ℝ) (4515829 : ℝ) (19086527 / 3 : ℝ) (8930869 : ℝ) (12484053 : ℝ)
    (16384 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_89_global (t : ℝ)
    (hl : (48 : ℝ) ≤ t) (hu : t ≤ (64 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (64 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (64 : ℝ) ^ 4 / 1024 = (16384 : ℝ) := by norm_num
  exact from_cell_F t (16384 : ℝ) (64 : ℝ) hN hK (cell_89 t hl hu)

private theorem cell_90 (t : ℝ)
    (hl : (64 : ℝ) ≤ t) (hu : t ≤ (128 : ℝ)) :
    (262144 : ℝ) ≤ F_real t := by
  let s : ℝ := (1 / 64 : ℝ) * t + (-1 : ℝ)
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]
  have hb := XedniDuplicationBound.bernstein_quartic_lower
    (12484053 : ℝ) (26696789 : ℝ) (169984127 / 3 : ℝ) (119155029 : ℝ) (247732181 : ℝ)
    (262144 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s, F_real] <;> ring

private theorem cell_90_global (t : ℝ)
    (hl : (64 : ℝ) ≤ t) (hu : t ≤ (128 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have hN : max (1 : ℝ) |t| ≤ (128 : ℝ) := by
    apply max_le
    · norm_num
    · apply abs_le.mpr
      constructor <;> linarith [hl, hu]
  have hK : (128 : ℝ) ^ 4 / 1024 = (262144 : ℝ) := by norm_num
  exact from_cell_F t (262144 : ℝ) (128 : ℝ) hN hK (cell_90 t hl hu)

set_option maxHeartbeats 5000000 in
theorem finite_archimedean_lower (t : ℝ) (hl : (-128 : ℝ) ≤ t)
    (hu : t ≤ (128 : ℝ)) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  by_cases h0 : t ≤ (-64 : ℝ)
  · exact cell_0_global t hl h0
  by_cases h1 : t ≤ (-32 : ℝ)
  · exact cell_1_global t (le_of_lt (lt_of_not_ge h0)) h1
  by_cases h2 : t ≤ (0 : ℝ)
  · exact cell_2_global t (le_of_lt (lt_of_not_ge h1)) h2
  by_cases h3 : t ≤ (4 : ℝ)
  · exact cell_3_global t (le_of_lt (lt_of_not_ge h2)) h3
  by_cases h4 : t ≤ (6 : ℝ)
  · exact cell_4_global t (le_of_lt (lt_of_not_ge h3)) h4
  by_cases h5 : t ≤ (8 : ℝ)
  · exact cell_5_global t (le_of_lt (lt_of_not_ge h4)) h5
  by_cases h6 : t ≤ (9 : ℝ)
  · exact cell_6_global t (le_of_lt (lt_of_not_ge h5)) h6
  by_cases h7 : t ≤ (19 / 2 : ℝ)
  · exact cell_7_global t (le_of_lt (lt_of_not_ge h6)) h7
  by_cases h8 : t ≤ (10 : ℝ)
  · exact cell_8_global t (le_of_lt (lt_of_not_ge h7)) h8
  by_cases h9 : t ≤ (41 / 4 : ℝ)
  · exact cell_9_global t (le_of_lt (lt_of_not_ge h8)) h9
  by_cases h10 : t ≤ (21 / 2 : ℝ)
  · exact cell_10_global t (le_of_lt (lt_of_not_ge h9)) h10
  by_cases h11 : t ≤ (43 / 4 : ℝ)
  · exact cell_11_global t (le_of_lt (lt_of_not_ge h10)) h11
  by_cases h12 : t ≤ (87 / 8 : ℝ)
  · exact cell_12_global t (le_of_lt (lt_of_not_ge h11)) h12
  by_cases h13 : t ≤ (11 : ℝ)
  · exact cell_13_global t (le_of_lt (lt_of_not_ge h12)) h13
  by_cases h14 : t ≤ (89 / 8 : ℝ)
  · exact cell_14_global t (le_of_lt (lt_of_not_ge h13)) h14
  by_cases h15 : t ≤ (45 / 4 : ℝ)
  · exact cell_15_global t (le_of_lt (lt_of_not_ge h14)) h15
  by_cases h16 : t ≤ (91 / 8 : ℝ)
  · exact cell_16_global t (le_of_lt (lt_of_not_ge h15)) h16
  by_cases h17 : t ≤ (23 / 2 : ℝ)
  · exact cell_17_global t (le_of_lt (lt_of_not_ge h16)) h17
  by_cases h18 : t ≤ (185 / 16 : ℝ)
  · exact cell_18_global t (le_of_lt (lt_of_not_ge h17)) h18
  by_cases h19 : t ≤ (93 / 8 : ℝ)
  · exact cell_19_global t (le_of_lt (lt_of_not_ge h18)) h19
  by_cases h20 : t ≤ (187 / 16 : ℝ)
  · exact cell_20_global t (le_of_lt (lt_of_not_ge h19)) h20
  by_cases h21 : t ≤ (47 / 4 : ℝ)
  · exact cell_21_global t (le_of_lt (lt_of_not_ge h20)) h21
  by_cases h22 : t ≤ (189 / 16 : ℝ)
  · exact cell_22_global t (le_of_lt (lt_of_not_ge h21)) h22
  by_cases h23 : t ≤ (95 / 8 : ℝ)
  · exact cell_23_global t (le_of_lt (lt_of_not_ge h22)) h23
  by_cases h24 : t ≤ (381 / 32 : ℝ)
  · exact cell_24_global t (le_of_lt (lt_of_not_ge h23)) h24
  by_cases h25 : t ≤ (191 / 16 : ℝ)
  · exact cell_25_global t (le_of_lt (lt_of_not_ge h24)) h25
  by_cases h26 : t ≤ (383 / 32 : ℝ)
  · exact cell_26_global t (le_of_lt (lt_of_not_ge h25)) h26
  by_cases h27 : t ≤ (12 : ℝ)
  · exact cell_27_global t (le_of_lt (lt_of_not_ge h26)) h27
  by_cases h28 : t ≤ (385 / 32 : ℝ)
  · exact cell_28_global t (le_of_lt (lt_of_not_ge h27)) h28
  by_cases h29 : t ≤ (193 / 16 : ℝ)
  · exact cell_29_global t (le_of_lt (lt_of_not_ge h28)) h29
  by_cases h30 : t ≤ (387 / 32 : ℝ)
  · exact cell_30_global t (le_of_lt (lt_of_not_ge h29)) h30
  by_cases h31 : t ≤ (97 / 8 : ℝ)
  · exact cell_31_global t (le_of_lt (lt_of_not_ge h30)) h31
  by_cases h32 : t ≤ (195 / 16 : ℝ)
  · exact cell_32_global t (le_of_lt (lt_of_not_ge h31)) h32
  by_cases h33 : t ≤ (49 / 4 : ℝ)
  · exact cell_33_global t (le_of_lt (lt_of_not_ge h32)) h33
  by_cases h34 : t ≤ (197 / 16 : ℝ)
  · exact cell_34_global t (le_of_lt (lt_of_not_ge h33)) h34
  by_cases h35 : t ≤ (99 / 8 : ℝ)
  · exact cell_35_global t (le_of_lt (lt_of_not_ge h34)) h35
  by_cases h36 : t ≤ (199 / 16 : ℝ)
  · exact cell_36_global t (le_of_lt (lt_of_not_ge h35)) h36
  by_cases h37 : t ≤ (25 / 2 : ℝ)
  · exact cell_37_global t (le_of_lt (lt_of_not_ge h36)) h37
  by_cases h38 : t ≤ (201 / 16 : ℝ)
  · exact cell_38_global t (le_of_lt (lt_of_not_ge h37)) h38
  by_cases h39 : t ≤ (101 / 8 : ℝ)
  · exact cell_39_global t (le_of_lt (lt_of_not_ge h38)) h39
  by_cases h40 : t ≤ (51 / 4 : ℝ)
  · exact cell_40_global t (le_of_lt (lt_of_not_ge h39)) h40
  by_cases h41 : t ≤ (103 / 8 : ℝ)
  · exact cell_41_global t (le_of_lt (lt_of_not_ge h40)) h41
  by_cases h42 : t ≤ (13 : ℝ)
  · exact cell_42_global t (le_of_lt (lt_of_not_ge h41)) h42
  by_cases h43 : t ≤ (105 / 8 : ℝ)
  · exact cell_43_global t (le_of_lt (lt_of_not_ge h42)) h43
  by_cases h44 : t ≤ (53 / 4 : ℝ)
  · exact cell_44_global t (le_of_lt (lt_of_not_ge h43)) h44
  by_cases h45 : t ≤ (107 / 8 : ℝ)
  · exact cell_45_global t (le_of_lt (lt_of_not_ge h44)) h45
  by_cases h46 : t ≤ (27 / 2 : ℝ)
  · exact cell_46_global t (le_of_lt (lt_of_not_ge h45)) h46
  by_cases h47 : t ≤ (109 / 8 : ℝ)
  · exact cell_47_global t (le_of_lt (lt_of_not_ge h46)) h47
  by_cases h48 : t ≤ (55 / 4 : ℝ)
  · exact cell_48_global t (le_of_lt (lt_of_not_ge h47)) h48
  by_cases h49 : t ≤ (111 / 8 : ℝ)
  · exact cell_49_global t (le_of_lt (lt_of_not_ge h48)) h49
  by_cases h50 : t ≤ (14 : ℝ)
  · exact cell_50_global t (le_of_lt (lt_of_not_ge h49)) h50
  by_cases h51 : t ≤ (113 / 8 : ℝ)
  · exact cell_51_global t (le_of_lt (lt_of_not_ge h50)) h51
  by_cases h52 : t ≤ (57 / 4 : ℝ)
  · exact cell_52_global t (le_of_lt (lt_of_not_ge h51)) h52
  by_cases h53 : t ≤ (115 / 8 : ℝ)
  · exact cell_53_global t (le_of_lt (lt_of_not_ge h52)) h53
  by_cases h54 : t ≤ (29 / 2 : ℝ)
  · exact cell_54_global t (le_of_lt (lt_of_not_ge h53)) h54
  by_cases h55 : t ≤ (117 / 8 : ℝ)
  · exact cell_55_global t (le_of_lt (lt_of_not_ge h54)) h55
  by_cases h56 : t ≤ (59 / 4 : ℝ)
  · exact cell_56_global t (le_of_lt (lt_of_not_ge h55)) h56
  by_cases h57 : t ≤ (119 / 8 : ℝ)
  · exact cell_57_global t (le_of_lt (lt_of_not_ge h56)) h57
  by_cases h58 : t ≤ (15 : ℝ)
  · exact cell_58_global t (le_of_lt (lt_of_not_ge h57)) h58
  by_cases h59 : t ≤ (61 / 4 : ℝ)
  · exact cell_59_global t (le_of_lt (lt_of_not_ge h58)) h59
  by_cases h60 : t ≤ (31 / 2 : ℝ)
  · exact cell_60_global t (le_of_lt (lt_of_not_ge h59)) h60
  by_cases h61 : t ≤ (63 / 4 : ℝ)
  · exact cell_61_global t (le_of_lt (lt_of_not_ge h60)) h61
  by_cases h62 : t ≤ (16 : ℝ)
  · exact cell_62_global t (le_of_lt (lt_of_not_ge h61)) h62
  by_cases h63 : t ≤ (65 / 4 : ℝ)
  · exact cell_63_global t (le_of_lt (lt_of_not_ge h62)) h63
  by_cases h64 : t ≤ (33 / 2 : ℝ)
  · exact cell_64_global t (le_of_lt (lt_of_not_ge h63)) h64
  by_cases h65 : t ≤ (67 / 4 : ℝ)
  · exact cell_65_global t (le_of_lt (lt_of_not_ge h64)) h65
  by_cases h66 : t ≤ (17 : ℝ)
  · exact cell_66_global t (le_of_lt (lt_of_not_ge h65)) h66
  by_cases h67 : t ≤ (69 / 4 : ℝ)
  · exact cell_67_global t (le_of_lt (lt_of_not_ge h66)) h67
  by_cases h68 : t ≤ (35 / 2 : ℝ)
  · exact cell_68_global t (le_of_lt (lt_of_not_ge h67)) h68
  by_cases h69 : t ≤ (71 / 4 : ℝ)
  · exact cell_69_global t (le_of_lt (lt_of_not_ge h68)) h69
  by_cases h70 : t ≤ (18 : ℝ)
  · exact cell_70_global t (le_of_lt (lt_of_not_ge h69)) h70
  by_cases h71 : t ≤ (73 / 4 : ℝ)
  · exact cell_71_global t (le_of_lt (lt_of_not_ge h70)) h71
  by_cases h72 : t ≤ (37 / 2 : ℝ)
  · exact cell_72_global t (le_of_lt (lt_of_not_ge h71)) h72
  by_cases h73 : t ≤ (75 / 4 : ℝ)
  · exact cell_73_global t (le_of_lt (lt_of_not_ge h72)) h73
  by_cases h74 : t ≤ (19 : ℝ)
  · exact cell_74_global t (le_of_lt (lt_of_not_ge h73)) h74
  by_cases h75 : t ≤ (77 / 4 : ℝ)
  · exact cell_75_global t (le_of_lt (lt_of_not_ge h74)) h75
  by_cases h76 : t ≤ (39 / 2 : ℝ)
  · exact cell_76_global t (le_of_lt (lt_of_not_ge h75)) h76
  by_cases h77 : t ≤ (79 / 4 : ℝ)
  · exact cell_77_global t (le_of_lt (lt_of_not_ge h76)) h77
  by_cases h78 : t ≤ (20 : ℝ)
  · exact cell_78_global t (le_of_lt (lt_of_not_ge h77)) h78
  by_cases h79 : t ≤ (41 / 2 : ℝ)
  · exact cell_79_global t (le_of_lt (lt_of_not_ge h78)) h79
  by_cases h80 : t ≤ (21 : ℝ)
  · exact cell_80_global t (le_of_lt (lt_of_not_ge h79)) h80
  by_cases h81 : t ≤ (43 / 2 : ℝ)
  · exact cell_81_global t (le_of_lt (lt_of_not_ge h80)) h81
  by_cases h82 : t ≤ (22 : ℝ)
  · exact cell_82_global t (le_of_lt (lt_of_not_ge h81)) h82
  by_cases h83 : t ≤ (23 : ℝ)
  · exact cell_83_global t (le_of_lt (lt_of_not_ge h82)) h83
  by_cases h84 : t ≤ (24 : ℝ)
  · exact cell_84_global t (le_of_lt (lt_of_not_ge h83)) h84
  by_cases h85 : t ≤ (26 : ℝ)
  · exact cell_85_global t (le_of_lt (lt_of_not_ge h84)) h85
  by_cases h86 : t ≤ (28 : ℝ)
  · exact cell_86_global t (le_of_lt (lt_of_not_ge h85)) h86
  by_cases h87 : t ≤ (32 : ℝ)
  · exact cell_87_global t (le_of_lt (lt_of_not_ge h86)) h87
  by_cases h88 : t ≤ (48 : ℝ)
  · exact cell_88_global t (le_of_lt (lt_of_not_ge h87)) h88
  by_cases h89 : t ≤ (64 : ℝ)
  · exact cell_89_global t (le_of_lt (lt_of_not_ge h88)) h89
  exact cell_90_global t (le_of_lt (lt_of_not_ge h89)) hu

#print axioms finite_archimedean_lower

end XedniDuplicationCells
