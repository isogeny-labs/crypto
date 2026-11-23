"""Render sealed session metadata and native ecbench band values only."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent.parent
PREAMBLE = r"""\documentclass[10pt]{article}
\usepackage[margin=15mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{3pt}
\begin{document}
"""
COLORS = {"ic-direct": "#ac6146", "ic-symmetric": "#147d92",
          "ic-pair-table": "#61727c", "rho-strong": "#654b8c"}
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})

def pdf(folder, title, body):
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "report.tex").write_text(PREAMBLE + r"\section*{" + title + "}\n" + body + "\n\\end{document}\n")
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"], cwd=folder,
                       stdout=log, stderr=subprocess.STDOUT, check=True)

for n in (13, 23):
    name = f"structured-n{n}-20261025-01"
    session = json.loads((ROOT / "pipeline-runs" / name / "session.json").read_text())
    plan = json.loads((ROOT / "pipeline-runs" / name / "plan.json").read_text())
    audit = json.loads((ROOT / "pipeline-receipts" / (name + ".json")).read_text())
    band = json.loads((ROOT / "pipeline-receipts" / f"structured-n{n}-band.json").read_text())
    folder = ROOT / "reports" / ("run-" + name + "-pricing-correction")
    folder.mkdir(parents=True, exist_ok=True)
    fig, ax = plt.subplots(figsize=(7.3, 3.15))
    for row in band["rows"]:
        if row["arm"].endswith("-aa") or row["verified"] != row["runs"]:
            continue
        ax.plot(range(3), row["mean_s"], marker="o", label=row["arm"], color=COLORS[row["arm"]])
    ax.set_xticks(range(3), ["0", "0.1", "1"])
    ax.set_yscale("log")
    ax.set_xlabel("common GAE weight per recorded native operation")
    ax.set_ylabel("mean recorded S (GAE / square root of r)")
    ax.legend(fontsize=8, frameon=False, loc="lower center", bbox_to_anchor=(.5, 1.02), ncol=2)
    ax.grid(axis="y", alpha=.2)
    fig.tight_layout()
    for suffix in ("svg", "pdf", "png"):
        fig.savefig(folder / ("native-band." + suffix), bbox_inches="tight", dpi=150)
        if suffix == "svg":
            path = folder / "native-band.svg"
            path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
    plt.close(fig)
    table = "\n".join(row["arm"] + f" & {row['verified']}/{row['runs']} & " +
                      (" / ".join(f"{v:.4g}" for v in row["mean_s"])
                       if row["verified"] == row["runs"] else "incomplete; not compared") + r" \\"
                      for row in band["rows"])
    c = plan["workloads"][0]["curve"]
    verdict = ("All ten recorded answers verify and all ten deterministic replays reproduce."
               if n == 13 else "Nine answers verify and reproduce; one direct run timed out. Its missing cost is not zero. The direct arm is excluded from the complete-arm mean plot.")
    body = rf"""Pricing correction: this supersedes the earlier report's pricing paragraph. Raw records and numerical bands are unchanged.

Report date 2026-10-25; session \texttt{{{session['session_id']}}}.
{verdict}

Exact curve: \path{{{c['icv1']}}}. Prime subgroup r={c['r']},
cofactor {c['cofactor']}, generator ({c['generator'][0]},{c['generator'][1]}).
Two public known-answer one-target workloads, target seed 202610230101;
one round, no warmup, alternate order, per-child timeout 900 seconds.
Shared x86-64 host earns L0; elapsed times are diagnostic.

Arms: strong signed-Frobenius rho and its identical repeat; direct S4;
symmetric product-space S4 with degree-two closure; generic three-summand
MITM with negation folding. Every IC arm uses the same geometric factor
base, seed 17. Frozen complete parameters are in the session spec/plan.
The direct hybrid extraction has an input-degree floor, up to six.

\begin{{center}}\includegraphics[width=.98\linewidth]{{native-band.pdf}}\end{{center}}

\begin{{center}}\begin{{tabular}}{{lrl}}\toprule
Arm & verified/planned & recorded S at weights 0 / 0.1 / 1\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

The graph quotes native ecbench band output, which re-prices recorded
group and native counters. It omits incompletely counted construction and
cubic splitting, and is not a complete-price comparison. No population
interval is claimed from two targets. The earlier report incorrectly said
raw S contains solver wall time. Actual completed n13/n23 solver charges
use pinned word rates. The measured n7 solver term is removed. Native
ic_campaign_pricing_audit reconstructs the available IC costs from integer
counts and fixed prices, matching the raw records. Raw S is a fixed-price
estimate of counted work, with the omissions above. The raw lower-bound flag does not mark these
omissions. Automated pair ranks do not repair missing costs or censored runs.

Source pushed through a5ff7036; the sealed records pin the actual binary.
The independent native audit reports {audit['records']} records,
{audit['verified_records']} verified, and {len(audit['replays'])} reproduced
replays, zero audit problems. Same-host replay checks correctness and
deterministic counts, not independent-host physical performance.

Raw sealed directory: \path{{../../pipeline-runs/{name}/}}.
Native replay: \path{{../../pipeline-receipts/{name}.json}};
native band: \path{{../../pipeline-receipts/structured-n{n}-band.json}}.
Plugin source: src/bin/ecbench/structured_s4.rs and structured_s4_math.rs.
All original failures remain recorded. No full IC gain or rho crossover
is promoted to the leaderboard or benchmark results table.
"""
    pdf(folder, f"Complete-DLP smoke: K0 over GF($2^{{{n}}}$)", body)


print("Rendered explicitly corrected n13/n23 pricing reports; originals untouched.")
