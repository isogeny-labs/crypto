"""Render completed native replay and instruction-audit values only."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent.parent
preamble = r"""\documentclass[10pt]{article}
\usepackage[margin=16mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}
\begin{document}
"""
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})

def graph(fig, folder, name):
    fig.tight_layout()
    for ext in ("svg", "png", "pdf"):
        p = folder / (name + "." + ext)
        fig.savefig(p, bbox_inches="tight", dpi=150)
        if ext == "svg":
            p.write_text("\n".join(l.rstrip() for l in p.read_text().splitlines()) + "\n")
    plt.close(fig)

def report(folder, title, body):
    (folder / "report.tex").write_text(preamble + "\\section*{" + title + "}\n" + body + "\n\\end{document}\n")
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)

audit = json.loads((root / "pipeline-receipts/instruction-profiles-n13-20261025-01/receipt.json").read_text())
folder = root / "reports/run-instruction-profiles-n13-20261025-01"
folder.mkdir(parents=True, exist_ok=True)
rows = audit["rows"]
fig, ax = plt.subplots(figsize=(7.3, 3.05))
for offset, subset, label, color in [(-.18, rows[:5], "public target 0", "#147d92"),
                                      (.18, rows[5:], "public target 1", "#ac6146")]:
    ax.bar([i + offset for i in range(5)], [r["solve_ir"] for r in subset],
           width=.34, label=label, color=color)
ax.set_xticks(range(5), ["strong rho", "direct S4", "symmetric S4", "generic MITM", "rho repeat"])
ax.set_yscale("log")
ax.set_ylabel("complete-solve callgrind.Ir")
ax.legend(frameon=False, fontsize=8)
ax.grid(axis="y", alpha=.2)
graph(fig, folder, "complete-solve-ir")
table = "\n".join(f"{r['seq']} & {r['arm']} & {r['solve_ir']:,} & {r['solve_ir_over_same_target_rho']:.6g}" + r" \\" for r in rows)
body = rf"""Instruction attribution, 2026-10-25; all ten frozen n=13 smoke
executions are profiled, including both rho repeats. K0 over GF($2^{{13}}$),
prime subgroup r=2003, cofactor 4; exact curve, target and generator identities
remain in the sealed source session ECBS1hd3bc41fb2eb2. Two independent
one-target workloads, target seed 202610230101. Frozen binary SHA-256 is in
PROFILE_PROTOCOL.md (adapter build 0a5137c2).

\begin{{center}}\includegraphics[width=.98\linewidth]{{complete-solve-ir.pdf}}\end{{center}}
\begin{{center}}\begin{{tabular}}{{rlrr}}\toprule
Seq & arm & whole-solve Ir & Ir / same-target rho\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

All ten recovered scalars independently reproduce the recorded target by
generator multiplication. Every integer phase counter, native counter,
factor-base record and solver-word count matches the native run. Four rho
rows have a one-ULP serialized phase-price difference; the strict failed
check is retained, numerical prices use the explicit 1e-12 relative tolerance,
and per-row exactness flags remain visible. Six IC rows have bit-exact prices.
The final native audit reproduces byte for byte, including per-marker sums.

The native parser recovers a complete-solve interval for all ten profiles.
The four rho profiles also have target-only boundaries; the six IC profiles
do not. Their online intervals remain null, with exact parser refusals.
No IC online speed comparison is inferred. Per-marker attribution is in the
native receipt; the PDP marker contains construction, solving and splitting
together and does not separately price those operations.

Valgrind 3.24.0, one thread, local AMD EPYC 9V74 host, L0, 360 seconds per
child. These are user-space implementation instruction counts under the
Valgrind allocator. Native allocator work is absent; profile wall times are
diagnostics. No PMU or GPU is exposed. Ir ratios are not runtime speedups,
hardware calibration or complete GAE prices. No algorithmic improvement,
fastest-tool claim or leaderboard promotion is made from this profile.

