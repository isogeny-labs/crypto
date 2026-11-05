import copy,json,os,tempfile,time,unittest
from pathlib import Path
from unittest.mock import patch,Mock
import supervise as s

class Controls(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name);self.addCleanup(self.tmp.cleanup)
  for name in ['run','Popen','check_output']:
   self.addCleanup(patch.stopall);patch('supervise.subprocess.'+name,side_effect=AssertionError('external execution forbidden')).start()
  self.x={'owner':'12345678-1234-1234-1234-123456789abc','mode':'assembly','limits':s.envelope('assembly'),'base_image':s.BASE,'base_archive':{'sha256':s.ARCHIVE_SHA,'bytes':s.ARCHIVE_BYTES,'path':'/pinned/base.tar'},'expires_at':'2099-01-01T00:00:00Z','supervisor_sha256':s.sha(Path(s.__file__).read_bytes()),'tools':{k:{'path':'/usr/bin/'+k,'sha256':'a'*64} for k in s.REQUIRED_TOOLS},'author':'producer','qualification':None,'predecessor':None}
  self.review={'decision':'approve','scope':'one-private-runtime-assembly','inputs_sha256':s.sha(s.canonical(self.x)),'independent':True,'reviewer':'reviewer','execution_allowance_approved':True};self.rp=self.root/'review.json';self.rp.write_text(json.dumps(self.review));self.b={'admitted':True,'inputs':self.x,'review_path':str(self.rp),'review_sha256':s.sha(s.canonical(self.review))}
 def test_valid_structure_requires_root_operator(self):
  with patch('supervise.os.geteuid',return_value=1000):
   with self.assertRaises(ValueError):s.authenticate(self.b)
  with patch('supervise.os.geteuid',return_value=0):self.assertEqual(s.authenticate(self.b),self.x)
 def test_unadmitted_rejected(self):
  self.b['admitted']=False
  with self.assertRaises(ValueError):s.validate(self.b)
 def test_stale_review_rejected(self):
  self.x['owner']='aaaaaaaa-1234-1234-1234-123456789abc'
  with self.assertRaises(ValueError):s.validate(self.b)
 def test_null_tool_pin_rejected(self):
  self.x['tools']['docker']['sha256']='pending'
  with self.assertRaises(ValueError):s.validate(self.b)
 def test_archive_substitution_rejected(self):
  self.x['base_archive']['sha256']='b'*64
  with self.assertRaises(ValueError):s.validate(self.b)
 def test_phase_limits_not_combined(self):
  self.x['limits']['total_seconds']=900
  with self.assertRaises(ValueError):s.validate(self.b)
  self.assertEqual(s.envelope('qualification')['total_seconds'],300)
 def test_service_contains_finite_controls(self):
  props=s.properties(self.x)
  for field in ['Type=exec','RemainAfterExit=yes','KillMode=control-group','TimeoutStopSec=5s','RuntimeMaxSec=590s','CPUQuota=200%','MemoryMax=2147483648','MemorySwapMax=0','TasksMax=256','PrivateNetwork=yes','PrivateMounts=yes','Restart=no']:self.assertIn(field,props)
  argv=s.launch_argv(self.x,self.root/'binding.json');self.assertNotIn('--scope',argv);self.assertIn('--no-block',argv)
 def test_qualification_narrow_disk_allowlist(self):
  self.x.update(mode='qualification',limits=s.envelope('qualification'),predecessor={'owner':'bbbbbbbb-1234-1234-1234-123456789abc','disk_path':'/var/tmp/pencil-private-bbbbbbbb-1234-1234-1234-123456789abc/storage.ext4'})
  props=s.properties(self.x);self.assertIn('RuntimeMaxSec=290s',props);self.assertTrue(any(self.x['predecessor']['disk_path'] in p for p in props if p.startswith('ReadWritePaths=')))
  self.x['predecessor']['disk_path']='/var/lib/docker'
  with self.assertRaises(ValueError):s.properties(self.x)
 def test_private_daemon_and_cli_endpoints(self):
  p=s.paths(self.x['owner']);args=s.daemon_argv(self.x,p,s.service_group(p['unit'])+'/payload');self.assertIn('--host=unix://'+str(p['socket']),args);self.assertIn('--exec-opt=native.cgroupdriver=cgroupfs',args);self.assertFalse(any(a.startswith('--containerd=') for a in args));self.assertIn('--bridge=none',args)
  cli=s.docker_argv(self.x,p,['info']);self.assertIn('unix://'+str(p['socket']),cli);self.assertNotIn('/var/run/docker.sock',' '.join(cli));self.assertNotIn('sudo',cli)
 def test_expired_admission_and_readonly_reconcile(self):
  self.x['expires_at']='2000-01-01T00:00:00Z'
  with self.assertRaises(ValueError):s.validate(self.b)
  self.review['inputs_sha256']=s.sha(s.canonical(self.x));self.rp.write_text(json.dumps(self.review));self.b['review_sha256']=s.sha(s.canonical(self.review));self.assertEqual(s.validate(self.b,allow_expired=True),self.x)
 def test_duplicate_json_rejected(self):
  with self.assertRaises(ValueError):s.strict('{"x":1,"x":2}')
 def test_non_main_thread_containerd_discovery(self):
  t1=self.root/'100';t2=self.root/'101';t1.mkdir();t2.mkdir();(t1/'children').write_text('');(t2/'children').write_text('220 221')
  with patch('supervise.Path.iterdir',return_value=iter([t1,t2])):self.assertEqual(s.managed_children(100),{220,221})
 def test_metadata_only_readiness_retry(self):
  w=object.__new__(s.Worker);w.daemon=Mock();w.daemon.poll.return_value=None;w.deadline=time.monotonic()+30;w.guard=Mock();w.docker=Mock(side_effect=[ValueError('not ready'),b'{"ID":"private"}'])
  with patch('supervise.time.sleep'):self.assertEqual(w.ready_info(),{'ID':'private'})
  self.assertEqual(w.docker.call_count,2);self.assertTrue(all(c.args[0][0]=='info' for c in w.docker.call_args_list))
 def test_early_setup_failure_has_receipt(self):
  w=s.Worker(self.x);w.p['root']=self.root;w.check_tools=Mock(side_effect=ValueError('pin mismatch'))
  with self.assertRaises(ValueError):w.run()
  r=json.loads((self.root/'worker-result.json').read_text());self.assertEqual(r['status'],'worker_unresolved');self.assertFalse(r['mounted']);self.assertIn('pin mismatch',r['error'])
 def test_success_snapshot_requires_exact_invocation(self):
  with patch('supervise.ROOT',self.root):
   root=s.paths(self.x['owner'])['root'];root.mkdir();(root/'worker-result.json').write_text(json.dumps({'status':'worker_phase_complete','service_invocation_id':'123'}))
   props={'Description':s.expected_description(self.x),'ActiveState':'active','SubState':'exited','Result':'success','ExecMainStatus':'0','InvocationID':'123'};self.assertEqual(s.completed_snapshot(self.x,props),props)
   props['InvocationID']='different'
   with self.assertRaises(ValueError):s.completed_snapshot(self.x,props)
 def test_remaining_deadline_rejects_late_success(self):
  with self.assertRaises(ValueError):s.remaining(time.monotonic()-1)
 def test_foreign_unit_not_stopped(self):
  with patch('supervise.query',return_value=b'foreign') as q:
   with self.assertRaises(ValueError):s.stop_owned(self.x)
   self.assertEqual(q.call_count,1)
 def test_reconcile_populated_cgroup_rejected(self):
  with patch('supervise.read_small',return_value=b'{}'),patch('supervise.completed_snapshot'),patch('supervise.properties_readback',return_value={'LoadState':'not-found'}),patch('supervise.cgroup_empty',return_value=False):
   with self.assertRaisesRegex(ValueError,'cgroup unresolved'):s.reconcile(self.x)
 def test_context_modes_survive_private_umask(self):
  src=self.root/'input';src.mkdir();(src/'runtime').mkdir(mode=0o755);(src/'runtime').chmod(0o755);(src/'runtime'/'payload').write_bytes(b'x');(src/'runtime'/'payload').chmod(0o644);recipe=b'ARG BASE_IMAGE\nFROM ${BASE_IMAGE}\nCOPY runtime/ /opt/runtime/\n';(src/'Dockerfile').write_bytes(recipe);(src/'Dockerfile').chmod(0o644)
  members=[{'path':'runtime','kind':'directory','mode':'0o755'},{'path':'runtime/payload','kind':'file','mode':'0o644','bytes':1,'sha256':s.sha(b'x')},{'path':'Dockerfile','kind':'file','mode':'0o644','bytes':len(recipe),'sha256':s.sha(recipe)}]
  self.x['context']={'path':str(src),'members':members,'root_mode':'0o755','recipe_sha256':s.sha(recipe)};w=s.Worker(self.x);mount=self.root/'mounted';mount.mkdir();w.p['mount']=mount;old=os.umask(0o077)
  try:dest=w.context()
  finally:os.umask(old)
  self.assertEqual((dest/'runtime').stat().st_mode&0o777,0o755);self.assertEqual((dest/'runtime'/'payload').stat().st_mode&0o777,0o644)
 def test_qualifier_dependency_snapshot(self):
  source=self.root/'qualifier';source.mkdir();files={}
  for name in ['supervise.py','head_contract.py','private_contract.py','gate.c']:
   raw=('# '+name+'\n').encode();(source/name).write_bytes(raw);files[name]={'path':str(source/name),'sha256':s.sha(raw)}
  binding=source/'binding.json';raw_binding=json.dumps({'gate_source_sha256':files['gate.c']['sha256']}).encode();binding.write_bytes(raw_binding);self.x['qualification']={'files':files,'source_sha256':files['supervise.py']['sha256'],'binding':str(binding),'binding_sha256':s.sha(raw_binding)};w=s.Worker(self.x);mount=self.root/'mounted';mount.mkdir();w.p['mount']=mount;a,b=w.qualification_bundle();self.assertEqual(a.read_bytes(),(source/'supervise.py').read_bytes());self.assertEqual(a.stat().st_mode&0o777,0o500);self.assertEqual(b.stat().st_mode&0o777,0o400)
 def test_stop_waits_for_inactive_not_only_empty(self):
  states=[{'LoadState':'loaded','Description':s.expected_description(self.x),'ActiveState':'deactivating'},{'LoadState':'loaded','Description':s.expected_description(self.x),'ActiveState':'inactive'}]
  with patch('supervise.properties_readback',side_effect=states) as q,patch('supervise.cgroup_empty',return_value=True),patch('supervise.time.sleep'):
   s.wait_stopped(self.x,time.monotonic()+30);self.assertEqual(q.call_count,2)

if __name__=='__main__':unittest.main(verbosity=2)
