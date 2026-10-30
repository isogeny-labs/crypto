"""Lay out recorded native CAS correctness receipts only."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent.parent
data = json.loads((root / "pipeline-receipts/external-cas-strengthened-20261025/replay.json").read_text())
rows = [r for r in data["results"] if r["algorithm"] == "std"]
folder = root / "reports/run-external-cas-20261025-01"
folder.mkdir(parents=True, exist_ok=True)
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(7.3, 3.1))
bars = ax.bar(range(len(rows)), [r["boolean_roots"] for r in rows],
              color=["#ac6146" if r["case"] in ("case-20", "case-21") else "#147d92" for r in rows])
ax.bar_label(bars, padding=3, fontsize=8)
ax.set_xticks(range(len(rows)), [r["case"].removeprefix("case-") for r in rows], fontsize=8)
ax.set_ylim(0, 17)
ax.set_ylabel("Boolean roots")
ax.set_xlabel("case index; 20 and 21 are constructed positive controls")
ax.grid(axis="y", alpha=.2)
fig.tight_layout()
for suffix in ("svg", "png", "pdf"):
    p = folder / ("boolean-roots." + suffix)
    fig.savefig(p, bbox_inches="tight", dpi=150)
    if suffix == "svg":
        p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
plt.close(fig)
body = rf"""\documentclass[10pt]{{article}}
\usepackage[margin=17mm]{{geometry}}
\usepackage{{graphicx,booktabs,hyperref}}
\usepackage[strings]{{underscore}}
\setlength{{\parindent}}{{0pt}}\setlength{{\parskip}}{{5pt}}
\begin{{document}}
\section*{{Same-input Singular correctness baseline}}
Local run 2026-10-25; source and protocol frozen at fa39bae96 before export.
Singular 4.3.1-p3, Debian 1:4.3.1-p3+ds-2, installed in a workspace prefix
from hash-checked official Debian packages. No package binary is committed.

All {data['planned_external_runs']} external commands finish, with
{data['failed_or_incomplete']} failed or incomplete outputs. For each of
{data['cases']} cases, both std and slimgb produce a basis with exactly the
same complete Boolean zero set as independent native enumeration and native
inherited-F4 root extraction. The CAS also reduces every original generator
to zero against its returned basis. The strengthened Rust validator rebuilds
the equations and their hashes, re-enumerates their zero set, checks the exact
exported input ideal and compares every external output. It reproduces the
original receipt byte for byte.

\begin{{center}}\includegraphics[width=.98\linewidth]{{boolean-roots.pdf}}\end{{center}}

Twenty source cases come from the frozen degree-two structural panel:
n=13,19, ell=2, all four basis families, replicate 0, target 0;
also ell=3 geometric bases at both n. Both eliminated S4 and symmetric-e
formulations retain the original equation hashes and complete root counts.
These are different variable layouts; root counts across formulations need
not be equal. The two additional positive algebraic controls set x_r to
the first n=13 prefix basis element. They are intentionally constructed
controls, with no claim to ordinary subgroup-query membership.

Singular uses characteristic 2, dp order, option(redSB), with all Boolean
field equations $x_i^2+x_i$ explicit. Each external command is single-threaded,
limited to 60 seconds and 1 GiB address space; the shared x86-64 host earns
L0. Resource times are retained only as diagnostics. This finite zero-set
check is not a general formal Gr\"obner proof. A basis computation and root
extraction also have different output contracts, so no speed ratio is claimed.

Magma is not licensed here; msolve, FGb and fastest-tool comparisons remain
open. This check does not qualify physical performance, larger guessing
cells, complete DLP costs or a rho crossover. The structural ratio figures,
leaderboard and progress series remain unchanged because this run adds
correctness coverage, not an algorithmic speed result.

Protocol: \path{{../../EXTERNAL_CAS_PROTOCOL.md}}.
Native exporter/validator: \path{{examples/ic_cas_correctness.rs}}.
Raw cases, generated scripts, package/version/host provenance, all resource
and exit records, under evidence/:\\
\path{{runs-external-cas-20261025-01.tar.xz}}.
Independent receipt: \path{{../../pipeline-receipts/external-cas-strengthened-20261025/replay.json}}.
Reproduction: \path{{../../verify-external-cas.sh}}.
\end{{document}}
"""
(folder / "report.tex").write_text(body)
with (folder / "render.log").open("w") as log:
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                   cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)
print("Rendered external CAS correctness report.")
