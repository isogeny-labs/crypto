#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.2pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
#show figure.caption: it => it.body

#let S = json("summary_edwards.json")
#let sci(x, d: 2) = {
  if x == none { return "—" }
  let v = float(x)
  if v == 0 { return "0" }
  let e = calc.floor(calc.log(calc.abs(v), base: 10))
  let m = v / calc.pow(10.0, e)
  [#calc.round(m, digits: d)·10#super[#e]]
}
#let r2(x, d: 2) = if x == none { "—" } else { str(calc.round(float(x), digits: d)) }
#let done = S.dlp.filter(d => d.complete)

= $k = 5$, 2-torsion Edwards route, end to end with F4 trace replay

2026-10-20 · `research/joux_vitse_n1_20261015/` · protocol `PROTOCOL_EDWARDS.md` (frozen at `3a8599b3f`) and its deviations `data/edwards/DEVIATION.md` · code `src/cryptanalysis/joux_vitse_edwards.rs`, `examples/joux_vitse_edwards.rs`, `examples/joux_vitse_edwards_summary.rs` · construction unchanged from `jv_quintic_edwards.rs` (RESEARCH_K5_TORSION_JOUX_VITSE.md). *Class: engineering* (a constant on the existing Edwards route); first end-to-end baseline for it. Checks failed: *#S.checks_failed*.

== Verdict

+ *Trace replay cuts the 2-torsion Edwards test by 1.74–1.77×* ($5.85 dot 10^7 -> 3.3$–$3.4 dot 10^7$ `F_p` multiplications), on 128 paired residuals; the note priced it at $3$–$10 times$. A verified stage improvement.
+ *End to end*, five complete DLPs (`p = 31, 41, 61, 71`; $log_2 n = 22.8$–$28.7$) are #sci(calc.min(..done.map(d => float(d.ratio_online_ic_over_rho)))) – #sci(calc.max(..done.map(d => float(d.ratio_online_ic_over_rho)))) times rho on the same target, every logarithm verified; a sixth arm stopped at its 10-hour cap with 3 of 7 relations, already $1.5 dot 10^7$ times rho. The plain Weierstrass route at $p = 31$ measured $6.8 dot 10^7$, $12$–$20 times$ further than the two Edwards arms at the same $p$.
+ *Distance remaining* (extrapolation, on the measured replayed test and the census rate): parity near $n approx 2^(#r2(S.model_extrapolation.log2_n_star, d: 1))$, against $2^233$ without replay. The route does not approach rho at any size this repository runs.

== The test: replay against the committed solve, same residuals

#table(columns: 9, align: (left,) + (right,) * 8,
  [run], [$|F|$], [committed solve], [replay], [factor], [min], [max], [replay div.], [checks failed],
  ..S.cost.map(c => (
    c.file, str(c.base_points), sci(c.full_mean), sci(c.replay_mean), r2(c.factor_mean), r2(c.factor_min), r2(c.factor_max), str(c.replay_mismatches), str(c.checks_failed),
  )).flatten()
)

32 residuals per run, each through both. The committed test now costs $5.85 dot 10^7$ (the note's $7.3 dot 10^7$ is from the 2026-09-29 engine); the replay recovers the factor by skipping every pair row that reduced to zero in the recorded run. The `1.00` minimum is one residual (trial 30 of `p = 271` seed 1, trial 25 of `p = 521` seed 1) on which the replay cost the full solve, $5.87 dot 10^7$, although no divergence was flagged: the suspected cause is that the recorded steps ended before the system resolved and `f4_fp` finished the pending pairs as a full run (not verified per residual; the plain route showed the same three cost levels). Every replayed residual's decompositions equal the committed solve's and the meet-in-the-middle oracle's.

#figure(image("figures/edwards_replay.svg", width: 82%), caption: [One test per residual: committed solve and replay (mean, min–max), with the note's figure.])

== Complete DLPs, paired with rho on the same target

#table(columns: 10, align: (left,) + (right,) * 9,
  [run], [$log_2 n$], [$|F|$], [trials], [rel.], [trials/rel. ÷ census (95 %)], [IC online], [IC / rho], [IC / exp. walk], [answer],
  ..S.dlp.map(d => if d.complete {(
    d.file, r2(d.log2_n, d: 1), str(int(d.base_points)), str(int(d.trials)), str(d.relations),
    r2(d.trials_per_relation_over_census) + " [" + r2(d.trials_per_relation_over_census_95.at(0)) + ", " + r2(d.trials_per_relation_over_census_95.at(1)) + "]",
    sci(d.online_muls), sci(d.ratio_online_ic_over_rho), sci(d.ratio_online_ic_over_expected_rho_walk),
    if d.verified and d.rho_verified { "both verified" } else { "NOT VERIFIED" },
  )} else {(
    d.file, r2(d.log2_n, d: 1), str(int(d.base_points)), str(int(d.trials)), str(int(d.relations_recorded)) + " of " + str(int(d.unknowns)),
    "census " + r2(d.census_trials_per_relation, d: 0), sci(d.cost_to_cap_online_muls), "≥ " + sci(d.lower_bound_ic_over_rho), "—", "cap, no log.",
  )}).flatten()
)

