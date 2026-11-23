"""Figure for report.md: gap to matched strong rho vs n for pair-table fixed l = 7 and l*(n), same targets,
w = 0, 0.1, 1, with OLS lines and 95% CIs from results.json."""
import json, pathlib
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
HERE = pathlib.Path(__file__).resolve().parents[1]
res = json.loads((HERE / "results.json").read_text()); U = res["units"]; LS = {int(k): v for k, v in res["l_star"].items()}
NS = [17, 19, 23, 29]
fig, axes = plt.subplots(1, 3, figsize=(12.5, 4.2), sharey=True)
for ax, w in zip(axes, ("0", "0.1", "1")):
    d = U[f"band@{w}"]
    for tag, lab, col, mk, off in (("l7", "fixed l = 7", "#8c8c8c", "s", -0.15), ("lstar", "l*(n) = 7, 7, 9, 10", "#08519c", "o", 0.15)):
        ys = [d[f"gap_{tag}_n{n}"] for n in NS]; cis = [d[f"gap_{tag}_n{n}_ci"] for n in NS]
        ax.errorbar([n + off for n in NS], ys, yerr=[[y - c[0] for y, c in zip(ys, cis)], [c[1] - y for y, c in zip(ys, cis)]], fmt=mk, color=col, capsize=2,
                    label=f"{lab}: slope {d['slope_' + tag]:+.3f}")
        mx = sum(NS) / 4; my = sum(ys) / 4; b = d["slope_" + tag]
        ax.plot([16.5, 29.5], [my + b * (16.5 - mx), my + b * (29.5 - mx)], "-", color=col, lw=1, alpha=0.7)
    for n in (23, 29): ax.annotate(f"l={LS[n]}", (n + 0.15, d[f"gap_lstar_n{n}"]), xytext=(4, -10), textcoords="offset points", fontsize=8, color="#08519c")
    ax.set_title(f"w = {w}   D = {d['D']:+.3f} [{d['D_ci'][0]:+.3f}, {d['D_ci'][1]:+.3f}]", fontsize=10)
    ax.set_xticks(NS); ax.set_xlabel("n (K1 over F_2^n)"); ax.grid(alpha=0.3); ax.legend(fontsize=8, loc="upper left")
axes[0].set_ylabel("gap to matched rho [bits]")
fig.suptitle("Pair-table factor base growing with n vs fixed l = 7 (same targets, 8 per n); lower is closer to rho", fontsize=10)
fig.tight_layout()
for ext in ("svg", "png"): fig.savefig(HERE / "figures" / f"fig_lgrow_gap.{ext}", dpi=150)
print("ok")
