import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest
import zipfile


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("n83_pattern_audit", HERE / "audit_artifact.py")
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)


class ArtifactReplayTests(unittest.TestCase):
    def test_pinned_zip_rejects_forged_or_unsafe_members(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "artifact.zip"
            with zipfile.ZipFile(path, "w") as bundle:
                bundle.writestr("n83-pattern-results/summary.json", "{}")
            exact = hashlib.sha256(path.read_bytes()).hexdigest()
            self.assertEqual(len(AUDIT.zip_files(path, exact)), 1)
            with path.open("ab") as stream:
                stream.write(b"s SATISFIABLE\n")
            with self.assertRaisesRegex(ValueError, "exact provider ZIP"):
                AUDIT.zip_files(path, exact)
            with zipfile.ZipFile(path, "w") as bundle:
                bundle.writestr("../outside", "x")
            with self.assertRaisesRegex(ValueError, "unique safe ZIP"):
                AUDIT.zip_files(path, hashlib.sha256(path.read_bytes()).hexdigest())

    def test_solver_stream_needs_one_status_and_matching_exit(self):
        receipt = {"watchdog": False, "oom_kill_delta": 0, "exit_code": 15}
        valid = b"c conflicts                : 123\ns INDETERMINATE\n"
        self.assertEqual(AUDIT.interpret(valid, b"", receipt),
                         ("INDETERMINATE", 123))
        forged = valid + b"s SATISFIABLE\n"
        self.assertEqual(AUDIT.interpret(forged, b"", receipt),
                         ("MALFORMED", 123))
        self.assertEqual(AUDIT.interpret(valid, b"", dict(receipt, exit_code=10)),
                         ("MALFORMED", 123))


if __name__ == "__main__":
    unittest.main()
