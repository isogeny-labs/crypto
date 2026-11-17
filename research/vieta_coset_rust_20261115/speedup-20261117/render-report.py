#!/usr/bin/env python3
"""Render existing frozen Rust statistics; no statistical computation."""
import json,pathlib
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import NullFormatter
import typst
r=pathlib.Path(__file__).parent
panels=[('Original4b',r/'evidence/primary-summary.json'),('Fresh8b',r/'evidence/budget-summary.json')]
rows=[]
for label,path in panels:
 s=json.loads(path.read_text())
 for g in s['groups']:
  rep,field,m,pop,identity=g['group'].split(':');ratio=g['paired_ratios']['newton/fiber-half:cold_ns'];a=g['arms']
  rows.append({'protocol':label,'source':str(path.relative_to(r)),'group':g['group'],'label':f'{field} {m} {rep} / '+('conditioned' if pop=='conditioned-sum' else 'random abscissa'),'n':a['fiber-half']['successful'],'newton_seconds':a['newton']['cold_ns'].get('median',None),'candidate_seconds':a['fiber-half']['cold_ns']['median'],'lookup_seconds':a['lookup-fiber']['cold_ns']['median'],'ratio':ratio,'lookup_ratio':g['paired_ratios']['lookup-fiber/fiber-half:cold_ns']})
# Units are rendered from native nanoseconds; medians/intervals are copied.
for row in rows:
 for key in ['newton_seconds','candidate_seconds','lookup_seconds']:
  if row[key] is not None:row[key]=row[key]/1e9
(r/'report-data.json').write_text(json.dumps({'schema':'vieta-report-render/v1','scope':'Frozen native summary values copied; presentation unit conversion only; no pooling or re-estimation','rows':rows},indent=2)+'\n')
points=[x for x in rows if x['ratio'].get('n',0)>0]
fig,axes=plt.subplots(1,2,figsize=(10.5,4.7),sharey=True,gridspec_kw={'width_ratios':[1.4,1]})
colors={'word62':'#2864a0','p224':'#8b3f80'}
for i,row in enumerate(points):
 color=colors[row['group'].split(':')[1]];v=row['ratio'];lo,hi=v['bootstrap_median95']
 axes[0].plot([lo,hi],[i,i],color=color,lw=2);axes[0].scatter([v['median']],[i],color=color,s=25,zorder=3)
 w=row['lookup_ratio'];lo,hi=w['bootstrap_median95'];axes[1].plot([lo,hi],[i,i],color=color,lw=2);axes[1].scatter([w['median']],[i],color=color,s=25,zorder=3)
for ax in axes:ax.set_xscale('log');ax.grid(axis='x',alpha=.2);ax.spines[['top','right']].set_visible(False)
axes[0].set_xticks([2,4,8,16,32,64],['2','4','8','16','32','64']);axes[1].set_xticks([.1,.25,.5,1],['0.1','0.25','0.5','1']);axes[0].xaxis.set_minor_formatter(NullFormatter());axes[1].xaxis.set_minor_formatter(NullFormatter())
axes[0].set_yticks(range(len(points)),[x['label']+(' [8b]' if x['protocol']=='Fresh8b' else '') for x in points],fontsize=8);axes[0].invert_yaxis();axes[0].axvline(2,color='#a24438',ls='--',lw=1);axes[0].set_xlabel('Newton cold / candidate cold');axes[1].axvline(1,color='#555555',ls='--',lw=1);axes[1].set_xlabel('Optimized lookup cold / candidate cold')
fig.suptitle('Local producer observations; below L2, unverified',fontsize=12);fig.text(.01,.01,'n=3 retained targets per population. Conditional native bootstrap intervals. Original word32768 references censored; fresh8b shown separately.',fontsize=7)
fig.tight_layout(rect=[0,.035,1,.94]);fig.savefig(r/'cold-ratios.svg');plt.close(fig)
head='| Workload / population | Cohort | n | Newton cold s | Candidate cold s | Paired ratio | Conditional95% | Lookup cold s |\n|---|---|---:|---:|---:|---:|---|---:|\n'
lines=[];typcells=[]
for row in rows:
 ratio=row['ratio'];pair=f"{ratio['median']:.2f}x" if 'median'in ratio else 'censored';ci=('['+f"{ratio['bootstrap_median95'][0]:.2f}, {ratio['bootstrap_median95'][1]:.2f}"+']') if 'median'in ratio else 'unavailable';base=f"{row['newton_seconds']:.4f}" if row['newton_seconds'] is not None else 'censored';cand=f"{row['candidate_seconds']:.4f}";lookup=f"{row['lookup_seconds']:.4f}"
 lines.append(f"| {row['label']} | {row['protocol']} | {row['n']} | {base} | {cand} | {pair} | {ci} | {lookup} |")
 typcells.append(', '.join('['+x+']' for x in [row['label'],row['protocol'],base,cand,pair,ci,lookup])+',')
