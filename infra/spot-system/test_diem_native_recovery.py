import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from diem_native_recovery import receipts


class NativeReceiptsTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.records = self.root/'instance-checkpoints-v1'
        self.records.mkdir()
        manifest = self.records/'manifest.json'
        manifest.write_text(json.dumps({'schema':1, 'source_sha256':'source', 'tasks':[[7,0],[7,1]]}))
        self.config = {'results':str(self.root), 'expected_records':2,
                       'manifest_sha256':hashlib.sha256(manifest.read_bytes()).hexdigest()}

    def record(self, index, **changes):
        payload = json.dumps({'k':7, 'idx':index})
        row = {'schema':1, 'source_sha256':'source', 'payload':payload,
               'payload_sha256':hashlib.sha256(payload.encode()).hexdigest()}
        row.update(changes)
        (self.records/('k07-i%02d.json' % index)).write_text(json.dumps(row))
        return payload

    def test_partial_receipts_survive_without_claiming_completion(self):
        self.record(0)
        (self.records/'.k07-i01.json.tmp-interrupted').write_text('partial')
        self.assertEqual(receipts(self.config), (1,False))

    def test_complete_requires_exact_final_list_and_hash(self):
        body = self.record(0)+'\n'+self.record(1)+'\n'
        final = self.root/'instances.jsonl'
        final.write_text(body)
        self.assertEqual(receipts(self.config), (2,False))
        (self.root/'FROZEN.sha256').write_text(hashlib.sha256(body.encode()).hexdigest()+'  instances.jsonl\n')
        self.assertEqual(receipts(self.config), (2,True))
        final.write_text(body+'{}\n')
        with self.assertRaises(ValueError):
            receipts(self.config)

    def test_corrupt_foreign_or_wrong_key_receipts_fail_closed(self):
        for changes in [{'source_sha256':'other'}, {'payload_sha256':'0'*64},
                        {'payload':'{"k":7,"idx":1}'}]:
            self.record(0, **changes)
            with self.assertRaises(ValueError):
                receipts(self.config)
        self.record(0)
        self.record(2)
        with self.assertRaises(ValueError):
            receipts(self.config)

    def test_manifest_change_is_not_a_new_run(self):
        (self.records/'manifest.json').write_text('{}')
        with self.assertRaises(ValueError):
            receipts(self.config)


if __name__ == '__main__':
    unittest.main()
