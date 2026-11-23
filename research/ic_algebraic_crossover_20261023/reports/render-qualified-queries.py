"""Plot retained native query/replay/closure receipts, without research calculations."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study=Path(__file__).resolve().parent.parent
query=json.loads((study/"pipeline-receipts/qualified-queries-20261025/summary.json").read_text())
closure=json.loads((study/"pipeline-receipts/qualified-closure-replay-20261025/summary.json").read_text())
assert query["rows"]==2048 and query["independent_point_arithmetic_rows_verified"]==2048
assert all(g["independent_hky_verified"]==g["attempts"] for g in closure["groups"])
plt.rcParams.update({"font.size":9,"svg.fonttype":"none"})

def save(fig,folder,stem):
    folder.mkdir(exist_ok=True)
    for ext in ("svg","png","pdf"):
        p=folder/(stem+"."+ext)
        fig.savefig(p,bbox_inches="tight",dpi=150)
        if ext=="svg":p.write_text("\n".join(l.rstrip() for l in p.read_text().splitlines())+"\n")
    plt.close(fig)

def render(folder,title,body):
    (folder/"report.tex").write_text(r"""\documentclass[10pt]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\section*{"""+title+"}\n"+body+"\n\\end{document}\n")
    with (folder/"render.log").open("w") as log:
        subprocess.run(["pdflatex","-interaction=nonstopmode","-halt-on-error","report.tex"],cwd=folder,stdout=log,stderr=subprocess.STDOUT,check=True)

folder=study/"reports/run-qualified-queries-20261025"
units=query["units"]
fig,ax=plt.subplots(figsize=(8.4,4.2))
ys=list(range(len(units)))
ax.barh([y-.17 for y in ys],[r["boolean_sat"] for r in units],height=.34,color="#ac6146",label="finite-chain SAT")
ax.barh([y+.17 for y in ys],[r["point_sat"] for r in units],height=.34,color="#167d8d",label="actual subgroup decomposition")
ax.set_yticks(ys,[f"({r['m']},{r['n']},{r['ell']}) {r['family']}" for r in units],fontsize=8)
ax.invert_yaxis();ax.set_xlim(0,139);ax.set_xlabel("queries out of 128 per natural cell/family file")
ax.legend(frameon=False,fontsize=8,loc="upper center",bbox_to_anchor=(.5,1.08),ncol=2)
ax.spines[["top","right"]].set_visible(False)
fig.tight_layout();save(fig,folder,"qualified-classes")
models="\n".join(f"{r['n']} & {r['b']} & {r['r']} & {r['cofactor']} & "+r"\texttt{"+r["record_sha256"][:12]+"}"+r" \\" for r in query["models"])
render(folder,"Qualified public queries: Boolean roots and actual points",rf"""
Native finite audit, 2026-10-25. All 2,048 planned rows are present in sixteen
128-query files. All finite-chain counts replay and all ordered signed point
counts independently agree in BinaryCurve/F2mElement arithmetic. The group
MITM combinatorial logic is shared, with separate exhaustive cyclic controls.
The independent arithmetic checks include lifts, actual bases, subgroup
membership, exact counts, witnesses and the predeclared selection.

\begin{{center}}\includegraphics[width=\linewidth]{{qualified-classes.pdf}}\end{{center}}
\textbf{{Observed:}} 1,591 finite-chain SAT queries; 161 actual subgroup-SAT
queries, all among those 1,591. These are different output contracts. The
finite chain allows algebraic roots without an all-summands-in-G point relation
and has no infinity-intermediate variable. No identity between its root count
and the ordered signed point count is assumed. Empty bases and direct hits
remain in the full data, alongside quota shortages and duplicates.

\begin{{center}}\begin{{tabular}}{{rrrrl}}\toprule
n & b (integer) & Prime r & h & Curve-record SHA prefix\\\midrule
{models}
\bottomrule\end{{tabular}}\end{{center}}
All four selected models have a=0, full-field-degree b, exact factory point
counts, coprime r/h, and an explicitly checked nonzero generator of order r.
Target seed 202610250001: hash a curve identity and (m,n,ell,draw,attempt),
lift a uniform x, select a hashed sign, clear h, reject O and check rR=O.
The 512 underlying targets are paired across four families. Basis seed
202610250003 uses the separately frozen domain-separated encoding. No target
scalar is supplied by this constructor. Curve construction and each file
use 2-GiB/900-second caps; all generation and both replay streams exit zero.

Producer source 3cabc768f, frozen binary SHA prefix 9c38ff6e56ba; finite
replay binary prefix 4e7a01970e62; separate point-audit source a2c37d7bb.
Shared AMD EPYC 9V74 Linux x86-64 host, one Rayon thread, L0. Timings and
point-oracle operations are diagnostics, not complete query prices.

\textbf{{Scope.}} These are additional controls selected by an explicit a=0,
full-degree-b, cofactor-at-most-16 constructor rule, not a generic-curve
population result or a same-instance speed comparison with old b=1 fixtures.
No decomposition search cost, global minimum-guess law or complete DLP claim.
The original 1,159 reference/twist-audited fixtures remain unchanged.
Protocols: BINARY_QUERY_LAW_PROTOCOL.md and QUALIFIED_POINT_REPLAY_PROTOCOL.md.
Native receipt and all input hashes:
\path{{../../pipeline-receipts/qualified-queries-20261025/summary.json}}.
Raw tree is retained locally pending artifact-store upload; no upload claimed.
""")

folder=study/"reports/run-qualified-closures-20261025"
groups=closure["groups"]
fig,axes=plt.subplots(1,2,figsize=(8.5,2.7))
labels=[f"D{g['degree']} "+("SAT control" if g["sat_control"] else "UNSAT") for g in groups]
axes[0].barh(labels,[g["nonrefuting"] for g in groups],color=["#167d8d" if not g["sat_control"] else "#ac6146" for g in groups])
axes[0].invert_yaxis();axes[0].set_xlim(0,65);axes[0].set_xlabel("completed nonrefuting fixed points")
axes[0].set_title("Independent HKY rank/verdict agreement")
axes[1].barh(labels,[g["all_solver_words"] for g in groups],color=["#167d8d" if not g["sat_control"] else "#ac6146" for g in groups])
axes[1].invert_yaxis();axes[1].set_xscale("log");axes[1].set_xlabel("sum of recorded native solver-word operations")
axes[1].set_title("Different class totals; no ratio or full price")
for ax in axes:ax.spines[["top","right"]].set_visible(False)
fig.tight_layout();save(fig,folder,"qualified-closure-coverage")
table="\n".join(f"{g['degree']} & "+("SAT" if g["sat_control"] else "UNSAT")+f" & {g['attempts']} & {g['nonrefuting']} & {g['independent_hky_verified']} & {g['all_solver_words']:,}"+r" \\" for g in groups)
render(folder,"Qualified zero-guess closure: degrees two and three",rf"""
Native additional panel, 2026-10-25. The frozen first-four-negative and
first-two-positive rules select 55 negative and 24 positive queries from
the complete qualified fixture audit. Nine negative and eight positive
quota slots are unfilled; no extra draws replace them. Every selected query
runs both degree caps, giving all 158 planned attempts, no caps or failures.
All 158 saturated verdicts and ranks agree with the independent HKY engine.

\begin{{center}}\includegraphics[width=\linewidth]{{qualified-closure-coverage.pdf}}\end{{center}}
\begin{{center}}\small\begin{{tabular}}{{rlrrrr}}\toprule
D & Class & Attempts & Nonrefuting & HKY checked & Solver words\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}
Every negative is finite-chain UNSAT and actual-subgroup UNSAT, has at least
two base points, is not a direct base hit, and is distinct in its file.
Every positive has both counts positive. The b-aware solver requires an
explicit SAT-control flag, and rejects any positive-control refutation.
Exact library and u128 polynomial equation builders agree before HKY replay.
The HKY engine uses independent monomial indexing, rounds and elimination;
its verdict/rank audit does not independently recount the producer's costs.