md='''# Vieta Rust native NTT and quadratic-fiber results

Producer observations, **unverified**: PCC-20261117-b4ee50. Full cold means fresh
native field setup, certified descriptor, complete point factor base, required
lookup index, and one relation query. Query clocks/counters remain separate.
No process launch, target fixture/oracle preparation, canonical output encoding,
or independent scalar replay is charged to these worker clocks.

The explicit fast arm combines native Montgomery fields, single-field NTT128,
the unchanged half-GCD algorithm, and closed linear/quadratic roots. Schoolbook,
Euclid/Newton and field-only/NTT-only references are preserved. The optimized
lookup reference uses exactly the candidate's construction flags; setup
primitive counters match in all40 input comparisons including warmups.

## Frozen observations

[pcc:unverified] These are paired medians and conditional native bootstrap
intervals from the frozen Rust analyzer, with three retained targets per
population/configuration. Populations/cohorts are separate. The original
192-call cohort is **inconclusive as a whole**:176 calls replayed and16 reference
calls at word32768 exceeded the fixed4b multiplication cap. All failures remain.
A separately preregistered fresh8b word32768 cohort completed/replayed all48
calls. The old calls were not replaced. Both cohorts pass their A/A and positive
control groups; the fresh cohort meets its frozen local engineering criterion.

'''+head+'\n'.join(lines)+'''

![Native stored ratios](cold-ratios.svg)

Optimized lookup has lower full cold cost in every reported subgroup. These
relative gains are over the Euclid/Newton reference. The clocks remain below L2;
there is no signed independent standing or fastest-external-tool claim.

## What changed and why

Field-only calibration missed the word goal (about1.15–1.25x) while substantially
improving P224 cosets. NTT polynomial products made recursive half-GCD effective
on the word workload. Rational construction still used generic polynomial
factoring for every quadratic fiber; exact discriminant roots remove that
cost while retaining coprimality, poles, every sibling, and exact full-splitting
checks. Repeated roots are deduplicated and therefore fail a required separable
fiber-size certificate. Higher-degree fibers retain the generic finder.

NTT acceleration requires the tested primes' large power-of-two roots. Root
order is checked, twiddle/root setup is counted per Arithmetic stage, and no
global cache amortizes construction. Native field constructor internals are
inside cold clocks but unpriced in evaluator counters: fast **full cold counted
cost is incomplete**, so it is never asserted. Query counters describe evaluator
primitive operations, including NTT setup. Failed transform-prefix counts may
be incomplete when a local limit interrupts the hook; failures are never wins.

## Validation and scope

- Independent algorithmic oracle: original BigUint scalar complete-set replay;
  its implementation/execution are producer-owned and unsigned.
- Native scale core15 tests; adapter/NTT7 and worker2 groups; strict Clippy.
  Exhaustive quadratics over5,7,17,19 include repeated/nonsplit/zero cases.
- Original and repaired primary exports retained. Original metadata mislabeled
  terminal failed processes; canonical completed includes failed. The repaired
  export passes file checks, with every raw status/metric/seed/hash unchanged.
- Every cohort has a canonical file-checked export, including exploratory
  profiling. Profiling warned about timer undercoverage and supports no precise
  CPU attribution or throughput comparison.
- Root library attempt reached its180s cap and emitted unchanged-main
  hashed-ecdl-tensor::metrics errors; no full-root-suite pass is claimed.
- Word62 is a synthetic prime-field curve workload without a prime-order
  subgroup assertion. P224 is the standard-curve fixture. Rational benchmarks
  use fully certified Mobius conjugates of squaring; this is not the cost of
  finding auxiliary isogenies.
- The general last-fall-degree argument and arbitrary-depth formal normal-form
  instantiation remain unverified. These measurements do not establish them.
- Local/Git-only evidence, zero paid/cloud experiments, no local S3 operation,
  no protected benchmark-index/leaderboard promotion or availability assertion.

## Reproduce/check

Source for the first cohort:149fd91f33844390511f6310a21ffa50ad607542;
source for the new8b cohort:2595478e2ac478055c7526f19af6dfc1fe604f8f.
`evidence/budget-source-equivalence.json` shows the only worker change is the
operation cap; core, adapter and frozen analyzer are unchanged.

```sh
cargo test --release --locked --manifest-path research/vieta_coset_rust_20261115/speedup-20261117/fiber-budget/Cargo.toml
cargo build --release --locked --manifest-path research/vieta_coset_rust_20261115/speedup-20261117/fiber-budget/Cargo.toml
research/vieta_coset_rust_20261115/speedup-20261117/fiber-budget/target/release/vieta-fiber-budget-lab replay research/vieta_coset_rust_20261115/speedup-20261117/runs/primary-budget8b/0-word62-coset-m32768
```

These commands test/replay saved evidence. Any new measurement needs its own
frozen protocol and new output directory. See [FIBER_PLAN.md](FIBER_PLAN.md),
[TRANSFORM_PLAN.md](TRANSFORM_PLAN.md), [protocol-primary-v2.json](protocol-primary-v2.json),
[protocol-budget8b.json](protocol-budget8b.json), the registration addenda and
[evidence/control-audit.json](evidence/control-audit.json). The control auditor
was authored after data solely to check stored frozen statistics; it is not an
independent validator or a replacement analysis.
'''
(r/'RESULTS.md').write_text(md)
typ='''#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: none)
= Vieta Rust: native NTT and exact quadratic fibers
Producer observations — unverified (PCC-20261117-b4ee50). Below-L2 local clocks.

Full cold charges native field setup, full certified point base, required lookup
index and one relation query. Fixture/oracle preparation, output encoding and
independent replay are outside worker clocks. Native field setup operations
are unpriced in counters; no complete fast cold-count claim.

#image("cold-ratios.svg", width: 100%)

#table(columns: (2.8fr,1fr,1fr,1fr,1fr,1.6fr,1fr), inset: 3pt,
  table.header([Workload/population],[Cohort],[Newton s],[Candidate s],[Paired ratio],[Conditional95%],[Lookup s]),
'''+ '\n'.join(typcells)+'''
)

Three retained targets per population. Medians/intervals copied from the frozen
Rust analyzer; no pooling. Optimized lookup is cheaper in every subgroup.

#pagebreak()
= Method and retained failures
The fast arm combines native Montgomery fields, NTT128 products, unchanged
half-GCD and exact linear/quadratic roots. Every sibling fiber, pole,
coprimality and separability/full-splitting check remains. Repeated roots are
deduplicated; incomplete splitting fails certification. Higher-degree roots
use the original generic finder. Root order and all transform twiddle/setup
multiplications are checked/counted per Arithmetic stage. No global cache.

Original cohort:192 calls,176 successful replays,16 references stopped by the
frozen4b multiplication cap at32768; the full cohort is inconclusive. The new
8b word32768 protocol has fresh targets and48/48 calls/replays. It does not
replace those failures. A/A and positive control groups pass;40 candidate/lookup
construction comparisons (including warmups) have equal primitive counts.

= Validation and limits
Native core15 tests plus7 library and2 worker groups pass with strict Clippy.
Exhaustive small-field quadratics cover repeated, nonsplit and zero cases.
BigUint scalar complete-set replay is algorithmically separate but unsigned and
producer-owned. Final exports for every cohort pass file checks; the original invalid
outcome-class export is preserved beside its corrected derivative and exact
difference audit. Canonical terminal completed includes failed, never success.

The root library attempt hit180s and emitted unchanged-main hashed-ecdl-tensor
metrics-module errors; no full-root pass. Profiling reported timer undercoverage
and supports no quantitative CPU or throughput claim. Word62 is synthetic with
no prime-order subgroup assertion. P224 uses the standard-curve fixture;
rational maps are certified conjugated-squaring fixtures. Transform acceleration
uses the specific primes' power-of-two roots. Arbitrary prime moduli need an
appropriate product backend.

The general degree-bound argument and arbitrary-depth formal instantiation
remain unverified. No independent signed standing, L2 admission, fastest-external
claim, local S3 availability assertion, or protected leaderboard promotion.

= Reproducibility
First source: #raw("149fd91f33844390511f6310a21ffa50ad607542")
New8b source: #raw("2595478e2ac478055c7526f19af6dfc1fe604f8f")

Protocols: #raw("protocol-primary-v2.json") and #raw("protocol-budget8b.json").
Raw records, native summaries, scalar replays, control audit, failed export,
repair audit and all preregistration corrections are preserved in this study.
See RESULTS.md for commands, exact paths and cost boundaries. Git-only/local;
no paid/cloud experiment, local S3 operation or protected promotion.
'''
(r/'report.typ').write_text(typ)
typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r.resolve()))
print('Rendered frozen native values:',len(rows),'rows; no new measurements.')
