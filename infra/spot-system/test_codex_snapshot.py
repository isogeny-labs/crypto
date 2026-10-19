import json
from contextlib import closing
import os
from pathlib import Path
import sqlite3
import tempfile
import unittest

import codex_snapshot as snapshot


class SnapshotTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name).resolve()
        self.home = self.base/'source'
        self.home.mkdir()
        self.root = self.base/'durable'
        (self.home/'sessions').mkdir()
        (self.home/'sessions'/'session.jsonl').write_text('{"event":"saved"}\n')
        (self.home/'auth.json').write_text('private test fixture')

    def test_committed_wal_data_survives_restore_without_copying_live_wal(self):
        db = sqlite3.connect(self.home/'state_5.sqlite')
        self.addCleanup(db.close)
        db.execute('PRAGMA journal_mode=WAL')
        db.execute('CREATE TABLE threads (id TEXT)')
        db.execute("INSERT INTO threads VALUES ('saved-thread')")
        db.commit()
        self.assertTrue((self.home/'state_5.sqlite-wal').exists())
        snapshot.snapshot(self.home,self.root,[{'id':'saved-thread'}])
        target = self.base/'restored'
        self.assertEqual(snapshot.restore(self.root,target),[{'id':'saved-thread'}])
        with closing(sqlite3.connect(target/'state_5.sqlite')) as restored:
            self.assertEqual(restored.execute('SELECT id FROM threads').fetchall(),[('saved-thread',)])
        self.assertFalse((target/'auth.json').exists())

    def test_authentication_requires_explicit_opt_in(self):
        result = snapshot.snapshot(self.home,self.root,[],include_auth=True)
        self.assertTrue(result['auth_included'])
        snapshot.restore(self.root,self.base/'restored')
        self.assertEqual((self.base/'restored'/'auth.json').read_text(),'private test fixture')

    def test_corrupt_generation_is_rejected_before_destination_changes(self):
        snapshot.snapshot(self.home,self.root,[])
        directory,_ = snapshot.load_generation(self.root)
        (directory/'sessions'/'session.jsonl').write_text('corrupt')
        with self.assertRaises(ValueError):
            snapshot.restore(self.root,self.base/'restored')
        self.assertFalse((self.base/'restored').exists())

    def test_new_generations_preserve_previous_bytes_and_file_modes(self):
        snapshot.snapshot(self.home,self.root,[])
        first,_ = snapshot.load_generation(self.root)
        file = self.home/'sessions'/'session.jsonl'
        file.chmod(0o700)
        snapshot.snapshot(self.home,self.root,[])
        self.assertEqual((first/'sessions'/'session.jsonl').stat().st_mode & 0o777,0o600)
        file.write_text('new bytes')
        snapshot.snapshot(self.home,self.root,[])
        self.assertEqual((first/'sessions'/'session.jsonl').read_text(),'{"event":"saved"}\n')

    def test_internal_package_symlink_is_portable_and_external_link_is_rejected(self):
        packages = self.home/'packages'
        packages.mkdir()
        (packages/'release').mkdir()
        (packages/'release'/'codex').write_text('binary fixture')
        (packages/'current').symlink_to(packages/'release')
        snapshot.snapshot(self.home,self.root,[])
        target = self.base/'restored'
        snapshot.restore(self.root,target)
        self.assertEqual((target/'packages'/'current'/'codex').read_text(),'binary fixture')
        pointer = (self.root/'current.json').read_bytes()
        (self.home/'sessions'/'escape').symlink_to('/etc/passwd')
        with self.assertRaises(ValueError):
            snapshot.snapshot(self.home,self.root,[])
        self.assertEqual((self.root/'current.json').read_bytes(),pointer)

    def test_existing_state_is_never_overwritten(self):
        snapshot.snapshot(self.home,self.root,[])
        with self.assertRaises(ValueError):
            snapshot.restore(self.root,self.home)

    def test_retention_preserves_current_and_two_prior_generations(self):
        first=snapshot.snapshot(self.home,self.root,[])['generation']
        for value in range(3):
            (self.home/'sessions'/'session.jsonl').write_text(str(value))
            snapshot.snapshot(self.home,self.root,[])
        self.assertFalse((self.root/first).exists())
        self.assertEqual(len(list(self.root.glob('gen-*'))),3)
        target=self.base/'restored'
        snapshot.restore(self.root,target)
        self.assertEqual((target/'sessions'/'session.jsonl').read_text(),'2')


if __name__ == '__main__':
    unittest.main()
