#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let v1 = json("../../paths.json")
#let fr = json("../../v2/paths_v2.json")
#let rp = json("../../v2/paths_v2_replay.json")
#let f(x, d: 1) = str(calc.round(x, digits: d))
#let cls(d) = d.classes.filter(c => "p" in c)
#let nbad(c) = c.pairs.filter(q => q.legacy.at("route_executable", default: true) == false).len()
#let ratio(a, b) = if b == 0 { "n/a" } else { f(a / b, d: 2) }

= R6.2 correction run report: the path search on certified edges (paths v2)

*Run id* `run-20261025T0156Z` (UTC start of the fresh paths run). *Commit* base `5cb553810e` (origin/main) plus the uncommitted correction on branch `research/r6-2-fix`, landed in the same PR as this report. *Host* `vm`, 4 vCPU shared with other build jobs: *no isolation earned*; every recorded result is a count, wall times are contended notes only. Toy instances; nothing here concerns a deployed curve and no sub-rho claim is made. This run corrects the path-finding half of `run-20261018T2219Z`; the batch half is not re-run (the batch class uses only $ell in {2, 3}$ and its edges all have duals).

*Commands* (worktree root, `target/release/examples/r6_2_pooled_class_rho`):
- fresh: `--mode paths --out round6/r6_2_pooled_rho/v2/paths_v2.json` (defaults: `--pbits 24,28,32 --classes 6 --pairs 10 --cap 3000 --seed 7`), log `v2/paths_v2.log`;
- replay of the 18 frozen v1 start curves: `--mode paths --replay round6/r6_2_pooled_rho/paths.json --out …/v2/paths_v2_replay.json`, log `v2/paths_v2_replay.log`.

Schema `#fr.schema`. Both runs completed; no failures. The v1 records `paths.json` and `paths.log` are unchanged (frozen, schema `#v1.schema`).

== What was wrong (derived from the code and checked by the controls below)

The v1 search expanded _outgoing_ neighbours from both the source $a$ and the target $b$. An outgoing edge $u -> v$ of $b$ is a predecessor edge $v -> b$ only if $v$ has a degree-$ell$ edge back to $u$. For $ell = 2, 3$ every edge has one. For $ell = 5, 7$, `SmallIsogeny::dual` builds the dual from a rational root of the $ell$-division polynomial, which exists iff $p equiv plus.minus 1 med (mod ell)$ (note in `run-20261018T2219Z`). Otherwise a meeting $a -> c <- b$ or a hop read backwards is not an executable route $a -> b$, and the length it reports is not the length of any route.

== Correction

The graph now keeps an edge $u -> v$ only when `SmallIsogeny::dual` certifies $v -> u$, so the edge set is symmetric and expanding outgoing edges of $b$ constructs real predecessors. Every certification is counted. Every returned route is then re-validated in the forward direction from $a$: at each hop the degree-$ell$ kernels of the _chained_ curve are re-enumerated (counted) and one with the hop's $j$-invariant is taken; the end must have $j(b)$. The pre-fix search runs on the same pairs as a control (`legacy`). Reported cost per pair = search enumerations + certifications + validation enumerations (each a count, derived by summing the recorded fields).

== Controls (native tests in the example, release build)

- `a_degree_five_edge_without_a_reverse_edge_is_excluded_from_the_certified_set`: for $p equiv.not plus.minus 1 med (mod 5, 7)$ a one-way degree-5 edge exists, `dual` fails, the outgoing set keeps it and the certified set drops it; a degree-5 cycle that returns to $j(e)$ through another kernel is counted and set aside; every certified edge has a forward reverse edge. Passed.
- `asymmetric_edges_break_the_outgoing_search_and_not_the_certified_one`: on those primes the pre-fix search returns routes that fail forward validation on pairs drawn from its own (outgoing) components, while every certified route validates edge by edge. Passed.
- `symmetric_primes_validate_under_both_edge_sets`: for $p equiv plus.minus 1 med (mod 5)$ and $(mod 7)$ both edge sets give valid routes (the failures above are the asymmetry, not the validator).
- `the_outgoing_control_reproduces_the_frozen_search_lengths`: the refactored outgoing search matches the pre-fix `bidirectional` lengths.

== Results (measured counts)

#table(columns: 10, align: right, stroke: 0.4pt + luma(160),
  [bits], [$p$], [comp.], [mean search], [mean cert.], [mean valid.], [mean total], [total / comp. (derived)], [pre-fix not exec. (of 10)], [pre-fix shorter],
  ..cls(fr).map(c => (str(c.pbits), str(c.p), str(c.component_size), f(c.mean_enumerations), f(c.mean_certifications), f(c.mean_validation_enumerations), f(c.mean_total_enumerations), ratio(c.mean_total_enumerations, c.component_size), str(nbad(c)), str(c.legacy_shorter_than_corrected))).flatten())

