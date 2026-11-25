#!/usr/bin/env python3
"""Analysis per PROTOCOL.md §6–7. Reads raw/m2.jsonl, raw/m3.jsonl, raw/posctl.jsonl;
writes summary.csv, slopes.csv, ANALYSIS.json, figures/dF_vs_n.svg. Stdlib only."""
import json, math, random, statistics, csv, os
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
P = lambda *a: os.path.join(HERE, *a)
ARMS = ["xchain-rand", "xchain-win", "xdirect-rand", "sym-rand", "sym-rand2", "sym-win"]


def load(name):
    p = P("raw", name)
    return [json.loads(l) for l in open(p)] if os.path.exists(p) else []


def last_c(r):
    per = r.get("per_degree") or {}
    return max([int(k) for k in per] or [1])


def dval(r, extra):
    """d_F, or censored value: last attempted (or refused) degree + extra."""
    if r["d_F"] is not None:
        return r["d_F"], False
    c = r["censored"] or ""
    # 'cols>N at c=K' means K was not attempted -> d_F > K-1
    if "cols>" in c:
        k = int(c.split("c=")[1]) - 1
    elif "budget" in c:
        k = int(c.split("c=")[1]) - 1  # degree c did not finish
    else:
        k = int(c.split("cmax=")[1])
    return k + extra, True


def log2cols(nv, d):
    return math.log2(sum(math.comb(nv, i) for i in range(d + 1)))


def ols(xs, ys):
    mx, my = statistics.fmean(xs), statistics.fmean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx


def boot_slopes(rows, arm_a, arm_b, extra, metric, B=2000, seed=20261123):
    """Bootstrap draws within cells; paired by draw (same draw resampled for both arms)."""
    by = defaultdict(dict)  # (n,draw) -> arm -> value
    for r in rows:
        v, _ = dval(r, extra)
        if metric == "L":
            v = log2cols(r["n_vars"], v)
        by[(r["n"], r["draw"])][r["arm"]] = v
    cells = defaultdict(list)
    for (n, d), a in by.items():
        if arm_a in a and (arm_b is None or arm_b in a):
            cells[n].append(a)
    ns = sorted(cells)
    if len(ns) < 3:
        return None

    def stat(sample):
        xs, ya, yb = [], [], []
        for n, lst in sample:
            for a in lst:
                xs.append(n)
                ya.append(a[arm_a])
                if arm_b:
                    yb.append(a[arm_b])
        sa = ols(xs, ya)
        return sa - ols(xs, yb) if arm_b else sa

    point = stat([(n, cells[n]) for n in ns])
    rng = random.Random(seed)
    bs = sorted(
        stat([(n, [rng.choice(cells[n]) for _ in cells[n]]) for n in ns]) for _ in range(B)
    )
    return point, bs[int(0.025 * B)], bs[int(0.975 * B) - 1]


