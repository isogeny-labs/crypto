import tempfile
import unittest
from pathlib import Path

import generic_build
import generic_build_v2
from oracle import InvalidEvidence


class GenericBuildV2Tests(unittest.TestCase):
    def tree(self, crate):
        root = Path(tempfile.mkdtemp()).resolve()
        for name in ('Cargo.toml', 'Cargo.lock', generic_build.WORKER, 'src/lib.rs'):
            (root / name).parent.mkdir(parents=True, exist_ok=True)
            (root / name).write_text('')
        (root / crate / 'src').mkdir(parents=True)
        (root / crate / 'src/main.rs').write_text('fn main() {}\n')
        (root / crate / 'target').mkdir()
        (root / crate / 'target/out').write_text('build output\n')
        return root, dict(packages=[dict(name='dep', version='0.1.0', source=None,
                                         manifest_path=str(root / crate / 'Cargo.toml'))])

    def test_admitted_path_crate_is_hashed_without_build_outputs(self):
        root, metadata = self.tree('tools/volcano-cli')
        [dep] = generic_build_v2.source_manifest(root, metadata)['dependencies']
        self.assertEqual(dep['source'], 'path+tools/volcano-cli')
        self.assertEqual(set(dep['files']), {'src/main.rs'})

    def test_other_path_crates_are_still_refused(self):
        root, metadata = self.tree('tools/other')
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build_v2.source_manifest(root, metadata)

    def test_v1_still_refuses_path_crates(self):
        root, metadata = self.tree('tools/volcano-cli')
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build.source_manifest(root, metadata)


if __name__ == '__main__':
    unittest.main()
