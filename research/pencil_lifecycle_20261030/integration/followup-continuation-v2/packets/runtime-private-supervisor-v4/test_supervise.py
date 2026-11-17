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
  self.x.update(mode='qualification',limits=s.envelope('qualification'),predecessor={'owner':'bbbbbbbb-1234-1234-1234-123456789abc','disk_path':'/var/tmp/ppr-bbbbbbbb-1234-1234-1234-123456789abc/storage.ext4'})
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

 def test_known_socket_lengths(self):
  values=s.validate_socket_paths(self.x['owner']);self.assertTrue(all(len(v.encode())<=107 for v in values.values()));self.assertEqual(len(values['assembly.shim']),82)
 def test_byte_bound_not_character_bound(self):
  for value,valid in [('/'+'a'*106,True),('/'+'a'*107,False),('/'+'é'*54,False)]:
   with patch('supervise.socket_paths',return_value={'edge':value}):
    if valid:s.validate_socket_paths(self.x['owner'])
    else:
     with self.assertRaises(ValueError):s.validate_socket_paths(self.x['owner'])
 def test_shim_namespace_property(self):
  props=s.properties(self.x);self.assertIn('TemporaryFileSystem=/run/containerd:ro,size=4096,mode=0700',props)
  self.assertNotIn('-/run/containerd',' '.join(p for p in props if p.startswith('InaccessiblePaths=')))
 def test_mount_parser_topmost_record(self):
  raw='1 0 0:2 / /run/containerd ro - tmpfs tmpfs ro\n2 1 7:1 /shim /run/containerd rw - ext4 /dev/loop0 rw\n';self.assertEqual(s.mount_record('/run/containerd',raw)['fstype'],'ext4')
 def test_shim_mask_rejects_host_filesystem(self):
  w=s.Worker(self.x)
  with patch('supervise.mount_record',return_value={'fstype':'ext4','options':['rw']}):
   with self.assertRaisesRegex(ValueError,'initial private shim mask'):w.setup_shim_mount()
 def test_shim_mount_failure_is_uncertain(self):
  w=s.Worker(self.x);w.p['mount']=self.root;w.p['shim']=self.root/'shim';(self.root/'shim').mkdir(mode=0o700)
  with patch('supervise.mount_record',return_value={'fstype':'tmpfs','options':['ro']}),patch('supervise.subprocess.run',side_effect=TimeoutError('mock timeout')):
   with self.assertRaises(TimeoutError):w.setup_shim_mount()
  self.assertTrue(w.shim_uncertain)
  with self.assertRaisesRegex(ValueError,'uncertain'):w.teardown_shim_mount()
 def test_shim_bind_identity_and_cleanup(self):
  w=s.Worker(self.x);w.p['mount']=self.root;w.p['shim']=self.root/'shim';(self.root/'shim').mkdir(mode=0o700);source_stat=(self.root/'shim').stat();realstat=Path.stat
  def fake_stat(path,*a,**kw):return source_stat if str(path)=='/run/containerd' else realstat(path,*a,**kw)
  records=[{'fstype':'tmpfs','options':['ro']},{'fstype':'ext4','options':['rw']},{'fstype':'tmpfs','options':['ro']}]
  with patch('supervise.mount_record',side_effect=records),patch('supervise.Path.stat',fake_stat),patch('supervise.subprocess.run') as call:
   w.setup_shim_mount();self.assertTrue(w.shim_mounted);w.teardown_shim_mount();self.assertFalse(w.shim_mounted);self.assertFalse(w.shim_uncertain)
   self.assertEqual(call.call_args_list[0].args[0][1:],['--bind',str(self.root/'shim'),'/run/containerd']);self.assertEqual(call.call_args_list[1].args[0][1:],['/run/containerd'])

 def test_retained_disk_distinct_phase_directories(self):
  a=s.Worker(self.x);a.p['mount']=self.root;a.p['exec']=self.root/a.p['exec'].name;a.p['shim']=self.root/a.p['shim'].name;a.setup_runtime_dirs()
  qx=copy.deepcopy(self.x);qx['mode']='qualification';qx['owner']='aaaaaaaa-1234-1234-1234-123456789abc';q=s.Worker(qx);q.p['mount']=self.root;q.p['exec']=self.root/q.p['exec'].name;q.p['shim']=self.root/q.p['shim'].name;q.setup_runtime_dirs()
  self.assertTrue(a.p['exec'].exists() and q.p['exec'].exists());self.assertNotEqual(a.p['exec'],q.p['exec']);self.assertNotEqual(a.p['shim'],q.p['shim']);self.assertNotEqual(a.p['socket'].name,q.p['socket'].name)

