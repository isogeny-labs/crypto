#!/usr/bin/env python3
"""Layout only. All counts, ranges and comparative ratios copied from Rust outputs."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import PercentFormatter
p=Path(__file__).resolve().parent
panels=[p,p/'candidate2',p/'candidate3']
summaries=[json.loads((b/'summary.json').read_text()) for b in panels]
audits=[json.loads((b/'grid-audit.json').read_text()) for b in panels]
ids=[r['instance'] for r in audits[-1]['goal_criteria']]
labels=[i.replace('scale-n','n').replace('-d12','').replace('distinct-planted-0','positive') for i in ids]
fig,axes=plt.subplots(2,1,figsize=(10,6),layout='constrained')
colors=['#b85849','#c58f2b','#267b78']
names=['Lazy/shared maps (miss)','Specialized + equal full LTO (miss)','Register arithmetic + inverse reuse']
uniform='scale-n131-d12-uniform'
axes[0].bar(range(3),[s['cells'][uniform+'/elimination']['metrics']['instructions']['median'] for s in summaries],color=colors)
axes[0].axhline(15_000_000,color='#343c50',linestyle='--',label='Strict 15M target')
axes[0].set_xticks(range(3),['First candidate','Second candidate','Third candidate']);axes[0].set_ylabel('Total cold-query instructions');axes[0].ticklabel_format(axis='y',style='plain');axes[0].legend(frameon=False);axes[0].set_title('Identical n131 d12 uniform input, five fresh repetitions per panel')
for j,(audit,name,color) in enumerate(zip(audits,names,colors)):
 axes[1].bar([i+(j-1)*.25 for i in range(len(ids))],[r['whole_removed_fraction'] for r in audit['goal_criteria']],width=.25,label=name,color=color)
axes[1].set_xticks(range(len(ids)),labels,rotation=12,ha='right');axes[1].set_ylabel('Fewer matched cold instructions');axes[1].yaxis.set_major_formatter(PercentFormatter(1));axes[1].legend(frameon=False,fontsize=8)
for a in axes:a.grid(axis='y',alpha=.2);a.set_axisbelow(True)
fig.suptitle('ECC2K-130 index-calculus component: below 15M total cold instructions',fontsize=13)
fig.savefig(p/'instructions.svg');fig.savefig(p/'instructions.png',dpi=150);plt.close(fig)
svg=p/'instructions.svg';svg.write_text('\n'.join(l.rstrip() for l in svg.read_text().splitlines())+'\n')
s=summaries[-1];a=audits[-1];rows=[]
for ident,r in zip(ids,a['goal_criteria']):
 cells=s['cells'];base=cells[ident+'/baseline']['metrics']['instructions'];cand=cells[ident+'/elimination']['metrics']['instructions'];rows.append((ident,base,cand,r))
with (p/'results-table.md').open('w') as f:
 f.write('| Case | Matched baseline cold Ir (median [min,max]) | Candidate cold Ir (median [min,max]) | Cold saved | Oracle saved |\n|---|---:|---:|---:|---:|\n')
 for ident,b,c,r in rows:f.write(f"| {ident} | {b['median']:,} [{b['min']:,},{b['max']:,}] | {c['median']:,} [{c['min']:,},{c['max']:,}] | {r['whole_removed_fraction']:.2%} | {r['oracle_removed_fraction']:.2%} |\n")
cell=s['cells'][uniform+'/elimination']['metrics'];base=s['cells'][uniform+'/baseline']['metrics']['instructions']['median'];r=next(x for x in a['goal_criteria'] if x['instance']==uniform)
table='\n'.join(f"[{i.replace('scale-','').replace('-d12','').replace('distinct-planted-0','positive')}], [{b['median']:,}], [{c['median']:,}], [{v['whole_removed_fraction']:.2%}], [{v['oracle_removed_fraction']:.2%}]," for i,b,c,v in rows)
text=f'''#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
#set table(inset:5pt,stroke:0.4pt+luma(180))
= ECC2K-130 index calculus: below 15M cold instructions

The unchanged n131 d12 uniform query uses *{cell['instructions']['median']:,} total simulated instructions* in each of five repetitions, below the user's strict15,000,000 target. Matched baseline:*{base:,}Ir*; *{r['whole_removed_fraction']:.2%} fewer*, *{r['baseline_over_elimination_whole']:.6f}x*. Cold setup:{cell['setup_instructions']['median']:,}; complete oracle:{cell['oracle_instructions']['median']:,}; post/original replay/output/exit:{cell['post_instructions']['median']:,}Ir. These phases sum to the whole cold count. PCC-20261124-807b8e. [pcc:unverified]

== Fixed workload and complete boundary
All process parsing, field/context construction, factor-base admission, index allocation, projected systems, root checks, relation replay, output and exit stay charged. Same frozen inputs, order and semantic outputs; no warmed state or cross-query reuse. Instruction counts are Callgrind Ir, not native instructions per second or a native runtime measurement. The final negative query certifies no decomposition in its supplied factor base; it is not a full discrete-log recovery.

#table(columns:(2fr,1fr,1fr,.8fr,.8fr),[*Case*],[*Baseline cold*],[*Candidate cold*],[*Cold saved*],[*Oracle saved*],
{table}
)

All six controls pass cold/oracle no-regression limits against both matched controls; A/A uses a source-identical baseline under another arm name. Five repetitions per cell; exact min/max in results-table.md. Deterministic repeats establish repeatability on this fixed grid, not a population confidence interval. Ratios come from each panel's own matched controls, never compounded across compiler settings.

#figure(image("instructions.svg",width:100%),caption:[All three candidate panels retained. First panel uses the earlier compiler profile; second and third use equal full LTO/codegen-units1 on every arm.])

#pagebreak()
= Implementation, checks and custody

== Qualified changes
Defer unused symbolic polynomial-reduction tables with a shared OnceLock; symbolic preparation remains available. Share the existing exact trace/half-trace table with compact arithmetic. Evaluate projected row maps in word-local chunks. Validate ASCII hexadecimal chunks in SIMD registers, with the original extended parser as fallback. Use runtime-guarded carryless canonical131 square/multiply, cache feature selection, and reuse consecutive equal nonzero denominators inside the original bounded simultaneous-inversion batches. Every point still undergoes its original admission, ordinate/group checks and replay. The prior direct-address dictionary and original order remain intact.

This is an implementation bundle and equal compiler configuration, without isolated per-change attribution. Portable paths remain; the measured hardware class is x86-64 with pclmulqdq/SSE2. Other classes are unmeasured.

== Exact checks and flow
62 focused release tests pass, including every131-bit basis product, separate square checks, all256 byte values at16 parser positions, exhaustive small-field symbolic evaluation, and repeated/exceptional denominator cases.65 retained cases match all three executable variants (195 native rows). Full primary: planned120, attempted120, completed120, excluded0, retries0;240 fresh measurement children (phased/unphased). Separate schoolbook polynomial arithmetic replays80 saved relation tuples and checks the retained negative cases. All three strict exports have zero findings. Current frozen source inventory is unchanged. Producer checks are unsigned, not independent admission. [pcc:unverified]

== Retained misses and launch corrections
First full panel:17,287,437Ir on the uniform n131 query, missing15M and failing one n83 positive oracle guard. Second:16,033,438Ir, all guards pass but target missed. No panel is removed. Two third-candidate preflight launch errors happened before any measurement child: stale study-directory text, then an extra absolute-path prefix. Erroneous bytes and invalid records are retained; affected earlier metadata restored exactly. Final runner derives its directory and refuses overwrites. The third candidate's original epoch1795491725 was never reset; primary completed2026-11-24T03:51:45Z within the original600seconds. A three-instruction baseline typo in an initial claim is corrected by PCC-20261124-807b8e; frozen summary is authoritative. [pcc:unverified]

== Source identities and reproduction
Final numerical source1861f64e14; corrected runner binding781e12c34c; protocol publication9e2d87e27f before primary data. Protocol SHA6ffbb8617211d163d4d24f7fd9b2d7af13ca1aa0ad96816287180226e5d00060. Candidate ELF SHA0c95b811c80e1669b645480a2c52b7635f2f78435f7d54748b4ab5da78d50d23. Inputs and all executables are archived locally with per-member hashes, extraction byte checks and strict exports. Large new objects remain local pending original-policy store upload; pending-artifacts.json enumerates their exact hashes and sizes. No object is marked uploaded. Native execution/harness/replay are Rust; Python handles metadata/custody/layout only.

Local CPU0,2GiB child address space,600seconds per panel, zero paid resources and zero infrastructure retries. This report closes the15M rung. Earlier50x goal remains future work. Broad root-library test execution remains blocked at dependency compilation in retained attempts; focused actual-source workspace checks pass. Remote artifact upload/checkpoint custody retains its prior403 blocker, original missing leaderboard source prevents regeneration, and signed independent admission is pending. No admitted leaderboard row or gate is changed. KB wrapper unavailable127; no claim of fastest external solver or mathematical trace-zero breakthrough. [pcc:unverified]
'''
(p/'report.typ').write_text(text)
