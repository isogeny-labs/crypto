import hashlib
import shutil
import unittest
from pathlib import Path

import generic_build_v2
import generic_build_v3
from oracle import InvalidEvidence
import test_generic_build_v2 as v2_tests


class GenericBuildV3Tests(unittest.TestCase):
    def tree(self, crate):
        root, metadata = v2_tests.GenericBuildV2Tests().tree(crate)
        self.addCleanup(shutil.rmtree, root)
        return root, metadata

    def test_every_admitted_crate_is_content_bound(self):
        for crate in sorted(generic_build_v3.ADMITTED_PATH_CRATES):
            with self.subTest(crate=crate):
                root, metadata = self.tree(crate)
                [dep] = generic_build_v3.source_manifest(root, metadata)['dependencies']
                self.assertEqual(dep['source'], 'path+' + crate)
                source = root / crate / 'src/main.rs'
                self.assertEqual(dep['files'], {'src/main.rs': hashlib.sha256(source.read_bytes()).hexdigest()})
                (root / crate / 'target/out').write_text('different build output')
                self.assertEqual(generic_build_v3.source_manifest(root, metadata)['dependencies'], [dep])
                source.write_text('fn changed() {}')
                self.assertNotEqual(generic_build_v3.source_manifest(root, metadata)['dependencies'], [dep])

    def test_v2_still_refuses_factor_base(self):
        root, metadata = self.tree('tools/factor-base')
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build_v2.source_manifest(root, metadata)

    def test_unadmitted_path_and_git_crates_remain_rejected(self):
        root, metadata = self.tree('tools/other')
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build_v3.source_manifest(root, metadata)
        metadata['packages'][0]['source'] = 'git+https://example.invalid/crate'
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build_v3.source_manifest(root, metadata)

    def test_admitted_directory_cannot_escape_root(self):
        root, metadata = self.tree('tools/factor-base')
        outside, _ = self.tree('outside')
        shutil.rmtree(root / 'tools/factor-base')
        (root / 'tools/factor-base').symlink_to(outside / 'outside', target_is_directory=True)
        with self.assertRaisesRegex(InvalidEvidence, 'explicit build admission'):
            generic_build_v3.source_manifest(root, metadata)

    def test_symlinked_admitted_file_is_rejected(self):
        root, metadata = self.tree('tools/factor-base')
        source = root / 'tools/factor-base/src/main.rs'
        source.unlink()
        source.symlink_to(root / 'src/lib.rs')
        with self.assertRaisesRegex(InvalidEvidence, 'symlinked dependency input'):
            generic_build_v3.source_manifest(root, metadata)


if __name__ == '__main__':
    unittest.main()
