import copy
import datetime
import json
import unittest
from launch import HERE, ROOT, capture, validate

class Admission(unittest.TestCase):
    def setUp(self):
        self.p = json.loads((HERE / 'comparison-protocol.draft.json').read_text())
        self.b = json.loads((HERE / 'source-binding.json').read_text())
        self.p['freeze'].update(status='frozen', protocol_sha256=capture.canonical_protocol(self.p))
        self.a = dict(status='admitted', protocol_sha256=capture.canonical_protocol(self.p),
                      source_commit=self.b['source_commit'], binary_sha256=self.b['binary_sha256'],
                      max_attempts=1, timeout_seconds=120, deadline_utc='2026-11-07T12:00:30Z')
        self.now = datetime.datetime(2026, 11, 7, 12, tzinfo=datetime.timezone.utc)
    def check(self):
        return validate(self.p, self.a, self.b['binary_sha256'], self.b, self.now)
    def test_published_closure_and_build_command(self):
        receipt = json.loads((ROOT / 'feature-native-monomial-progress/release-receipt.json').read_text())
        expected = receipt['argv'][:]
        expected[0] = 'rustc'
        expected[-1] = 'runner'
        for arm in self.p['arms']:
            self.assertEqual(arm['instrument']['build'], [expected])
        for path, digest in self.b['dependencies'].items():
            self.assertEqual(capture.sha(ROOT / path), digest, path)
    def test_synthetic_admission(self):
        self.assertEqual(self.check(), (self.a['protocol_sha256'], 30))
    def test_source_mismatch_before_launch(self):
        self.a['source_commit'] = '0' * 40
        with self.assertRaisesRegex(ValueError, 'source mismatch'): self.check()
    def test_old_instrument_even_with_consistent_receipt(self):
        for arm in self.p['arms']: arm['instrument']['binary_sha256'] = '0' * 64
        self.a['binary_sha256'] = '0' * 64
        self.p['freeze']['protocol_sha256'] = capture.canonical_protocol(self.p)
        self.a['protocol_sha256'] = capture.canonical_protocol(self.p)
        with self.assertRaisesRegex(ValueError, 'published binding'): self.check()
    def test_invalid_admissions(self):
        for field, value in [('status','pending'), ('max_attempts',True), ('max_attempts',2),
                             ('timeout_seconds',120.0), ('timeout_seconds',121),
                             ('protocol_sha256','0'*64), ('binary_sha256','0'*64),
                             ('deadline_utc','2026-11-07T12:00:00Z'),
                             ('deadline_utc','2026-11-07T12:00:30')]:
            original = copy.deepcopy(self.a)
            with self.subTest(field=field,value=value):
                self.a[field] = value
                with self.assertRaises(ValueError): self.check()
            self.a = original
    def test_draft_and_changed_protocol(self):
        self.p['freeze']['status'] = 'draft'
        with self.assertRaises(ValueError): self.check()
        self.p['freeze']['status'] = 'frozen'
        self.p['version'] += 1
        with self.assertRaises(ValueError): self.check()

if __name__ == '__main__': unittest.main()
