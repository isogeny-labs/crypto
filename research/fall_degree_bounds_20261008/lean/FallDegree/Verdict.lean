import FallDegree.Data

/-!
# Fall-degree bounds: the scoring on the recorded draws

`report` is `Rule.lean`; `rows` is generated from the E3, E4, E6 records by
`examples/fall_degree_lean_data.rs`. Kernel `decide`.

Disclosure: `Rule.lean` transcribes rules registered in prose (the
pre-registration and Amendments 2–6) into Lean; the transcription was
written after part of the data had been seen. The prose rules were not.
-/

namespace FallDegree

set_option maxRecDepth 100000
set_option maxHeartbeats 0

theorem e3_rows : (rows.filter (·.kind == 0)).length = 112 := by decide

/-- P3a, P3b (E3), P4a, P4b (E4) hold; every unsatisfiable Semaev draw has
`d_F ≤ 4` and every satisfiable one `d_last ≤ 4` ("bounded at toy size");
the structure-matched controls read `d_last ≥ 5` at every shape
("Semaev-specific"). -/
theorem registered_scoring :
    report rows = { p3a := true, p3b := true, p4a := true, p4b := true,
                    bounded := true, semaevSpecific := true } := by
  decide

end FallDegree
