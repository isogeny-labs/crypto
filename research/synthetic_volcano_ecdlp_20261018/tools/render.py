"""Lay out recorded synthetic values. This file performs no research simulation.

The native simulator owns every statistic, comparison and model value.
Python only filters records, formats documents and renders figures.
"""
import csv
import html
import json
import os
from pathlib import Path
import sys

os.environ.setdefault("MPLCONFIGDIR", "/tmp/volcano-render-cache/matplotlib")
os.environ.setdefault("XDG_CACHE_HOME", "/tmp/volcano-render-cache")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak, KeepTogether
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT.parents[1] / "tools" / "volcano-diagram"))
from volcano_diagram import render as volcano_render, _positions, _geometry

RUN = Path(sys.argv[1]).resolve()
OUT = RUN / "reports"
OUT.mkdir(exist_ok=True)
DATA = json.loads((RUN / "dashboard_data.json").read_text())
CONFIG = json.loads((RUN / "config.json").read_text())
VALIDATION = json.loads((RUN / "validation.json").read_text())
FAMILIES = ["prime", "binary", "koblitz_origin"]
NAMES = {"prime": "Prime field", "binary": "Ordinary binary", "koblitz_origin": "Koblitz origin"}
METHODS = ["rho", "ic2_scan", "ic3_pair_table"]
MNAMES = {"rho": "Rho", "ic2_scan": "IC2 scan", "ic3_pair_table": "IC3 pair table"}
SNAMES = {"null": "No injected level effect", "planted_floor_advantage": "Injected floor advantage", "planted_floor_penalty": "Injected floor penalty"}
COLORS = {"rho": "#126b78", "ic2_scan": "#b74735", "ic3_pair_table": "#6551aa"}
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.spines.top": False,
                    "axes.spines.right": False, "axes.labelcolor": "#273648", "text.color": "#15263b",
                    "axes.titleweight": "bold", "figure.facecolor": "#ffffff", "svg.fonttype": "none"})


def pick(rows, **kwargs):
    return [r for r in rows if all(r.get(k) == v for k, v in kwargs.items())]


def one(rows, **kwargs):
    result = pick(rows, **kwargs)
    if len(result) != 1:
        raise ValueError((kwargs, len(result)))
    return result[0]


def number(x):
    return f"{x:.3g}"


