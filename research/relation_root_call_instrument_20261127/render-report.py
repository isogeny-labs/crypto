"""Render recorded Rust/configuration values only; no experiment arithmetic."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

s = Path(__file__).resolve().parent
outcome = json.loads((s / "OUTCOME.json").read_text())
plan = json.loads((s / "PLAN_V2.json").read_text())
transport = outcome["transport"]
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10})
fig, ax = plt.subplots(figsize=(8.0, 2.7), layout="constrained")
names = ["Earlier generic / packed", "Earlier flat candidates", "Current root exhaustive", "Current root pair graph"]
values = [36, 12, plan["current_root_exhaustive_controls"], plan["current_root_pairgraph_candidates"]]
colors = ["#8493a3", "#8493a3", "#278c83", "#6554c0"]
ax.barh(names, values, color=colors)
for y, value in enumerate(values):
    ax.text(value + .5, y, str(value), va="center")
ax.invert_yaxis(); ax.set_xlim(0, 42); ax.set_xlabel("Declared arms, including both launchers")
ax.set_title("Future A/B roster: every earlier arm remains an eligible control", loc="left", fontsize=11)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(s / "control-roster.pdf"); fig.savefig(s / "control-roster.svg"); plt.close(fig)
fig, ax = plt.subplots(figsize=(8.0, 3.0), layout="constrained")
ax.set_xlim(-.15, 10.1); ax.set_ylim(0, 4); ax.axis("off")
blocks = [
    (0, 2.1, "Parent starts call clock\nCapture / command setup\nCapped child launch"),
    (3.5, 2.1, "Child reads / decodes System\nRoot API: construct + wire\nOwn replay + native check"),
    (7.0, 2.1, "Extra worker proof readback\nWrite original + witness\nReturn stdout / process"),
    (3.5, .1, "Parent receipt + proof readback\nFull replay, basis/stats/hash binding\nParent stops call clock"),
]
for x, y, label in blocks:
    ax.add_patch(FancyBboxPatch((x, y), 2.9, 1.45, boxstyle="round,pad=0.035", facecolor="#eef2f6", edgecolor="#8493a3"))
    ax.text(x + 1.45, y + .725, label, ha="center", va="center", fontsize=9)
for start, end in [((2.95, 2.82), (3.4, 2.82)), ((6.45, 2.82), (6.9, 2.82)), ((8.45, 2.0), (6.5, .84))]:
    ax.annotate("", xy=end, xytext=start, arrowprops={"arrowstyle": "->", "color": "#4a5968"})
fig.savefig(s / "call-boundary.pdf"); fig.savefig(s / "call-boundary.svg"); plt.close(fig)
text = r'''\documentclass[10pt,a4paper]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Full-call relation instrument: qualification}\\[3pt]
{\small Rust, 27 November 2026. PR \#5251. No measured speedup.}\\[3pt]
{\small $p=2147483647,9007199254740881$; $n=3,4,6$; seed7 LAST-star.}
\paragraph{Result.} The actual root worker and parent transport path pass all 432 fixed-input calls over both launchers, with 432 complete parent proof replays and 1728 deliberate corruptions rejected. Qualification creates no worker performance clocks. All 33 instrument unit functions pass. These are unsigned same-host results, not signed execution attestations. [pcc:unverified]
\begin{center}\includegraphics[width=\linewidth]{control-roster.pdf}\end{center}
\noindent The next draft keeps all 48 previous generic/packed/flat arms as eligible controls and adds 36 current-root exhaustive arms. The 36 pair-graph candidates receive identical forms, kernels, launchers and extra readback. The primary is fixed as root-pairgraph-flat-dag-487-native before any performance observations. The design has 168 A/A arms and 120 A/B arms, five rounds and the same six inputs: 8640 future calls, none collected.
\paragraph{Analysis controls.} Two explicitly synthetic cohorts each complete 8640 whole-proof replays. Every clock and monitor flag in these cohorts is invented. Neutral threshold arithmetic is false. Hypothetical 20$\times$ clocks pass threshold arithmetic, while admission remains false. Copied clock-receipt and duplicate-cell sabotage views are both refused with no report. All 51863 neutral-source file hashes remain unchanged. [pcc:unverified]
\begin{center}\begin{tabular}{lrrr}\toprule
Synthetic scenario & Whole proofs & Arithmetic passes & Admission\\\midrule
Neutral &8640 &false &false\\
Hypothetical 20$\times$ &8640 &true &false\\\bottomrule
\end{tabular}\end{center}
\noindent No synthetic number is a host timing, A/A measurement, contention observation or performance result. The analyzer's admission is explicitly false while canonical protocol validation is pending. No 15$\times$ result is established. [pcc:unverified]
\newpage
\noindent{\Large\bfseries Boundary, custody and remaining work}
\begin{center}\includegraphics[width=\linewidth]{call-boundary.pdf}\end{center}
\noindent The entire existing parent-call boundary is retained. Case-directory creation and final outcome/report writes remain outside that frozen boundary. Root input decoding, API setup, graph/exhaustive native certification, proof transport and parent full readback remain inside. The coarse root construct phase includes the API's own wire replay; an additional complete worker readback is visible and charged. No root work is amortized or substituted for another proof obligation.
\paragraph{Source custody.} SOURCE\_V2 pins the corrected instrument and unchanged production library. Initial workspace/lock setup and the first compiler failure are retained. System decoding exposes usize exponents; the correction uses fallible u32 conversion. Every registry version/checksum from the prior native lock is preserved. Historical 365/366/372 source and existing evidence are not rewritten. Local workers are copied read-only and hashed before calls. Complete transport, synthetic and corruption-view trees plus immutable workers are archived.
\paragraph{Scope.} Correctness builds use opt0, LTO/debug/incremental off, codegen16, default-features=false, CPU0/jobs1 and one numerical thread. This is not an optimized performance qualification. Prior whole-library libtest allocation failures remain historical failures; this component does not turn them into a whole-library pass. The relation API constructs and certifies a basis; it does not invoke the regular F4 solve. Intrinsic regularity and actual F4 solving degree remain unknown.
\paragraph{CI.} On published head 8d3a59ff303, both native Rust checks and both gate jobs pass. Artifact migration fails PutObject with InvalidAccessKeyId. No key, gate, artifact-store configuration or credential policy is changed. This snapshot is not an all-workflow pass. [pcc:unverified]
\paragraph{Next acceptance.} Original 15$\times$ acceptance is unchanged: prospective full-call timing against the strongest matched controls, both primes, all six fixed cases, repeated/A/A measurements, every proof/failure and published validated evidence. Canonical timing instructions remain encrypted; the separate required-document condition is not waived. Conductor and checkpoint/S3 access have already been waived. No comparative timing or new PCC claim is produced here.
\paragraph{Reproduce.} README and REPRODUCE describe fresh-path builds, no-clock transport, complete synthetic replay and corruption views. Report figures only lay out recorded Rust/configuration values. OUTCOME, source manifests, pre-call worker hashes, raw logs, archives and custody receipts preserve the exact scope.
\end{document}
'''
(s / "report.tex").write_text(text)
subprocess.run(["xelatex", "-interaction=nonstopmode", "-halt-on-error", "-output-directory", str(s), str(s / "report.tex")], cwd=s, check=True,
               stdout=(s / "raw/report-build-2.stdout").open("x"), stderr=(s / "raw/report-build-2.stderr").open("x"))
