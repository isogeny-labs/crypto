# Plots recorded/recomputed values only (raw/recompute_A_haiku.json, raw/recompute_B_haiku.json,
# raw/replay_all/*.receipt.json). No new computation beyond reading fields.
import json, glob, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
A = sorted(json.load(open(f"{root}/raw/recompute_A_haiku.json")), key=lambda r: r["n"])
B = json.load(open(f"{root}/raw/recompute_B_haiku.json"))
fig, ax = plt.subplots(1, 3, figsize=(16, 4.6))
x = range(len(A))
ax[0].bar([i - 0.2 for i in x], [r["xref"] for r in A], 0.4, label="published (rho as built)", color="#4477aa")
ax[0].bar([i + 0.2 for i in x], [r["xref_canonical_step"] for r in A], 0.4, label="rho at canonical step", color="#ee6677")
ax[0].set_xticks(list(x)); ax[0].set_xticklabels([f"n={r['n']}" for r in A])
ax[0].set_ylabel("IC cost / rho cost (x reference)"); ax[0].set_title("Table A: reference kernel sensitivity")
ax[0].legend(fontsize=8)
for k, w in enumerate([0, 0.1, 1.0]):
    ax[1].plot(range(len(B)), [r["band"][k]["ratio"] for r in B], marker="o", ms=3, label=f"w = {w} GAE/native op")
ax[1].plot(range(len(B)), [r["xref"] for r in B], "k--", lw=1, label="published")
ax[1].set_yscale("log"); ax[1].set_xticks(range(len(B))); ax[1].set_xticklabels([r["instance"].replace("random-binary-", "bin-").replace("generated-", "gen-").replace("bench-", "") for r in B], rotation=70, fontsize=7)
ax[1].axhline(1, color="gray", lw=0.8); ax[1].set_ylabel("IC / rho under the section 5a band"); ax[1].set_title("Table B: ordering holds at every weight"); ax[1].legend(fontsize=7)
names, rep, tot = [], [], []
for f in sorted(glob.glob(f"{root}/raw/replay_all/*.receipt.json")):
    j = json.load(open(f))
    if not j["replays"]: continue
    names.append(os.path.basename(f).split(".receipt")[0].replace("__", "\n").replace("ecbench_", "").replace("_2026", "\n2026")[:34])
    rep.append(sum(1 for r in j["replays"] if r["reproduced"])); tot.append(len(j["replays"]))
ax[2].bar(range(len(names)), tot, color="#cccccc", label="replays run"); ax[2].bar(range(len(names)), rep, color="#228833", label="identical")
ax[2].set_xticks(range(len(names))); ax[2].set_xticklabels(names, fontsize=5, rotation=70); ax[2].set_title("ecbench verify --replay-all, this host"); ax[2].legend(fontsize=8)
fig.suptitle("Slice A audit of docs/ic/LEADERBOARD.md, tree cb2ec6873; values recomputed from raw records (sources: raw/)", fontsize=9)
fig.tight_layout()
fig.savefig(f"{root}/figures/audit_summary.svg"); fig.savefig(f"{root}/figures/audit_summary.png", dpi=150)
print("ok")
