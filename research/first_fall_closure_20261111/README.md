# First-fall-degree closure items (2026-11-11)

Three small items that close the remaining first-fall-degree questions of the
fall-degree ledger (`research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`).
Owner decision 2026-11-11: no more measurement time on the first fall degree;
these were run in parallel (Sonnet subagents) with the last-fall-degree compute
plan (`research/fall_degree_bounds_20261008/PREREGISTRATION.md`, Amendment 20).
Nothing here is a cost, security or leaderboard result. All numbers `[pcc:unverified]`.

| directory | item | outcome |
|:--|:--|:--|
| `propA_lean/` | Prop. A (Kosters–Yeo trace relation, `m = 2`) in Lean | `TraceRelation.lean`: identity, span and affine-in-bits statements machine-checked; axioms `propext`, `Classical.choice`, `Quot.sound`; no `sorry` (`check.log`). Degree/first-fall notions and the concrete `GF(2^n)` trace are not formalised. |
| `propB_exact_count/` | exact count `n' − 1 + Tr(b/x_3²)` of degree-≤1 relations for the subfield base | PROVED (prose), with the exceptional set `{x_3 ∈ V*}` characterised and the E1 outlier explained (`x_3 ∈ V`, `b ∉ V`, count 2). 122,752 pairs checked at `n' = 2…6`, 0 disagreements; all 40 E1 draws recomputed. |
| `kw_sharpness/` | sharpness of Kousidis–Wiemers `d_ff ≤ m² − m + 1` at `m ≥ 3` | `m = 3` reduced to a rank condition (Theorem A, PROVED given KW Lemma 3.1): top-form `d_ff ∈ {6, 7}`, `= 7` iff `dim_F2 span{e_p e_q e_r} = dim V` with `e_ab = ν_a ν_b (ν_a + ν_b)`. Not sharp when `n > ρ(n')` (`ρ(3..7) = 10, 49, 170, 475, 1141`; `ρ(3..5)` re-derived by the main session) or `V` in a proper subfield: there `d_ff = 6`. MEASURED 7 on all 1,276 independent instances incl. every KW `m = 3` cell (n = 13…21). The generic case `n ≤ ρ(n')` (KW's regime) stays a CONJECTURE with a named obstruction; `m ≥ 4` is a sketch. |

Lean toolchain used: Lean 4.34.1 with the Mathlib checkout of
`research/guarded_fall_degree_20261028/formal` (rev `d13f23b7`); check with
`cd research/guarded_fall_degree_20261028/formal && lake env lean ../../first_fall_closure_20261111/propA_lean/TraceRelation.lean`.
Conductor was not installed and the research-kb server was unavailable in the
authoring session; each note records that under "Sources checked".
