import unittest,tempfile,sys,json
from pathlib import Path
from capture import capture,sha,canonical_protocol
class Capture(unittest.TestCase):
 def run_stub(self,code,timeout=2):
  self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup);out=Path(self.temp.name)/'attempt';r=capture([sys.executable,'-c',code],out,timeout,{'protocol_sha256':'a'*64});return out,r
 def test_success_and_exclusive(self):
  out,r=self.run_stub('print("synthetic")');self.assertEqual(r['exit_code'],0);self.assertFalse(r['timed_out']);self.assertEqual(r['raw_sha256'],sha(out/'rows.jsonl'))
  with self.assertRaises(FileExistsError):capture([sys.executable,'-c','raise Exception()'],out,1,{})
  self.assertEqual((out/'rows.jsonl').read_text(),'synthetic\n')
 def test_nonzero_retained(self):
  out,r=self.run_stub('import sys;print("partial",flush=True);print("error",file=sys.stderr);sys.exit(7)');self.assertEqual(r['exit_code'],7);self.assertEqual((out/'stderr.log').read_text(),'error\n')
 def test_timeout_retained(self):
  out,r=self.run_stub('import time;print("partial",flush=True);time.sleep(10)',.2);self.assertTrue(r['timed_out']);self.assertNotEqual(r['exit_code'],0);self.assertIn((out/'rows.jsonl').read_text(),('', 'partial\n'));self.assertEqual(r['raw_sha256'],sha(out/'rows.jsonl'));self.assertTrue((out/'attempt.json').exists())
 def test_missing_executable(self):
  with tempfile.TemporaryDirectory()as root:
   out=Path(root)/'attempt';r=capture(['/nonexistent-monomial-test-runner'],out,1,{});self.assertIsNone(r['exit_code']);self.assertIn('spawn_error',r)
 def test_canonical_hash(self):
  a={'x':1,'freeze':{'status':'draft'}};b={'freeze':{'status':'frozen'},'x':1};self.assertEqual(canonical_protocol(a),canonical_protocol(b));b['x']=2;self.assertNotEqual(canonical_protocol(a),canonical_protocol(b))
if __name__=='__main__':unittest.main()
