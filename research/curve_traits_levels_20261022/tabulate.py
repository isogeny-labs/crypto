"""Tabulate the volcano levels of two committed traits.json files and plot
them: a layout of recorded values only (AGENTS.md: no Python computes a
result).  Usage: tabulate.py BEFORE.json AFTER.json OUTDIR"""
import collections, json, re, sys

import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

before, after, out = sys.argv[1], sys.argv[2], sys.argv[3]


def open_kind(method):
    if "above the scalar test" in method:
        return "ℓ above the tests' range"
    if "beyond the cost" in method:
        return "depth 1, beyond the budget"
    return "only bounded"


def tally(path):
    d = json.load(open(path))
    status = collections.Counter((r.get("endomorphism") or {}).get("status") for r in d["curves"])
    open_ = collections.Counter()
    decided = collections.Counter()
    for r in d["curves"]:
        for lv in (r.get("endomorphism") or {}).get("levels", []):
            m = lv["method"]
            if lv["level"] is None:
                open_[open_kind(m)] += 1
            elif m.startswith("certified") or m.startswith("f₀"):
                decided["other (certified, f₀ prime to ℓ)"] += 1
            elif "isogen" in m:
                decided["isogeny walks"] += 1
            elif "GF(q^k)" in m:
                decided["depth-1 point test"] += 1
            elif re.search(r"E\[ℓ\^\d|E\[ℓ²\]|E\[ℓ\]", m):
                decided["scalar tests on E[ℓ^j]"] += 1
            else:
                decided["other (certified, f₀ prime to ℓ)"] += 1
    return {"status": dict(status), "open": dict(open_), "decided": dict(decided)}


data = {"before": tally(before), "after": tally(after)}
json.dump(data, open(f"{out}/levels.json", "w"), indent=1, ensure_ascii=False)

kinds = ["only bounded", "depth 1, beyond the budget", "ℓ above the tests' range"]
fig, ax = plt.subplots(figsize=(7, 3.6))
xs = range(len(kinds))
b = [data["before"]["open"].get(k, 0) for k in kinds]
a = [data["after"]["open"].get(k, 0) for k in kinds]
ax.bar([x - 0.2 for x in xs], b, 0.4, label=f"main ({sum(b)} open)", color="#9aa5b1")
ax.bar([x + 0.2 for x in xs], a, 0.4, label=f"this PR ({sum(a)} open)", color="#2f6fb0")
for x, v in zip(xs, b):
    ax.text(x - 0.2, v + 2, str(v), ha="center", fontsize=8)
for x, v in zip(xs, a):
    ax.text(x + 0.2, v + 2, str(v), ha="center", fontsize=8)
ax.set_xticks(list(xs), kinds, fontsize=8)
ax.set_ylabel("open volcano levels (count)")
ax.set_title("Open End(E) levels, docs/curves/traits.json (2748 curves)", fontsize=9)
ax.legend(fontsize=8)
fig.tight_layout()
fig.savefig(f"{out}/open_levels.svg")
fig.savefig(f"{out}/open_levels.png", dpi=150)
print(json.dumps(data, ensure_ascii=False))
