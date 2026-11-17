"""Report layout only: plot and format statistics already computed by native code.

No benchmark execution, root finding, scalar recovery, fitting or statistical
estimation occurs in this file. Values come directly from results/report-data.json.
"""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak

HERE = Path(__file__).resolve().parent
OUT = HERE / "reports" / "run-20261117T051441Z"
OUT.mkdir(parents=True, exist_ok=True)
DATA = json.loads((HERE / "results/report-data.json").read_text())
CUSTODY = json.loads((HERE / "results/LOCAL_CUSTODY.json").read_text())
ROWS = {(r["B"], r["arm"]): r for r in DATA["table"]}
BS = [8, 16, 32, 64, 128]
LABELS = {
    "known": "Affine + Horner", "compose": "Window + composition",
    "pairs": "Prior pair control", "straus-pairs": "Straus + same pairs",
    "window-pairs": "Window + same pairs", "window-horner": "Window + Horner",
    "support": "Full support", "support-bloom": "Full support + Bloom",
    "first": "First witness", "first-bloom": "First witness + Bloom",
    "first-bloom-aa": "First witness + Bloom (A/A)", "rho": "Matched rho",
}
ARMS = ["known", "window-horner", "compose", "pairs", "straus-pairs", "window-pairs",
        "support", "support-bloom", "first", "first-bloom", "first-bloom-aa", "rho"]

def ms(value):
    return f"{value * 1000:.3f}"

def ratio(name, b=128):
    return DATA["paired_completed_only"][name][str(b)]["wall_ratio"]["median"]

def save(fig, name):
    fig.savefig(OUT / f"{name}.svg", bbox_inches="tight")
    fig.savefig(OUT / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)

plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False})
fig, ax = plt.subplots(figsize=(9.2, 5.0))
for arm in ["known", "compose", "pairs", "first", "first-bloom", "rho"]:
    ax.plot(BS, [ROWS[b, arm]["wall_seconds"]["median"] * 1000 for b in BS], marker="o", label=LABELS[arm])
ax.set_yscale("log")
ax.set_xscale("log", base=2)
ax.set_xticks(BS, labels=BS)
ax.set(xlabel="Pool size B; subgroup order approximately 8 B³", ylabel="Median bounded target wall time (ms)",
       title="Complete charged target work across the fixed panel")
ax.grid(True, alpha=.18)
ax.legend(fontsize=8.5, ncols=2)
fig.text(.02, .01, "Local a2 · 12 distinct targets per B × 5 measured rounds · misses and four-attempt censored units retained\n"
                    "Descriptive medians; observed ranges are in the report data. No fitted exponent or confidence interval.", fontsize=8)
fig.tight_layout(rect=(0, .075, 1, 1))
save(fig, "scaling")

fig, ax = plt.subplots(figsize=(9.0, 6.0))
palette = ["#5b6f8a" if a in ("known", "window-horner", "compose") else "#00877a" for a in ARMS]
palette[-1] = "#b15c21"
for i, arm in enumerate(ARMS):
    d = ROWS[128, arm]["wall_seconds"]
    value, low, high = d["median"] * 1000, d["min"] * 1000, d["max"] * 1000
    ax.barh(i, value, color=palette[i], alpha=.85)
    ax.errorbar(value, i, xerr=[[value-low], [high-value]], fmt="none", color="#333333", capsize=2, lw=.8)
    ax.text(value * 1.06, i-.13, f"{value:.3f}", fontsize=8)
ax.set_yticks(range(len(ARMS)), labels=[LABELS[a] for a in ARMS])
ax.invert_yaxis()
ax.set_xscale("log")
ax.set(xlabel="Wall time (ms), logarithmic axis", title="B = 128: all registered arms, complete target cost")
ax.grid(axis="x", alpha=.15)
fig.text(.02, .01, "Local a2 · all 60 target-round units complete per arm · whiskers: observed min–max, not confidence intervals", fontsize=8)
fig.tight_layout(rect=(0, .035, 1, 1))
save(fig, "all-arms-b128")

