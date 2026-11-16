import hashlib
from pathlib import Path
import subprocess
import tempfile
import unittest

from check_rust_syntax import check


class SyntaxGate(unittest.TestCase):
    def repository(self, root: Path, source: str) -> Path:
        subprocess.run(["git", "init", "-q", str(root)], check=True)
        (root / "src").mkdir()
        (root / "src/lib.rs").write_text(source)
        subprocess.run(["git", "add", "src/lib.rs"], cwd=root, check=True)
        subprocess.run([
            "git", "-c", "core.hooksPath=/dev/null", "-c", "user.name=CI fixture",
            "-c", "user.email=fixture@example.invalid", "commit", "-qm", "fixture",
        ], cwd=root, check=True)
        return root

    def test_valid_unformatted_source_passes_without_rewriting_original(self):
        with tempfile.TemporaryDirectory() as directory:
            root = self.repository(Path(directory), "pub fn value()->u32{7}\n")
            path = root / "src/lib.rs"
            before = hashlib.sha256(path.read_bytes()).hexdigest()
            code, count, _ = check(root)
            self.assertEqual((code, count), (0, 1))
            self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), before)
            subprocess.run(["git", "diff", "--quiet", "HEAD"], cwd=root, check=True)

    def test_dangling_nested_guard_from_spliced_versions_is_rejected(self):
        source = "pub fn bound(m: usize) -> Option<u64> {\nif m > 9 {\nif m > 15 { return None; }\nSome(1)\n}\n"
        with tempfile.TemporaryDirectory() as directory:
            root = self.repository(Path(directory), source)
            code, count, diagnostic = check(root)
            self.assertNotEqual(code, 0)
            self.assertEqual(count, 1)
            self.assertIn("src/lib.rs", diagnostic)
            self.assertEqual((root / "src/lib.rs").read_text(), source)

    def test_sparse_missing_working_copy_still_checks_the_committed_blob(self):
        with tempfile.TemporaryDirectory() as directory:
            root = self.repository(Path(directory), "pub fn broken( {\n")
            (root / "src/lib.rs").unlink()
            code, count, _ = check(root)
            self.assertNotEqual(code, 0)
            self.assertEqual(count, 1)
            self.assertFalse((root / "src/lib.rs").exists())

    def test_untracked_working_edits_do_not_replace_the_ci_commit(self):
        with tempfile.TemporaryDirectory() as directory:
            root = self.repository(Path(directory), "pub fn value() {}\n")
            (root / "src/lib.rs").write_text("pub fn broken( {\n")
            code, count, _ = check(root)
            self.assertEqual((code, count), (0, 1))
            self.assertEqual((root / "src/lib.rs").read_text(), "pub fn broken( {\n")


if __name__ == "__main__":
    unittest.main()
