"""Unprivileged real file controls; UID/GID alone substituted for root custody.
Proc anchors, mount observations and pidfd are mocked; no proc writes occur.
"""
import contextlib,os,stat,tempfile,types,unittest
from pathlib import Path
from unittest.mock import patch,Mock
import private_contract as c
N='12345678-1234-1234-1234-123456789abc'
class Release(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.p=Path(self.tmp.name)
  self.fd=os.open(self.p,os.O_RDONLY|os.O_DIRECTORY);self.records=[]
  actual=os.fstat
  def rootstat(fd):
   st=actual(fd)
   return types.SimpleNamespace(**{k:(0 if k in ('st_uid','st_gid') else getattr(st,k)) for k in ('st_mode','st_uid','st_gid','st_nlink','st_size','st_dev','st_ino')})
  self.mock=patch.object(c.os,'fstat',side_effect=rootstat);self.mock.start()
 def tearDown(self):self.mock.stop();os.close(self.fd);self.tmp.cleanup()
 def publish(self,before=lambda:None):return c._publish_token(self.fd,N,before,self.records.append)
 def test_real_atomic_umask(self):
  old=os.umask(0o077)
  try:r=self.publish()
  finally:os.umask(old)
  self.assertTrue(r['publication_confirmed']);self.assertEqual((self.p/'gate-release').read_bytes(),(N+'\n').encode())
  st=(self.p/'gate-release').stat();self.assertEqual(stat.S_IMODE(st.st_mode),0o444);self.assertEqual(st.st_nlink,1)
  self.assertFalse(list(self.p.glob('*.partial')))
 def test_final_collisions(self):
  for kind in ('file','symlink','fifo','directory'):
   with self.subTest(kind=kind):
    target=self.p/'gate-release'
    if kind=='file':target.write_bytes(b'old')
    elif kind=='symlink':target.symlink_to('missing')
    elif kind=='fifo':os.mkfifo(target)
    else:target.mkdir()
    with self.assertRaises(OSError):self.publish()
    self.assertFalse(self.records[-1]['publication_confirmed'])
    if kind=='directory':target.rmdir()
    else:target.unlink()
    for stage in self.p.glob('*.partial'):stage.unlink()
 def test_stage_collision(self):
  stage=self.p/('.gate-release-'+N+'.partial');stage.symlink_to('absent')
  with self.assertRaises(FileExistsError):self.publish()
  self.assertTrue(stage.is_symlink());self.assertFalse((self.p/'gate-release').exists())
 def test_prepublication_denial_cleans_stage(self):
  with self.assertRaisesRegex(RuntimeError,'denied'):self.publish(lambda:(_ for _ in ()).throw(RuntimeError('denied')))
  self.assertEqual(list(self.p.iterdir()),[]);self.assertFalse(self.records[-1]['publication_attempted'])
 def test_short_write_completes(self):
  write=os.write
  with patch.object(c.os,'write',side_effect=lambda fd,b:write(fd,b[:3])):self.publish()
  self.assertEqual((self.p/'gate-release').stat().st_size,37)
 def test_zero_write_rejects(self):
  with patch.object(c.os,'write',return_value=0):
   with self.assertRaises(c.ContractError):self.publish()
  self.assertEqual(list(self.p.iterdir()),[])
 def test_uncertain_publish_never_retries(self):
  rename=c._rename_exclusive
  def interrupted(fd,name):rename(fd,name);raise KeyboardInterrupt()
  with patch.object(c,'_rename_exclusive',side_effect=interrupted) as fn:
   with self.assertRaises(KeyboardInterrupt):self.publish()
   self.assertEqual(fn.call_count,1)
  self.assertTrue((self.p/'gate-release').exists());self.assertTrue(self.records[-1]['publication_uncertain'])
 def test_unsupported_no_fallback(self):
  with patch.object(c,'_rename_exclusive',side_effect=OSError(38,'unsupported')) as fn:
   with self.assertRaises(OSError):self.publish()
   self.assertEqual(fn.call_count,1)
  self.assertFalse((self.p/'gate-release').exists())
 def test_wrong_token_bytes_mode_alias(self):
  f=self.p/'x';f.write_bytes((N+'\n').encode());fd=os.open(f,os.O_RDWR)
  try:
   for mutation in ('mode','partial','wrong','alias'):
    with self.subTest(mutation=mutation):
     os.ftruncate(fd,0);os.write(fd,(N+'\n').encode());os.fchmod(fd,0o444)
     if mutation=='mode':os.fchmod(fd,0o400)
     elif mutation=='partial':os.ftruncate(fd,36)
     elif mutation=='wrong':os.pwrite(fd,b'X',0)
     else:os.link(f,self.p/'alias')
     with self.assertRaises(c.ContractError):c._token_custody(fd,N)
     os.lseek(fd,0,os.SEEK_SET)
  finally:os.close(fd)
 def test_cleanup_does_not_unlink_replacement(self):
  stage=self.p/('.gate-release-'+N+'.partial')
  def replaced():stage.unlink();stage.write_bytes(b'replacement');raise RuntimeError('stop')
  with self.assertRaises(RuntimeError):self.publish(replaced)
  self.assertEqual(stage.read_bytes(),b'replacement')
 def test_invalid_nonce_no_write(self):
  with self.assertRaises(c.ContractError):c._publish_token(self.fd,'../bad',lambda:None,self.records.append)
  self.assertEqual(list(self.p.iterdir()),[])

class Anchors(unittest.TestCase):
 def expected(self):return dict(pid=42,start_ticks=99,cgroup='/unit/payload/id',nonce=N)
 def test_mock_anchor_order_and_close(self):
  expected=self.expected();events=[]
  with contextlib.ExitStack() as s:
   s.enter_context(patch.object(c.os,'open',side_effect=[10,11,13,14,15]))
   s.enter_context(patch.object(c.os,'pidfd_open',return_value=12))
   close=s.enter_context(patch.object(c.os,'close'))
   s.enter_context(patch.object(c,'_fs_type',return_value=0x9fa0))
   s.enter_context(patch.object(c,'_held_identity',return_value=(99,expected['cgroup'])))
   s.enter_context(patch.object(c,'_pid_live'))
   s.enter_context(patch.object(c,'_work_custody',return_value=(1,2)))
   def pub(work,nonce,before,record):before();events.append('published');return {'ok':True}
   s.enter_context(patch.object(c,'_publish_token',side_effect=pub))
   c.publish_release_fd(expected,N,c.time.monotonic()+10,lambda:expected,lambda _:None)
   self.assertEqual(events,['published']);self.assertEqual([x.args[0] for x in close.call_args_list],[15,14,13,12,11,10])
 def test_anchor_failures_before_write(self):
  expected=self.expected()
  for failure in ('start','death','proc','access','ancestry','deadline','work'):
   with self.subTest(failure=failure),contextlib.ExitStack() as s:
    s.enter_context(patch.object(c.os,'open',side_effect=PermissionError('denied') if failure=='access' else [10,11,13,14,15]))
    s.enter_context(patch.object(c.os,'pidfd_open',return_value=12));s.enter_context(patch.object(c.os,'close'))
    s.enter_context(patch.object(c,'_fs_type',return_value=0 if failure=='proc' else 0x9fa0))
    s.enter_context(patch.object(c,'_held_identity',return_value=(98 if failure=='start' else 99,expected['cgroup'])))
    s.enter_context(patch.object(c,'_pid_live',side_effect=c.ContractError('dead') if failure=='death' else None))
    s.enter_context(patch.object(c,'_work_custody',side_effect=c.ContractError('work') if failure=='work' else None,return_value=(1,2)))
    pub=s.enter_context(patch.object(c,'_publish_token'))
    with self.assertRaises((c.ContractError,PermissionError)):
     c.publish_release_fd(expected,N,c.time.monotonic()+(-1 if failure=='deadline' else 10),lambda:{} if failure=='ancestry' else expected,lambda _:None)
    pub.assert_not_called()
 def test_held_stat_change(self):
  def row(t):return '42 (gate name) S '+' '.join(['0']*18+[str(t)])+'\n'
  with patch.object(c,'_bounded_at',side_effect=[row(99),'0::/unit/payload/id\n',row(100)]):
   with self.assertRaises(c.ContractError):c._held_identity(1,42)
 def test_pidfd_exit(self):
  with patch.object(c.select,'poll') as p:
   p.return_value.poll.return_value=[(12,1)]
   with self.assertRaises(c.ContractError):c._pid_live(12)
 def test_work_custody_rejections(self):
  good=types.SimpleNamespace(st_mode=stat.S_IFDIR|0o1777,st_uid=0,st_gid=0,st_dev=1,st_ino=2)
  flags=os.ST_NOSUID|os.ST_NODEV|os.ST_NOEXEC
  for failure in ('root','type','owner','mode','magic','flags','capacity'):
   with self.subTest(failure=failure):
    st=types.SimpleNamespace(**vars(good))
    if failure=='type':st.st_mode=stat.S_IFREG|0o1777
    if failure=='owner':st.st_uid=1000
    if failure=='mode':st.st_mode=stat.S_IFDIR|0o777
    root=types.SimpleNamespace(f_flag=0 if failure=='root' else os.ST_RDONLY)
    work=types.SimpleNamespace(f_flag=0 if failure=='flags' else flags,f_blocks=8192 if failure=='capacity' else 4096,f_frsize=4096)
    with patch.object(c.os,'fstat',return_value=st),patch.object(c.os,'fstatvfs',side_effect=[root,work]),patch.object(c,'_fs_type',return_value=0 if failure=='magic' else 0x01021994):
     with self.assertRaises(c.ContractError):c._work_custody(1,2)
 def test_final_revalidation_denies_publication(self):
  expected=self.expected()
  with contextlib.ExitStack() as s:
   s.enter_context(patch.object(c.os,'open',side_effect=[10,11,13,14]))
   s.enter_context(patch.object(c.os,'pidfd_open',return_value=12));s.enter_context(patch.object(c.os,'close'))
   s.enter_context(patch.object(c,'_fs_type',return_value=0x9fa0));s.enter_context(patch.object(c,'_held_identity',return_value=(99,expected['cgroup'])))
   s.enter_context(patch.object(c,'_pid_live'));s.enter_context(patch.object(c,'_work_custody',return_value=(1,2)))
   commits=[]
   def pub(work,nonce,before,record):before();commits.append(True)
   s.enter_context(patch.object(c,'_publish_token',side_effect=pub))
   with self.assertRaises(c.ContractError):c.publish_release_fd(expected,N,c.time.monotonic()+10,Mock(side_effect=[expected,expected,{}]),lambda _:None)
   self.assertEqual(commits,[])

class DurableBoundary(Release):
 # Inherits the same real-file controls; two targeted new reviewer findings below.
 def test_record_io_changes_boundary_before_rename(self):
  allowed=[True]
  def boundary():c.require(allowed[0],'deferred signal/deadline')
  def record(r):self.records.append(r);allowed[0]=False
  with patch.object(c,'_rename_exclusive') as rename:
   with self.assertRaises(c.ContractError):c._publish_token(self.fd,N,boundary,record)
   rename.assert_not_called()
  self.assertFalse((self.p/'gate-release').exists());self.assertEqual(list(self.p.iterdir()),[])
 def test_immutable_receipt_survives_confirmation_failure(self):
  records=self.p/'records';records.mkdir();calls=[0]
  def record(r):
   calls[0]+=1
   if calls[0]>1:raise OSError('confirmation failure')
   import json
   with (records/'prepublication.json').open('x') as f:json.dump(r,f);f.flush();os.fsync(f.fileno())
  with self.assertRaises(OSError):c._publish_token(self.fd,N,lambda:None,record)
  import json
  kept=json.loads((records/'prepublication.json').read_text())
  self.assertTrue(kept['publication_possible']);self.assertTrue((self.p/'gate-release').exists())
