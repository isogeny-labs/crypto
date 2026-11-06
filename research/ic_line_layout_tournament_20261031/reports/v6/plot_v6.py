import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
A, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
def load(p): return json.load(open(p))
# session -> (run, index)
src = {"s1 (H1)": ("analysis-a", 1), "s2 (H1)": ("analysis-a", 2), "s3 (H2)": ("analysis-b", 1)}
rows = []
for name, (r, i) in src.items():
    w = load(f"{A}/{r}/session-{i}-candidate.json")["wall"]
    rows.append(dict(session=name, total=w["total_wall_ratio"], total_ci=w["total_wall_ci95"],
                     corrected=w["corrected_ratio"], corrected_ci=w["corrected_ci95"],
                     process=w["process_wall_ratio"], process_ci=w["process_wall_ci95"],
                     aa=w["aa_ratio"], aa_ci=w["aa_ci95"], pairs=w["pairs"]))
ops = {}
for wt in ["0", "0.1", "1"]:
    c = load(f"{A}/analysis-a/counted-compare-incumbent__candidate-w{wt}.json")["ops"]
    r = load(f"{A}/analysis-a/counted-compare-rho-neg__incumbent-w{wt}.json")["ops"]
    ops[wt] = dict(cand_over_inc=c["ratio_b_over_a"], cand_ci=c["ci95"], ic_over_rho=r["ratio_b_over_a"], ic_rho_ci=r["ci95"])
verdicts = {}
for r in ["analysis-a", "analysis-b"]:
    for s in load(f"{A}/{r}/verdict.json")["sessions"]:
        verdicts[(r, s["session"])] = s["verdict"]
for row, (r, i) in zip(rows, src.values()):
    row["verdict"] = verdicts[(r, i)]
json.dump(dict(sessions=rows, counted=ops, combined_verdict="invalid_measurement"), open(f"{out}/summary.json", "w"), indent=1)

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5.0), gridspec_kw=dict(width_ratios=[3, 2]))
y = list(range(len(rows)))[::-1]
for yy, row in zip(y, rows):
    for off, key, col, lab in [(0.18, "total", "#c0392b", "candidate / incumbent, total wall"),
                               (0.0, "process", "#e67e22", "candidate / incumbent, process wall"),
                               (-0.18, "aa", "#2c3e50", "A/A incumbent-aa / incumbent")]:
        v = row[key]; lo, hi = row[key + "_ci"]
        ax1.errorbar(v, yy + off, xerr=[[v - lo], [hi - v]], fmt="o", color=col, capsize=3, label=lab if yy == y[0] else None)
ax1.axvline(1.0, color="grey", lw=1, ls="--")
ax1.axvspan(0.9, 1.1, color="#95a5a6", alpha=0.12, label="A/A admission band [0.9, 1.1]")
ax1.set_yticks(y); ax1.set_yticklabels([f"{r['session']}\n{r['verdict']}" for r in rows])
ax1.set_xlabel("wall-time ratio (95% CI); > 1 means the candidate is slower")
ax1.set_title("Wall time: layout=lines vs map layout, fp28, c8a.2xlarge, L2")
ax1.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.2), ncol=2)
wts = list(ops)
ax2.plot(wts, [ops[w]["ic_over_rho"] for w in wts], "s-", color="#8e44ad", label="IC incumbent / rho-neg (counted S)")
ax2.plot(wts, [ops[w]["cand_over_inc"] for w in wts], "o-", color="#27ae60", label="candidate / incumbent (counted S)")
ax2.axhline(1.0, color="grey", lw=1, ls="--")
ax2.set_yscale("log"); ax2.set_xlabel("weight w on unpriced native work (GAE/op)")
ax2.set_ylabel("counted-cost ratio"); ax2.set_title("Counted cost: equal arms; gap to rho")
ax2.legend(fontsize=7)
fig.suptitle("IC-LINE-LAYOUT-E2E-FP28-V6: frozen verdict invalid_measurement (s1 A/A interval excludes 1)", fontsize=9)
fig.tight_layout()
for ext in ["svg", "png", "pdf"]:
    fig.savefig(f"{out}/v6-wall-and-counted.{ext}", dpi=200 if ext == "png" else None)
print(json.dumps(rows, indent=0)); print(json.dumps(ops))