Producer source 35d265fc3, frozen binary SHA prefix 0b6c79f5a5df. Replay
source b98122f0b. Each process: one Rayon thread, column cap 40,000,
2-GiB address-space and 180-second cap. HKY_CHUNK=1024 for replay. Shared
AMD EPYC 9V74 Linux x86-64 host, L0. Diagnostic wall seconds are unqualified.

\textbf{{Limits.}} Nonrefuting closure does not prove SAT and does not prove
a degree law. This additional panel does not execute degree four or determine
the global minimum number of guesses. It does not discharge the original
b=1 all-branch campaign or larger-host shards. Construction, splitting,
point verification and native physical prices remain absent; the solver-word
totals are finite-stage counts, not complete query or DLP costs.
No cross-family performance ratio is formed because their selected queries
and bases differ. Original failed/capped runs are preserved separately.

Protocols: QUALIFIED_CLOSURE_PROTOCOL.md and QUALIFIED_CLOSURE_REPLAY_PROTOCOL.md.
Native source of this table/graph:
\path{{../../pipeline-receipts/qualified-closure-replay-20261025/summary.json}}.
All process records remain in the two separate raw run trees. No additional
paid VM allocation or artifact-store upload is claimed.
""")
print("Rendered two qualified-query/closure reports; inspect both PDFs before registration.")
