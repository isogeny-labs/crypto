#!/usr/bin/env python3
"""Rule v4's report.typ and report.pdf, from analysis.json, recipe.json,
training/summary.json and replay-certificate.json only.  The measurement and
the analysis are native (src/bin/ic_rule_v4.rs); this writes text.
Usage: make_report.py <rule/v4 dir> (needs plot.py's figures beside it)."""
import json
import os
import sys

root = sys.argv[1]
A = json.load(open(f"{root}/analysis.json"))
R = json.load(open(f"{root}/recipe.json"))
T = json.load(open(f"{root}/training/summary.json"))
L = json.load(open(f"{root}/training/lever.json"))
try:
    P = json.load(open(f"{root}/replay-certificate.json"))
except OSError:
    P = None
S = sorted(A["sizes"], key=lambda s: s["log2_r"])


def ci(d, k=3):
    return f"{d['value']:.{k}f} [{d['ci95'][0]:.{k}f}, {d['ci95'][1]:.{k}f}]"


def ratio(r, k=3):
    return f"{r['value']:.{k}f} [{r['ci95'][0]:.{k}f}, {r['ci95'][1]:.{k}f}]"


rows = []
for s in S:
    p = s["paired_v4_over_v3"]
    aa = s.get("aa_v3b_over_v3")
    pos = s.get("lever_control_over_v3")
    rows.append(
        f"[{s['n']}], [{s['log2_r']:.1f}], [{ratio(p['cold'])}], [{s['verdict'].replace('_', ' ')}], "
        f"[{ratio(p['setup'])}], [{ratio(p['online'], 2)}], "
        f"[{aa['cold_ratio']:.3f}], [{pos['cold_ratio']:.2f}]" if aa and pos else
        f"[{s['n']}], [{s['log2_r']:.1f}], [{ratio(p['cold'])}], [{s['verdict'].replace('_', ' ')}], [{ratio(p['setup'])}], [{ratio(p['online'], 2)}], [], []"
    )
gap = []
for s in S:
    g = s["gap_ic_cold_over_rho_cold"]
    sv = s["s"]
    gap.append(
        f"[{s['n']}], [{sv['ic_cold_v3']:.2f}], [{sv['ic_cold_v4']:.2f}], [{sv['rho_cold_canonical_v4proc']:.3f}], "
        f"[{g['v3_canonical']['value']:.2f}], [{g['v4_canonical']['value']:.2f} [{g['v4_canonical']['ci95'][0]:.2f}, {g['v4_canonical']['ci95'][1]:.2f}]], "
        f"[{g['v3_as_built']['value']:.2f}], [{g['v4_as_built']['value']:.2f}]"
    )
band = []
for s in S:
    b = s["band_5a"]
    band.append(
        f"[{s['n']}], [{b['w0']['gap_v4']:.2f}], [{b['w0_1']['gap_v4']:.2f}], [{b['w1']['gap_v4']:.2f}], "
        f"[{b['w0']['v4_over_v3']:.3f}], [{b['w0_1']['v4_over_v3']:.3f}], [{b['w1']['v4_over_v3']:.3f}], "
        f"[{'yes' if b['order_holds_across_band'] else 'no'}]"
    )
rhoc = []
for s in S:
    c = s["rho_reference_check"]
    f = lambda k, d=1: ("-" if c.get(k) is None else f"{c[k]:.{d}f}")
    rhoc.append(f"[{s['n']}], [{f('parallel_rho_ns_per_step_median_T01_T08')}], [{f('strong_fixture_ns_per_step_median')}], [{f('parallel_over_strong', 2)}], [{c.get('parallel_rho_admissible')}], [{f('strong_units_per_step')}], [{s['rho']['units_per_step_as_built_v4proc']:.1f}], [{s['rho']['canonical_step_units_v4proc']:.2f}]")
lev = []
for s in S:
    v = L[s["curve"]]
    lev.append(f"[{s['n']}], [{v['pairs']}], [{v['cost_on_over_off']:.3f}], [{v['log_cost_se']:.3f}], [{[b for b in v['by_columns'] if abs(b['columns_over_v3'] - 1) < 0.02][0]['trials_on_over_off']:.3f}], [{[b for b in v['by_columns'] if abs(b['columns_over_v3'] - 1) < 0.02][0]['build_on_over_off']:.2f}]")
rec = []
for s in S:
    sl = s["curve"]
    c = R["sizes"][sl]
    v3p = json.load(open(f"{root}/../v3/runs/{sl}/T01.params.json"))
    pr = c["model"]["predicted_ratio"] if c.get("accepted") else 1.0
    rec.append(
        f"[{s['n']}], [{v3p['factor_base']['spec']['points'] // (2 * s['n'])} / {c['points'] // (2 * s['n'])}], "
        f"[{v3p['collection_window']} / {c['collection_window']}], "
        f"[{v3p['collection']['unit_trials']} / {c['unit_trials']}], [{pr:.3f}]"
    )

