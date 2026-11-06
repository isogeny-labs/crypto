"""Render recorded native values only; no cryptographic result generation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
RAW = HERE.parent / "runs"
plt.rcParams.update({"font.size": 10, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

def read(name):
    return json.loads((RAW / name).read_text())

def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)

field = read("cyclotomic-n83.json")
presentation = read("presentation-cert.json")
fig, axes = plt.subplots(1, 2, figsize=(10, 3.8))
axes[0].bar(["C1", "C2"], [field["c1_cardinality"], field["c2_cardinality"]], color="#327294")
axes[0].set_yscale("log")
axes[0].set_ylabel("Distinct field elements (log scale)")
axes[0].set_title("GF(2^83): recorded factor-base candidates")
for i, value in enumerate([field["c1_cardinality"], field["c2_cardinality"]]):
    axes[0].text(i, value, f"{value:,}", ha="center", va="bottom")
panels = presentation["panels"]
axes[1].bar([f"m={p['m']}, ell={p['ell']}" for p in panels],
            [p["ordered_tuple_checks"] for p in panels], color="#508366")
axes[1].set_ylabel("Exhaustive ordered tuple checks")
axes[1].set_title("GF(2^7): coordinate transport certificate")
for i, p in enumerate(panels):
    axes[1].text(i, p["ordered_tuple_checks"], str(p["ordered_tuple_checks"]), ha="center", va="bottom")
fig.suptitle("Component certificates, 2026-10-18; no solving-degree or cost claim")
save(fig, "components")

if (RAW / "panels-summary.json").exists():
    summary = read("panels-summary.json")["panels"]
    fig, ax = plt.subplots(figsize=(9, 3.4))
    labels = ["Step linear-consequence gate", "Oracle exact-root-set gate"]
    ax.barh(labels, [p["gate_true"] for p in summary], color="#508366", label="Gate true")
    ax.barh(labels, [p["gate_false"] for p in summary],
            left=[p["gate_true"] for p in summary], color="#b36646", label="Gate false")
    ax.set_xlabel("Targets (the two panels test different properties)")
    ax.set_title("2026-10-18 exact panels: L0 shared host; no timing comparison")
    ax.legend(loc="lower right")
    save(fig, "gates")

if (RAW / "unprojected-anf.json").exists() and (RAW / "cyclotomic-n83-anf.json").exists():
    addition = read("unprojected-anf.json")
    membership = read("cyclotomic-n83-anf.json")
    charts = addition["charts"]
    names = ["Generic addition, n=7", "Doubling, n=7", "C2 membership, n=83"]
    variables = [c["boolean_variables"] for c in charts] + [membership["boolean_variables"]]
    equations = [c["equations"] for c in charts] + [membership["boolean_equations"]]
    fig, ax = plt.subplots(figsize=(9, 3.8))
    positions = list(range(3))
    ax.bar([i - 0.18 for i in positions], variables, 0.36, label="Boolean variables", color="#327294")
    ax.bar([i + 0.18 for i in positions], equations, 0.36, label="Boolean equations", color="#bc913d")
    ax.set_xticks(positions, names)
    ax.set_ylabel("Recorded input size (log scale)")
    ax.set_yscale("log")
    ax.set_title("2026-10-18 quadratic inputs; F4 solving degree unmeasured")
    ax.legend()
    save(fig, "encodings")

if (RAW / "factorial-release-summary-v2.json").exists():
    factorial = read("factorial-release-summary-v2.json")
    arms = ["neither", "2lp-only", "d4-only", "d4-2lp", "d4-eager-incumbent", "d4-aa", "rho-negation"]
    curves = factorial["by_curve"]
    primes = list(curves)
    matrix = [[next(c["correct"] for c in curves[p] if c["arm"] == arm) for p in primes] for arm in arms]
    fig, ax = plt.subplots(figsize=(9, 4.5))
    ax.imshow(matrix, cmap="Blues", vmin=0, vmax=8, aspect="auto")
    ax.set_xticks(range(len(primes)), [f"constructor p={p}" for p in primes])
    ax.set_yticks(range(len(arms)), arms)
    for i, row in enumerate(matrix):
        for j, value in enumerate(row):
            ax.text(j, i, f"{value}/8", ha="center", va="center", color="white" if value > 4 else "black")
    ax.set_title("Verified targets: release ECBS1h328da50bcef9 / current ECBS1hfff2bdffb6cf")
    ax.set_xlabel("Each: 152 verified, 72 exhausted; all 224 deterministic payloads match; no cost ranking")
    save(fig, "factorial-outcomes")
