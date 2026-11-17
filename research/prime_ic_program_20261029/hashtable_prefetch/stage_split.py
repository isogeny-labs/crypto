#!/usr/bin/env python3
"""Per-trial stage split of the prime relation trial, from PR #2806's diagnosis.

Reads `research/ic_wide_ext_20261028/results.json` (branch lead/ic-wide-ext-20261028,
PR #2806) and writes `stage_split.csv` and `stage_split.svg` next to this script.
Stdlib only.  No new measurement: every number is copied from the input's
`diagnosis` block.  The loop-body stages are TSC cycle counts there; they are
converted at 2.1 GHz, the conversion the PR's README and report use.

Usage: python3 stage_split.py PATH/TO/results.json
"""
import csv
import hashlib
import json
import pathlib
import sys

TSC_GHZ = 2.1
HERE = pathlib.Path(__file__).resolve().parent
STAGES = [  # (column, label, colour)
    ("add", "additions + shared inversion", "#8da0cb"),
    ("prep", "lane preparation", "#b3b3b3"),
    ("queue", "queue fill", "#d9d9d9"),
    ("draw", "target draw", "#e5c494"),
    ("touch", "touch (demand reads)", "#fc8d62"),
    ("guard", "guard insert", "#e78ac3"),
    ("oracle", "oracle probe", "#d53e4f"),
    ("rest", "rest (row build, loop, timers)", "#f0f0f0"),
]
HASH_STAGES = {"touch", "guard", "oracle"}


def rows(results):
    out = []
    for bits, arms in sorted(results["diagnosis"].items(), key=lambda kv: int(kv[0])):
        for r in arms:
            if "add" not in r:
                out.append({"bits": int(bits), "arm": r["arm"], "total": r["rel_ns_per_trial"]})
                continue
            row = {
                "bits": int(bits),
                "arm": r["arm"],
                "total": r["rel_ns_per_trial"],
                "add": r["add"],
                "prep": r["prep"],
                "queue": r["queue"],
                "touch": r["touch"],
                "draw": r["draw_cycles"] / TSC_GHZ,
                "guard": r["guard_cycles"] / TSC_GHZ,
                "oracle": r["oracle_cycles"] / TSC_GHZ,
            }
            named = sum(row[k] for k, _, _ in STAGES if k != "rest")
            row["rest"] = row["total"] - named
            row["hash_table"] = row["touch"] + row["guard"] + row["oracle"]
            row["hash_share"] = row["hash_table"] / row["total"]
            out.append(row)
    return out


def write_csv(data, path):
    cols = ["bits", "arm", "total"] + [k for k, _, _ in STAGES] + ["hash_table", "hash_share"]
    with open(path, "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(cols)
        for r in data:
            w.writerow([r.get(c, "") if not isinstance(r.get(c), float) else f"{r[c]:.1f}"
                        if c != "hash_share" else f"{r[c]:.3f}" for c in cols])


def write_svg(data, path, source_note):
    lanes = [r for r in data if "add" in r]
    sizes = sorted({r["bits"] for r in lanes})
    bar_w, gap, left, top, h = 22, 6, 70, 40, 300
    per = max(len([r for r in lanes if r["bits"] == b]) for b in sizes)
    panel_w = per * (bar_w + gap) + 30
    width = left + len(sizes) * panel_w + 230
    height = top + h + 90
    vmax = 500.0
    y = lambda v: top + h - v / vmax * h
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
           f'font-family="sans-serif" font-size="11">',
           f'<rect width="{width}" height="{height}" fill="white"/>',
           f'<text x="{left}" y="18" font-size="13" font-weight="bold">Prime relation trial, '
           f'ns per trial by lane-round stage (measured, PR #2806 diagnosis)</text>']
    for t in range(0, 501, 100):
        out.append(f'<line x1="{left - 4}" x2="{left + len(sizes) * panel_w}" y1="{y(t):.1f}" '
                   f'y2="{y(t):.1f}" stroke="#eee"/>')
        out.append(f'<text x="{left - 8}" y="{y(t) + 4:.1f}" text-anchor="end">{t}</text>')
    out.append(f'<text transform="translate(18,{top + h / 2}) rotate(-90)" text-anchor="middle">'
               f'ns per relation trial</text>')
    for pi, b in enumerate(sizes):
        x0 = left + pi * panel_w + 10
        out.append(f'<text x="{x0}" y="{top + h + 42}" font-weight="bold">{b}-bit curve, base '
                   f'{512 if b == 28 else 1024}</text>')
        for i, r in enumerate([r for r in lanes if r["bits"] == b]):
            x = x0 + i * (bar_w + gap)
            acc = 0.0
            for k, label, colour in STAGES:
                v = max(r[k], 0.0)
                out.append(f'<rect x="{x}" y="{y(acc + v):.1f}" width="{bar_w}" '
                           f'height="{v / vmax * h:.1f}" fill="{colour}" stroke="#666" '
                           f'stroke-width="0.3"><title>{r["arm"]} {label}: {v:.1f} ns</title></rect>')
                acc += v
            out.append(f'<text x="{x + bar_w / 2}" y="{top + h + 14}" text-anchor="middle">'
                       f'{r["arm"][1:]}</text>')
        out.append(f'<text x="{x0}" y="{top + h + 28}" fill="#555">lanes</text>')
    lx = left + len(sizes) * panel_w + 10
    for i, (k, label, colour) in enumerate(reversed(STAGES)):
        yy = top + 10 + i * 18
        out.append(f'<rect x="{lx}" y="{yy - 10}" width="12" height="12" fill="{colour}" '
                   f'stroke="#666" stroke-width="0.3"/>')
        bold = ' font-weight="bold"' if k in HASH_STAGES else ""
        out.append(f'<text x="{lx + 18}" y="{yy}"{bold}>{label}</text>')
    out.append(f'<text x="{lx}" y="{top + 10 + len(STAGES) * 18 + 6}" fill="#555">bold: hash-table '
               f'work</text>')
    out.append(f'<text x="{left}" y="{height - 10}" fill="#555" font-size="9">{source_note}</text>')
    out.append("</svg>")
    path.write_text("\n".join(out) + "\n")


def main(src):
    raw = pathlib.Path(src).read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    data = rows(json.loads(raw))
    write_csv(data, HERE / "stage_split.csv")
    note = (f"source: research/ic_wide_ext_20261028/results.json @ lead/ic-wide-ext-20261028 "
            f"(sha256 {digest[:16]}...); one timer-instrumented session per size, descriptive; "
            f"cycles at {TSC_GHZ} GHz")
    write_svg(data, HERE / "stage_split.svg", note)
    print(f"sha256 {digest}")
    for r in data:
        if r["arm"] in ("b16", "b32"):
            print(f'{r["bits"]}-bit {r["arm"]}: total {r["total"]:.1f}, touch {r["touch"]:.1f}, '
                  f'guard {r["guard"]:.1f}, oracle {r["oracle"]:.1f}, guard+oracle '
                  f'{r["guard"] + r["oracle"]:.1f}, hash-table {r["hash_table"]:.1f} '
                  f'({100 * r["hash_share"]:.1f}%)')


if __name__ == "__main__":
    main(sys.argv[1])
