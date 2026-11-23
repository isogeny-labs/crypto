from pathlib import Path
import json,csv,hashlib,collections
D=Path(__file__).parent;x=json.loads((D/'run.stdout').read_text());status={'schema':'research.software-integration/v1','transition_family':'original_non_torsion_G_step','exit_code':0,'models':len(x['cases']),'transitions_per_arm':63,'selected_pairs_and_labels_equal':all(c['equal_selected_state'] for c in x['cases']),'recovery_neighbour':'propagated pair[1], before reference construction','scientific_execution':False,'complete_cost_ratio':None,'arms':{}}
for arm in ['eager','candidate']:
 rs=[r for c in x['cases'] for r in c[arm]['rows']];status['arms'][arm]={'exact_recoveries':sum(r['signed_ok'] for r in rs),'recovery_modes':dict(collections.Counter(r['recovery_status'] for r in rs)),'successful_repair_slots':sum(z is not None for r in rs for z in r['roots']),'labelled_fallbacks':sum(r['fallback'] for r in rs),'internal_expected_transition_checks':sum(r['internal_transition_checks'] for r in rs),'failures':[{'p':c[arm]['p'],'index':r['index'],'status':r['recovery_status'],'expected':r['expected']} for c in x['cases'] for r in c[arm]['rows'] if not r['pass']]}
