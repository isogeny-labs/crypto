#!/usr/bin/env python3
"""Figure for the flat-presentation exponent study: T and W against D (log-log) with the m = 3 fits, the m = 4
cells, the k = 24 controls, the recorded chain cells, and the per-operation time. Reads analysis.json."""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = os.path.dirname(os.path.abspath(__file__))
A = json.load(open(os.path.join(H, "analysis.json")))
cells = A["cells"]
k20 = [c for c in cells if c["miniature"] == "20" and not c["capped"]]
k24 = [c for c in cells if c["miniature"] != "20" and not c["capped"]]
cen = [c for c in cells if c["miniature"] == "20" and c["capped"]]
chain = [r for r in A["chain_vs_flat"] if r["chain_T"]]

fig, ax = plt.subplots(1, 3, figsize=(15, 4.8))
# panel 1: T
for m, mk, col in ((3, "o", "#1f77b4"), (4, "s", "#d62728")):
    pts = sorted((c["D"], c["T"]) for c in k20 if c["m"] == m)
    ax[0].plot([d for d, _ in pts], [t for _, t in pts], mk, color=col, label=f"flat m = {m} (k = 20)")
    ch = sorted((r["D"], r["chain_T"]) for r in chain if r["m"] == m)
    ax[0].plot([d for d, _ in ch], [t for _, t in ch], mk, mfc="none", color=col, alpha=0.6, label=f"chain m = {m} (recorded)")
ax[0].plot([c["D"] for c in k24], [c["T"] for c in k24], "^", color="#2ca02c", label="flat m = 3, k = 24 control")
f = A["fits"]["m3_T_all"]
xs = [9, 60]
ax[0].plot(xs, [math.exp(f["intercept"] + f["slope"] * math.log(x)) for x in xs], "-", color="#1f77b4", lw=1,
           label=f"m = 3 fit s_T = {f['slope']:.2f} [{f['lo']:.2f}, {f['hi']:.2f}]")
for c in cen:
    ax[0].axvline(c["D"], color="#d62728" if c["m"] == 4 else "#1f77b4", ls=":", lw=0.8)
ax[0].set_xscale("log"); ax[0].set_yscale("log"); ax[0].set_xlabel("D = |V|"); ax[0].set_ylabel("T = mean field_ops per target")
ax[0].set_title("Counted unit"); ax[0].legend(fontsize=7); ax[0].grid(alpha=0.3, which="both")
# panel 2: W
for m, mk, col in ((3, "o", "#1f77b4"), (4, "s", "#d62728")):
    pts = sorted((c["D"], c["W_ms"], c["load_flag"]) for c in k20 if c["m"] == m)
    ax[1].plot([d for d, _, fl in pts if not fl], [w for _, w, fl in pts if not fl], mk, color=col, label=f"flat m = {m}")
    ex = [(d, w) for d, w, fl in pts if fl]
    if ex:
        ax[1].plot([d for d, _ in ex], [w for _, w in ex], "x", color=col, ms=9, label=f"m = {m}, load-flagged (excluded)")
ax[1].plot([c["D"] for c in k24], [c["W_ms"] for c in k24], "^", color="#2ca02c", label="k = 24 control")
f = A["fits"]["m3_W_all"]
ax[1].plot(xs, [math.exp(f["intercept"] + f["slope"] * math.log(x)) for x in xs], "-", color="#1f77b4", lw=1,
           label=f"m = 3 fit s_W = {f['slope']:.2f} [{f['lo']:.2f}, {f['hi']:.2f}]")
ax[1].set_xscale("log"); ax[1].set_yscale("log"); ax[1].set_xlabel("D = |V|"); ax[1].set_ylabel("W = mean wall ms per target")
ax[1].set_title("Wall time"); ax[1].legend(fontsize=7); ax[1].grid(alpha=0.3, which="both")
# panel 3: ns per op
for m, mk, col in ((3, "o", "#1f77b4"), (4, "s", "#d62728")):
    pts = sorted((c["D"], c["ns_per_op"], c["regime"]) for c in k20 if c["m"] == m)
    ax[2].plot([d for d, _, r in pts if r == "first container"], [v for _, v, r in pts if r == "first container"], mk, color=col, label=f"m = {m}, first container")
    sec = [(d, v) for d, v, r in pts if r != "first container"]
    if sec:
        ax[2].plot([d for d, _ in sec], [v for _, v in sec], mk, mfc="none", color=col, label=f"m = {m}, second container")
ax[2].plot([c["D"] for c in k24], [c["ns_per_op"] for c in k24], "^", color="#2ca02c", label="k = 24 control")
ax[2].set_xscale("log"); ax[2].set_xlabel("D = |V|"); ax[2].set_ylabel("wall ns per counted field op")
ax[2].set_title("Per-operation time"); ax[2].legend(fontsize=7); ax[2].grid(alpha=0.3, which="both")
fig.suptitle("Flat presentation, sym form, k = 20 (p = 1,048,573, A = 926), N = 6 targets per cell; dotted lines: censored cells [pcc:unverified]", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(H, "flat_exponent.svg")); fig.savefig(os.path.join(H, "flat_exponent.png"), dpi=55)  # kept under the 64 KB artifact-policy limit for data files
print("wrote flat_exponent.svg/png")
