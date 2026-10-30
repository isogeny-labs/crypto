"""Figures for PILOT_20261030 section 9 (follow-up A). Reads runs/T9_priced.json, runs/T13_priced.json,
../pilot_20261030/graphs/numbers.json and probes/n13_*.jsonl; writes fig_tuned.{svg,pdf,png} and fig_sweep.{svg,pdf,png}."""
import json, glob, os, re, statistics as st
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..")
t9 = json.load(open(os.path.join(R, "runs/T9_priced.json"))); t13 = json.load(open(os.path.join(R, "runs/T13_priced.json"))); t13b = json.load(open(os.path.join(R, "runs/T13b_priced.json")))
pil = json.load(open(os.path.join(R, "../pilot_20261030/graphs/numbers.json")))
def per(d, arm, key):
    return [x[key] for x in d["rows"] if x["arm"] == arm and x["status"] == "verified"]
fig, ax = plt.subplots(figsize=(7.6, 4.4))
cells = [
    ("n=9 lab (inherited F4)", per(t9, "inherited-f4", "ir_per_target"), "#4c72b0"),
    ("n=9 msolve pilot (e-form, untuned)", pil["n9"]["msolve_ir_run1"], "#c44e52"),
    ("n=9 msolve tuned, whole run", per(t9, "msolve-x-l4-s4", "ir_per_target"), "#dd8452"),
    ("n=9 msolve tuned, solver only", per(t9, "msolve-x-l4-s4", "solver_only_ir"), "#e5ae38"),
    ("n=13 lab (inherited F4)", per(t13, "inherited-f4", "ir_per_target"), "#4c72b0"),
    ("n=13 msolve l=5 s=5, whole run", per(t13, "msolve-x-l5-s5", "ir_per_target"), "#dd8452"),
    ("n=13 msolve tuned (l=4 s=4), whole run", per(t13b, "msolve-x-l4-s4", "ir_per_target"), "#dd8452"),
    ("n=13 msolve tuned (l=4 s=4), solver only", per(t13b, "msolve-x-l4-s4", "solver_only_ir"), "#e5ae38"),
]
for i, (lab, v, c) in enumerate(cells):
    ax.scatter([i] * len(v), v, color=c, s=18, zorder=3)
    ax.hlines(st.median(v), i - 0.3, i + 0.3, color=c, lw=2)
ax.set_yscale("log"); ax.set_xticks(range(len(cells))); ax.set_xticklabels([c[0] for c in cells], rotation=30, ha="right", fontsize=8)
ax.set_ylabel("instructions retired per target (whole solve)"); ax.grid(axis="y", alpha=0.3)
ax.set_title("K_0, 8 targets each, all verified: lab arm vs published arm before/after tuning", fontsize=9)
fig.tight_layout()
for ext in ("svg", "pdf"): fig.savefig(os.path.join(H, "fig_tuned." + ext))
fig.savefig(os.path.join(H, "fig_tuned.png"), dpi=150)
fig, ax = plt.subplots(figsize=(6.4, 3.8))
pts = {}
for f in glob.glob(os.path.join(R, "probes/n13_l*_x_s*.jsonl")):
    m = re.search(r"n13_l(\d)_x_s(\d+)", f); l, k = int(m.group(1)), int(m.group(2))
    rows = [json.loads(x) for x in open(f)]
    pts.setdefault(l, []).append((k, st.mean(r["msolve_ir"] for r in rows)))
for l, c in zip(sorted(pts), ["#4c72b0", "#dd8452", "#55a868"]):
    v = sorted(pts[l]); ax.plot([a for a, _ in v], [b for _, b in v], "o-", color=c, label=f"l={l}")
ax.axhline(5.70e12, color="#c44e52", ls="--", lw=1); ax.text(6.2, 3.2e12, "pilot e-form, l=5, spec=0: 5.7e12", fontsize=7, color="#c44e52")
ax.axhline(5.3e7, color="#4c72b0", ls=":", lw=1); ax.text(6.2, 6.5e7, "lab arm whole run per solver call: 5.3e7", fontsize=7, color="#4c72b0")
ax.set_yscale("log"); ax.set_xlabel("x-bits specialised (hybrid, 2^spec msolve calls)"); ax.set_ylabel("msolve instructions per relation system")
ax.set_title("K_0 n=13, x-form, 3 targets per point (probe; all roots agree with brute force)", fontsize=9); ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout()
for ext in ("svg", "pdf"): fig.savefig(os.path.join(H, "fig_sweep." + ext))
fig.savefig(os.path.join(H, "fig_sweep.png"), dpi=150)
print("ok")
