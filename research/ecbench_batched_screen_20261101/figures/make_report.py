#!/usr/bin/env python3
"""Writes REPORT.typ from summary.json (every figure of merit is computed here,
none is typed).  Standard library only."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(HERE)
S = json.load(open(f"{HERE}/summary.json"))
C = ["n13a0", "n17a1", "n19a0", "n23a0"]; B = [1, 2, 4, 8, 16, 32, 64, 128, 256]
arm = lambda c, b: S["cells"][c]["arms"][f"ic-b{b}"]
tot = lambda c, b: arm(c, b)["mean_gae_total"]
rat = lambda c, b: (1.0, (1.0, 1.0)) if b == 64 else (arm(c, b)["ratio_to_b64"]["ratio"], arm(c, b)["ratio_to_b64"]["ci95"])
setup = lambda c, b: arm(c, b)["phase_gae"]["oracle_setup"] / tot(c, b)
P = S["predictions"]
rng = lambda f, fmt="{:.3f}": f"{fmt.format(min(f(c) for c in C))}–{fmt.format(max(f(c) for c in C))}"
held = [k for k in P if P[k]["held"]]; missed = [k for k in P if not P[k]["held"]]
mq = lambda c, b: arm(c, b)["counters"]["collection_trials"]
at = lambda c, b: arm(c, b)["counters"]["solve_attempts"]
rows = "\n".join(
    f"    [`ic-b{b}`], " + ", ".join(f"[{rat(c,b)[0]:.3f} \\ [{rat(c,b)[1][0]:.3f}, {rat(c,b)[1][1]:.3f}]]" for c in C) + ","
    for b in B if b != 64)
qrows = "\n".join(f"    [`b{b}`], " + ", ".join(f"[{mq(c,b):.1f} / {at(c,b):.2f}]" for c in C) + "," for b in B)
prow = lambda k, t: f"    [{k}], [{t}], [{'held' if P[k]['held'] else '*not held*'}],"
argmin = P["P4"]["evidence"]["argmin_batch"]
doc = f'''#set page(paper: "a4", margin: (x: 1.9cm, y: 1.9cm), numbering: "1")
#set text(size: 9.6pt)
#set par(justify: true)
#show heading: set block(above: 1.1em, below: 0.6em)
#show raw: set text(size: 8.6pt)

= `batch_trials` development screen on `ic.koblitz_batched`

*Local, diagnostic, counted-unit screen. Not a registered tournament round, not isolated, not promotable.* Protocol `PROTOCOL.md` (frozen in `d98cc0d107`, Amendment 1 before any data in `60a451606b`). Instrument: `ic.koblitz_batched`, PR \\#3134 head `729847dcea` (the sessions record commit `39f147d96b`; `src/`, `Cargo.*` and `examples/` are identical between the two).

== Result

Over round 0026's four development cells, the counted cost (group additions and doublings, GAE) of the tournament worker's pair-table pipeline *falls monotonically as `batch_trials` shrinks, down to 1*. Against the incumbent `batch_trials = 64`, `batch16` is cheaper by a factor of {rng(lambda c: rat(c,16)[0])} (paired 95% intervals all below 1), and `batch1` by {rng(lambda c: rat(c,1)[0])}. Larger batches cost more: `batch256` is {rng(lambda c: rat(c,256)[0], "{:.2f}")} times the incumbent. All {sum(arm(c,b)["runs"] for c in C for b in B)+sum(arm(c,"64-aa")["runs"] for c in C)} measured records verified, and every measured record was replayed exactly (1280 replays per cell, all reproduced, 0 problems; the 320 warm-up records per cell are verified but not replayed), and the A/A control reads {S["cells"]["n13a0"]["aa"]["ratio"]:.3f} on all four cells.

*Two of the five predictions frozen in the protocol held ({", ".join(held)}); three did not ({", ".join(missed)}).*

#table(columns: (auto, 1fr, auto), align: (left, left, left), stroke: 0.4pt + luma(170),
  table.header([*id*], [*prediction (frozen before data)*], [*outcome*]),
{prow("P1", "queries drawn are 8 for batch ≤ 8 and equal to the batch for batch ≥ 16")}
{prow("P2", "GAE rises monotonically from batch 16 to 256; 128 and 256 are the two most expensive arms")}
{prow("P3", "GAE is non-increasing in batch from 1 to 8 (per-batch overhead)")}
{prow("P4", "the minimum over the nine arms lies in \\{4, 8, 16\\} on every cell")}
{prow("P5", "`ic-b16 / ic-b64` < 1, upper 95% bound below 1, on every cell")}
)

- *P1 not held.* Batches 32 and above draw exactly their batch (every run is one batch). Smaller batches draw more than eight queries on average, because a run needs a few relations beyond the {S["cells"]["n13a0"]["arms"]["ic-b64"]["counters"]["columns"]:.0f} to {S["cells"]["n19a0"]["arms"]["ic-b64"]["counters"]["columns"]:.0f} columns before the solve succeeds (table below). The three single jobs I pinned against the worker at batch 4 and 2 (n13a0, n17a1, n19a0) happened to need eight.
- *P3 and P4 not held, in the opposite direction.* There is no per-batch overhead in the counted unit: batch 1 is the cheapest arm on every cell (argmin: {", ".join(f"{c} b{argmin[c]}" for c in C)}), and cost rises with the batch all the way from 1.
- *P2 and P5 held.*

== Paired ratio to the incumbent (`ic-b64`), counted GAE

Cells: `n13a0` (log₂ r 10.97), `n17a1` (16.00), `n19a0` (17.00), `n23a0` (21.00); 32 public targets each, paired by target; cluster-bootstrap 95% intervals from `ecbench compare --save`. A ratio below 1 means the arm is cheaper than `batch_trials = 64`.

#table(columns: (auto, 1fr, 1fr, 1fr, 1fr), align: (left, center, center, center, center), stroke: 0.4pt + luma(170),
  table.header([*arm*], [*n13a0*], [*n17a1*], [*n19a0*], [*n23a0*]),
{rows}
)

#figure(image("figures/fig1-ratio.svg", width: 100%), caption: [Counted GAE of each arm relative to the incumbent, by cell.])

== Why: the setup is most of the cost, and small batches stop sooner

#figure(image("figures/fig2-phases.svg", width: 100%), caption: [Counted GAE per phase (mean per run). `oracle_setup`, the pair table, is {setup("n13a0",64)*100:.0f}% (n13a0) to {setup("n23a0",64)*100:.0f}% (n23a0) of the incumbent's cost and does not depend on `batch_trials`; only `relations` and `linear_algebra` move.])

At `batch_trials = 64` a run draws 64 queries and makes one solve attempt; at `batch_trials = 1` it stops after about {rng(lambda c: mq(c,1), "{:.1f}")} queries and makes {rng(lambda c: at(c,1), "{:.1f}")} solve attempts. Each query costs a roughly constant number of GAE in the relation phase, so the surplus queries of a large batch are the whole difference. Mean queries drawn / mean solve attempts per run:

#table(columns: (auto, 1fr, 1fr, 1fr, 1fr), align: (left, center, center, center, center), stroke: 0.4pt + luma(170),
  table.header([*arm*], [*n13a0*], [*n17a1*], [*n19a0*], [*n23a0*]),
{qrows}
)

The extra solve attempts of the small batches are cheap in the counted unit, and also in the native work the unit leaves unpriced: the sparse-solve row operations per run fall from {S["cells"]["n13a0"]["arms"]["ic-b64"]["uncharged"]["la_row_ops_uncharged"]:.0f} at `batch64` to {S["cells"]["n13a0"]["arms"]["ic-b1"]["uncharged"]["la_row_ops_uncharged"]:.0f} at `batch1` on n13a0 (n19a0: {S["cells"]["n19a0"]["arms"]["ic-b64"]["uncharged"]["la_row_ops_uncharged"]:.0f} to {S["cells"]["n19a0"]["arms"]["ic-b1"]["uncharged"]["la_row_ops_uncharged"]:.0f}), and the pair-table bucket reads and filter probes fall with the batch too. In the five unpriced native counters recorded (sparse-solve row operations, pair-filter probes, bucket reads, Frobenius maps, canonicalisations), `batch16` is not above `batch64` on any cell: {"yes" if all(all(v.values()) for v in S["b16_native_not_above_b64"].values()) else "not on every cell (see summary.json)"}.

== What this does and does not show

- *Direction agrees with round 0026.* Round 0026 measured `batch16` at 0.630× the incumbent in Valgrind instruction reads. Here it is {rng(lambda c: rat(c,16)[0], "{:.2f}")}× in GAE. The units differ and the magnitudes are not comparable; the sign agrees.
- *The counted unit cannot find the true minimum.* GAE prices group operations only. It does not price the fixed cost of one solve attempt (building the matrix, allocation), field arithmetic outside the group law, hashing, or any wall time. Those costs are what push back against very small batches, and the measurements that see them (instruction reads, isolated wall time) are the ones round 0026 and the open isolated confirmation use. So this screen does *not* show that `batch_trials = 1` is best; it shows that, on the work GAE and the recorded native counters see, nothing counts against small batches.
- *Round 0026's panel stopped at 16.* Of its default candidates, `batch16`, `batch32` and `batch128` varied `batch_trials`; the tournament's own refinement rule around a selected batch (`batch/2`, `batch/4`, `2·batch`, `4·batch`, 1, 2, 8, 256 in `src/bin/icprog/tournament.rs`) would try 8, 4, 2 and 1 next. This screen says those are worth including in the next registered, isolated round.
- *Wall time is not measured here.* The sessions are isolation L0 (`--cpus none`); the wall figures in the records are descriptive only. The open round 0026 question, wall time under a core reservation, needs the isolated round.
- *Not claimed:* anything about larger curves, other factor bases, `dense`, `excess8`, `window8`, rho, or scaling. No cross-cell pooling. Nothing here is promotable.

== Evidence and reproduction

- Executed by `tools/task_card.py run` on `CARD-ecbench-batched-screen-20261101-1` (63 commands, exit 0, no retries); the director's assessment (`cards/card-1.assessment.md`) is `executed_as_specified`.
- `audits/<cell>.verify.txt`: `ecbench verify --replay-all` on each session, 1600 records verified and 1280 replays reproduced, 0 problems. The ecbench binary is `447a6d3c4842…` in all four sessions.
- `summary.json` is made by `figures/make_summary.py` from the verified records and the saved comparisons; this report is made from it by `figures/make_report.py`. `comparisons/` holds each arm's comparison text.
- The four `sessions/<cell>/records.jsonl` files (17 MB each) are in the artifact store, with an `.artifacts.json` manifest per cell; `python3 tools/artifacts/artifacts.py pull research/ecbench_batched_screen_20261101/sessions` hydrates them. Run on them, `figures/make_summary.py` reproduces `figures/summary.json` byte for byte.
'''
open(f"{ROOT}/REPORT.typ", "w").write(doc)
print("wrote REPORT.typ", len(doc), "chars")
