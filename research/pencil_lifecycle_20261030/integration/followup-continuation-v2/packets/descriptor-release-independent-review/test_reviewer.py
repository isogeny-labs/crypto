import os,tempfile,unittest,contextlib
from pathlib import Path
from unittest.mock import patch
import private_contract as c
class ReviewerControls(unittest.TestCase):
 def test_bounded_observation_and_symlink(self):
  with tempfile.TemporaryDirectory() as tmp:
   p=Path(tmp);(p/'stat').write_text('abc');(p/'link').symlink_to('stat')
   fd=os.open(tmp,os.O_RDONLY|os.O_DIRECTORY)
   try:
    self.assertEqual(c._bounded_at(fd,'stat',3),'abc')
    with self.assertRaises(c.ContractError):c._bounded_at(fd,'stat',2)
    with self.assertRaises(OSError):c._bounded_at(fd,'link',3)
   finally:os.close(fd)
 def test_abi_reject_before_libc(self):
  with patch.object(c.platform,'machine',return_value='aarch64'),patch.object(c.ctypes,'CDLL') as lib:
   with self.assertRaises(c.ContractError):c._linux_libc()
   lib.assert_not_called()
 def test_kernel_root_only_nofollow_exception(self):
  e=dict(pid=42,start_ticks=99,cgroup='/unit/payload/id',nonce='12345678-1234-1234-1234-123456789abc')
  with contextlib.ExitStack() as s:
   op=s.enter_context(patch.object(c.os,'open',side_effect=[10,11,13,14,15]))
   s.enter_context(patch.object(c.os,'pidfd_open',return_value=12));s.enter_context(patch.object(c.os,'close'))
   s.enter_context(patch.object(c,'_fs_type',return_value=0x9fa0));s.enter_context(patch.object(c,'_held_identity',return_value=(99,e['cgroup'])))
   s.enter_context(patch.object(c,'_pid_live'));s.enter_context(patch.object(c,'_work_custody',return_value=(1,2)))
   def publish(work,nonce,before,record):before();return dict(ok=True)
   s.enter_context(patch.object(c,'_publish_token',side_effect=publish))
   c.publish_release_fd(e,e['nonce'],c.time.monotonic()+10,lambda:e,lambda _:None)
   for call in op.call_args_list:
    name,flags=call.args
    self.assertTrue(flags&os.O_DIRECTORY);self.assertTrue(flags&os.O_CLOEXEC)
    self.assertEqual(bool(flags&os.O_NOFOLLOW),name!='root')
