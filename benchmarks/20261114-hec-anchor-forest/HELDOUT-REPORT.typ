#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: false, leading: 0.5em)
#set heading(numbering: none)
= Held-out hyperelliptic anchor relations
2026-11-15 UTC | source `2404e34dca` | record `20261115T0700Z-hec-heldout-scaling`

#text("[pcc:unverified]") Producer assertions pending independent scientific replay.

All 20 attempted dense-curve fixtures were selected. All 100 single-target group gates and 1,500 interleaved timed cells completed and passed their local answer checks. Curve-level medians of paired fastest-control/compact ratios range *0.94x to 3.17x*; the largest individual target ratio is 4.15x. This screen does not establish the mathematical 10x objective or novelty.

#image("heldout.svg", width: 100%)

#table(columns: (0.35fr, 0.8fr, 0.45fr, 0.65fr, 0.65fr, 0.65fr, 0.75fr), align: (left,right,right,right,right,right,right), inset: 2.4pt, stroke: 0.3pt + rgb("#cad5db"),
table.header([*g*],[*p*],[*family*],[*min*],[*median*],[*max*],[*A/A max*]),
[2],[1009],[0],[1.72x],[1.86x],[2.00x],[1.04x],
[2],[1009],[1],[1.67x],[1.82x],[1.87x],[1.62x],
[2],[4001],[0],[2.04x],[2.08x],[2.71x],[1.41x],
[2],[4001],[1],[1.87x],[2.12x],[2.21x],[1.49x],
[2],[16381],[0],[1.49x],[1.84x],[2.38x],[1.44x],
[2],[16381],[1],[1.74x],[2.11x],[2.25x],[1.77x],
[2],[65537],[0],[1.13x],[1.57x],[1.92x],[1.51x],
[2],[65537],[1],[1.33x],[2.05x],[2.64x],[1.57x],
[3],[127],[0],[1.68x],[1.74x],[1.81x],[1.66x],
[3],[127],[1],[1.34x],[1.92x],[1.93x],[1.11x],
[3],[503],[0],[0.82x],[1.53x],[2.29x],[2.39x],
[3],[503],[1],[1.16x],[1.38x],[2.56x],[1.88x],
[3],[1009],[0],[0.76x],[0.94x],[2.11x],[1.91x],
[3],[1009],[1],[0.79x],[1.47x],[2.47x],[1.96x],
[4],[31],[0],[0.35x],[0.95x],[1.33x],[3.09x],
[4],[31],[1],[1.02x],[1.29x],[1.80x],[2.10x],
[4],[61],[0],[0.68x],[2.48x],[3.13x],[1.08x],
[4],[61],[1],[1.24x],[2.25x],[2.73x],[2.61x],
[4],[127],[0],[2.07x],[2.65x],[3.32x],[2.52x],
[4],[127],[1],[2.08x],[3.17x],[4.15x],[1.39x],
)

Each ratio is a separate single-target cold comparison; no cross-target setup is amortized. The minimum of twelve control times is taken separately for each target. This conservative exploratory baseline selection can bias the ratio downward. Table medians are summaries of five paired ratios, not ratios of pooled throughput.

#pagebreak()
= Method, limits and structural decision

*Cohort.* Two seeded dense-coefficient families at each field size, monic squarefree odd-degree models of genus 2, 3 and 4. Selection tried at most 128 curves and eight initial degree-one places, accepting the first certified prime-order generator meeting the frozen bounds. No solver timing entered selection. Exact f, generator, subgroup order, attempt and target identity are retained in raw receipts. A generator order is not a full Jacobian-order assertion.

*Targets and controls.* Five independently seeded nonzero scalars per curve; the same public target is given to each arm. Controls comprise native/table, branch count 1/64, Fast/Hybrid, and reference/direct oracle for the native arm: twelve controls. Original orbit is secondary, compact primary. An additional independently cold table/Fast/one-branch A/A arm checks host variability and identical workload counts.

*Correctness.* All 100 group-gated compact solves compare every generated signed orbit against the reference, check 265,559 raw principal identities and 4,449 projected anchor identities, and verify the answer. All 1,500 timed cells have the expected scalar and target; every A/A pair agrees on catalogue size, trials, relation count, solve retries, scalar and target. Native receipt collection validates the frozen build ID, arguments and fixture SHA256 before native jq reduction. These remain producer self-checks, not signed independent replay.

*Timing.* One sequential fresh process per cell, L0 macOS ARM. Method clocks charge all cold catalogue/tables, anchors, inverses, descents, attempts, forest, solver and final group verification. Process launch, fixture parsing/validation and construction of the known-answer target are excluded equally. A further harness check is timed separately. A/A spread reaches *3.09x*, so no statistical confidence or isolation claim follows. Per-target raw values, all controls and diagnostic repeats remain available.

*Caps and custody.* Selection child cap 30s and cumulative 600s; gate child cap 60s and cumulative 1,800s; timed child cap 30s and cumulative 1,800s. Existing candidate limits and two-million control trials remain fixed. No selection failure, timeout or unstarted cell occurred. Terminal receipts are immutable and resumption rejects a changed build, argument or input. No cloud worker was used. Scientific execution, collection and reduction use Rust/native jq; the historical shared record wrapper only logged initial selection bookkeeping.

*Observed cost structure.* The phase fractions below are medians of per-solve fractions across the two families and five targets at the largest field of each genus. They are descriptive summaries; sums of independently computed medians need not equal one.

#table(columns: (0.5fr,0.8fr,1fr,1fr,1fr,1fr), align: (left,right,right,right,right,right), inset: 4pt,
table.header([*g*],[*p*],[*relation*],[*forest*],[*solve*],[*setup*]),
[2],[65537],[47.8%],[31.7%],[0.9%],[10.7%],
[3],[1009],[48.6%],[9.8%],[17.5%],[10.3%],
[4],[127],[68.5%],[3.1%],[3.8%],[7.3%],
)

*Decision.* Stop equivalent arithmetic rewrites. Investigate the structural cost of H-wide affine potentials and dense anchor elimination: genus-2 forest share grows, and genus-3 solve share grows as H increases. A sparse short-cycle representation and an appropriate sparse anchor solve need a fresh cost argument, literature screen and correctness protocol before implementation. No larger-size exponent or mathematical breakthrough is inferred from this ladder.

*Sources and reproduction.* `research/hec_anchor_relations_20261114/HELDOUT-PROTOCOL.md`, `heldout.rs`, `heldout_collect.rs`, `heldout_summary.jq`, `heldout_render.rs`, and the frozen raw archive. Prior art and assumptions are in the principal/anchor protocols (GTTD signed cycles and principal-divisor geometric IC). External fastest-tool comparison and independent controlled replay remain outstanding. No benchmark-index or leaderboard promotion.
