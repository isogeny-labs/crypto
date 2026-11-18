import hashlib
from pathlib import Path
import tempfile
import unittest

from panel_supervisor import NATURAL, PLAN, verify_host_hashes


class CachePanelSupervisorTests(unittest.TestCase):
    def test_frozen_pairing_has_controls_and_off_on_on_off_natural_order(self):
        self.assertEqual(len(PLAN), 12)
        self.assertEqual(len({arm[0] for arm in PLAN}), 12)
        self.assertEqual(
            [(cache, mode, shape, fixture) for _, cache, mode, shape, fixture in NATURAL],
            [('off', 'natural', 'UUDD', 0),
             ('on', 'natural', 'UUDD', 0),
             ('on', 'natural', 'UUDD', 0),
             ('off', 'natural', 'UUDD', 0)],
        )
        self.assertEqual(
            {(cache, mode, shape) for _, cache, mode, shape, _ in PLAN[:8]},
            {(cache, mode, shape)
             for cache in ('off', 'on')
             for mode in ('preflight', 'positive')
             for shape in ('UUUU', 'UUDD')},
        )

    def test_raw_manifest_rejects_tampering_and_omissions(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            payload = root / 'receipt.json'
            payload.write_bytes(b'{"status":"PASS"}\n')
            digest = hashlib.sha256(payload.read_bytes()).hexdigest()
            (root / 'raw.sha256').write_text(f'{digest}  receipt.json\n')
            verify_host_hashes(root)
            payload.write_bytes(b'{"status":"FAIL"}\n')
            with self.assertRaisesRegex(RuntimeError, 'mismatch'):
                verify_host_hashes(root)
            payload.write_bytes(b'{"status":"PASS"}\n')
            (root / 'unlisted.txt').write_text('extra')
            with self.assertRaisesRegex(RuntimeError, 'omits'):
                verify_host_hashes(root)


if __name__ == '__main__':
    unittest.main()
