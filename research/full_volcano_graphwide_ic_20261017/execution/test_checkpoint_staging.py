"""Publication file-handling checks; fixtures are not scientific measurements."""
import os
import errno
import json
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest import mock
from pathlib import Path

from checkpoint_staging import stage_checkpoint
from package_checkpoint import archive_tree, digest


class CheckpointStagingTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / "snapshot"
        self.session = self.source / "campaign/sessions/attempt-001"
        self.session.mkdir(parents=True)
        (self.source / "inventory.json").write_text('{"ok":true}\n')
        (self.source / "campaign/ledger.sqlite").write_bytes(b"ledger fixture")
        (self.source / "campaign/worker.json").write_text('{}\n')
        (self.session / "records.jsonl").write_text('{"fixture":true}\n')
        os.link(self.session / "records.jsonl", self.session / "same-bytes.jsonl")

    def test_linked_payloads_and_independent_metadata(self):
        target = self.root / "linked"
        result = stage_checkpoint(self.source, target, link_sessions=True)
        self.assertEqual(result["linked_files"], 2)
        for name in ("records.jsonl", "same-bytes.jsonl"):
            self.assertTrue(os.path.samefile(self.session / name,
                                            target / "campaign/sessions/attempt-001" / name))
        for name in ("ledger.sqlite", "worker.json"):
            self.assertFalse(os.path.samefile(self.source / "campaign" / name,
                                             target / "campaign" / name))
        (target / "campaign/ledger.sqlite").write_bytes(b"changed stage metadata")
        self.assertEqual((self.source / "campaign/ledger.sqlite").read_bytes(), b"ledger fixture")

    def test_copy_and_link_archives_identical_and_self_contained(self):
        archives = []
        for mode in (False, True):
            target = self.root / str(mode)
            stage_checkpoint(self.source, target, link_sessions=mode)
            archive = self.root / f"{mode}.tar.xz"
            archive_tree(target, archive)
            with tarfile.open(archive) as packed:
                for member in packed.getmembers():
                    self.assertFalse(member.issym() or member.islnk())
                    if member.isfile():
                        self.assertEqual(packed.extractfile(member).read(),
                                         (self.source / member.name).read_bytes())
            archives.append(archive)
        self.assertEqual(digest(archives[0]), digest(archives[1]))

    def test_reject_symlink_and_special_file(self):
        bad = self.session / "bad"
        bad.symlink_to(self.root)
        with self.assertRaises(ValueError):
            stage_checkpoint(self.source, self.root / "bad-link", link_sessions=True)
        bad.unlink()
        os.mkfifo(bad)
        with self.assertRaises(ValueError):
            stage_checkpoint(self.source, self.root / "bad-fifo", link_sessions=True)

    def test_cross_filesystem_link_does_not_silently_copy(self):
        import shutil
        with mock.patch('checkpoint_staging.os.link', side_effect=OSError(errno.EXDEV, 'cross-device fixture')):
            with self.assertRaises(shutil.Error):
                stage_checkpoint(self.source, self.root / "cross-device", link_sessions=True)

    def test_empty_sqlite_reader_caches_omitted_nonempty_wal_rejected(self):
        wal = self.source / 'campaign/ledger.sqlite-wal'
        shm = self.source / 'campaign/ledger.sqlite-shm'
        wal.touch()
        shm.write_bytes(b'reader shared-memory fixture')
        for mode in (False, True):
            target = self.root / f'cache-{mode}'
            result = stage_checkpoint(self.source, target, link_sessions=mode)
            self.assertEqual(set(result['omitted_empty_wal_reader_caches']),
                             {'ledger.sqlite-wal', 'ledger.sqlite-shm'})
            self.assertFalse((target / 'campaign/ledger.sqlite-wal').exists())
            self.assertFalse((target / 'campaign/ledger.sqlite-shm').exists())
        wal.write_bytes(b'uncheckpointed write fixture')
        with self.assertRaises(ValueError):
            stage_checkpoint(self.source, self.root / 'nonempty-wal', link_sessions=True)

    def test_reject_live_marker_existing_stage_and_recursive_stage(self):
        marker = self.source / "campaign/writer.lock"
        marker.touch()
        with self.assertRaises(ValueError):
            stage_checkpoint(self.source, self.root / "live", link_sessions=True)
        marker.unlink()
        with self.assertRaises(FileExistsError):
            stage_checkpoint(self.source, self.source, link_sessions=True)
        with self.assertRaises(ValueError):
            stage_checkpoint(self.source, self.source / "recursive", link_sessions=True)

    def test_packager_cli_copy_link_and_all_manifest_bytes(self):
        # Deliberately synthetic packaging fixture. No solver/measurement is run.
        execution = Path(__file__).resolve().parent
        repo = execution.parents[2]
        outcomes = {"fixture_only": 1}
        inventory = {"ok": True, "outcome_totals": outcomes,
                     "states": {"audited": 1}, "vertices_audited": 1,
                     "vertices_planned": 1, "strata_audited": 1, "strata_planned": 1}
        (self.source / "inventory.json").write_text(json.dumps(inventory))
        head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=repo, text=True).strip()
        (self.source / "campaign/campaign.json").write_text(json.dumps({
            "ecbench_sha256": "fixture-only", "source_head": head}))
        (self.source / "bin").mkdir()
        directories = {}
        for name in ("summary", "report", "graphs", "helpers"):
            directories[name] = self.root / name
            directories[name].mkdir()
        (directories["graphs"] / "fixture.sqlite").write_bytes(b"not a research graph")
        (self.source / "campaign/queue.jsonl").write_text(json.dumps({
            "graph": "fixture", "input": {"graph_sha256": digest(directories['graphs'] / 'fixture.sqlite')}}) + '\n')
        (directories["summary"] / "summary.json").write_text(json.dumps({
            "outcome_totals": outcomes, "binary_sha256": "fixture-only", "measured_records": 1}))
        (directories["report"] / "README.txt").write_text("Packaging fixture only.\n")
        for name in ("volcano-ic-checkpoint", "volcano-ic-summarize-v2"):
            (directories["helpers"] / name).write_bytes(b"fixture, not executable")
        note = self.root / 'note.md'
        note.write_text('Synthetic publication validation; not scientific evidence.\n')
        archives = []
        for mode in (False, True):
            target = self.root / f"cli-{mode}"
            archive = self.root / f"cli-{mode}.tar.xz"
            argv = [sys.executable, str(execution / 'package_survey_checkpoint.py'),
                    str(repo), str(self.source), *(str(directories[name]) for name in
                        ('summary', 'report', 'graphs', 'helpers')),
                    str(target), str(archive), str(note)]
            if mode:
                argv.append('--link-sessions')
            result = json.loads(subprocess.check_output(argv, text=True))
            self.assertEqual(result['staging']['mode'], 'link-sessions' if mode else 'copy')
            with tarfile.open(archive) as packed:
                manifest = packed.extractfile('MANIFEST.sha256').read().decode()
                import hashlib
                for line in manifest.splitlines():
                    sha, name = line.split('  ', 1)
                    self.assertEqual(hashlib.sha256(packed.extractfile(name).read()).hexdigest(), sha, name)
                    self.assertTrue(packed.getmember(name).isfile())
                self.assertEqual(packed.extractfile('DELIVERY_NOTE.md').read(), note.read_bytes())
            archives.append(archive)
        self.assertEqual(digest(archives[0]), digest(archives[1]))


if __name__ == "__main__":
    unittest.main()