class LoopControls(unittest.TestCase):
 def test_empty_list(self):s.require_no_loops(b'{"loopdevices":[]}')
 def test_empty_bytes(self):
  with self.assertRaises(ValueError):s.require_no_loops(b'')
 def test_attached(self):
  with self.assertRaises(ValueError):s.require_no_loops(b'{"loopdevices":[{"name":"/dev/loop0"}]}')
 def test_duplicate(self):
  with self.assertRaises(ValueError):s.require_no_loops(b'{"loopdevices":[],"loopdevices":[]}')
 def test_null(self):
  with self.assertRaises(ValueError):s.require_no_loops(b'{"loopdevices":null}')
 def test_extra(self):
  with self.assertRaises(ValueError):s.require_no_loops(b'{"loopdevices":[],"extra":1}')
 def test_completed_reconcile_explicit_list(self):
  x={'owner':'12345678-1234-1234-1234-123456789abc','mode':'assembly','tools':{'losetup':{'path':'/pinned/losetup'}}};w={'owner':x['owner'],'mounted':False,'shim_mounted':False,'shim_uncertain':False,'disk_path':'/owned/storage.ext4','disk_custody':{'mock':'custody'}}
  with patch('supervise.read_small',side_effect=[b'{}',json.dumps(w).encode()]),patch('supervise.completed_snapshot'),patch('supervise.properties_readback',return_value={'LoadState':'not-found'}),patch('supervise.cgroup_empty',return_value=True),patch('supervise.file_custody',return_value=w['disk_custody']),patch('supervise.subprocess.check_output',return_value=b'{"loopdevices":[]}') as q:
   self.assertEqual(s.reconcile(x)['status'],'assembled_stopped');self.assertEqual(q.call_args.args[0],['/pinned/losetup','--list','--json','--associated','/owned/storage.ext4'])

