import FallDegree.Data

/-!
# Fall-degree bounds: the scoring on the recorded draws

`report` is `Rule.lean`; `rows` is generated from the E3, E4, E6, E7, E8, E9 and E10 records by
`examples/fall_degree_lean_data.rs`. Kernel `decide`.

Disclosure: `Rule.lean` transcribes rules registered in prose (the
pre-registration and Amendments 2–8) into Lean; the transcription was
written after part of the data had been seen. The prose rules were not.
-/

namespace FallDegree

set_option maxRecDepth 100000
set_option maxHeartbeats 0

theorem e3_rows : (rows.filter (·.kind == 0)).length = 112 := by decide

/-- P3a, P3b (E3), P4a, P4b (E4) hold; every unsatisfiable Semaev draw has
`d_F ≤ 4` and every satisfiable one `d_last ≤ 4` ("bounded at toy size");
the structure-matched controls read `d_last ≥ 5` at every shape
("Semaev-specific"); every E7 draw (m = 5, 6; N = 44–54) has `d_F ≤ 4`
and no E7 cell has median `d_F ≥ 5` ("bounded through m = 6"). E8 (Amendment 9): the `(8, 2)` ladder, `m = 3 … 7`, reads
"flat"; the `(6, 2)` ladder, `m = 3 … 8`, is neither "flat" nor "grows", i.e. "mixed",
because its median falls to 3 at `m = 8`. E9 (Amendment 10): every large-host draw
(`N = 58, 60`) has `d_F ≤ 4` ("bounded"). E10 (Amendment 10), along `mℓ = n`: the
`m = 3` diagonal "grows" (every draw of (21, 7), (24, 8), (27, 9) is censored above 4);
the `m = 4, 5, 6` diagonals are "flat"; cross-`m` at `N = 48` and `N = 60` is
"not rising". -/
theorem registered_scoring :
    report rows = { p3a := true, p3b := true, p4a := true, p4b := true,
                    bounded := true, semaevSpecific := true,
                    e7Bounded := true, e7Grows := false,
                    e8Flat82 := true, e8Grows82 := false, e8Flat62 := false, e8Grows62 := false,
                    e9Bounded := true, e9Grows := false,
                    e10Flat3 := false, e10Grows3 := true,
                    e10Flat4 := true, e10Grows4 := false,
                    e10Flat5 := true, e10Grows5 := false,
                    e10Flat6 := true, e10Grows6 := false,
                    e10RisesN48 := false, e10RisesN60 := false } := by
  decide

end FallDegree
