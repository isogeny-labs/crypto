#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9pt)
#set heading(numbering: "1.")
#show raw: set text(size: 7.5pt)
#show figure.caption: set text(size: 8pt)

#align(center)[
  #text(size: 14pt, weight: "bold")[PS-4 run report: candidate reduction by Frobenius-orbit representatives and the 2-torsion symmetry] \
  #text(size: 9.5pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps4_symmetry/` --- run started 2026-10-11 23:46 UTC, finished 2026-10-18; frozen protocol `protocols/PS-4.md`; class (as frozen): engineering; stage measurement]
]

= What ran

`ps4_symmetry/run_ps4.sh` (stages build, checks, panel, run, torsion, control, summary, figures, pdf) with the standalone example `examples/pdp_ps4`, the panel `examples/hamming_nb_sat_panel` for its exact oracle, and the PS-2 group-law kernel on the PS-0 corpus (library counters, `--features pdp-opcount`) as the subspace-leaf control. Every build and run went through `tools/isolated_bench.py busy --wait`. Conductor: `conductor check` was clear; the checkout has no `.conductor/WORKFLOW.md` or task card.

*Workload.* Fourteen settings, the Hamming-weight base `wt <= w` with `w` in {2, 3} at the type-II degrees `n` in {11, 23, 29, 33, 35, 39, 41} of the curve `K_0: y^2 + xy = x^3 + 1`; per setting the panel's 8 targets (6 uniform, 2 planted sums of three base points; seed 20261011; `m = 3`): 112 targets. Raw records: `ps4_symmetry/raw/*.jsonl`; tables: `raw/summary.md`.

*Hosts.* The checks, the panel runs and the unreduced/reduced runs up to `(n, w) = (39, 3)` ran on an Intel Xeon 2.80 GHz KVM guest (kernel v114, 4 vCPU, AVX-512, `pclmulqdq`); a container restart moved the rest (run `(41, 3)`, torsion, control) to an Intel Xeon 2.10 GHz guest (kernel v139); a second restart interrupted the torsion stage at `(33, 3)`, whose partial records are kept in `raw/attempts/`. Every count below is deterministic and host independent; wall figures are labelled and never pooled across the two hosts.

= Observations

== Candidates and counted operations, reduced against unreduced

Full-scan mode, sums over the 8 targets of a setting. The unreduced loops take every base abscissa as first summand (`ordered`: every abscissa as second summand, `m^2` pairs; `unordered`: from the first on, `m(m+1)/2` pairs, the PS-0/PS-2 loop); the reduced loop takes one representative per Frobenius orbit as first summand and every abscissa as second (`reps * m` pairs). `m` = abscissae of weight `<= w` that lift. `M` = counted field multiplications (`mul` + `batch_mul`, inversion products included).

#text(size: 6.4pt)[#include "ps4_tab_candidates.typ"]

`short orbits` lists the orbits smaller than `n` (the point `T`, `x = 0`, with size 1 in every setting; at `(33, 3)` also a period-11 orbit of size 11); orbit sizes add up to `m` in all 14 settings. `op/pair` = (M factor) / (pair factor): the fraction of the candidate factor that the counted multiplications keep (the frozen criterion asks for at least 0.8).

#figure(
  image("ps4_candidate_factor.svg", width: 92%),
  caption: [Candidate-pair factor, unreduced over reduced, per setting (source `raw/plot_data.json`, derived from `raw/runs.jsonl`, `raw/torsion.jsonl`; 8 targets per setting, 2 at the three largest `w = 3` torsion settings). Filled markers: pairs. Hollow rings: counted multiplications `M`. Triangles: the 2-torsion reduction, measured on the T-closed base `B+-`, not on the weight base.],
)

Counted operations per pair are the same in every loop and setting: `mul` 4.00 to 5.35, `sqr` 4.0 to 4.7 (a coordinate permutation here), `batch_mul` 6.00, `inv` 0.0006 to 0.25, `add` 16, `popcnt` 4, `M` 10.0 to 11.3 (`raw/summary.md`, Table 2b).

== Verdicts and solution sets

#text(size: 6.4pt)[#include "ps4_tab_agree.typ"]

Over the 112 targets: the oracle calls 66 decomposable and 46 not; the unreduced kernel's solution set equals the oracle's on 112/112 (unordered and ordered loops); the panel (run on 11 of the 14 settings, 88 targets) agrees with the pair-table oracle of the example on `x_R`, verdict and `ref_count` on 88/88. The reduced loop's set is contained in the oracle's on 112/112 and equals the oracle's triples that have a representative member on 112/112 (`kept solutions`: pooled distinct solutions of the reduced loop over the oracle's).

#text(size: 6.4pt)[#include "ps4_tab_decision.typ"]

*The reduced loop on the target alone returned the unreduced verdict on 84 of 112 targets; on the 66 decomposable targets it answered no on 28.* Taking all conjugates of the target (`closure = U`), the union of the reduced loop's solutions, mapped back, equals the unreduced set on 112/112; that run visits 1.007 to 1.435 times the pairs of the ordered unreduced loop on the single target (Table 5 of `raw/summary.md`). Hits per candidate third, pooled: reduced over unreduced = 1.000 at `(11, 3)` (14,505 and 2,872 hits) and 0.992 at `(23, 3)` (6,105 and 536 hits); elsewhere the hit counts are 0 to 496 and the ratio is not informative.

