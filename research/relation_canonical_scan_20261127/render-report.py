"""Render recorded Rust outcomes and metadata only; no experiment arithmetic."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

s=Path(__file__).resolve().parent
out=json.loads((s/"OUTCOME.json").read_text())
fig,ax=plt.subplots(figsize=(8.0,2.55),layout="constrained")
ax.set_xlim(-.15,10.15);ax.set_ylim(0,3.8);ax.axis("off")
for x,y,w,h,label,color in [
    (0,1.7,3,1.6,"Nonempty, leading coefficient 1\nAccounting active?\nUnsupported prime / shape?","#eef2f6"),
    (6.1,2.05,3.8,1.05,"Yes: original normalization predicate\nSame accounting convention","#eef2f6"),
    (6.1,.15,3.8,1.05,"No: read-only coefficient/order scan\nNo clone, sort or rescaling","#dceeea")]:
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle="round,pad=.035",facecolor=color,edgecolor="#8493a3"))
    ax.text(x+w/2,y+h/2,label,ha="center",va="center",fontsize=9)
for a,b in [((3.05,2.65),(6.0,2.58)),((3.05,2.1),(6.0,.7))]:ax.annotate("",xy=b,xytext=a,arrowprops={"arrowstyle":"->","color":"#4a5968"})
fig.savefig(s/"canonical-check.pdf");fig.savefig(s/"canonical-check.svg");plt.close(fig)
text=r'''\documentclass[10pt,a4paper]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Canonical-row scan: production integration}\\[3pt]
{\small Rust, 27 November 2026. PR \#5251. Runtime benefit unmeasured.}
\paragraph{Change.} Both native certificate checkers now use a read-only row-form scan on the bounded uncounted path. The same comparator checks strict order; coefficients must be canonical/nonzero. Empty or nonmonic rejection, input reductions, all pair/graph obligations and original-ideal proof checks remain. Active Visit or bitop accounting and unsupported shapes keep the original normalization path. The public normalizer is unchanged.
\begin{center}\includegraphics[width=\linewidth]{canonical-check.pdf}\end{center}
\noindent Fast domain: primes2147483647/9007199254740881, uniform3..16-variable rows, exponents$\leq64$. No new selection seed or performance input is introduced.
\paragraph{Qualification.} Three candidate unit functions pass on15804 differential rows against actual production normalization plus branch/fallback tests. A fourth unit imports the live production file directly and passes31500 comparisons under both accounting flags. In the tested monic row, the uncounted helper makes zero normalization calls; the accounted helper makes one. No runtime is inferred from that operation check. [pcc:unverified]
\begin{center}\begin{tabular}{lr}\toprule
Scope & Recorded outcome\\\midrule
Old/new fixed matrices &432 cases each\\
Complete public reverification &432 per matrix\\
Exact matrix payload parity &1729 files identical\\
Native original / matched cases &36 /108 pass\\
Mutation assertions / unsupported refusals &1116 /6\\
Complete separate native proof audits &144 proofs;2+5 canaries refused\\
Native output parity &181 /433 files identical\\\bottomrule\end{tabular}\end{center}
\noindent Both bitop-disabled and enabled modes are covered. Every complete proof, basis, native receipt and enabled per-phase primitive count matches the baseline byte for byte. Source and actual root builds are pinned; the promoted helper equals the qualified candidate. All results are unsigned same-host evidence. [pcc:unverified]
\paragraph{Limits.} No separate full Visit-ledger campaign or optimized performance build was qualified. Counted calls retain legacy normalization, so no frozen cost-model credit is booked. Prior whole-libtest allocation failures remain historical. Baseline raw exits are0 while the outer observer reports1; both observations remain retained, with no silent rerun. Candidate/native/auditor captured exits are0; the additional live unit also has raw0/outer1, retained separately. [pcc:unverified]
\paragraph{Next.} No comparative timing ran;15$\times$ remains unachieved, and intrinsic regularity/F4 solving degree remain unknown. Canonical timing-document choice is pending; Conductor and checkpoint/S3 are waived. A future campaign must retain old root pair-graph arms as eligible controls and qualify both new native versions, with the original full boundary, repetitions/A/A, six fixed inputs and every proof/failure gate.
\paragraph{Reproduce.} README, PLAN/INTEGRATION\_PLAN, separate source/pre-call records, complete old/new matrices and native cohorts, six immutable workers plus the live-unit worker, raw logs and source preimages are supplied. The report only renders recorded values.
\end{document}
'''
for a,b in {'primes2147483647':'primes 2147483647','uniform3':'uniform 3','on15804':'on 15804','passes31500':'passes 31500','are0':'are 0','reports1':'reports 1','ran;15':'ran; 15'}.items():text=text.replace(a,b)
(s/"report.tex").write_text(text)
subprocess.run(["xelatex","-interaction=nonstopmode","-halt-on-error","-output-directory",str(s),str(s/"report.tex")],cwd=s,check=True,stdout=(s/"raw/report-build-2.stdout").open("x"),stderr=(s/"raw/report-build-2.stderr").open("x"))