def main():
    os.makedirs(P("figures"), exist_ok=True)
    out = {"standing": "[pcc:unverified] L0 local container"}
    pos = load("posctl.jsonl")
    pc = defaultdict(list)
    for r in pos:
        pc[(r["k"], r["e"], r["semireg_dreg"])].append(r["d_F"])
    meds = [(k, statistics.median(v)) for k, v in sorted(pc.items(), key=lambda x: x[0][2])]
    out["positive_control"] = {
        "rows": [{"k": k[0], "e": k[1], "semireg": k[2], "median_dF": m} for k, m in meds],
        "pass": all(meds[i][1] < meds[i + 1][1] for i in range(len(meds) - 1))
        and all(abs(m - k[2]) <= 1 for k, m in meds),
    }
    # G0 fidelity gate (PROTOCOL.md section 2): preregistered = 72/72 identical incl. plain dims.
    rep = open(P("raw", "replay.out")).read().splitlines() if os.path.exists(P("raw", "replay.out")) else []
    last = [l for l in rep if l.startswith("replay:")]
    nm = int(last[0].split()[1]) if last else None
    nmis = int(last[0].split()[3]) if last else None
    out["G0"] = {
        "replay_match": nm, "replay_mismatch": nmis,
        "preregistered_pass": nmis == 0,
        "relaxed_pass_dF_and_closed": nmis is not None and all(
            l.split(" got ")[0].split("want ")[1].split(" ")[0] == l.split(" got ")[1].split(" ")[0]
            and [t.split(",")[2] for t in l.split(" got ")[0].split("[")[1].split("(")[1:]]
            == [t.split(",")[2] for t in l.split(" got ")[1].split("[")[1].split("(")[1:]]
            for l in rep if l.startswith("MISMATCH")),
        "note": "G0 failed as preregistered (62/72); relaxed post hoc to d_F + closed dims only, "
                "after posctl and m=2 data existed (README D1)",
    }
    summ = []
    for m, fn in [(2, "m2.jsonl"), (3, "m3.jsonl")]:
        rows = load(fn) if fn != "m3.jsonl" else [r for k in (9, 12, 15, 18, 21) for r in load(f"m3_n{k}.jsonl")]
        if not rows:
            continue
        # only complete draws (all six arms present); a draw cut by the wall cap is dropped
        arms_of = defaultdict(set)
        for r in rows:
            arms_of[(r["n"], r["draw"])].add(r["arm"])
        dropped = sorted(k for k, a in arms_of.items() if len(a) < len(ARMS))
        out.setdefault("dropped_incomplete_draws", {})[m] = dropped
        rows = [r for r in rows if len(arms_of[(r["n"], r["draw"])]) == len(ARMS)]
        cell = defaultdict(list)
        for r in rows:
            cell[(r["n"], r["arm"])].append(r)
        for (n, arm), rs in sorted(cell.items()):
            ds = [dval(r, 1) for r in rs]
            vals = [d for d, _ in ds]
            cens = sum(c for _, c in ds)
            med = statistics.median(vals)
            summ.append({
                "m": m, "n": n, "ell": rs[0]["ell"], "arm": arm, "draws": len(rs),
                "n_vars": rs[0]["n_vars"], "input_degree": rs[0]["input_degree"],
                "semireg_dreg_median": statistics.median(r["semireg_dreg"] for r in rs),
                "dF_values": "/".join(str(v) + ("+" if c else "") for v, c in ds),
                "dF_median": med, "censored": cens,
                "excess_median": med - rs[0]["input_degree"],
                "L_median": round(statistics.median(log2cols(r["n_vars"], d) for r, (d, _) in zip(rs, ds)), 2),
                "secs_median": round(statistics.median(r["secs"] for r in rs), 3),
            })
        sl = []
        for arm in ARMS:
            for extra in (1, 3):
                for metric in ("dF", "L"):
                    b = boot_slopes(rows, arm, None, extra, metric)
                    if b:
                        sl.append({"m": m, "arm": arm, "vs": "", "metric": metric, "cens_extra": extra,
                                   "slope": round(b[0], 4), "lo": round(b[1], 4), "hi": round(b[2], 4)})
        for a, bb in [("sym-rand", "xchain-rand"), ("sym-rand", "xdirect-rand"), ("sym-rand", "sym-rand2"),
                      ("sym-win", "xchain-win"), ("xchain-win", "xchain-rand")]:
            for extra in (1, 3):
                for metric in ("dF", "L"):
                    b = boot_slopes(rows, a, bb, extra, metric)
                    if b:
                        sl.append({"m": m, "arm": a, "vs": bb, "metric": metric, "cens_extra": extra,
                                   "slope": round(b[0], 4), "lo": round(b[1], 4), "hi": round(b[2], 4)})
        out.setdefault("slopes", []).extend(sl)
    out["summary"] = summ
    with open(P("summary.csv"), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(summ[0].keys()))
        w.writeheader(); w.writerows(summ)
    with open(P("slopes.csv"), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(out["slopes"][0].keys()))
        w.writeheader(); w.writerows(out["slopes"])
    # verdicts (m = 3 primary)
    def get(arm, vs, metric, extra, m=3):
        for s in out["slopes"]:
            if (s["m"], s["arm"], s["vs"], s["metric"], s["cens_extra"]) == (m, arm, vs, metric, extra):
                return s
    v = {}
    h1 = []
    for extra in (1, 3):
        a = get("sym-rand", "xchain-rand", "dF", extra); b = get("sym-rand", "xdirect-rand", "dF", extra)
        h1.append(bool(a and b and a["hi"] < 0 and b["hi"] < 0))
    aa = get("sym-rand", "sym-rand2", "dF", 1)
    m3 = [s for s in summ if s["m"] == 3]
    med = {(s["n"], s["arm"]): s for s in m3}
    aa_cells = all(abs(med[(n, "sym-rand")]["dF_median"] - med[(n, "sym-rand2")]["dF_median"]) <= 1
                   for n, a in med if a == "sym-rand" and (n, "sym-rand2") in med)
    v["A/A_holds"] = bool(aa and aa["lo"] <= 0 <= aa["hi"] and aa_cells)
    v["H1_supported"] = all(h1) and v["A/A_holds"] and out["positive_control"]["pass"]
    v["H2_sym_dF_eq_4_all_cells"] = all(med[(n, a)]["dF_median"] == 4 for n, a in med if a == "sym-rand") if m3 else None
    v["H3_L_sym_lt_chain"] = all(
        med[(n, "sym-rand")]["L_median"] < med[(n, "xchain-rand")]["L_median"]
        for n, a in med if a == "sym-rand" and (n, "xchain-rand") in med
        and med[(n, "xchain-rand")]["censored"] == 0 and med[(n, "sym-rand")]["censored"] == 0) if m3 else None
    # A/A informativeness (red-team review): do sym closure dims depend on V at all?
    aa_dims = {}
    for n in (9, 12, 15, 18):
        rs = [r for r in load(f"m3_n{n}.jsonl") if r["arm"] in ("sym-rand", "sym-rand2")]
        aa_dims[n] = len({json.dumps({k: (d["plain"], d["closed"]) for k, d in r["per_degree"].items()},
                                     sort_keys=True) for r in rs})
    v["A/A_informative"] = any(c > 1 for c in aa_dims.values())
    v["A/A_note"] = ("sym-frame closure dims are independent of V at m=3 (one distinct (plain, closed) "
                     "profile per cell across sym-rand/sym-rand2, draws and x_R), so the A/A arm cannot "
                     "measure noise; A/A_holds is formal only")
    out["A/A_distinct_sym_dim_profiles_per_cell_m3"] = aa_dims
    out["verdicts"] = v
    json.dump(out, open(P("ANALYSIS.json"), "w"), indent=1)
    svg(summ)
    print(json.dumps(v, indent=1))
    print(json.dumps(out["positive_control"], indent=1))


