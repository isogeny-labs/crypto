import unittest,tempfile,os
from pathlib import Path
from unittest.mock import patch
import supervise as s
class Independent(unittest.TestCase):
 def test_capture_resides_on_explicit_directory_device(self):
  with tempfile.TemporaryDirectory() as d:
   with patch.object(tempfile,'tempdir',None),patch.object(tempfile,'_get_default_tempdir',side_effect=AssertionError('default forbidden')):
    with s.inspection_capture(Path(d)) as f:
     self.assertEqual(os.fstat(f.fileno()).st_dev,Path(d).stat().st_dev)
     f.write(b'independent');f.seek(0);self.assertEqual(f.read(),b'independent')
     self.assertIsNone(tempfile.tempdir)
 def test_non_directory_target_fails_without_global_fallback(self):
  with tempfile.TemporaryDirectory() as d:
   target=Path(d)/'file';target.write_bytes(b'unchanged')
   with patch.object(tempfile,'gettempdir',side_effect=AssertionError('fallback forbidden')):
    with self.assertRaises(NotADirectoryError):s.inspection_capture(target)
   self.assertEqual(target.read_bytes(),b'unchanged')
