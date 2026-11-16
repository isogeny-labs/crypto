#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let r=s.strata.at(0)
#let show_value(v)=if v == none { [Unavailable] } else { [#v] }
#let f(v)=if v == none { [—] } else { [#calc.round(v,digits:5)] }
#let yes(v)=if v { [True] } else { [False] }
#let get(v,k)=if v == none { none } else { v.at(k,default:none) }
= Weak-cover graph IC: censored comparison
16 November 2026 · Public synthetic weak elliptic family over Fp6

Frozen source #raw(s.source_revision.slice(0,9)), p1009, fresh seed 257.
Exit #e.exit_code, complete cohort wall #e.outer_wall_seconds seconds.
One attempt, zero retries. [pcc:unverified]

#table(columns:(1.5fr,1fr),inset:5pt,
 [Planned / attempted / completed], [#s.planned_executions / #s.attempted_executions / #s.completed_executions],
 [Retained warmup / measured executions], [#s.warmup_executions / #s.measured_executions],
 [Complete / correct measured rounds], [#r.complete_rounds / #r.correct_rounds of #r.planned_rounds],
 [Unattempted], [#s.unattempted_executions],
 [Prime subgroup order], show_value(r.l),
 [Controlled comparison eligible], yes(r.eligible_local_controlled_comparison),
 [Nontrivial graph in all planned measured arms], yes(r.nontrivial_graph_projection_in_all_measured_arms),
 [Permits later frozen trial], yes(s.allows_later_frozen_scale_trial),
 [Singleton local-reference parity flag], yes(s.local_native_reference_parity),
)

All 25 completed solves recover correctly. All eight completed graph arms
accept projection from 516 to 387 rows with 387 witness replays and no fallback.
The full-schedule projection flag is false because planned arms are missing.
[pcc:unverified]

== Partial paired descriptions
Outer cold process wall is primary. Geometric A/B means and 10000 whole-paired-
round bootstrap draws; intervals describe repeated timings on this fixed
curve and deterministic walk. Inner clock is diagnostic. Invalid/censored
cases remain; incomplete intervals are unavailable. [pcc:unverified]

#table(columns:(1.35fr,0.6fr,0.8fr,0.6fr,0.8fr),inset:4pt,
 [Pair],[Outer mean],[95% interval],[Inner mean],[95% interval],
 ..("graph_over_ic","graph_over_rho","ic_over_rho").map(k=>{
   let q=r.ratios.at(k)
   let ci(v)=if v == none { [Unavailable] } else { [#f(v.at(0))–#f(v.at(1))] }
   (k.replace("_over_"," / "),f(get(q.outer,"geometric_mean")),ci(get(q.outer,"bootstrap_95")),f(get(q.inner,"geometric_mean")),ci(get(q.inner,"bootstrap_95")))
 }).flatten(),
)

#image("ratios.svg",width:100%)
Gray points describe the three complete rounds. All three comparisons are
ineligible under the frozen protocol, and no confidence interval is usable.
[pcc:unverified]

#pagebreak()
= Controls, graph work and custody

All three methods require both-clock same-method median A/A ratios within
[1/1.10,1.10], matching semantic signatures and actual full scalar replay.
The parent withholds the answer from each worker; zero legacy answer fields
make native known-answer flags false by design. [pcc:unverified]

#table(columns:(1fr,1fr,1fr),inset:5pt,
 [Method],[Outer A/A median],[Inner A/A median],
 ..("ic","graph","rho").map(k=>(k,f(r.timing_aa.at(k).outer),f(r.timing_aa.at(k).inner))).flatten(),
)

Global protocol problems: #raw(repr(s.global_problems)).
Case rejection reasons: #raw(repr(r.invalid_reasons)).
In flight without completion: #raw(repr(s.in_flight_without_completion)).
These are producer analyzer outputs, not signed independent verdicts.
[pcc:unverified]

#let g=r.first_measured_work.at("graph",default:(:)).at("graph_filter",default:none)
#if g != none [
  First measured graph work: #g.input_rows input rows, #g.output_rows output
  rows, #g.replayed_rows replayed rows; #g.calls calls, #g.projected accepted
  projections, #g.refusals refusals and #g.fallback_solves original-solve
  fallbacks. Detailed native reports and every repetition remain in the full
  summary/raw evidence. [pcc:unverified]
]

Methods use the same cheap native cover sieve, with incidence/shared-table
options enabled in both IC arms. Tracefalse, fast linear default, stop64,
degree24, margin1.25 and automaticm. Graph protects target plus75percent of
base columns, caps4096, checks input-combination witnesses and charges any
fallback. Rho uses existing negation, batch16, r256 and public-order DPbits.
Restricted weak Fp6 curves with full rational2torsion and a prime subgroup;
this is not a general-curve or prime-order-full-group result. [pcc:unverified]

Every arm is a fresh process. Primary wall includes load, public validation,
transfer/setup, search/descent/elimination/solve, failures, scalar group replay,
serialization and teardown. Shared challenge preparation is separately
recorded and retained in total cohort wall. Legacy operation counters omit
graph/wrapper work and do not provide complete cost-band rankings. Native IC
rho-related fields are analytic/placeholder values; comparisons use actual
rho process records only. [pcc:unverified]

Frozen limits: CPU0,8GiB virtual,900seconds per worker,3600wholecohort,
1retained warmup and5measured rounds with six arms and reversed odd rounds;
no retries or compilation during timing. Start/completion records and native
progress persist. Cgroup CPU snapshots include the environment and do not
identify phase wall shares. [pcc:unverified]

Full native summary is byte-identical to the original copied Rust analyzer.
Original raw, progress, binaries, checkpoint, source and host metadata remain
at #raw(e.run_path). Raw/summary hashes and exact flow are recorded in
local-evidence.json. No canonical encrypted-schema, strongest-external,
standard lab, signed independent or admitted ranking assertion. The
singleton parity flag is frozen false regardless of any observed ratio.
[pcc:unverified]

Partial means and observed A/A medians remain descriptive. The final child's
last recorded progress is descent residuals 3648; its exact cut phase and
partial solve wall are unknown. [pcc:unverified]
