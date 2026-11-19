#!/usr/bin/env python3
"""Paired comparison of every arm against `inc`, distance to rho, per-phase constants.
Reads sessions/{p24,p28} (ecbench sessions), calls `ecbench compare --json`.
Writes analysis/summary.json, analysis/summary.md, figures/ratios.svg."""
import json, subprocess, pathlib, collections, sys
here = pathlib.Path(__file__).resolve().parent.parent
B = "/home/user/crypto/target/release/ecbench"
W = [0, 0.1, 1]
FB = {"p24": 128, "p28": 256}
N = {"p24": 16782299, "p28": 268407199}

def cmp(c, a, b, w):
    out = subprocess.run([B, "compare", "--dir", str(here/"sessions"/c), "--a", a, "--b", b,
                          "--weight", str(w), "--json"], capture_output=True, text=True)
    d = json.loads(out.stdout)["ops"]
    if d.get("status") != "ok":
        return {"status": d.get("status")}
    ci = d.get("ci95_bootstrap") or d["ci95"]
    return {"ratio": d["ratio_b_over_a"], "lo": ci[0], "hi": ci[1], "pairs": d["pairs"]}

def phases(c):
    acc = collections.defaultdict(lambda: collections.defaultdict(list))
    for l in open(here/"sessions"/c/"records.jsonl"):
        r = json.loads(l)
        if r["warmup"]:
            continue
        ph = {p["name"]: p for p in r["phases"]}
        a = r["arm"]
        acc[a]["setup"].append(ph.get("oracle_setup", {}).get("gae", 0))
        acc[a]["rel"].append(ph.get("relations", {}).get("gae", 0))
        acc[a]["trials"].append(r["counters"].get("targets_tried", 0))
        acc[a]["relations"].append(r["counters"].get("relations_found", 0))
        acc[a]["rowops"].append(r["counters"].get("matrix_work", 0))
        acc[a]["n"].append(1)
    return {a: {k: sum(v)/len(v) for k, v in d.items() if k != "n"} | {"runs": len(d["n"])} for a, d in acc.items()}

arms = None
res = {}
for c in ("p24", "p28"):
    ph = phases(c)
    arms = [a for a in ph if a not in ("rho",)]
    F, n = FB[c], N[c]
    res[c] = {"F": F, "r": n, "arms": {}}
    for a in arms:
        p = ph[a]
        e = {"phase": p}
        e["a_eff"] = p["setup"]/F**2          # table constant, meaningful for m = 2 pair oracles
        e["b_eff"] = p["rel"]/(n/(4*F))       # trial constant (GAE per N/(4F))
        e["vs_inc"] = {str(w): cmp(c, "inc", a, w) for w in W} if a != "inc" else None
        e["vs_rho"] = {str(w): cmp(c, "rho", a, w) for w in W}
        if a != "inc":
            v = e["vs_inc"]
            ok = all("hi" in v[str(w)] for w in W)
            e["improves_w0"] = ok and v["0"]["hi"] < 1
            e["improves_band"] = ok and all(v[str(w)]["hi"] < 1 for w in W)
            e["tie"] = ok and all(abs(v[str(w)]["ratio"]-1) < 1e-9 and abs(v[str(w)]["hi"]-1) < 1e-6 for w in (0,))
        res[c]["arms"][a] = e
    res[c]["rho"] = {str(w): None for w in W}
json.dump(res, open(here/"analysis"/"summary.json", "w"), indent=1)

