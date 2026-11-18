"""Draw recorded values from report.json (written by `btn-scaling report`).
Layout only: every plotted number is read from the report; the fitted lines
use the recorded slope and intercept. Writes scaling.svg and scaling.png."""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(H, "report.json")))
rows = [json.loads(l) for l in open(os.path.join(H, "scaling.jsonl"))]
fits = R["scaling"]["fits_8_bits_and_up"]
fig, ax = plt.subplots(1, 3, figsize=(17, 4.6))

for full, colour, label in ((True, "#2a6fdb", "threshold s"), (False, "#e69f00", "threshold s/4")):
    sel = [r for r in rows if (r["threshold"] == r["hash_prime"]) == full]
    b = [r["bits"] for r in sel]
    ax[0].plot(b, [math.log2(r["steps"]) for r in sel], "o", color=colour, label=f"charged steps, {label}")
    f = fits["threshold_s" if full else "threshold_s_over_4"]["log2_steps_vs_bits"]
    xs = [3, 17]
    ax[0].plot(xs, [f["slope"] * x + f["intercept"] for x in xs], "-", color=colour, lw=1,
               label=f"fit: {f['slope']:.2f} per bit (R² {f['r2']:.3f})")
a = [r for r in rows if r["threshold"] == r["hash_prime"]]
ax[0].plot([r["bits"] for r in a], [math.log2(r["functional_assignments"]) for r in a], "s", color="#888",
           label="visited assignments (1.00 per bit)")
ax[0].plot([3, 17], [1.5 + 0.5 * 3, 1.5 + 0.5 * 17], "--", color="#27ae60", label="rho: 0.5 per bit (slope only)")
ax[0].set_xlabel("field bits b"); ax[0].set_ylabel("log2 per exact count")
ax[0].set_title("Measured: one two-slot count, every row = independent count"); ax[0].legend(fontsize=7)

T = R["cost_table"]
for slots, mk in ((2, "o"), (3, "s")):
    t = [x for x in T if x["slots"] == slots]
    ax[1].plot([x["bits"] for x in t], [x["lambda_estimate"] for x in t], mk + "-", label=f"lambda = ceil(log2 2s), m={slots}")
ax[1].axhline(131072, color="#c0392b", ls="--", label="recursion needs lambda >= 131072")
ax[1].set_yscale("log"); ax[1].set_ylim(10, 4e5)
ax[1].set_xlabel("field bits b"); ax[1].set_ylabel("lambda (log scale)")
ax[1].set_title("Recursive backend guard at 120-500 bits"); ax[1].legend(fontsize=7, loc="center right")

b = sorted({x["bits"] for x in T})
rho = [next(x["log2_rho"] for x in T if x["bits"] == bb) for bb in b]
ax[2].plot(b, rho, "--", color="#27ae60", label="Pollard rho (analytic)")
for slots, colour in ((2, "#2a6fdb"), (3, "#8e44ad")):
    t = [x for x in T if x["slots"] == slots]
    ax[2].plot([x["bits"] for x in t], [x["log2_fallback_total_lower"] for x in t], "o-", color=colour,
               label=f"implemented, m={slots}: lower bound")
    ax[2].plot([x["bits"] for x in t], [x["log2_solver_calls"] for x in t], ":", color=colour,
               label=f"solver calls alone, m={slots}")
ax[2].set_xlabel("field bits b"); ax[2].set_ylabel("log2 operations")
ax[2].set_title("Remark 2 cost with the implemented solver"); ax[2].legend(fontsize=7)

fig.suptitle("hashed-ecdl-tensor scaling and the Bochum v0.2 cost composition (research/btn_scaling_exponent_20261118/runs)", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(H, "scaling.svg"))
fig.savefig(os.path.join(H, "scaling.png"), dpi=150)
print("wrote scaling.svg, scaling.png")
