"""Render recorded native values only; no research arithmetic or experiments."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

study=Path(__file__).resolve().parent.parent
source='pipeline-receipts/publication-followup-audit-v3-20261026/output.json'
data=json.loads((study/source).read_text())
assert data['status']=='native_profile_accounting_and_partial_scalar_checks_pass'
plt.rcParams.update({'font.size':9,'svg.fonttype':'none'})
def save(fig,folder,stem):
    folder.mkdir(exist_ok=True)
    for ext in ('svg','png','pdf'):
        p=folder/(stem+'.'+ext);fig.savefig(p,bbox_inches='tight',dpi=150)
        if ext=='svg':p.write_text('\n'.join(l.rstrip() for l in p.read_text().splitlines())+'\n')
    plt.close(fig)
def render(folder,title,body):
    (folder/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=18mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\section*{'''+title+'}\n'+body+'\n\\end{document}\n')
    with (folder/'render.log').open('w') as log:
        subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=folder,stdout=log,stderr=subprocess.STDOUT,check=True)

folder=study/'reports/run-narrow-constructor-20261026'
arms=data['constructor']['arms']
fig,ax=plt.subplots(figsize=(7.4,2.65))
ax.bar([r['arm'] for r in arms],[r['whole_process_ir'] for r in arms],color=['#ac6146','#167d8d'],width=.5)
for r in arms:ax.text(r['arm'],r['whole_process_ir'],f"{r['whole_process_ir']:,}",ha='center',va='bottom')
ax.set_ylim(0,10000000);ax.set_ylabel('simulated whole-process instructions (Ir)')
ax.set_title('Frozen narrow6 constructor: retained raw Callgrind totals')
ax.spines[['top','right']].set_visible(False);fig.tight_layout();save(fig,folder,'narrow-constructor-instructions')
render(folder,'Narrow quartic constructor: preserved field, fewer recorded instructions',rf'''
Original runs 2026-10-25; native provenance/accounting audit 2026-10-26.
Baseline source 335d168aac85, binary SHA prefix 81ab248653df; candidate library
source c754e7111. Same narrow6 workload, one thread, shared AMD EPYC 9V74
Linux x86-64 host, L0. Raw manifests and both executable hashes are checked.

\begin{{center}}\includegraphics[width=.92\linewidth]{{narrow-constructor-instructions.pdf}}\end{{center}}
The simulated whole-process totals are 8,774,220 and 570,927 Ir. They include
loader/startup work and the Valgrind allocator. Complete stdout is
byte-identical. All seven frozen m=4--10 even/odd controls return identical
base polynomials, quartic coefficients and checked field products. The
library gate on this source passed 4,635 tests, zero failed, 119 ignored;
affected example tests and scoped clippy pass. These are recorded source
checks, not checks on a later merged PR head.

The change excludes c3=0 for even m and c3=c1=0 for every m, preserving
the first irreducible field. The readable exclusion and six checked Lean
algebraic dependencies are retained; finite-field specialization and the
irreducibility bridge remain unformalized. Original active quartic runs used
their separate frozen binary and were not modified by this change.

\textbf{{Limits.}} This is instruction attribution, not native runtime,
physical throughput, a fastest-external-tool comparison, or a complete-IC
speedup. Repeated A/A and interleaved native A/B controls, complete phase
prices and matched whole-pipeline comparisons remain missing. No new
leaderboard or admitted benchmark row follows.

\begin{{flushleft}}\footnotesize
Native source: examples/ic_followup_publication_audit.rs.\par
Receipt: \texttt{{../../{source}}}.\par
Protocol: NARROW_CONSTRUCTOR_EXCLUSION_PROTOCOL.md and FOLLOWUP_PUBLICATION_AUDIT_PROTOCOL.md.\par
Archive: evidence/runs-narrow-constructor-checked-20261026.tar.xz.\par
Existing PDFs and raw measurements are unchanged; new oversized artifacts remain local pending verified store upload.
\end{{flushleft}}
''')

folder=study/'reports/run-partial-quartics-20261026'
fig,axes=plt.subplots(1,2,figsize=(8.2,2.9),sharey=True)
colors={'verified':'#167d8d','timeout':'#ac6146','crashed':'#8969a5'}
for ax,row in zip(axes,data['partial_quartics']):
    labels=['rho-neg','ic-frobenius','rho-neg-aa']
    for arm in labels:
        bottom=0
        for status,color in colors.items():
            value=row['counts'].get(arm,{}).get(status,0)
            ax.bar(arm,value,bottom=bottom,color=color,label=status if arm=='rho-neg' else None)
            bottom+=value
    ax.bar('not recorded',row['missing_registered_executions'],color='#b6b8bc')
    ax.set_title(f"m={row['m']}: {row['recorded']} of {row['registered']} executions recorded")
    ax.set_ylim(0,10);ax.tick_params(axis='x',rotation=22);ax.spines[['top','right']].set_visible(False)
axes[0].set_ylabel('execution records (count)')
handles=[plt.Rectangle((0,0),1,1,color=color) for color in list(colors.values())+['#b6b8bc']]
fig.legend(handles,list(colors)+['missing'],loc='lower center',ncol=4,bbox_to_anchor=(.5,-.08))
fig.tight_layout();save(fig,folder,'partial-quartic-outcomes')
render(folder,'Supported quartic continuation: interrupted runs and checked answers',rf'''
Original additional L0 runs began 2026-10-25 at 17:54:16 UTC (m14) and
22:49:42 UTC (m15). Same registered eight one-target workloads and three
arms, frozen ecbench source 0a5137c2, binary SHA prefix efa25f209e57.
Shared AMD EPYC 9V74 host, four Rayon threads, 12-GiB address space,
1,800 seconds per arm and 14,400 seconds outer cap per run/replay.
Wall/CPU time is diagnostic; these do not replace the original L2 ladder.

\begin{{center}}\includegraphics[width=\linewidth]{{partial-quartic-outcomes.pdf}}\end{{center}}
m14 retains 17/24 records: 11 successful rho answers, five IC timeouts and
one interrupted/crashed IC attempt, with seven registered executions missing.
Its original native audit binds all files and reproduces the 11 successful
records byte-identically, but returns failure because the session is
interrupted. m15 retains 15/24 records: ten successful rho answers and five
IC timeouts, with nine executions missing. It has no completed-session or
all-record replay certificate. The environment restart left no live runner.

The new native audit independently verifies every one of those 11 and ten
successful scalars with BinaryCurve arithmetic, using the exact recorded
generator and target. Curve identity, subgroup/order and generator checks
agree with the reconstructed model. Legacy point coordinates use the packed
tower basis; the adapter converts them to the canonical polynomial basis.
Two initial verifier failures from misreading that encoding are preserved;
the corrected audit repeats byte-identically. The factories' generators
agree after the coordinate conversion on both checked models.

\textbf{{Limits.}} No IC attempt in these retained records completed.
Missing/censored work has unknown cost; no successful-only cost mean or
IC/rho ratio is computed. Answer checks are not new rho walks, independent
counter recounts, or a completed session. Original L2 m14/15, full wide IC
adapters and the 124-branch independent HKY program remain unresolved.

\begin{{flushleft}}\footnotesize
Exact curve/subgroup models: \texttt{{../../curve-identity-adapter.v2.json}}.\par
Native receipt: \texttt{{../../{source}}}.\par
Protocol: FOLLOWUP_PUBLICATION_AUDIT_PROTOCOL.md; preserved original SUPPORTED_LOCAL_QUARTIC_PROTOCOL.md.\par
Archive: evidence/runs-partial-quartics-complete-20261026.tar.xz.\par
No measured crossover, complete-price IC gain, or artifact-store upload is inferred.
\end{{flushleft}}
''')
