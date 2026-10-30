#!/usr/bin/env python3
"""Draw band_ratio.svg: IC / matched rho at w = 0, 0.1, 1 for the eight Table B prime rows
and the 20-bit fb-size row, read from committed evidence (no new numbers).

Left panel: Table B, legacy `ic boundary` unit (pinned calibration), values from
research/validation_sweep_20261019/A-leaderboard/verdicts.jsonl (recomputed.band_ratio_*).
Right panel: ecbench unit (C = 0), values from
benchmarks/20261029-prime-fb-size-20bit/VALIDATION.json (rho_reference.paired_ratio_sums).
The two panels are different units and different curves; they are not compared.
Stdlib only.  python3 plot_band.py  -> writes band_ratio.svg next to this file.
"""
from __future__ import annotations

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
SLUGS = ["icv1-fp10-t5-192cb216", "icv1-fp12-t19-d4a315df", "icv1-fp14-tm43-158b6914",
         "icv1-fp16-t351-d433f24e", "icv1-fp18-t175-45770ebc", "icv1-fp20-t1293-3c2999ba",
         "icv1-fp22-tm1267-a5acaf6d", "icv1-fp24-t1577-77336155"]
W = ["0", "0.1", "1"]
COL = {"0": "#2a6fb0", "0.1": "#d08a1e", "1": "#b03a3a"}


def table_b():
    out = {}
    for line in (REPO / "research/validation_sweep_20261019/A-leaderboard/verdicts.jsonl").read_text().splitlines():
        v = json.loads(line)
        slug = v.get("row_id", "").partition(":")[2]
        if slug in SLUGS:
            r = v["recomputed"]
            out[slug] = [r["band_ratio_w0"], r["band_ratio_w0_1"], r["band_ratio_w1"]]
    return [(s.split("-")[1], out[s]) for s in SLUGS]


def fb_size():
    m = json.loads((REPO / "benchmarks/20261029-prime-fb-size-20bit/VALIDATION.json").read_text())
    p = m["rho_reference"]["paired_ratio_sums"]
    return [("inc N=32", [p[w]["inc_over_rho"] for w in W]), ("sel N=45", [p[w]["sel_over_rho"] for w in W])]


def panel(x0, y0, w, h, rows, title, ymax):
    s = [f'<text x="{x0 + w / 2}" y="{y0 - 28}" text-anchor="middle" font-size="13" font-weight="bold">{title}</text>']
    s.append(f'<line x1="{x0}" y1="{y0 + h}" x2="{x0 + w}" y2="{y0 + h}" stroke="#444"/>')
    s.append(f'<line x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0 + h}" stroke="#444"/>')
    for t in range(0, int(ymax) + 1, 2 if ymax <= 20 else 5):
        y = y0 + h - h * t / ymax
        s.append(f'<line x1="{x0 - 4}" y1="{y:.1f}" x2="{x0 + w}" y2="{y:.1f}" stroke="#ddd"/>')
        s.append(f'<text x="{x0 - 7}" y="{y + 4:.1f}" text-anchor="end" font-size="10">{t}</text>')
    # rho parity line at 1
    y1 = y0 + h - h * 1 / ymax
    s.append(f'<line x1="{x0}" y1="{y1:.1f}" x2="{x0 + w}" y2="{y1:.1f}" stroke="#000" stroke-dasharray="4 3"/>')
    s.append(f'<text x="{x0 + w - 2}" y="{y1 - 4:.1f}" text-anchor="end" font-size="9">IC = rho</text>')
    gw = w / len(rows)
    bw = gw * 0.8 / 3
    for i, (label, vals) in enumerate(rows):
        gx = x0 + i * gw + gw * 0.1
        for j, v in enumerate(vals):
            bh = h * v / ymax
            s.append(f'<rect x="{gx + j * bw:.1f}" y="{y0 + h - bh:.1f}" width="{bw - 1:.1f}" height="{bh:.1f}" '
                     f'fill="{COL[W[j]]}"><title>{label} w={W[j]}: {v:.3f}</title></rect>')
        s.append(f'<text x="{gx + 1.5 * bw:.1f}" y="{y0 + h + 14}" text-anchor="middle" font-size="10">{label}</text>')
    s.append(f'<text transform="translate({x0 - 32},{y0 + h / 2}) rotate(-90)" text-anchor="middle" '
             f'font-size="11">IC / matched rho.negation</text>')
    return s


def main():
    width, height = 900, 380
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
         f'font-family="Helvetica, Arial, sans-serif"><rect width="100%" height="100%" fill="#fff"/>']
    s += panel(70, 70, 520, 240, table_b(), "Table B prime rows (legacy ic boundary unit)", 18)
    s += panel(680, 70, 180, 240, fb_size(), "20-bit fb-size (ecbench unit)", 18)
    for j, w in enumerate(W):
        s.append(f'<rect x="{70 + j * 120}" y="{height - 30}" width="12" height="12" fill="{COL[w]}"/>')
        s.append(f'<text x="{88 + j * 120}" y="{height - 20}" font-size="11">w = {w} GAE / native op</text>')
    s.append(f'<text x="{width - 10}" y="{height - 20}" text-anchor="end" font-size="9" fill="#555">'
             'recorded values; panels are different units and curves and are not compared</text>')
    s.append('</svg>')
    (HERE / "band_ratio.svg").write_text("\n".join(s) + "\n")


if __name__ == "__main__":
    main()
