"""Mock-only controls: no filesystem, subprocess, compiler, solver or cloud calls."""
import hashlib,signal,subprocess,unittest
from materialize_safety import copy_candidate,cleanup_owned_group
class Bytes:
 def __init__(self,raw):self.raw=raw;self.reads=0
 def read_bytes(self):self.reads+=1;return self.raw
 def write_bytes(self,raw):self.raw=raw
class Changing(Bytes):
 def read_bytes(self):
  raw=super().read_bytes();self.raw=b'changed-after-read';return raw
class CorruptDestination(Bytes):
 def write_bytes(self,raw):self.raw=b'corrupt'
class Process:
 pid=123
 def __init__(self,timeout=False):self.timeout=timeout;self.wait_calls=[]
 def wait(self,timeout):
  self.wait_calls.append(timeout)
  if self.timeout:raise subprocess.TimeoutExpired('mock',timeout)
  return 0
class Safety(unittest.TestCase):
 def test_candidate_read_once_same_verified_bytes(self):
  c=Changing(b'approved');d=Bytes(b'');h=hashlib.sha256(b'approved').hexdigest()
  self.assertEqual(copy_candidate(c,d,h),h);self.assertEqual(c.reads,1);self.assertEqual(d.raw,b'approved')
 def test_candidate_drift_before_read_rejected(self):
  with self.assertRaises(AssertionError):copy_candidate(Bytes(b'changed'),Bytes(b''),hashlib.sha256(b'approved').hexdigest())
 def test_destination_drift_rejected(self):
  with self.assertRaises(AssertionError):copy_candidate(Bytes(b'approved'),CorruptDestination(b''),hashlib.sha256(b'approved').hexdigest())
 def test_exited_leader_still_kills_descendant_group(self):
  calls=[]
  def kill(pg,sig):
   calls.append((pg,sig))
   if sig==0:raise ProcessLookupError()
  p=Process();r=cleanup_owned_group(p,kill)
  self.assertEqual(calls,[(123,signal.SIGKILL),(123,0)]);self.assertEqual(p.wait_calls,[2]);self.assertFalse(r['errors']);self.assertTrue(r['outer_cgroup_confirmation_required'])
 def test_wait_timeout_retained_without_unbounded_wait(self):
  p=Process(True)
  def kill(pg,sig):raise ProcessLookupError()
  r=cleanup_owned_group(p,kill);self.assertEqual(p.wait_calls,[2]);self.assertFalse(r['reaped']);self.assertTrue(r['errors'])
 def test_visible_group_rejects_success(self):
  r=cleanup_owned_group(Process(),lambda pg,sig:None);self.assertFalse(r['group_absent']);self.assertTrue(r['errors'])
if __name__=='__main__':unittest.main()
