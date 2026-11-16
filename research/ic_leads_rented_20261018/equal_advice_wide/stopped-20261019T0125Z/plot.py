"""BL precompute progress of the stopped n83 run, with a linear-in-rate projection. python3 plot.py"""
import re, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
pts = [(int(s), int(d)) for d, s in re.findall(r"(\d+)/934680064 DPs, \d+ steps, epoch \d+, (\d+)s", open("n83-bl.stderr").read())]
T, N = 934680064, None
t = [p[0] / 3600 for p in pts]; d = [p[1] / 1e6 for p in pts]
# per-epoch DP gain and its decline, fitted on the recorded epochs
gains = [(pts[i][1] - pts[i - 1][1], pts[i][0] - pts[i - 1][0]) for i in range(1, len(pts))]
g0, dt = gains[-1]; decay = (gains[-1][0] / gains[0][0]) ** (1 / max(1, len(gains) - 1))
pt, pd, cur, now = [t[-1]], [d[-1]], pts[-1][1], pts[-1][0]
while cur < T and len(pt) < 400:
    g0 *= decay; cur += g0; now += dt; pt.append(now / 3600); pd.append(min(cur, T) / 1e6)
fig, ax = plt.subplots(figsize=(7, 4))
ax.plot(t, d, "o-", ms=3, label="recorded (bl.stderr)")
ax.plot(pt, pd, "--", label=f"projection (per-epoch gain x{decay:.4f} each epoch)")
ax.axhline(T / 1e6, c="grey", ls=":"); ax.text(0.1, T / 1e6 * 1.01, "table target 934.7M DPs", fontsize=8)
ax.axvline(6, c="red", ls=":"); ax.text(6.1, 50, "pod lifetime cap (6 h)", fontsize=8, color="red")
ax.axvline(t[-1], c="k", ls="-", lw=.6); ax.text(t[-1] + .1, 450, "stopped", fontsize=8)
ax.set_xlabel("hours since BL precompute start"); ax.set_ylabel("distinguished points (millions)")
ax.set_title("equal-advice n = 83, BL table precompute, 32 threads (cpu3g-32-128)", fontsize=9)
ax.grid(alpha=.3); ax.legend(fontsize=8, loc="lower right"); fig.tight_layout()
fig.savefig("bl-progress.svg"); fig.savefig("bl-progress.png", dpi=150)
print(f"recorded {d[-1]:.1f}M at {t[-1]:.2f} h; projected finish {pt[-1]:.1f} h; decay {decay:.4f}")
