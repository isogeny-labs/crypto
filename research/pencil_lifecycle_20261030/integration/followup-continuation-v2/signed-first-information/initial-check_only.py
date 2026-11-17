"""Fresh direct-sum verification; imports no Gaussian producer."""
from common import *
import copy

def check_packet(members,archive,inputs):
 expected={name.replace('.jsonl','.boundary.json') for name in inputs}
 assert len(members)==len(expected) and {m['file'] for m in members}==expected
 with tarfile.open(archive) as t:
  assert len(t.getmembers())==len(expected) and set(t.getnames())==expected
  records=[]
  for m in members:
   raw=t.extractfile(m['file']).read();assert len(raw)==m['bytes'] and h(raw)==m['sha256'];a=json.loads(raw);name=m['file'].replace('.boundary.json','.jsonl')
   assert a['source_stream']==name;item=inputs[name];d=item['system'];p=d['modulus'];n=d['nuisance_columns'];rows=d['rows'];first=a['first_row_count'];cert=a['certificate'];prev=a['predecessor']
   assert a['witness_sha256']==item['witness_sha256'] and check_boundary(p,n,rows,first,cert,prev)
   terminal=item['terminal'];origin=d['row_origins'][first-1] if first is not None else None;attempt=origin['attempt'] if origin else None
   if first is None:assert terminal['recovered'] is None
   else:assert cert['scalar']==terminal['recovered']
   expected_record={'stream':name,'kind':cert['kind'],'total_rows':len(rows),'first_row_count':first,'first_sequence':origin['sequence'] if origin else None,'first_attempt':attempt,'native_terminal_attempt':terminal['attempts'],'attempts_after_first':terminal['attempts']-attempt if attempt is not None else None,'rows_after_first':len(rows)-first if first else None,'scalar':cert.get('scalar'),'native_scalar':terminal['recovered']}
   assert a['record']==expected_record;records.append(expected_record)
 return sorted(records,key=lambda r:r['stream'])
inputs,bindings=load_inputs();members=json.loads((P/'boundary-manifest.json').read_text())['members'];archive=P/'boundaries.tar.gz'
records=check_packet(members,archive,inputs);summary=json.loads((P/'summary.json').read_text());assert records==summary['records'] and bindings==summary['input_bindings']
bad=copy.deepcopy(members);bad[1]=bad[0]
try:check_packet(bad,archive,inputs)
except AssertionError:duplicate_rejected=True
else:raise AssertionError('duplicate accepted')
result={'schema':'signed.first-information.readback/v1','checked_systems':len(records),'determined_boundaries':sum(r['kind']=='determined' for r in records),'terminal_undetermined':sum(r['kind']=='undetermined' for r in records),'source_rows':sum(r['total_rows'] for r in records),'duplicate_manifest_rejected':duplicate_rejected,'all_records_recomputed':True,'collector_invocations':0,'elimination_invocations':0,'independent_non_author_review':False,'source_hashes':{str(f.relative_to(P.parent)):h(f.read_bytes()) for f in [P/'check_only.py',P/'common.py',W/'check_certificate.py']}}
print(json.dumps(result,indent=2))
