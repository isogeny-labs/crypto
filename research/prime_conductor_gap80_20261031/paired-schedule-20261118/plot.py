"""Plot actual planned ordering; no EC arithmetic, solver, or timer."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import ListedColormap

root = Path(__file__).resolve().parent
plan = json.loads((root / "proof/v4-paired.json").read_text())
source = json.loads((root / "paired-cli-spec.json").read_text())
orders = sorted({w["curve"]["r"] for w in plan["workloads"]})
fig, axes = plt.subplots(1, 2, figsize=(8.2, 3.0), constrained_layout=True)
colors = ["#427a9c", "#419274", "#b58445"]
for ax, order in zip(axes, orders):
    data = []
    for round_index in range(source["measurement"]["rounds"]):
        row = []
        for event in plan["executions"]:
            workload = plan["workloads"][event["workload"]]
            if (event["warmup"] or event["round"] != round_index + source["measurement"]["warmup"]
                    or workload["target_index"] != 0 or workload["curve"]["r"] != order):
                continue
            curve_index = source["workloads"]["curves"].index(workload["curve_spec"]) % 3
            if not row or row[-1] != curve_index:
                row.append(curve_index)
        assert len(row) == 3
        data.append(row)
    ax.imshow(data, cmap=ListedColormap(colors), vmin=0, vmax=2, aspect="auto")
    ax.set_xticks(range(3), ["first", "middle", "last"])
    ax.set_yticks(range(len(data)), range(1, len(data) + 1))
    ax.set_ylabel("Measured round")
    ax.set_title("r = " + str(order))
    for i, row in enumerate(data):
        for j, value in enumerate(row):
            ax.text(j, i, ["source", "image", "u=2"][value], color="white", ha="center", va="center", fontsize=9)
fig.suptitle("Actual paired plan · target index 0 · no measured session")
reports = root / "reports"; reports.mkdir(exist_ok=True)
for ext in ("svg", "png"):
    path = reports / ("endpoint-order." + ext)
    fig.savefig(path, dpi=160)
    if ext == "svg":
        path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
