"""Draw recorded values from report.json (written by the native
`btn-prime-scaling report`).  Layout only: every plotted number is read from
the report; nothing is computed here beyond log2 of a recorded integer for the
axis.  Writes native_scaling.svg and native_scaling.png (150 dpi).
"""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(H, "report.json")))
sizes = R["sizes"]["rows"]
counts = R["counts"]["rows"]
eccp = R["eccp131"]

fig, ax = plt.subplots(1, 3, figsize=(14, 4.2))

d = [s["d"] for s in sizes]
ax[0].plot(d, [s["edges"] for s in sizes], "o-", color="#2a6fdb", label="edges")
ax[0].plot(d, [s["vertices"] for s in sizes], "s-", color="#888", label="vertices")
net = (eccp.get("network") or {}).get("networks") or []
if net:
    two = [n for n in net if n["slots"] == 2][0]
    ax[0].annotate(f"ECCp-131 (d=131): {two['edges']:,} edges", xy=(0.02, 0.92), xycoords="axes fraction", fontsize=8)
ax[0].set_xlabel("field bits d"); ax[0].set_ylabel("count")
ax[0].set_title("Decomposition network size (2 slots)"); ax[0].legend(fontsize=8)

cd = [c["d"] for c in counts]
ax[1].plot(cd, [c["log2_functional_assignments"] for c in counts], "o", color="#2a6fdb",
           label="recorded functional assignments")
ax[1].plot([3, 4], [12, 16], "-", color="#888", lw=1, label="2^(4d)")
ax[1].plot(cd, [math.log2(c["steps"]) for c in counts], "^", color="#c0392b", label="recorded charged steps")
ax[1].set_xlabel("field bits d"); ax[1].set_ylabel("log2 per contraction")
ax[1].set_title(f"One exact count ({len(counts)} counts, all = brute force)"); ax[1].legend(fontsize=8)
ax[1].set_xticks([3, 4])

ax[2].bar(["Functional backend:\nassignments per\ncontraction", "Pollard rho:\ngroup ops for\nwhole DLP"],
          [eccp["functional_backend_log2_assignments_per_contraction"], eccp["pollard_rho_log2_group_ops"]],
          color=["#2a6fdb", "#27ae60"])
for i, v in enumerate([eccp["functional_backend_log2_assignments_per_contraction"], eccp["pollard_rho_log2_group_ops"]]):
    ax[2].text(i, v + 8, f"2^{v:.1f}", ha="center", fontsize=9)
ax[2].set_ylabel("log2 operations"); ax[2].set_title("ECCp-131 (d = 131), derived")
ax[2].set_ylim(0, 600)

fig.suptitle("hashed-ecdl-tensor, native measurements (research/btn_ecdl_prime_20261029/native_runs)", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(H, "native_scaling.svg"))
fig.savefig(os.path.join(H, "native_scaling.png"), dpi=150)
print("wrote native_scaling.svg, native_scaling.png")
