#!/usr/bin/env python3
"""Generate N83 root11 local fine-grid benchmark reports from committed records."""
import json, math, shutil
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parents[4]
STUDY = ROOT / "research/n83_root11_finegrid_holdout_20261017"
RESULTS = STUDY / "results"
OUT = Path(__file__).resolve().parent
BENCH = ROOT / "benchmarks/20261017-n83-root11-finegrid-screen"
KS = [2752, 2816, 2880, 2944, 3072]
def read(path):
    return json.loads(path.read_text())

rows = [read(RESULTS / f"batch_k{k}_result.json") for k in KS]
selection = read(RESULTS / "local_selection.json")
host = read(STUDY / "single_target/host.json")
stratum = read(RESULTS / "stratum3_analysis.json")
assert [r["k"] for r in rows] == KS
assert selection["status"] == "SELECTED_BY_COMPLETE_REPLAY"
assert selection["selected_k"] == 3072
assert all(r["target_count"] == 128 and r["exit_code"] == 0 for r in rows)
assert all(r["status"] == "PASS_INDEPENDENT_BASE_RELATION_SCALAR_REPLAY" for r in rows)
assert all(len(r["per_target_online_ms"]) == 128 for r in rows)
assert stratum["status"] == "BOUNDED_UNKNOWN"
point18 = next(p for p in stratum["points"] if p["point_index"] == 18)
selected_id = next(x["candidate_id"] for x in selection["rows"] if x["k"] == 3072)
k2816_id = next(x["candidate_id"] for x in selection["rows"] if x["k"] == 2816)
spreads = point18["calibration_by_candidate"]
selected_spread = spreads[selected_id]["relative_spread"] * 100
k2816_spread = spreads[k2816_id]["relative_spread"] * 100
baseline = next(r for r in rows if r["k"] == 3072)
table = []
for r in rows:
    paired = [math.log(a / b) for a,b in zip(r["per_target_online_ms"], baseline["per_target_online_ms"])]
    mean = sum(paired) / len(paired)
    sd = math.sqrt(sum((x-mean)**2 for x in paired)/(len(paired)-1))
    ratio = math.exp(mean)
    low, high = math.exp(mean - 1.979*sd/math.sqrt(len(paired))), math.exp(mean + 1.979*sd/math.sqrt(len(paired)))
    table.append({"k":r["k"],"points":r["actual_factor_base_points"],"columns":r["folded_columns"],
      "geo":r["online_geometric_mean_ms"],"median":r["online_median_ms"],
      "cold":r["cold_process_ms"]/1000.0,"ratio":ratio,"low":low,"high":high})
assert all(abs(a["geo"] - b["geometric_mean_online_ms"]) < 1e-9 for a,b in zip(table, selection["rows"]))
assert abs(table[-1]["ratio"] - 1.0) < 1e-12

run_md = f"""# N83 root11 fine-grid screen - run report

Run date: 2026-10-17
Report generated: 2026-10-21
Scope: Local exploratory arm selection on 128 public N83 points. No new benchmark was run to prepare this report.

## Frozen procedure

Five fixed K arms ({", ".join(map(str, KS))}) ran sequentially on the same ordinary macOS ARM64 host. The sealed public batch contains 128 targets (SHA-256 {selection["batch_targets_sha256"]}). Each arm independently replayed its factor base, four-point relations, fixture and recovered scalars, target timing phases, and source/binary binding. Each arm exited successfully; full records are in [results](../../results/).

The selector was frozen as minimum geometric mean target-online interval across the 128-point local batch, with smaller K as the tie-break. See [RESULT.md](../../RESULT.md), [PROTOCOL.md](../../PROTOCOL.md), [full replay](../../results/batch_full_replay.json), [selection record](../../results/local_selection.json), and [host receipt](../../single_target/host.json) for complete provenance.

## Host and isolation

- Platform: {host["platform"]}; machine: {host["machine"]}; RAM: 48 GiB.
- Isolation: shared ordinary workstation; no dedicated or cloud-host isolation claim.
- Arms ran sequentially, not concurrently. Process interval includes base construction, root-index and log preparation, all 128 target queries, and producer checks.
- This is an exploratory local selection screen, not production or Tier 1 performance qualification.

## Reproduction

Run from repository root with Python 3 and Matplotlib installed:

    python3 research/n83_root11_finegrid_holdout_20261017/reports/finegrid_benchmark/build_reports.py

The script reads committed per-arm JSON, selection, host receipt, and canonical stratum analysis. It writes this source report set and copies PDFs and graph into [the dated benchmark directory](../../../../benchmarks/20261017-n83-root11-finegrid-screen/). SVG and PNG use the same Matplotlib figure; PNG is exported at 150 dpi.
"""
def row_md(t):
    interval = "reference" if t["k"] == 3072 else "[%.3f, %.3f]" % (t["low"], t["high"])
    return f'| {"**" if t["k"]==3072 else ""}{t["k"]}{"**" if t["k"]==3072 else ""} | {t["points"]:,} | {t["columns"]:,} | {t["geo"]:.3f} | {t["median"]:.3f} | {t["cold"]:.3f} | {t["ratio"]:.3f} | {interval} |'
