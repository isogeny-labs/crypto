"""Prospective only: never invoked by this source preparation."""
import json,tempfile,unittest
from pathlib import Path
from panel_acceptance import trace_identity
class IdentityNegatives(unittest.TestCase):
 def check_rejected(self,field):
  fixture=json.loads((Path(__file__).parent/'driver-fixture.json').read_text())
  identity=dict(source_revision='a'*40,source_sha256='b'*64,workload_id=fixture['target']['workload_id'],run_id='observed')
  identity[field]='c'*len(identity[field])
  with tempfile.TemporaryDirectory() as tmp:
   p=Path(tmp)/'trace.jsonl';p.write_text(json.dumps(dict(sequence=0,event=dict(kind='ObservedHeader',data=dict(identity=identity,native={}))))+'\n')
   with self.assertRaisesRegex(AssertionError,'trace source identity mismatch'):
    trace_identity(p,fixture,'a'*40,'b'*64,'observed')
 def test_changed_revision(self):self.check_rejected('source_revision')
 def test_changed_source_hash(self):self.check_rejected('source_sha256')
if __name__=='__main__':unittest.main()
