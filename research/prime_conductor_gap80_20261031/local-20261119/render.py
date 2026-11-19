#!/usr/bin/env python3
"""Presentation only. All aggregates come from the native analysis/manifest."""
from pathlib import Path
import json,os
os.environ.setdefault('MPLCONFIGDIR','/tmp/conductor-local-mpl');os.environ.setdefault('XDG_CACHE_HOME','/tmp/conductor-local-fonts')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
s=Path(__file__).resolve().parent;v=json.loads((s/'summary.json').read_text());out=s/'reports';out.mkdir(exist_ok=True)
analysis=v['ECDLP_analysis']['groups'];methods=['rho-counted','bsgs-exact-key','rho-aa'];method_names=['rho','BSGS','rho A/A']
panels=[]
for field,label in [(202,'pilot-l0'),(96,'pilot96-l0')]:
 p=json.loads((s/'results'/label/'plan.json').read_text());slugs=list(dict.fromkeys(w['curve']['slug'] for w in p['workloads']))
 rows={(r['curve_slug'],r['arm']):r for r in analysis if r['field_bits']==field}
 raw=[json.loads(l) for l in (s/'results'/label/'records.jsonl').read_text().splitlines()]
 panels.append((field,slugs,rows,raw))
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'axes.spines.top':False,'axes.spines.right':False})
fig,axes=plt.subplots(1,2,figsize=(10.6,3.8),constrained_layout=True)
for ax,(field,slugs,rows,raw) in zip(axes,panels):
 for side,(slug,color,title) in enumerate(zip(slugs,['#356c9b','#d58b36'],['source','image'])):
  x=[i+(-0.19 if side==0 else 0.19) for i in range(3)]
  vals=[rows[(slug,m)]['mean_solve_seconds'] for m in methods]
  ax.bar(x,vals,width=.35,label=title,color=color,alpha=.83)
  for pos,m in zip(x,methods):
   samples=[r['time']['solve_wall_ns']/1e9 for r in raw if r['workload']['curve']['slug']==slug and r['arm']==m and not r['warmup']]
   ax.scatter([pos]*len(samples),samples,color='#172333',s=14,zorder=4)
 ax.set_xticks(range(3),method_names);ax.set_ylabel('Solve seconds (L0)');ax.set_title(f'{field}-bit field; one target, two rounds');ax.grid(axis='y',alpha=.18);ax.legend(frameon=False)
fig.suptitle('Observed local ECDLP controls — different subgroup orders; no admitted speedup',fontsize=11)
for ext in ['svg','png']:fig.savefig(out/f'ecdLP.{ext}',dpi=120)
plt.close(fig)
sp=json.loads((s/'results/scalar-l0/plan.json').read_text());ordered=list(dict.fromkeys(w['curve']['slug'] for w in sp['workloads']));sm={r['curve_slug']:r for r in v['scalar_summary']['descriptive_by_curve']}
fig,axes=plt.subplots(1,2,figsize=(10.6,3.4),constrained_layout=True)
for ax,field,slugs in [(axes[0],202,ordered[:3]),(axes[1],96,ordered[3:])]:
 ax.bar(range(3),[sm[z]['mean_ns_per_mul']/1000 for z in slugs],color=['#356c9b','#d58b36','#64966b'],alpha=.83)
 ax.scatter(range(3),[sm[z]['median_ns_per_mul']/1000 for z in slugs],color='#172333',s=20,label='median')
 ax.set_xticks(range(3),['source','image','u=2 control']);ax.set_ylabel('Microseconds per multiplication');ax.set_title(f'{field}-bit field; 32 points × 6 measured rounds');ax.grid(axis='y',alpha=.18);ax.legend(frameon=False)
fig.suptitle('Batches of64 repeated multiplications; setup/verification excluded; descriptive L0',fontsize=11)
for ext in ['svg','png']:fig.savefig(out/f'scalar.{ext}',dpi=120)
plt.close(fig)
rows=''
for field,slugs,rs,_ in panels:
 for method,name in zip(methods,method_names):
  rows+=f'[{field}], [{name}], [{rs[(slugs[0],method)]["mean_solve_seconds"]:.2f}], [{rs[(slugs[1],method)]["mean_solve_seconds"]:.2f}],\n'
