"""Layout only: render values already computed by the frozen Rust programs."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
p=Path(__file__).resolve().parent
s=json.loads((p/'summary.json').read_text());g=json.loads((p/'grid-audit.json').read_text())
ids=[c['instance'] for c in g['goal_criteria']]
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,axes=plt.subplots(2,1,figsize=(9,5.8),layout='constrained')
for ax,metric,title in zip(axes,['instructions','setup_instructions'],['Complete cold query: parsing, setup, oracle, replay and exit','Cold setup: includes complete field-table construction']):
 for offset,arm,color in [(-.18,'baseline','#697586'),(.18,'elimination','#007f8b')]:
  vals=[s['cells'][f'{i}/{arm}']['metrics'][metric] for i in ids]
  x=[i+offset for i in range(len(ids))];ax.bar(x,[v['median'] for v in vals],width=.34,label=arm,color=color)
  ax.errorbar(x,[v['median'] for v in vals],yerr=[[v['median']-v['min'] for v in vals],[v['max']-v['median'] for v in vals]],fmt='none',ecolor='black',capsize=2)
 ax.set_title(title,loc='left',fontweight='bold');ax.set_ylabel('Million simulated instructions');ax.yaxis.set_major_formatter(FuncFormatter(lambda x,_:f'{x/1e6:g}'));ax.set_xticks(range(len(ids)),[i.replace('scale-n','n').replace('-d12-','\n').replace('distinct-planted-0','planted') for i in ids]);ax.spines[['top','right']].set_visible(False);ax.grid(axis='y',alpha=.15);ax.set_axisbelow(True)
axes[0].legend(frameon=False,ncol=2)
fig.suptitle('Exact half-trace table construction · fixed d12 IC queries',fontweight='bold')
fig.text(.5,-.015,'TZH-20261123 · five repeats per cell · whiskers: observed ranges · unsigned producer evidence',ha='center',fontsize=8)
fig.savefig(p/'instructions.svg',bbox_inches='tight');fig.savefig(p/'instructions.png',dpi=150,bbox_inches='tight');plt.close(fig)
rows=[];md=[]
for c in g['goal_criteria']:
 i=c['instance'];b=s['cells'][i+'/baseline']['metrics']['instructions'];a=s['cells'][i+'/elimination']['metrics']['instructions'];label=i.replace('scale-n','').replace('-d12-','/').replace('distinct-planted-0','planted')
 rows.append(f'[{label}], [{b["median"]:,}], [{a["median"]:,}], [{c["whole_removed_fraction"]:.2%}], [{c["oracle_removed_fraction"]:+.2%}],')
 md.append(f'| {label} | {b["median"]:,} | {a["median"]:,} | {c["whole_removed_fraction"]:.2%} | {c["oracle_removed_fraction"]:+.2%} |')
(p/'results-table.md').write_text('| Fixed d12 query | Baseline cold Ir | Elimination cold Ir | Cold saved | Oracle saved |\n|---|---:|---:|---:|---:|\n'+'\n'.join(md)+'\n')
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.5em)
= Exact half-trace tables for faster cold IC setup
TZH-20261123 · ECC2K-130 algebraic index-calculus relation queries

The fixed degree131 uniform query uses *20.50% fewer complete-cold-query simulated instructions* than the merged adjoint solver; the degree83 control uses *15.78% fewer*. Uniform oracle counts increase slightly, by 0.22% and 0.11%, within the frozen 1% ceiling. All 90 primary outputs agree. Unsigned producer statement PCC-20261123-811919. [pcc:unverified]

#image("instructions.svg", width: 100%)

#table(columns: (1.4fr,1.2fr,1.2fr,1fr,1fr), inset: 4pt, [Fixed d12 query], [Baseline cold Ir], [Elimination cold Ir], [Cold saved], [Oracle saved],
'''+ '\n'.join(rows)+''')
Five repeats per arm/cell; whiskers are recorded min/max, not population intervals. Negative oracle-saved values mean increased oracle cost. The saving is in cold field-table setup, including immediate-cancellation cases; no relation-yield or warmed-throughput gain is established. [pcc:unverified]

#pagebreak()
= Construct the same map with fewer basis squarings
The general half-trace function was already a cached linear byte lookup. Its cold initialization constructed every basis image with a separate Frobenius squaring chain. The source-bound baseline profile identifies that construction stack; the new code replaces it with one packed matrix inverse and leaves every lookup entry unchanged.

For odd n, let `B(x)=x²+x+Tr(x)`. Its kernel is zero: tracing B(x)=0 gives Tr(x)=0, so x²+x=0 and x is in F2; B(1)=1. Thus B is invertible. If `epsilon=((n+1)/2) mod 2`, the ordinary half-trace H satisfies `H(x)²+H(x)=x+Tr(x)` and `Tr(H(x))=epsilon*Tr(x)`. Consequently,

`H(x)=B^-1(x)+(1+epsilon)*Tr(x)`.

The correction fixes the exact half-trace root rather than any quadratic solution. For degrees83 and131, epsilon=0 and H(1)=0. Tests also cover the other parity with degrees5/13. The existing Newton trace mask supplies B's rank-one correction. Building B takes n basis squarings; packed binary elimination produces its inverse; the unchanged linear-combination loop fills the byte tables. All work occurs on first use and is charged in each fresh process. No pre-generated trace/half-trace tables are shipped.

Dispatch checks the complete polynomial: `X^83+X^45+X²+X+1` or `X^131+X^13+X²+X+1`. The NIST131 polynomial is a different field representation and explicitly fails this dispatch. Other fields and singular inversions retain scalar chains. The solver, factor base, signs, quotient rows, model order, limits, root/lift checks and group replay are unchanged. This does not shrink a summation-polynomial system or establish new Z[tau] relation-finding leverage.

= Exact checks and honest scope
The focused release suite passes44 checks. Every basis image and every byte-table entry agrees with scalar-chain references, including trace-one inputs and H(1); the alternate131-bit modulus and a singular reducible input are canaries. Two frozen executables agree across all65 original queries, retaining130 rows. A separate schoolbook polynomial/group checker replays92 development tuples and all60 primary tuples, and checks all65 admissions/negative fixtures. These remain producer-written checks. [pcc:unverified]

Flow: 90 planned,90 attempted,90 completed,0 excluded,0 retries. Both fixed uniform cells exceed the frozen5% cold criterion; all six A/A checks and the1% cold/oracle no-regression ceiling pass. The online penalties are actual measured differences, not dismissed as noise. No native clock, population interval, external-best ranking or complete-DLP result is claimed. [pcc:unverified]

#pagebreak()
= Reproduction and qualification
Numerical source:8d3905a93a5197ee0c19117f4052468bf6846c47. Frozen protocol publication:8453eb6263d6a9e50f21e657228c877b7520487a, before primary data. Protocol hash:8d1b3ce420e754021767778ca5151e5188e37b0239af6fca52754a7cccfb3186.

One release harness, Rust1.100.0, same non-field source and flags. The study-only `legacy-halftrace` feature selects the exact original f2m source snapshot; default selects the production constructor. Separate executables prevent a shared global cache from applying one builder to both arms. Runtime arm guards reject incorrect labels; both executable hashes and source variants are frozen. Integrated counts include compiler/layout effects. A/A uses the same baseline binary.

Intel Xeon Platinum8573C; Callgrind3.24.0; CPU0,2GiB child limit, zero retries and zero paid resources. Both partition calibrations pass the0.1% tolerance. The calibration/pilot/freeze/primary sequence finishes inside the600second guard. Whole cold Ir includes parsing, fixture/context/base setup, complete cache construction, admission, oracle search, roots/signs, replay, serialization, destruction and exit. The compact-field counters exclude general F2m construction and cannot price this change. [pcc:unverified]

The broad root release build timed out with exit124 after600seconds compiling dependencies, before tests ran. Actual numeric modules are compiled by the focused facade; frozen SAT/chain interfaces are imported but not executed. Full-library integration, current CI/review and independent scientific admission remain separate gates. [pcc:unverified]

Evidence: protocol.json, export.json, summary.json, grid-audit.json, polynomial-replay.json, checks/source-inputs.json, RAW-QUERIES.tar.xz and FROZEN-BINARIES.tar.xz. Both full source variants, locked dependencies, build failures/corrections and all raw outputs remain preserved. Initial dispatch testing used NIST131 as a positive fixture; the exact guard rejected it. The corrected test explicitly covers ECC2K131 and keeps NIST131 as a negative canary.

= Sources and follow-up
KB unavailable (wrapper127, no MCP). Reviewed actual trace/half-trace cache and compact-field sources plus the source-bound profile. FLINT's finite-field factoring documentation, Lemma5, states the Artin–Schreier kernel/image and linearity: #link("https://flintlib.org/doc/algorithms/multivariate/modp-factoring.html")[official documentation]. Normalized inversion is an established linear-algebra construction; no novelty claim.

The next independent lead is reducing repeated compact-map construction/cache access after the global table exists, with a warmed-oracle comparison to address the observed online penalty. Current results establish cold setup improvement on the disclosed queries; cache reuse in a long-running relation collector needs its own measured boundary. Existing admitted leaderboard/runtime rows remain historical. [pcc:unverified]
''')
