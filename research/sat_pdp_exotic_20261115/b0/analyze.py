#!/usr/bin/env python3
"""B0 analysis: rows/l*.jsonl -> ANALYSIS.json, table.md, figures/*.svg.

Standard library only. Every number is derived from the committed rows; the
script computes medians and reads the protocol's claims B1-B5 against their
intervals. It never imputes a censored solve.
"""
import json
import math
import os
import statistics as st
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROWS = os.path.join(HERE, "rows")
FIG = os.path.join(HERE, "figures")
ARMS = ["native", "native-aa", "cms", "cadical", "chain-p12", "lin", "lin-nolex"]


def load():
    """Rows per rung. Several files may hold the same rung (AM2: a partial first run
    kept beside the restarted run); rows with the same (l, idx) are merged by arm,
    the first file's record of an arm winning. Targets are seeded, so the same idx
    is the same target; the merge asserts that."""
    out = {}
    for fn in sorted(os.listdir(ROWS)):
        if not fn.endswith(".jsonl"):
            continue
        for x in open(os.path.join(ROWS, fn)):
            if not x.strip():
                continue
            r = json.loads(x)
            by_idx = out.setdefault(r["l"], {})
            if r["idx"] in by_idx:
                prev = by_idx[r["idx"]]
                assert prev["x_r"] == r["x_r"], f"target mismatch at l={r['l']} idx={r['idx']}"
                for arm, v in r["arms"].items():
                    prev["arms"].setdefault(arm, v)
            else:
                by_idx[r["idx"]] = r
    return {l: [by_idx[i] for i in sorted(by_idx)] for l, by_idx in out.items()}


def rung_summary(rows):
    l = rows[0]["l"]
    t_enum = rows[0]["t_enum_oracle"]
    p2 = 1 << l
    t3 = rows[0].get("t_enum_triples") or p2 * (p2 - 1) * (p2 - 2) // 6
    res = {"l": l, "n": rows[0]["n"], "t_enum": t_enum, "t_triples": t3, "arms": {}}
    for arm in ARMS:
        uni = [r for r in rows if r["kind"] == "uniform" and arm in r["arms"]]
        pla = [r for r in rows if r["kind"] == "planted" and arm in r["arms"]]
        if not uni and not pla:
            continue
        dec = [r for r in uni if r["arms"][arm]["status"] in ("sat", "unsat")]
        cens = len(uni) - len(dec)
        R = [r["arms"][arm]["conflicts"] / t_enum for r in dec]
        R3 = [r["arms"][arm]["conflicts"] / t3 for r in dec]
        unsat_R = [r["arms"][arm]["conflicts"] / t_enum for r in dec if not r["oracle"]["sat"]]
        disagree = sum(1 for r in uni + pla if r["arms"][arm]["agree"] is False)
        unverified = sum(1 for r in uni + pla if r["arms"][arm]["status"] == "sat" and r["arms"][arm].get("verified") is not True)
        planted_ok = sum(1 for r in pla if r["arms"][arm]["status"] == "sat" and r["arms"][arm].get("verified") is True)
        wall = [r["arms"][arm]["wall_ms"] for r in dec if "wall_ms" in r["arms"][arm]]
        a = {
            "uniform": len(uni), "decided": len(dec), "censored": cens,
            "median_R": st.median(R) if R else None, "min_R": min(R) if R else None, "max_R": max(R) if R else None,
            "median_R3": st.median(R3) if R3 else None,
            "min_R_on_refutations": min(unsat_R) if unsat_R else None,
            "refutations": len(unsat_R),
            "disagreements": disagree, "unverified_models": unverified,
            "planted_sat_verified": planted_ok, "planted": len(pla),
            "median_wall_ms": st.median(wall) if wall else None,
        }
        if arm == "lin":
            hist = {}
            for r in dec:
                for w, c in enumerate(r["arms"][arm].get("learnt_priority_width", [])):
                    hist[w] = hist.get(w, 0) + c
            a["learnt_priority_width_hist"] = {str(k): v for k, v in sorted(hist.items())}
        res["arms"][arm] = a
    if "native" in res["arms"] and "native-aa" in res["arms"]:
        spread = []
        for r in rows:
            if r["kind"] == "uniform" and not r["oracle"]["sat"] and "native" in r["arms"] and "native-aa" in r["arms"]:
                c0, c1 = r["arms"]["native"]["conflicts"], r["arms"]["native-aa"]["conflicts"]
                if c0:
                    spread.append(abs(c0 - c1) / c0)
        res["aa_relative_spread_median"] = st.median(spread) if spread else None
    return res