def f(x): return f"{x:.3f}"
lines = []
for c in ("p24", "p28"):
    lines.append(f"\n### {c} (|F| = {FB[c]}), 8 targets x 3 rounds, ratio arm / inc [95% cluster bootstrap]; distance = arm / rho.negation\n")
    lines.append("| arm | setup GAE | trial GAE | a_eff | b_eff | w=0 | w=0.1 | w=1 | improves (w0 / band) | dist w=0 / 0.1 / 1 |")
    lines.append("|:--|--:|--:|--:|--:|:--|:--|:--|:--|:--|")
    for a, e in res[c]["arms"].items():
        v = e["vs_inc"]
        cells = ["1 (ref)"]*3 if v is None else [f"{f(v[str(w)]['ratio'])} [{f(v[str(w)]['lo'])}, {f(v[str(w)]['hi'])}]" if "ratio" in v[str(w)] else v[str(w)]["status"] for w in W]
        imp = "-" if v is None else ("tie" if e["tie"] else f"{'Y' if e['improves_w0'] else 'n'} / {'Y' if e['improves_band'] else 'n'}")
        d = " / ".join(f"{e['vs_rho'][str(w)]['ratio']:.1f}" if "ratio" in e["vs_rho"][str(w)] else "?" for w in W)
        lines.append(f"| {a} | {e['phase']['setup']:.0f} | {e['phase']['rel']:.0f} | {e['a_eff']:.3f} | {e['b_eff']:.3f} | " + " | ".join(cells) + f" | {imp} | {d} |")
open(here/"analysis"/"summary.md", "w").write("\n".join(lines)+"\n")
print("\n".join(lines))

# figure: ratio vs inc at w=0 and w=1, log scale, arms with ratio < 30
import math
W_, H_ = 900, 520
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W_}" height="{H_}" font-family="sans-serif" font-size="11">',
       '<style>:root{--fg:#222;--bg:#fff;--c0:#1f77b4;--c1:#d95f02;--c2:#7b3294}@media(prefers-color-scheme:dark){:root{--fg:#ddd;--bg:#111;--c0:#6baed6;--c1:#fd8d3c;--c2:#c2a5cf}}text{fill:var(--fg)}</style>',
       f'<rect width="{W_}" height="{H_}" fill="var(--bg)"/>',
       '<text x="450" y="18" text-anchor="middle" font-size="13">arm / inc, counted S (log scale, 1 = incumbent), p24 (|F|=128) and p28 (|F|=256) [pcc:unverified]</text>']
names = [a for a in res["p24"]["arms"] if a != "inc"]
x0, y0, pw, ph_ = 130, 40, 740, 440
lo_, hi_ = math.log10(0.3), math.log10(300)
def X(r): return x0 + (math.log10(max(min(r, 300), 0.3)) - lo_)/(hi_-lo_)*pw
rowh = ph_/len(names)
for k in (0.3, 0.5, 1, 2, 5, 10, 30, 100, 300):
    svg.append(f'<line x1="{X(k):.1f}" x2="{X(k):.1f}" y1="{y0}" y2="{y0+ph_}" stroke="var(--fg)" stroke-opacity="{0.8 if k==1 else 0.15}"/><text x="{X(k):.1f}" y="{y0+ph_+14}" text-anchor="middle">{k}</text>')
for i, a in enumerate(names):
    y = y0 + i*rowh + rowh/2
    svg.append(f'<text x="{x0-6}" y="{y+4:.1f}" text-anchor="end">{a}</text>')
    for j, (c, col) in enumerate((("p24", "var(--c0)"), ("p28", "var(--c1)"))):
        e = res[c]["arms"][a]["vs_inc"]
        for wi, (w, mk) in enumerate((("0", "circle"), ("1", "rect"))):
            if "ratio" not in e[w]: continue
            yy = y + (j*2-1)*4 + (wi-0.5)*0
            x = X(e[w]["ratio"])
            if mk == "circle":
                svg.append(f'<circle cx="{x:.1f}" cy="{yy:.1f}" r="3.5" fill="{col}"/>')
            else:
                svg.append(f'<rect x="{x-3:.1f}" y="{yy-3:.1f}" width="6" height="6" fill="none" stroke="{col}"/>')
svg.append(f'<text x="{x0}" y="{H_-8}">filled circle = w = 0, open square = w = 1; blue = p24, orange = p28; values clipped to [0.3, 300]</text></svg>')
open(here/"figures"/"ratios.svg", "w").write("\n".join(svg))
