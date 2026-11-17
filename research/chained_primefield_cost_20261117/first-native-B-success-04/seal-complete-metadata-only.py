"""Whole recorded packet and derived metadata custody only; no research sums."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_primefield_cost_20261117';h=Path('/dev/shm/ct812-first-cost16B');o=s/'first-native-B-success-04';o.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
j=json.loads((h/'all-phases.json').read_text());assert j['source_commit']=='e7efdbf14d97ef061a5e8ba707bb8cc803313d17' and j['first_failure'] is None and len(j['phases'])==15 and all(x['exit']==x['expected_exit'] for x in j['phases'])
raw=h/'first-native-unit.stdout';text=raw.read_text();error=(h/'first-native-unit.stderr').read_text();assert '1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out' in text and error.count('zero in compositum inverse')==2
accepted=[x for x in text.splitlines() if x.startswith('CT16|accepted|')];assert len(accepted)==1 and 'snapshot_rows=106' in accepted[0]
summaries=[x for x in text.splitlines() if x.startswith('CT16|totals|') or (x.startswith('CT16|budget|') and '|key=TOTAL|' in x)]
assert len(summaries)==15
assert sha(h/'compiler-output')==sha(h/'immutable-qualifier') and (h/'immutable-qualifier').stat().st_mode&0o7777==0o555
files={('custody/'+p.relative_to(h).as_posix()):p for p in sorted(h.rglob('*')) if p.is_file()}
for ext in ['stdout','stderr']:
 p=Path('/workspace/scratch/ct812-first-cost16B-caller.'+ext);files['outer-caller/'+p.name]=p;shutil.copy2(p,o/('outer-caller.'+ext))
(o/'outer-caller.exit').write_text('0\n');files['outer-caller/caller.exit']=o/'outer-caller.exit'
a=Path('/dev/net/chained-transfers-owned-storage-20261117/native15-complete-packets/entire-first16B-source-three-rlibs-both-ELFs-and-all-raw-streams.tar.xz');assert not a.exists()
with tarfile.open(a,'w:xz',preset=6) as t:
 for n,p in files.items():t.add(p,arcname=n,recursive=False)
inventory=[]
with tarfile.open(a,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for n,p in files.items():
  m=t.getmember(n);b=t.extractfile(m).read();assert b==p.read_bytes() and m.mode==p.stat().st_mode&0o7777;inventory.append({'member':n,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
for p in sorted(h.iterdir()):
 if p.is_file() and p.suffix in ['.stdout','.stderr','.json'] and p!=raw:
  assert p.stat().st_size<=65536 or p.suffix=='.json';shutil.copy2(p,o/p.name)
data={'recorded':datetime.now(timezone.utc).isoformat(),'successful_source_B':j['source_commit'],'prospective_protocol':'dd6223faae3ff638eb2582e8f40ceca41c5a142c','all15_phase_expectations_match':True,'new_native_test':'1 passed,15 inherited compiled/filtered','complete_native_accepted_line':accepted[0],'original_fixed_criteria_changed':False,'complete_stdout':{'private_original_path':str(raw),'bytes':raw.stat().st_size,'sha256':sha(raw),'not_inlined_or_renamed_for_artifact_policy':True},'complete_expected_panic_stderr':{'bytes':(h/'first-native-unit.stderr').stat().st_size,'sha256':sha(h/'first-native-unit.stderr')},'pre_call_qualifier':json.loads((h/'PRE_CALL_MANIFEST16.json').read_text()),'all_post_call_guards':json.loads((h/'POST_CALL_MANIFEST16.json').read_text()),'Rust_emitted_aggregate_budget_and_comparison_lines_only':summaries,'derived_summary_scope':'only15 Rust-emitted aggregate lines,not the full106 per-pair costs/snapshots; no Python arithmetic or result aggregation','whole_packet':{'path':str(a),'bytes':a.stat().st_size,'sha256':sha(a),'members':len(files),'all_full_bytes_and_modes_compared':True},'raw_publication_status':'whole source/dependencies/bothELFs/full161784B stdout and every recorded stream retained privately; S3 migration unresolved; no new inline compressed data or raw suffix/size evasion','scope':'fixed same-engine component counts with atomic modpow,input histograms and two scopes; no empirical runtime/GAE pricing or complete whole-map solver cost','first_A_negative_and_original_Git_stderr_gap_remain':True,'signed_independent_verdict':False}
(o/'ACTUAL_FIRST16B_SUCCESS_AND_NATIVE_AGGREGATES.json').write_text(json.dumps(data,indent=2)+'\n');(o/'WHOLE_PACKET_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n');shutil.copy2(Path(__file__),o/'seal-complete-metadata-only.py')
print(json.dumps({'source':j['source_commit'],'native_accepted':accepted[0],'whole_packet':data['whole_packet'],'stdout':data['complete_stdout'],'totals_lines':[x for x in summaries if x.startswith('CT16|totals|')]},indent=2))
