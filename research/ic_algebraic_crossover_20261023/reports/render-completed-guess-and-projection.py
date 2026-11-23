"""Present native completed-prefix and finite cofactor-projection receipts."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study=Path(__file__).resolve().parent.parent
guess=json.loads((study/'pipeline-receipts/guess-completion-20261025-01/summary.json').read_text())
projection=json.loads((study/'pipeline-receipts/cofactor-projection-20261025/receipt.json').read_text())
assert guess['status']=='all_registered_attempts_terminated_native_raw_accounting_recomputed'
assert projection['status']=='all_finite_projected_membership_and_relation_checks_agree'
plt.rcParams.update({'font.size':9,'svg.fonttype':'none'})
def save(fig,folder,stem):
    folder.mkdir(exist_ok=True)
    for ext in ('svg','png','pdf'):
        p=folder/(stem+'.'+ext);fig.savefig(p,bbox_inches='tight',dpi=150)
        if ext=='svg':p.write_text('\n'.join(l.rstrip() for l in p.read_text().splitlines())+'\n')
    plt.close(fig)
def render(folder,title,body):
    (folder/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\section*{'''+title+'}\n'+body+'\n\\end{document}\n')
    with (folder/'render.log').open('w') as log:
        subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=folder,stdout=log,stderr=subprocess.STDOUT,check=True)

folder=study/'reports/run-completed-balanced-guess-20261025'
fig,axes=plt.subplots(2,2,figsize=(8.5,4.4),sharex=True,sharey=True)
for ax,draw in zip(axes.flat,guess['draws']):
    groups=draw['groups'];xs=[g['guesses'] for g in groups];ys=[g['all_branch_solver_word_ops'] for g in groups]
    ax.plot(xs,ys,color='#8a8f99',linewidth=1)
    for g in groups:
        if g['all_branch_solver_word_ops'] is None:
            ax.text(.04,.9,'k=1: one timeout; total unknown',transform=ax.transAxes,fontsize=8)
        else:
            passing=g['verdict']=='all_branches_refuted'
            ax.scatter(g['guesses'],g['all_branch_solver_word_ops'],marker='o' if passing else 'x',color='#167d8d' if passing else '#ac6146',zorder=3)
    ax.set_title(f"draw {draw['draw']}: first passing balanced prefix k={draw['balanced_prefix_minimum']}")
    ax.set_xticks(range(5));ax.set_yscale('log');ax.set_xlabel('guessed variables in fixed prefix')
    ax.set_ylabel('all-branch solver-word operations');ax.spines[['top','right']].set_visible(False)
fig.tight_layout();save(fig,folder,'balanced-prefix-costs')
rows='\n'.join(f"{r['draw']} & {r['balanced_prefix_minimum']} & {r['passing_set_solver_words']:,} & {r['search_through_first_passing_set_solver_words']:,}"+r' \\' for r in guess['draws'])
render(folder,'Completed balanced guessing: the registered minima',rf'''
Original ell=8, m=3, n=22 Boolean fixtures; fixed prefixes of [7,15,23,24].
The 116 new processes ended at 2026-10-25 17:53:53 UTC; the eight
earlier draw-0/k3 cases remain separate raw records. Of all 124 attempts,
78 refute, 45 saturate without refuting and one is unknown after its 900-second
timeout (draw 6, k=1, branch 1). No missing attempt or zero-cost replacement.

\begin{{center}}\includegraphics[width=\linewidth]{{balanced-prefix-costs.pdf}}\end{{center}}
\begin{{center}}\small\begin{{tabular}}{{rrrr}}\toprule
Draw & First passing k & All passing-set words & Prefix-search words through success\\\midrule
{rows}
\bottomrule\end{{tabular}}\end{{center}}
At k=4 all 64 branches refute, charging 23,078,681,663,212 solver-word
operations across four draws. Every smaller prefix on draws 0,3,5 has
nonrefuting branches, so four is the minimum in this registered prefix family.
Draw 6 refutes at k=0, hence its minimum is zero; the later k=1 timeout is
retained and is irrelevant to that zero lower bound. Green circles indicate
all-branch refutation; crosses indicate a nonrefuting branch; null totals
are missing, never plotted as zero. Prefix-search columns include failed
lower-k work, not just the final passing set. The complete 124-attempt
experimental word total stays unknown because one attempt is censored.

Frozen producer source f83ed00d8, binary SHA prefix ab544a41df94;
input SHA prefix 95ee147e1221. Four Rayon threads, 12-GiB address-space,
900 seconds per new branch, degree four, outer column cap 200,000. Shared
AMD EPYC 9V74 Linux x86-64 host, L0; wall time is diagnostic.
The native completion audit reproduces twice; the complete raw archive
re-extracts byte-identically and its raw-derived summary reproduces.

\textbf{{Limits.}} This is no minimum over arbitrary variable subsets or
other orderings. It is finite Boolean closure, not a completed ECDLP query.
The explicit reference/twist subgroup gaps and possible cofactor-projected
interpretation remain separate obligations. Independent HKY verdict replay
is genuinely queued after the current m14/15 quartic program; it has not
run and does not independently recount producer words. Construction,
extraction/lifting, verification and native physical prices remain absent.
No full-query/IC/rho or asymptotic claim follows.

\begin{{flushleft}}\footnotesize
Source: \path{{../../pipeline-receipts/guess-completion-20261025-01/summary.json}}\par
Complete archive: \path{{../../evidence/runs-balanced-guess-complete-all124-20261025.tar.xz}}\par
Protocol: COMPLETED_GUESS_REPLAY_PROTOCOL.md; original cap amendment remains
unchanged. No additional paid VM or artifact-store upload is claimed.
\end{{flushleft}}
''')

folder=study/'reports/run-cofactor-projection-20261025'
fig,axes=plt.subplots(1,2,figsize=(8.5,2.8),sharey=True)
for ax,a,label in zip(axes,[0,2097152],['E0, r=991, h=4232','twist, r=2,097,349, h=2']):
    data=sorted((r for r in projection['rows'] if r['a']==a),key=lambda r:r['draw'])
    xs=[r['draw'] for r in data]
    ax.bar([x-.18 for x in xs],[r['strict_prime_subgroup_base_points'] for r in data],width=.36,color='#ac6146',label='original points in G')
    ax.bar([x+.18 for x in xs],[r['projected_distinct_nonzero_points'] for r in data],width=.36,color='#167d8d',label='distinct nonzero hP')
    ax.set_title(label);ax.set_xticks(xs);ax.set_xlabel('original draw');ax.spines[['top','right']].set_visible(False)
axes[0].set_ylabel('factor-base point inventory')
fig.legend(*axes[0].get_legend_handles_labels(),loc='lower center',ncol=2,frameon=False,bbox_to_anchor=(.5,-.02))
fig.tight_layout(rect=(0,.12,1,1));save(fig,folder,'cofactor-inventories')
render(folder,'Strict subgroup support and the projected alternative',rf'''
Additional native finite audit, 2026-10-25. All seven original x-only fixtures
on both fixed n=22 reference/twist models, exact polynomial $z^{{22}}+z+1$,
recorded prime r, cofactor h and generator. Source input SHA prefix
95ee147e1221. Source/binary frozen before output; fresh native rerun is
byte-identical. One thread, 2-GiB address-space and 300-second caps,
shared AMD EPYC 9V74 Linux x86-64 host, correctness diagnostic only.

\begin{{center}}\includegraphics[width=\linewidth]{{cofactor-inventories.pdf}}\end{{center}}
The strict audit is correct about original points in G; it does not rule
out projected IC relations. Enumerating all full-curve points above V,
keeping projection multiplicities/zero images and hashing the exact distinct
inventory, gives nonempty projected bases on both models for every draw.
All projected points pass [r](hP)=O. The projected abscissae need not lie in V;
an implementation would retain the original preimage coordinates for its
algebraic oracle and the corresponding hP for logarithm variables.

For the three original SAT draws, one selected Boolean witness and one
canonical (least-y) target sign produce three verified curve relations.
All three remain verified nonzero prime-subgroup relations after multiplying
each point and target by h, although zero have every original summand in G.
The earlier audit counted both target signs; that convention gives two
relations per witness and is not contradicted by this one-sign audit.
Thus ``no original all-in-G witness'' is not a universal obstruction to IC.

Every liftable chosen-model target has nonzero hR. The E0 projected
targets for draws 0 and 1 are already in the projected base; lookup would
require its logs to have been computed and charged. Draw 6 has 254 distinct
nonzero E0 projected points despite zero original E0 points in G, and its
projected target is nonzero and not a direct projected-base hit. These are
finite support checks, not supplied discrete logarithms or measured solves.

\textbf{{Open method obligations.}} A projected-IC version still needs a
declared target/preimage/cofactor-component law, projected-base log collection,
exact preimage-to-projected inventory handling, scalar recovery and independent
verification, with all setup and target-dependent work charged. The original
fixtures still do not specify that complete law. This audit changes no old
strict-G receipt or Boolean count and establishes no complete-price gain,
query-rate comparison, scaling law or crossover. The 1,591/161 counts in the
separate qualified panel retain their explicitly strict-G contract.

\begin{{flushleft}}\footnotesize
Protocol: COFACTOR_PROJECTION_PROTOCOL.md.\par
Native receipt: \path{{../../pipeline-receipts/cofactor-projection-20261025/receipt.json}}\par
Exact global model identities are provided by curve-identity-adapter.v1.json.
No extra paid VM allocation or artifact upload is claimed.
\end{{flushleft}}
''')
print('Rendered completed guessing and cofactor-interpretation reports; inspect both PDFs.')