def claims(summ):
    out = {}
    def med(l, arm, key):
        a = summ[l]["arms"].get(arm)
        return a[key] if a and a.get(key) is not None else None
    ls = sorted(summ)
    b1 = all(med(l, "native", "median_R3") is not None and 0.9 <= med(l, "native", "median_R3") <= 1.3 for l in ls)
    out["B1"] = {"supported": b1, "native_median_R3_by_l": {str(l): med(l, "native", "median_R3") for l in ls}}
    b2a = all(med(l, "lin", "median_R") is not None and 1.1 <= med(l, "lin", "median_R") <= 1.4 for l in ls)
    ratio = {str(l): (med(l, "lin-nolex", "median_R") / med(l, "lin", "median_R")) if med(l, "lin", "median_R") and med(l, "lin-nolex", "median_R") else None for l in ls}
    gain = {str(l): (med(l, "native", "median_R") / med(l, "lin", "median_R")) if med(l, "lin", "median_R") and med(l, "native", "median_R") else None for l in ls}
    out["B2"] = {"supported": b2a, "lin_median_R_by_l": {str(l): med(l, "lin", "median_R") for l in ls}, "nolex_over_lex": ratio, "incumbent_over_lin_conflicts": gain, "two_to_l_over_3": {str(l): 2 ** l / 3 for l in ls}}
    mins = {}
    for l in ls:
        for arm in ARMS:
            m = med(l, arm, "min_R_on_refutations")
            if m is not None:
                mins[f"{arm}@l{l}"] = m
    out["B3"] = {"supported": all(v >= 1.0 for v in mins.values()) if mins else None, "min_R_on_refutations": mins}
    b4 = {}
    for l in ls:
        nat = med(l, "native", "median_R3")
        for arm, lo, hi in (("cms", 0.5, 5.0), ("cadical", 1.0, 3.0)):
            v = med(l, arm, "median_R3")
            b4[f"{arm}@l{l}"] = None if (v is None or nat is None) else (lo <= v / nat <= hi, v / nat)
    out["B4"] = b4
    c7 = med(7, "chain-p12", "median_R") if 7 in summ else None
    out["B5"] = {"supported": None if c7 is None else (100 <= c7 <= 300), "chain_p12_median_R_l7": c7}
    return out


def svg_lines(series, title, ylab, path, logy=True, refs=None):
    W, H, ML, MB, MT, MR = 720, 440, 70, 50, 40, 160
    xs = sorted({x for s in series.values() for x, _ in s})
    ys = [y for s in series.values() for _, y in s if y is not None and y > 0]
    if refs:
        ys += [y for _, pts in refs for _, y in pts]
    if not xs or not ys:
        return
    def fy(y):
        if logy:
            lo, hi = math.log10(min(ys)) - 0.2, math.log10(max(ys)) + 0.2
            return MT + (H - MT - MB) * (1 - (math.log10(y) - lo) / (hi - lo))
        lo, hi = 0, max(ys) * 1.1
        return MT + (H - MT - MB) * (1 - (y - lo) / (hi - lo))
    def fx(x):
        if len(xs) == 1:
            return ML + (W - ML - MR) / 2
        return ML + (W - ML - MR) * (x - xs[0]) / (xs[-1] - xs[0])
    colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f"]
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
         f'<rect width="{W}" height="{H}" fill="white"/>',
         f'<text x="{W/2}" y="22" text-anchor="middle" font-size="14">{title}</text>',
         f'<line x1="{ML}" y1="{H-MB}" x2="{W-MR}" y2="{H-MB}" stroke="black"/>',
         f'<line x1="{ML}" y1="{MT}" x2="{ML}" y2="{H-MB}" stroke="black"/>',
         f'<text x="{(ML+W-MR)/2}" y="{H-12}" text-anchor="middle">ℓ (factor-base dimension)</text>',
         f'<text x="16" y="{(MT+H-MB)/2}" text-anchor="middle" transform="rotate(-90 16 {(MT+H-MB)/2})">{ylab}</text>']
    for x in xs:
        o.append(f'<text x="{fx(x)}" y="{H-MB+16}" text-anchor="middle">{x}</text>')
    if logy:
        lo, hi = int(math.floor(math.log10(min(ys)))), int(math.ceil(math.log10(max(ys))))
        for e in range(lo, hi + 1):
            y = 10 ** e
            if min(ys) / 1.6 <= y <= max(ys) * 1.6:
                o.append(f'<line x1="{ML}" y1="{fy(y)}" x2="{W-MR}" y2="{fy(y)}" stroke="#ddd"/>')
                o.append(f'<text x="{ML-6}" y="{fy(y)+4}" text-anchor="end">{y:g}</text>')
    if refs:
        for name, pts in refs:
            d = " ".join(f"{'M' if i == 0 else 'L'}{fx(x)},{fy(y)}" for i, (x, y) in enumerate(pts))
            o.append(f'<path d="{d}" stroke="#555" stroke-dasharray="6 4" fill="none"/>')
            o.append(f'<text x="{fx(pts[-1][0])+6}" y="{fy(pts[-1][1])+4}" fill="#555">{name}</text>')
    for i, (name, pts) in enumerate(series.items()):
        pts = [(x, y) for x, y in pts if y is not None]
        if not pts:
            continue
        c = colors[i % len(colors)]
        d = " ".join(f"{'M' if j == 0 else 'L'}{fx(x)},{fy(y)}" for j, (x, y) in enumerate(pts))
        o.append(f'<path d="{d}" stroke="{c}" stroke-width="2" fill="none"/>')
        for x, y in pts:
            o.append(f'<circle cx="{fx(x)}" cy="{fy(y)}" r="4" fill="{c}"/>')
        o.append(f'<rect x="{W-MR+10}" y="{MT+18*i}" width="12" height="12" fill="{c}"/>')
        o.append(f'<text x="{W-MR+28}" y="{MT+18*i+10}">{name}</text>')
    o.append("</svg>")
    open(path, "w").write("\n".join(o))


