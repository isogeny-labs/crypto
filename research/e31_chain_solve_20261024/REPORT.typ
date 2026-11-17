// E31 report.  Every number below is read from summary.json, which
// figures/plot.py writes from the raw records; registered bands (G2.85-G2.94)
// are quoted from the pre-registration (note §9.22) and labelled as such.
#let S = json("summary.json")
#let SA = S.stage_a
#let EE = S.end_to_end
#let CX = S.cross_checks
#let G = S.gae_fp_muls.at(0)

#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set table(inset: 3.5pt, stroke: 0.4pt + luma(170))
#show table: set text(size: 7.6pt)

// ── number formatting ──
#let fix(x, d) = {
  let s = str(calc.round(x, digits: d))
  if d == 0 { return s }
  if not s.contains(".") { s = s + "." }
  let have = s.split(".").at(1).len()
  s + "0" * (d - have)
}
#let sep(n) = {
  let s = str(int(n))
  let out = ""
  let k = s.len()
  for (i, ch) in s.clusters().enumerate() {
    if i > 0 and calc.rem(k - i, 3) == 0 { out += "," }
    out += ch
  }
  out
}
#let sci(x, d: 2) = {
  let e = int(calc.floor(calc.log(x, base: 10)))
  let m = x / calc.pow(10.0, e)
  if calc.round(m, digits: d) >= 10 { m = m / 10; e = e + 1 }
  $#fix(m, d) dot 10^#e$
}
#let rg(p, d) = [#fix(p.at(0), d)–#fix(p.at(1), d)]
#let rgs(p) = [#sci(p.at(0))–#sci(p.at(1))]
#let pct(p, d) = [#fix(100 * p.at(0), d)–#fix(100 * p.at(1), d) %]

