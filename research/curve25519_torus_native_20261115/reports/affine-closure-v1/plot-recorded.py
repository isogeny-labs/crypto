"""Layout only: copy recorded node counts; never invoke research executables."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
matplotlib.rcParams["svg.fonttype"] = "none"

here = Path(__file__).resolve().parent
raw = here.parent.parent / "affine-closure-v1/native-v1/controls.json"
data = json.loads(raw.read_text())
rows = []
for group in (data["toy"], data["actual_q1"]):
    for item in group:
        case = item["case"]
        rows.append({"id": case["id"],
                     "before_nodes": case["before"]["counts"]["nodes"],
                     "after_nodes": case["after"]["counts"]["nodes"]})
(here / "recorded-nodes.json").write_text(json.dumps({
    "source": "affine-closure-v1/native-v1/controls.json",
    "scope": "recorded fixed-input nodes, not complete work or timing",
    "standing": "producer unsigned/unverified", "rows": rows}, indent=2) + "\n")
fig, axes = plt.subplots(1, 3, figsize=(10.6, 2.55), gridspec_kw={"width_ratios": [4, 4, 1.8]})
for ax, suffix, title in zip(axes[:2], ("original", "indexed"), ("Toy original: Unsupported", "Toy indexed: Root")):
    selected = [r for r in rows if r["id"].startswith("toy-") and r["id"].endswith(suffix)]
    xs = list(range(13))
    ax.plot(xs, [r["before_nodes"] for r in selected], "o-", color="#1b5b8f", label="Before", markersize=5)
    ax.plot(xs, [r["after_nodes"] for r in selected], "x--", color="#df7729", label="After", markersize=5)
    ax.set_title(title, fontsize=10)
    ax.set_xticks([0, 3, 6, 9, 12])
    ax.set_xlabel("Fixed public target index")
    ax.set_ylabel("Recorded search nodes")
    ax.grid(alpha=.2)
actual = [r for r in rows if r["id"].startswith("actual-q1")]
axes[2].plot([0, 1], [r["before_nodes"] for r in actual], "o", color="#1b5b8f", markersize=7)
axes[2].plot([0, 1], [r["after_nodes"] for r in actual], "x", color="#df7729", markersize=7)
axes[2].set_xticks([0, 1], ["Original", "Indexed"], fontsize=8)
axes[2].set_title("Actual q1: Root", fontsize=10)
axes[2].set_ylim(0, 10)
axes[2].set_ylabel("Recorded search nodes")
axes[2].grid(alpha=.2)
axes[0].legend(fontsize=8, loc="lower right")
fig.suptitle("Static affine v1: unchanged before/after nodes on all 28 fixed curve inputs", fontsize=11)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(here / ("nodes." + ext), dpi=150)
svg = here / "nodes.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
