"""Render recorded source-vector counts and the implementation flow only."""
from pathlib import Path
import json
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
cases = json.loads((root.parent / "source-vectors.json").read_text())
counts = [len(case["targets"]) for case in cases]
fig, ax = plt.subplots(figsize=(8, 2.7))
ax.barh(["Frozen 202-bit source", "96-bit pilot source"], counts,
        color=["#326c98", "#649e87"])
ax.invert_yaxis()
ax.set_xlim(0, 36)
ax.set_xlabel("Source-only PARI scalar vectors available")
ax.set_title("Correctness inputs; no isogeny image or runtime result")
for row, count in enumerate(counts):
    ax.text(count + 0.5, row, str(count), va="center")
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
fig.savefig(root / "vectors.svg")
fig.savefig(root / "vectors.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(8, 2.9))
ax.axis("off")
nodes = [(0.15, 0.76, "Exact decimal inputs\np, a, b, N, r, G"),
         (0.78, 0.76, "PrimeMont4Explicit\nfour-limb coordinates"),
         (0.15, 0.18, "Shared-scalar targets\nexact workload identity"),
         (0.78, 0.18, "CountedGroup methods\nrho / full-point BSGS")]
for x, y, label in nodes:
    ax.text(x, y, label, ha="center", va="center",
            bbox={"boxstyle": "round,pad=0.6", "fc": "#edf3f7", "ec": "#326c98"})
for start, end in [((0.35, 0.76), (0.56, 0.76)),
                   ((0.78, 0.57), (0.78, 0.39)),
                   ((0.35, 0.18), (0.56, 0.18))]:
    ax.annotate("", xy=end, xytext=start,
                arrowprops={"arrowstyle": "->", "color": "#326c98"})
fig.tight_layout()
fig.savefig(root / "flow.svg")
fig.savefig(root / "flow.png", dpi=150)
plt.close(fig)
for name in ["vectors.svg", "flow.svg"]:
    file = root / name
    file.write_text("\n".join(line.rstrip() for line in file.read_text().splitlines()) + "\n")

rows = json.loads((root.parent / "paired-row-counts.json").read_text())
fig, ax = plt.subplots(figsize=(8, 2.7))
ax.barh([f'{row["field_bits"]}-bit field' for row in rows],
        [row["matching_rows"] for row in rows], color=["#649e87", "#326c98"])
ax.invert_yaxis()
ax.set_xlim(0, 108)
ax.set_xlabel("Exact source / destination / coordinate-control rows matched")
ax.set_title("Actual CLI and native comparator; no measured session")
for i, row in enumerate(rows):
    ax.text(row["matching_rows"] + 1, i, str(row["matching_rows"]), va="center")
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
fig.savefig(root / "paired-rows.svg")
fig.savefig(root / "paired-rows.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(8, 2.7))
ax.axis("off")
for y, field, degree in [(0.75, "202", "2"), (0.25, "96", "5")]:
    for x, label in [(0.18, f"E / Fp ({field} bits)\nG, sG"),
                     (0.82, f"F / Fp ({field} bits)\nH, sH")]:
        ax.text(x, y, label, ha="center", va="center",
                bbox={"boxstyle": "round,pad=0.5", "fc": "#edf3f7", "ec": "#326c98"})
    ax.annotate("", xy=(0.65, y), xytext=(0.35, y),
                arrowprops={"arrowstyle": "->", "color": "#326c98"})
    ax.text(0.5, y + 0.09, f"degree-{degree} isogeny", ha="center")
fig.tight_layout()
fig.savefig(root / "transfers.svg")
fig.savefig(root / "transfers.png", dpi=150)
plt.close(fig)
for name in ["paired-rows.svg", "transfers.svg"]:
    file = root / name
    file.write_text("\n".join(line.rstrip() for line in file.read_text().splitlines()) + "\n")
