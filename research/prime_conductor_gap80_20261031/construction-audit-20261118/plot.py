"""Plot the recorded field degrees only; not a crypto search or timer."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = Path(__file__).resolve().parent
data = json.loads((root / "attempt-b/positive.json").read_text())
rows = data["full_two_power_torsion_orders"]
fig, ax = plt.subplots(figsize=(8.0, 3.5), constrained_layout=True)
ax.plot([r["m"] for r in rows], [float(r["extension_degree"]) for r in rows], color="#346b92")
ax.set_yscale("log", base=2)
for m, text in [(data["interpolation_cardinality_m"], "cardinality condition"),
                (data["single_power_embedding_m"], "single-power embedding")]:
    row = next(r for r in rows if r["m"] == m)
    value = float(row["extension_degree"])
    ax.scatter([m], [value], color="#a74d41")
    ax.annotate(text, (m, value), xytext=(-10, -21), textcoords="offset points", ha="right", fontsize=8)
ticks = [0, 20, 39, 60, 78]
ax.set_yticks([float(2**k) for k in ticks], [f"2^{k}" for k in ticks])
ax.set_xlabel("m in full E[2^m]")
ax.set_ylabel("Extension degree over the frozen Fp")
ax.set_title("Single-power-of-two route · conditional on frozen CM trace")
ax.spines[["top", "right"]].set_visible(False)
out = root / "reports"; out.mkdir(exist_ok=True)
for ext in ("svg", "png"):
    path = out / ("torsion-orders." + ext)
    fig.savefig(path, dpi=160)
    if ext == "svg":
        path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
