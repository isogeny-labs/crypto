#!/usr/bin/env python3
"""Aggregate raw_m2.json / raw_m3.json (+ optional extra raw files) into results.json and tables."""
import json, sys, math, statistics as st, glob, os

here = os.path.dirname(os.path.abspath(__file__))
rows = []
for f in sorted(glob.glob(os.path.join(here, "raw_*.json"))):
    for r in json.load(open(f)):
        r["source_file"] = os.path.basename(f)
        rows.append(r)


def fit(xs, ys):
    if len(xs) < 2:
        return None
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    den = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / den


METRICS = ["peak_live", "peak_work", "pool_initial", "max_single_constraint", "elim_calls"]
ext_rows = [r for r in rows if "cap24M" in r["source_file"]]
for r in ext_rows:
    r["off_protocol"] = "node cap raised 4e6 -> 24e6, 2 targets"
cells = {}
for r in rows:
    if r in ext_rows:
        continue
    cells.setdefault((r["m"], r["order"], r["sched"], r["l"]), []).append(r)

summary = []
for key in sorted(cells):
    m, o, s, l = key
    rs = cells[key]
    ok = [r for r in rs if r["status"] == "ok"]
    c = dict(m=m, order=o, sched=s, l=l, n=rs[0]["n"], runs=len(rs), ok=len(ok),
             blowups=len(rs) - len(ok),
             n_sat=sum(1 for r in ok if r["truth_sat"]), n_unsat=sum(1 for r in ok if not r["truth_sat"]),
             all_answers_match=all(r["answer_matches_truth"] for r in ok))
    for mt in METRICS:
        for tag, sel in (("", ok), ("_sat", [r for r in ok if r["truth_sat"]]),
                         ("_unsat", [r for r in ok if not r["truth_sat"]])):
            v = [r[mt] for r in sel]
            c["median_" + mt + tag] = st.median(v) if v else None
        v = [r[mt] for r in ok]
        c["min_" + mt] = min(v) if v else None
        c["max_" + mt] = max(v) if v else None
    c["median_time_s"] = st.median([r["time_s"] for r in ok]) if ok else None
    summary.append(c)

slopes = []
arms = sorted({(c["m"], c["order"], c["sched"]) for c in summary})
for (m, o, s) in arms:
    cs = [c for c in summary if (c["m"], c["order"], c["sched"]) == (m, o, s) and c["ok"] == c["runs"] and c["ok"] > 0]
    d = dict(m=m, order=o, sched=s, sizes=[c["l"] for c in cs], ns=[c["n"] for c in cs])
    for mt in METRICS:
        for tag in ("", "_sat", "_unsat"):
            pts = [(c["l"], math.log2(c["median_" + mt + tag])) for c in cs if c["median_" + mt + tag]]
            d["slope_" + mt + tag] = fit([p[0] for p in pts], [p[1] for p in pts]) if len(pts) >= 3 else None
    pts = [(c["l"], math.log2(c["median_peak_live"] / c["n"])) for c in cs]
    d["slope_peak_live_over_n"] = fit([p[0] for p in pts], [p[1] for p in pts]) if len(pts) >= 3 else None
    pts = [(c["l"], math.log2(c["median_peak_work"] / c["n"])) for c in cs]
    d["slope_peak_work_over_n"] = fit([p[0] for p in pts], [p[1] for p in pts]) if len(pts) >= 3 else None
    slopes.append(d)

json.dump(dict(note="pilot, unverified [pcc:unverified]", rows=rows, cell_summary=summary, slopes=slopes),
          open(os.path.join(here, "results.json"), "w"), indent=1)


def f2(x):
    return "-" if x is None else f"{x:.2f}"


