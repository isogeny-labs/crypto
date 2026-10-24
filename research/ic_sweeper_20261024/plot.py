"""Lay out graph_data.json (recorded values only) as distance_to_rho.svg / .png."""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
d = json.loads((here / "graph_data.json").read_text())
fig, (a, b) = plt.subplots(1, 2, figsize=(10.5, 4.2))

lad = d["panel_a_wall_ratio"]["ladder"]["points"]
ref = d["panel_a_wall_ratio"]["msweep_reference"]["points"]
a.plot([p["log2_r"] for p in lad], [p["ratio"] for p in lad], "o-", color="#1f77b4",
       label="algebraic linear m=2 (ladder, 7 rungs)")
a.plot([p["log2_r"] for p in ref], [p["ratio"] for p in ref], "s", color="#d62728",
       label="table root-finder m=4, K=1107 (excluded by criterion; context)")
a.set_yscale("log")
a.set_xlabel("log2 r (prime subgroup order)")
a.set_ylabel("IC wall time / strong-rho wall time")
a.set_title("A. Wall-time distance to rho (measured)")
a.legend(fontsize=7, loc="upper left")
a.grid(True, which="both", alpha=0.3)

pts = d["panel_b_S_ratio"]["points"]
b.plot([p["log2_r"] for p in pts], [p["ratio"] for p in pts], "o-", color="#2ca02c")
for p in pts:
    b.annotate(p["curve"], (p["log2_r"], p["ratio"]), fontsize=6, xytext=(3, 3), textcoords="offset points")
b.set_xlabel("log2 r")
b.set_ylabel("S_IC / S_rho")
b.set_title("B. Leaderboard Table A, Koblitz (S units)")
b.grid(True, alpha=0.3)

fig.suptitle("Distance remaining to rho: panels use different units and are not comparable", fontsize=9)
fig.tight_layout()
fig.savefig(here / "distance_to_rho.svg")
fig.savefig(here / "distance_to_rho.png", dpi=150)
