"""Retain full API and decoded CI log custody; source/metadata checks only."""
from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';raw=Path('/dev/net/chained-transfers-owned-storage-20261117/PR4544-first-CI');o=s/'PR4544-first-full-CI-custody-03';o.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def pages(p):
 text=p.read_text();d=json.JSONDecoder();out=[]
 while text.strip():j,n=d.raw_decode(text.lstrip());out.append(j);text=text.lstrip()[n:]
 return out
heads={}
for tag,head,run_count,check_count in [('tracking6f','6f1815b9f90b71f407f68e77901d3cc6f12cb8f9',18,28),('result2160','2160eb2a523fb0e60709aabaf6dd53c7f7fd32a3',34,56),('latestDA','da590ab79c5d46248644c734bac15f264d750e71',19,30)]:
 runs=[x for p in pages(raw/(tag+'-all-event-run-pages.json-stream')) for x in p['workflow_runs']];checks=[x for p in pages(raw/(tag+'-all-check-pages.json-stream')) for x in p['check_runs']]
 assert len(runs)==run_count and len(checks)==check_count and all(x['head_sha']==head for x in runs+checks)
 heads[tag]={'head':head,'workflow_runs':len(runs),'events':dict(collections.Counter(x['event'] for x in runs)),'check_runs':len(checks),'conclusions':dict(collections.Counter(x['conclusion'] for x in checks)),'all_workflow_ids':[x['id'] for x in runs],'all_checks':[{'id':x['id'],'name':x['name'],'status':x['status'],'conclusion':x['conclusion']} for x in checks]}
log_rows=[]
for id in [133950798477,133950768052,133950800970,133950799205,133950880488,133950801952,133950768037,133950767668,133950767156]:
 full=json.loads((raw/f'job-{id}-full-decoded-tool-response.json').read_text());assert not full.get('isError');text=full['structuredContent']['content'];p=raw/f'complete-decoded-job-{id}.txt';p.write_bytes(text.encode());assert p.read_bytes().decode()==text
 log_rows.append({'job_id':id,'full_decoded_bytes':p.stat().st_size,'sha256':sha(p),'no_excerpt_or_tail_replacement':True})
files={p.relative_to(raw).as_posix():p for p in sorted(raw.rglob('*')) if p.is_file()}
a=Path('/dev/net/chained-transfers-owned-storage-20261117/native15-complete-packets/entire-first-PR4544-three-heads-all-APIs-nine-full-logs.tar.xz');assert not a.exists()
with tarfile.open(a,'w:xz',preset=6) as t:
 for n,p in files.items():t.add(p,arcname=n,recursive=False)
inventory=[]
with tarfile.open(a,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for n,p in files.items():
  member=t.getmember(n);b=t.extractfile(member).read();assert b==p.read_bytes() and member.mode==p.stat().st_mode&0o7777
  inventory.append({'member':n,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':member.mode})
data={'recorded':datetime.now(timezone.utc).isoformat(),'PR':4544,'cadence':'first all-event/check metadata read once after50min; logs completed from this same observation; no workflow retry or busy poll','heads':heads,'all_seven_current_failed_job_logs_retained':True,'full_success_logs_retained':['learning','large-files'],'failures':{'artifacts':'added/modified data policy check passes; workflow store pull wiring reports35 existing workflows stale or missing pulls','gate':'86 tests/1 committed-tree failure;90 additional diagnostics abbreviated by unittest, with missing raw_data and curve registry mismatch','records':'58+8 tests pass; AWS credential configuration rejects invalid security token','corpus':'no Git-history object matches store manifest SHA256 for claims.tsv; no S3 contacted','names':'no Git-history object matches manifest for quadratic-twist-native-20261021.json; no S3 contacted','recompute':'12 tamper cases and missing-row control caught; committed currency file is not current','parity':'25 skill pairs pass; shared-code lock for tools/artifacts/artifacts.py mismatches'},'unchanged_base_input_subtrees':json.loads((raw/'UNCHANGED_BASE_INPUT_SUBTREES.json').read_text()),'historical_result2160_check_budget_violation_preserved':56,'current_DA_check_budget':30,'current_head_has_not_passed_all_gates':True,'CI_failures_not_waived_or_hidden':True,'complete_logs':log_rows,'whole_packet':{'path':str(a),'bytes':a.stat().st_size,'sha256':sha(a),'members':len(files),'all_full_bytes_and_modes_compared':True},'publication_status':'full raw archive retained privately; unchanged store policy, no new inline compressed dataset; full immutable CI job/run references and custody metadata published','next_unchanged_head_CI_read_no_earlier_than':'2026-11-17T14:40:07Z','new_substantive_head_requires50min_before_first_CI_read':True}
(o/'ACTUAL_FIRST_CURRENT4544_CI.json').write_text(json.dumps(data,indent=2)+'\n');(o/'WHOLE_PACKET_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n');shutil.copy2(Path(__file__),o/'seal-complete-CI-metadata-only.py');print(json.dumps({'archive':data['whole_packet'],'counts':{k:{x:v[x] for x in ['workflow_runs','check_runs','conclusions']} for k,v in heads.items()}},indent=2))
