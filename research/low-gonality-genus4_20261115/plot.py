#!/usr/bin/env python3
"""Draws figure.svg from RESULTS.json (pure Python, no plotting module). usage: python3 plot.py"""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "RESULTS.json")))
W, H = 1100, 820
PAL = ["#0072B2", "#D55E00", "#009E73", "#CC79A7", "#E69F00", "#56B4E9", "#000000"]
out = []


def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;")


def text(x, y, s, size=12, anchor="start", weight="normal", color="#222", rot=None):
    t = f' transform="rotate({rot} {x} {y})"' if rot else ""
    out.append(f'<text x="{x:.1f}" y="{y:.1f}" font-size="{size}" text-anchor="{anchor}" font-weight="{weight}" fill="{color}" font-family="DejaVu Sans, Arial, sans-serif"{t}>{esc(s)}</text>')


def line(x1, y1, x2, y2, color="#999", w=1, dash=None):
    d = f' stroke-dasharray="{dash}"' if dash else ""
    out.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{color}" stroke-width="{w}"{d}/>')


def circ(x, y, r, color):
    out.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="{color}" fill-opacity="0.85"/>')


class Panel:
    def __init__(self, x0, y0, w, h, title, xl, yl, xlog=False, ylog=False, xr=None, yr=None):
        self.x0, self.y0, self.w, self.h = x0, y0, w, h
        self.xlog, self.ylog = xlog, ylog
        self.xr, self.yr = xr, yr
        out.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="#fff" stroke="#bbb"/>')
        text(x0 + w / 2, y0 - 10, title, 13, "middle", "bold")
        text(x0 + w / 2, y0 + h + 36, xl, 11, "middle")
        text(x0 - 46, y0 + h / 2, yl, 11, "middle", rot=-90)

    def fx(self, v):
        a, b = self.xr
        t = (math.log(v) - math.log(a)) / (math.log(b) - math.log(a)) if self.xlog else (v - a) / (b - a)
        return self.x0 + t * self.w

    def fy(self, v):
        a, b = self.yr
        t = (math.log(v) - math.log(a)) / (math.log(b) - math.log(a)) if self.ylog else (v - a) / (b - a)
        return self.y0 + self.h - t * self.h

    def xticks(self, ticks, fmt=str):
        for t in ticks:
            x = self.fx(t)
            line(x, self.y0 + self.h, x, self.y0 + self.h + 4, "#555")
            text(x, self.y0 + self.h + 17, fmt(t), 10, "middle")

    def yticks(self, ticks, fmt=str):
        for t in ticks:
            y = self.fy(t)
            line(self.x0 - 4, y, self.x0, y, "#555")
            line(self.x0, y, self.x0 + self.w, y, "#eee")
            text(self.x0 - 7, y + 3.5, fmt(t), 10, "end")


text(W / 2, 22, "Genus-4 non-hyperelliptic: relation yields and DLP cost against matched rho  [pcc:unverified]", 15, "middle", "bold")
Y = R["yield"]
ps = sorted(int(p) for p in Y)

# (a) split3 fraction
A = Panel(70, 60, 450, 300, "(a) fibres with 3 rational points (both pencils)", "p", "fraction", xlog=True, xr=(25, 650), yr=(0.10, 0.22))
A.yticks([0.10, 0.125, 0.15, 0.1667, 0.2083, 0.22], lambda v: f"{v:.3f}")
A.xticks(ps)
line(A.x0, A.fy(0.125), A.x0 + A.w, A.fy(0.125), "#D55E00", 1, "5,4")
line(A.x0, A.fy(0.2083), A.x0 + A.w, A.fy(0.2083), "#D55E00", 1, "5,4")
line(A.x0, A.fy(1 / 6), A.x0 + A.w, A.fy(1 / 6), "#009E73", 1.5)
text(A.x0 + A.w - 4, A.fy(0.2083) - 4, "1/6 + 25%", 10, "end", color="#D55E00")
text(A.x0 + A.w - 4, A.fy(0.125) + 12, "1/6 - 25%", 10, "end", color="#D55E00")
for p in ps:
    s = Y[str(p)]["split3_fraction"]
    x = A.fx(p)
    line(x, A.fy(max(s[1], 0.1)), x, A.fy(min(s[2], 0.22)), "#0072B2", 2)
    circ(x, A.fy(s[0]), 4.5, "#0072B2")