class SuccessorControls(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup);self.root=Path(self.tmp.name);self.mount=self.root/'retained';self.mount.mkdir();self.src=self.root/'inputs';self.src.mkdir();self.files={}
  for n in ['supervise.py','head_contract.py','private_contract.py','gate.c']:
   raw=('# pinned '+n).encode();p=self.src/n;p.write_bytes(raw);self.files[n]={'path':str(p),'sha256':s.sha(raw)}
  self.binding=self.src/'binding.json';self.binding.write_text(json.dumps({'gate_source_sha256':self.files['gate.c']['sha256']}))
  for n in ['run','Popen','check_output']:
   patch('supervise.subprocess.'+n,side_effect=AssertionError('external execution forbidden')).start();self.addCleanup(patch.stopall)
 def worker(self,owner):
  x={'owner':owner,'mode':'qualification','limits':s.envelope('qualification'),'expires_at':'2099-01-01T00:00:00Z','tools':{'python':{'path':'/pinned/python'}},'binary_sha256':'b'*64,'qualification':{'files':self.files,'source_sha256':self.files['supervise.py']['sha256'],'binding':str(self.binding),'binding_sha256':s.sha(self.binding.read_bytes())}}
  w=s.Worker(x);w.p['mount']=self.mount;w.p['root']=self.root/owner;w.p['root'].mkdir(exist_ok=True);(w.p['root']/'endpoint.json').write_bytes(b'{"pinned":"endpoint"}')
  for n in ['exec','shim','socket']:w.p[n]=self.mount/w.p[n].name
  return w
 def test_distinct_full_owner_bundles_preserve_legacy(self):
  old=self.mount/'qualifier-source';old.mkdir();(old/'evidence').write_bytes(b'old');(self.mount/'qualification-receipts').mkdir()
  a=self.worker('aaaaaaaa-1111-4111-8111-111111111111');b=self.worker('bbbbbbbb-1111-4111-8111-111111111111');sa,ba=a.qualification_bundle();sb,bb=b.qualification_bundle();self.assertNotEqual(sa,sb);self.assertEqual(sa.read_bytes(),sb.read_bytes());self.assertEqual((old/'evidence').read_bytes(),b'old');self.assertNotEqual(a.qualification_paths()[1],b.qualification_paths()[1])
 def test_same_owner_source_collision_preserves_bytes(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');src,_=w.qualification_bundle();before=src.read_bytes()
  with self.assertRaises(FileExistsError):w.qualification_bundle()
  self.assertEqual(src.read_bytes(),before)
 def test_receipts_collision_precedes_copy(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');src,rec=w.qualification_paths();rec.mkdir();(rec/'evidence').write_bytes(b'old')
  with self.assertRaisesRegex(ValueError,'receipts collision'):w.qualification_bundle()
  self.assertFalse(src.exists());self.assertEqual((rec/'evidence').read_bytes(),b'old')
 def test_qualifier_source_pin_still_rejects(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');(self.src/'head_contract.py').write_bytes(b'changed')
  with self.assertRaisesRegex(ValueError,'dependency pin'):w.qualification_bundle()
 def test_callback_binds_actual_fresh_paths(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');src,b=w.qualification_bundle();a=w.qualification_argv('sha256:'+'c'*64,src,b);self.assertEqual(a[2],str(src));self.assertEqual(a[a.index('--binding')+1],str(b));self.assertEqual(a[a.index('--receipts')+1],str(w.qualification_paths()[1]));self.assertEqual(a[a.index('--private-endpoint-sha256')+1],s.sha((w.p['root']/'endpoint.json').read_bytes()));self.assertEqual(b.read_bytes(),self.binding.read_bytes())
 def test_different_short_prefix_runtime_reuse(self):
  a=self.worker('aaaaaaaa-1111-4111-8111-111111111111');b=self.worker('bbbbbbbb-1111-4111-8111-111111111111');a.setup_runtime_dirs();b.setup_runtime_dirs();self.assertNotEqual(a.p['exec'],b.p['exec']);self.assertNotEqual(a.p['shim'],b.p['shim']);self.assertNotEqual(a.p['socket'],b.p['socket'])
 def test_shared_short_prefix_rejected_without_overwrite(self):
  a=self.worker('aaaaaaaa-1111-4111-8111-111111111111');b=self.worker('aaaaaaaa-2222-4222-8222-222222222222');a.setup_runtime_dirs();marker=a.p['exec']/'evidence';marker.write_bytes(b'old')
  with self.assertRaises(FileExistsError):b.setup_runtime_dirs()
  self.assertEqual(marker.read_bytes(),b'old');self.assertEqual(a.p['exec'],b.p['exec'])
 def test_retained_socket_rejected(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');w.p['socket'].write_bytes(b'old')
  with self.assertRaisesRegex(ValueError,'socket collision'):w.setup_runtime_dirs()
  self.assertEqual(w.p['socket'].read_bytes(),b'old')
 def test_dangling_socket_rejected(self):
  w=self.worker('aaaaaaaa-1111-4111-8111-111111111111');w.p['socket'].symlink_to(self.root/'missing')
  with self.assertRaisesRegex(ValueError,'socket collision'):w.setup_runtime_dirs()
 def test_known_longest_paths(self):
  vals=s.validate_socket_paths('ffffffff-ffff-4fff-8fff-ffffffffffff');self.assertEqual({k:len(v.encode()) for k,v in vals.items() if k.startswith('qualification.')},{'qualification.docker':71,'qualification.containerd':93,'qualification.containerd_debug':99,'qualification.containerd_ttrpc':99,'qualification.shim':82});self.assertEqual(len(vals['assembly.docker']),62)

if __name__=='__main__':unittest.main(verbosity=2)
