#!/usr/bin/env python3
"""Draws figure.svg from RESULTS.json (pure Python, no plotting library).
usage: python3 plot.py [RESULTS.json] [figure.svg]"""
import json, math, sys

src = sys.argv[1] if len(sys.argv) > 1 else "RESULTS.json"
dst = sys.argv[2] if len(sys.argv) > 2 else "figure.svg"
R = json.load(open(src))
W, H = 1180, 930
PW, PH = 480, 330
COL = {"0": "#2a6fb0", "0.1": "#d9822b", "1": "#b03a48"}
out = []


def esc(t):
    return t.replace("&", "&amp;").replace("<", "&lt;")


def panel(x0, y0, title, xs, series, ylabel, ylog=True, hline=None, ymin=None, ymax=None):
    out.append('<g transform="translate(%d,%d)">' % (x0, y0))
    out.append('<text x="0" y="-14" font-size="14" font-weight="bold" fill="#222">%s</text>' % esc(title))
    allv = [v for s in series for (x, v) in s["pts"] if v is not None and v > 0]
    lo = ymin if ymin else min(allv) / 1.5
    hi = ymax if ymax else max(allv) * 1.5
    lx0, lx1 = math.log(min(xs) / 1.3), math.log(max(xs) * 1.3)
    ly0, ly1 = math.log(lo), math.log(hi)

    def X(x):
        return PW * (math.log(x) - lx0) / (lx1 - lx0)

    def Y(v):
        return PH - PH * (math.log(v) - ly0) / (ly1 - ly0)

    out.append('<rect x="0" y="0" width="%d" height="%d" fill="#fcfcfc" stroke="#999"/>' % (PW, PH))
    e = math.floor(math.log10(lo))
    while 10 ** e <= hi:
        if 10 ** e >= lo:
            y = Y(10 ** e)
            out.append('<line x1="0" y1="%.1f" x2="%d" y2="%.1f" stroke="#ddd"/>' % (y, PW, y))
            out.append('<text x="-6" y="%.1f" font-size="10" text-anchor="end" fill="#444">1e%d</text>' % (y + 3, e))
        e += 1
    for q in xs:
        x = X(q)
        out.append('<line x1="%.1f" y1="0" x2="%.1f" y2="%d" stroke="#eee"/>' % (x, x, PH))
        out.append('<text x="%.1f" y="%d" font-size="10" text-anchor="middle" fill="#444">%d</text>' % (x, PH + 14, q))
    if hline:
        y = Y(hline)
        out.append('<line x1="0" y1="%.1f" x2="%d" y2="%.1f" stroke="#222" stroke-dasharray="5,3"/>' % (y, PW, y))
    for s in series:
        pts = [(X(x), Y(v)) for x, v in s["pts"] if v is not None and v > 0]
        if not pts:
            continue
        dash = ' stroke-dasharray="%s"' % s["dash"] if s.get("dash") else ""
        out.append('<polyline fill="none" stroke="%s" stroke-width="2"%s points="%s"/>' % (s["color"], dash, " ".join("%.1f,%.1f" % p for p in pts)))
        for p in pts:
            out.append('<circle cx="%.1f" cy="%.1f" r="3" fill="%s"/>' % (p[0], p[1], s["color"]))
    out.append('<text x="%d" y="%d" font-size="11" text-anchor="middle" fill="#444">q</text>' % (PW // 2, PH + 30))
    out.append('<text x="-52" y="%d" font-size="11" fill="#444" transform="rotate(-90 -52 %d)" text-anchor="middle">%s</text>' % (PH // 2, PH // 2, esc(ylabel)))
    # legend (white backing so lines behind stay readable)
    out.append('<rect x="4" y="2" width="%d" height="%d" fill="white" fill-opacity="0.85"/>' % (265, 8 + 14 * len(series)))
    for i, s in enumerate(series):
        yy = 14 + 14 * i
        dash = ' stroke-dasharray="%s"' % s["dash"] if s.get("dash") else ""
        out.append('<line x1="10" y1="%d" x2="34" y2="%d" stroke="%s" stroke-width="2"%s/>' % (yy, yy, s["color"], dash))
        out.append('<text x="40" y="%d" font-size="10" fill="#222">%s</text>' % (yy + 3, esc(s["name"])))
    out.append("</g>")


cells = [c for c in R["cells"] if c["n_ok"] and c["tag"] == "main" and c["c"] == 2.0]
qs = sorted(c["q"] for c in cells)
rm = {r["q"]: r for r in R["rho_model_by_q"]}
meas = {}
for r in R["rho"]:
    if r["kind"] == "rho":
        meas.setdefault(r["q"], r)
series = []
for w in ("0", "0.1", "1"):
    key = "S_w%s_cI3" % w
    series.append({"name": "IC-sparse c=2, w=%s" % w, "color": COL[w], "pts": [(c["q"], c[key]) for c in cells]})
    series.append({"name": "rho (measured then modelled), w=%s" % w, "color": COL[w], "dash": "6,3",
                   "pts": [(q, meas[q]["S_w%s" % w] if q in meas else rm[q]["S_w%s" % w]) for q in qs if q in rm]})
panel(70, 80, "A  cost S in GAE, c = 2", qs, series, "S (GAE)")
ser = []
for w in ("0", "0.1", "1"):
    pts = []
    for d in R["distance"]:
        if d["tag"] == "main" and d["c"] == 2.0:
            v = d["S_over_rho_model_w%s" % w]
            pts.append((d["q"], v[0] if v else None))
    ser.append({"name": "S_IC / S_rho(model), w=%s" % w, "color": COL[w], "pts": pts})
for lab, name, col in (("wcal", "w = w_cal (calibrated, about 3e-4)", "#2e8b57"), ("wcal10", "w = 10 w_cal", "#6a9f2d")):
    pts = []
    for d in R["distance"]:
        if d["tag"] == "main" and d["c"] == 2.0 and ("S_over_rho_model_" + lab) in d:
            v = d["S_over_rho_model_" + lab]
            pts.append((d["q"], v[0] if v else None))
    ser.append({"name": "S_IC / S_rho(model), " + name, "color": col, "dash": "2,2", "pts": pts})
panel(640, 80, "B  distance to matched rho (c = 2); 1 = parity", qs, ser, "S_IC / S_rho", hline=1.0)
ser = []
for tag, name, col, dash in (("main", "primary: elimination + Wiedemann", "#2a6fb0", None), ("bfs", "bfs rows + Wiedemann (exploratory)", "#5aa469", "6,3"),
                             ("light", "lightest rows + Wiedemann (exploratory)", "#8a5ab0", "6,3")):
    pts = [(c["q"], c["LA_elim"] + c["LA_wied"] + c["LA_bm"]) for c in R["cells"] if c["n_ok"] and c["tag"] == tag and c["c"] == 2.0]
    if pts:
        ser.append({"name": name, "color": col, "dash": dash, "pts": pts})
ser.append({"name": "dense Gauss-Jordan on the same rows (A/B)", "color": "#b03a48",
            "pts": [(a["q"], a["LA_dense_total"]) for a in R["ab"]]})
panel(70, 540, "C  linear-algebra operations mod ell, c = 2", qs, ser, "mod-ell operations")
ser = []
for tag, name, col, dash in (("main", "after elimination (primary)", "#2a6fb0", None), ("bfs", "bfs rows", "#5aa469", "6,3"), ("light", "lightest rows", "#8a5ab0", "6,3")):
    pts = [(c["q"], c["w_fb_mean"]) for c in R["cells"] if c["n_ok"] and c["tag"] == tag and c["c"] == 2.0]
    if pts:
        ser.append({"name": name, "color": col, "dash": dash, "pts": pts})
panel(640, 540, "D  mean row weight handed to the solver, c = 2 (raw rows: at most 5)", qs, ser, "entries per row", ymin=5, ymax=120)
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d" font-family="Helvetica,Arial,sans-serif">' % (W, H, W, H),
       '<rect width="%d" height="%d" fill="white"/>' % (W, H),
       '<text x="20" y="22" font-size="16" font-weight="bold" fill="#111">Generic plane quartics: Diem IC with a c sqrt(q) factor base, double large primes, sparse LA</text>',
       '<text x="20" y="40" font-size="11" fill="#666">[pcc:unverified] local development check; S priced at w = 0, 0.1, 1 GAE per native operation; rho above its measured range is modelled (dashed).</text>']
svg += out + ["</svg>"]
open(dst, "w").write("\n".join(svg))
print("wrote", dst)
