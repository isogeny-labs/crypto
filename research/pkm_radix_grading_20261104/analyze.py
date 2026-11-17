#!/usr/bin/env python3
"""Frozen analysis for EXP-PKM-RADIX-001 (PROTOCOL.md section 5).

Reads runs/*.jsonl, applies the validity checks and the band rule, and writes
results.json plus two SVG graphs. Counted metrics only; no clock is read.

    python3 -I analyze.py [study_dir]
"""
import json
import math
import os
import sys

EXPECTED_G0 = {
    9: {"solving_degree_max": 5, "max_cols": 8644, "muladds": 45957046778},
    10: {"solving_degree_max": 6, "max_cols": 33446, "muladds": 481960954036},
}
BANDS = (0.5, 2.0)


def loadRows(runDir):
    rows = []
    for name in sorted(os.listdir(runDir)):
        if not name.endswith(".jsonl"):
            continue
        cell = name[: -len(".jsonl")]
        with open(os.path.join(runDir, name)) as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                r = json.loads(line)
                if "kind" not in r:
                    continue
                r["_cell"] = cell
                rows.append(r)
    return rows


def isCensored(r):
    return bool(r.get("timed_out")) or bool(r.get("oversize")) or r.get("pairs_above_bound", 0) > 0


def cellFamily(r):
    return r["_cell"].split("-")[0]


def radixOf(r):
    return int(r.get("radix", 2))


def checkValidity(rows):
    problems = []
    g0 = [r for r in rows if cellFamily(r) == "G0"]
    if not g0:
        problems.append("G0 missing")
    for r in g0:
        want = EXPECTED_G0.get(r["t"])
        if want is None:
            continue
        for key, val in want.items():
            if r.get(key) != val:
                problems.append("G0 t=%d draw %s: %s=%s, expected %s" % (r["t"], r["target_index"], key, r.get(key), val))
        if r.get("inconsistent") is not True:
            problems.append("G0 t=%d draw %s not a refutation" % (r["t"], r["target_index"]))
    # Graded rows carry weights; planted rows are solved.
    for r in rows:
        if radixOf(r) > 2 and not r.get("weights"):
            problems.append("%s: graded row without weights" % r["_cell"])
        if r.get("target") == "planted" and not isCensored(r):
            if r.get("planted_ok") is not True or r.get("inconsistent") is not False:
                problems.append("%s t=%d: planted row not solved" % (r["_cell"], r["t"]))
    # Every arm decides each system the same way.
    decided = {}
    for r in rows:
        if isCensored(r):
            continue
        key = (cellFamily(r), r["p"], r["m"], r["t"], r["control"], r["target"], r["target_index"])
        decided.setdefault(key, set()).add(r["inconsistent"])
    for key, vals in decided.items():
        if len(vals) > 1:
            problems.append("arms disagree on inconsistent for %s" % (key,))
    # A/A determinism.
    countKeys = ("muladds", "max_cols", "solving_degree_max", "steps", "inconsistent")
    a1 = {(r["control"], r["target"], r["target_index"]): r for r in rows if cellFamily(r) == "A1"}
    ref = {(r["control"], r["target"], r["target_index"]): r for r in rows
           if cellFamily(r) == "L2" and r["t"] == 8 and radixOf(r) == 4}
    if a1:
        for key, r in a1.items():
            other = ref.get(key)
            if other is None:
                problems.append("A1 %s has no L2 counterpart" % (key,))
                continue
            for c in countKeys:
                if r.get(c) != other.get(c):
                    problems.append("A1 %s differs on %s" % (key, c))
    return problems


def ratioTable(rows, family, m, control, p=None):
    """{t: {draw: {radix: muladds or None if censored}}} for refutation rows."""
    table = {}
    for r in rows:
        if cellFamily(r) != family or r["m"] != m or r["control"] != control or r["target"] != "random":
            continue
        if p is not None and r["p"] != p:
            continue
        val = None if isCensored(r) else r.get("muladds")
        table.setdefault(r["t"], {}).setdefault(r["target_index"], {})[radixOf(r)] = val
    return table