# (b) yield ratio
B = Panel(610, 60, 450, 300, "(b) A-line yield per pair, measured / (phi^3/6)", "p", "ratio", xlog=True, xr=(25, 650), yr=(0.0, 1.6))
B.yticks([0, 0.5, 0.7, 1.0, 1.3, 1.6], lambda v: f"{v:.1f}")
B.xticks(ps)
line(B.x0, B.fy(0.7), B.x0 + B.w, B.fy(0.7), "#D55E00", 1, "5,4")
line(B.x0, B.fy(1.3), B.x0 + B.w, B.fy(1.3), "#D55E00", 1, "5,4")
line(B.x0, B.fy(1.0), B.x0 + B.w, B.fy(1.0), "#009E73", 1.5)
phis = [0.3, 0.5, 0.7, 1.0]
for k, phi in enumerate(phis):
    for p in ps:
        for c in Y[str(p)]["phi_cells"]:
            if c["phi"] == phi:
                x = B.fx(p) + (k - 1.5) * 6
                filled = c["m_mean"] >= 60
                col = PAL[k]
                if filled:
                    circ(x, B.fy(c["ratio_naive_pooled"]), 4.5, col)
                else:
                    out.append(f'<circle cx="{x:.1f}" cy="{B.fy(c["ratio_naive_pooled"]):.1f}" r="4.5" fill="none" stroke="{col}" stroke-width="1.5"/>')
for k, phi in enumerate(phis):
    circ(B.x0 + 10 + 62 * k, B.y0 + 14, 4.5, PAL[k])
    text(B.x0 + 18 + 62 * k, B.y0 + 18, f"phi={phi}", 10)
text(B.x0 + B.w - 4, B.y0 + B.h - 6, "filled: |F| >= 60 (tested); open: |F| < 60", 10, "end", color="#555")

# (c) cost vs p
D = R.get("dlp", [])
cells = [(c, [i for i in c["ic"] if i.get("ok") and "ratio" in i]) for c in D]
cells = [(c, i[0]) for c, i in cells if i]
if cells:
    allv = []
    for c, i in cells:
        allv += [c["S_rho"]["w=0"], i["S"]["w=0"], i["S"]["w=1"], i["S"]["w=cal"]]
    lo, hi = 10 ** math.floor(math.log10(min(allv))), 10 ** math.ceil(math.log10(max(allv)))
    C = Panel(70, 450, 450, 300, "(c) cost per solved DLP (GAE = one group operation)", "p", "GAE", xlog=True, ylog=True, xr=(45, 700), yr=(lo, hi))
    C.xticks([61, 127, 251, 509])
    dec = []
    v = lo
    while v <= hi * 1.01:
        dec.append(v)
        v *= 10
    C.yticks(dec, lambda v: f"1e{int(round(math.log10(v)))}")
    series = [("rho (plain, measured)", lambda c, i: c["S_rho"]["w=0"], PAL[6]), ("IC w=0", lambda c, i: i["S"]["w=0"], PAL[0]), ("IC w=w_cal", lambda c, i: i["S"]["w=cal"], PAL[2]),
              ("IC w=0.1", lambda c, i: i["S"]["w=0.1"], PAL[4]), ("IC w=1", lambda c, i: i["S"]["w=1"], PAL[1])]
    for k, (nm, fn, col) in enumerate(series):
        pts = {}
        for c, i in cells:
            pts.setdefault(c["p"], []).append(fn(c, i))
        xs = sorted(pts)
        mean = [sum(pts[p]) / len(pts[p]) for p in xs]
        for a, b in zip(range(len(xs) - 1), range(1, len(xs))):
            line(C.fx(xs[a]), C.fy(mean[a]), C.fx(xs[b]), C.fy(mean[b]), col, 1.5)
        for p in xs:
            for v in pts[p]:
                circ(C.fx(p), C.fy(v), 3, col)
        text(C.x0 + 8, C.y0 + 16 + 14 * k, nm, 10, color=col)

    # (d) break-even w
    be = R.get("break_even_by_p", {})
    allw = [v for lst in be.values() for v in lst if v]
    if allw:
        Dp = Panel(610, 450, 450, 300, "(d) break-even price w* (S_IC = S_rho)", "p", "w* (GAE per native op)", xlog=True, ylog=True, xr=(45, 700), yr=(1e-5, 10))
        Dp.xticks([61, 127, 251, 509])
        Dp.yticks([1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1], lambda v: f"1e{int(round(math.log10(v)))}")
        for yv, nm in [(0.1, "w=0.1"), (1.0, "w=1")]:
            line(Dp.x0, Dp.fy(yv), Dp.x0 + Dp.w, Dp.fy(yv), "#D55E00", 1, "5,4")
            text(Dp.x0 + Dp.w - 4, Dp.fy(yv) - 4, nm, 10, "end", color="#D55E00")
        cal = [c["calib"]["w_cal_p"] for c, _ in cells]
        if cal:
            wc = sum(cal) / len(cal)
            line(Dp.x0, Dp.fy(wc), Dp.x0 + Dp.w, Dp.fy(wc), "#009E73", 1.2, "5,4")
            text(Dp.x0 + 6, Dp.fy(wc) - 4, "w_cal (mulmod, F_p)", 10, color="#009E73")
        for p, lst in be.items():
            for v in lst:
                if v and v > 0:
                    circ(Dp.fx(int(p)), Dp.fy(v), 4.5, PAL[0])
out.insert(0, f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}"><rect width="{W}" height="{H}" fill="#fff"/>')
out.append("</svg>")
open(os.path.join(HERE, "figure.svg"), "w").write("\n".join(out))
print("wrote figure.svg")