typ = f"""#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)
#set par(justify: true)
= Rule v4: a recipe against v3's recipe, paired, on the Table A sizes

*Outcome: {A['outcome']}.* {A['sizes_improved']} of {A['sizes_read']} sizes improve under the frozen rule (paired cold ratio v4 over v3 with a 95% bootstrap interval below 1); {A['sizes_regressed']} regress. Class: engineering where it improves (S falls, the method is the same). Source: `analysis.json`, `recipe.json`, `training/`, `PROTOCOL.md` (frozen before any timed row; amendments A1 to A7 in it), `replay-certificate.json`. The v4 arm is the v3 recipe with K, the collection window and the unit size retuned on training seeds (lever (ii)); lever (i), free relations from the pair table's duplicate keys, was implemented and measured on the training grid (72 paired runs a size: 5 to 9% *more* cost) and is off. Same binary for both arms; one `isolated_bench` process per row; 16 recipe seeds, 2 targets each; this host has no GFNI.

== Paired cold ratio, v4 recipe over v3 recipe
#table(columns: 8, align: (right, right, left, left, left, left, right, right),
[n], [log2 r], [cold, v4 / v3 [95%]], [verdict], [set-up [95%]], [online [95%]], [A/A], [pos. ctl],
{', '.join(rows)})
The A/A is the v3 recipe against itself (its interval must contain 1; otherwise the size is UNRESOLVED). The positive control is the v3 recipe with the free-relation lever on (a slowdown of known direction, about 1.05 to 1.09 in the training pairs).

#image("paired-cold.svg", width: 100%)

== Distance remaining to rho (index calculus cold / rho cold, rho in the same process)
S is cost / sqrt(r) in the process's own batched-addition unit. Canonical step: rho priced at one batched addition plus the table canonicalisation. As built: rho's measured walk. Above 1, rho is cheaper.
#table(columns: 8, align: (right, right, right, right, right, left, right, right),
[n], [S_IC v3], [S_IC v4], [S_rho canonical], [gap v3 canon.], [gap v4 canon. [95%]], [gap v3 as built], [gap v4 as built],
{', '.join(gap)})

#image("gap-to-rho.svg", width: 100%)

== Section 5a band (counted group-addition equivalents; unpriced native work at 0, 0.1, 1 GAE)
#table(columns: 8, [n], [gap v4 w=0], [w=0.1], [w=1], [v4/v3 w=0], [w=0.1], [w=1], [order holds],
{', '.join(band)})

Counted GAE is a lower bound: the build's cache misses, the linear algebra and the descent's bookkeeping are not priced, so it can rank two recipes differently from wall time (it prices a larger table's build at one addition a pair). "Order holds" = v4 below v3 at all three weights, interval upper end below 1.

== The reference: rho against the strong fixture, T01 to T08, ns per step
ParallelRho is the rho of the `ic price` process. At the second half of n = 41 and at n = 53, 59 and 61 those processes ran with the neighbours' jobs active (amendment A8); the strong fixture rows ran with none, so ParallelRho reads slower against it there than it would. Units: the strong fixture's ns a step over this arm's batched-addition unit.
#table(columns: 8, [n], [ParallelRho ns/step], [strong ns/step], [ratio], [admissible], [strong units/step], [ParallelRho units/step], [canonical units/step],
{', '.join(rhoc)})

== Lever (i) on the training grid: the same configuration and seed, lever off and on
#table(columns: 6, [n], [pairs], [cost on / off], [se (log)], [trials on / off, v3 K], [build on / off, v3 K],
{', '.join(lev)})

== The recipe selected on training seeds (disjoint from the evaluation targets)
v3 / v4 values; the lever is off in the v4 arm (it cost 5 to 9% in the training pairs). Predicted: the fitted cost of the v4 configuration over v3's on the training seeds (it is a prediction, not a result).
#table(columns: 5, [n], [columns], [window], [unit trials], [predicted v4/v3],
{', '.join(rec)})
"""
if P:
    typ += f"""
== Checks
Replay: {P['process_count']} processes replayed in a separate process with `icprog`'s oracle, {P['failures']} failures; {len(P['untimed_reruns'])} untimed re-runs. Status {P['status']}.
"""
open(f"{root}/report/report.typ", "w").write(typ)
import typst  # noqa: E402

typst.compile(f"{root}/report/report.typ", output=f"{root}/report/report.pdf")
print("wrote report.typ and report.pdf")