for m in (2, 3):
    print(f"\n### m={m} cell medians (all targets; ok/runs)")
    print("| order | sched | l | n | ok/runs | sat/unsat | pool_initial | max_single | peak_work | peak_live | elim apply calls | 2^l | 2^(ceil(m/2)l) | 2^((m-1)l) | match |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for c in summary:
        if c["m"] != m:
            continue
        g = lambda k: "-" if c[k] is None else f"{c[k]:.0f}"
        print(f"| {c['order']} | {c['sched']} | {c['l']} | {c['n']} | {c['ok']}/{c['runs']} | {c['n_sat']}/{c['n_unsat']} | "
              f"{g('median_pool_initial')} | {g('median_max_single_constraint')} | {g('median_peak_work')} | "
              f"{g('median_peak_live')} | {g('median_elim_calls')} | {2**c['l']} | {2**(math.ceil(m/2)*c['l'])} | "
              f"{2**((m-1)*c['l'])} | {c['all_answers_match']} |")
    print(f"\n### m={m} fitted slopes (bits per l, least squares on log2 of cell median; only fully completed cells)")
    print("| order | sched | sizes l | peak_live | peak_live sat | peak_live unsat | peak_live/n | peak_work | peak_work/n | max_single | pool_initial | elim calls |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for d in slopes:
        if d["m"] != m:
            continue
        print(f"| {d['order']} | {d['sched']} | {d['sizes']} | {f2(d['slope_peak_live'])} | {f2(d['slope_peak_live_sat'])} | "
              f"{f2(d['slope_peak_live_unsat'])} | {f2(d['slope_peak_live_over_n'])} | {f2(d['slope_peak_work'])} | "
              f"{f2(d['slope_peak_work_over_n'])} | {f2(d['slope_max_single_constraint'])} | {f2(d['slope_pool_initial'])} | "
              f"{f2(d['slope_elim_calls'])} |")

# successive-size local slopes for the best m=3 arm
print("\n### blow-ups")
for c in summary:
    if c["blowups"]:
        rs = [r for r in cells[(c['m'], c['order'], c['sched'], c['l'])] if r['status'] != 'ok']
        print(f"m={c['m']} {c['order']} {c['sched']} l={c['l']}: {c['blowups']}/{c['runs']} ({sorted(set(r['status']+':'+r.get('reason','') for r in rs))})")

# off-protocol l=7 extension (node cap 24e6): extended fit for xint_t arms
print("\n### off-protocol extension: m=3 l=7 n=23, order xint_t, node cap 24e6, 2 targets")
ext = {}
for r in ext_rows:
    if r["status"] == "ok":
        ext.setdefault((r["order"], r["sched"]), []).append(r)
extfits = []
for (o, s_), rs in sorted(ext.items()):
    med = {mt: st.median([r[mt] for r in rs]) for mt in METRICS}
    print(f"{o} {s_} l=7: runs={len(rs)} " + " ".join(f"{k}={v:.0f}" for k, v in med.items()),
          "match=", all(r["answer_matches_truth"] for r in rs))
    cs = [c for c in summary if (c["m"], c["order"], c["sched"]) == (3, o, s_) and c["ok"] == c["runs"] and c["ok"] > 0]
    d = dict(m=3, order=o, sched=s_, sizes=[c["l"] for c in cs] + [7], off_protocol=True)
    for mt in METRICS:
        xs = [c["l"] for c in cs] + [7]
        ys = [math.log2(c["median_" + mt]) for c in cs] + [math.log2(med[mt])]
        d["slope_" + mt] = fit(xs, ys)
        d["local_slopes_" + mt] = [ys[i + 1] - ys[i] for i in range(len(ys) - 1)]
    extfits.append(d)
    print("   extended slopes l=%s: " % d["sizes"] + " ".join(f"{mt}={d['slope_'+mt]:.2f}" for mt in METRICS))
    print("   local (successive-l) slopes peak_live:", [round(x, 2) for x in d["local_slopes_peak_live"]],
          " peak_work:", [round(x, 2) for x in d["local_slopes_peak_work"]],
          " max_single:", [round(x, 2) for x in d["local_slopes_max_single_constraint"]])
res = json.load(open(os.path.join(here, "results.json")))
res["off_protocol_extended_fits"] = extfits
json.dump(res, open(os.path.join(here, "results.json"), "w"), indent=1)

try:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    fig, axs = plt.subplots(1, 2, figsize=(11, 4.5))
    for ax, m in zip(axs, (2, 3)):
        for (mm, o, s_) in arms:
            if mm != m:
                continue
            cs = [c for c in summary if (c["m"], c["order"], c["sched"]) == (m, o, s_) and c["ok"] > 0 and c["ok"] == c["runs"]]
            if cs:
                ax.plot([c["l"] for c in cs], [math.log2(c["median_peak_live"]) for c in cs], marker="o", label=f"{o}/{s_}")
        ls = sorted({c["l"] for c in summary if c["m"] == m})
        ax.plot(ls, [math.ceil(m / 2) * l for l in ls], "k--", label=f"MITM memory 2^({math.ceil(m/2)}l)")
        ax.plot(ls, [l for l in ls], "k:", label="linear scan 2^l")
        ax.set_xlabel("l (factor-base dimension)"); ax.set_ylabel("log2 median peak live BDD nodes")
        ax.set_title(f"m={m} (pilot, unverified)"); ax.legend(fontsize=6); ax.grid(alpha=.3)
    fig.tight_layout(); fig.savefig(os.path.join(here, "peak_nodes.svg")); fig.savefig(os.path.join(here, "peak_nodes.png"), dpi=110)
    print("plot written")
except Exception as e:
    print("no plot:", e)
