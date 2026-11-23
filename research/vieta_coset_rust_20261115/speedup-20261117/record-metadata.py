#!/usr/bin/env python3
"""Control-plane registration: hash files and copy already-computed Rust values.
No benchmark, research/statistical calculation, S3 action or protected promotion.
"""
import json,pathlib,subprocess
r=pathlib.Path(__file__).parent
items=[
 ('20261117T0709Z-vieta-speedup-cpu-profile','profile',None,None),
 ('20261117T0757Z-vieta-shared-field-selection','selection','selection-summary.json','newton/fast:cold_ns'),
 ('20261117T0815Z-vieta-transform-selection','transform-selection','transform-selection-summary.json','newton/ntt-half:cold_ns'),
 ('20261117T0840Z-vieta-native-ntt-full-cost-primary','primary','primary-summary.json','newton/fiber-half:cold_ns'),
 ('20261117T0905Z-vieta-word32768-budget8b','primary-budget8b','budget-summary.json','newton/fiber-half:cold_ns')]
for slug,panel,summary,key in items:
 record=pathlib.Path('benchmarks/records')/(slug+'.json');data=json.loads(record.read_text());assert data['status']=='open' and not data['metrics'],'register once only'
 files=[]
 if panel=='profile':files=list((r/'profiles/attribution-20261117T0709Z').glob('*'))+[r/'profiles/isolation.jsonl',r/'protocol-profile.json',r/'export-profile.json',r/'PROFILE_FINDINGS.md']
 else:
  files=list((r/'runs'/panel).glob('**/*'))
  if panel=='selection':files += [r/'protocol-selection.json',r/'export-selection.json']
  elif panel=='transform-selection':files += [r/'protocol-transform-selection.json',r/'export-transform-selection.json']
  elif panel=='primary':files += [r/'protocol-primary.json',r/'protocol-primary-v2.json',r/'export-primary.json',r/'export-primary-corrected.json',r/'evidence/export-outcome-repair-audit.json']
  else:files += [r/'protocol-budget8b.json',r/'export-budget8b.json',r/'evidence/budget-source-equivalence.json']
  files += [r/'evidence'/summary]
  prefix={'primary-budget8b':'budget'}.get(panel,panel)
  files += list((r/'evidence').glob(prefix+'-replay.json'))+list((r/'evidence').glob(prefix+'-*export-check.log'))+list((r/'evidence').glob(prefix+'-supervisor.log'))
 files += [r/'report.pdf',r/'cold-ratios.svg',r/'RESULTS.md']
 paths=sorted({str(p) for p in files if p.is_file()})
 subprocess.run(['python3','tools/benchrec.py','add-raw',str(record),*paths],check=True)
 if summary:
  s=json.loads((r/'evidence'/summary).read_text())
  subprocess.run(['python3','tools/benchrec.py','metric',str(record),'--name','failed_calls','--value',str(s['failed']),'--unit','count','--better','neutral','--note','Stored Rust summary; includes warmups; producer unverified.'],check=True)
  for group in s['groups']:
   value=group['paired_ratios'][key]
   if 'median' not in value:continue # unknown comparator is never a zero/win
   name='cold_ratio_'+group['group'].replace(':','_').replace('-','_')
   subprocess.run(['python3','tools/benchrec.py','metric',str(record),'--name',name,'--value',str(value['median']),'--unit','ratio','--better','higher','--n',str(value['n']),'--spread-low',str(value['bootstrap_median95'][0]),'--spread-high',str(value['bootstrap_median95'][1]),'--note','Copied frozen Rust paired median/conditional bootstrap; belowL2,unverified; full native field setup timed but primitive constructor counts unpriced. No S3 availability assertion.'],check=True)
 print(slug,'registered',len(paths),'local files; no upload')