def verdictFor(table, radix):
    complete = []
    for t in sorted(table):
        draws = table[t]
        if draws and all(d.get(2) and d.get(4) and d.get(8) for d in draws.values()):
            complete.append(t)
    if not complete:
        return {"verdict": "no_completed_t", "t_max": None, "ratios": {}}
    ratios = {}
    for t in complete:
        ratios[t] = {str(k): table[t][k][radix] / table[t][k][2] for k in sorted(table[t])}
    tMax = complete[-1]
    atMax = list(ratios[tMax].values())
    lo, hi = BANDS
    if all(x <= lo for x in atMax):
        last = complete[-3:]
        nonIncreasing = len(last) == 3 and all(
            all(ratios[last[i + 1]][d] <= ratios[last[i]][d] for d in ratios[last[i]])
            for i in range(2)
        )
        verdict = "gain_growing" if nonIncreasing else "gain_constant"
    elif all(x >= hi for x in atMax):
        verdict = "loss"
    elif all(lo < x < hi for x in atMax):
        verdict = "no_material_effect"
    else:
        verdict = "mixed"
    return {"verdict": verdict, "t_max": tMax, "ratios": {str(t): ratios[t] for t in complete}}


def slopes(table, m):
    out = {}
    for radix in (2, 4, 8):
        xs, ys = [], []
        for t in sorted(table):
            vals = [d.get(radix) for d in table[t].values()]
            if vals and all(vals):
                xs.append(m * t)
                ys.append(math.log2(sum(vals) / len(vals)))
        if len(xs) >= 2:
            mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
            sxx = sum((x - mx) ** 2 for x in xs)
            out[str(radix)] = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    return out


