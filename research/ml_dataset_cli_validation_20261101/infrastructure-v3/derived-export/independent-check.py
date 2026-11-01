from pathlib import Path
import json,hashlib,importlib.util,collections,datetime,secrets
r=Path(__file__).resolve().parent;f=Path('/var/tmp/ml-dataset-cli46-032469-v3-20261101')
load=lambda p:json.loads(Path(p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
audit=load(r/'audit.json');before={name:sha(f/name)for name in audit['original_inputs_sha256']};assert before==audit['original_inputs_sha256']
raw=load(f/'experiment.export.json');derived=load(r/'experiment.export.derived.json');protocol=load(f/'protocol.json');state=load(f/'prepared.json')
assert len(raw['runs'])==len(derived['runs'])==92
for a,b in zip(raw['runs'],derived['runs']):assert {k:v for k,v in a.items()if k!='artifacts'}=={k:v for k,v in b.items()if k!='artifacts'}
assert len({(x['arm'],x['instance_id'])for x in derived['runs']})==92
assert collections.Counter(x['arm']for x in derived['runs'])=={'producer-source':46,'verifier-replay':46}
for key in ['source','producer','host','flow','backend','created','export_id']:assert raw[key]==derived[key],key
assert sha(r/'experiment.export.derived.json')=='4aee068ce1aa909ef96ca851cf2d8ba7e7ff90bd5a2c1115cd10c5e3d70d2822'
# All attached evidence references independently hash and size verified.
refs=[]
def walk(v):
 if isinstance(v,dict):
  if {'path','sha256','bytes'}<=set(v):refs.append(v)
  for x in v.values():walk(x)
 elif isinstance(v,list):
  for x in v:walk(x)
walk(derived)
for ref in refs:
 p=Path(ref['path']);assert p.is_absolute();assert p.stat().st_size==ref['bytes'];assert sha(p)==ref['sha256'],str(p)
armmap={a['id']:a for a in protocol['arms']}
for arm in derived['arms']:
 assert arm['commit']==armmap[arm['id']]['instrument']['commit'];assert arm['binary_sha256']==armmap[arm['id']]['instrument']['binary_sha256']
assert derived['analysis']['script']['sha256']==state['driver_sha256']==sha(f/'retained-analysis.py')
commands=[load(p/'argv.json')for p in (f/'commands').iterdir()];dataset_commands=sum(len(a)>1 and a[1]=='dataset'for a in commands);assert dataset_commands==125
assert load(f/'failure-export-status.json')['no_dataset_commands'] is True
assert load(f/'execution-failed.json')['no_retry'] is True
# Independent case tally from each producer receipt, not producer audit counts.
tally=collections.Counter();cases={}
for row in derived['runs']:
 if row['arm']!='producer-source':continue
 assert len(row['artifacts'])==1
 manifest=load(row['artifacts'][0]['path']);o=manifest['outcome'];panel=row['instance_id'].rsplit('-',1)[0]
 tally[(panel,o.get('status'),str(o.get('solver_completed')))]+=1;cases[row['instance_id']]=sha(row['artifacts'][0]['path'])
assert cases=={x['id']:x['manifest_sha256']for x in audit['case_details']}
assert tally=={(x['panel'],x['status'],x['solver_completed']):x['count']for x in audit['outcomes']}
source=load(Path(state['checker']).parent.parent/'SOURCE.json');assert source['commit']=='37e193bf8b3e673e0942709794f44d28e6ab66fb'
for rel,h in source['sha256'].items():assert sha(Path(state['checker']).parent.parent/rel)==h
spec=importlib.util.spec_from_file_location('independent_official_checker',state['checker']);checker=importlib.util.module_from_spec(spec);spec.loader.exec_module(checker)
findings=checker.schema_errors(derived,'experiment.export/v1')+checker.schema_errors(protocol,'experiment.protocol/v1')+checker.lint_export(derived,protocol,r,True)
assert not findings,findings
assert before=={name:sha(f/name)for name in before}
summary={'derived_sha256':sha(r/'experiment.export.derived.json'),'original_export_sha256':sha(f/'experiment.export.json'),'originals_before_after_equal':True,'observations_preserved':92,'cases':46,'artifact_references_verified':len(refs),'dataset_commands':dataset_commands,'official_findings':findings,'checker_sha256':sha(state['checker']),'outcomes':[{'panel':k[0],'status':k[1],'solver_completed':k[2],'count':v}for k,v in sorted(tally.items())],'data_reruns':0,'scope':'provenance-only reconciliation; original terminal failure preserved'}
(r/'independent-check.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
