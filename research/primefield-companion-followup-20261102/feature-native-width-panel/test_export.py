"""Temporary synthetic records only; no production execution or freeze."""
import copy,json,tempfile,unittest
from pathlib import Path
import export as e
from test_analyze import fixture
class Tests(unittest.TestCase):
 def test_adapter_and_refusals(self):
  with tempfile.TemporaryDirectory()as temp:
   d=Path(temp);pp=d/'protocol.json';raw=d/'rows.jsonl';receipt=d/'receipt.json'
   p=e.read(e.D/'protocol.draft.json')
   for v in p['extensions']['bindings'].values():v['path']=str((e.D/v['path']).resolve())
   p['analysis_plan']['script']['path']=str(e.D/'analyze.py');p['analysis_plan']['frozen_before_data']=True
   p['sample_size']['preflight_runtime']['estimated_total_seconds']=1
   for arm in p['arms']:arm['instrument'].update(commit='a'*40,binary_sha256='b'*64)
   p['freeze'].update(status='frozen',commit='a'*40,frozen_at='2026-11-07T00:00:00Z',filed_before_affected_data=True)
   p['freeze']['protocol_sha256']=e.canonical.protocol_hash(p);pp.write_text(json.dumps(p))
   raw.write_text(''.join(json.dumps(x)+'\n'for x in fixture()))
   r=dict(exit_code=0,raw_sha256=e.sha(raw),protocol_sha256=p['freeze']['protocol_sha256'],binary_sha256='b'*64,source_commit='a'*40,execution_admission_uri='synthetic-only://not-an-admission',execution_admission_sha256='c'*64)
   r['source_bindings']={k:v['sha256']for k,v in p['extensions']['bindings'].items()}
   receipt.write_text(json.dumps(r));out,summary=e.build(pp,raw,receipt,d)
   self.assertEqual(len(out['runs']),100);self.assertEqual(summary['rows'],200)
   self.assertIn('doubled.scalar_products',out['runs'][0]['measurements']['counted'])
   for field,value in [('exit_code',1),('exit_code',False),('raw_sha256','d'*64),('protocol_sha256','d'*64),('binary_sha256','d'*64),('source_commit','d'*40),('execution_admission_uri',None),('execution_admission_sha256','bad'),('source_bindings',{})]:
    with self.subTest(field=field):
     bad=r.copy();bad[field]=value;receipt.write_text(json.dumps(bad))
     with self.assertRaises(ValueError):e.build(pp,raw,receipt,d)
   receipt.write_text(json.dumps(r));p['freeze']['status']='draft';pp.write_text(json.dumps(p))
   with self.assertRaises(ValueError):e.build(pp,raw,receipt,d)
if __name__=='__main__':unittest.main()
