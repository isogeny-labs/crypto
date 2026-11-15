#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let R = json("report.json")
#let C2 = R.at("counts").at("slots_2")
#let C3 = R.at("counts").at("slots_3")
#let f(x) = str(calc.round(x, digits: 2))
#let sp(s) = [#f(s.at("min"))–#f(s.at("max")) (median #f(s.at("median")))]

= hashed-ecdl-tensor: smaller decomposition networks, same language

Study: `research/hashed_ecdl_dedupe_20261115/`. Tool: `native/`
(`dedupe-measure`, Rust), built unchanged against the crate at `origin/main`
23cf569058 (*before*) and after each step below. Every number is read from
`runs/report.json`, produced by `dedupe-measure report runs` from the raw records.

*Unit.* Step counts are the crate's `Meter` charges: deterministic diagnostics,
not calibrated time. No wall-clock time is reported.

*Result.* The final build uses at least 10x fewer charged steps than main on
every one of the 36 recorded counts and 5 solves (table below).

== The steps (cumulative)

+ *dedupe* (commit 1). `canonical(w)` and `valid_point(P)` are emitted once per
  (signals, modulus or curve, semantic region); modular add/subtract/multiply on
  the same operand signals return the word already built (*memo only*). The
  derived running sum starts at $A_1$ instead of $cal(O) + A_1$ (*skip only*).
+ *fold*. A definition whose inputs are all constants returns constant words;
  an all-constant constraint that holds is dropped (one that fails is kept);
  `and`/`or` with a constant operand and `gated_equal` with a constant selector
  simplify. Fallback to the unfolded definition whenever folding would change
  evaluation (function error, output too wide).
+ *valid*. `point_add` records its result as valid instead of re-checking it:
  in every case the result is pinned to the identity, a valid input, or the
  chord/tangent point of the unique slope. `negate_point` records $(x, -y)$ of
  a valid point as valid. New exhaustive test: for every input pair on two tiny
  curves the graph accepts exactly one of all $2^(1+2w)$ output values, $a+b$;
  deleting one output constraint makes it fail.
+ *chains*. `modular_multiply` by a canonical constant with a double-and-add
  chain of at most four additions (e.g. the $3x^2$ of doubling) uses modular
  additions instead of two Fourier products. Tested against $x c mod 13$ for
  every $c$ and every 4-bit $x$.
+ *implied*. A checked definition emits a Define step and the Check of the
  same function on exactly its inputs and outputs. Program evaluation computes
  those outputs with that function, so the check holds there by construction
  and is skipped (the graph keeps the vertex; every accepted assignment is
  still verified against the whole graph, so a disagreement raises an
  integrity error instead of a wrong count). Census rewiring, which can swap
  endpoints, clears the marks.
+ *auto*. `Multiplier::Auto` (now `decomposition_network`'s default) builds
  each product with whichever gadget, Fourier or schoolbook, has fewer graph
  ports for those operand shapes, chosen once per shape by building both in a
  scratch encoder. Schoolbook wins at these widths; Fourier still wins at
  $d = 131$, so the ECCp-131 network keeps it.
+ *final*. A ripple-adder bit with two constant-zero inputs passes the third
  through instead of emitting a full adder (shifted schoolbook rows and
  constant multipliers have many such bits).

The language is unchanged at every step: each removed piece is a duplicate of,
or implied by, constraints that remain.

== Exact counts against brute force

#table(
  columns: 5,
  [slots], [variant], [all = brute force, same counts and visited assignments], [charged steps, before / variant], [edges, variant / before],
  ..(("2", C2), ("3", C3)).map(((m, C)) => ("skip_only", "memo_only", "dedupe", "fold", "valid", "chains", "implied", "auto", "final").map(v => (
    m, v,
    if C.at(v).at("all_match_brute_force") and C.at(v).at("same_counts_as_before") and C.at(v).at("same_functional_assignments_as_before") { "yes" } else { "NO" },
    sp(C.at(v).at("step_ratio_before_over_variant")), sp(C.at(v).at("edge_fraction_variant_over_before")),
  ))).flatten()
)

Instances: p = 5, 7, 11, 13 (m = 2, 24 counts) and p = 5, 7 (m = 3, 12 counts);
thresholds $s$, $s\/2$, $s\/4$; two targets each.

#figure(image("dedupe.svg", width: 100%), caption: [Per-count step ratios by step, network size by field width (ECCp-131 marked), whole-solve charged steps. Source: `runs/report.json`.])

== End-to-end solves (`reduction::solve`, then `reduction::verify`)

#let S = R.at("solves").at("final")
Final vs before: all verified *#S.at("all_verified_before_and_after")*; same scalar
and relation count *#S.at("same_scalar_and_relations")*; step ratio #sp(S.at("step_ratio")).

#table(
  columns: 7,
  [p], [seed], [secret], [recovered], [relations], [steps before], [steps final],
  ..S.at("rows").map(r => (str(r.at("p")), str(r.at("seed")), str(r.at("secret")),
    r.at("recovered"), str(r.at("relations")), str(r.at("steps_before")), str(r.at("steps")))).flatten()
)

== Network size, including ECCp-131

#table(
  columns: 6,
  [input], [slots], [edges before], [edges final], [vertices before], [vertices final],
  ..R.at("eccp131").at("final").map(e => ("ECCp-131 (d = 131)", str(e.at("slots")), str(e.at("edges_before")), str(e.at("edges")), str(e.at("vertices_before")), str(e.at("vertices")))).flatten(),
  ..R.at("sizes").at("final").map(r => ("p = " + str(r.at("p")), str(r.at("slots")), str(r.at("edges_before")), str(r.at("edges")), str(r.at("vertices_before")), str(r.at("vertices")))).flatten(),
)

== What this does not show

These are constant factors in the network every backend contracts. The
exponent is unchanged: the `Functional` backend still visits
$2^"free input bits"$ assignments, and the recursive backend still needs
$>= 2^131070$ edges, against
$2^#calc.round(R.at("eccp131").at("final").at(0).at("log2_edges"), digits: 1)$ for the
two-slot ECCp-131 network now. No ECCp-131 scalar is computed. No wall-clock
time is measured, so no `benchmarks/` or leaderboard row is added; the
validation gate admits none without a replay certificate, validator report and
matched rho reference.
