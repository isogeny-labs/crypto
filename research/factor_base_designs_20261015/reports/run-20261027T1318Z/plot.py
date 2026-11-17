"""log2 field_ops against b for A0 and A11 at m = 4, p1, from runs/analysis-amendment14.txt
(the committed output of `analyze --amendment14`) and the two R256-A11 timeout rows.
Reads those files only; computes log2 and nothing else."""
import json, math, os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")
section = re.compile(r"^── (\w+) m = 4 p1")
row = re.compile(r"^\s+(\d)\s+([0-9.]+)\s+\[[^\]]*\]\s+\[[^\]]*\]\s+\[([0-9, ]+)\]")
series, cur = {}, None
for line in open(os.path.join(runs, "analysis-amendment14.txt"), encoding="utf-8"):
    s = section.match(line)
    if s:
        cur = s.group(1); series[cur] = []; continue
    r = row.match(line)
    if r and cur:
        series[cur].append((float(r.group(2)), [int(x) for x in r.group(3).split(",")]))
censored = [json.loads(l) for l in open(os.path.join(runs, "R256-A11-m4-t5.jsonl"))]
assert all(c["status"] == "timeout" for c in censored)

fig, ax = plt.subplots(figsize=(7.6, 5.2))
style = {"A0": ("#222222", "o"), "A11": ("#d62728", "D")}
for arm, pts in series.items():
    col, mk = style[arm]
    xs = [b for b, v in pts for _ in v]
    ys = [math.log2(x) for b, v in pts for x in v]
    ax.plot(xs, ys, mk, color=col, label=f"{arm}: completed systems", ms=6)
    mean = [(b, sum(math.log2(x) for x in v) / len(v)) for b, v in pts if v]
    ax.plot([m[0] for m in mean], [m[1] for m in mean], "-", color=col, lw=0.9, alpha=0.6)
cx = [5.0 for _ in censored]
cy = [math.log2(c["field_ops"]) for c in censored]
ax.plot(cx, cy, "v", color="#d62728", mfc="white", ms=8, mew=1.5,
        label="A11 t = 5: stopped at the 14 400 s budget (lower bound)")
ax.annotate("not completed:\nat least this many operations", (5.0, cy[0]), textcoords="offset points",
            xytext=(-120, -4), fontsize=8)
ax.set_xlabel("b = log₂|V| (factor-base bits)")
ax.set_ylabel("log₂ field_ops (counted F_p multiplications in row reduction)")
ax.set_title("m = 4, p₁: counted cost against b (R256 host; open markers are lower bounds)", fontsize=10)
ax.set_xticks([2, 3, 4, 5]); ax.grid(alpha=0.25); ax.legend(fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig(os.path.join(here, "field_ops_vs_b.svg")); fig.savefig(os.path.join(here, "field_ops_vs_b.png"), dpi=150)
