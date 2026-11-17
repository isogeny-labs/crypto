"""Plots recorded values only (reports/analysis.json); computes nothing."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("reports/analysis.json"))
A = d["stage_A_scan"]
fig, ax = plt.subplots(1, 3, figsize=(15, 4.8))
ctl = A["ctl_ns_per_probe_median"]
arms = ["sparse", "algebra", "all", "ctl2"]
labs = ["sparse\nreduce", "root\nalgebra", "all\nT21", "A/A\n(ctl2)"]
for i, a in enumerate(arms):
    ys = A["arms"][a]["ns_per_probe_all"]
    ax[0].plot([i + 1] * len(ys), ys, "o", alpha=.6, color="#3b7dd8")
    ax[0].plot([i + 1], [A["arms"][a]["ns_per_probe_median"]], "_", color="#d62728", ms=24, mew=2)
ax[0].axhline(ctl, color="#555", ls="--", label=f"ctl median {ctl:.1f}")
ax[0].axhline(ctl / 1.2, color="#c33", ls=":", label="1.2x gate")
ax[0].set_xticks(range(1, 5)); ax[0].set_xticklabels(labs); ax[0].set_ylabel("ns per probe (20,000-state scan, K=600)")
ax[0].set_title("(a) scan, 7 interleaved rounds"); ax[0].legend(fontsize=8)

for i, a in enumerate(arms):
    r = A["arms"][a]["ratio_to_ctl"]
    ax[1].plot([r["lo"], r["hi"]], [i, i], color="#333", lw=2)
    ax[1].plot(r["point"], i, "o", color="#d62728")
    ax[1].text(r["hi"] + .01, i + .05, f'{r["point"]:.3f}', fontsize=8)
ax[1].axvline(1.0, color="#888", ls="--"); ax[1].axvline(1 / 1.2, color="#c33", ls=":")
ax[1].text(1 / 1.2 + .005, -0.35, "1/1.2 gate", color="#c33", fontsize=8)
ax[1].set_yticks(range(4)); ax[1].set_yticklabels(labs); ax[1].invert_yaxis()
ax[1].set_xlim(0.75, 1.3); ax[1].set_xlabel("ratio to ctl (median; 95% bootstrap over rounds)")
ax[1].set_title("(b) paired ratio vs the 1.2x gate")

P = d["stage_P_process_start"]
ax[2].plot(range(1, 6), P["old_real_s"], "o-", color="#7f7f7f", label="old (T20 head)")
ax[2].plot(range(1, 6), P["new_real_s"], "o-", color="#3b7dd8", label="new (early-stop factorise)")
ax[2].set_yscale("log"); ax[2].set_xlabel("round"); ax[2].set_ylabel("process start wall, s (construct:73:1:40, 1 core)")
ax[2].set_title("(c) process start, paired rounds"); ax[2].legend(fontsize=8)
fig.suptitle("T21 recorded values: wide producer arithmetic cuts (no scan gain) and the process-start factorise cut", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, .95))
fig.savefig("reports/t21.svg"); fig.savefig("reports/t21.png", dpi=150)
