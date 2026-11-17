from pathlib import Path
import json,collections,csv,hashlib
D=Path(__file__).parent;v=json.loads((D/'run.stdout').read_text());status={'schema':'research.bounded-software-graph-integration/v1','exit_code':json.loads((D/'receipt.json').read_text())['exit_code'],'models':len(v['cases']),'arms':{},'controls':v['controls'],'scientific_execution':False,'timing_ratio':None,'independent_review':'pending'}
rows=[]
for arm in ['eager','candidate']:
 rr=[r for c in v['cases'] for r in c[arm]['rows']];status['arms'][arm]={'states':len(rr),'graph_status_counts':dict(collections.Counter(r['graph']['status']for r in rr)),'cycles':sum(r['graph'].get('cycles_delta',0)for r in rr),'terminal_returns':sum(r['graph']['answer']is not None for r in rr),'signed_recovery_successes':sum(r['signed_ok']for r in rr)}
 for c in v['cases']:
  for r in c[arm]['rows']:
   z=r['graph'];rows.append({'arm':arm,'p':c[arm]['p'],'index':r['index'],'status':z['status'],'graph_operations':z.get('graph_operations',0),'equation':json.dumps(z.get('equation')),'native_phase_counters':json.dumps({'walk':r['counts'],'graph':z})})
for c in v['cases']:
 for e,p in zip(c['eager']['rows'],c['candidate']['rows']):
  assert e['graph'].get('equation')==p['graph'].get('equation')
  assert e['graph']['answer']==p['graph']['answer']
status['matching_emitted_equations']=True
(D/'status.json').write_text(json.dumps(status,indent=2)+'\n')
with (D/'phases.csv').open('w')as f:
 w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
report='''# Propagated Kummer states connected to signed relation graphs

The fixed original-G-step software trajectory now passes recovered signed divisors into the existing signed graph collector. On21 retained models with3 transitions each, both arms emit35 identical equations. All35 decompositions per arm pass exact point-divisor validity and sum checks. Every inserted prefix agrees with a separately implemented dense Gaussian elimination reference over F_r.

## Observations

| Observation | Eager | Candidate |
|---|---:|---:|
| Propagated states |63|63|
| Exact signed recoveries |61|63|
| Inserted graph edges |35|35|
| Nonsplit quadratic |26|26|
| Recovery failure |2|0|
| Degree1 graph skip |0|2|
| Graph cycles |0|0|
| Geometric terminal returns |0|0|

The two added degree1 recovery successes do not increase emitted-edge coverage: the preserved graph interface accepts degree2 divisors with distinct rational roots. No trajectory was extended to seek a cycle. Prior recovery failures remain in source-parent evidence and this run's eager rows.

Three separate synthetic signed-equation controls over F_101 return the planted value7. A positive cycle stops at prefix3; a negative cycle anchors a node and stops at prefix2; a target-free repeated-edge cycle increments zero_coefficient then stops at prefix4. The dense reference returns the same value at every prefix. Each control has an inconsistent trailing sentinel that is not consumed after stopping. These controls exercise elimination and stopping, separately from the63 geometric states.

## Interfaces and correctness boundary

The paired walker keeps (K(R),K(R+G)) and labels R=aG+bQ. Recovery consumes this propagated pair directly. For recovered degree2 divisor (u,v), the adapter splits u into distinct roots x0,x1, obtains yi=v(xi), uses node xi with sign determined by yi<=p/2, and inserts s0 X0+s1 X1=a+b*k. It deduplicates signed divisors up to negation as the collector does. Exact factor verification occurs after insertion; aG+bQ reference construction also follows recovery and insertion. The adapter receives no expected divisor.

Dense elimination retains every inserted equation, with node variables and k, and produces a target value only from a row whose node coefficients vanish. Graph operations, constraints and prefix length are recorded. Point-divisors are not assumed individually r-torsion; subgroup assertions apply to G,Q and each hypothetical returned scalar is checked by multiplication and planted-answer equality. No geometric scalar-return branch was exercised here.

This wrapper attempts recovery on every state before the split gate, continuing the preceding recovery validation. The historical collector normally filters using general Kummer coordinates before attempting recovery. Thus emitted-edge semantics are compared, not production recovery scheduling or total costs. Degree1 skips are explicit. Original-G repair/fallback and Delta-zero branches are not exercised by these fixtures.

## Cost boundary and remaining integration

Separate records contain initialization, transition, recovery, splitting/sign extraction, graph insertion, and factor/reference verification. Ring snapshots and graph-operation deltas accompany each emitted equation. Dense elimination is a checking reference, with no independent operation census; allocation, hashing, fixture parsing, and reference work are not a complete priced cost model. Underlying square-root nonresidue search remains work, and native field counters are incomplete for some modular exponentiation. No timing or rho ratio is reported.

The next concrete integration gap is a fixed geometric relation-cycle fixture that enters via propagated original-G states and reaches scalar-return verification. The current63-state trajectory contains no cycle; the terminal controls cover only signed equations. Duplicate and repeated-root geometric gates likewise have no observed coverage. Complete production scheduling, restart behavior, stopping and source-elliptic verification remain separate interfaces to qualify.

## Custody and references

Frozen PLAN.md preceded implementation and the bounded offline Cargo invocation. receipt.json records exit0. run.stdout retains both arms, all equations, phase counters and terminal controls; status.json and phases.csv are standardized derivatives. All changes are private. Independent review pending.

Chung–Costello–Smith, Fast, uniform, and compact scalar multiplication, sections2,6,7: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf. Existing collector protocol: research/kummer_differential_collector_20261022/PROTOCOL.md; SignedGraph implementation in native/src/three_leads.rs. The Gaudry–Thome–Theriault–Diem double-large-prime paper was located at https://eprint.iacr.org/2004/153.pdf but fetch returned403; no theorem claim rests on its search summary. Plan and source hashes are in manifest.json.
'''
(D/'REPORT.md').write_text(report)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="850" height="235" viewBox="0 0 850 235"><rect width="850" height="235" fill="white"/><g font-family="sans-serif" fill="#17324d"><text x="20" y="30" font-size="19">Fixed original-G trajectory: 63 states per arm</text><rect x="20" y="55" width="455" height="45" fill="#79b1cc"/><text x="32" y="84">35 inserted signed edges (both arms)</text><rect x="20" y="110" width="338" height="45" fill="#d3dce5"/><text x="32" y="139">26 nonsplit states (both arms)</text><text x="20" y="183">2 eager recovery failures → 2 candidate degree1 skips</text><text x="20" y="214">0 geometric cycles; terminal controls are separate synthetic equations</text></g></svg>''';(D/'coverage.svg').write_text(svg)
typ='#set page(paper:"a4",margin:16mm)\n#set text(size:9pt)\n#text(size:17pt,weight:"bold")[Kummer state to signed graph]\n#image("coverage.svg",width:100%)\n'
for para in report.split('\n\n'):
 if para.startswith('|'):
  continue
 if para.startswith('#'):
  typ+='\n#text(weight:"bold")['+para.lstrip('# ').replace('#','')+']\n'
 else:typ+='\n#text('+json.dumps(para,ensure_ascii=False)+')\n'
(D/'report.typ').write_text(typ)
