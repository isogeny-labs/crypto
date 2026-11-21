#!/usr/bin/env python3
"""Write figures/sweep_outcomes.svg from phase2_index.json and leads.json
(pure Python). Computes only counts. PNG/PDF are rendered by render.js."""
import json
import pathlib
from collections import Counter, defaultdict
from xml.sax.saxutils import escape

HERE = pathlib.Path(__file__).resolve().parent
(HERE / "figures").mkdir(exist_ok=True)
leads = {l["id"]: l for l in json.loads((HERE / "leads.json").read_text())["leads"]}
res = json.loads((HERE / "phase2_index.json").read_text())["leads"]

FAM = ["exotic", "ic-prime", "generic", "transfer", "ic-binary", "ic-engine", "structure"]
VERD = ["consistent_with_prediction", "falsified", "inconclusive", "not_run"]
VCOL = {"consistent_with_prediction": "#4c72b0", "falsified": "#c44e52", "inconclusive": "#dd8452", "not_run": "#9a9a9a"}
BEAR = ["closes_or_confirms_obstruction", "constant_factor_only", "none", "possible_signal_needs_replication"]
BCOL = {"closes_or_confirms_obstruction": "#55a868", "constant_factor_only": "#8172b2", "none": "#bdbdbd", "possible_signal_needs_replication": "#c44e52"}

byfam = defaultdict(Counter); bear = defaultdict(Counter); ctl = Counter()
for r in res:
    f = leads[r["lead"]]["family"]
    byfam[f][r["verdict"]] += 1
    bear[f][r["bearing_on_sqrt_p"]] += 1
    ctl[(r["verdict"], r["controls_ok"])] += 1
tot_v = Counter(); tot_b = Counter()
for f in FAM:
    tot_v.update(byfam[f]); tot_b.update(bear[f])

W, H = 1500, 600
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica, Arial, sans-serif" font-size="12">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       '<text x="20" y="26" font-size="15" font-weight="bold">Lead sweep 2026-11-10: 173 frozen protocols executed once at toy scale, one Sonnet agent each (all results [pcc:unverified])</text>']
def text(x, y, s, size=12, color="#000", anchor="start", weight="normal", style="normal"):
    out.append(f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}" text-anchor="{anchor}" font-weight="{weight}" font-style="{style}">{escape(str(s))}</text>')

def bars(x0, y0, title, data, keys, cols, note):
    text(x0, y0, title, 14, weight="bold"); text(x0, y0 + 18, note, 11, style="italic")
    maxn = max(sum(data[f].values()) for f in FAM)
    scale = 420 / maxn
    for i, f in enumerate(FAM):
        y = y0 + 45 + i * 48
        text(x0, y + 20, f"{f} ({sum(data[f].values())})", 11.5)
        x = x0 + 120
        for k in keys:
            n = data[f][k]
            if not n: continue
            w = n * scale
            out.append(f'<rect x="{x:.1f}" y="{y}" width="{w:.1f}" height="30" fill="{cols[k]}"/>')
            if w > 16: text(x + w / 2, y + 20, n, 11, "white", anchor="middle", weight="bold")
            x += w
    ly = y0 + 45 + len(FAM) * 48 + 8
    x = x0
    for k in keys:
        out.append(f'<rect x="{x}" y="{ly}" width="12" height="12" fill="{cols[k]}"/>')
        label = f"{k} ({sum(data[f][k] for f in FAM)})"
        text(x + 16, ly + 11, label, 10.5)
        x += 6.2 * len(label) + 28

bars(20, 60, "A. Verdict against the preregistered prediction, by family", byfam, VERD, VCOL,
     f"run.py verdict field; {tot_v['falsified']} falsified of which {ctl[('falsified', True)]} with all controls passing")
bars(780, 60, "B. Agent reading: bearing on the sub-sqrt(p) question", bear, BEAR, BCOL,
     "the two 'possible signal' flags are read as toy artefacts in SWEEP_REPORT.md section 3")
out.append("</svg>")
(HERE / "figures" / "sweep_outcomes.svg").write_text("\n".join(out))
print("verdicts", dict(tot_v)); print("bearing", dict(tot_b))
for f in FAM: print(f, dict(byfam[f]), dict(bear[f]))
print("falsified with controls ok:", ctl[("falsified", True)], "falsified controls failed:", ctl[("falsified", False)])
