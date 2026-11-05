#!/usr/bin/env python3
"""results/analysis-main.json + results/fig-kappa-ladder.svg -> REPORT.html (PDF via Chromium, see README)"""
import json, os
os.chdir(os.path.join(os.path.dirname(__file__), ".."))
a = json.load(open("results/analysis-main.json")); L = a["lines"]; cal = a["calibration"]; o = cal["original_rule_reported_only"]
FAM = {1: "Dickson (u^k + u^-k)", 2: "geometric progression (x0 rho^k)", 3: "quadratic progression (x0 + k^2)"}
f = lambda x: f"{x:.4f}"
def rows(kind):
    out = ""
    for l in L:
        if l["kind"] != kind: continue
        name = FAM[l["d"]] if kind in ("struct", "aa") else (f'gamma = {l["gamma"]}' if kind == "plant" else "Z/N interval")
        ks = " / ".join(f'{k["kappa"]:.2f}' for k in l["kappa_by_rung"])
        out += f'<tr><td>{name} θ={l["theta"]:.2f}</td><td>{l["class"]}</td><td>{f(l["delta_hat"])} [{f(l["ci95"][0])}, {f(l["ci95"][1])}]</td><td>{ks}</td></tr>'
    return out
ub = max(l["ci95"][1] for l in L if l["kind"] == "struct")
minM = min(c["M"] for c in a["cells"] if "M" in c)
svg = open("results/fig-kappa-ladder.svg").read()
html = f"""<!doctype html><meta charset=utf-8><title>PFDR ladder, other structured families</title>
<style>body{{font:13px/1.45 Helvetica,Arial,sans-serif;max-width:900px;margin:24px auto;color:#111}}h1{{font-size:20px}}h2{{font-size:15px;margin-top:22px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 8px;text-align:right}}td:first-child,th:first-child{{text-align:left}}code{{background:#f3f3f3;padding:0 3px}}</style>
<h1>Dickson, geometric and quadratic x-bases do not show growing pair-sum coincidences either</h1>
<p><b>Reading: <code>{a["reading"]}</code></b> for <code>EXP-PFDR-2d36f9-R2-FAM</code>. All six structured lines (3 families × θ 0.30, 0.35) are valid (smallest rung mean coincidence count {minM:.0f} ≥ 2500), the selftest and all blocking controls (C3 plants, C4 Z/N, C5 A/A, C6 z_cal) pass, no line is ALIVE and no job failed (19,887 jobs). Five-rung slopes of log₂κ on log₂N are between −0.005 and +0.007, with the largest upper 95% limit {ub:.4f} against the break-even rate 1/3.</p>
<p><b>What was run.</b> The same R2 ladder (kernel, curves, cells, statistics, controls, readings; <code>PROTOCOL.md</code>, frozen in commit b159698c17 before data) with the structured base replaced by three families built from the abscissa stream: Dickson x<sub>k</sub> = u<sup>k</sup> + u<sup>−k</sup>, a geometric progression x<sub>0</sub>ρ<sup>k</sup>, and a quadratic progression x<sub>0</sub> + k². One amendment before analysis: a first census attempt aborted on a cycling Dickson/geometric stream (parameter of small multiplicative order); the fix resamples such parameters (4 of 19,887 jobs needed one resample). The partial data of the aborted attempt are kept unmodified in <code>aborted-attempt1/</code>.</p>
{svg}
<h2>Structured lines (κ per rung 16 / 20 / 24 / 28 / 32 bits; slope with 95% cluster-bootstrap interval)</h2>
<table><tr><th>line</th><th>class</th><th>slope [95%]</th><th>κ per rung</th></tr>{rows("struct")}</table>
<h2>Controls</h2>
<table><tr><th>line</th><th>class</th><th>slope [95%]</th><th>κ per rung</th></tr>{rows("plant")}{rows("zn")}{rows("aa")}</table>
<p>The instrument sees growth (planted γ = 1/6, 1/3, 1/2 recovered as 0.149, 0.314–0.325, 0.486–0.491; Z/N 0.699 and 0.644) and reads γ = 0 and all A/A lines as SUB-BE. <b>C6:</b> n = {cal["n"]}, mean {cal["mean"]:.3f}, sd {cal["sd"]:.3f} (bounds |mean| ≤ 0.62, sd in [0.548, 1.454]); the original 30-cell rule gives mean {o["mean"]:.3f}, sd {o["sd"]:.3f}.</p>
<h2>Scope and what it says</h2>
<ul><li>With the window / rank-d label-GAP lines of the rerun (PR #3188) this makes twelve structured x-base lines (six families × two θ) over 16–32 bits with valid controls, none showing κ growth with N: the pair-sum coincidence excess of these bases tracks a random base. Obstruction: max upper95(δ) = {ub:.4f} against 1/3.</li>
<li><b>Not covered, and why.</b> The FAM-1 mitm-engine arms (<code>subgroup</code>, <code>small_x</code>, <code>dickson</code> of EXP-PFDR-011cd0) are not reproduced: that engine is not in this repository, the <code>small_x</code> arm is the x-window line already measured, and <code>subgroup</code> needs a proper subgroup of the curve group (these curves have prime order); the Dickson line here is a base-construction analogue, not that arm. The bbc53e-base is the planted shared-unknown base: it is the γ control (recovered slopes above), a plant for κ growth by construction, not a candidate family.</li>
<li>Sizes above 32 bits and other families are not covered. It prices no end-to-end index calculus and says nothing about rho. Run as a study in this repository (local, counts deterministic); the lead disposition is the sibling repository's call. Raw jobs are in the artifact store with manifests.</li></ul>"""
open("REPORT.html", "w").write(html)