phase_arms = ["pairs", "straus-pairs", "window-pairs", "first", "first-bloom", "support-bloom"]
phases = ["public_setup_and_tables", "pool_generation", "incidence_and_recovery"]
fig, ax = plt.subplots(figsize=(9.0, 4.2))
left = [0.0] * len(phase_arms)
for phase, label, color in zip(phases, ["Public setup + tables", "Actual pool generation", "Incidence + recovery"], ["#637ea5", "#dfac42", "#258d80"]):
    values = [ROWS[128, a]["phase_cpu_means_seconds"][phase] * 1000 for a in phase_arms]
    ax.barh(range(len(phase_arms)), values, left=left, label=label, color=color)
    left = [x+y for x, y in zip(left, values)]  # Drawing coordinates only.
ax.set_yticks(range(len(phase_arms)), labels=[LABELS[a] for a in phase_arms])
ax.invert_yaxis()
ax.set(xlabel="Mean charged CPU time (ms)", title="B = 128: charged phases identify the remaining costs")
ax.legend(fontsize=8, loc="lower right")
fig.text(.02, .01, "Native-computed phase means, 60 completed units per arm; means add. Rho CPU phases were not recorded.", fontsize=8)
fig.tight_layout(rect=(0, .04, 1, 1))
save(fig, "phases-b128")

styles = getSampleStyleSheet()
styles["Normal"].fontSize = 9
styles["Normal"].leading = 12
styles["Title"].fontSize = 20
styles["Heading2"].fontSize = 12

def p(text, style="Normal"):
    return Paragraph(text, styles[style])