#align(center)[
  #text(size: 14pt, weight: "bold")[E31: the chain solve for the $k = 4$ $V_4$-line decomposition]
  #v(3pt)
  Goal G2, `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.22 (pre-registration G2.85–G2.94), §9.23 (results) · 2026-10-24 \
  *Result class: #S.class* · binary #raw(S.binary_commit.slice(0, 10))
]

= Result

E31 adds a second solver for one $k = 4$ decomposition on the $V_4$ line (the degree-11 system in
the invariants of $V_4^4 _0 times.r S_4$), beside E25’s unchanged Macaulay solver. It works in the
quotient as a 12-generator module over $FF_p [P_2]$: left-looking normal forms of the 12 chain
ends, $det R(x)$ of a $12 times 12$ polynomial matrix, kernels of $R(lambda)$. *A* is that solver;
*B* is A with the sparsest pivot rows in the echelon. On the same #sep(SA.calls_total) calls (Stage A,
#SA.curves curves) both returned the existing solver's decompositions on every call
(#sep(SA.agree_total_A_B.at(0)) / #sep(SA.calls_total) and #sep(SA.agree_total_A_B.at(1)) / #sep(SA.calls_total)),
and the per-call median $C$ fell by *#rg(SA.existing_over_A_range, 2)$times$ (A)* and
*#rg(SA.existing_over_B_range, 2)$times$ (B)* counted $FF_p$ multiplications.

End to end on E25’s ten curves, both arms, the same target streams (#EE.records records), every
relation, logarithm and walk is unchanged and $S$ falls by *#rg(EE.E25_over_A_range_by_weight.at(0), 2)$times$
(A)* and *#rg(EE.E25_over_B_range_by_weight.at(0), 2)$times$ (B)* at 0 GAE. Priced at the §5a band
(one GAE $= #fix(G, 0)$ $FF_p$ multiplications per Macaulay-matrix entry and per walk step), the
factor is #rg(EE.E25_over_A_range_by_weight.at(1), 2) (A) and #rg(EE.E25_over_B_range_by_weight.at(1), 2)
(B) at 0.1 GAE, and #rg(EE.E25_over_A_range_by_weight.at(2), 3) (A) and
#rg(EE.E25_over_B_range_by_weight.at(2), 3) (B) at 1 GAE: E31 stays below E25 on every row at every
weight (#if EE.below_E25_every_row_every_weight [yes] else [*no*]). What E31 cut is the arithmetic,
not the Macaulay matrix the solve still builds; the exponents are unchanged. *Class: #S.class*, a
verified, measured improvement of the index-calculus baseline under the 2026-10-15 criterion.

*Distance remaining.* The matched $chevron.l plus.minus psi chevron.r$ rho is still ahead of every row
of both solvers at every weight (#if EE.rho_ahead_every_row_every_weight [yes] else [*no*]). At 0 GAE,
B's rows are #rgs(EE.S_over_rho_B_range_by_weight.at(0))$times$ the matched rho
(best #sci(EE.best_S_over_rho_0gae.B.S_over_rho)$times$ at $p = #EE.best_S_over_rho_0gae.B.p$ seed
#EE.best_S_over_rho_0gae.B.seed, arm #(EE.best_S_over_rho_0gae.B.arm)\; E25’s best on the same curve was
#sci(EE.best_S_over_rho_0gae.E25.S_over_rho)$times$); at 0.1 GAE #rgs(EE.S_over_rho_B_range_by_weight.at(1))$times$;
at 1 GAE #rgs(EE.S_over_rho_B_range_by_weight.at(2))$times$.

#figure(image("figures/distance_to_rho.svg", width: 100%), caption: [$S_"IC" slash S_rho$ per curve and arm,
E25 (existing solver) against E31 A and B, each at 0 (circle), 0.1 (triangle) and 1 GAE (square) per
unpriced native operation. $S_rho$ is the mean of 16 matched $chevron.l plus.minus psi chevron.r$ walks per
curve (`rho_matched_muls`, `rho_matched_steps`). Records: `experiments/30_gls_v4/p*s*.json`,
`experiments/31_solve_cost/e2e_{chain,chain-sparsest}_p*s*.json`.])

= What ran

*Stage A.* `examples/e31_stage_a.rs`, one thread: the first 1,000 targets of E25 arm A's stream on each
of E25’s ten GLS curves ($p = 101$–$397$, two seeds), the three solvers interleaved per call from the same
per-target RNG seed. Record: `experiments/31_solve_cost/stage_a.json`. *End to end.*
`experiments/31_solve_cost/run_e2e.sh` (E25’s `run_all.sh` plus `--solver`, `RAYON_NUM_THREADS=4`),
both solvers, both arms (A: the $V_4$ fold; B: the $chevron.l V_4, psi chevron.r$ fold), all ten curves,
16 rho walks per curve. Binary #raw(S.binary_commit) for both. Tables: `experiments/31_solve_cost/tables.md`
(`examples/e31_tables.rs`). Host: one shared 4-vCPU cloud container, isolation L0. Every cost is an
operation count ($FF_p$ multiplications; a mod-$n$ multiplication at 16); wall times are a practicality
note only. Derived values (ratios, the band) are computed from the recorded fields exactly as
`e31_tables.rs` does: $S(w) = "total_muls" + w dot G dot "macaulay_entries"$, $rho(w) =
"rho_matched_muls" + w dot G dot "rho_matched_steps"$, $G = #fix(G, 0)$.

= Stage A: one call, three solvers

#table(
  columns: 12,
  align: right,
  [*p*], [*seed*], [*C med. existing*], [*C med. A*], [*C med. B*], [*exist. / A*], [*exist. / B*],
  [*NF exist. / A*], [*charpoly exist. / A*], [*eigvec exist. / A*], [*echelon A / B*], [*fallbacks A, B*],
  ..SA.rows.map(r => (
    str(r.p), str(r.seed), sep(r.c_median.macaulay), sep(r.c_median.chain), sep(r.c_median.at("chain-sparsest")),
    fix(r.existing_over_A, 2), fix(r.existing_over_B, 2), fix(r.nf_existing_over_A, 1),
    fix(r.charpoly_existing_over_A, 2), fix(r.eigvec_existing_over_A, 1), fix(r.echelon_A_over_B, 2),
    [#r.fallbacks_A_B.at(0), #r.fallbacks_A_B.at(1)],
  )).flatten()
)

$C$ = Weil restriction + solve per call; phase ratios are of per-call means over all calls, fallbacks
charged. $M_(P_2)$ and the characteristic polynomial agreed on #sep(SA.quotient_agree_total_A_B.at(0)) of
#sep(SA.quotient_checked_total_A_B.at(0)) calls the existing solver does not refuse. Fallbacks (charged reruns
of the existing solver): #pct(SA.fallback_rate_A_range, 1) of calls per curve. Wall median existing / A:
#rg(SA.wall_existing_over_A_range, 2).

#figure(image("figures/per_call_cost.svg", width: 80%), caption: [Median (bar) and mean (×) $C$ per call, 1,000 calls per curve, three solvers on the same calls; `experiments/31_solve_cost/stage_a.json`.])

#figure(image("figures/phase_breakdown.svg", width: 80%), caption: [Phase means per call (all calls, fallbacks charged): existing, A, B left to right per curve; `stage_a.json`.])

= End to end: E25’s curves rerun

#table(
  columns: 11,
  align: right,
  [*p*], [*seed*], [*arm*], [*$S$ E25*], [*$S$ A*], [*$S$ B*], [*E25 / A*], [*E25 / B*],
  [*$S slash rho$ E25*], [*$S slash rho$ A*], [*$S slash rho$ B*],
  ..EE.rows.map(r => (
    str(r.p), str(r.seed), r.arm, sci(r.S_E25), sci(r.S_A), sci(r.S_B),
    fix(r.E25_over_A.at(0), 2), fix(r.E25_over_B.at(0), 2),
    sci(r.S_over_rho_E25.at(0)), sci(r.S_over_rho_A.at(0)), sci(r.S_over_rho_B.at(0)),
  )).flatten()
)

At 0 GAE. Replay: every E31 record matches its E25 record on the fields #raw(S.replay_fields_checked.join(", ")) (#if EE.all_replay_fields_identical [all identical] else [*differences*]);
fallbacks equal E25’s `reruns + unsolved` in every record (#if EE.all_fallbacks_eq_reruns_plus_unsolved [yes] else [*no*]);
every log and walk correct (#if EE.all_correct [yes] else [*no*]). Weil + solves are #pct(EE.solve_share_A_range, 1)
of $S$ (A) and #pct(EE.solve_share_B_range, 1) (B).

#figure(image("figures/s_by_curve.svg", width: 88%), caption: [$S$ per curve, E25 against E31 A and B, both arms, 0 GAE.])

== The §5a band (ranges over all 20 curve–arm rows)

#table(
  columns: 6,
  align: (left, right, right, right, right, right),
  [*GAE per native op*], [*$S_"E25" slash S_A$*], [*$S_"E25" slash S_B$*], [*$S slash rho$ E25*], [*$S slash rho$ A*], [*$S slash rho$ B*],
  ..range(3).map(i => (
    str(S.weights_gae.at(i)),
    rg(EE.E25_over_A_range_by_weight.at(i), if i == 2 { 3 } else { 2 }),
    rg(EE.E25_over_B_range_by_weight.at(i), if i == 2 { 3 } else { 2 }),
    rgs(EE.S_over_rho_E25_range_by_weight.at(i)), rgs(EE.S_over_rho_A_range_by_weight.at(i)),
    rgs(EE.S_over_rho_B_range_by_weight.at(i)),
  )).flatten()
)

The order E31 < E25 holds at every weight on every row, and so does rho < E31: both rankings are
admitted by §5a. The size of the E31 gain is not band-stable (it shrinks to a few percent at 1 GAE).

= Predictions and verdicts

Registered bands are quoted from §9.22; measured values from the records.

#table(
  columns: 3,
  align: left,
  [*id*], [*registered*], [*measured → verdict*],
  [G2.85], [identical outputs on every call], [Stage A #sep(SA.agree_total_A_B.at(0)) / #sep(SA.calls_total) (A), #sep(SA.agree_total_A_B.at(1)) / #sep(SA.calls_total) (B); end to end identical → holds],
  [G2.86], [NF existing / A in $[12, 40]$], [#rg(SA.nf_existing_over_A_range, 1) (fallbacks charged) → falsified low],
  [G2.87], [charpoly existing / A in $[1.8, 3.2]$], [#rg(SA.charpoly_existing_over_A_range, 2) → holds],
  [G2.88], [eigenvectors existing / A in $[15, 80]$], [#rg(SA.eigvec_existing_over_A_range, 1) → holds],
  [G2.89], [$C$ median existing / A in $[2.2, 3.0]$; kill $< 1.5$], [#rg(SA.existing_over_A_range, 2) → holds],
  [G2.90], [fallbacks $<= 1 %$ of calls], [Stage A #pct(SA.fallback_rate_A_range, 1)\; end to end #pct(CX.e2e_fallback_rate_range_all_40_records, 2) → falsified at $p = 101$],
  [G2.91], [wall existing / A in $[1.2, 2.0]$], [#rg(SA.wall_existing_over_A_range, 2) → holds],
  [G2.92], [echelon A / B in $[1.0, 1.6]$ (exploratory)], [#rg(SA.echelon_A_over_B_range, 2) → falsified high; B carried end to end],
  [G2.93], [$S_"E25" slash S_A$ in $[2.1, 2.9]$, counts reproduced], [#rg(EE.E25_over_A_range_by_weight.at(0), 2) → holds],
  [G2.94], [$[1.3, 1.6]$ at 0.1, $[1.03, 1.10]$ at 1 GAE; A below E25], [#rg(EE.E25_over_A_range_by_weight.at(1), 2) and #rg(EE.E25_over_A_range_by_weight.at(2), 3) → holds],
)

= Cross-check of §9.23 against the records

Every value in §9.23’s Stage A and end-to-end tables, the §5a ranges, the agreement counts and the
§9.22 profile table agree with the JSON records at the precision printed. Secondary statements, re-read
from the records (`summary.json`, `cross_checks`):

- *Fallback rate per end-to-end run* (§9.23 G2.90 row and "next levers": "`0.2`–`1.1 %`"): the 40
  records give #pct(CX.e2e_fallback_rate_range_all_40_records, 2) (fallbacks / calls), i.e. about
  0.3–1.2 %. The upper end is 1.2 %, not 1.1 % (e.g. 85 / 7,106 at $p = 101$ seed 2 arm B,
  488 / 41,043 at $p = 101$ seed 1 arm A). Verdict unchanged.
- *Cost of a fallback call to the existing solver* ("`4.5`–`4.8·10⁶`"): the Stage A calls identified as
  fallbacks (rule: #CX.stage_a_fallback_rule; finds #CX.stage_a_fallbacks_found_of_recorded.at(0) of
  #CX.stage_a_fallbacks_found_of_recorded.at(1) recorded) cost #rgs(CX.stage_a_fallback_call_existing_cost_range).
  The chain attempt adds #fix(100 * CX.stage_a_fallback_chain_extra_over_existing_mean, 1) % (§9.23: ≈ 13 %).
- *A's echelon share* ("`≈ 70 %`"): #pct(CX.stage_a_A_echelon_share_of_mean_C_range, 1) of the mean $C$;
  charpoly #pct(CX.stage_a_A_charpoly_share_of_mean_C_range, 1) (§9.23: ≈ 15 %).
- *Fallback weight in B at $p = 101$* ("up to `≈ 16 %` of B's mean $C$"): the identified fallback calls
  carry at most #fix(100 * CX.stage_a_fallback_share_of_B_multiplications_range.at(1), 1) % of B's
  Stage A multiplications (approximate: Stage A records no per-call fallback flag).
- Clean-call normal forms: A #rgs(CX.stage_a_clean_call_nf_mean_A_range), B #rgs(CX.stage_a_clean_call_nf_mean_B_range) (§9.23: `65.6`–`66.8·10³`, `31.6`–`32.2·10³`): agree.
- "every row is still `2.39·10³`–`1.52·10⁵×`" is B's range; A's rows reach #sci(EE.S_over_rho_A_range_by_weight.at(0).at(1))$times$.

= Scope and limits

E25’s GLS twists over $FF_(p^4)$, $p <= #calc.max(..EE.rows.map(r => r.p))$, $n <= 2^(#fix(calc.max(..EE.rows.map(r => r.log2_n)), 1))$, the degree-11 solve; E24’s curves use the
same solver but were not rerun end to end. Operation counts only, on an L0 shared host. The §11.16
handover was not recomputed. Validation gate: the replay certificate, validator report and
`VALIDATION.json` for these rows are prepared separately in `benchmarks/20261024-e31-chain-solve/`; until
they land, the rows are unvalidated and do not enter the leaderboard.

= Records

Raw: `experiments/31_solve_cost/` (`stage_a.json`, `e2e_chain_p*s*.json`, `e2e_chain-sparsest_p*s*.json`,
logs, `profile_p269_p397.json`), `experiments/30_gls_v4/p*s*.json` (E25). Code:
`klein_v4::solve_weighted_chain` beside `solve_weighted_traced`; runners `examples/e31_stage_a.rs`,
`examples/quartic_pipeline_gls.rs --solver`, `examples/e31_tables.rs`. Figures and this report's numbers:
`research/e31_chain_solve_20261024/figures/plot.py` → `summary.json`.
