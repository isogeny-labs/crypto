"""Render native audit receipts; no research arithmetic or experiments."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent.parent
PREAMBLE = r"""\documentclass[10pt]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}
\begin{document}
"""
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})

def graph(fig, folder, name):
    fig.tight_layout()
    for suffix in ("svg", "png", "pdf"):
        path = folder / (name + "." + suffix)
        fig.savefig(path, bbox_inches="tight", dpi=150)
        if suffix == "svg":
            path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
    plt.close(fig)

def report(folder, title, body):
    (folder / "report.tex").write_text(PREAMBLE + "\\section*{" + title + "}\n" + body + "\n\\end{document}\n")
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)

audit = json.loads((ROOT / "pipeline-receipts/fixture-group-audit-20261025/receipt.json").read_text())
folder = ROOT / "reports/run-fixture-group-audit-20261025"
folder.mkdir(parents=True, exist_ok=True)
draws = audit["draws"]
fig, ax = plt.subplots(figsize=(7.3, 3.0))
ax.bar([str(d["draw"]) for d in draws], [d["curve_abscissae_including_zero"] for d in draws],
       color=["#147d92" if d["target_curve_liftable"] else "#ac6146" for d in draws])
ax.set_ylim(0, 256)
ax.set_xlabel("original fixture draw (blue: target lifts; orange: target does not lift)")
ax.set_ylabel("curve abscissae in V (of 256)")
ax.grid(axis="y", alpha=.2)
graph(fig, folder, "curve-lifts")
table = "\n".join(f"{d['draw']} & {d['boolean_chain_solutions']} & " +
                  ("yes" if d["target_curve_liftable"] else "no") +
                  f" & {d['curve_abscissae_including_zero']} & {d['curve_points_including_two_torsion']}" + r" \\"
                  for d in draws)
body = rf"""Diagnostic date 2026-10-25. Source frozen at c470f5d51 before output.
The byte-identical original seven Boolean fixtures use GF($2^{{22}}$),
modulus $X^{{22}}+X+1$, and $E_0:y^2+xy=x^3+1$.
Exact input SHA-256 (two joined lines):\\
\texttt{{{audit['input_sha256'][:32]}}}\\
\texttt{{{audit['input_sha256'][32:]}}}.

\begin{{center}}\includegraphics[width=.98\linewidth]{{curve-lifts.pdf}}\end{{center}}
\begin{{center}}\begin{{tabular}}{{rrrrr}}\toprule
Draw & Boolean solutions & target on curve & curve x in V & curve points in V\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

The native audit finds {audit['unsat_curve_liftable']}/{audit['unsat_draws']}
UNSAT targets and {audit['sat_curve_liftable']}/{audit['sat_draws']} SAT targets
liftable. Draws 4 and 5 are not ordinary curve-query abscissae for this model.
Draw 0, used by the all-eight balanced k=3 replay, is liftable.
This qualifies the application of these fixtures; it does not invalidate their
finite Boolean-system solution counts or refutation observations.

Every returned y root is checked by direct curve-equation substitution.
An independently evaluated Artin--Schreier trace criterion agrees for every
target and every element of each complete 256-element V. Counts include
x=0 and its two-torsion point. Neither target nor factor-base subgroup
membership is established. Curve membership alone does not prove a signed
point decomposition, a degree law, a complete DLP solve or a speed result.

