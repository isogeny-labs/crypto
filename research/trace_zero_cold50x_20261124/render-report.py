#!/usr/bin/env python3
"""Layout only: every measured count/ratio is copied from frozen Rust output."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import PercentFormatter
p=Path(__file__).resolve().parent
v=json.loads((p/'v2-grid-audit.json').read_text());s=json.loads((p/'inline/grid-audit.json').read_text());summary=json.loads((p/'inline/summary.json').read_text())
ids=[r['instance'] for r in s['goal_criteria']];labels=[i.replace('scale-n','n').replace('-d12','').replace('distinct-planted-0','positive') for i in ids]
fig,axes=plt.subplots(2,1,figsize=(10,6),layout='constrained')
for ax,metric,title in zip(axes,['whole_removed_fraction','oracle_removed_fraction'],['Complete cold query','Complete relation oracle']):
 ax.bar([i-.18 for i in range(len(ids))],[r[metric] for r in v['goal_criteria']],width=.36,label='Rejected first index',color='#b9574a')
 ax.bar([i+.18 for i in range(len(ids))],[r[metric] for r in s['goal_criteria']],width=.36,label='Inline direct index',color='#247b79')
 ax.axhline(0,color='black',lw=.7);ax.yaxis.set_major_formatter(PercentFormatter(1));ax.set_xticks(range(len(ids)),labels,rotation=12,ha='right');ax.set_ylabel('Fewer matched instructions');ax.set_title(title,fontsize=12);ax.grid(axis='y',alpha=.2);ax.set_axisbelow(True)
axes[0].legend(loc='upper right',frameon=False);fig.suptitle('Each candidate versus its own matched previous-best control\n50x requires98% fewer; that target is not attained',fontsize=13)
fig.savefig(p/'instructions.svg');fig.savefig(p/'instructions.png',dpi=150);plt.close(fig)
rows=[]
for ident,r in zip(ids,s['goal_criteria']):
 cell=summary['cells'];a=cell[ident+'/baseline']['metrics']['instructions'];b=cell[ident+'/elimination']['metrics']['instructions'];rows.append((ident,a,b,r))
with (p/'results-table.md').open('w') as f:
 f.write('| Case | Matched baseline cold Ir | Inline cold Ir | Whole saved | Oracle saved |\n|---|---:|---:|---:|---:|\n')
 for ident,a,b,r in rows:f.write(f"| {ident} | {a['median']:,} [{a['min']:,},{a['max']:,}] | {b['median']:,} [{b['min']:,},{b['max']:,}] | {r['whole_removed_fraction']:.2%} | {r['oracle_removed_fraction']:.2%} |\n")
uniform=next(r for r in s['goal_criteria'] if r['instance']=='scale-n131-d12-uniform');cold=summary['cells']['scale-n131-d12-uniform/elimination']['metrics']['instructions']['median'];base=summary['cells']['scale-n131-d12-uniform/baseline']['metrics']['instructions']['median'];oracle=summary['cells']['scale-n131-d12-uniform/elimination']['metrics']['oracle_instructions']['median']
table='\n'.join(f"[{ident.replace('scale-','').replace('-d12','').replace('distinct-planted-0','positive')}], [{a['median']:,}], [{b['median']:,}], [{r['whole_removed_fraction']:.2%}], [{r['oracle_removed_fraction']:.2%}]," for ident,a,b,r in rows)
text=f'''#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
#set table(inset:5pt,stroke:0.4pt+luma(180))
= ECC2K-130: additional improvement,50x target remains open

The unchanged n131 d12 uniform cold query uses *{cold:,} simulated instructions*, versus matched previous-best *{base:,}*: *{uniform['baseline_over_elimination_whole']:.6f}x*, or *{uniform['whole_removed_fraction']:.2%} fewer*. Complete oracle:{oracle:,}Ir, *{uniform['oracle_removed_fraction']:.2%} fewer*. Requested another50x requires at most427,207 complete Ir; it is *not met*. Unsigned producer evidence PCC-20261124-bc5973. [pcc:unverified]

== All six fixed cells, five fresh replicates each
#table(columns:(2fr,1fr,1fr,.8fr,.8fr),
[*Case*],[*Baseline cold*],[*Inline cold*],[*Whole saved*],[*Oracle saved*],
{table}
)

These are frozen Rust medians. Exact min/max ranges are in results-table.md and both summaries. There is no population confidence interval or native-runtime claim. Each series uses its own paired baseline; ratios are not compounded across panels.

#figure(image("instructions.svg",width:100%),caption:[Instruction changes from each candidate's own matched previous-best baseline.50x would require98% removal, above the plotted gains.])

== Implementation and validation
Bounded direct addressing for low polynomial-coordinate windows, at most65,536 slots and at most8 slots per supplied point; arbitrary windows retain the ordered tree. The first two indices stay inline; further duplicates spill in original order. Sorted keys, per-input width/sign/curve admission, model order and every original root/group check are retained. All allocation/setup is charged.

52 focused release tests pass. All65 retained cases match three executable variants (195 rows), including m83. The inline primary contains120/120 rows with no exclusions/retries; all six control criteria pass.80 saved relation tuples replay with separate schoolbook arithmetic. Both strict final exports have zero findings. [pcc:unverified]

#pagebreak()
= Retained misses, provenance and next obligations

== The first complete index panel lost
The initial slot bound allowed at most4 slots per supplied point. The actual n131 window requires8192 slots for1994 points, so that candidate used its tree fallback and added selection overhead. Its120-row complete panel is retained: uniform n13121,566,178Ir, about1.02% more than its matched baseline; oracle about1.56% more. Every no-regression guard failed. The revised candidate has an explicit real-window boundary test and inline storage. [pcc:unverified]

== A global S4 filter was derived and falsified
A symmetric identity in a=x1+x2+x3,b=x1x2+x1x3+x2x3,c=x1x2x3 was expanded from the production quadratic resultant. All4096 F8 tuples match. Relaxing the nonlinear coefficient products gives318 columns at the actual degree12 window. Both uniform targets have full ranks83/131 and all six targets are consistent. This particular relaxation supplies no global rejection. The initial narrower degree11 grid is also retained and explicitly corrected. See leads/S4-ENVELOPE.md and both exact Rust grids. [pcc:unverified]

== Reproduction and custody
Inline numerical source2e9a7b71ff; protocol publication bd2b17ed59 before every primary row; protocol SHA906243b747bf6c0f8f158704fbb2b12070a40401904c40cbe1047e6e9c741de9. Previous failed primary froze52f1042c... . Three matched executables use the same harness/compiler and exact previous-best source snapshots for controls. Independent isolated build prevents the initial shared-target stale-control problem.

Original stale-control pilot binaries/rows are retained as invalid; an embedded relocated helper briefly edited old planning metadata. The erroneous bytes are preserved and original files restored exactly from Git before further comparisons. Invalid schema/export drafts are retained. No numeric row was overwritten or omitted.

RAW-ALL.tar.xz and FROZEN-EXECUTABLES.tar.xz preserve all raw Callgrind/native/qualification/input bytes and every executed binary, identified by archive-members.json. Extraction checks establish local producer custody only.600second local envelopes, CPU0/2GiB, zero paid resources and zero infrastructure retries; scientific counters are Rust, Python performs metadata binding/rendering only.

== Goal and remaining work
Another50x remains an unchecked goal. The present decoder alone was measured at710,124Ir, above the427,207 total target; solver-only edits retaining that ingestion path cannot satisfy it. Further work needs both less parsing/setup and a stronger algebraic filter retaining dependencies discarded by the full-rank S4 relaxation. These are leads, not observed gains.

Full root-library integration, native timing/warmed throughput, remote artifact custody (prior403), missing historical leaderboard source and signed independent admission remain open. No admitted leaderboard row or gate is changed. KB unavailable (wrapper127); literature/source review and exact reproduction are documented in README.md. All observations are unsigned producer evidence. [pcc:unverified]
'''
(p/'report.typ').write_text(text)
