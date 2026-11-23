import unittest,copy
import reconcile as r
class Controls(unittest.TestCase):
 def test_empty_json(self):self.assertTrue(r.loops_empty(b'{"loopdevices":[]}'))
 def test_empty_bytes_rejected(self):
  with self.assertRaises(ValueError):r.loops_empty(b'')
 def test_attached_rejected(self):
  with self.assertRaises(ValueError):r.loops_empty(b'{"loopdevices":[{"name":"/dev/loop0"}]}')
 def test_null_rejected(self):
  with self.assertRaises(ValueError):r.loops_empty(b'{"loopdevices":null}')
 def test_duplicate_rejected(self):
  with self.assertRaises(ValueError):r.loops_empty(b'{"loopdevices":[],"loopdevices":[]}')
 def test_extra_shape_rejected(self):
  with self.assertRaises(ValueError):r.loops_empty(b'{"loopdevices":[],"unknown":1}')
 def test_collected(self):self.assertTrue(r.stopped({'LoadState':'not-found','ActiveState':'inactive','SubState':'dead','ControlGroup':''},'d','i'))
 def test_notfound_active_rejected(self):
  with self.assertRaises(ValueError):r.stopped({'LoadState':'not-found','ActiveState':'active','SubState':'running'},'d','i')
 def test_loaded_owned(self):self.assertTrue(r.stopped({'LoadState':'loaded','ActiveState':'inactive','SubState':'dead','Description':'d','InvocationID':'i'},'d','i'))
 def test_wrong_owner(self):
  with self.assertRaises(ValueError):r.stopped({'LoadState':'loaded','ActiveState':'inactive','SubState':'dead','Description':'other','InvocationID':'i'},'d','i')
 def test_wrong_invocation(self):
  with self.assertRaises(ValueError):r.stopped({'LoadState':'loaded','ActiveState':'inactive','SubState':'dead','Description':'d','InvocationID':'other'},'d','i')
 def fixture(self):
  x={'owner':r.OWNER,'mode':'assembly'};desc='pencil-private-'+r.OWNER+'-'+r.sha(r.canonical(x));b={'Description':desc,'ActiveState':'active','SubState':'exited','Result':'success','ExecMainStatus':'0','InvocationID':'i'};w={'owner':r.OWNER,'mode':'assembly','status':'worker_phase_complete','service_invocation_id':'i','unit':r.UNIT,'service_cgroup':r.GROUP,'mounted':False,'shim_mounted':False,'shim_uncertain':False,'endpoint_live':False,'disk_path':str(r.ROOT/'storage.ext4'),'disk_custody':{'bytes':4*1024**3}};return x,b,w
 def test_saved_good(self):self.assertTrue(r.saved_ok(*self.fixture()))
 def test_teardown_unknown(self):
  x,b,w=self.fixture();w['shim_uncertain']=True
  with self.assertRaises(ValueError):r.saved_ok(x,b,w)
 def test_missing_false(self):
  x,b,w=self.fixture();del w['endpoint_live']
  with self.assertRaises(ValueError):r.saved_ok(x,b,w)
 def test_worker_not_complete(self):
  x,b,w=self.fixture();w['status']='worker_unresolved'
  with self.assertRaises(ValueError):r.saved_ok(x,b,w)
if __name__=='__main__':unittest.main()
