"""CI API/log metadata only; preserve complete provided decoded streams."""
from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';raw=Path('/dev/net/chained-transfers-owned-storage-20261117/parent4514-first-CI');o=s/'parent-PR4514-final-CI-12';o.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def pages(p):
 text=p.read_text();decoder=json.JSONDecoder();out=[]
 while text.strip():obj,n=decoder.raw_decode(text.lstrip());out.append(obj);text=text.lstrip()[n:]
 return out
head_data={}
for tag,head in [('submitted233','233c0cab0e2facffa22769d063450e1bccba00d8'),('finalbc','bc6931688fab092518ca66c231b0949f2356b163')]:
 runs=[x for p in pages(raw/tag/'all-event-run-pages.json-stream') for x in p['workflow_runs']]
 checks=[x for p in pages(raw/tag/'all-check-pages.json-stream') for x in p['check_runs']]
 assert len(runs)==19 and len(checks)==30 and all(x['head_sha']==head for x in runs+checks)
 assert all(x['status']=='completed' for x in runs+checks)
 head_data[tag]={'head':head,'workflow_runs':19,'events':dict(collections.Counter(x['event'] for x in runs)),'check_runs':30,'conclusions':dict(collections.Counter(x['conclusion'] for x in checks)),'complete_workflow_and_check_payloads_in_archive':True}
logs=[]
for id in [133853027136,133853026594,133853027438,133853027273]:
 text=json.loads((raw/f'whole-log-{id}.json').read_text())['content'];path=raw/f'whole-decoded-job-{id}.txt';path.write_bytes(text.encode())
 assert path.read_bytes().decode()==text
 logs.append({'job_id':id,'full_decoded_log_bytes':path.stat().st_size,'sha256':sha(path),'logs_have_not_been_tail_or_excerpt_replaced':True})
validation=json.loads((raw/'whole-log-133853027136.json').read_text())['content'];artifact=json.loads((raw/'whole-log-133853026594.json').read_text())['content']
assert 'Ran 86 tests' in validation and 'FAILED (failures=1)' in validation and 'InvalidAccessKeyId' in validation
assert '9 data file(s) belong in the store' in artifact
pr=json.loads((raw/'closed-PR4514.json').read_text());assert pr['merged'] and pr['merge_commit_sha']=='017ad45fbc6dc3475e00d63d8e098bb72cfba912'
(raw/'CLI_FLAG_FAILURE_BEFORE_CORRECTED_CUSTODY.json').write_text(json.dumps({'argv_initial_flag':'gh api --paginate --slurp','exit':1,'complete_error_summary':'unknown flag: --slurp; installed gh supports --paginate, not --slurp','initial_empty_redirect_files_retained':True,'corrected_native_API_custody':'--paginate raw concatenated JSON preserved without jq filtering','no_CI_rerun_or_failed_remote_request_in_initial_flag_attempt':True},indent=2)+'\n')
files={p.relative_to(raw).as_posix():p for p in sorted(raw.rglob('*')) if p.is_file()}
archive=o/'whole-first-PR4514-all-APIs-and-four-full-decoded-logs.tar.xz'
with tarfile.open(archive,'w:xz',preset=9) as t:
 for n,p in files.items():t.add(p,arcname=n,recursive=False)
inventory=[]
with tarfile.open(archive,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for n,p in files.items():
  m=t.getmember(n);b=t.extractfile(m).read();assert b==p.read_bytes() and m.mode==p.stat().st_mode&0o7777;inventory.append({'member':n,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
data={'time':datetime.now(timezone.utc).isoformat(),'PR':4514,'merged_at':pr['merged_at'],'merge_commit':pr['merge_commit_sha'],'actual_final_head':pr['head']['sha'],'base':pr['base']['sha'],'first_elapsed_PR_only_metadata':'approximately07:40Z; connector16 PR-only workflows per head preserved','complementary_full_API_custody':'single first observation completion;19 all-event workflows/30 checks each; no CI rerun or busy poll','heads':head_data,'validation':'86 tests/1 failure; committed-tree diagnostic abbreviated by unittest; InvalidAccessKeyId on one S3 GetObject line; full provided decoded stream retained','artifact_gate':'9 files belong in store; exact listed paths in full log','learning_and_large_file_gates':'success','full_selected_decoded_logs':logs,'whole_archive':{'path':str(archive.relative_to(r)),'bytes':archive.stat().st_size,'sha256':sha(archive),'members':len(files),'all_full_member_bytes_and_modes_compared':True},'no_green_CI_inference_from_merge':True,'revert_check_due':'2026-12-01T06:00:38Z','next_result_PR':'fresh canonical main intake; no closed branch pushes; avoid new inline binary data under current artifact policy, retain immutable full checkpoints with exact materialization references'}
(o/'ACTUAL_PR4514_FIRST_FULL_CI.json').write_text(json.dumps(data,indent=2)+'\n');(o/'WHOLE_ARCHIVE_MEMBERS.json').write_text(json.dumps(inventory,indent=2)+'\n')
print(json.dumps({'archive_bytes':archive.stat().st_size,'archive_sha256':sha(archive),'members':len(files),'heads':head_data},indent=2))
