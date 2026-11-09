import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import receipt_reader as reader


class ReaderTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.row = {'pid': 17, 'stopped': True}
        self.data = (json.dumps(self.row, separators=(',', ':')) + '\n').encode()
        self.receipt = {'status': 'failed', 'child_cleanup_storage': 'journal-prefix/v1',
                        'child_cleanup_journal': {'path': 'child-cleanup.jsonl',
                        'bytes': len(self.data), 'records': 1,
                        'sha256': hashlib.sha256(self.data).hexdigest()}}
    def tearDown(self):
        self.tmp.cleanup()
    def write(self, receipt=None, data=None):
        (self.root/'receipt.json').write_text(json.dumps(self.receipt if receipt is None else receipt))
        (self.root/'child-cleanup.jsonl').write_bytes(self.data if data is None else data)
    def reject(self, receipt=None, data=None):
        self.write(receipt, data)
        with self.assertRaises((reader.InvalidReceipt, OSError)):
            reader.verify(self.root)
    def final(self, status='passed'):
        value = copy.deepcopy(self.receipt)
        value.update(status=status, child_cleanup_storage='inline-final-and-journal-prefix/v1',
                     child_cleanup=[self.row])
        return value
    def test_prefix_status_never_terminal(self):
        for status in ['failed', 'passed', 'incomplete']:
            self.receipt['status'] = status
            self.write()
            result = reader.verify(self.root)
            self.assertEqual(result['consistency'], 'storage_incomplete')
            self.assertFalse(result['terminal_established'])
    def test_incomplete_final_rejected(self):
        self.reject(self.final('incomplete'), self.data+b'{}\n')
    def test_passed_final(self):
        self.write(self.final())
        result = reader.verify(self.root)
        self.assertEqual(result['consistency'], 'storage_complete')
        self.assertFalse(result['terminal_owner_authenticated'])
    def test_failed_final_extends_prefix(self):
        value = self.final('failed')
        value['child_cleanup'].append({'pid': 18})
        self.write(value, self.data+b'{partial')
        result = reader.verify(self.root)
        self.assertEqual(result['inline_records'], 2)
        self.assertEqual(result['unreferenced_tail_bytes'], 8)
    def test_passed_rejects_extra_inline_and_tail(self):
        self.reject(self.final(), self.data+b'{}\n')
        value = self.final()
        value['child_cleanup'].append({})
        self.reject(value)
    def test_failed_does_not_allow_mismatched_prefix(self):
        value = self.final('failed')
        value['child_cleanup'] = [{'pid': 19}]
        self.reject(value)
    def test_bool_is_not_integer_in_records(self):
        value = self.final()
        value['child_cleanup'] = [{'pid': 17, 'stopped': 1}]
        self.reject(value)
    def test_integer_metadata(self):
        for field in ['bytes', 'records']:
            for invalid in [True, -1, 1.0, '1', None, 10**15]:
                with self.subTest(field=field, invalid=invalid):
                    value = copy.deepcopy(self.receipt)
                    value['child_cleanup_journal'][field] = invalid
                    self.reject(value)
    def test_mismatches(self):
        for field, invalid in [('bytes', len(self.data)+1), ('bytes', len(self.data)-1),
                               ('records', 2), ('sha256', '0'*64), ('path', '../child-cleanup.jsonl')]:
            value = copy.deepcopy(self.receipt)
            value['child_cleanup_journal'][field] = invalid
            self.reject(value)
    def test_bad_jsonl(self):
        for data in [b'{}', b'\n', b'[]\n', b'{"a":1,"a":2}\n', b'{"x":NaN}\n',
                     b'{"x":1e999}\n', b'{}\n\n', b'\xff\n']:
            value = copy.deepcopy(self.receipt)
            value['child_cleanup_journal'].update(bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
            self.reject(value, data)
    def test_no_pointer_is_unacknowledged(self):
        self.write({'status': 'failed', 'child_cleanup_storage': 'journal-prefix/v1'})
        result = reader.verify(self.root)
        self.assertFalse(result['journal_verified'])
        self.assertIsNone(result['unreferenced_tail_bytes'])
        self.assertEqual(result['consistency'], 'storage_incomplete')
    def test_failed_final_no_pointer(self):
        self.write({'status': 'failed', 'child_cleanup_storage': 'inline-final-and-journal-prefix/v1'})
        self.assertEqual(reader.verify(self.root)['consistency'], 'storage_failed')
    def test_empty_committed_prefix(self):
        value = self.final()
        value['child_cleanup'] = []
        value['child_cleanup_journal'].update(bytes=0, records=0, sha256=hashlib.sha256(b'').hexdigest())
        self.write(value, b'')
        self.assertEqual(reader.verify(self.root)['consistency'], 'storage_complete')
    def test_legacy_weaker_result(self):
        self.write({'status': 'passed', 'child_cleanup': [self.row]})
        result = reader.verify(self.root)
        self.assertEqual(result['consistency'], 'legacy_inline_only')
        self.assertFalse(result['terminal_established'])
    def test_unknown_and_null_metadata(self):
        for field, value in [('status', 'done'), ('child_cleanup_storage', 'unknown'),
                             ('child_cleanup_journal', None), ('child_cleanup_storage', None)]:
            candidate = copy.deepcopy(self.receipt)
            candidate[field] = value
            self.reject(candidate)
    def test_symlink_and_directory_rejection(self):
        self.write()
        journal = self.root/'child-cleanup.jsonl'
        journal.rename(self.root/'other')
        journal.symlink_to(self.root/'other')
        with self.assertRaises(OSError): reader.verify(self.root)
        journal.unlink()
        journal.mkdir()
        with self.assertRaises((OSError, reader.InvalidReceipt)): reader.verify(self.root)
    def test_symlink_directory_and_receipt(self):
        self.write()
        link = self.root/'alias'
        link.symlink_to(self.root, target_is_directory=True)
        with self.assertRaises(OSError): reader.verify(link)
        receipt = self.root/'receipt.json'
        receipt.rename(self.root/'other')
        receipt.symlink_to(self.root/'other')
        with self.assertRaises(OSError): reader.verify(self.root)
    def test_limits(self):
        self.write()
        with patch.object(reader, 'MAX_RECEIPT', 1):
            with self.assertRaises(reader.InvalidReceipt): reader.verify(self.root)
        with patch.object(reader, 'MAX_JOURNAL', 1):
            with self.assertRaises(reader.InvalidReceipt): reader.verify(self.root)
    def test_duplicate_receipt_key(self):
        self.write()
        (self.root/'receipt.json').write_bytes(b'{"status":"passed","status":"failed"}')
        with self.assertRaises(reader.InvalidReceipt): reader.verify(self.root)

if __name__ == '__main__':
    unittest.main()
