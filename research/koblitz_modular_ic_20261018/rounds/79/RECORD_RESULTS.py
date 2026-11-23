"""Verify both R79 attempts and their frozen source/donor bytes."""
from datetime import datetime
from pathlib import Path
import hashlib,json,re,subprocess

h=Path(__file__).resolve().parent;repo=h.parents[3]
specs=[('candidate-u0-79-01','rejected'),('candidate-u0-79-02','accepted')]
assert sorted(p.name for p in (h/'runs').iterdir() if p.is_dir())==[x[0] for x in specs]
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[];pins=[]
for name,status in specs:
 raw=h/'runs'/name;rev=(raw/'source-revision.txt').read_text().strip()
 code=int((raw/'exit.txt').read_text());log=(raw/'native.log').read_text().splitlines()
 errors=[s for s in log if re.match(r'^\s*\?',s)]
 marker='CANDIDATE_U0_79_ALL_CHECKS_PASS'
 if status=='rejected':
  assert code==0 and errors and any('ideal(...) failed' in s for s in errors)
  assert not any(s.startswith('CAND79_RESULT ') for s in log)
 else:
  assert code==0 and not errors and log.count(marker)==1
  assert [s for s in log if s.startswith('CAND79_COMPONENT ')]==[
   'CAND79_COMPONENT y=0 dimension=16 source_factor=1',
   'CAND79_COMPONENT y=1 dimension=16 source_factor=1']
  assert [s for s in log if s.startswith('CAND79_RESULT ')]==[
   'CAND79_RESULT components=2 total_dimension=32 candidate_only=1']
 for local,source in [('input.sing','CANDIDATE_U0_79.sing'),('driver.sh','NATIVE_DRIVER.sh'),
                      ('launcher.sh','REPLAY_CAS.sh'),('PROTOCOL.md','PROTOCOL.md'),
                      ('protocol.json','protocol.json'),('donors.json','donors.json')]:
  path=str((h/source).relative_to(repo))
  assert subprocess.check_output(['git','show',rev+':'+path],cwd=repo)==(raw/local).read_bytes()
  pins.append({'attempt':name,'artifact':local,'git_source':path,'revision':rev,'sha256':digest(raw/local)})
 for donor in json.loads((raw/'donors.json').read_text()):
  p=h/donor['path'];assert p.stat().st_size==donor['bytes'] and digest(p)==donor['sha256']
 start=(raw/'started-utc.txt').read_text().strip();finish=(raw/'finished-utc.txt').read_text().strip()
 rows.append({'attempt':name,'kind':'Singular','status':status,'source_revision':rev,'native_exit':code,
              'marker_present':marker in log,'error_count':len(errors),'errors':errors if status=='rejected' else [],
              'started_utc':start,'finished_utc':finish,
              'native_wall_seconds':(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds(),
              'artifacts':[{'path':str(p.relative_to(h)),'sha256':digest(p),'bytes':p.stat().st_size} for p in sorted(raw.iterdir()) if p.is_file()]})
assert len(pins)==12
results={'schema':'koblitz.results/v1','round':79,'runs':rows,'input_pins':pins,'donor_hashes':6,
         'candidate_component_dimensions':[16,16],'normality_certified':False}
(h/'results.json').write_text(json.dumps(results,indent=2)+'\n')
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R79-CANDIDATE-U0-FIBERS','claim_class':'exact_computation',
        'protocols':[{'attempt':x['attempt'],'path':'runs/'+x['attempt']+'/protocol.json','sha256':digest(h/'runs'/x['attempt']/'protocol.json'),'frozen_revision':x['source_revision']} for x in rows],
        'raw_archive':'runs.tar.xz; content address in .artifacts.json after custody publication',
        'results':{'path':'results.json','sha256':digest(h/'results.json')},
        'flow':{'accepted':1,'rejected':1,'interrupted':0},
        'canonical_schema_validation':'unavailable in selected Git index and disk; manual contract/source-byte checks only',
        'notes':'Rejected R50 normalization candidate has finite 16+16 specialized quotient dimensions and source-factor containment; no normality claim.'}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n')
print(json.dumps({'accepted':1,'rejected':1,'source_byte_pins':12,'candidate_dimensions':[16,16]}))
