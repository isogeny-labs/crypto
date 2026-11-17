"""Roofline SVG for optlevel records (standard library only).

Log-log axes: x = arithmetic intensity in operations per byte of memory
traffic, y = operations per second.  One roof per operation class (solid: one
core, dashed: all cores), one marker per record at its single-thread point
(filled) and its full-parallel point (ring).  Records whose traffic is unknown
or whose working set stays in cache are drawn at the right edge and say so.

Colours are the first three categorical slots of the lab's validated chart
palette (blue, orange, aqua), which pass the all-pairs colour-vision checks on
both surfaces; a fourth class folds into grey "other".  Every element carries
a light-mode presentation attribute and a class that the embedded stylesheet
re-colours under `prefers-color-scheme: dark`, so the file reads on both
backgrounds and in renderers that ignore stylesheets.
"""
from __future__ import annotations

import math
from html import escape
from pathlib import Path

LIGHT = {"surface": "#fcfcfb", "text": "#0b0b0b", "text2": "#52514e", "muted": "#898781", "grid": "#e4e3dd", "axis": "#c3c2b7",
         "s0": "#2a78d6", "s1": "#eb6834", "s2": "#1baf7a", "s3": "#898781"}
DARK = {"surface": "#1a1a19", "text": "#f0efec", "text2": "#c3c2b7", "muted": "#898781", "grid": "#2c2c2a", "axis": "#383835",
        "s0": "#3987e5", "s1": "#d95926", "s2": "#199e70", "s3": "#898781"}
W, H, ML, MR, MT, MB = 920, 560, 86, 250, 54, 62


def _fmt(v):
    for div, suf in ((1e12, "T"), (1e9, "G"), (1e6, "M"), (1e3, "k")):
        if abs(v) >= div:
            return f"{v / div:.3g}{suf}"
    return f"{v:.3g}"


def _css():
    def block(pal, pre=""):
        out = [f"{pre}.surface{{fill:{pal['surface']}}}", f"{pre}.t1{{fill:{pal['text']}}}", f"{pre}.t2{{fill:{pal['text2']}}}",
               f"{pre}.mut{{fill:{pal['muted']}}}", f"{pre}.grid{{stroke:{pal['grid']}}}", f"{pre}.axis{{stroke:{pal['axis']}}}",
               f"{pre}.ring{{stroke:{pal['surface']}}}", f"{pre}.halo{{stroke:{pal['surface']}}}"]
        for i in range(4):
            out += [f"{pre}.l{i}{{stroke:{pal['s' + str(i)]}}}", f"{pre}.f{i}{{fill:{pal['s' + str(i)]}}}"]
        return "".join(out)
    return ("text{font-family:system-ui,-apple-system,'Segoe UI',Helvetica,Arial,sans-serif}" + block(LIGHT)
            + "@media (prefers-color-scheme: dark){" + block(DARK) + "}")


