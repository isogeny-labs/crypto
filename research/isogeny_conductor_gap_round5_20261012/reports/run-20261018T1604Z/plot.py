"""Plot recorded mean S per arm against log2 #E for ecbench session e7a_h72 (recorded values only)."""
import json, os, collections, math
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
by = collections.defaultdict(list); order = {}
for l in open(os.path.join(here, "../../sessions/e7a_h72/records.jsonl")):
    r = json.loads(l)
    if r["warmup"] or r["outcome"]["status"] != "verified":
        continue
    c = r["workload"]["curve"]; by[(r["arm"], c["slug"])].append(r["cost"]["s"]); order[c["slug"]] = c["group_order"]
arms = sorted({a for a, _ in by}, key=lambda a: (a != "rho-neg", a.split("-b")[0], int(a.split("-b")[1]) if "-b" in a else 0))
fig, ax = plt.subplots(figsize=(7.5, 4.4))
closed = [a for a in arms if a.startswith("closed")]; sub = [a for a in arms if a.startswith("subgroup")]
for a in arms:
    pts = sorted((math.log2(order[s]), sum(v) / len(v)) for (aa, s), v in by.items() if aa == a)
    xs = [p[0] for p in pts]; ys = [p[1] for p in pts]
    if a == "rho-neg":
        kw = dict(color="black", marker="o", lw=2, label="rho-neg (reference)")
    elif a in closed:
        kw = dict(color=plt.get_cmap("Oranges")(0.4 + 0.5 * closed.index(a) / max(1, len(closed) - 1)), marker="s", ls="--", label=a + " (fiber-closed, raw base)")
    else:
        kw = dict(color=plt.get_cmap("Blues")(0.4 + 0.5 * sub.index(a) / max(1, len(sub) - 1)), marker="^", label=a + " (blind, cleared base)")
    ax.plot(xs, ys, ms=4, **kw)
ax.set_yscale("log"); ax.set_xlabel("log2 #E"); ax.set_ylabel("mean S over 6 verified runs")
ax.set_title("e7a_h72 (ECBS1hd38ca558d7e4)", fontsize=9); ax.grid(True, which="both", alpha=0.3); ax.legend(fontsize=6, ncol=2)
fig.tight_layout(); fig.savefig(os.path.join(here, "s_by_arm.svg")); print("ok")