Fresh classes (seed 7). Components skipped over the cap 3000 / under 8 curves: #fr.classes.filter(c => "skipped_over_cap" in c).map(c => str(c.pbits) + " bits: " + str(c.skipped_over_cap) + " / " + str(c.skipped_under_8)).join("; ").

#table(columns: 10, align: right, stroke: 0.4pt + luma(160),
  [bits], [$p$], [v1 comp.], [v2 comp.], [mean search], [mean cert.], [mean valid.], [mean total], [v1 mean enum.], [pre-fix not exec. (of 10)],
  ..cls(rp).map(c => {
    let o = v1.classes.filter(x => "p" in x and x.p == c.p and x.a == c.a and x.b == c.b).first()
    (str(c.pbits), str(c.p), str(o.component_size), str(c.component_size), f(c.mean_enumerations), f(c.mean_certifications), f(c.mean_validation_enumerations), f(c.mean_total_enumerations), f(o.mean_enumerations), str(nbad(c)))
  }).flatten())

Replay of the 18 v1 start curves, same pair count. A component of one curve gives pairs $a = b$ and 0 enumerations.

#figure(image("paths_v2_enumerations_vs_component.svg", width: 94%), caption: [Run `run-20261025T0156Z`: recorded mean neighbour enumerations per pair against component size, 10 pairs per class (180 pairs per set). Grey circles: frozen v1 (`paths.json`). Squares: v2 replay of the same start curves. Triangles: v2 fresh classes. Source `v2/paths_v2.json`, `v2/paths_v2_replay.json`, `paths.json`; plot script `plot_v2.py`.])

#figure(image("paths_v2_legacy_not_executable.svg", width: 94%), caption: [Run `run-20261025T0156Z`: per class, the pairs (of 10) whose pre-fix outgoing-edge route fails forward validation, on the pairs drawn from the corrected certified components. Source `v2/paths_v2.json`, `v2/paths_v2_replay.json`; plot script `plot_v2.py`.])

== Observations

- All #(cls(fr).map(c => c.pairs.len()).sum() + cls(rp).map(c => c.pairs.len()).sum()) corrected routes (180 fresh, 180 replay) validated forward edge by edge; none ended on a twist of $b$ only.
- On the pairs of the corrected components the pre-fix search returned a non-executable route for #cls(fr).map(nbad).sum() of 180 fresh pairs (classes with $p equiv.not plus.minus 1 med (mod 5, 7)$) and #cls(rp).map(nbad).sum() of 180 replay pairs, and a shorter length than the executable one for #cls(fr).map(c => c.legacy_shorter_than_corrected).sum() and #cls(rp).map(c => c.legacy_shorter_than_corrected).sum() pairs. These are lower bounds on the v1 defect: v1 drew its pairs from the larger outgoing components.
- Replaying the 18 v1 start curves, #cls(rp).filter(c => c.component_size < v1.classes.filter(x => "p" in x and x.p == c.p and x.a == c.a and x.b == c.b).first().component_size).len() certified components are smaller than the v1 component (the one-way $ell = 5, 7$ edges are gone); the rest are unchanged. The v1 component sizes were sizes of an outgoing-edge reachability set, not of a symmetric graph.
- In the larger components (above 1000 curves) the corrected total per pair is 2.7 to 5.1 times the component size (derived, table above), against 0.7 to 2.4 for the v1 search enumerations alone (the five v1 components above 1000 curves); certification and validation account for the difference. The cost still grows with component size and stays on its order; it does not follow the $O(sqrt("class size"))$ of the hypothesis, as in v1 (the hypothesis is not shown by these data).

== Missing data and limits

No isolated or independent-runner measurement; no ecbench session; pairs drawn uniformly from the certified component, so the cost is that of a bidirectional breadth-first search between two random nodes of a small toy class, not of an Elkies walk; selection is biased toward components under the cap 3000 (4, 5 and 9 components skipped over it at 24, 28 and 32 bits); the fresh run draws different classes from v1 (same seed, a different edge set changes which components pass), so fresh and v1 are compared only through the replay. The corrected search needs the same dual check for every edge, so its certification counts include edges the search then drops. The pooled-batch half of R6.2 is unchanged (no re-run; its class uses only $ell in {2, 3}$).
