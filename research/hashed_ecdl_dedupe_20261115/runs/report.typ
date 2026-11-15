#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let R = json("report.json")
#let C2 = R.at("counts").at("slots_2")
#let C3 = R.at("counts").at("slots_3")
#let f(x) = str(calc.round(x, digits: 2))
#let sp(s) = [#f(s.at("min"))–#f(s.at("max")) (median #f(s.at("median")), n = #s.at("n"))]

= hashed-ecdl-tensor: emit each check once, start the sum at $A_1$

Study: `research/hashed_ecdl_dedupe_20261115/`. Tool: `native/`
(`dedupe-measure`, Rust), built unchanged against the crate at `origin/main`
(*before*), with each change alone (*skip only*, *memo only*), and with both
(*after*). Every number below is read from `runs/report.json`, produced by
`dedupe-measure report runs` from the raw records in `runs/`.

*Unit.* Step counts are the crate's `Meter` charges: deterministic diagnostics,
not calibrated time. No wall-clock time is reported.

== The two changes

+ *Encoder reuse.* `canonical(w)` and `valid_point(P)` are emitted once per
  (signals, modulus or curve, semantic region); `modular_add`, `modular_subtract`
  and `modular_multiply` on the same operand signals return the word already
  built. A repeated check is the same constraint on the same signals; each
  modular output and its auxiliary witnesses are uniquely determined by the
  inputs (the crate's own documented invariant). So the language and every
  count are unchanged; only duplicated edges go. Before, `point_add` re-validated
  both inputs and every `modular_multiply` re-checked both operands canonical.
+ *No $cal(O) + A_1$.* The derived-last-slot network (`reduction.rs`) began
  its running sum at a constant identity and added $A_1$ through a full
  addition network. It now starts the sum at $A_1$.

== Exact counts against brute force

#table(
  columns: 5,
  [slots], [variant], [all = brute force], [charged steps before / variant], [edges variant / before],
  ..(("2", C2), ("3", C3)).map(((m, C)) => ("skip_only", "memo_only", "after").map(v => (
    m, v, if C.at(v).at("all_match_brute_force") and C.at(v).at("same_counts_as_before") { "yes" } else { "NO" },
    sp(C.at(v).at("step_ratio_before_over_variant")), sp(C.at(v).at("edge_fraction_variant_over_before")),
  ))).flatten()
)

Instances: p = 5, 7, 11, 13 (m = 2) and p = 5, 7 (m = 3); thresholds $s$, $s\/2$, $s\/4$;
two targets each. Visited functional assignments are identical to *before* in
every row (#C2.at("after").at("same_functional_assignments_as_before"),
#C3.at("after").at("same_functional_assignments_as_before")): the saving is per
assignment, not fewer assignments.

#figure(image("dedupe.svg", width: 100%), caption: [Per-count step ratios, network size by field width (ECCp-131 marked), whole-solve charged steps. Source: `runs/report.json`.])

== End-to-end solves (`reduction::solve`, then `reduction::verify`)

#let S = R.at("solves")
All verified before and after: *#S.at("all_verified_before_and_after")*; same
scalar and relation count: *#S.at("same_scalar_and_relations")*. Step ratio
#sp(S.at("step_ratio")).

#table(
  columns: 7,
  [p], [seed], [secret], [recovered], [relations], [steps before], [steps after],
  ..S.at("rows").map(r => (str(r.at("p")), str(r.at("seed")), str(r.at("secret")),
    r.at("recovered"), str(r.at("relations")), str(r.at("steps_before")), str(r.at("steps")))).flatten()
)

== Network size, including ECCp-131

#table(
  columns: 4,
  [input], [slots], [edges before], [edges after],
  ..R.at("eccp131").map(e => ("ECCp-131 (d = 131)", str(e.at("slots")), str(e.at("edges_before")), str(e.at("edges")))).flatten(),
  ..R.at("sizes").map(r => ("p = " + str(r.at("p")), str(r.at("slots")), str(r.at("edges_before")), str(r.at("edges")))).flatten(),
)

Maximum degree is unchanged (132 at every recorded size).

== What this does not show

It is a constant-factor change to the network every backend contracts. It does
not change the exponent: the `Functional` backend still visits
$2^"free input bits"$ assignments, and the recursive backend still needs
$>= 2^131070$ edges to engage, against $2^#calc.round(R.at("eccp131").at(0).at("log2_edges"), digits: 1)$
for the two-slot ECCp-131 network after this change. No ECCp-131 scalar is
computed. No wall-clock time is measured, so no `benchmarks/` or leaderboard
row is added; the validation gate admits none without a replay certificate,
validator report and matched rho reference.
