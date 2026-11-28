"""Layout only. Every scientific count/fraction comes from the Rust outputs."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
p=Path(__file__).resolve().parent
s=json.loads((p/'summary.json').read_text());g=json.loads((p/'grid-audit.json').read_text());f=json.loads((p/'frozen-reference.json').read_text());q=json.loads((p/'protocol.json').read_text())
criteria=g['goal_criteria'];ids=[r['instance'] for r in criteria]
headline=next(r for r in criteria if r['instance']=='scale-n131-d12-uniform')
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,axes=plt.subplots(2,1,figsize=(9.4,6.1),layout='constrained')
for ax,metric,title in zip(axes,['instructions','oracle_instructions'],['Complete cold query: all setup, parsing, search and replay','Complete algebraic oracle: preparation, search, roots and checks']):
 for offset,arm,label,color in [(-.24,'baseline','Original','#697586'),(0,'incumbent','Prior incumbent','#7795c4'),(.24,'elimination','Final candidate','#007f8b')]:
  values=[s['cells'][i+'/'+arm]['metrics'][metric] for i in ids]
  x=[i+offset for i in range(len(ids))];ax.bar(x,[v['median'] for v in values],width=.22,label=label,color=color)
  ax.errorbar(x,[v['median'] for v in values],yerr=[[v['median']-v['min'] for v in values],[v['max']-v['median'] for v in values]],fmt='none',ecolor='black',capsize=2)
 ax.set_title(title,loc='left',fontweight='bold');ax.set_ylabel('Million simulated instructions');ax.yaxis.set_major_formatter(FuncFormatter(lambda x,_:f'{x/1e6:g}'));ax.set_xticks(range(len(ids)),[i.replace('scale-n','n').replace('-d12-','\n').replace('distinct-planted-0','planted') for i in ids]);ax.spines[['top','right']].set_visible(False);ax.grid(axis='y',alpha=.15);ax.set_axisbelow(True)
axes[0].plot([3.62,4.42],[f['ceiling_ir']]*2,color='#ba4937',linestyle='--',linewidth=1.5,label='Fixed n131 uniform ceiling')
axes[0].legend(frameon=False,ncol=2,fontsize=8)
fig.suptitle('Fixed d12 trace-zero relation queries · original, prior and final code',fontweight='bold')
fig.text(.5,-.015,'Five fresh repeats per cell · whiskers: observed min/max · unsigned producer evidence',ha='center',fontsize=8)
fig.savefig(p/'instructions.svg',bbox_inches='tight');fig.savefig(p/'instructions.png',dpi=150,bbox_inches='tight');plt.close(fig)
rows=[];md=[]
for r in criteria:
 i=r['instance'];label=i.replace('scale-n','').replace('-d12-','/').replace('distinct-planted-0','planted');b=s['cells'][i+'/baseline']['metrics']['instructions']['median'];c=s['cells'][i+'/elimination']['metrics']['instructions']['median']
 rows.append(f'[{label}], [{b:,}], [{c:,}], [{r["whole_removed_fraction"]:.2%}], [{r["oracle_removed_fraction"]:.2%}],')
 md.append(f'| {label} | {b:,} | {c:,} | {r["whole_removed_fraction"]:.2%} | {r["oracle_removed_fraction"]:.2%} |')
(p/'results-table.md').write_text('| Fixed d12 query | Matched original cold Ir | Final cold Ir | Cold saved | Oracle saved |\n|---|---:|---:|---:|---:|\n'+'\n'.join(md)+'\n')
source=q['arms'][0]['instrument']['commit'];pub='0a2e495ef2d5ec76f9a4c2a8ef5c200f93a6e7f4'
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.5em)
= ECC2K-130: more than 50% fewer cold-query instructions
The unchanged degree 131 d12 uniform relation query uses *21,360,373 complete cold simulated instructions*: *'''+f'{f["removed_fraction"]:.2%}'+'''* below the fixed original count of 44,023,404, meeting its unchanged 22,011,702 ceiling. Against the simultaneous matched original implementation (44,038,338), the reduction is '''+f'{headline["whole_removed_fraction"]:.2%}'+'''. All 120 primary rows and every frozen control criterion pass. Unsigned producer statements PCC-20261123-84a0ce / PCC-20261123-9a757d. [pcc:unverified]
#image("instructions.svg", width: 100%)
#table(columns: (1.4fr,1.2fr,1.2fr,1fr,1fr), inset: 4pt, [Fixed d12 query], [Original cold Ir], [Final cold Ir], [Cold saved], [Oracle saved],
'''+ '\n'.join(rows)+''')
The table compares simultaneous matched implementations. Five repeats per cell; whiskers show observed ranges, not population intervals. The fixed historical denominator remains separate and unchanged. [pcc:unverified]
#pagebreak()
= What changed, and what remains the same
The prior exact half-trace constructor remains: invert the trace-normalized Artin–Schreier map and build the same byte tables, with the exact conventional half-trace root. The final candidate adds four concrete implementation changes:

+ Parse ordinary ASCII hex directly into field words, using baseline SSE2 for eight-digit validation/packing on x86-64 and portable decoding elsewhere. The complete legacy Unicode-whitespace/plus/underscore grammar remains the fallback; truncated high digits are still validated.
+ Use three/six carry-less Karatsuba products for two/three limbs under the existing runtime PCLMUL guard, preserving XOR accumulation and generic/ISA fallbacks.
+ Fuse the canonical 131-bit product with the existing exact two-fold sparse reduction, inline compact reduction, and use separate immutable OnceLocks for the two completely qualified field polynomials. All table construction occurs at runtime and is fully charged.
+ Retain one BTreeMap Entry search across the same original sign/admission checks and insertion. The original supplied index order is retained.

The existing trace-zero variety, factor base, d12 query, projected equations, 13 coefficient bits, signs, model order, rank/limit behavior, every root check and original-group replay remain unchanged. No relation-yield or system-size reduction is inferred.

= Validation and accounting
The final focused release suite passes 49 checks. These include every byte at each SIMD position, legacy parsing and word/inline/heap boundaries, all 53,248 raw two/three-limb basis products, all 17,161 fused canonical 131-bit basis products against independent bitwise reduction, qualified cache separation, and existing context/projector/field/group checks. Three exact source variants agree on every semantic/witness/model/order/group/projected diagnostic across all 65 retained inputs (195 rows). These remain producer-written checks. [pcc:unverified]

A separate schoolbook polynomial/group implementation replays all 80 primary tuples and 138 development tuples. Primary flow: 120 planned, attempted and completed; zero excluded, zero retries. Recorded compact multiplication/squaring counts agree; those tallies exclude general setup, and total Ir prices all parsing, allocation, arithmetic and index changes. All six A/A and no-regression guards pass. [pcc:unverified]

For the fixed n131 uniform query, complete oracle instructions fall from 18,083,340 to 13,811,466 ('''+f'{headline["oracle_removed_fraction"]:.2%}'+'''). The prior incumbent cold count is 34,999,978; final code removes '''+f'{headline["incumbent_whole_removed_fraction"]:.2%}'+''' of that matched cost. Warmed relation-collector throughput and native runtime need their own measured boundary. [pcc:unverified]
#pagebreak()
= Reproduction and retained attempts
The two earlier engineering pilots are retained: candidate 1 saves 45.9443% and candidate 2 saves 49.1996% against their matched originals, both below the unchanged goal. Neither has an attempted primary row. Their complete source/binary identities, calibration, raw pilot data and reports remain under candidate1/ and candidate2/. The inherited missing arm allowlist was caught and corrected before comparative data; the failed qualifier prefix is preserved. [pcc:unverified]

Final numerical source: '''+source+'''. Primary protocol publication: '''+pub+''', before every primary row. Protocol SHA256: '''+q['freeze']['protocol_sha256']+'''. Source inventory is unchanged after the panel. The fixed-reference percentage is secondary post-run Rust arithmetic; the original absolute ceiling and contemporaneous 50% acceptance rule were already frozen and recomputed by a separately implemented Rust grid checker from the same producer. [pcc:unverified]

One release harness, three exact numerical source variants and separate frozen executables. Controls select exact original/prior field modules plus unchanged old word/index source snapshots; the final candidate selects actual production modules. Compiler/layout effects are charged. A/A uses the identical original executable. Runtime label guards reject source/executable mismatches. Qualified unexecuted SAT/chain interfaces support compilation; numerical field/curve/context/oracle modules are actual production sources.

Rust 1.100.0, Callgrind 3.24.0, Intel Xeon Platinum 8573C. CPU0, 2 GiB child address space, zero paid resources or retries. The final calibration/pilot/freeze/primary sequence spans 23:50:03–23:52:56 UTC, within its 600-second guard. Each full cold count includes input/fixture/context setup, first-use tables, all oracle preparation/search/roots/signs, replay, output, destruction and exit. There is no uncharged offline lookup-table construction. [pcc:unverified]

The broader release root-library attempts time out while compiling dependencies, before tests execute. Final root integration, independent signed scientific admission, artifact-store custody and historical leaderboard-validator reconciliation remain separate gates. No admitted leaderboard row is added. [pcc:unverified]

= Sources and evidence
KB unavailable (wrapper 127, no MCP). Reviewed actual field, word-workspace and index code, the num-bigint parser grammar, existing Karatsuba/reduction formulas, current source-bound profiles and Rust Entry semantics. The algebra and data-structure methods are established; no novelty claim.

Evidence: protocol.json, rows.jsonl, export.json, summary.json, grid-audit.json, polynomial-replay.json, frozen-reference.json, checks/source-inputs.json and raw/executable archives described in the parent README. Plots only render the recorded Rust summaries; no Python scientific analysis or solver computation is used. All observations remain unsigned producer evidence pending independent review.
''')