scalar_rows=''
for field,slugs in [(202,ordered[:3]),(96,ordered[3:])]:
 for role,slug in zip(['source','image','u=2'],slugs):
  r=sm[slug];scalar_rows+=f'[{field}], [{role}], [{r["mean_ns_per_mul"]/1000:.1f}], [{r["median_ns_per_mul"]/1000:.1f}], [{r["samples"]}],\n'
text='''#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set heading(numbering:none)
= Authorized local isogeny/ECDLP measurements
19 November2026. Actual existing native solvers; all timings descriptive L0.

Both field pilots completed24 parent-verified solve attempts and24 exact native
replays. Independent PARI checks confirm all recovered scalars. Each field has
one shared ECDLP target, two endpoint models, two measured rounds and three
arms. This is not24 independent targets. No L2 wall admission or conductor-
effect conclusion is established. [pcc:unverified]

#table(columns:(1fr,1.3fr,1.3fr,1.3fr),inset:5pt,stroke:0.35pt,
 [*Field bits*],[*Method*],[*Source mean s*],[*Image mean s*],
'''+rows+'''
)
#image("ecdLP.svg",width:100%)

The202-bit field uses prime subgroup1125023250121 and a certified conductor
1→2 degree2 map. The96-bit field uses subgroup125000000302579 and a degree5
horizontal map (conductors1→1). Different subgroup orders preclude reading
the two field rows as a scaling comparison. [pcc:unverified]

The rho partitions hash coordinates, so paired scalar/algorithm seeds do not
make identical endpoint walks. Their work counts differ; A/A controls replay
the same work counts on each model. BSGS rebuilds its full table every run;
its table comparison/allocation work and rho canonicalization remain unpriced.
GAE figures are lower bounds. No operation-cost or wall ranking is admitted.
[pcc:unverified]

= Execution conditions
The host reports16GiB cgroup memory and four CPU cores quota. Automatic
reserved-core admission refuses unavailable CPU/memory PSI. The separately
preregistered local diagnostic uses --cpus none and keeps actual L0 records;
perf events are also unavailable. No build/test/other benchmark overlapped
measured calls. Timings cover the harness solve interval, including per-call
solver setup, not complete large-isogeny construction. [pcc:unverified]

#pagebreak()
= Scalar multiplication across varied points
The new scalar-timing adapter calls the original CountedGroup::mul implementation
in the qualified release library. It adds timing/recording, not new arithmetic
or an ECDLP solver. Three focused native controls and clippy pass; historical
pilot plan bytes match. [pcc:unverified]

#image("scalar.svg",width:100%)
#table(columns:(1fr,1.3fr,1fr,1fr,1fr),inset:4pt,stroke:0.35pt,
 [*Field*],[*Model*],[*Mean µs*],[*Median µs*],[*Samples*],
'''+scalar_rows+'''
)
Each model uses32 varied target bases Q, six measured rounds plus one warmup.
Every timed sample repeats the same [k]Q64 times; k varies by paired round seed.
Means/medians above exclude warmups and describe batched throughput, not cold
single-call latency. Source/image/u2 models retain their exact original IDs.
Paired reversal balances pairwise orientation, not every three-model position.
[pcc:unverified]

The timer includes counted multiplication, loop/black-box and ledger updates.
Curve/target construction, independently computed expected results, verification
and output are excluded and retained separately. Field/group ledgers are written
for every sample. All1344 batch outputs independently match PARI, including
warmups. Wrong output and truncated-coverage audit controls reject.
[pcc:unverified]

The audit's first three versions are preserved: a reserved GP name, duplicate
hex prefix and failed text replacement caused parser/coverage failures. Stderr
and expected-count checks rejected their zero-sample outputs. The final native
checker catches exceptions and requires exact coverage. No failed audit is
reported as successful. [pcc:unverified]

#pagebreak()
= Original80-bit construction: actual capped attempt
The unchanged prime degree is729845429276318409687041. The qualified prepared
native constructor was executed locally with G plus32 frozen source points,
64-sized polynomial tiles, projective batches64, a12GiB address-space cap and
60second watchdog. The maximum global window is an index guard, not a full
allocation. [pcc:unverified]

#table(columns:(1.2fr,2.3fr),inset:5pt,stroke:0.35pt,
 [*Actual disposition*],[*Recorded value*],
 [Attempt exit],[124; watchdog interrupt retained],
 [Quotient and images],[Not produced],
 [Durable partial coverage],[Not produced; prototype has no resumable journal],
 [Original crossing complete],[No],
)
The9.7TB necessary model minimum concerns a single full rectangle; blocking
can avoid that allocation. This timeout does not establish nonexistence or a
universal runtime lower bound. A complete affordable schedule remains open.
No stored prefix or partial tile is turned into a completed quotient.
[pcc:unverified]

== Index-calculus request
The actual wide-field run request for ic.pipeline is rejected before solving:
“ic.pipeline has no prime_mont4 adapter”. The supported wide adapter methods
are rho.negation and bsgs.prime_mont4_v1. No IC runtime or complexity conclusion
is invented. Existing word-field CM controls have their own applicability and
are not substituted for the requested wide-field run. [pcc:unverified]

== Requirement status
#table(columns:(1.3fr,2.2fr),inset:4pt,stroke:0.35pt,
 [*Requirement*],[*Evidence and remaining scope*],
 [Run locally],[Executed with explicit user authorization; EKS is not a prerequisite],
 [Endpoint ECDLP],[Two small-degree control maps, one target per field; all24 attempts/replays/independent checks pass],
 [Dozens of scalar points],[32 points per model in both fields; six measured rounds, warmup and coordinate controls],
 [Full ECDLP32-target protocol],[Still open; pilots do not replace it],
 [80-bit prime crossing],[Still open: no target quotient/images or endpoint timings],
 [Admitted performance result],[None: L0 clocks, small pilot sample and unpriced work retained],
 [Raw-data publication],[Local bytes/hashes preserved; S3 identity remains unavailable],
)
All producer evidence remains unsigned/not_covered by a signed independent
verdict. PCC-20261119-22bcb7/77c30f/5c4b81 bind native data, audits and caps;
PCC-20261119-4a8667 binds the descriptive first-field analysis. No leaderboard
row or world-record claim is promoted. [pcc:unverified]

#pagebreak()
= Next larger map: concrete proposal, not executed
Retained CM case0 has field prime5629830687772387, ordinary source order of
conductor1, and a certified floor model of conductor33555419. The original
cofactor-one subgroup order is5629830654216971. Its explicit point transport
is still missing here; these retained equations are not a new evaluated crossing.
[pcc:unverified]

The existing complete evaluator takes a rational kernel on auxiliary curve T.
The old benchmark points are on its quadratic twist E. With twist factor d,
E has coefficients d²A,d³B. If f is the normalized quotient x-map on T, the
conjugated map has formula (x,y) ↦ (d f(x/d), y f′(x/d)); its codomain has
coefficients d²A′,d³B′. Square-root factors cancel. Source/output validation
must use the actual twisted models. Implementation and cost remain open.
[pcc:unverified]

#image("twist-bridge.svg",width:100%)

Implement a separately versioned native evaluator, preserve immutable defaults,
and qualify tiny square/nonsquare twists against direct PARI kernel-polynomial
maps. Then freeze the retained large case and attempt its full map locally,
verify the retained codomain, transported prime-order targets and dual
composition on the original subgroup. Do not silently replace that subgroup
or infer map correctness from j or field size. [pcc:unverified]

This proposal is a lower milestone than the original80-bit criterion. The
separate4774-bit Frobenius-conductor arithmetic certificate supplies no evaluated
crossing or endpoint timings. No broader record search is completed by this
local report. [pcc:unverified]

== Sources and exact reproduction
Current source: native/complete_pipeline.cpp and its frozen prepared evaluator;
actual ecbench runner/spec/record and prime_mont4 implementation; paired-schedule
and retained-fixture conductor certificates. Old CM input lineage is
conductor_gap_cm_orbits_20261015/prepared/kernels-reference.txt and catalog.json;
the constructor derivation is its native/CONSTRUCTOR.md. See exact argv/source/
rlib hashes in build and pre-call manifests. [pcc:unverified]

Primary ordinary-order background:
#link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[Galbraith, introduction/section2],
#link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf")[Galbraith–Gilchrist–Robert, section2.1].
KB remains unavailable; prior exact primary PDF review is reused. No latest-
literature or novelty assertion. [pcc:unverified]
'''
for old,new in {'November2026':'November 2026','completed24':'completed 24','and24':'and 24','not24':'not 24','The202':'The 202','The96':'The 96','subgroup112':'subgroup 112','subgroup125':'subgroup 125','degree2':'degree 2','degree5':'degree 5','conductors1':'conductors 1','reports16GiB':'reports 16 GiB','uses32':'uses 32','[k]Q64':'[k]Q 64','All1344':'All 1344','Original80':'Original 80','is729845':'is 729845','plus32':'plus 32','batches64':'batches 64','a12GiB':'a 12 GiB','60second':'60 second','The9.7TB':'The 9.7 TB','all24':'all 24','ECDLP32':'ECDLP 32','case0':'case 0','prime562':'prime 562','conductor1':'conductor 1','conductor335':'conductor 335','is562':'is 562','original80':'original 80','separate4774':'separate 4774','section2':'section 2'}.items(): text=text.replace(old,new)
(out/'twist-bridge.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="960" height="280" viewBox="0 0 960 280"><rect width="960" height="280" fill="#f5f8fc"/><g font-family="DejaVu Sans,sans-serif" fill="#18364c" text-anchor="middle"><rect x="90" y="20" width="200" height="65" rx="8" fill="#ddebf6"/><rect x="670" y="20" width="200" height="65" rx="8" fill="#ddebf6"/><rect x="90" y="195" width="200" height="65" rx="8" fill="#e4f0e7"/><rect x="670" y="195" width="200" height="65" rx="8" fill="#e4f0e7"/><text x="190" y="48" font-size="20">T: auxiliary source</text><text x="190" y="71" font-size="13">rational kernel on this model</text><text x="770" y="48" font-size="20">T′: quotient</text><text x="770" y="71" font-size="13">normalized map recipe</text><text x="190" y="224" font-size="20">E: actual source</text><text x="190" y="247" font-size="13">original benchmark subgroup</text><text x="770" y="224" font-size="20">E′: actual target</text><text x="770" y="247" font-size="13">point transport still pending</text><path d="M302 52H658m-8-6 8 6-8 6M190 183V97m-6 8 6-8 6 8M770 97v86m-6-8 6 8 6-8" fill="none" stroke="#456d8d" stroke-width="2"/><path d="M302 227H658m-8-6 8 6-8 6" fill="none" stroke="#528363" stroke-width="2" stroke-dasharray="7,5"/><text x="480" y="42" font-size="14">φT over Fp (case not executed here)</text><text x="480" y="216" font-size="14">proposed φd over Fp</text><text x="313" y="133" font-size="15">ιd over Fp²</text><text x="645" y="154" font-size="15">ι′d⁻¹ over Fp²</text></g></svg>''')
(out/'report.typ').write_text(text)
# Markdown numerical prose is generated from the same checked native means.
md='# Local execution results\n\n| Field bits | Method | Source mean seconds | Image mean seconds |\n|---|---|---:|---:|\n'
for field,slugs,rs,_ in panels:
 for method,name in zip(methods,method_names):md+=f'| {field} | {name} | {rs[(slugs[0],method)]["mean_solve_seconds"]:.2f} | {rs[(slugs[1],method)]["mean_solve_seconds"]:.2f} |\n'
