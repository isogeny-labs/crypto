"""Hashes, admission and official-schema export only. No research arithmetic.
Native C++ and SQLite perform every group, polynomial and counting computation.
"""
from pathlib import Path
import hashlib,importlib.util,json,subprocess,sys

ROOT=Path('/workspace/crypto-audit')
STUDY=ROOT/'research/bk_basefield_census_20261112'
TOOL=Path('/workspace/crypto-autoresearcher-current/tools/experiment_protocol.py')
sp=importlib.util.spec_from_file_location('audit_experiment_protocol',TOOL)
ep=importlib.util.module_from_spec(sp);sys.modules[sp.name]=ep;sp.loader.exec_module(ep)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def protocol(mode):
 assert mode in ['preflight','main']
 name='protocol-preflight.json' if mode=='preflight' else 'protocol.json'
 return name,json.loads((STUDY/name).read_text())
def admit(mode):
 name,p=protocol(mode)
 errors=ep.schema_errors(p,'experiment.protocol/v1')+ep.lint_protocol(p,STUDY)
 assert not errors,errors
 assert p['freeze']['status']=='frozen' and p['freeze']['filed_before_affected_data']
 assert ep.protocol_hash(p)==p['freeze']['protocol_sha256']
 assert subprocess.check_output(['git','show','HEAD:'+str((STUDY/name).relative_to(ROOT))],cwd=ROOT)==(STUDY/name).read_bytes()
 manifest=STUDY/p['extensions']['input_manifest']['path']
 assert sha(manifest)==p['extensions']['input_manifest']['sha256']
 for line in manifest.read_text().splitlines():
  digest,path=line.split('  ',1);target=Path(path) if Path(path).is_absolute() else STUDY/path
  assert sha(target)==digest,path
 assert not (STUDY/'runs'/mode).exists()
 if mode=='main':
  pilot=STUDY/'export-preflight.json';x=p['extensions']['preflight']
  assert sha(pilot)==x['export_sha256']
  assert sha(STUDY/'analysis/preflight/analysis.jsonl')==x['analysis_sha256']
  assert (STUDY/'evidence/preflight-export-check.exit').read_text().strip()=='0'
  assert (STUDY/'runs/preflight/controls.exit').read_text().strip()=='0'
  assert (STUDY/'runs/preflight/negative.exit').read_text().strip()=='0'
  assert (STUDY/'runs/preflight/analysis.exit').read_text().strip()=='0'
  _,pp=protocol('preflight');pe=json.loads(pilot.read_text())
  errors=ep.schema_errors(pe,'experiment.export/v1')+ep.lint_export(pe,pp,STUDY,True)
  assert not errors,errors
 print(json.dumps({'admitted':mode,'protocol_sha256':p['freeze']['protocol_sha256']}))
def export(mode):
 name,p=protocol(mode);out=STUDY/('export-preflight.json' if mode=='preflight' else 'export.json')
 assert not out.exists(),'exports are write-once'
 e=ep.export_skeleton(p,name,'local',p['protocol_id']+'-export')
 e['producer'].update(runtime='codex',model='Codex',session='fourteen-target-20261113')
 e['source'].update(repo='isogeny-labs/crypto',commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),dirty=True)
 e['host']={'path':'evidence/build-provenance.json','sha256':sha(STUDY/'evidence/build-provenance.json'),
  'class_id':'local-shared-bk-boundary','summary':{'cpu_model':None,'logical_cpus':None,'memory_bytes':None,'kernel':None,'hypervisor':None,'isolation_tier_max':'none'}}
 e['arms']=[{'id':a['id'],'role':a['role'],'binary_sha256':a['instrument']['binary_sha256'],'commit':a['instrument']['commit']} for a in p['arms']]
 analysis_ok=(STUDY/f'runs/{mode}/analysis.exit').read_text().strip()=='0'
 for a in p['arms']:
  arm=a['id'];prefix=STUDY/f'runs/{mode}/{arm}';rc=int(prefix.with_suffix('.exit').read_text())
  status='succeeded' if rc==0 else ('timeout' if rc in [124,137] else 'failed')
  counted={'population_complete':int(rc==0)}
  if rc==0:
   with prefix.with_suffix('.jsonl').open() as f:
    last=None
    for line in f:last=line
   record=json.loads(last)
   counted.update({k:v for k,v in record.items() if isinstance(v,(int,float))})
  artifacts=[ep.file_entry(STUDY,str(x.relative_to(STUDY))) for x in sorted(prefix.parent.glob(arm+'.*')) if x.is_file()]
  e['runs'].append({'run_id':mode+'-'+arm,'arm':arm,'instance_id':p['extensions']['population_id'],
    'seed':'No randomness; full lexicographic population','replicate':0,'warmup':False,'status':status,
    'outcome_class':'completed' if status in ['succeeded','failed'] else 'not_completed','attempts':1,'resumed':0,'checkpoint':None,
    'measurements':{'counted':counted,'wall_seconds':None,'wall_valid':False,'cpu_user_seconds':None,'cpu_sys_seconds':None,'max_rss_bytes':None,'counters':{}},
    'isolation':{'tier':'none','contended':None,'blockers':['Shared local host; exact counts only.']},
    'verification':{'status':'verified' if rc==0 and analysis_ok else 'error','verifier':'Frozen native C++ group arithmetic, polynomial oracles and independent native SQLite analysis; unsigned producer evidence.'},
    'exclusion':{'excluded':False,'rule':None},'artifacts':artifacts,
    'started_at':prefix.with_suffix('.start').read_text().strip(),'finished_at':prefix.with_suffix('.end').read_text().strip(),
    'worker':'same local shared host','backend_ref':f'runs/{mode}/{arm}'})
 e['flow']=ep.compute_flow(e['runs'])
 e['files']=[ep.file_entry(STUDY,str(q.relative_to(STUDY))) for d in [STUDY/'runs'/mode,STUDY/'analysis'/mode] for q in sorted(d.iterdir()) if q.is_file()]
 e['files']+=[ep.file_entry(STUDY,n) for n in ['inputs.sha256','evidence/build-provenance.json','native/census.cpp','native/analyze.cpp','native/queries.sql','run.sh','analyze.sh','metadata.py']]
 e['analysis']={'script':ep.file_entry(STUDY,'analyze.sh','analysis'),'blinded':False,'blinding_key_sha256':None,
    'outputs':[ep.file_entry(STUDY,str(q.relative_to(STUDY)),'summary') for q in sorted((STUDY/'analysis'/mode).iterdir()) if q.is_file()],'unmasked_at':None}
 e['notes']=[{'kind':'observation','by':'Codex','text':'Exact basefield k1 boundary census only. All scientific clocks invalid. No extension-field allk or independent-host/compiled-refinement assertion.'}]
 errors=ep.schema_errors(e,'experiment.export/v1')+ep.lint_export(e,p,STUDY,True)
 assert not errors,errors
 ep.write_once(out,e)
 print(json.dumps({'export':str(out),'flow':e['flow']}))
if __name__=='__main__':
 if len(sys.argv)!=3 or sys.argv[1] not in ['admit','export']:raise SystemExit('metadata.py admit|export preflight|main')
 {'admit':admit,'export':export}[sys.argv[1]](sys.argv[2])