def save(fig, name):
    fig.savefig(OUT / f"{name}.svg", bbox_inches="tight")
    fig.savefig(OUT / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def scaling():
    fig, axes = plt.subplots(1, 3, figsize=(14.2, 4.5), sharey=True)
    for ax, family in zip(axes, FAMILIES):
        for method in METHODS:
            rows = [one(DATA["strategies"], family=family, bits=b, scenario="null", method=method,
                        strategy="direct_surface", weight=.1, mode="cold") for b in [30, 40, 50]]
            ax.plot([30, 40, 50], [r["stats"]["mean"] for r in rows], "o-", color=COLORS[method], label=MNAMES[method], lw=2)
            ax.fill_between([30, 40, 50], [r["stats"]["min"] for r in rows], [r["stats"]["max"] for r in rows], color=COLORS[method], alpha=.12)
        ax.set_yscale("log")
        ax.set_xticks([30, 40, 50])
        ax.set_xlabel("Nominal log₂(field size)")
        ax.set_title(NAMES[family])
        ax.grid(axis="y", alpha=.18)
    axes[0].set_ylabel("Synthetic work per target · cold · weight 0.1")
    axes[-1].legend(loc="lower right", frameon=False)
    fig.suptitle("SYNTHETIC · direct surface cost across size", fontsize=16, x=.08, ha="left")
    fig.text(.08, -.035, "Seed 20261018 · 10 invented targets per cell · bands = min–max, lines = means · no measured timings", fontsize=10)
    fig.tight_layout(rect=(0, 0, 1, .95))
    save(fig, "scaling")


def levels():
    fig, axes = plt.subplots(1, 3, figsize=(14.2, 4.6), sharey=True)
    palette = {"null": "#263a51", "planted_floor_advantage": "#1d806b", "planted_floor_penalty": "#b74735"}
    for ax, family in zip(axes, FAMILIES):
        for scenario, color in palette.items():
            rows = [one(DATA["levels"], family=family, bits=50, scenario=scenario, method="ic3_pair_table", level=l, weight=.1, mode="cold") for l in range(CONFIG["depth"] + 1)]
            ax.plot([r["level"] for r in rows], [r["stats"]["mean"] for r in rows], "o-", color=color, label=SNAMES[scenario], lw=2)
            ax.fill_between([r["level"] for r in rows], [r["stats"]["min"] for r in rows], [r["stats"]["max"] for r in rows], color=color, alpha=.11)
        ax.set_yscale("log")
        ax.set_xticks(range(CONFIG["depth"] + 1))
        ax.set_xlabel("Stratum (0 = surface, 4 = floor)")
        ax.set_title(NAMES[family])
        ax.grid(axis="y", alpha=.18)
    axes[0].set_ylabel("Synthetic IC3 work · mean over vertices · cold")
    axes[-1].legend(frameon=False, fontsize=8)
    fig.suptitle("SYNTHETIC · distinguish a null model from a planted trend", fontsize=16, x=.08, ha="left")
    fig.text(.08, -.04, "50-bit labels · weight 0.1 · per-level vertex counts 1 / 4 / 12 / 36 / 108 · bands summarize 10 target-level means", fontsize=9.5)
    fig.tight_layout(rect=(0, 0, 1, .95))
    save(fig, "levels")


def strategies():
    fig, axes = plt.subplots(1, 3, figsize=(14.2, 4.8), sharey=True)
    names = ["direct_surface", "predeclared_floor", "profiled_one_per_level", "profiled_all_vertices"]
    short = ["Surface", "Floor path", "Profile 5", "Profile 161"]
    for ax, family in zip(axes, FAMILIES):
        for i, (mode, label, color) in enumerate([("cold", "Cold", "#6551aa"), ("batch10_per_target", "Batch of 10 / target", "#2a8d90")]):
            rows = [one(DATA["strategies"], family=family, bits=50, scenario="planted_floor_advantage", method="ic3_pair_table", strategy=s, weight=.1, mode=mode) for s in names]
            ax.bar(np.arange(4) + (i - .5) * .36, [r["ratio"] for r in rows], width=.34, label=label, color=color)
        ax.axhline(1, ls="--", color="#b74735", lw=1)
        ax.set_yscale("log")
        ax.set_xticks(range(4), short, rotation=25)
        ax.set_title(NAMES[family])
        ax.grid(axis="y", alpha=.17)
    axes[0].set_ylabel("Synthetic IC3 cost / surface rho cost · lower is cheaper")
    axes[-1].legend(frameon=False, fontsize=9)
    fig.suptitle("SYNTHETIC · charge for finding a destination", fontsize=16, x=.08, ha="left")
    fig.text(.08, -.05, "50-bit labels · injected floor advantage · 3 separate pilot targets per candidate · all pilot and graph costs included", fontsize=9.5)
    fig.tight_layout(rect=(0, 0, 1, .95))
    save(fig, "strategies")


def heatmap():
    family, bits, scenario = "prime", 50, "planted_floor_advantage"
    vertices = []
    for v in DATA["graph_vertices"]:
        vals = {}
        for method in ["rho", "ic3_pair_table"]:
            row = one(DATA["curves"], family=family, bits=bits, scenario=scenario, vertex=v["id"], method=method)
            vals[method] = row["cold"]
        neighbours = [e["b"] if e["a"] == v["id"] else e["a"] for e in DATA["graph_edges"] if not e["horizontal"] and v["id"] in [e["a"], e["b"]]]
        vertices.append({"id": v["id"], "label": f"SIM-v{v['id']}", "level": v["level"], "nbrs": neighbours,
                         "values": vals, **({"highlight": "symbolic origin"} if v["id"] == 0 else {})})
    spec = {"schema": "volcano-diagram/v3", "synthetic": True, "ell": 3, "depth": CONFIG["depth"],
            "title": "SYNTHETIC · 161 abstract vertices · 10 invented targets per vertex",
            "subtitle": "Prime-field label 50 · injected floor advantage · seed 20261018 · no EC constructed",
            "footnote": "Degree-3 skeleton shown; all edges unverified. Degree-11/23/29 overlays are in the explorer and edges.csv. Units: invented work, weight 0.1.",
            "metrics": [{"key": "rho", "title": "Rho · mean synthetic work", "label": "cold work / target"},
                        {"key": "ic3_pair_table", "title": "IC3 · mean synthetic work", "label": "cold work / target"}],
            "vertices": vertices,
            "edges": [{"a": e["a"], "b": e["b"], "status": "unverified"} for e in DATA["graph_edges"] if not e["horizontal"]]}
    (OUT / "volcano.diagram.json").write_text(json.dumps(spec, indent=2))
    volcano_render(spec, OUT / "volcano")
    # Reuse the standard renderer's layout for the interactive diagram.
    positions, _ = _positions({v["id"]: v for v in vertices})
    rx, ry, yc = _geometry(CONFIG["depth"])
    import math
    for v in DATA["graph_vertices"]:
        angle = 2 * math.pi * positions[v["id"]]
        v["plot_x"] = rx[v["level"]] * math.cos(angle)
        v["plot_y"] = yc[v["level"]] + ry[v["level"]] * math.sin(angle)


def read_csv(name):
    with (RUN / name).open() as f:
        return list(csv.DictReader(f))


def build_pdf():
    font = Path("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
    if font.exists():
        pdfmetrics.registerFont(TTFont("Report", str(font)))
        pdfmetrics.registerFont(TTFont("ReportBold", str(font.with_name("DejaVuSans-Bold.ttf"))))
        regular, bold = "Report", "ReportBold"
    else:
        regular, bold = "Helvetica", "Helvetica-Bold"
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle("BodyR", fontName=regular, fontSize=9, leading=14, spaceAfter=9, textColor=colors.HexColor("#23374c")))
    styles.add(ParagraphStyle("TitleR", fontName=bold, fontSize=24, leading=29, spaceAfter=13, textColor=colors.HexColor("#14283f")))
    styles.add(ParagraphStyle("HeadR", fontName=bold, fontSize=14, leading=19, spaceAfter=10, textColor=colors.HexColor("#14283f")))
    styles.add(ParagraphStyle("SmallR", fontName=regular, fontSize=7.6, leading=11, spaceAfter=7))
    styles.add(ParagraphStyle("CellR", fontName=regular, fontSize=7.1, leading=10))
    styles.add(ParagraphStyle("BannerR", fontName=bold, fontSize=10, leading=15, textColor=colors.HexColor("#93421a"), spaceAfter=14))
    story = []
    width = A4[0] - 88

    def p(s, style="BodyR"):
        story.append(Paragraph(s, styles[style]))

    def heading(s):
        p(s, "HeadR")

    def table(rows, widths=None, padding=5):
        converted = [[Paragraph(html.escape(str(cell)), styles["CellR"]) for cell in row] for row in rows]
        t = Table(converted, colWidths=widths or [width / len(rows[0])] * len(rows[0]), repeatRows=1, hAlign="LEFT")
        t.setStyle(TableStyle([("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e8edf3")),
                               ("VALIGN", (0, 0), (-1, -1), "TOP"), ("BOTTOMPADDING", (0, 0), (-1, -1), padding),
                               ("TOPPADDING", (0, 0), (-1, -1), padding), ("LINEBELOW", (0, 0), (-1, 0), .7, colors.HexColor("#b9c7d5")),
                               ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#f6f8fb")])]))
        story.append(t)
        story.append(Spacer(1, 10))

    def picture(name, max_height=340):
        img = Image(str(OUT / f"{name}.png"))
        scale = min(width / img.imageWidth, max_height / img.imageHeight)
        img.drawWidth = img.imageWidth * scale
        img.drawHeight = img.imageHeight * scale
        story.append(img)
        story.append(Spacer(1, 12))

    def page():
        story.append(PageBreak())

    p("FULLY SYNTHETIC · INVENTED DATA · NO ECDLP SOLVES", "BannerR")
    p("An isogeny-volcano<br/>comparison experiment", "TitleR")
    p("Thirty, forty and fifty-bit field-size labels. Prime-field, ordinary binary and Koblitz-origin models. Ten symbolic targets at every vertex.")
    p("This is an executable simulation for planning a later experiment. Every cost, success probability, relation count, curve variation and comparison in the result tables is generated from declared assumptions. No curve coefficients, field representation, point coordinates, isogeny kernels, relation matrices or recovered logarithms were constructed.")
    heading("The requested scope")
    table([["Condition", "Implementation"],
           ["Sizes", "log₂(q) labels 30, 40, 50; nominal prime r just below 2^(b−2), matched across families"],
           ["Deep graph", "Five strata, depth 4; vertex counts 1 / 4 / 12 / 36 / 108; 161 abstract vertices per condition"],
           ["Horizontal connections", "Three illustrative degree labels: 11, 23, 29. 483 overlay multiedges plus 160 vertical edges"],
           ["Targets", "10 holdout target labels per vertex and method; 3 separate pilot targets for destination selection"],
           ["Methods", "Rho; two-summand index calculus with a scan; three-summand index calculus with a pair table"],
           ["Controls", "Null; oracle scan cost ×0.55 per level; oracle scan cost ×1.7 per level; native weights 0 / 0.1 / 1"],
           ["Accounting", "Cold single-target cost; reusable precomputation shared across exactly 10 targets"]], [100, width - 100])
    p("Graph coverage means the complete specified abstract component, repeated for each family and size. It does not mean all elliptic curves or all isogeny degrees over any field.", "SmallR")
    heading("What the simulation illustrates")
    p("The generated results can show how subgroup size, an imposed oracle trend, Frobenius discounts, memory and destination discovery costs interact. A size trend here reflects the equations in config.json. It provides no empirical evidence that real curves have the same trend.")
    page()

    heading("Ten-target summaries at the symbolic surface")
    p("Null scenario; cold accounting; native-operation weight 0.1. All entries below are invented synthetic work units. The ratio is mean method cost divided by mean matched surface-rho cost; a larger value is more expensive.")
    rows = [["Family", "Bits", "Method", "Min", "Mean", "Max", "Ratio / rho"]]
    for family in FAMILIES:
        for bits in [30, 40, 50]:
            for method in METHODS:
                r = one(DATA["strategies"], family=family, bits=bits, scenario="null", method=method, strategy="direct_surface", weight=.1, mode="cold")
                s = r["stats"]
                rows.append([NAMES[family], bits, MNAMES[method], number(s["min"]), number(s["mean"]), number(s["max"]), number(r["ratio"])])
    table(rows, [80, 28, 90, 77, 77, 77, width - 429], padding=3)
    p("Full precision, medians and standard deviations are retained in strategy_summary.csv. curve_summary.csv has min/mean/max for each of the 161 vertices in every condition. level_summary.csv first averages vertices within each target, then summarizes the ten target-level means; this is a different estimand from per-curve target variability.", "SmallR")
    p("GAE here means an invented group-operation count plus weight × an invented native-operation count. The weights are an accounting sensitivity band, not a hardware calibration. Prime versus binary timings are not inferred from these uncalibrated counts.", "SmallR")
    page()

    heading("The deep graph and horizontal overlays")
    picture("volcano", 390)
    p("The heat map follows the repository’s standard volcano layout. All displayed links are dashed because none is a verified isogeny. The interactive explorer contains the full labeled multigraph, including same-stratum overlay edges and surface self-loops.")
    table([["Overlay labels", "Multiedges", "Mean path length", "Diameter"]] +
          [[r["overlay_degrees"], r["undirected_multiedges"], number(float(r["mean_shortest_path"])), r["diameter"]] for r in read_csv("topology_ablation.csv")])
    p("The degree-3 skeleton has an inert surface and no horizontal degree-3 edges. Other degree labels supply the horizontal connections at every stratum. The cyclic shifts 1, 5 and 7 are illustrative permutations, not calculated actions of actual ideals. Shorter graph paths alone do not change either solver model; the topology-only ablation leaves solver costs fixed by assumption.", "SmallR")
    p("A fixed-degree ordinary volcano has horizontal edges only at its surface. The mixed-degree overlay is the explicit correction to the requested picture of horizontal isogenies at all depths.", "SmallR")
    page()

    heading("How size changes the modeled comparison")
    picture("scaling", 270)
    heading("The assumptions determine the scaling")
    p("With r the nominal subgroup order, the signed factor-base size is B = 2 ceil(r^(1/3)/2). For m summands, p = 1 − exp(−B^m/(m! r)) is the assumed decomposition probability. Relations needed are ceil(1.08 U) + 8, where U = ceil(B/(2s)) and s is the assumed Frobenius orbit discount.")
    p("At fixed ordinary-curve parameters, the scan model makes IC2 collection proportional to r in its leading term. The pair-table model balances a B² setup against collection near r^(2/3). Rho uses a square-root collision surrogate. These are imposed model behaviors, not fitted experimental exponents or a comparison with the fastest known implementation.")
    heading("Memory is a separate model output")
    mem = [["Size label", "IC3 pair-table workspace", "Profiled search workspace"]]
    for bits in [30, 40, 50]:
        a = one(DATA["strategies"], family="prime", bits=bits, scenario="null", method="ic3_pair_table", strategy="direct_surface", weight=.1, mode="cold")
        b = one(DATA["strategies"], family="prime", bits=bits, scenario="null", method="ic3_pair_table", strategy="profiled_all_vertices", weight=.1, mode="cold")
        mem.append([bits, f"{a['memory']:,.0f} bytes", f"{b['memory']:,.0f} bytes"])
    table(mem)
    p("Memory is a synthetic capacity estimate; these tables were not allocated. The model charges 24 bytes for every pair-table entry. Profiling keeps the incumbent and current candidate workspaces, plus graph storage. A bounded-memory solver would need a different model.", "SmallR")
    page()

    heading("Does moving to the floor help?")
    picture("levels", 280)
    p("The null has no injected ordinary-curve level advantage. Its vertex-to-vertex variation is generated from a fixed lognormal multiplier. The green and red trends are deliberate positive and negative controls: they multiply the oracle scan cost by 0.55^level or 1.7^level. Pair-table setup and linear algebra do not receive that multiplier.")
    heading("A Koblitz origin is not a Koblitz curve at every level")
    p("Only the symbolic Koblitz surface receives a Frobenius orbit discount equal to the extension degree. Its descendants use the ordinary-binary model with discount one. A descent can lose the degree-2 Frobenius endomorphism that justified the Koblitz optimization. The graph is a hypothetical depth-4 surrogate: no claim is made that Koblitz curves at exactly extension degrees 30, 40 and 50 realize this graph or its nominal subgroup.")
    p("All three requested binary extension degrees are composite. Results from a future real run at these degrees would need separate analysis before any transfer to prime-degree binary extensions.")
    heading("Avoid a graph-selection artifact")
    p("There are 108 floor candidates and only one surface candidate. Under an exchangeable null, 108 of 161 possible minima belong to the floor purely because of candidate count. The package therefore records both profiling all vertices and profiling one prespecified vertex per stratum. Observing a low minimum is not evidence of an intrinsic level effect.")
    p("Only ten symbolic targets are requested per cell. The displayed min–max ranges and pattern frequencies are descriptive. They are not confidence intervals for real curves, and the package makes no significance or universal exponent claim.", "SmallR")
    page()

    heading("Count discovery, transport and shared setup")
    picture("strategies", 280)
    p("Each bar includes the full modeled pipeline for its accounting mode. Profiling all candidates charges graph construction, all candidate precomputations and all three pilot descents per candidate. The selected candidate is tested on ten separate target labels. Candidate selection never reads those ten test labels.")
    rows = [["Prime-field label 50; injected advantage", "Cold mean", "Batch / target", "Cold / rho"]]
    for s, label in [("direct_surface", "Direct surface"), ("predeclared_floor", "Prespecified floor path"), ("profiled_one_per_level", "Profile one per level"), ("profiled_all_vertices", "Profile all 161")]:
        cold = one(DATA["strategies"], family="prime", bits=50, scenario="planted_floor_advantage", method="ic3_pair_table", strategy=s, weight=.1, mode="cold")
        batch = one(DATA["strategies"], family="prime", bits=50, scenario="planted_floor_advantage", method="ic3_pair_table", strategy=s, weight=.1, mode="batch10_per_target")
        rows.append([label, number(cold["stats"]["mean"]), number(batch["stats"]["mean"]), number(cold["ratio"])])
    table(rows, [width - 207, 69, 69, 69])
    p("The synthetic floor oracle advantage can improve the modeled IC pipeline without making it cheaper than modeled rho. Sharing setup across ten targets changes the accounting; it is reported as batch cost per target and never substituted for a cold solve.")
    p("The two-summand and three-summand models use reusable factor-base-log preprocessing with independent target descent. This is a planning model, not a claim that every existing repository IC implementation exposes this reusable interface.", "SmallR")
    page()

    heading("Reproduce and inspect")
    p("Build and run from the package directory:")
    for line in ["bash run.sh run-new", "build/simulate --self-test", "build/audit run-new", "build/simulate --seed 20261018 --out run-replay", "diff -qr run-new run-replay"]:
        p(html.escape(line), "SmallR")
    p("The final comparison command applies before reports and audit metadata are added, or should compare the core data files only. Output directories are write-once: the simulator refuses to overwrite an existing run. Fixed-seed replay is byte-identical on the recorded compiler and standard library; C++ distribution algorithms may differ across toolchains.")
    heading("Validation performed")
    table([["Check", "Status"],
           ["Structural and accounting assertions", str(VALIDATION["graph_and_accounting_assertions"]) + " passed"],
           ["Raw observations", f"{VALIDATION['raw_trial_rows']:,} synthetic rows"],
           ["Independent CSV audit", "Checks all raw-to-summary extrema, means, medians and standard deviations; target coverage; pilot/test separation flags"],
           ["Replay", "All core CSV/JSON files compared byte-for-byte on the recorded runtime"],
           ["Cryptographic verification", "Not applicable: no constructed curves, points, maps, relations or recovered logs"]], [135, width - 135])
    p("trials.csv.gz is the full stage-level observation table. curve_summary.csv.gz retains all per-vertex ten-target summaries. strategy_trials.csv.gz and strategy_summary.csv retain the charged comparisons. level_summary.csv, patterns.csv and pilot_selection.csv retain every control and accounting band. config.json is the frozen model contract.")
    heading("Requirement status")
    table([["Requested item", "Status within the synthetic scope"],
           ["30 / 40 / 50 and three families", "Complete; all nine label combinations retained"],
           ["Ten points; min / max / average", "Complete as symbolic targets; no actual points or DLP answers"],
           ["Deep strata and horizontal links", "Complete abstract graph; mixed degrees; actual EC realization not attempted"],
           ["Entire graph comparison, IC focus", "Complete for all 161 vertices and three selected solver cost models; all-degree or all-algorithm comparison not claimed"],
           ["Size-dependent patterns", "Complete as outputs of the chosen model; empirical mathematical patterns not established"]], [135, width - 135])
    page()

    heading("Repository context and limits")
    p("The named repositories supplied the experiment architecture and reporting conventions. Their measured results were not inserted into the synthetic tables, and their cryptographic solvers were not executed. The standard volcano drawing tool is the only retained repository implementation used in producing this package; it is adapted to label abstract vertices and simulated color ranges.")
    links = [
        ("crypto: prime-field IC module", "https://github.com/aburan28/crypto/blob/main/src/cryptanalysis/ec_index_calculus.rs"),
        ("crypto: Koblitz IC module", "https://github.com/aburan28/crypto/blob/main/src/cryptanalysis/koblitz_index_calculus.rs"),
        ("cryptanalysis: volcano interface", "https://github.com/aburan28/cryptanalysis/blob/claude/cryptanalysis-repo-setup-p59kek/include/cryptanalysis/ca_volcano.h"),
        ("crypto: isogeny/PDP note", "https://github.com/aburan28/crypto/blob/main/RESEARCH_PDP_SPEEDUP_AND_ISOGENY.md"),
        ("crypto: standard renderer", "https://github.com/aburan28/crypto/blob/main/tools/volcano-diagram/volcano_diagram.py"),
        ("Sutherland: Isogeny volcanoes", "https://arxiv.org/abs/1208.5370"),
        ("Galbraith et al.: subfield index calculus", "https://eprint.iacr.org/2020/1315"),
    ]
    for label, url in links:
        p(f'<a href="{url}" color="#126b78">{html.escape(label)}</a>', "SmallR")
    p("reference/sources.json pins the inspected file blob hashes. The repository links above may move; the hashes identify the inspected versions.", "SmallR")
    heading("What a later real experiment must replace")
    p("Construct ordinary curves with certified field definitions, orders, subgroup generators and conductor data. Verify every selected isogeny and subgroup transport. Run matched rho and IC implementations on independent targets, recording relation yields, actual rank, oracle work, memory and all setup/transport/verification phases. Replace the assumed cost weights with measured primitive calibrations. Preserve failures and budget limits.")
    p("The model intentionally provides no invented coefficients, fabricated isogeny certificates, fake solver correctness flags or synthetic seconds that could be mistaken for runtime measurements. A lower cost in this report demonstrates only the effect of a chosen input assumption.")

    def frame(canvas, doc):
        canvas.saveState()
        canvas.setFillColor(colors.HexColor("#a34e25"))
        canvas.setFont(bold, 7.5)
        canvas.drawString(44, A4[1] - 29, "SYNTHETIC / INVENTED DATA — NOT A CRYPTOGRAPHIC BENCHMARK")
        canvas.setFillColor(colors.HexColor("#68798a"))
        canvas.setFont(regular, 7)
        canvas.drawString(44, 27, "volcano-synthetic-20261018 · seed 20261018")
        canvas.drawRightString(A4[0] - 44, 27, str(doc.page))
        canvas.restoreState()

    doc = SimpleDocTemplate(str(OUT / "experiment_report.pdf"), pagesize=A4, rightMargin=44, leftMargin=44,
                            topMargin=51, bottomMargin=46, title="Synthetic isogeny-volcano / ECDLP experiment", author="Synthetic experiment package")
    doc.build(story, onFirstPage=frame, onLaterPages=frame)


scaling()
levels()
strategies()
heatmap()
build_pdf()
template = (ROOT / "tools" / "explorer_template.html").read_text()
payload = json.dumps(DATA, separators=(",", ":")).replace("</", "<\\/")
(OUT / "explorer.html").write_text(template.replace("__EXPERIMENT_DATA__", payload))
print("Rendered synthetic report, figures and self-contained explorer:", OUT)
