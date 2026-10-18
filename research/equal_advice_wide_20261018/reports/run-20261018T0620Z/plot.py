"""Lays out recorded values only (no computation beyond reading): n=61 narrow vs wide BL counters."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = "../../instrument_check_n61/"
def load(f):
    rows = {}
    for l in open(D + f):
        d = json.loads(l)
        if d["kind"] == "bl_online_target":
            rows[(d["prefix_level"], d["target_index"])] = (d["online_steps"], d["online_ms"])
        if d["kind"] == "bl_precompute":
            ns = d["ns_per_precompute_step"]
    return rows, ns
rn, nsn = load("narrow_multi.jsonl"); rw, nsw = load("wide_multi.jsonl")
fig, ax = plt.subplots(1, 2, figsize=(9, 3.8))
cols = {0: "#1f77b4", 1: "#ff7f0e", 2: "#2ca02c"}
for lv in (0, 1, 2):
    ks = [k for k in rn if k[0] == lv]
    ax[0].scatter([rn[k][0] for k in ks], [rw[k][0] for k in ks], c=cols[lv], label=f"prefix level {lv} (8 targets)")
m = max(v[0] for v in rn.values()); ax[0].plot([1e3, m], [1e3, m], "k--", lw=0.8, label="y = x")
ax[0].set_xscale("log"); ax[0].set_yscale("log")
ax[0].set_xlabel("narrow instrument: online steps"); ax[0].set_ylabel("wide instrument: online steps")
ax[0].set_title("Online steps per target, n=61, T=4096"); ax[0].legend(fontsize=7)
ax[1].bar(["narrow", "wide"], [nsn, nsw], color=["#7f7f7f", "#1f77b4"])
ax[1].set_ylabel("ns per precompute step (9.2e7 steps)")
ax[1].set_title("Precompute step cost, shared 4-core host\n(contended; indicative only)")
fig.tight_layout(); fig.savefig("fig_n61_check.svg"); fig.savefig("fig_n61_check.png", dpi=150)
