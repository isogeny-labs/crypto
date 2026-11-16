"""Layout only: plot already-recorded native aggregates; never run research math."""
from pathlib import Path
import html
import json
import typst

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(__file__).resolve().parent
data = json.loads((ROOT / "results-summary.json").read_text())
toy = next(c for c in data["cells"] if c["cell"]["p"] == 17)
colors = ["#126b66", "#a35b24", "#4162a0", "#8a6ba8"]
labels = ["Graph secant (4 signs)", "Control secant (4 signs)",
          "Graph dense", "Control dense"]
arms = ["candidate-secant", "control-secant", "candidate-dense", "control-dense"]
rows = [next(r for r in toy["v1_recorded_status"] if r["arm"] == arm)
        for arm in arms]

svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="710" viewBox="0 0 1000 710">',
       '<rect width="1000" height="710" fill="#fff"/>',
       '<style>text{font-family:DejaVu Sans,sans-serif;fill:#172b38}.h{font-size:19px;font-weight:700}.s{font-size:14px}.n{font-size:16px;font-weight:600}.muted{fill:#536571}</style>']

def text(x, y, value, cls="s", **attrs):
    extra = " ".join(f'{k.replace("_", "-")}="{html.escape(str(v))}"'
                     for k, v in attrs.items())
    svg.append(f'<text x="{x}" y="{y}" class="{cls}" {extra}>{html.escape(str(value))}</text>')

def line(x1, y1, x2, y2, color="#cad5db"):
    svg.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color}"/>')

text(32, 35, "Split-graph bases: degree and all-branch work", "h")
text(32, 60, "run-20261031T201044Z · stock neogb F4 · exact counts; clocks are diagnostic", "s muted")
text(32, 101, "A   Toy handled degree", "h")
text(518, 101, "B   Toy reduced rows (all branches)", "h")
for i, row in enumerate(rows):
    y = 132 + 43 * i
    degree = next(k for k in row["handled_degree_histogram"] if k != "unknown")
    text(32, y + 17, labels[i])
    svg.append(f'<rect x="222" y="{y}" width="{int(degree)*31}" height="24" rx="3" fill="{colors[i]}"/>')
    text(230 + int(degree)*31, y + 18, degree, "n")
    text(518, y + 17, labels[i])
    value = row["rows_sum_all_branches"]
    svg.append(f'<rect x="708" y="{y}" width="{value*0.076}" height="24" rx="3" fill="{colors[i]}"/>')
    text(716 + value*0.076, y + 18, f"{value:,}", "n")
text(32, 322, "18 targets/arm; degree=max(input, selected-pair LCM).", "s muted")
text(518, 322, "All four sign branches charged; dense has one solve.", "s muted")
line(32, 343, 966, 343)
text(32, 379, "C   Larger degree comparison (F83)", "h")
text(518, 379, "D   Graph-construction attempts", "h")
for degree in (0, 4, 8, 12):
    yy = 588 - degree * 13
    line(86, yy, 426, yy)
    text(60, yy + 5, degree)
for i, d in enumerate((3, 4, 5)):
    text(132 + i*124, 612, f"d={d}")
for arm, control, color, label in [
    ("candidate-secant", "candidate", colors[0], "Graph secant"),
    ("control-secant", "low-x", colors[1], "Low-x secant"),
    ("candidate-dense", "candidate", colors[2], "Graph dense")]:
    points = []
    for i, d in enumerate((3, 4, 5)):
        cell = next(c for c in data["cells"] if c["cell"]["p"] == 83 and c["cell"]["d"] == d)
        row = next(r for r in cell["v1_recorded_status"] if r["arm"] == arm and r["control"] == control)
        degree = next(k for k in row["handled_degree_histogram"] if k != "unknown")
        xx, yy = 148+i*124, 588-int(degree)*13
        points.append(f"{xx},{yy}")
        svg.append(f'<circle cx="{xx}" cy="{yy}" r="5" fill="{color}"/>')
        text(xx+9, yy-5, degree)
    svg.append(f'<polyline points="{" ".join(points)}" stroke="{color}" fill="none" stroke-width="2"/>')
    offset = ["Graph secant", "Low-x secant", "Graph dense"].index(label)
    svg.append(f'<rect x="{55+offset*143}" y="638" width="12" height="12" fill="{color}"/>')
    text(73+offset*143, 649, label)
text(532, 423, "Field", "n")
for i, d in enumerate((3, 4, 5)):
    text(664+i*90, 423, f"d={d}", "n")
for j, p in enumerate((83, 101, 127)):
    y = 467+j*49
    text(532, y, f"F{p}", "n")
    for i, d in enumerate((3, 4, 5)):
        cell = next(c for c in data["cells"] if c["cell"]["p"] == p and c["cell"]["d"] == d)
        text(676+i*90, y, cell["construction"]["attempts"], "n")