table_md = "\n".join(row_md(t) for t in table)
results_md = f"""# N83 root11 fine-grid screen - results

Selection: K3072/root11 under the frozen minimum-geometric-mean target-online rule on these same 128 points. This is a selected local screen, not an independent confirmation set.

| K | Usable points | Folded columns | Geometric online mean (ms) | Median online (ms) | Complete shared-setup process (s) | Geometric ratio K/K3072 | Descriptive paired 95% interval |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
{table_md}

K3072 has the lowest observed target-online geometric mean (16.405 ms). K2816 had the shortest complete shared-setup process (367.939 s); K3072 took 491.832 s. K3072 was faster than K2816 on 64 of 128 points. The paired K2816/K3072 geometric ratio is 1.078 with descriptive 95% interval [0.813, 1.430].

Intervals use paired per-point log ratios and the 1.979 standard-error multiplier specified in the source result. Since K3072 was selected on these same points, intervals are descriptive after selection and do not establish a general speedup. Do not treat this local screen as a fresh-point result.

## Limits of the separate cloud panel

The separate 24-point cloud Tier 1 selector remains BOUNDED_UNKNOWN. Corrected independent replay admits relations and scalars but marks stratum 3 NOT_ADMITTED: point 18 calibration spreads are {k2816_spread:.3f}% for K2816 and {selected_spread:.3f}% for K3072, both above the frozen 10% threshold. See [STRATUM3_CENSORED.md](../../results/STRATUM3_CENSORED.md) and canonical [stratum3_analysis.json](../../results/stratum3_analysis.json).

The earlier producer diagnostic reports different point-18 spreads (12.471% and 13.355%). That discrepancy has not been reconciled; this report uses corrected independent-replay values above and does not rely on the producer diagnostic calibration percentages. No cloud-wide selector result or speedup is claimed.

## Provenance

Selected candidate: {selected_id}. The 128 public target input SHA-256 is {selection["batch_targets_sha256"]}. The 640 per-arm target intervals and probe counts remain in the committed [result files](../../results/). See [RESULT.md](../../RESULT.md) for the full run record. Source tree, protocol, inputs, source binding, and replay receipts remain the record of truth.\n\n![Fine-grid screen](screen.png)
"""

def make_plot(svg_path, png_path):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5.7), constrained_layout=True)
    x = list(range(len(KS)))
    geos = [t["geo"] for t in table]
    ax1.plot(x, geos, color="#334e68", linewidth=2, marker="o")
    ax1.scatter([4], [geos[4]], s=95, color="#c35b52", zorder=4, label="Selected by local rule")
    ax1.set_xticks(x, [str(k) for k in KS])
    ax1.set_xlabel("Folded columns K")
    ax1.set_ylabel("Geometric target-online mean (ms)")
    ax1.set_title("128-point local screen")
    ax1.grid(axis="y", alpha=.25)
    ax1.legend(frameon=False, loc="upper right")
    ratios = [t["ratio"] for t in table]
    lows = [t["low"] for t in table]
    highs = [t["high"] for t in table]
    ax2.errorbar(x, ratios, yerr=[[v-l for v,l in zip(ratios,lows)],[h-v for v,h in zip(ratios,highs)]],
                 fmt="o", capsize=4, color="#334e68", ecolor="#779955", linewidth=1.5)
    ax2.axhline(1.0, color="#c35b52", linestyle="--", linewidth=1)
    ax2.set_xticks(x, [str(k) for k in KS])
    ax2.set_xlabel("Folded columns K")
    ax2.set_ylabel("Paired geometric ratio vs K3072")
    ax2.set_title("Descriptive paired intervals")
    ax2.grid(axis="y", alpha=.25)
    ax2.text(.02, .02, "Post-selection descriptive intervals.", transform=ax2.transAxes, fontsize=8, color="#52606d")
    fig.suptitle("N83 root11 fine-grid arm selection | 2026-10-17", fontsize=14)
    fig.savefig(svg_path, format="svg")
    fig.savefig(png_path, format="png", dpi=150)
    return fig

