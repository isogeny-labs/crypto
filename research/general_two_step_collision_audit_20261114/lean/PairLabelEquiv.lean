import QueryGeometry

namespace GeneralTwoStepCollisionAudit
noncomputable section
variable {G I : Type*} [AddCommGroup G]

theorem query_pairLabels (g : I → G) (h : G → I) (v : G) (xy : G × G)
    (hx : step g h (step g h xy.1) = v) (hy : step g h (step g h xy.2) = v) :
    query g v (pairLabels g h xy) = pairQueries g h xy := by
  funext a
  fin_cases a
  · change v - (g (h xy.1) + g (h (step g h xy.1))) = xy.1
    rw [← hx]
    unfold step
    abel
  · change v - g (h (step g h xy.1)) = step g h xy.1
    rw [← hx]
    unfold step
    abel
  · change v - (g (h xy.2) + g (h (step g h xy.2))) = xy.2
    rw [← hy]
    unfold step
    abel
  · change v - g (h (step g h xy.2)) = step g h xy.2
    rw [← hy]
    unfold step
    abel

theorem pairLabels_admissible (g : I → G) (h : G → I) (v : G) (xy : G × G)
    (hf : FreshAt g h v xy) : Admissible (pairLabels g h xy) := by
  apply admissible_of_query_injective g v
  rw [query_pairLabels g h v xy hf.2.1 hf.2.2]
  exact hf.1

theorem pairLabels_match (g : I → G) (h : G → I) (v : G) (xy : G × G)
    (hx : step g h (step g h xy.1) = v) (hy : step g h (step g h xy.2) = v) :
    Match g h v (pairLabels g h xy) := by
  intro a
  rw [query_pairLabels g h v xy hx hy]
  fin_cases a <;> rfl

def decodePair (g : I → G) (v : G) (w : Labels I) : G × G :=
  (query g v w 0, query g v w 2)

theorem matched_first_steps (g : I → G) (h : G → I) (v : G) (w : Labels I)
    (hm : Match g h v w) :
    step g h (query g v w 0) = query g v w 1 ∧
      step g h (query g v w 2) = query g v w 3 := by
  constructor
  · unfold step
    rw [hm 0]
    dsimp [query, offsets, labelsAt]
    abel
  · unfold step
    rw [hm 2]
    dsimp [query, offsets, labelsAt]
    abel

theorem matched_endpoints (g : I → G) (h : G → I) (v : G) (w : Labels I)
    (hm : Match g h v w) :
    step g h (query g v w 1) = v ∧ step g h (query g v w 3) = v := by
  constructor
  · unfold step
    rw [hm 1]
    dsimp [query, offsets, labelsAt]
    abel
  · unfold step
    rw [hm 3]
    dsimp [query, offsets, labelsAt]
    abel

theorem decoded_pairQueries (g : I → G) (h : G → I) (v : G) (w : Labels I)
    (hm : Match g h v w) : pairQueries g h (decodePair g v w) = query g v w := by
  have hs := matched_first_steps g h v w hm
  funext a
  fin_cases a <;> first | rfl | exact hs.1 | exact hs.2

theorem decoded_fresh (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (h : G → I) (v : G) (w : Labels I) (hw : Admissible w) (hm : Match g h v w) :
    FreshAt g h v (decodePair g v w) := by
  have hs := matched_first_steps g h v w hm
  have he := matched_endpoints g h v w hm
  refine ⟨?_, ?_, ?_⟩
  · rw [decoded_pairQueries g h v w hm]
    exact query_injective g hg v w hw
  · change step g h (step g h (query g v w 0)) = v
    rw [hs.1]
    exact he.1
  · change step g h (step g h (query g v w 2)) = v
    rw [hs.2]
    exact he.2

abbrev PairAt (g : I → G) (h : G → I) (v : G) := {xy : G × G // FreshAt g h v xy}
abbrev WordAt (g : I → G) (h : G → I) (v : G) :=
  {w : Labels I // Admissible w ∧ Match g h v w}

/-- An actual equivalence, with both inverses proved from actual step and
terminal equations. No count/probability/coefficient is a premise. -/
def pairLabelEquiv (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (h : G → I) (v : G) : PairAt g h v ≃ WordAt g h v where
  toFun p := ⟨pairLabels g h p.val, pairLabels_admissible g h v p.val p.property,
    pairLabels_match g h v p.val p.property.2.1 p.property.2.2⟩
  invFun w := ⟨decodePair g v w.val, decoded_fresh g hg h v w.val w.property.1 w.property.2⟩
  left_inv p := by
    apply Subtype.ext
    apply Prod.ext
    · exact congrFun (query_pairLabels g h v p.val p.property.2.1 p.property.2.2) 0
    · exact congrFun (query_pairLabels g h v p.val p.property.2.1 p.property.2.2) 2
  right_inv w := by
    apply Subtype.ext
    have hs := matched_first_steps g h v w.val w.property.2
    apply Prod.ext
    · apply Prod.ext
      · change h (step g h (query g v w.val 0)) = w.val.1.1
        rw [hs.1]
        exact w.property.2 1
      · change h (step g h (query g v w.val 2)) = w.val.1.2
        rw [hs.2]
        exact w.property.2 3
    · apply Prod.ext
      · exact w.property.2 0
      · exact w.property.2 2

#print axioms pairLabelEquiv
end
end GeneralTwoStepCollisionAudit