Protocol: \path{{../../PROFILE_PROTOCOL.md}}. Native validator:
\path{{examples/ic_instruction_profile_audit.rs}}.
Receipt: \path{{../../pipeline-receipts/instruction-profiles-n13-20261025-01/receipt.json}}.
Raw archive under evidence/: \path{{runs-instruction-profiles-n13-20261025-01.tar.xz}}.
Publication of oversized artifacts remains blocked; unchanged bytes are local.
"""
report(folder, "n13 complete-solve instruction diagnostic", body)

name = "quartic-smoke-20261025-02"
folder = root / "reports" / ("run-" + name)
folder.mkdir(parents=True, exist_ok=True)
replay = json.loads((root / "pipeline-receipts" / (name + ".json")).read_text())
plan = json.loads((root / "pipeline-runs" / name / "plan.json").read_text())
session = json.loads((root / "pipeline-runs" / name / "session.json").read_text())
costs = json.loads((root / "pipeline-receipts/pricing-audit-20261025/receipt.json").read_text())
virtual = {v["curve"]: v["evidence"] for v in costs["quartic_virtual_calibrations"]}
ic = sorted([r for r in costs["rows"] if r["arm"] == "ic-frobenius"], key=lambda r: virtual[r["curve"]]["m"])
rho = {r["curve"]: r for r in costs["rows"] if r["arm"] == "rho-neg"}
fig, ax = plt.subplots(figsize=(7.3, 2.8))
ax.plot([virtual[r["curve"]]["m"] for r in ic], [r["raw_s_mean"] for r in ic], marker="o", label="quartic IC", color="#147d92")
ax.plot([virtual[r["curve"]]["m"] for r in ic], [rho[r["curve"]]["raw_s_mean"] for r in ic], marker="o", label="matched negation rho", color="#654b8c")
ax.set_xticks([6, 8, 10, 12])
ax.set_yscale("log")
ax.set_xlabel("base-field m; working field GF((2^m)^4)")
ax.set_ylabel("mean recorded S")
ax.legend(frameon=False, fontsize=8)
ax.grid(axis="y", alpha=.2)
graph(fig, folder, "quartic-smoke-costs")
curves = {}
for w in plan["workloads"]:
    curves[w["curve"]["slug"]] = w["curve"]
table = "\n".join(f"{virtual[r['curve']]['m']} & {curves[r['curve']]['r']} & {r['raw_s_mean']:.7g} & {rho[r['curve']]['raw_s_mean']:.7g}" + r" \\" for r in ic)
identities = "\\\\\n".join(r"\path{" + curves[r["curve"]]["icv1"] + "}" for r in ic)
body = rf"""Completed native smoke and independent replay, 2026-10-25.
Session \texttt{{{session['session_id']}}}: {replay['records']} records,
{replay['verified_records']} verified, 24/24 deterministic replays identical,
zero audit problems. Every native audit check passes. This completes this
four-rung correctness smoke, not the larger crossover campaign.

Base-field m=6,8,10,12, seed 1, max cofactor 16; two independently generated
public one-target workloads per rung. Arms: bin-t-line / gaudry-d4-bin with
Frobenius-reduced equations, same-point negation rho, and identical rho repeat.
One round, no warmup, alternate order, 900-second native child limit, L0.
The arbitrary coefficients include b outside the quadratic subfield;
proper extension structure used by IC does not make q-Frobenius an
automorphism of this same curve. Exact ICV1 identities:

{{\footnotesize {identities}}}

\begin{{center}}\includegraphics[width=.98\linewidth]{{quartic-smoke-costs.pdf}}\end{{center}}
\begin{{center}}\begin{{tabular}}{{rrrr}}\toprule
Base m & subgroup r & IC S, mean of 2 & rho S, mean of 2\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

Unit: recorded GAE / square root of r, using the adapter's deterministic
Fq-operation conversion and seeded lift probes. These count conventions
are not nanoseconds or measured hardware prices. Raw IC costs independently
reconstruct from integer counts. The canonical band refuses missing global
calibration rows; its exact refusal and separately labeled compatibility
and all-native scenarios are in the pricing audit. Word scans, physical
conversion and other complete-cost obligations remain unresolved. Two targets
give no population interval. No measured crossover or complete-price win is
inferred; historical m=14 and extrapolated crossover claims remain separate.

Spec, plan, host and sealed rows: \path{{../../pipeline-runs/{name}/}}.
Independent replay: \path{{../../pipeline-receipts/{name}.json}}.
Pricing: \path{{../run-pricing-audit-20261025/report.pdf}}.
Source: src/cryptanalysis/gaudry_quartic_bin.rs and native ecbench adapters.
m=14/15 full panels remain outstanding; m=16--18 need wider arithmetic.
The separate HKY continuation retains its memory failures. No leaderboard
or admitted benchmark row is promoted.
"""
report(folder, "Quartic IC: four-rung native replay", body)
print("Rendered completed quartic replay and instruction diagnostics.")
