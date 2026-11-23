import Lean

/-! Core-Lean checks for the unequal-bound reach bound (README sections 2, 3 and 6).

HYPOTHESIS CARRIED BY EVERY e-FORM INSTANCE BELOW: the polynomial has the stated *extent* `W_j` in
coordinate `e_j` (for the headline closure, the full extent `D = 2^(m-1)` of the generic specialization).
The instances take Theorem K at that extent (`W_j * E <= c_j`) as an explicit assumption; they say
nothing about a specialization with smaller extents, for which `eform_S3_ext` gives the weaker ceiling
and `z0_*` below records an exact example that exceeds the generic ceiling 1/6.

What is checked here, core Lean only (no Mathlib):
* `pairs_le_sum`: the counting lemma at the end of the proof of Theorem K,
  `w * #{pairs of a set S of naturals differing by a positive multiple of w} <= sum S`.
* `reach_le`: from Theorem K's conclusion `W_i * E <= c_i` for every coordinate and the
  Howgrave-Graham inequality `sum_i n_i c_i <= D * E`, the reach inequality `sum_i n_i W_i <= D`
  (that is, `eps <= 1 / (sum_i dir_i W_i)` for every bound direction).
* the arithmetic instances for the symmetric e-form of S_3, S_4, S_5.

NOT checked here: Theorem K itself (the fibre-injection step, the layer-cake step, the
Newton-polytope containment from Ostrowski's lemma), the layer bound on `E(M)` inherited from
research/smallroot_optimal_reach_20261026, and the e-form supports (computed in PARI/GP). -/
namespace EFormReach

/-- Pairs `(a, b)` of a descending list with `a` earlier than `b` and `w ∣ a - b`. -/
def pairs (w : Nat) : List Nat → Nat
  | [] => 0
  | a :: l => (l.filter (fun b => decide (w ∣ a - b))).length + pairs w l

/-- Elements of one residue class below `b`, listed in descending order, number at most `b / w`. -/
theorem class_count_le (w : Nat) (hw : 0 < w) :
    ∀ (l : List Nat) (b : Nat), l.Pairwise (· > ·) →
      (∀ a ∈ l, a < b ∧ w ∣ b - a) → w * l.length ≤ b
  | [], b, _, _ => by simp
  | x :: l, b, hp, h => by
    have hx := h x (by simp)
    have hle : w ≤ b - x := Nat.le_of_dvd (by omega) hx.2
    have hp' := List.pairwise_cons.1 hp
    have ih := class_count_le w hw l x hp'.2 (fun a ha => by
      have hxa : a < x := hp'.1 a ha
      have ha' := h a (List.mem_cons_of_mem _ ha)
      refine ⟨hxa, ?_⟩
      have hsub : w ∣ (b - a) - (b - x) := Nat.dvd_sub ha'.2 hx.2
      have : (b - a) - (b - x) = x - a := by omega
      rwa [this] at hsub)
    simp only [List.length_cons, Nat.mul_add, Nat.mul_one]
    omega