def main():
    data = load()
    if not data:
        print("no rows", file=sys.stderr)
        return 1
    summ = {l: rung_summary(rows) for l, rows in data.items()}
    cl = claims(summ)
    os.makedirs(FIG, exist_ok=True)
    ls = sorted(summ)
    series_R = {arm: [(l, summ[l]["arms"].get(arm, {}).get("median_R")) for l in ls] for arm in ARMS if any(arm in summ[l]["arms"] for l in ls)}
    svg_lines(series_R, "Median R = conflicts / T_enum (oracle pairs) per arm", "R (log)", os.path.join(FIG, "median_R.svg"),
              refs=[("oracle, R = 1", [(ls[0], 1.0), (ls[-1], 1.0)]), ("2^ℓ/3 (one conflict per triple)", [(l, 2 ** l / 3) for l in ls])])
    series_R3 = {arm: [(l, summ[l]["arms"].get(arm, {}).get("median_R3")) for l in ls] for arm in ARMS if any(arm in summ[l]["arms"] for l in ls)}
    svg_lines(series_R3, "Median R₃ = conflicts / C(2^ℓ, 3) per arm", "R₃ (log)", os.path.join(FIG, "median_R3.svg"),
              refs=[("one conflict per triple", [(ls[0], 1.0), (ls[-1], 1.0)])])
    json.dump({"schema": "b0.analysis/v1", "rungs": {str(l): summ[l] for l in ls}, "claims": cl}, open(os.path.join(HERE, "ANALYSIS.json"), "w"), indent=1)
    lines = ["# B0 rung table (derived from rows/*.jsonl by analyze.py)", "",
             "| ℓ | n | T_enum | C(2^ℓ,3) | arm | decided/uniform | censored | median R | min R (refutations) | median R₃ | disagree | planted ok | median wall ms |",
             "|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|"]
    for l in ls:
        s = summ[l]
        for arm, a in s["arms"].items():
            f = lambda v, p=3: "—" if v is None else (f"{v:.{p}f}" if isinstance(v, float) else str(v))
            lines.append(f"| {l} | {s['n']} | {s['t_enum']} | {s['t_triples']} | {arm} | {a['decided']}/{a['uniform']} | {a['censored']} | {f(a['median_R'])} | {f(a['min_R_on_refutations'])} | {f(a['median_R3'],4)} | {a['disagreements']} | {a['planted_sat_verified']}/{a['planted']} | {f(a['median_wall_ms'],0)} |")
    lines += ["", "## Claims", "", "```", json.dumps(cl, indent=1), "```"]
    open(os.path.join(HERE, "table.md"), "w").write("\n".join(lines) + "\n")
    print("\n".join(lines[:3 + sum(len(s["arms"]) for s in summ.values())]))
    print(json.dumps(cl, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
