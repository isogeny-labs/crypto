"""Lay out what ecbench recorded: mean S per arm and curve (ecbench table)
and each candidate's paired operation ratio to its reference (ecbench
compare --save).  Reads only those outputs; computes nothing new."""

import json
import re
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE / sys.argv[1] if len(sys.argv) > 1 else HERE / "figures"
OUT.mkdir(parents=True, exist_ok=True)
FAMILIES = [
    ("genus2", "genus-2 Jacobians (Jacobian operations)"),
    ("quartic", "plane-quartic Jacobians (Jacobian operations)"),
    ("ghs", "E(F_{p^6}), GHS curves (additions on E)"),
    ("prime", "prime-field curve (additions on E)"),
]


def table(name):
    rows = []
    for line in (HERE / "out" / f"table-{name}.md").read_text().splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 9 or cells[0] in ("session", "---") or cells[0].startswith("-"):
            continue
        lo, hi = re.findall(r"[\d.]+", cells[7])[:2]
        rows.append(
            {
                "session": cells[0],
                "arm": cells[1],
                "method": cells[2],
                "curve": cells[3],
                "log2r": float(cells[4]),
                "verified": cells[5],
                "s": float(cells[6]),
                "lo": float(lo),
                "hi": float(hi),
            }
        )
    return rows


def figure_s():
    fig, axes = plt.subplots(4, 1, figsize=(8.6, 11.5))
    for ax, (name, title) in zip(axes, FAMILIES):
        rows = [r for r in table(name) if not r["arm"].endswith("-aa")]
        labels = [f'{r["arm"]}  ·  {r["curve"]} (2^{r["log2r"]:.1f})' for r in rows]
        y = range(len(rows))
        s = [r["s"] for r in rows]
        err = [[r["s"] - r["lo"] for r in rows], [r["hi"] - r["s"] for r in rows]]
        colors = ["#2f6db3" if r["method"].startswith(("rho.", "bsgs.")) else "#c8553d" for r in rows]
        ax.barh(list(y), s, xerr=err, color=colors, alpha=0.85, capsize=2)
        ax.set_yticks(list(y))
        ax.set_yticklabels(labels, fontsize=6.5)
        ax.invert_yaxis()
        ax.set_xscale("log")
        ax.axvline(1.2533, color="#555", lw=0.8, ls="--")
        ax.set_title(f"{title} — session {rows[0]['session']}", fontsize=8.5)
        ax.set_xlabel("mean S = GAE / √r (log scale); bars: 95% interval as ecbench table records it", fontsize=7)
        ax.tick_params(axis="x", labelsize=7)
    fig.suptitle(
        "ecbench Jacobian workloads and transfers, 2026-10-25: mean S per arm and curve\n"
        "blue = generic method on the workload, red = index calculus or transfer; dashed: √(π/2) (plain rho); "
        "12 verified runs per bar",
        fontsize=8.5,
    )
    fig.tight_layout(rect=(0, 0, 1, 0.96))
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(OUT / f"s_by_arm.{ext}", **kw)


def figure_ratio():
    pairs = []
    for name, _ in FAMILIES:
        for f in sorted((HERE / "sessions" / name / "comparisons").glob("*.json")):
            c = json.loads(f.read_text())
            a, b = f.stem.split("__")
            if b.endswith("-aa"):
                continue
            ops = c["ops"]
            ratio = ops["ratio_b_over_a"]
            ci = ops["ci95"]
            pairs.append((f"{name}: {b} / {a}", ratio, ci))
    fig, ax = plt.subplots(figsize=(8.6, 3.8))
    y = range(len(pairs))
    ax.barh(
        list(y),
        [p[1] for p in pairs],
        xerr=[[p[1] - p[2][0] for p in pairs], [p[2][1] - p[1] for p in pairs]],
        # blue: interval below 1; red: above 1; grey: it covers 1
        color=["#2f6db3" if p[2][1] < 1 else "#c8553d" if p[2][0] > 1 else "#999999" for p in pairs],
        alpha=0.85,
        capsize=2,
    )
    ax.set_yticks(list(y))
    ax.set_yticklabels([p[0] for p in pairs], fontsize=7)
    ax.invert_yaxis()
    ax.set_xscale("log")
    ax.axvline(1.0, color="#555", lw=0.8)
    ax.set_xlabel("paired operation ratio to the reference arm at w = 0 (ecbench compare), 95% interval; log scale", fontsize=7)
    ax.set_title("Each candidate against its matched reference on the same workloads (distance to rho)", fontsize=8.5)
    ax.tick_params(axis="x", labelsize=7)
    fig.tight_layout()
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(OUT / f"ratio_to_reference.{ext}", **kw)


figure_s()
figure_ratio()