OUT.mkdir(parents=True, exist_ok=True)
BENCH.mkdir(parents=True, exist_ok=True)
(OUT/"run_report.md").write_text(run_md)
(OUT/"results_report.md").write_text(results_md)
fig = make_plot(OUT/"screen.svg", OUT/"screen.png")
with PdfPages(OUT/"run_report.pdf", metadata={"Title":"N83 root11 fine-grid run report"}) as pdf:
    page=plt.figure(figsize=(8.27,11.69)); page.clf()
    page.text(.08,.94,"N83 root11 fine-grid screen",fontsize=20,weight="bold")
    page.text(.08,.90,"Run report | 2026-10-17 | generated 2026-10-21",fontsize=10)
    page.text(.08,.85,"Five K arms ran sequentially on one ordinary macOS ARM64 host.",fontsize=10)
    page.text(.08,.81,"Each arm independently replayed 128 public points and exited 0.",fontsize=10)
    page.text(.08,.76,"Selection rule: lowest geometric mean target-online interval;",fontsize=10)
    page.text(.08,.72,"ties go to smaller K. K3072 was selected.",fontsize=10)
    page.text(.08,.65,"Shared workstation, 48 GiB RAM; no dedicated-host claim.",fontsize=10)
    page.text(.08,.61,"Process interval includes base/index/log preparation, all queries,",fontsize=10)
    page.text(.08,.57,"and producer checks. This is an exploratory selection screen.",fontsize=10)
    page.text(.08,.47,"Targets SHA-256: "+selection["batch_targets_sha256"],fontsize=8)
    page.text(.08,.43,"Executed source SHA-256: 8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12",fontsize=7)
    page.text(.08,.36,"Records: research/n83_root11_finegrid_holdout_20261017/results/",fontsize=9)
    page.text(.08,.31,"No new benchmark was run to prepare this report.",fontsize=10,style="italic")
    pdf.savefig(page); plt.close(page)
    pdf.savefig(fig)
with PdfPages(OUT/"results_report.pdf", metadata={"Title":"N83 root11 fine-grid results"}) as pdf:
    page=plt.figure(figsize=(8.27,11.69)); page.clf()
    page.text(.08,.94,"N83 root11 fine-grid screen",fontsize=20,weight="bold")
    page.text(.08,.90,"Results | selected local screen, not a speedup claim",fontsize=10)
    page.text(.08,.85,"K     points    cols     online ms    median ms    cold s    ratio K/K3072",fontsize=9,family="monospace")
    y=.81
    for t in table:
        page.text(.08,y,f'{t["k"]:4d} {t["points"]:8,d} {t["columns"]:6,d} {t["geo"]:11.3f} {t["median"]:11.3f} {t["cold"]:9.3f} {t["ratio"]:14.3f}',fontsize=9,family="monospace")
        y-=.035
    page.text(.08,y-.02,"K3072: 16.405 ms online geometric mean; K2816 had shortest",fontsize=9)
    page.text(.08,y-.05,"cold process at 367.939 s versus 491.832 s for K3072.",fontsize=9)
    page.text(.08,y-.11,"Intervals are paired and descriptive after selecting K3072 on",fontsize=9)
    page.text(.08,y-.14,"the same 128 targets; they do not establish a speedup.",fontsize=9)
    page.text(.08,y-.22,"24-point cloud panel: BOUNDED_UNKNOWN; stratum 3 not admitted.",fontsize=9,weight="bold")
    page.text(.08,y-.26,f"Point-18 calibration: K2816 {k2816_spread:.3f}%, K3072 {selected_spread:.3f}%.",fontsize=8)
    page.text(.08,y-.30,"Producer diagnostic has different values; discrepancy unresolved.",fontsize=8)
    pdf.savefig(page); plt.close(page)
    pdf.savefig(fig)
plt.close(fig)
for name in ("run_report.pdf","results_report.pdf","screen.svg","screen.png"):
    shutil.copy2(OUT/name, BENCH/name)
print(json.dumps({"table":table,"output":str(OUT),"benchmark":str(BENCH),"png_dpi":150,"status":"generated"}))