md+='\nEach cell averages two runs of one target. All24 solves and24 exact replays pass, with independent PARI recovery verification. Source/image conductor ratios are2 in the202-bit field and1 in the96-bit control. Different rho walks and subgroup sizes prevent a conductor or scaling conclusion. All clocks are descriptive L0, with unpriced work retained. [pcc:unverified]\n\n'
md+='Scalar controls cover32 varied target bases per model, six measured rounds plus warmup and1344 independently verified batches of64 calls across six models. Native means/medians and exact source/sample identities are in [summary.json](summary.json). [pcc:unverified]\n\n'
md+='The actual80-bit native construction attempt reaches its60second cap, exit124, without quotient/images or durable completed coverage. The IC dispatch rejects the missing wide adapter. Neither is presented as a successful crossing or an impossibility proof. [pcc:unverified]\n\n'
md+='[Four-page report](reports/report.pdf), [execution protocol](PROTOCOL.md), [scalar protocol](SCALAR_PROTOCOL.md), [original80-bit attempt](CONSTRUCTION_ATTEMPT.md), [next larger map proposal](LARGER_MAP_NEXT.md). [pcc:unverified]\n\n'
md+='Producer evidence PCC-20261119-22bcb7/77c30f/5c4b81 remains unsigned/not_covered. Large raw artifact publication is pending S3 identity; exact local data and hashes are preserved, with no false remote pointer or leaderboard promotion. [pcc:unverified]\n'
(s/'RESULTS.md').write_text(md)