Execution: native Rust, local shared x86-64 host; 12 GiB address-space cap,
900-second limit, exit 0, no timing comparison. A separate exhaustive
small-field test compares every y value at n=5,6,7; all three example tests pass.
Protocol: \path{{../../FIXTURE_GROUP_PROTOCOL.md}}.
Source: \path{{examples/ic_fixture_group_audit.rs}}.
Raw receipt and provenance: \path{{../../pipeline-receipts/fixture-group-audit-20261025/}}.
The old branch reports and source-pinned inputs are preserved unchanged.
"""
report(folder, "Original fixtures: finite curve-membership audit", body)

audit = json.loads((ROOT / "pipeline-receipts/pricing-audit-20261025/receipt.json").read_text())
folder = ROOT / "reports/run-pricing-audit-20261025"
folder.mkdir(parents=True, exist_ok=True)
rows = audit["rows"]
virtual = {v["curve"]: v["evidence"] for v in audit["quartic_virtual_calibrations"]}
quartic_ic = sorted([r for r in rows if r["arm"] == "ic-frobenius"], key=lambda r: virtual[r["curve"]]["m"])
quartic_rho = {r["curve"]: r for r in rows if r["arm"] == "rho-neg"}
fig, ax = plt.subplots(figsize=(7.3, 3.2))
ax.plot([virtual[r["curve"]]["m"] for r in quartic_ic], [r["raw_s_mean"] for r in quartic_ic],
        marker="o", label="IC: original deterministic count convention", color="#147d92")
ax.plot([virtual[r["curve"]]["m"] for r in quartic_ic], [quartic_rho[r["curve"]]["raw_s_mean"] for r in quartic_ic],
        marker="o", label="same-target negation rho", color="#654b8c")
ax.set_xticks([6, 8, 10, 12])
ax.set_yscale("log")
ax.set_xlabel("base-field m; working field GF((2^m)^4)")
ax.set_ylabel("mean S (GAE / sqrt(r))")
ax.legend(fontsize=8, frameon=False)
ax.grid(axis="y", alpha=.2)
graph(fig, folder, "quartic-recorded-costs")
largest = quartic_ic[-1]
rho = quartic_rho[largest["curve"]]
fig, ax = plt.subplots(figsize=(7.3, 3.2))
for values, label, color in [
    (largest["compatibility_s_mean"], "IC: compatibility, Fq solver conversion fixed", "#147d92"),
    (largest["all_native_scenario_s_mean"], "IC: additional scenario, Fq solver operations varied", "#ac6146"),
    (rho["compatibility_s_mean"], "same-target negation rho", "#654b8c")]:
    ax.plot(range(3), values, marker="o", color=color, label=label)
ax.set_xticks(range(3), ["0", "0.1", "1"])
ax.set_yscale("log")
ax.set_xlabel("common GAE weight per recorded native operation")
ax.set_ylabel("mean S (GAE / sqrt(r))")
ax.legend(fontsize=8, frameon=False)
ax.grid(axis="y", alpha=.2)
graph(fig, folder, "quartic-accounting-scenarios")
table = "\n".join(f"{virtual[r['curve']]['m']} & {r['raw_s_mean']:.6g} & {quartic_rho[r['curve']]['raw_s_mean']:.6g}" + r" \\" for r in quartic_ic)
scenario = " / ".join(f"{x:.7g}" for x in largest["compatibility_s_mean"])
additional = " / ".join(f"{x:.7g}" for x in largest["all_native_scenario_s_mean"])
body = rf"""Post-outcome accounting audit, 2026-10-25, source frozen at db732b62b.
This explicitly corrects the earlier n=13/n=23 solver-wall-pricing description.
The native auditor reconstructs every available IC charge from integer
counts and fixed prices, checking the raw total and the solver component.
It also checks the ordinary band wherever that band is available.

The completed n=13 and n=23 structured solvers use pinned word rates,
0.002304 and 0.001164 GAE per word. The n=7 measured solver term is removed
by ic_report. Elapsed solver time is a diagnostic and contributes no wall
charge to these reported costs. Raw S remains incomplete: symbolic
construction and splitting are not fully counted. Original PDFs and
sealed data are preserved; corrected n13/n23 reports supersede their pricing
paragraph. Missing cost does not become zero through this correction.

\begin{{center}}\includegraphics[width=.98\linewidth]{{quartic-recorded-costs.pdf}}\end{{center}}
\begin{{center}}\begin{{tabular}}{{rrr}}\toprule
Base m & original IC S, mean of 2 & negation rho S, mean of 2\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

The quartic raw convention counts Fq solver operations and converts them
using a deterministic operations-per-addition ratio. Successful-lift prices
are reconstructed from 32 probes with seed 0x5e. Public constructor inputs,
order, subgroup and generator coordinates are independently checked against
each raw record. The virtual ratios are count conventions, not measured
nanoseconds or hardware calibration. Both rho repeats retain exactly equal
deterministic counts. These are two known-answer one-target workloads per
rung at L0; no population interval or physical crossover is established.
Scalar replay of the smoke is separate from this accounting audit.

\newpage
\section*{{Quartic band scope and retained refusal}}
The current canonical ecbench band refuses these constructed curves because
their adapter-created virtual calibration rows were not persisted in the
global table. The exact command, output and status are retained beside this
audit. No global hardware calibration or sealed record is rewritten.

\begin{{center}}\includegraphics[width=.98\linewidth]{{quartic-accounting-scenarios.pdf}}\end{{center}}

At m=12, compatibility means at weights 0/0.1/1 are {scenario}.
This reconstructs the existing band's arithmetic and holds the counted Fq
solver conversion fixed, as that implementation does. The additional
all-native scenario also varies those Fq solver operations; its means are
{additional}. This is a separately labeled accounting scenario, not an
algorithm change, a speedup or a redefinition of old S. All means require
every planned target to verify; censored or failed arms have null means.

The complete-price obligation remains open for construction, splitting,
word scans and physical conversion. The graph at weight zero must not be
used to claim an IC win. No result is promoted to the performance leaderboard
or admitted benchmark table. The separate structural scoreboard panel changes
only its incorrect pricing description; historical performance plots are
unchanged because this audit adds no algorithmic improvement.

Raw evidence: \path{{../../pipeline-receipts/pricing-audit-20261025/receipt.json}}.
Protocol: \path{{../../PRICING_AUDIT_PROTOCOL.md}}.
Native source: \path{{examples/ic_campaign_pricing_audit.rs}}.
Reproduction command and byte-comparison record, source/input/binary hashes,
limits and exit status are beside the receipt. Sealed input sessions:
\path{{../../pipeline-runs/structured-n7-20261025-01/}}, n13, n23,
and \path{{../../pipeline-runs/quartic-smoke-20261025-02/}}.
"""
report(folder, "Independent native pricing reconstruction", body)
print("Rendered native curve-membership and pricing audit receipts.")