COL = {"xchain-rand": "#1f5fbf", "xchain-win": "#6fa0e8", "xdirect-rand": "#7a7a7a",
       "sym-rand": "#c0392b", "sym-rand2": "#e8907f", "sym-win": "#8e44ad"}


def svg(summ):
    W, H = 980, 430
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
             f'<rect width="{W}" height="{H}" fill="white"/>']
    panels = [(3, "dF_median", "m = 3: median last fall degree d_F", 0, 9),
              (3, "L_median", "m = 3: log2 Macaulay columns at d_F", 0, 24)]
    for pi, (m, key, title, y0, y1) in enumerate(panels):
        ox, oy, pw, ph = 60 + pi * 470, 40, 380, 300
        parts.append(f'<text x="{ox}" y="{oy-14}" font-weight="bold">{title}</text>')
        parts.append(f'<rect x="{ox}" y="{oy}" width="{pw}" height="{ph}" fill="none" stroke="#444"/>')
        X = lambda n: ox + (n - 8) / (22 - 8) * pw
        Y = lambda v: oy + ph - (v - y0) / (y1 - y0) * ph
        for n in (9, 12, 15, 18, 21):
            parts.append(f'<text x="{X(n)}" y="{oy+ph+16}" text-anchor="middle">{n}</text>')
        step = 1 if key == "dF_median" else 4
        for v in range(y0, y1 + 1, step):
            parts.append(f'<line x1="{ox}" x2="{ox+pw}" y1="{Y(v)}" y2="{Y(v)}" stroke="#eee"/>')
            parts.append(f'<text x="{ox-6}" y="{Y(v)+4}" text-anchor="end">{v}</text>')
        parts.append(f'<text x="{ox+pw/2}" y="{oy+ph+34}" text-anchor="middle">n (ℓ = n/3)</text>')
        for ai, arm in enumerate(ARMS):
            pts = sorted((s["n"], s[key], s["censored"]) for s in summ if s["m"] == m and s["arm"] == arm)
            if not pts:
                continue
            off = (ai - 2.5) * 2.2
            path = " ".join(f'{"M" if i == 0 else "L"}{X(n)+off:.1f},{Y(v):.1f}' for i, (n, v, c) in enumerate(pts))
            dash = ' stroke-dasharray="5,3"' if arm in ("sym-rand2", "xchain-win", "sym-win") else ""
            parts.append(f'<path d="{path}" fill="none" stroke="{COL[arm]}" stroke-width="2"{dash}/>')
            for n, v, c in pts:
                fill = "white" if c else COL[arm]
                parts.append(f'<circle cx="{X(n)+off:.1f}" cy="{Y(v):.1f}" r="3.5" fill="{fill}" stroke="{COL[arm]}" stroke-width="1.5"/>')
    # exploratory sym-only rows (D3): diamonds, not part of any verdict
    ex = [r for fn in ("m3_sym_explore.jsonl", "m3_sym_explore_n24.jsonl") for r in load(fn) if r["arm"] == "sym-rand"]
    for key_i, (key, y0, y1) in enumerate([("dF", 0, 9), ("L", 0, 24)]):
        ox, oy, pw, ph = 60 + key_i * 470, 40, 380, 300
        X = lambda n: ox + (n - 8) / (22 - 8) * pw
        Y = lambda v: oy + ph - (v - y0) / (y1 - y0) * ph
        for n in sorted({r["n"] for r in ex}):
            if n > 22:
                continue
            rs = [r for r in ex if r["n"] == n and r["d_F"] is not None]
            if not rs:
                continue
            d = statistics.median(r["d_F"] for r in rs)
            v = d if key == "dF" else log2cols(rs[0]["n_vars"], int(d))
            cx, cy = X(n), Y(v)
            parts.append(f'<path d="M{cx},{cy-6} L{cx+6},{cy} L{cx},{cy+6} L{cx-6},{cy} Z" fill="none" stroke="{COL["sym-rand"]}" stroke-width="1.5"/>')
            parts.append(f'<text x="{cx-8}" y="{cy-9}" font-size="10" fill="{COL["sym-rand"]}" text-anchor="end">sym, exploratory</text>')
    lx = 60
    for arm in ARMS:
        parts.append(f'<rect x="{lx}" y="{H-38}" width="14" height="4" fill="{COL[arm]}"/><text x="{lx+18}" y="{H-33}">{arm}</text>')
        lx += 150
    parts.append(f'<text x="60" y="{H-12}" fill="#555">Open circle = censored (value plotted is a lower bound). [pcc:unverified] L0, 4 draws per cell, seed 20261123.</text>')
    parts.append("</svg>")
    open(P("figures", "dF_vs_n.svg"), "w").write("\n".join(parts))


if __name__ == "__main__":
    main()
