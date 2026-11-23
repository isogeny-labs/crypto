import sys,tempfile,json,hashlib
from pathlib import Path
P=Path(__file__).parent
sys.path.insert(0,str(P.parent/'owned-search-report-join-20261107'))
from test_report_join import report_for
from test_capture import make,B,S,js,digest
from test_replay_capture import state
from report_join import join_report
with tempfile.TemporaryDirectory() as t:
 r=Path(t);make(r,lambda x:x.update(state()));report=report_for(r)
 head=json.loads((r/'committed.json').read_bytes());event={'schema':'crypto.ml-dataset.live-event/v2','binding_sha256':B,'sequence':head['event_count']+1,'previous_event_sha256':head['last_event_sha256'],'event':{'kind':'review-trailing-committed-event'}};line=js(event)
 with (r/'events.jsonl').open('ab') as f:f.write(line)
 head['event_count']+=1;head['byte_end']+=len(line);head['last_event_sha256']=digest(line);(r/'committed.json').write_bytes(js(head))
 result=join_report(r,B,S,report,0)
 (P/'TRAILING_HEAD_PROBE.json').write_text(json.dumps({'status':result['status'],'report_capture_join_verified':result['report_capture_join_verified'],'joined_snapshot_head_count':result['capture_ack']['event_count'],'actual_committed_head_count':result['capture']['event_count'],'probe_scope':'self-authored adversarial capture, caller source hashes, not real producer'},indent=2)+'\n')
 print(result['status'],result['capture_ack']['event_count'],result['capture']['event_count'])