#figure(
  image("ps4_decision.svg", width: 92%),
  caption: [(a) fraction of each setting's 8 targets whose verdict the reduced loop keeps when run on the target alone, and whose solution set the reduction over all conjugates reproduces; (b) pooled fraction of the oracle's solutions the reduced loop returns on the target alone; (c) ratio of counted multiplications, reduced over unreduced, to the first witness on decomposable targets (the reduced loop's full scan counted where it found nothing), and for the all-conjugates run against the ordered unreduced loop. Source `raw/plot_data.json`.],
)

== The 2-torsion symmetry

Translation by `T = (0, 1)` is `x -> 1/x` (checked: 0 mismatches, below); in `u = 1/(x + 1)` it is `u -> u + 1`, and in the ONB `1` is the all-ones word, so it is the complement, of weight `n - wt(u)`. On the weight base, for liftable abscissae: the image of `T` is in the base for 0 of the base's abscissae in all 14 settings, in the `x` frame and in the `u` frame (columns below). The reduction was tested on the nearest `T`-closed base `B+- = {wt(u) <= w or wt(u) >= n - w}` (ordered loops, leaf a table probe; `T` alone forms a class, `T + T = O`):

#text(size: 6.4pt)[#include "ps4_tab_torsion.typ"]

Here `closure` is the `T`-reduced set closed under translating two non-`T` summands by `T`, compared with the pair-table oracle on `B+-`. Not run: `T` on a Frobenius-invariant subspace containing 1 (the repository's own `u`-frame base); the combined reduction by orbits and `T` as a loop.

== The leaf, weight base against the subspace control

The control is the PS-2 group-law kernel on the first 6 uniform and 2 planted targets of each PS-0 rung `l = 8..12` (`n = 24..36`, `a = b = 1`, polynomial basis; 66,048 to 15,880,224 pairs per rung): `M/pair` = 10.53, 10.24, 10.15, 10.06, 10.03 and `popcnt`/compare 4 per pair; verdicts equal the corpus on 8/8 targets of every rung. `bits` is the model of `model.rs` (a convention: schoolbook two-input gate counts, ONB product `3 n^2 - 2 n`, no sub-quadratic products); the last-but-one column puts the control at the weight base's degree by `(n / n_l)^2`.

#text(size: 6.4pt)[#include "ps4_tab_leaf.typ"]

#figure(
  image("ps4_leaf.svg", width: 92%),
  caption: [Leaf cost per pair, weight base in the ONB against the subspace control at the rung with `3 l` nearest `n`. (a) counted multiplications; (b) the same pairs in the bit-operation model, at each side's own degree and with the control scaled to `n`; the dashed line is the frozen stop condition (ratio above 2). Source `raw/plot_data.json`.],
)

*Wall time, a labelled note, unisolated, shared container.* Weight kernel (portable software cyclic-convolution product): 480 to 1,233 ns per pair on the 2.80 GHz host and 911 ns at `(41, 3)` on the 2.10 GHz host; control (`pclmulqdq`): 26 to 39 ns per pair on the 2.10 GHz host. The one same-host pair, `(41, 3)` against `l = 12`: 911 against 31 ns per pair. The two kernels are different code on different instructions, so this does not compare the bases.

== Algebra and field checks (`K_0`, `a = 0`, `b = 1`)

- ONB field against the library's `Type2Onb`: products exhaustive for `n <= 11` (4,194,304 pairs at `n = 11`) and 200,000 samples at each of `n = 23..41`: 0 mismatches; squarings, inversions (Itoh--Tsujii, products counted = formula) and lifts: 0 mismatches.
- Group law with `a = 0` against the library's `OnbCurve::add`: 257,556 pairs exhaustive at `n = 9`, 200,000 sampled at each of `n = 11..41`: 0 mismatches. Point counts of `K_0` at `n = 3, 5, 9, 11, 23` equal the Koblitz recurrence (4, 44, 508, 2,116, 8,383,412).
- `x(P + T) = 1/x`: 0 mismatches for `P` and `-P`, exhaustive for `n <= 23` (4,191,705 points at `n = 23`), about 10^5 samples at each of `n = 29..41`.
- Exactness theorem (roots of `S_4(., ., ., x_R)` over `V^3` are rational decompositions unless two abscissae are non-liftable with `x_a = x_b, x_c = x_R` or `x_a x_b = x_c x_R = 1`): exhaustive for `V = F` at `n = 3, 5, 9` and for `wt <= 2, 3` at `n = 11` (1,057 targets each); 0 decompositions that are not roots; 0 unexplained extra roots; 0 extra roots at targets with `x_R` and `1/x_R` outside `V` (extra roots F1/F2: 6/3, 210/105, 65,274/32,637 for `V = F`; 968/0 and 13,068/2,178 for `wt <= 2, 3`).

= Missing, partial or not done

- The panel's own oracle was not run for `(39, 3)` and `(41, 3)` (its pair-table code needs about 10^7 to 10^8 additions on the library's inversion and, for refuting targets, a full scan each; measured 449 s for the table at `(33, 3)`); at those two settings (16 targets) the verdicts and solution sets are checked against the example's own pair-table oracle, which reproduces the panel's `ref_count` and verdict on all 88 panel targets. The `(35, 3)` panel run is listed in `raw/` only if it finished (see README).
- Torsion at the three largest `w = 3` bases ran on 2 of 8 targets (a uniform and a planted one).
- Not attempted: a hardware-assisted normal-basis product; `T` on the repository's invariant subspace bases; the combined orbit-and-`T` loop; any `m = 2` or `m > 3` loop; any end-to-end cost.

= Mechanism

#figure(
  image("ps4_mechanism.svg", width: 92%),
  caption: [What each reduction decides. Frobenius maps decompositions of `R` to decompositions of its conjugates; translation by `T` maps decompositions of `R` to decompositions of the same `R`. Mechanism, not a measurement.],
)