/-- Counting lemma: in a strictly descending list, `w` times the number of pairs at a positive
multiple of `w` apart is at most the sum. -/
theorem pairs_le_sum (w : Nat) (hw : 0 < w) :
    ∀ l : List Nat, l.Pairwise (· > ·) → w * pairs w l ≤ l.sum
  | [], _ => by simp [pairs]
  | b :: l, hp => by
    have hp' := List.pairwise_cons.1 hp
    have ih := pairs_le_sum w hw l hp'.2
    have hc := class_count_le w hw (l.filter (fun a => decide (w ∣ b - a))) b
      (hp'.2.filter _) (fun a ha => by
        rw [List.mem_filter] at ha
        exact ⟨hp'.1 a ha.1, by simpa using ha.2⟩)
    simp only [pairs, List.sum_cons, Nat.mul_add]
    omega

/-- Triples `(n, W, c)`: `delta_i = n_i / D`, the extent `W_i` of the Newton polytope of `f` in
coordinate `i`, and the weight `c_i = sum_{mu in M} mu_i`.  The key inequality of Theorem K
(`W_i * E <= c_i`) and the Howgrave-Graham inequality (`sum_i n_i c_i <= D * E`) imply
`sum_i n_i W_i <= D`. -/
theorem reach_aux (E : Nat) :
    ∀ l : List (Nat × Nat × Nat), (∀ t ∈ l, t.2.1 * E ≤ t.2.2) →
      E * (l.map (fun t => t.1 * t.2.1)).sum ≤ (l.map (fun t => t.1 * t.2.2)).sum
  | [], _ => by simp
  | t :: l, h => by
    have h1 := h t (by simp)
    have ih := reach_aux E l (fun s hs => h s (List.mem_cons_of_mem _ hs))
    simp only [List.map_cons, List.sum_cons, Nat.mul_add]
    have : E * (t.1 * t.2.1) ≤ t.1 * t.2.2 := by
      calc E * (t.1 * t.2.1) = t.1 * (t.2.1 * E) := by ac_rfl
        _ ≤ t.1 * t.2.2 := Nat.mul_le_mul_left _ h1
    omega

theorem reach_le (E D : Nat) (hE : 0 < E) (l : List (Nat × Nat × Nat))
    (key : ∀ t ∈ l, t.2.1 * E ≤ t.2.2)
    (hg : (l.map (fun t => t.1 * t.2.2)).sum ≤ D * E) :
    (l.map (fun t => t.1 * t.2.1)).sum ≤ D := by
  have h := reach_aux E l key
  have : E * (l.map (fun t => t.1 * t.2.1)).sum ≤ E * D := by
    calc E * (l.map (fun t => t.1 * t.2.1)).sum ≤ (l.map (fun t => t.1 * t.2.2)).sum := h
      _ ≤ D * E := hg
      _ = E * D := Nat.mul_comm _ _
  exact Nat.le_of_mul_le_mul_left this hE

/-! Instances for the symmetric e-form: `delta_{e_j} = j * u / Dn` (`u / Dn` is `log_p B`).
The `k_j` hypotheses are Theorem K at extent `W_j = D = 2^(m-1)` (S_3: 2, S_4: 4, S_5: 8), which holds
only for specializations whose e-form support has these full extents (GP, README section 3; the
exceptional specializations are listed in README section 3a).  They are assumptions here, not
consequences of the lemmas above. -/

theorem eform_S3 (u Dn E c1 c2 : Nat) (hE : 0 < E)
    (k1 : 2 * E ≤ c1) (k2 : 2 * E ≤ c2)
    (hg : u * c1 + (2 * u) * c2 ≤ Dn * E) : 6 * u ≤ Dn := by
  have := reach_le E Dn hE [(u, 2, c1), (2 * u, 2, c2)] (by simp; omega) (by simpa [Nat.add_assoc] using hg)
  simp at this; omega

theorem eform_S4 (u Dn E c1 c2 c3 : Nat) (hE : 0 < E)
    (k1 : 4 * E ≤ c1) (k2 : 4 * E ≤ c2) (k3 : 4 * E ≤ c3)
    (hg : u * c1 + (2 * u) * c2 + (3 * u) * c3 ≤ Dn * E) : 24 * u ≤ Dn := by
  have := reach_le E Dn hE [(u, 4, c1), (2 * u, 4, c2), (3 * u, 4, c3)] (by simp; omega) (by simpa [Nat.add_assoc] using hg)
  simp at this; omega

theorem eform_S5 (u Dn E c1 c2 c3 c4 : Nat) (hE : 0 < E)
    (k1 : 8 * E ≤ c1) (k2 : 8 * E ≤ c2) (k3 : 8 * E ≤ c3) (k4 : 8 * E ≤ c4)
    (hg : u * c1 + (2 * u) * c2 + (3 * u) * c3 + (4 * u) * c4 ≤ Dn * E) : 80 * u ≤ Dn := by
  have := reach_le E Dn hE [(u, 8, c1), (2 * u, 8, c2), (3 * u, 8, c3), (4 * u, 8, c4)]
    (by simp; omega) (by simpa [Nat.add_assoc] using hg)
  simp at this; omega

/-- The targets 1/4 (S_4) and 1/5 (S_5) are out of reach: `u / Dn >= 1/4` contradicts `24 u <= Dn`. -/
theorem s4_target_unreachable (u Dn : Nat) (hu : 0 < u) (h : 24 * u ≤ Dn) : ¬ Dn ≤ 4 * u := by omega
theorem s5_target_unreachable (u Dn : Nat) (hu : 0 < u) (h : 80 * u ≤ Dn) : ¬ Dn ≤ 5 * u := by omega

/-- S_3 with arbitrary extents `(W1, W2)`: the ceiling is `(W1 + 2*W2) * u <= Dn`.
Generic `z`: `W = (2, 2)` gives 6; the exceptional specialization `z = 0` has `W = (1, 2)`, giving 5. -/
theorem eform_S3_ext (u Dn E c1 c2 W1 W2 : Nat) (hE : 0 < E)
    (k1 : W1 * E ≤ c1) (k2 : W2 * E ≤ c2)
    (hg : u * c1 + (2 * u) * c2 ≤ Dn * E) : (W1 + 2 * W2) * u ≤ Dn := by
  have := reach_le E Dn hE [(u, W1, c1), (2 * u, W2, c2)] (by simp; omega) (by simpa [Nat.add_assoc] using hg)
  simp at this
  have e : (W1 + 2 * W2) * u = u * W1 + 2 * u * W2 := by
    rw [Nat.add_mul]; ac_rfl
  omega

/-! The review example (PR #2900), exact integer facts.  Region `R = {(i, j) : 2i + j <= 6}` in the
e-variables, level `t = 3`.  `Fgen` is the exponent set of the generic S_3 e-form (full simplex of
degree 2); `Fz0` is that of the exceptional specialization `z = 0`, `T = (e2 - a)^2 - 4 b e1`.
`etr` is the translate count `sum_{s=1}^{3} #{y : y + s * nu in R for all nu in F}`; for a convex `M`
it equals the exact lattice gain `E(M)` (README addendum). -/

def inR (i j : Nat) : Bool := decide (2 * i + j ≤ 6)
def okAt (F : List (Nat × Nat)) (s y1 y2 : Nat) : Bool :=
  F.all (fun v => inR (y1 + s * v.1) (y2 + s * v.2))
def layer (F : List (Nat × Nat)) (s : Nat) : Nat :=
  ((List.range 7).flatMap (fun y1 => (List.range 7).filter (fun y2 => okAt F s y1 y2))).length
def etr (F : List (Nat × Nat)) : Nat := layer F 1 + layer F 2 + layer F 3
def Fgen : List (Nat × Nat) := [(2, 0), (0, 1), (1, 1), (1, 0), (0, 0), (0, 2)]
def Fz0 : List (Nat × Nat) := [(0, 0), (0, 1), (0, 2), (1, 0)]
def pts : List (Nat × Nat) :=
  (List.range 7).flatMap (fun i => ((List.range 7).filter (fun j => inR i j)).map (fun j => (i, j)))

theorem pts_count : pts.length = 16 := by decide
theorem c1_val : (pts.map (fun p => p.1)).sum = 14 := by decide
theorem c2_val : (pts.map (fun p => p.2)).sum = 34 := by decide
theorem etr_gen : etr Fgen = 4 := by decide
theorem etr_z0 : etr Fz0 = 14 := by decide

/-- For `z = 0` the gain is 14, so the determinant bound is `delta_B < 14/82 = 7/41`, and
`6 * 14 > 14 + 2 * 34` says that `7/41 > 1/6`: the generic ceiling 1/6 does not hold for `z = 0`. -/
theorem z0_exceeds_generic_ceiling : 6 * 14 > 14 + 2 * 34 := by decide

/-- ... but it respects the ceiling for its own extents `W = (1, 2)`: Theorem K's hypotheses hold and
`(1 + 2 * 2) * 14 <= 14 + 2 * 34`, i.e. `delta_B <= 1/5`. -/
theorem z0_respects_own_ceiling : 1 * 14 ≤ 14 ∧ 2 * 14 ≤ 34 ∧ (1 + 2 * 2) * 14 ≤ 14 + 2 * 34 := by decide

end EFormReach
