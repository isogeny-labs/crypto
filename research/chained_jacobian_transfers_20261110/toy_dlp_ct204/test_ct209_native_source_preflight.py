"""Local fixtures for the CT209 non-authoritative sparse checkout checker."""

import tempfile
import unittest
from pathlib import Path

from ct209_native_source_preflight import check


class TestSourcePreflight(unittest.TestCase):
    def fixture(self, temp: Path, code: str) -> None:
        (temp / "Cargo.toml").write_text('[[bin]]\nname = "driver"\npath = "src/bin/driver.rs"\n')
        path = temp / "src/bin/driver.rs"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(code)

    def test_missing_manifest_bin_detected(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "Cargo.toml").write_text('[[bin]]\nname="driver"\npath="src/bin/driver.rs"\n')
            bins, includes, missing = check(root)
            self.assertEqual((bins, includes), (1, 0))
            self.assertEqual(len(missing), 1)
            self.assertIn("missing registered source", missing[0])

    def test_missing_literal_include_detected_before_compilation(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.fixture(root, 'const TEXT: &str = include_str!("../../execution/PROTOCOL.md");\n')
            bins, includes, missing = check(root)
            self.assertEqual((bins, includes), (1, 1))
            self.assertEqual(len(missing), 1)
            self.assertIn("missing include input", missing[0])

    def test_valid_literal_include(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.fixture(root, 'const TEXT: &str = include_str!("../../execution/PROTOCOL.md");\n')
            (root / "execution").mkdir()
            (root / "execution/PROTOCOL.md").write_text("fixture")
            self.assertEqual(check(root), (1, 1, []))

    def test_line_comment_ignored(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.fixture(root, '// include_str!("../../missing.txt")\nfn main() {}\n')
            self.assertEqual(check(root), (1, 0, []))

    def test_include_bytes_and_build_rs(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.fixture(root, 'const X: &[u8] = include_bytes!("../../bytes.dat");\n')
            (root / "bytes.dat").write_bytes(b"hi")
            (root / "build.rs").write_text('include!("generated.rs");\n')
            bins, includes, missing = check(root)
            self.assertEqual((bins, includes), (1, 2))
            self.assertEqual(len(missing), 1)
            self.assertIn("build.rs", missing[0])


if __name__ == "__main__":
    unittest.main()
