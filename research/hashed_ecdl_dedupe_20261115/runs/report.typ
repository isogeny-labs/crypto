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

*Result.* Against main, the final build uses 26--45x fewer charged steps per
exact count at m = 2 and 31--39x at m = 3, 20--29x per seeded solve; networks
have 9--30x fewer edges and 3--10x fewer vertices on the recorded curves. A
100x target is not reached: see the last section.

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
+ *zeroadd*. A ripple-adder bit with two constant-zero inputs passes the
  third through instead of emitting a full adder.
+ *compressed*. Free points of the derived network are enumerated as
  $(x, "parity of" y)$, $w+1$ free bits instead of $2w$; the other bits of $y$
  are a defined witness (the root of $x^3+a x+b$ with that parity) pinned by
  the unchanged validity constraints. For odd $q$, $y$ and $q-y$ differ in
  parity, so the accepted points and every count are unchanged; visited
  assignments drop (hence the column below says _no_ from here on).
  `decomposition_network_with_multiplier` keeps the $(x, y)$ layout.
+ *verify*. The encoder proves once that every vertex is a Check step's
  vertex read through exactly its edges' signals, or a copy/endpoint vertex
  on one signal; an accepted assignment then re-evaluates only the implied
  (skipped) checks' vertices instead of the whole graph.
+ *hashfirst*. The hash test reads only $x$ and the parity bit, so it runs
  before the curve equation.
+ *final*. `point_add` with a constant first point $T$ pins the tangent slope
  to the constant $(3x_T^2+a)\/(2y_T)$ instead of multiplying, and the same-$x$
  test reuses $x_2 - x_1$.

The language is unchanged at every step: each removed piece is a duplicate of,
or implied by, constraints that remain.

== Exact counts against brute force

#table(
  columns: 5,
  [slots], [variant], [all = brute force and same counts (same visited assignments)], [charged steps, before / variant], [edges, variant / before],
  ..(("2", C2), ("3", C3)).map(((m, C)) => ("skip_only", "memo_only", "dedupe", "fold", "valid", "chains", "implied", "auto", "zeroadd", "compressed", "verify", "hashfirst", "final").map(v => (
    m, v,
    (if C.at(v).at("all_match_brute_force") and C.at(v).at("same_counts_as_before") { "yes" } else { "NO" }) + (if C.at(v).at("same_functional_assignments_as_before") { " (yes)" } else { " (no: fewer)" }),
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

== Why not 100x

At p = 13 the two-slot network has 1,215 vertices: 541 definition-check
vertices, 441 copy-equality vertices (the construction's degree-3 fan-out
trees), 215 other constraints. The arithmetic gadgets left are bit-level
ripple adders and AND arrays, about 228 ports per modular product at w = 4.
Going much further would mean fusing gates into wider predicates or replacing
the degree-3 equality trees with wide stars: fewer, larger steps and vertices
for the same work, which is relabelling under the bounded-degree model, not an
improvement. Per accepted assignment, the implied checks are still evaluated as
the integrity net.

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