def table(rows, widths):
    t = Table(rows, colWidths=widths, repeatRows=1, hAlign="LEFT")
    t.setStyle(TableStyle([("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e7edf2")),
                          ("FONTNAME", (0, 0), (-1, 0), "Helvetica-Bold"),
                          ("FONTSIZE", (0, 0), (-1, -1), 8), ("LEADING", (0, 0), (-1, -1), 10),
                          ("BOTTOMPADDING", (0, 0), (-1, -1), 5), ("TOPPADDING", (0, 0), (-1, -1), 5),
                          ("LINEBELOW", (0, 0), (-1, 0), .5, colors.grey),
                          ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#f5f8fa")])]))
    return t

def footer(canvas, doc):
    canvas.setFont("Helvetica", 7)
    canvas.drawString(36, 20, "PCC-20261117-c1a1e9 · unsigned producer measurement · local custody exception")
    canvas.drawRightString(576, 20, str(doc.page))

source = "f1549fc821220641406b51a60324ef4ec81ab69b"
common = [p("Local native runtime comparison", "Title"),
          p("Run a2 · 17 November 2026 · source " + source), Spacer(1, 9),
          p(f"At B=128, the new first-witness/Bloom path improves on the prior pair control by {ratio('pairs/first-bloom'):.3f}× in the median paired wall comparison. The composition path improves on the same-generator Horner path by {ratio('window-horner/compose'):.3f}×. These are measured local improvements at the tested scope."), Spacer(1, 10)]
rows = [["Arm", "Median wall (ms)", "Median CPU (ms)", "RSS (KiB)"]]
for arm in ARMS:
    r = ROWS[128, arm]
    rows.append([LABELS[arm], ms(r["wall_seconds"]["median"]), "unavailable" if r["cpu_seconds"] is None else ms(r["cpu_seconds"]["median"]), f"{r['peak_rss_kib']['median']:.0f}"])
story = common + [table(rows, [210, 110, 110, 95]), Spacer(1, 12),
    p("Protocol and checks", "Heading2"),
    p("Sixty public synthetic targets; B=8,16,32,64,128 with r approximately 8B³. Each B has twelve distinct targets and five measured rounds; repeated rounds are not independent targets. One warmup is retained. Four pool attempts maximum, stop at first informative relation, all preceding misses charged. Every candidate pays public setup, fixed-window tables, actual mixed labels, rejected draws, norm/index/Bloom construction, recovery and original-curve scalar checking."), Spacer(1, 8),
    p("8346 raw rows: 1391 warmups and 6955 measured. All 4320 planned target-round-arm units are present; no timed-row failure. Each IC arm completes 275/300 measured units and censors 25; rho completes 300/300. At B128 every arm completes 60/60. The separate native post-check replays 3990 public scalars with the frozen Euclidean-inversion pair arithmetic and checks 2904 support agreements plus 7260 generation and existence agreements. Qualifier and dependency hashes match before/after execution."),
    PageBreak(), p("Scaling and retained limitations", "Title"),
    Image(str(OUT / "scaling.png"), width=7.25*inch, height=3.94*inch), Spacer(1, 8)]
small = [["B", "Prior pairs (ms)", "New Bloom (ms)", "Rho (ms)", "IC complete / 60"]]
for b in BS:
    r = ROWS[b, "first-bloom"]
    small.append([str(b), ms(ROWS[b, "pairs"]["wall_seconds"]["median"]), ms(r["wall_seconds"]["median"]), ms(ROWS[b, "rho"]["wall_seconds"]["median"]), str(r["completed"])])
story += [table(small, [45, 120, 120, 105, 135]), Spacer(1, 10),
    p("AMD EPYC 9V74 shared managed host, one pinned CPU, four-CPU quota and 16GiB limit. Kernels are reused exact qualified binaries, not a fresh source build. The registered rho binary has its own c0176f942 provenance. Complete-call clocks exclude process startup/output, which GNU time retains separately at coarse resolution. No L2, confidence interval, strongest-external-tool or security-tier claim follows. Inputs are small prime fields; this is not a cryptographic-size ECDLP result."), Spacer(1, 8),
    p("First attempt stopped before any timed candidate because GNU time was absent; it is retained. Attempt 2 uses Debian GNU time with its package checksum matched. The first post-parser rejected resource-stream EOF; a corrected copied qualifier produced the accepted checks. No measurement was rerun after viewing timing results."), Spacer(1, 8),
    p("Sources: LOCAL_PROTOCOL.json, LOCAL_AMENDMENT_2.json, run-local-v2.sh, postprocess.cpp, results/native-summary.json and report-data.json. The frozen parent protocols/controls remain unchanged. Temporary raw custody is explicitly user-authorized; S3 admission/upload is not claimed. Archive SHA256: " + CUSTODY["archive_sha256"] + ".")]
SimpleDocTemplate(str(OUT / "RUN_REPORT.pdf"), pagesize=(612, 792), leftMargin=36, rightMargin=36, topMargin=34, bottomMargin=35).build(story, onFirstPage=footer, onLaterPages=footer)

ablation_names = [("pairs/straus-pairs", "Existing backend / Straus"), ("straus-pairs/window-pairs", "Straus / batched window"),
                  ("pairs/window-pairs", "Prior / window, same pair kernel"), ("first/first-bloom", "First witness / Bloom"),
                  ("support/support-bloom", "Full support / Bloom"), ("first-bloom/first-bloom-aa", "Identical-source A/A"),
                  ("first-bloom/rho", "New Bloom / rho (cost ratio)")]
ab = [["Matched comparison, B128", "Median wall ratio", "n"]]
for key, label in ablation_names:
    d = DATA["paired_completed_only"][key]["128"]["wall_ratio"]
    ab.append([label, f"{d['median']:.4f}", str(d["n"])])
story = [p("Results and interpretation", "Title"),
    p(f"The first-witness/Bloom path takes 2.493 ms median wall versus 13.393 ms for the prior pair control. The median paired improvement is {ratio('pairs/first-bloom'):.4f}×; ratios are computed per matched target/round before taking the median. The new/rho paired ratio is {ratio('first-bloom/rho'):.4f}, so the measured gain does not establish parity with this rho reference."),
    Image(str(OUT / "all-arms-b128.png"), width=7.2*inch, height=4.8*inch),
    p("The composition route returns the same full support as Horner, and is 4.9883× cheaper in the B128 same-generator paired comparison. Generic full-support enumeration remains much cheaper than composition:56.5550× in the paired compose/support comparison. Full support and first-witness tasks are kept separate."),
    PageBreak(), p("Ablations, remaining cost and scope", "Title"), table(ab, [335, 120, 70]), Spacer(1, 10),
    Image(str(OUT / "phases-b128.png"), width=7.2*inch, height=3.36*inch),
    p("The largest mean CPU phase of the new Bloom path is public setup/tables (1.511ms), followed by incidence/recovery (0.991ms) and actual pool generation (0.416ms). These phase means include prior attempts and sum to the recorded mean complete cost; component medians are not stacked."), Spacer(1, 8),
    p("Observed small-input losses remain in the data. At B8 the same-generator composition/Horner paired gain is only1.0614× and individual ratios fall below1. At B128 the A/A wall ratio has median 1.0118 and observed range0.7740–1.7945, demonstrating shared-host variability. No post-hoc seed selection, fitted asymptotic exponent or significance claim is made."), Spacer(1, 8),
    p("This experiment measures the incidence/pool-generation extension. Earlier dense/trace/pullback root timings belong to the separate Runpod experiment. Structured factor bases are not combined with free mixed labels. Canonical benchmark/leaderboard promotion and signed validation remain pending; raw S3 custody will be completed only when authorized access is restored.")]
SimpleDocTemplate(str(OUT / "RESULTS_REPORT.pdf"), pagesize=(612, 792), leftMargin=36, rightMargin=36, topMargin=34, bottomMargin=35).build(story, onFirstPage=footer, onLaterPages=footer)

md = ["# Local runtime measurement — 17 November 2026", "",
      f"The new batched/Bloom pair path gives a **{ratio('pairs/first-bloom'):.2f}× median paired wall improvement** over the prior pair control at B128. The composition route gives **{ratio('window-horner/compose'):.2f}×** over same-generator Horner. All preceding misses, public setup/tables, actual labels/rejections and recovery are charged. [pcc:unverified]", "",
      "These are provisional local measurements, registered as **PCC-20261117-c1a1e9** with unsigned standing. The user explicitly authorized local execution and temporary local checkpoint/raw custody. No S3 admission or upload is claimed.", "",
      "[Run report PDF](reports/run-20261117T051441Z/RUN_REPORT.pdf) · [Results PDF](reports/run-20261117T051441Z/RESULTS_REPORT.pdf) · [Native report data](results/report-data.json) · [Raw custody and hashes](results/LOCAL_CUSTODY.json)", "",
      "## Complete target costs at B128", "",
      "Wall and CPU entries below are medians over 60 target-round units (12 distinct targets × five measured rounds), all completed. RSS is median peak process KiB. These are complete-call intervals; process startup/output is separately retained at coarse GNU-time resolution. [pcc:unverified]", "",
      "| Arm | Wall (ms) | CPU (ms) | Peak RSS (KiB) |", "|---|---:|---:|---:|"]
for arm in ARMS:
    r = ROWS[128, arm]
    cpu = "unavailable" if r["cpu_seconds"] is None else ms(r["cpu_seconds"]["median"])
    md.append(f"| {LABELS[arm]} | {ms(r['wall_seconds']['median'])} | {cpu} | {r['peak_rss_kib']['median']:.0f} |")
md += ["", f"The median of matched prior-pair/new ratios is {ratio('pairs/first-bloom'):.4f}; it differs from dividing the two marginal medians. The new/rho median paired ratio is {ratio('first-bloom/rho'):.4f}: this is measured progress toward the registered reference, without parity. [pcc:unverified]", "",
       "![All registered arms](reports/run-20261117T051441Z/all-arms-b128.svg)", "",
       "## Ablations and observed spread", "",
       "| Matched B128 comparison | Median wall ratio | n |", "|---|---:|---:|"]
for key, label in ablation_names:
    d = DATA["paired_completed_only"][key]["128"]["wall_ratio"]
    md.append(f"| {label} | {d['median']:.4f} | {d['n']} |")
md += ["", "These are descriptive paired ratios. At B128 the same-generator Horner/composition ratio has median 4.9883; generic full-support enumeration remains 56.5550× cheaper than composition in the paired comparison. Full-support and first-witness tasks are separate. The A/A range is 0.7740–1.7945 (median 1.0118), and small-input losses remain in individual comparisons. No confidence interval or fitted exponent is inferred. [pcc:unverified]", "",
       "![Scaling](reports/run-20261117T051441Z/scaling.svg)", "",
       "## Protocol and checks", "",
       "- Source: `f1549fc821220641406b51a60324ef4ec81ab69b`; parent [protocol](../accelerated/PROTOCOL.json), [local amendment](LOCAL_PROTOCOL.json), [timer amendment](LOCAL_AMENDMENT_2.json), [authorization](AUTHORIZATION.json).",
       "- Host: AMD EPYC 9V74 managed shared CPU, one pinned CPU, four-CPU quota, 16 GiB limit; no L2 claim. Exact qualified binaries reused, with distinct rho source c0176f942 and binary provenance retained.",
       "- Same 60 public natural targets across B8,16,32,64,128, three curves and four targets per size. Subgroup order is approximately 8B³. These are small prime fields; no cryptographic-size ECDLP result is claimed.",
       "- Fresh actual `alpha*P+beta*Q` pools; no validation-secret input and no free structured-base labels. Up to four fixed-seed attempts; stop at first informative relation. All prior misses and rejected draws charged.",
       "- One warmup/five measured rounds, all 11 arms plus registered rho. Full matrix: 8346 rows, 1391 warmups, 6955 measured, 4320 target-round-arm units. No timed-row execution failure. [pcc:unverified]",
       "- Every IC arm completes 275/300 measured units and censors 25 after four misses; rho completes 300/300. B8 has 50/60 IC completions; B16/32/64 have 55/60; B128 has 60/60. Bounded-work medians at smaller B include censored units. [pcc:unverified]",
       "- Native post-check replays 3990 public scalars using the separate frozen pair arithmetic, checks 2904 complete-support agreements, and checks 7260 generation/existence agreements. Binary and dependency pre/post hashes match. Producer checks remain unsigned. [pcc:unverified]", "",
       "![Charged CPU phases](reports/run-20261117T051441Z/phases-b128.svg)", "",
       "Phase bars use native-computed means, which add. For the new Bloom path, public setup/tables account for 1.511 ms mean CPU, pool generation 0.416 ms and incidence/recovery 0.991 ms. These include preceding attempts. Rho phase CPU is unavailable. [pcc:unverified]", "",
       "## Failures, custody and reproducibility", "",
       "Attempt 1 stopped at its first warmup wrapper because GNU time was absent; no candidate was timed. The failed attempt is retained. Attempt 2 uses Debian GNU time with its package checksum matched (version 1.9-0.2) and completed. The first post-check parser rejected an EOF; its failed source/qualifier is retained, and a corrected copied qualifier produced the accepted output. Scientific measurements were not rerun after viewing results.", "",
       "The deterministic raw archive is retained at `" + CUSTODY["archive_path"] + "`, " + str(CUSTODY["archive_bytes"]) + " bytes, SHA256 `" + CUSTODY["archive_sha256"] + "`. It contains both attempts, raw rows/process records, all qualifiers and dependencies, both analysis attempts and local checkpoint. This is explicitly authorized **local-only custody**; no `.artifacts.json` falsely asserts an S3 object.", "",
       "Extract with `tar -xJf trace-local-evidence-20261117.tar.xz`, then verify `FILES.sha256` inside `trace-local-evidence-20261117`. Run the native `postprocess.cpp` against attempt2 and the frozen study snapshot to reproduce the report data. `render_report.py` only lays out already-computed statistics.", "",
       "The original dense/trace/pullback Runpod root experiment and all its graphs remain unchanged. These local incidence results do not inherit its speed ratios. Canonical benchmark index/leaderboard rows remain unchanged without the required independent signed validation and S3 custody. Copies of these descriptive reports/graphs are in `benchmarks/20261117-trace-incidence-local/`; its local record is not an admitted `bench.record/v1`.", "",
       "Sources checked (KB unavailable): frozen parent native protocols and controls, existing prime_fast backend, historical composition implementation, FLINT composition/multipoint contracts, and official Debian Packages metadata for the timer. No new priority claim.", ""]
(HERE / "README.md").write_text("\n".join(md))
for svg in OUT.glob("*.svg"):
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
print(OUT)
