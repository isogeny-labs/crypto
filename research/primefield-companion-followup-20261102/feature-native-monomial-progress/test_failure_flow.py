import unittest,json,hashlib
import failure_flow as f
class Prefixes(unittest.TestCase):
 def encode(self,events):return b''.join(b'progress\t'+json.dumps(x).encode()+b'\n'for x in events)
 def test_every_prefix(self):
  s=f.schedule(f.labels());self.assertEqual(len(s),127)
  for n in range(128):
   out=f.parse(self.encode(s[:n]));self.assertEqual(out['complete_markers'],n);self.assertEqual(len(out['started_queries'])+len(out['not_started']),60);self.assertLessEqual(len(out['active_or_incomplete']),1);self.assertFalse(out['complete_measurement_claim']);self.assertEqual(out['panel_output_finished'],n==127)
 def test_partial_last_marker(self):
  raw=self.encode(f.schedule(f.labels())[:2]);out=f.parse(raw+b'progress\t{"event":');self.assertTrue(out['partial_marker_tail']);self.assertEqual(len(out['started_queries']),1);self.assertEqual(len(out['verified_returns']),0)
 def test_invalid_sequences(self):
  s=f.schedule(f.labels())
  for bad in [s[1:3],[s[0],s[0]],s[:2]+[s[3]],s+[s[-1]],[dict(event='arm_started',arm='candidate',label=-1)]]:
   with self.assertRaises(ValueError):f.parse(self.encode(bad))
  with self.assertRaises(ValueError):f.parse(b'progress\t{"event":"arm_started","arm":"baseline","label":-1,"label":-1}\n')
 def test_receipt_bindings(self):
  stderr=self.encode(f.schedule(f.labels())[:3]);raw=b'partial';r={'exit_code':-9,'timed_out':True,'stderr_sha256':hashlib.sha256(stderr).hexdigest(),'raw_sha256':hashlib.sha256(raw).hexdigest(),'binary_sha256':'b'*64,'protocol_sha256':'c'*64}
  out=f.record(stderr,r,raw,'b'*64,'c'*64);self.assertEqual(len(out['observed_flow']['verified_returns']),1)
  for key,val in [('raw_sha256','0'*64),('stderr_sha256','0'*64),('binary_sha256','0'*64),('protocol_sha256','0'*64),('timed_out',1)]:
   bad=dict(r);bad[key]=val
   with self.assertRaises(ValueError):f.record(stderr,bad,raw,'b'*64,'c'*64)
  r.update(exit_code=0,timed_out=False)
  with self.assertRaises(ValueError):f.record(stderr,r,raw,'b'*64,'c'*64)
if __name__=='__main__':unittest.main()
