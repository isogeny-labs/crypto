"""Plots recorded values only (reports/analysis.json); computes nothing."""
import json, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("reports/analysis.json"))
S = d["scan"]
fig, ax = plt.subplots(1, 3, figsize=(15, 4.8))
old = S["old_ns_per_probe_median"]
cols = {"new": "#3b7dd8", "old2": "#7f7f7f"}
for i, a in enumerate(["new", "old2"]):
    ys = S["arms"][a]["ns_per_probe_all"]
    ax[0].plot([i + 1] * len(ys), ys, "o", alpha=.6, color=cols[a])
    ax[0].plot([i + 1], [S["arms"][a]["ns_per_probe_median"]], "_", color="#d62728", ms=26, mew=2)
ax[0].axhline(old, color="#555", ls="--", label=f"old median {old:.1f}")
ax[0].axhline(old / 1.2, color="#c33", ls=":", label="1.2x gate")
ax[0].set_xticks([1, 2]); ax[0].set_xticklabels(["new\n(inlined clmul)", "old2\n(A/A)"])
ax[0].set_ylabel("ns per probe (20,000-state scan, K=600)"); ax[0].set_title("(a) scan, 7 rotated rounds"); ax[0].legend(fontsize=8)
for i, a in enumerate(["new", "old2"]):
    r = S["arms"][a]["ratio_to_old"]
    ax[1].plot([r["lo"], r["hi"]], [i, i], color="#333", lw=2); ax[1].plot(r["point"], i, "o", color="#d62728")
    ax[1].text(r["hi"] + .01, i + .05, f'{r["point"]:.3f}', fontsize=8)
ax[1].axvline(1.0, color="#888", ls="--"); ax[1].axvline(1 / 1.2, color="#c33", ls=":")
ax[1].text(1 / 1.2 + .005, 1.45, "1/1.2 gate", color="#c33", fontsize=8)
ax[1].set_yticks([0, 1]); ax[1].set_yticklabels(["new / old", "old2 / old (A/A)"]); ax[1].invert_yaxis()
ax[1].set_xlim(0.75, 1.2); ax[1].set_xlabel("ratio (median; 95% bootstrap over rounds)"); ax[1].set_title("(b) paired ratio vs the 1.2x gate")

def parse(arm):
    out = {}
    for l in d["callgrind_k40"][arm]["lines"]:
        m = re.match(r"([\d,]+) \(.*?\)\s+\S*?:(.*?) \[", l)
        if m: out[re.split(r"::", m.group(2))[-1]] = int(m.group(1).replace(",", ""))
    return out
o, n = parse("old"), parse("new")
keys = [k for k in ["reduce_fold", "mul_fold", "clmul_u64", "sqr", "extract128", "solve", "build_index128_impl"] if k in o or k in n]
w = .38
ax[2].bar([i - w / 2 for i in range(len(keys))], [o.get(k, 0) / 1e6 for k in keys], w, color="#7f7f7f", label=f'old total {d["callgrind_k40"]["old"]["collected_ir"] / 1e6:.0f} M')
ax[2].bar([i + w / 2 for i in range(len(keys))], [n.get(k, 0) / 1e6 for k in keys], w, color="#3b7dd8", label=f'new total {d["callgrind_k40"]["new"]["collected_ir"] / 1e6:.0f} M')
ax[2].set_xticks(range(len(keys))); ax[2].set_xticklabels(keys, rotation=30, ha="right", fontsize=8)
ax[2].set_ylabel("callgrind Ir, M (K=40, 2000 states)"); ax[2].set_title("(c) instructions by function"); ax[2].legend(fontsize=8)
fig.suptitle("T22 recorded values: inlining clmul_u64 removes instructions but not scan time", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, .95))
fig.savefig("reports/t22.svg"); fig.savefig("reports/t22.png", dpi=150)
