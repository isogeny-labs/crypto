# Boundary-evaluation functionals are not the annihilator of the closure (counterexample record)

Instance F1 (`p 12289, A 1210, B 0, t 11`) and the N8 instance (`p 12289, A 4021, B 9917, t 4408`).
Functionals: `λ_P : u ↦ Σ_{(a,b)} u_{ab} ξ^{ℓ(a)} η^{ℓ(b)}` for the points `P = (ξ, η)` of `S_3(x1, x2, t) = 0`
over `F_{p²}` with one coordinate in `μ_N` (`4N` points), with lift `ℓ`: standard `[0, N)`, balanced
`(−N/2, N/2]` in both coordinates, or balanced only in the non-`μ_N` coordinate. Recorded: number of
basis rows of `V_D` (engine echelon basis) that some functional does not annihilate (engine `evaltest`).

| instance | k | D | rows | not annihilated: standard | balanced both | balanced one | 1 ∈ V_D |
|---|---|---|---|---|---|---|---|
| N8 | 2 | 3 | 5 | 2 | – | – | no |
| N8 | 3..9 | 3 | 2k+1 | 0 | – | – | no |
| N8 | 4 | 4 | 163 | 107 | – | – | yes |
| N8 | 5 | 4 | 386 | 203 | – | – | yes |
| N8 | 6 | 4 | 605 | **188** | – | – | **no** |
| N8 | 7 | 4 | 829 | 0 | – | – | no |
| N8 | 8 | 4 | 1061 | 0 | – | – | no |
| N8 | 6 | 5 | 1586 | 1169 | – | – | yes |
| F1 | 5 | 4 | 386 | 203 | 326 | 326 | yes |
| F1 | 6 | 4 | 605 | 188 | 415 | 415 | no |
| F1 | 7 | 4 | 829 | 0 | 403 | 403 | no |
| F1 | 6 | 3 | 13 | 0 | 2 | 2 | no |

Reading: at `k = 6, D = 4` the closure does not contain `1` yet is not annihilated by the standard-lift
evaluations (188 of 605 rows fail), so `U_4(6)^⊥` is not spanned by these functionals; the balanced
lifts are worse everywhere. The functionals do annihilate `V_3` for `k ≥ 3` and `V_4` for `k = 7, 8`.
Source: `scratchpad/runs/explore1.jsonl` and the F1 `evaltest` runs (2026-11-15).