status['scope_notes']=['all fixture outcomes retained','baseline helper failure is not asserted to be a full collector solve failure','no successful repair or Delta-zero branch coverage in this original G-step packet','square-root nonresidue search remains work and native counters are not a complete field-operation census','independent review pending'];(D/'status.json').write_text(json.dumps(status,indent=2)+'\n')
fkeys=['m','s','i','field_adds','bit_ops','ec_adds','jac_adds','sqrt_calls'];keys=['arm','p','index','choice','recovery_status']+[phase+'.'+bank+'.'+k for phase in['transition','recovery','verification'] for bank,ks in [('walk',['m','a','i']),('signed',['m','a','i']),('field',fkeys)] for k in ks]
with (D/'phase-counters.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=keys);w.writeheader()
 for c in x['cases']:
  for arm in ['eager','candidate']:
   for r in c[arm]['rows']:
    row={'arm':arm,'p':c[arm]['p'],'index':r['index'],'choice':r['choice'],'recovery_status':r['recovery_status']}
    for k in keys[5:]:phase,bank,key=k.split('.');row[k]=r['counts'][phase][bank][key]
    w.writerow(row)
with(D/'initialization-counters.csv').open('w')as f:
 keys=['arm','p']+[bank+'.'+k for bank,ks in [('walk',['m','a','i']),('signed',['m','a','i']),('field',fkeys)] for k in ks];w=csv.DictWriter(f,fieldnames=keys);w.writeheader()
 for c in x['cases']:
  for arm in ['eager','candidate']:
   row={'arm':arm,'p':c[arm]['p']}
   for k in keys[2:]:bank,key=k.split('.');row[k]=c[arm]['initialization'][bank][key]
   w.writerow(row)
report='''# Original non-torsion G-step: paired software integration

The additive two-call candidate completes63 fixed transitions across21 models with63 exact signed recoveries. Its selected pairs, branch choices and labels match the eager walker on all63 transitions. The eager recovery helper succeeds61 times and returns general_conversion_inapplicable twice; both outcomes are retained. The new degree1 helper recovers those two points. The bounded locked/offline software gate exits0.

## Original transition and direct neighbour
This packet uses the original non-torsion anchor G, not the torsion-grid replacement. State is([R],[R+G]), R=aG+bQ. At each step c=branch([R]); the eager version computes both doubles and mixed addition while the candidate computes the selected double plus mixed addition. Output is([2R+cG],[2R+(c+1)G]); labels are(2a+c,2b) modulo r.
Before checks, verify G,Q valid, rG=rQ=0, G not2-torsion,2<=r<=2^63 and canonical labels. Input fixtures are the pinned first retained target from each21 models; each has exactly3 steps. Initialization constructs Q+G and projects the pair and fixed G/identity. Model setup and subgroup checks are separately outside phase estimates.
The transition uses advance with expected-point diagnostic validation disabled. Recovery immediately consumes the propagated pair and signed G. Only afterward does the harness compute aG+bQ and its neighbour to verify both corners and exact signed result. No expected-neighbour construction feeds the ordinary candidate. The unchanged labelled fallback would reconstruct from public G,Q and labels if needed and be counted; it is not invoked in these63 cases.

## Coverage and failures
All126 candidate quotient corners match the independent signed reference. Candidate recovery modes are61 generic and2 degree1. Baseline failures are recorded as helper-call failures, not asserted to be full collector solve failures: the original collector can skip unsuitable coordinate cases before relation processing, whereas this harness deliberately attempts signed recovery at every selected state.
Neither arm observes a successful geometric repair or labelled fallback. The Delta_zero helper is included but unexercised here. Successful repairs and Delta coverage from the separately scoped torsion-grid fixtures are not counted as original G-step coverage. No163? target count is implied: these are63 bounded transition checks, not independent target solves.

## Accounting and source boundaries
phase-counters.csv contains126 arm/transition rows; initialization-counters.csv contains42 arm/model rows. Native field, walk and signed counters are separate for initialization, transition, recovery and reference verification. Every ordinary candidate transition skips exactly one eager doubling call; no timing or complete cost ratio is inferred. Recovery work, including additional coordinate conversion, square roots, candidate validity and signed-neighbour checks, is retained in its own phase.
The reused square-root implementation searches for a quadratic nonresidue. The at-most-four Mumford candidate bound does not eliminate that work. Counters are implementation observations, not a complete independent census of exponentiation/search costs. Counter banks and verification costs must not be silently combined into a claimed end-to-end price.

## Reproducibility
PLAN.md freezes the original G-step family, assumptions, fixed21x3 checks and120s bound before code. pre-execution-manifest.json pins fixtures/source. receipt.json records exit0. The production candidate is a private additive source snapshot; no original cohort, cloud job, published runtime, or shared repository source was changed. Independent review remains pending before integration or prospective scientific execution.
References: pinned N19 main_labelled.rs and prior two-call/degree1/Delta additive source; Chung-Costello-Smith sections2,3,6.4, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf . The earlier supplied-neighbour and square-root wording corrections remain preserved in their immutable addenda.
'''.replace('No163? target count is implied: these are63 bounded transition checks, not independent target solves.','These are63 bounded transition checks, not independent target solves.')
(D/'REPORT.md').write_text(report)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="850" height="240"><rect width="850" height="240" fill="white"/><g font-family="sans-serif" font-size="17"><text x="20" y="26">Original G-step: 21 fixed models × 3 transitions</text><text x="20" y="75">Eager helper</text><rect x="215" y="52" width="427" height="30" fill="#527b62"/><rect x="642" y="52" width="14" height="30" fill="#a86442"/><text x="675" y="75">61 pass, 2 null</text><text x="20" y="122">Candidate helper</text><rect x="215" y="99" width="427" height="30" fill="#527b62"/><rect x="642" y="99" width="14" height="30" fill="#336ba0"/><text x="675" y="122">61 + 2 pass</text><text x="20" y="171">Recovery consumes propagated [R], [R+G] before reference.</text><text x="20" y="211">0 successful repairs / 0 labelled fallbacks in this packet.</text></g></svg>''';(D/'coverage.svg').write_text(svg);typ='#set page(paper:"a4",margin:17mm)\n#set text(size:10pt)\n#image("coverage.svg",width:100%)\n'
for l in report.splitlines():
 if l.startswith('# '):typ+='\n= '+l[2:]+'\n'
 elif l.startswith('## '):typ+='\n== '+l[3:]+'\n'
 elif l:typ+='\n#text('+json.dumps(l,ensure_ascii=False)+')\n\n'
(D/'report.typ').write_text(typ)