def svgLines(series, title, xLabel, yLabel, path, logY=False, bands=None):
    width, height, pad = 640, 400, 60
    pts = [(x, y) for s in series.values() for x, y in s if y is not None and y > 0]
    if not pts:
        return
    tf = (lambda v: math.log2(v)) if logY else (lambda v: v)
    xMin, xMax = min(p[0] for p in pts), max(p[0] for p in pts)
    yVals = [tf(p[1]) for p in pts] + ([tf(b) for b in bands] if bands else [])
    yMin, yMax = min(yVals), max(yVals)
    if xMax == xMin:
        xMax += 1
    if yMax == yMin:
        yMax += 1

    def sx(x):
        return pad + (x - xMin) / (xMax - xMin) * (width - 2 * pad)

    def sy(y):
        return height - pad - (tf(y) - yMin) / (yMax - yMin) * (height - 2 * pad)

    colors = ["#2a6fdb", "#d9822b", "#3a9d5d", "#9b59b6", "#c0392b", "#7f8c8d"]
    parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="12">' % (width, height),
             '<rect width="100%" height="100%" fill="white"/>',
             '<text x="%d" y="24" font-size="14">%s</text>' % (pad, title),
             '<line x1="%d" y1="%d" x2="%d" y2="%d" stroke="#333"/>' % (pad, height - pad, width - pad, height - pad),
             '<line x1="%d" y1="%d" x2="%d" y2="%d" stroke="#333"/>' % (pad, pad, pad, height - pad),
             '<text x="%d" y="%d">%s</text>' % (width // 2 - 40, height - 15, xLabel),
             '<text x="12" y="%d" transform="rotate(-90 12 %d)">%s</text>' % (height // 2, height // 2, yLabel)]
    if bands:
        for b in bands:
            parts.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" stroke="#aaa" stroke-dasharray="4 4"/>' % (pad, sy(b), width - pad, sy(b)))
            parts.append('<text x="%d" y="%.1f" fill="#888">%g</text>' % (width - pad + 4, sy(b) + 4, b))
    xTicks = sorted(set(p[0] for p in pts))
    for x in xTicks:
        parts.append('<text x="%.1f" y="%d" text-anchor="middle">%g</text>' % (sx(x), height - pad + 16, x))
    for i, (name, s) in enumerate(series.items()):
        c = colors[i % len(colors)]
        good = [(x, y) for x, y in s if y is not None and y > 0]
        if len(good) > 1:
            parts.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (c, " ".join("%.1f,%.1f" % (sx(x), sy(y)) for x, y in good)))
        for x, y in good:
            parts.append('<circle cx="%.1f" cy="%.1f" r="3" fill="%s"/>' % (sx(x), sy(y), c))
        parts.append('<text x="%d" y="%d" fill="%s">%s</text>' % (width - pad - 150, pad + 16 * i, c, name))
    parts.append("</svg>")
    with open(path, "w") as fh:
        fh.write("\n".join(parts))


def main():
    studyDir = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.abspath(__file__))
    rows = loadRows(os.path.join(studyDir, "runs"))
    problems = checkValidity(rows)
    results = {"schema": "pkm-radix.results/v1", "rows": len(rows), "validity_problems": problems,
               "instrument_invalid": bool(problems), "verdicts": {}, "slopes_log2_muladds_per_N": {}}
    tables = {
        "m2_tower": ratioTable(rows, "L2", 2, "tower"),
        "m2_null": ratioTable(rows, "L2", 2, "null"),
        "m3_tower": ratioTable(rows, "L3", 3, "tower"),
        "m2_tower_p786433": ratioTable(rows, "X2", 2, "tower", p=786433),
    }
    for name, table in tables.items():
        m = 3 if name.startswith("m3") else 2
        for radix in (4, 8):
            results["verdicts"]["%s_r%d" % (name, radix)] = verdictFor(table, radix)
        results["slopes_log2_muladds_per_N"][name] = slopes(table, m)
    # Attribution and replication (section 5, items 3 and 4).
    for radix in (4, 8):
        tv = results["verdicts"]["m2_tower_r%d" % radix]["verdict"]
        nv = results["verdicts"]["m2_null_r%d" % radix]["verdict"]
        results["verdicts"]["attribution_r%d" % radix] = "shape" if tv == nv else "elliptic_specific_at_these_sizes"
        xv = results["verdicts"]["m2_tower_p786433_r%d" % radix]
        lv = results["verdicts"]["m2_tower_r%d" % radix]
        xr = xv["ratios"].get("10")
        lr = lv["ratios"].get("10")

        def band(x):
            return "gain" if x <= BANDS[0] else ("loss" if x >= BANDS[1] else "none")

        if xr and lr:
            same = set(band(v) for v in xr.values()) == set(band(v) for v in lr.values())
            results["verdicts"]["replication_r%d" % radix] = "replicated" if same else "not_replicated"
        else:
            results["verdicts"]["replication_r%d" % radix] = "not_available"
    with open(os.path.join(studyDir, "results.json"), "w") as fh:
        json.dump(results, fh, indent=2, sort_keys=True)
    # Graphs.
    ratioSeries = {}
    for name in ("m2_tower", "m2_null", "m3_tower"):
        for radix in (4, 8):
            v = results["verdicts"]["%s_r%d" % (name, radix)]["ratios"]
            m = 3 if name.startswith("m3") else 2
            ratioSeries["%s radix %d" % (name, radix)] = [
                (m * int(t), sum(d.values()) / len(d)) for t, d in sorted(v.items(), key=lambda kv: int(kv[0]))
            ]
    svgLines(ratioSeries, "muladds ratio vs radix 2 (dashed: decision bands)", "N = m*t", "ratio (log scale)",
             os.path.join(studyDir, "ratio_vs_t.svg"), logY=True, bands=list(BANDS))
    costSeries = {}
    for name in ("m2_tower", "m3_tower"):
        m = 3 if name.startswith("m3") else 2
        table = tables[name]
        for radix in (2, 4, 8):
            costSeries["%s radix %d" % (name, radix)] = [
                (m * t, (sum(v for v in (d.get(radix) for d in table[t].values()) if v) / max(1, len([1 for d in table[t].values() if d.get(radix)])))
                 if any(d.get(radix) for d in table[t].values()) else None)
                for t in sorted(table)
            ]
    svgLines(costSeries, "refutation muladds vs N", "N = m*t", "muladds (log2 scale)",
             os.path.join(studyDir, "muladds_vs_N.svg"), logY=True)
    print(json.dumps({"instrument_invalid": results["instrument_invalid"], "problems": problems[:10],
                      "verdicts": {k: (v["verdict"] if isinstance(v, dict) else v) for k, v in results["verdicts"].items()}}, indent=2))


if __name__ == "__main__":
    main()
