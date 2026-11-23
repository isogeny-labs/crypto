"""Mechanical readout of raw/grid.jsonl against PROTOCOL.md: results.json, results.md, figures/k2_decisive.{svg,png}.
Usage: analyze.py [raw/grid.jsonl]"""
import json
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
raw = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "raw" / "grid.jsonl"
rows = [json.loads(l) for l in raw.read_text().splitlines() if l.strip()]

DECISIVE = {"D1", "D2", "D3"}
sys_rows = defaultdict(list)
for r in rows:
    sys_rows[(r["cell"], r["m"], r["n"], r["k"], r["seed"], r.get("target", 0))].append(r)

per = []
for (cell, m, n, k, seed, t), rs in sorted(sys_rows.items()):
    gate = [r for r in rs if "skipped" in r and str(r["skipped"]).startswith("gate")]
    done = [r for r in rs if "solved" in r]
    rec = {"cell": cell, "m": m, "n": n, "k": k, "seed": seed, "target": t}
    if gate and not done:
        rec.update(status="not run", reason=gate[0]["skipped"])
        per.append(rec)
        continue
    if not done:
        sk = [r for r in rs if "skipped" in r]
        rec.update(status="not run", reason=sk[0]["skipped"] if sk else "no row")
        per.append(rec)
        continue
    r4 = next((r for r in done if r["D"] == 4), None)
    first = next((r["D"] for r in sorted(done, key=lambda r: r["D"]) if r["solved"]), None)
    d = done[-1]
    rec.update(N=d["N"], eqs=d["eqs"], nsol=d["nsol"], cols=d["cols"], dsol=first)
    if r4 is None:
        rec.update(status="not reached", reason="D=4 not run")
    elif r4.get("censored"):
        rec.update(status="censored", reason="time cap reached before the closure finished (rank is a lower bound)",
                   rank=r4["rank"], target_rank=r4["target_rank"], secs=r4["secs"], peak_rss_kb=r4.get("peak_rss_kb"))
    else:
        rec.update(status="solved at 4" if r4["solved"] else "D=4 stall", rank=r4["rank"], target_rank=r4["target_rank"],
                   deficit=r4["deficit"], secs=r4["secs"], early_one=r4.get("early_one"), batches=r4.get("batches"),
                   products=r4.get("products"), t_reduce=r4.get("t_reduce"), t_inner=r4.get("t_inner"),
                   t_backsub=r4.get("t_backsub"), peak_rss_kb=r4.get("peak_rss_kb"), matrix_bytes=r4.get("matrix_bytes"),
                   dim_by_deg=r4.get("dim_by_deg"))
    per.append(rec)

ctrl_p = all(p.get("dsol") is not None and p["dsol"] <= 4 for p in per if p["cell"] == "P" and p["status"] != "not run")
ctrl_n = all(p.get("deficit", 0) > 100 and p["status"] in ("D=4 stall", "censored") for p in per if p["cell"].startswith("N") and p["status"] != "not run")
ctrl_n = ctrl_n and any(p["cell"].startswith("N") for p in per)

cells = defaultdict(list)
for p in per:
    cells[p["cell"]].append(p)


def verdict(cell):
    ps = [p for p in cells.get(cell, []) if p["status"] != "not run"]
    if not ps:
        return "not reached (not run)"
    comp = [p for p in ps if p["status"] in ("solved at 4", "D=4 stall")]
    if not comp:
        return "not reached (censored)"
    ok = sum(1 for p in comp if p["status"] == "solved at 4")
    if ok == len(comp):
        return "flat"
    if ok / len(comp) <= 0.5:
        return "breaks"
    return "inconclusive"


vd = {c: verdict(c) for c in ("D1", "D2", "D3")}
vl = {c: verdict(c) for c in ("L1", "L2")}
if any(v == "breaks" for v in vd.values()):
    overall = "breaks"
elif all(v == "flat" for v in vd.values()):
    overall = "flat"
elif any(v == "flat" for v in vd.values()):
    overall = "flat where reached; other decisive cells not reached"
else:
    overall = "not reached"
summary = {"controls": {"positive_pass": ctrl_p, "negative_pass": ctrl_n}, "decisive": vd, "ladder": vl, "overall": overall, "systems": per}
(HERE / "results.json").write_text(json.dumps(summary, indent=1))

L = ["| cell | m | n | k | N | cols (D=4) | target | status | D_sol | deficit | s | peak RSS (GB) | matrix (GB) |", "|:--|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|"]
for p in per:
    if p["status"] == "not run":
        L.append(f"| {p['cell']} | {p['m']} | {p['n']} | {p['k']} | - | - | {p['target']} | not run | - | - | - | - | - |")
        continue
    gb = lambda x: f"{x/1e9:.2f}" if x else "-"
    L.append(f"| {p['cell']} | {p['m']} | {p['n']} | {p['k']} | {p['N']} | {p['cols']} | {p['target']} | {p['status']} | "
             f"{p['dsol'] if p.get('dsol') else '>4' if p['status'] in ('D=4 stall','censored') else '-'} | {p.get('deficit','-')} | "
             f"{p.get('secs','-')} | {gb((p.get('peak_rss_kb') or 0)*1024)} | {gb(p.get('matrix_bytes'))} |")
L.append("")
L.append(f"Controls: positive {'PASS' if ctrl_p else 'FAIL'}, negative {'PASS' if ctrl_n else 'FAIL'}. "
         f"Decisive cells: {vd}. Ladder: {vl}. Overall: **{overall}**.")
(HERE / "results.md").write_text("\n".join(L) + "\n")
print("\n".join(L))

# figure: wall time vs columns of every completed D=4 closure, original dense instrument vs this one
try:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    orig = []
    op = HERE.parent / "k2_flatness_20261115" / "raw" / "grid.jsonl"
    if op.exists():
        for l in op.read_text().splitlines():
            r = json.loads(l)
            if r.get("D") == 4 and "secs" in r and r["cell"] in ("A", "B", "C") and r.get("solved"):
                orig.append((r["cols"], r["secs"]))
    fig, ax = plt.subplots(figsize=(6.4, 4.2))
    if orig:
        ax.scatter(*zip(*orig), s=14, c="#888", label="dense closure (k2_flatness_20261115)")
    mine = [(p["cols"], p["secs"], p["cell"], p["status"]) for p in per if p.get("secs") and p["cell"] in ("L1", "L2", "D1", "D2", "D3") and p["status"] != "censored"]
    if mine:
        ax.scatter([a for a, *_ in mine], [b for _, b, *_ in mine], s=36, c="#1f6fb4", label="blocked closure, 4 threads (this study)")
        for a, b, c, _ in mine:
            ax.annotate(c, (a, b), textcoords="offset points", xytext=(4, 4), fontsize=8)
    ax.set_xscale("log"); ax.set_yscale("log")
    ax.set_xlabel("columns M(N, <= 4)"); ax.set_ylabel("wall seconds, degree-4 closure")
    ax.set_title("K2 decisive cells: closure cost vs matrix size [pcc:unverified]", fontsize=9)
    ax.legend(fontsize=7)
    (HERE / "figures").mkdir(exist_ok=True)
    fig.tight_layout()
    fig.savefig(HERE / "figures" / "k2_decisive.svg")
    fig.savefig(HERE / "figures" / "k2_decisive.png", dpi=140)
except Exception as e:  # figure is best effort
    print("figure failed:", e)
