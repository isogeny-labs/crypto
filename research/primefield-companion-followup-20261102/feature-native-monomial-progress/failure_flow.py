"""Exact observed-prefix failure custody; never invent unstarted executions."""
from pathlib import Path
import argparse,json,hashlib
D=Path(__file__).resolve().parent;R=D.parent
ARMS=('baseline','candidate','aa')
def labels():return [int(l.split('\t')[0])for l in (R/'feature-native-monomial-recovery/recovery.tsv').read_text().splitlines()[1:]]
def schedule(targets):
 out=[]
 for arm in ARMS:
  out.append(dict(event='arm_started',arm=arm,label=-1))
  for k in targets:out.extend([dict(event='query_started',arm=arm,label=k),dict(event='query_verified',arm=arm,label=k)])
  out.append(dict(event='arm_finished',arm=arm,label=-1))
 out.append(dict(event='panel_finished',arm='all',label=-1));return out
def unique(pairs):
 d={}
 for k,v in pairs:
  if k in d:raise ValueError('duplicate marker key')
  d[k]=v
 return d
def parse(stderr,targets=None):
 targets=labels()if targets is None else targets;expected=schedule(targets);events=[];partial=False
 for line in stderr.splitlines(keepends=True):
  if not line.startswith(b'progress\t'):continue
  if not line.endswith(b'\n'):partial=True;break
  e=json.loads(line[len(b'progress\t'):],object_pairs_hook=unique)
  if type(e)is not dict or type(e.get('label'))is not int:raise ValueError('marker types')
  if len(events)>=len(expected)or e!=expected[len(events)]:raise ValueError('non-prefix progress')
  events.append(e)
 started=[(e['arm'],e['label'])for e in events if e['event']=='query_started'];verified=[(e['arm'],e['label'])for e in events if e['event']=='query_verified'];planned=[(a,k)for a in ARMS for k in targets]
 return {'planned_queries':len(planned),'started_queries':started,'verified_returns':verified,'active_or_incomplete':[x for x in started if x not in verified],'not_started':[x for x in planned if x not in started],'arms_with_finished_teardown':[e['arm']for e in events if e['event']=='arm_finished'],'panel_output_finished':bool(events and events[-1]['event']=='panel_finished'),'complete_markers':len(events),'partial_marker_tail':partial,'complete_measurement_claim':False}
def record(stderr,receipt,raw,expected_binary,expected_protocol):
 if type(receipt.get('timed_out'))is not bool:raise ValueError('timeout type')
 if receipt.get('exit_code')==0 and not receipt['timed_out']:raise ValueError('use strict successful-panel exporter')
 if receipt.get('exit_code')is not None and type(receipt['exit_code'])is not int:raise ValueError('exit type')
 for key,value in [('stderr_sha256',hashlib.sha256(stderr).hexdigest()),('raw_sha256',hashlib.sha256(raw).hexdigest()),('binary_sha256',expected_binary),('protocol_sha256',expected_protocol)]:
  if receipt.get(key)!=value:raise ValueError('receipt binding: '+key)
 return {'schema':'monomial.failure-flow/v1','process_exit_code':receipt['exit_code'],'timed_out':receipt['timed_out'],'binary_sha256':expected_binary,'protocol_sha256':expected_protocol,'stderr_sha256':receipt['stderr_sha256'],'raw_sha256':receipt['raw_sha256'],'observed_flow':parse(stderr),'disposition':'invalid/incomplete measurement; preserve this attempt, no replacement or success inference'}
def main():
 p=argparse.ArgumentParser();p.add_argument('attempt',type=Path);p.add_argument('--binary-sha256',required=True);p.add_argument('--protocol-sha256',required=True);a=p.parse_args()
 if a.binary_sha256!=json.loads((D/'release-receipt.json').read_text())['binary_sha256']:raise ValueError('not the pinned progress-enabled binary')
 out=record((a.attempt/'stderr.log').read_bytes(),json.loads((a.attempt/'producer-receipt.json').read_text()),(a.attempt/'rows.jsonl').read_bytes(),a.binary_sha256,a.protocol_sha256)
 with(a.attempt/'failure-flow.json').open('x')as f:json.dump(out,f,indent=2);f.write('\n')
if __name__=='__main__':main()
