"""Synthetic failure-boundary tests. No native comparison panel."""
from pathlib import Path
import copy,json,sys,tempfile,unittest
import failure as f
import success
D=Path(__file__).resolve().parent;R=D.parent

def data():
 panel=R/'feature-native-resultant-panel';inputs=f.analyzer.fixtures((panel/'inputs.tsv').read_text());names=(panel/'metrics.txt').read_text().splitlines();rows=[];events=[]
 for c in inputs:
  for off in range(5):
   ordinal=len(rows);arm=f.analyzer.ARMS[(off+c['case_id'])%5];m=dict.fromkeys(names,0);m.update(Product=10+(arm=='positive'),Projection=1)
   rows.append(dict({k:c[k]for k in ['case_id','p','kind','n','m','expected']},arm=arm,ordinal=ordinal,value=c['expected'],protocol_sha256='a'*64,input_sha256='b'*64,metrics=m))
   for phase in ['start','complete']:events.append(dict(ordinal=ordinal,phase=phase,protocol_sha256='a'*64,input_sha256='b'*64))
 return inputs,names,rows,events
class FailureTests(unittest.TestCase):
 def test_all_journal_boundaries(self):
  inputs,names,rows,events=data()
  for count in range(481):
   done=count//2;started=(count+1)//2
   flow=f.classify(events[:count],rows[:done],inputs,names,'a'*64,'b'*64)
   self.assertEqual(flow['started'],list(range(started)));self.assertEqual(flow['completed'],list(range(done)));self.assertEqual(len(flow['never_started']),240-started)
   if count%2:
    # Fully written result before completion marker remains unconfirmed.
    flow=f.classify(events[:count],rows[:started],inputs,names,'a'*64,'b'*64);self.assertEqual(flow['completion_unconfirmed'],[done]);self.assertEqual(flow['raw_rows_without_completion_marker'],[done])
 def test_corruption_rejected(self):
  inputs,names,rows,events=data()
  mutations=[lambda e,r:e[0].update(ordinal=1),lambda e,r:e[1].update(ordinal=1),lambda e,r:e[1].update(phase='start'),lambda e,r:e[0].update(protocol_sha256='c'*64),lambda e,r:e[0].update(ordinal=True),lambda e,r:r[0].update(value=123456),lambda e,r:r[0]['metrics'].update(Product=True),lambda e,r:r[0]['metrics'].update(Projection=0),lambda e,r:r[0].update(extra=1)]
  for fn in mutations:
   e=copy.deepcopy(events[:4]);r=copy.deepcopy(rows[:2]);fn(e,r)
   with self.assertRaises(ValueError):f.classify(e,r,inputs,names,'a'*64,'b'*64)
  with self.assertRaises(ValueError):f.classify([],rows[:1],inputs,names,'a'*64,'b'*64)
  with self.assertRaises(ValueError):f.classify(events[:2],[],inputs,names,'a'*64,'b'*64)
 def test_truncated_records(self):
  records,tail=f.prefix(b'{"x":1}\n{"x":');self.assertEqual(records,[{'x':1}]);self.assertEqual(tail,b'{"x":')
  with self.assertRaises((ValueError,UnicodeDecodeError)):f.prefix(b'bad\n')
  with self.assertRaises(ValueError):f.prefix(b'{"x":1,"x":2}\n')
 def test_canonical_partial_and_empty_exports(self):
  inputs,names,rows,events=data();p=json.loads((R/'feature-native-resultant-protocol/protocol.draft.json').read_text())
  for item in p['extensions']['bindings'].values():item['path']=str((R/'feature-native-resultant-protocol'/item['path']).resolve())
  p['analysis_plan']['script']['path']=str(R/'feature-native-resultant-panel/analyze.py');p['sample_size']['preflight_runtime']['estimated_total_seconds']=1;p['analysis_plan']['frozen_before_data']=True
  for arm in p['arms']:arm['instrument']['binary_sha256']='c'*64
  p['freeze'].update(status='frozen',frozen_at='2026-11-07T00:00:00Z',commit='4724db13378bd54a6fa02e810b975a96101b21fe',filed_before_affected_data=True);p['freeze']['protocol_sha256']=f.canonical.protocol_hash(p)
  ih=f.sha(R/'feature-native-resultant-panel/inputs.tsv');ph=p['freeze']['protocol_sha256']
  for r in rows:r.update(protocol_sha256=ph,input_sha256=ih)
  for e in events:e.update(protocol_sha256=ph,input_sha256=ih)
  for event_count,row_count,fragment in [(0,0,False),(1,0,True),(3,1,False),(3,2,False),(480,240,False)]:
   with self.subTest(events=event_count),tempfile.TemporaryDirectory()as tmp:
    out=Path(tmp);pp=out/'protocol.json';pp.write_text(json.dumps(p));rp=out/'rows';ep=out/'events';receipt=out/'receipt'
    rp.write_bytes((''.join(json.dumps(r)+'\n'for r in rows[:row_count])).encode()+(b'{"partial":'if fragment else b''));ep.write_text(''.join(json.dumps(e)+'\n'for e in events[:event_count]))
    r=dict(status='timeout',exit_code=124,protocol_sha256=ph,input_sha256=ih,binary_sha256='c'*64,metrics_sha256=f.sha(R/'feature-native-resultant-panel/metrics.txt'),raw_sha256=f.sha(rp),events_sha256=f.sha(ep));receipt.write_text(json.dumps(r))
    export,flow=f.build(p,pp,rp,ep,receipt,out);self.assertEqual(len(export['runs']),(event_count+1)//2);self.assertFalse(flow['valid_comparison']);self.assertTrue(all(x['exclusion']['excluded']for x in export['runs']))
    self.assertEqual(f.canonical.schema_errors(export,'experiment.export/v1'),[]);self.assertEqual(f.canonical.lint_export(export,None,out,True),[])
    if event_count<480:self.assertTrue(flow['canonical_protocol_findings'])
    if event_count%2:self.assertEqual(export['runs'][-1]['measurements']['counted'],{})
    if event_count==480:
     r.update(status='succeeded',exit_code=0);receipt.write_text(json.dumps(r));ex,analysis=success.build(p,pp,rp,ep,receipt,out);self.assertTrue(analysis['controls_passed']);self.assertEqual(f.canonical.lint_export(ex,p,out,True),[])
     ep.write_text(''.join(json.dumps(e)+'\n'for e in events[:-1]));r['events_sha256']=f.sha(ep);receipt.write_text(json.dumps(r))
     with self.assertRaises(ValueError):success.build(p,pp,rp,ep,receipt,out)
    if event_count==0:
     rp.unlink();ep.unlink();r.update(raw_sha256=None,events_sha256=None,status='infra_error',exit_code=None);receipt.write_text(json.dumps(r));ex,fl=f.build(p,pp,rp,ep,receipt,out);self.assertEqual(ex['runs'],[]);self.assertEqual(len(fl['never_started']),240)
if __name__=='__main__':unittest.main()