text(518, 614, "9/9 prospective cells accepted within frozen caps.", "s muted")
text(518, 641, "Lower degree did not lower the all-branch row proxy.", "s muted")
line(32, 669, 966, 669)
text(32, 696, "14,194/14,220 original extractions verified; 26 caps retained. Separate exact saved-basis replay: 14,220/14,220.", "s muted")
svg.append("</svg>")
(OUT / "comparison.svg").write_text("\n".join(svg) + "\n")
typst.compile(str(OUT / "figure.typ"), output=str(OUT / "comparison.pdf"), root=str(ROOT))
typst.compile(str(OUT / "report.typ"), output=str(OUT / "report.pdf"), root=str(ROOT))

# Presentation of existing native aggregate fields only; no new metrics.
table = []
for cell in data["cells"]:
    for stage in ("v1_recorded_status", "saved_basis_v2_verified"):
        for row in cell[stage]:
            degree = ", ".join(f"{k}: {v} targets" for k, v in row["handled_degree_histogram"].items())
            fields = [cell["curve_slug"], cell["cell"]["d"],
                      f'{row["arm"]} {row["control"] or ""}', degree,
                      row["rows_sum_all_branches"], row["peak_columns"],
                      f'{row["sat_targets"]}/{row["target_count"]}',
                      row["unknown_or_failed_cases"]]
            content = "".join(f'<td>{html.escape(str(v)) if v is not None else "unknown"}</td>' for v in fields)
            table.append(f'<tr data-p="{cell["cell"]["p"]}" data-stage="{stage}">{content}</tr>')
dashboard = '''<!doctype html><html lang="en"><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Split-graph F4 comparison</title><style>
body{font:16px/1.5 system-ui,sans-serif;color:#172b38;max-width:1150px;margin:32px auto;padding:0 20px}
h1{line-height:1.15}a{color:#126b66}img{width:100%;height:auto}label{display:inline-block;margin:12px 24px 12px 0}
select{padding:7px;font:inherit}table{border-collapse:collapse;width:100%;font-size:13px}td,th{border-bottom:1px solid #ccd7dd;padding:8px;text-align:left}th{background:#eaf2f3}td:first-child{max-width:260px;overflow-wrap:anywhere}.scroll{overflow:auto}
.status{border-left:4px solid #126b66;padding:10px 18px;background:#eef6f5}[hidden]{display:none!important}
</style><main><h1>Split-graph factor bases: measured F4 degree</h1>
<p>Run <code>run-20261031T201044Z</code> · two summands · prime fields · stock neogb F4.</p>
<p class="status">Complete toy audit: zero mismatches. Original extraction: <strong>14,194/14,220 verified; 26 capped</strong>.
Separate offline saved-basis verification: <strong>14,220/14,220</strong>, with no CAS reruns.
The original extraction caps remain in the record.</p>
<p>The graph secant encoding lowers observed handled degree. Charging all four signs increases reduced rows versus its own dense encoding. Complete field cost per usable relation and timing speedup are unknown.</p>
<nav><a href="RESULTS.md">Result report</a> · <a href="reports/report.pdf">Illustrated PDF</a> ·
<a href="results-summary.json">Native aggregate JSON</a> · <a href="protocol.json">Frozen protocol</a> ·
<a href="REPRODUCE.md">Reproduce</a> · <a href="review/DATA.md">Independent review</a></nav>
<img src="reports/comparison.svg" alt="Toy degrees 3,4,6,6; branch-summed rows2102,2522,736,706; larger degree comparison and construction attempts">
<h2>Complete populations and all-branch counters</h2>
<p>Degree entries give target counts at each degree. Handled degree means max(input-generator degree, maximum selected-pair LCM degree). It differs from homogeneous delta and sugar degree. SAT coverage for an incomplete original group is also incomplete; the separate saved-basis proof is explicitly selectable below.</p>
<label>Verification record <select id="stage"><option value="v1_recorded_status">Original extraction (26 caps retained)</option><option value="saved_basis_v2_verified">Separate exact saved-basis proof</option></select></label>
<label>Field <select id="field"><option value="all">All fields</option><option>17</option><option>83</option><option>101</option><option>127</option></select></label>
<div class="scroll"><table><thead><tr><th>Curve</th><th>Graph d</th><th>Arm / control</th><th>Handled degree</th><th>Reduced rows, all branches</th><th>Peak columns</th><th>SAT targets</th><th>Unknown cases</th></tr></thead><tbody>'''
dashboard += "\n".join(table)
dashboard += '''</tbody></table></div><p>Exact finite populations; no sampling interval or timing inference.
The original protocol and frozen analysis retain their censored cases. The post-data verifier establishes finite-field root-set equality, not operational uncapped relation discovery, ideal multiplicities or a complete cost claim. No priced-pipeline leaderboard row is promoted.</p></main>
<script>const stage=document.getElementById('stage'),field=document.getElementById('field');
function filter(){for(const r of document.querySelectorAll('tbody tr'))r.hidden=r.dataset.stage!==stage.value||(field.value!=='all'&&r.dataset.p!==field.value)}
stage.addEventListener('change',filter);field.addEventListener('change',filter);filter();</script></html>'''
(ROOT / "comparison.html").write_text(dashboard + "\n")
