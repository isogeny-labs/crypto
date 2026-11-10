"""Independent deterministic source-consumer fixtures, not durable runtime tests."""
import importlib.util,json,hashlib,tempfile,pathlib,unittest,os,sys
P=pathlib.Path(__file__).resolve().parent
SOURCE=P.parent/'materializer-journal-reader-20261107/receipt_reader.py'
spec=importlib.util.spec_from_file_location('reviewed_reader',SOURCE);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class Adversarial(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.p=pathlib.Path(self.tmp.name);self.raw=b'{"pid":7,"nested":{"ok":true}}\n';self.r={'status':'passed','child_cleanup_storage':'inline-final-and-journal-prefix/v1','child_cleanup':[json.loads(self.raw)],'child_cleanup_journal':{'path':'child-cleanup.jsonl','bytes':len(self.raw),'records':1,'sha256':hashlib.sha256(self.raw).hexdigest()}}
 def tearDown(self):self.tmp.cleanup()
 def run_read(self,raw=None):
  (self.p/'receipt.json').write_text(json.dumps(self.r));(self.p/'child-cleanup.jsonl').write_bytes(self.raw if raw is None else raw);return m.verify(self.p)
 def reject(self,raw=None):
  with self.assertRaises((m.InvalidReceipt,OSError)):self.run_read(raw)
 def test_matching_pass_does_not_authenticate_owner(self):
  x=self.run_read();self.assertEqual(x['consistency'],'storage_complete');self.assertFalse(x['terminal_owner_authenticated']);self.assertFalse(x['terminal_established'])
 def test_storage_match_does_not_verify_error_semantics(self):
  self.r['errors']=['synthetic error'];x=self.run_read();self.assertEqual(x['consistency'],'storage_complete');self.assertFalse(x['cleanup_semantics_verified'])
 def test_boolean_integer_inline_collision(self):self.r['child_cleanup'][0]['nested']['ok']=1;self.reject()
 def test_numeric_float_integer_collision(self):self.r['child_cleanup'][0]['pid']=7.0;self.reject()
 def test_pass_hidden_tail(self):self.reject(self.raw+b'garbage')
 def test_failed_hidden_tail_allowed(self):
  self.r['status']='failed';self.r['child_cleanup'].append({'pid':8});self.assertEqual(self.run_read(self.raw+b'partial')['unreferenced_tail_bytes'],7)
 def test_incomplete_final_hidden_tail_rejected(self):self.r['status']='incomplete';self.reject(self.raw+b'partial')
 def test_truncated_prefix(self):self.reject(self.raw[:-1])
 def test_null_pointer(self):self.r['child_cleanup_journal']=None;self.reject()
 def test_receipt_duplicate_key(self):
  self.run_read();(self.p/'receipt.json').write_bytes(b'{"status":"passed","status":"failed"}')
  with self.assertRaises(m.InvalidReceipt):m.verify(self.p)
 def test_bool_count(self):self.r['child_cleanup_journal']['records']=True;self.reject()
 def test_length_bound(self):self.r['child_cleanup_journal']['bytes']=m.MAX_PREFIX+1;self.reject()
 def test_count_mismatch(self):self.r['child_cleanup_journal']['records']=2;self.reject()
 def test_path_traversal(self):self.r['child_cleanup_journal']['path']='../child-cleanup.jsonl';self.reject()
 def test_duplicate_nested_key(self):
  self.raw=b'{"nested":{"x":1,"x":2}}\n';self.r['child_cleanup_journal'].update(bytes=len(self.raw),sha256=hashlib.sha256(self.raw).hexdigest());self.reject()
 def test_nonfinite_exponent(self):
  self.raw=b'{"x":1e9999}\n';self.r['child_cleanup_journal'].update(bytes=len(self.raw),sha256=hashlib.sha256(self.raw).hexdigest());self.reject()
 def test_blank_record(self):
  self.raw=b'\n';self.r['child_cleanup_journal'].update(bytes=1,sha256=hashlib.sha256(self.raw).hexdigest());self.reject()
 def test_symlink_journal(self):
  self.run_read();(self.p/'target').write_bytes(self.raw);(self.p/'child-cleanup.jsonl').unlink();(self.p/'child-cleanup.jsonl').symlink_to('target')
  with self.assertRaises(OSError):m.verify(self.p)
 def test_symlink_parent(self):
  self.run_read();(self.p/'link').symlink_to(self.p,target_is_directory=True)
  with self.assertRaises(OSError):m.verify(self.p/'link')
 def test_fifo_rejected_without_blocking(self):
  self.run_read();(self.p/'child-cleanup.jsonl').unlink();os.mkfifo(self.p/'child-cleanup.jsonl')
  with self.assertRaises(m.InvalidReceipt):m.verify(self.p)
 def test_absent_pointer_never_terminal(self):
  self.r={'status':'passed','child_cleanup_storage':'journal-prefix/v1'};x=self.run_read();self.assertEqual(x['consistency'],'storage_incomplete');self.assertFalse(x['journal_verified']);self.assertFalse(x['terminal_established'])
if __name__=='__main__':unittest.main(verbosity=2)