def write_roofline(calib, records, out, title="Roofline"):
    classes = []
    for r in records:
        c = r["kernel"]["op_class"]
        if c not in classes and c in calib["compute"]:
            classes.append(c)
    slot = {c: min(i, 3) for i, c in enumerate(classes)}
    bw = calib["memory"]["bandwidth_bytes_per_s"]
    ks = sorted(int(k) for k in bw)
    k1, kn = ks[0], ks[-1]
    pts = []
    for r in records:
        c = r["kernel"]["op_class"]
        if c not in slot:
            continue
        opb = r["intensity"]["ops_per_byte"]
        for name, key in (("single", "single"), ("full", None)):
            if name not in r["attainment"]:
                continue
            ach = r["achieved"]["single"] if name == "single" else r["achieved"][f"threads_{r['roofline']['full']['threads']}"]
            pts.append({"label": r["label"] or c, "cls": c, "x": opb, "y": ach["ops_per_s"]["median"],
                        "lo": ach["ops_per_s"]["min"], "hi": ach["ops_per_s"]["max"], "which": name,
                        "att": r["attainment"][name]["value"], "tier": r["tier"],
                        "threads": r["roofline"][name]["threads"],
                        "incache": bool(r["intensity"].get("in_cache"))})
    peaks = [calib["compute"][c]["total_ops_per_s"][str(k)]["max"] for c in classes for k in (k1, kn)]
    xs = [p["x"] for p in pts if p["x"]]
    knees = [calib["compute"][c]["total_ops_per_s"][str(k)]["max"] / bw[str(k)]["max"] for c in classes for k in (k1, kn)]
    xmin = 10 ** math.floor(math.log10(min(xs + knees) / 4))
    xmax = 10 ** math.ceil(math.log10(max(xs + knees) * 4))
    ys = [p["lo"] for p in pts] + [xmin * bw[str(k1)]["max"]]
    ymin = 10 ** math.floor(math.log10(min(ys) / 1.5))
    ymax = 10 ** math.ceil(math.log10(max(peaks + [p["hi"] for p in pts]) * 1.2))
    pw, ph = W - ML - MR, H - MT - MB

    def X(v):
        return ML + pw * (math.log10(v) - math.log10(xmin)) / (math.log10(xmax) - math.log10(xmin))

    def Y(v):
        v = min(max(v, ymin), ymax)
        return MT + ph * (1 - (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin)))

    o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" role="img" '
         f'aria-label="{escape(title)}: roofline, operations per second against operations per byte">',
         f"<style>{_css()}</style>",
         f'<rect class="surface" x="0" y="0" width="{W}" height="{H}" fill="{LIGHT["surface"]}"/>',
         f'<text class="t1" x="{ML}" y="24" font-size="16" font-weight="600" fill="{LIGHT["text"]}">{escape(title)}</text>',
         f'<text class="t2" x="{ML}" y="42" font-size="11.5" fill="{LIGHT["text2"]}">host {escape(calib["host"]["cpu_model"])} '
         f'({calib["host"]["fingerprint"]}); attainment = marker / roof at its intensity; not a speed claim</text>']
    e = math.floor(math.log10(xmin))
    while 10 ** e <= xmax * 1.0001:
        x = X(10 ** e)
        o.append(f'<line class="grid" x1="{x:.1f}" y1="{MT}" x2="{x:.1f}" y2="{MT + ph}" stroke="{LIGHT["grid"]}" stroke-width="1"/>')
        o.append(f'<text class="mut" x="{x:.1f}" y="{MT + ph + 18}" font-size="11" text-anchor="middle" fill="{LIGHT["muted"]}">{_fmt(10 ** e)}</text>')
        e += 1
    e = math.floor(math.log10(ymin))
    while 10 ** e <= ymax * 1.0001:
        y = Y(10 ** e)
        o.append(f'<line class="grid" x1="{ML}" y1="{y:.1f}" x2="{ML + pw}" y2="{y:.1f}" stroke="{LIGHT["grid"]}" stroke-width="1"/>')
        o.append(f'<text class="mut" x="{ML - 8}" y="{y + 4:.1f}" font-size="11" text-anchor="end" fill="{LIGHT["muted"]}">{_fmt(10 ** e)}</text>')
        e += 1
    o.append(f'<line class="axis" x1="{ML}" y1="{MT + ph}" x2="{ML + pw}" y2="{MT + ph}" stroke="{LIGHT["axis"]}" stroke-width="1"/>')
    o.append(f'<text class="t2" x="{ML + pw / 2}" y="{H - 14}" font-size="12" text-anchor="middle" fill="{LIGHT["text2"]}">'
             'arithmetic intensity (operations per byte of memory traffic)</text>')
    o.append(f'<text class="t2" transform="translate(18 {MT + ph / 2}) rotate(-90)" font-size="12" text-anchor="middle" fill="{LIGHT["text2"]}">'
             'operations per second</text>')
    # roofs
    for c in classes:
        i = slot[c]
        for k, dash in ((k1, ""),) + (((kn, ' stroke-dasharray="6 4"'),) if kn != k1 else ()):
            if True:
                peak, b = calib["compute"][c]["total_ops_per_s"][str(k)]["max"], bw[str(k)]["max"]
                knee = min(max(peak / b, xmin), xmax)
                d = f"M{X(xmin):.1f},{Y(xmin * b):.1f} L{X(knee):.1f},{Y(min(peak, knee * b)):.1f} L{X(xmax):.1f},{Y(peak):.1f}"
                o.append(f'<path class="l{i}" d="{d}" fill="none" stroke="{LIGHT["s" + str(i)]}" stroke-width="2"{dash}>'
                         f'<title>{escape(c)} roof, {k} core(s): peak {_fmt(peak)} ops/s, bandwidth {_fmt(b)} B/s</title></path>')
    # markers, with direct labels stacked top to bottom in marker order
    geo = []
    for p in pts:
        unknown = p["x"] is None
        geo.append((Y(p["y"]), X(p["x"]) if not unknown else ML + pw - 10, unknown, p))
    geo.sort(key=lambda g: g[0])
    last_right = last_left = -1e9
    for y, x, unknown, p in geo:
        i = slot[p["cls"]]
        col = LIGHT["s" + str(i)]
        tip = (f'{p["label"]} ({p["which"]}, {p["threads"]} thread(s), {p["tier"]}): {_fmt(p["y"])} ops/s '
               f'[{_fmt(p["lo"])}, {_fmt(p["hi"])}], attainment {p["att"]:.3f}'
               + ("; working set in cache, no DRAM traffic, drawn at the right edge" if p["incache"]
                  else "; memory traffic unknown, drawn at the right edge" if unknown else ""))
        o.append(f'<line class="l{i}" x1="{x:.1f}" y1="{Y(p["lo"]):.1f}" x2="{x:.1f}" y2="{Y(p["hi"]):.1f}" stroke="{col}" stroke-width="2"/>')
        if p["which"] == "single":
            o.append(f'<circle class="f{i} ring" cx="{x:.1f}" cy="{y:.1f}" r="5.5" fill="{col}" stroke="{LIGHT["surface"]}" stroke-width="2"><title>{escape(tip)}</title></circle>')
        else:
            o.append(f'<circle class="l{i} surface" cx="{x:.1f}" cy="{y:.1f}" r="5" fill="{LIGHT["surface"]}" stroke="{col}" stroke-width="2"><title>{escape(tip)}</title></circle>')
        right = x > ML + pw - 170
        ly = max(y + 4, (last_right if right else last_left) + 14)
        if right:
            last_right = ly
        else:
            last_left = ly
        tag = "1t" if p["which"] == "single" else f'{p["threads"]}t'
        anchor, lx = ("end", x - 22) if right else ("start", x + 22)
        ex = x - 7 if right else x + 7
        o.append(f'<path class="axis" d="M{ex:.1f},{y:.1f} L{lx + (4 if right else -4):.1f},{ly - 4:.1f}" fill="none" stroke="{LIGHT["axis"]}" stroke-width="1"/>')
        o.append(f'<text class="t1 halo" x="{lx:.1f}" y="{ly:.1f}" font-size="11" text-anchor="{anchor}" fill="{LIGHT["text"]}" '
                 f'stroke="{LIGHT["surface"]}" stroke-width="3" paint-order="stroke" stroke-linejoin="round">'
                 f'{escape(p["label"])} {tag}: {p["att"]:.2f}{"" if not unknown else " (in cache)" if p["incache"] else "*"}</text>')
    # legend
    lx, ly = ML + pw + 18, MT + 6
    o.append(f'<text class="t2" x="{lx}" y="{ly}" font-size="11.5" font-weight="600" fill="{LIGHT["text2"]}">roof per operation class</text>')
    for c in classes:
        i = slot[c]
        ly += 20
        o.append(f'<line class="l{i}" x1="{lx}" y1="{ly - 4}" x2="{lx + 26}" y2="{ly - 4}" stroke="{LIGHT["s" + str(i)]}" stroke-width="2"/>')
        o.append(f'<text class="t1" x="{lx + 32}" y="{ly}" font-size="11.5" fill="{LIGHT["text"]}">{escape(c)}</text>')
    notes = [f"solid: {k1} core", f"dashed: all {kn} cores" if kn != k1 else "", "filled marker: single thread", "ring: full parallelism",
             "bar: min to max over runs", "label: attainment", "in cache: no DRAM traffic (right edge)", "*: traffic unknown (right edge)"]
    ly += 14
    for n in notes:
        if n:
            ly += 16
            o.append(f'<text class="t2" x="{lx}" y="{ly}" font-size="11" fill="{LIGHT["text2"]}">{escape(n)}</text>')
    o.append("</svg>")
    Path(out).parent.mkdir(parents=True, exist_ok=True)
    Path(out).write_text("\n".join(o) + "\n")
    return out
