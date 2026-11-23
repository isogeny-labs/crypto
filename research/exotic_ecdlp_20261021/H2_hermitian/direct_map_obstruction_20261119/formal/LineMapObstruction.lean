import Init

/-!
A line-constant point map on a point-line geometry with the projection
property is constant. The theorem quantifies over the incidence relation;
the concrete Hermitian geometry and the no-map P¹ → E bridge are stated
separately in the accompanying proof map.
-/

universe u v w

namespace H2DirectMap

theorem map_constant_of_projection
    {Point : Type u} {Line : Type v} {Target : Type w}
    (inc : Point → Line → Prop) (f : Point → Target)
    (line_constant : ∀ (l : Line) (p q : Point),
      inc p l → inc q l → f p = f q)
    (point_has_line : ∀ q : Point, ∃ l : Line, inc q l)
    (projection : ∀ (p : Point) (l : Line),
      inc p l ∨ ∃ (r : Point) (m : Line), inc p m ∧ inc r m ∧ inc r l) :
    ∀ p q : Point, f p = f q := by
  intro p q
  obtain ⟨l, hql⟩ := point_has_line q
  rcases projection p l with hpl | ⟨r, m, hpm, hrm, hrl⟩
  · exact line_constant l p q hpl hql
  · exact (line_constant m p r hpm hrm).trans
      (line_constant l r q hrl hql)

#print axioms H2DirectMap.map_constant_of_projection

end H2DirectMap