The census column is the measured trials per relation over the census expectation $n \/ (8 binom(|F|, 4))$ of RESEARCH_K5 §3, with an exact Poisson 95 % interval on the relation count: #r2(S.model_extrapolation.measured_ratio_to_census) on the geometric mean of the five solved arms, every interval containing 1. It is an expectation, not a bound. The set-up (symmetrised $S_5$, $9.95 dot 10^6$; trace recording, $5.7 dot 10^7$) is excluded from "online" and reported in each record's setup row; rho's counts include its 20 multipliers' set-up, which is why the expected-walk column is also shown. $p = 71$ resumed twice and $p = 41$ seed 1 ran 10.0 h to its cap (`DEVIATION.md`); the resumed record equals an uninterrupted run's (`data/edwards/resume_equivalence/`).

== Unpriced work (AGENTS.md §5a)

Counters not in the unit: rho's partition hash and cycle comparison per walk step (2 per step, bounded above by the group operations), and for the IC arm the F4 symbolic preprocessing (monomial hashing, pair selection), which `f4_fp` does not emit, so the IC side is a lower bound and an implementation gap. Rho's unpriced work at 0, 0.1 and 1 group-addition equivalents per operation:

#table(columns: 4, align: (left,) + (right,) * 3,
  [run], [IC / rho at 0], [at 0.1], [at 1],
  ..done.map(d => (d.file, sci(d.unpriced_band.at(0).ic_over_rho), sci(d.unpriced_band.at(1).ic_over_rho), sci(d.unpriced_band.at(2).ic_over_rho))).flatten()
)

Rho stays ahead at every weight in the band (the IC arm's own unpriced symbolic work only widens the gap), so the order is resolved; the distance remaining is $10^5$–$10^7$.

== Where it would meet rho (extrapolation)

On the measured replay cost (#sci(S.model_extrapolation.c_trial_replay)), the census rate $1\/(192 p)$, $|F| + 1 approx p\/4$ relations and rho with the negation map at $c_"add" = 452$: parity at $n approx 2^(#r2(S.model_extrapolation.log2_n_star, d: 1))$ ($p approx 2^(#r2(S.model_extrapolation.log2_p_star, d: 1))$). With the measured ratio to the census instead, $2^(#r2(S.model_extrapolation.log2_n_star_at_measured_ratio, d: 1))$ (a sensitivity: the interval contains 1). The replay moves the crossover about 6 bits, from $2^233$.

#figure(image("figures/edwards_ic_over_rho.svg", width: 84%), caption: [IC / rho against the order of the group: complete DLPs (Edwards circles, plain squares; open triangle: the capped arm's cost to the cap) and the two models (dashed; extrapolation).])

== What was not done

No ecbench full-pricing run (`main` has no $FF_(p^k)$ instance; `problems/k5-joux-vitse-trial-cost-and-fpk-ecbench.md`). Sizes stop at $p = 71$, the protocol's choice; no larger arm was run, so nothing here measures how the ratio moves beyond $log_2 n = 28.7$. One host, isolation none. The frozen binary (SHA-256 `622a47d3…`, built 2026-10-16) ran every DLP arm; a binary from the merged head reproduces its counts on a 6-residual check (`data/edwards/engine_equivalence/`), not on every arm.
