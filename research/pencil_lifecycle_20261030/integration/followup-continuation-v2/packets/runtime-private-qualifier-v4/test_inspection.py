import ast
import errno
import os
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch
import supervise as s

class InspectionCaptureTests(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
  self.directory=Path(self.tmp.name)/'001-daemon-info';self.directory.mkdir(mode=0o700)
 def test_default_resolver_failure_regression(self):
  with patch.object(tempfile,'tempdir',None),patch.object(tempfile,'_get_default_tempdir',side_effect=FileNotFoundError('no usable global temp')) as default:
   with self.assertRaises(FileNotFoundError):tempfile.TemporaryFile(mode='w+b')
   default.reset_mock()
   with s.inspection_capture(self.directory) as f:f.write(b'{}');f.seek(0);self.assertEqual(f.read(),b'{}')
   default.assert_not_called()
 def test_poisoned_environment_is_ignored(self):
  with patch.dict(os.environ,{'TMPDIR':'/missing/ambient-a','TEMP':'/missing/ambient-b','TMP':'/missing/ambient-c'}),patch.object(tempfile,'gettempdir',side_effect=AssertionError('ambient temp forbidden')):
   with s.inspection_capture(self.directory) as f:self.assertEqual(os.fstat(f.fileno()).st_nlink,0)
 def test_binary_roundtrip_and_unlinked_lifecycle(self):
  with s.inspection_capture(self.directory) as f:
   fd=f.fileno();st=os.fstat(fd);self.assertTrue(stat.S_ISREG(st.st_mode));self.assertEqual(stat.S_IMODE(st.st_mode),0o600);self.assertEqual(st.st_nlink,0)
   f.write(b'\x00private-inspection\xff');f.seek(0);self.assertEqual(f.read(),b'\x00private-inspection\xff');self.assertEqual(list(self.directory.iterdir()),[])
  with self.assertRaises(OSError):os.fstat(fd)
  self.assertEqual(list(self.directory.iterdir()),[])
 def test_no_otmpfile_fallback_remains_unlinked(self):
  with patch.object(tempfile,'_O_TMPFILE_WORKS',False),patch.object(tempfile,'gettempdir',side_effect=AssertionError('ambient temp forbidden')):
   with s.inspection_capture(self.directory) as f:
    f.write(b'fallback');self.assertEqual(os.fstat(f.fileno()).st_nlink,0);self.assertEqual(list(self.directory.iterdir()),[])
 def test_missing_explicit_directory_has_no_fallback(self):
  with patch.object(tempfile,'gettempdir',side_effect=AssertionError('ambient temp forbidden')):
   with self.assertRaises(FileNotFoundError):s.inspection_capture(self.directory/'absent')
 def test_readonly_explicit_directory_has_no_fallback(self):
  with patch.object(tempfile,'gettempdir',side_effect=AssertionError('ambient temp forbidden')),patch.object(tempfile._os,'open',side_effect=OSError(errno.EROFS,'read-only explicit directory')):
   with self.assertRaises(OSError) as e:s.inspection_capture(self.directory)
   self.assertEqual(e.exception.errno,errno.EROFS)
 def test_exception_closes_inspection_file(self):
  with self.assertRaisesRegex(RuntimeError,'controlled failure'):
   with s.inspection_capture(self.directory) as f:
    fd=f.fileno();f.write(b'partial');raise RuntimeError('controlled failure')
  with self.assertRaises(OSError):os.fstat(fd)
  self.assertEqual(list(self.directory.iterdir()),[])
 def test_command_uses_helper_before_popen_static(self):
  tree=ast.parse(Path(s.__file__).read_text());command=next(n for n in ast.walk(tree) if isinstance(n,ast.FunctionDef) and n.name=='command')
  calls=[n for n in ast.walk(command) if isinstance(n,ast.Call)]
  alloc=[n for n in calls if isinstance(n.func,ast.Name) and n.func.id=='inspection_capture']
  launches=[n for n in calls if isinstance(n.func,ast.Attribute) and n.func.attr=='Popen']
  self.assertEqual(len(alloc),1);self.assertEqual(len(launches),1);self.assertEqual(ast.unparse(alloc[0].args[0]),'d');self.assertLess(alloc[0].lineno,launches[0].lineno)
  temporary=[n for n in ast.walk(tree) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='TemporaryFile']
  self.assertEqual(len(temporary),1);self.assertTrue(any(k.arg=='dir' and isinstance(k.value,ast.Name) and k.value.id=='directory' for k in temporary[0].keywords))

if __name__=='__main__':unittest.main(verbosity=2)
