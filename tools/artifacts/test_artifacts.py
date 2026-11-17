"""End-to-end tests for tools/artifacts against a file:// remote in a scratch repository.

    python3 -m unittest tools/artifacts/test_artifacts.py
"""

import base64
import gzip
import shlex
import shutil
from unittest import mock
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

TOOL = Path(__file__).resolve().parent / "artifacts.py"
sys.path.insert(0, str(TOOL.parent))
import artifacts  # noqa: E402
import wire_workflows  # noqa: E402


def sh(cwd, *argv, check=True, env=None):
    return subprocess.run(argv, cwd=cwd, capture_output=True, text=True, check=check, env=env)


class RoundTrip(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name).resolve()
        self.repo, self.remote, self.cache = base / "repo", base / "remote", base / "cache"
        self.repo.mkdir()
        self.env = {**os.environ, "ARTIFACTS_REMOTE": f"file://{self.remote}", "ARTIFACTS_CACHE": str(self.cache),
                    "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t", "GIT_COMMITTER_NAME": "t",
                    "GIT_COMMITTER_EMAIL": "t@t"}
        sh(self.repo, "git", "init", "-q", "-b", "main")
        runs = self.repo / "research" / "r1" / "runs"
        runs.mkdir(parents=True)
        self.big = os.urandom(300_000)
        (runs / "a.jsonl").write_bytes(self.big)
        (runs / "weird [1]*.log").write_bytes(os.urandom(70_000))
        (runs / "small.jsonl").write_bytes(b"{}\n")        # under data_min_bytes: stays
        (self.repo / "research" / "r1" / "README.md").write_text("x" * 2_000_000)  # code ext: stays
        (self.repo / "research" / "r1" / "blob.unknown").write_bytes(os.urandom(1_100_000))  # > max_bytes
        self.git("add", "-A")
        self.git("commit", "-qm", "seed")

    def tearDown(self):
        self.tmp.cleanup()

    def git(self, *a):
        return sh(self.repo, "git", *a, env=self.env).stdout

    def art(self, *a, check=True, cwd=None):
        return sh(cwd or self.repo, sys.executable, str(TOOL), *a, check=check, env=self.env)

    def test_s3_default_recovers_exact_git_bytes_without_aws(self):
        self.art("push", "research")
        self.git("add", "-A")
        self.git("commit", "-qm", "move data to pointers")
        self.env["ARTIFACTS_REMOTE"] = "s3://disabled-bucket/store"
        self.env["ARTIFACTS_AWS"] = "/does-not-exist/aws"
        self.env["ARTIFACTS_CACHE"] = str(self.repo / "cold-cache")
        path = "research/r1/runs/a.jsonl"
        (self.repo / path).unlink()
        self.art("pull", path)
        self.assertEqual((self.repo / path).read_bytes(), self.big)

    def test_git_source_refuses_same_size_wrong_historical_bytes(self):
        self.art("push", "research")
        self.git("add", "-A")
        self.git("commit", "-qm", "move data to pointers")
        manifest = self.repo / "research/r1/runs/.artifacts.json"
        data = json.loads(manifest.read_text())
        data["files"]["a.jsonl"]["sha256"] = hashlib.sha256(b"z" * len(self.big)).hexdigest()
        manifest.write_text(json.dumps(data))
        (self.repo / "research/r1/runs/a.jsonl").unlink()
        self.env["ARTIFACTS_CACHE"] = str(self.repo / "cold-cache")
        result = self.art("pull", "--source", "git", "research/r1/runs/a.jsonl", check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no Git-history object matches", result.stderr)
        self.assertIn("S3 was not contacted", result.stderr)
        self.assertFalse((self.repo / "research/r1/runs/a.jsonl").exists())

    def test_git_source_finds_older_version_and_reuses_verified_cache(self):
        path = self.repo / "research/r1/runs/a.jsonl"
        path.write_bytes(b"z" * len(self.big))
        self.git("add", "-A")
        self.git("commit", "-qm", "new same-size version")
        cfg = artifacts.Config(root=self.repo, remote="s3://disabled/store", endpoint=None,
                               read_url=None, cache=self.repo / "cold-cache", aws="/no/aws")
        digest = hashlib.sha256(self.big).hexdigest()
        reader = artifacts.GitHistory(cfg, path, len(self.big))
        cached = artifacts.ensure_cached(cfg, reader, digest, len(self.big))
        self.assertEqual(cached.read_bytes(), self.big)
        with mock.patch.object(reader, "get", side_effect=AssertionError("cache must avoid Git")):
            self.assertEqual(artifacts.ensure_cached(cfg, reader, digest, len(self.big)), cached)

    def test_git_source_follows_history_hidden_by_merge_simplification(self):
        self.git("switch", "-qc", "historical-data")
        path = self.repo / "research/side-only.jsonl"
        path.write_bytes(self.big)
        self.git("add", "-A")
        self.git("commit", "-qm", "side branch artifact")
        self.git("switch", "-q", "main")
        self.git("merge", "-q", "-s", "ours", "historical-data", "-m", "retain ancestry without data")
        self.assertEqual(self.git("rev-list", "HEAD", "--", "research/side-only.jsonl"), "")
        cfg = artifacts.Config(root=self.repo, remote="s3://disabled/store", endpoint=None,
                               read_url=None, cache=self.repo / "cold-cache", aws="/no/aws")
        digest = hashlib.sha256(self.big).hexdigest()
        cached = artifacts.ensure_cached(cfg, artifacts.GitHistory(cfg, path, len(self.big)), digest, len(self.big))
        self.assertEqual(cached.read_bytes(), self.big)

    def test_git_recovery_bundle_and_blob_still_require_manifest_digest(self):
        cfg = artifacts.Config(root=self.repo, remote="s3://disabled/store", endpoint=None,
                               read_url=None, cache=self.repo / "cold-cache", aws="/no/aws")
        source = self.repo / "missing-at-original-path.jsonl"
        recovery = self.repo / ".artifacts/recovery"
        recovery.mkdir(parents=True)
        digest = hashlib.sha256(self.big).hexdigest()
        oid = self.git("rev-parse", "HEAD:research/r1/runs/a.jsonl").strip()
        for entry, payload, valid in [
            ({"gzip": "data.gz"}, self.big, True),
            ({"git_blob": oid}, None, True),
            ({"gzip": "data.gz"}, b"z" * len(self.big), False),
            ({"gzip": "data.gz"}, self.big + b"extra", False),
            ({"gzip": "../outside.gz"}, self.big, False),
        ]:
            with self.subTest(entry=entry, valid=valid):
                shutil.rmtree(cfg.cache, ignore_errors=True)
                (recovery / "index.json").write_text(json.dumps({"schema": "artifacts.recovery/v1", "objects": {digest: entry}}))
                if payload is not None:
                    (recovery / "data.gz").write_bytes(gzip.compress(payload, mtime=0))
                reader = artifacts.GitHistory(cfg, source, len(self.big))
                if valid:
                    cached = artifacts.ensure_cached(cfg, reader, digest, len(self.big))
                    self.assertEqual(cached.read_bytes(), self.big)
                else:
                    with mock.patch("sys.stderr"), self.assertRaises(SystemExit):
                        artifacts.ensure_cached(cfg, reader, digest, len(self.big))
                    self.assertFalse(artifacts.cache_path(cfg, digest).exists())

    def test_push_refuses_unmerged_index_without_changing_stages(self):
        path = "research/r1/runs/a.jsonl"
        oids = []
        for payload in (self.big, self.big + b"ours\n", self.big + b"theirs\n"):
            result = subprocess.run(["git", "hash-object", "-w", "--stdin"], cwd=self.repo,
                                    input=payload, capture_output=True, check=True, env=self.env)
            oids.append(result.stdout.decode().strip())
        self.assertEqual(len(set(oids)), 3)
        self.git("update-index", "--force-remove", path)
        rows = "".join(f"100644 {oid} {stage}\t{path}\n" for stage, oid in enumerate(oids, 1))
        subprocess.run(["git", "update-index", "--index-info"], cwd=self.repo,
                       input=rows, text=True, check=True, env=self.env)
        original_index = self.git("ls-files", "--stage")
        for arguments in (("push", path), ("push", "research"), ("push", "--dry-run", path)):
            with self.subTest(arguments=arguments):
                result = self.art(*arguments, check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("unresolved Git index conflicts", result.stderr)
                self.assertEqual(self.git("ls-files", "--stage"), original_index)
                self.assertEqual((self.repo / path).read_bytes(), self.big)
                self.assertFalse(self.remote.exists())
                self.assertFalse((self.repo / "research/r1/runs/.artifacts.json").exists())

    def test_pull_preserves_tracked_successor_and_unstaged_edits(self):
        self.art("push", "research")
        path = self.repo / "research/r1/runs/a.jsonl"
        successor = b"current authoritative tracked successor\n"
        path.write_bytes(successor)
        self.git("add", "-f", "research/r1/runs/a.jsonl")
        # Siblings still hydrate, while the staged successor stays authoritative.
        sibling = self.repo / "research/r1/blob.unknown"
        sibling.unlink()
        self.art("pull", "research")
        self.assertEqual(path.read_bytes(), successor)
        self.assertTrue(sibling.is_file())
        edited = successor + b"local edit\n"
        path.write_bytes(edited)
        result = self.art("pull", "research/r1/runs/a.jsonl")
        self.assertEqual(path.read_bytes(), edited)
        self.assertIn("preserved 1 tracked files", result.stderr)
        # An omitted tracked path is also authoritative, not a hydration target.
        path.unlink()
        self.art("pull", "research/r1/runs/a.jsonl")
        self.assertFalse(path.exists())

    def test_pull_preserves_case_and_unicode_aliases_of_tracked_paths(self):
        self.art("push", "research/r1/runs/a.jsonl")
        directory = self.repo / "research/r1/runs"
        successor = b"current tracked successor\n"
        for source_name, alias in (("a.jsonl", "A.JSONL"), ("caf\u00e9.jsonl", "cafe\u0301.JSONL")):
            with self.subTest(alias=alias):
                source = directory / source_name
                source.write_bytes(successor)
                self.git("add", "-f", f"research/r1/runs/{source_name}")
                manifest = {"schema": artifacts.MANIFEST_SCHEMA, "files": {
                    alias: {"sha256": hashlib.sha256(self.big).hexdigest(), "bytes": len(self.big)}}}
                (directory / ".artifacts.json").write_text(json.dumps(manifest))
                self.git("add", "research/r1/runs/.artifacts.json")
                self.art("pull", "research")
                self.assertEqual(source.read_bytes(), successor)
                # Source ownership remains authoritative while sparse/omitted.
                source.unlink()
                self.art("pull", "research")
                self.assertFalse(source.exists())
                self.assertFalse((directory / alias).exists())

    def test_manifest_cannot_replace_linked_worktree_control_files(self):
        self.art("push", "research/r1/runs/a.jsonl")
        self.git("commit", "-qm", "store fixture")
        linked = Path(self.tmp.name) / "linked"
        self.git("worktree", "add", "-b", "linked-control", str(linked), "HEAD")
        original = (linked / ".git").read_bytes()
        for name in (".git", ".GIT", ".git ", ".git.", ".git::$DATA",
                     ".artifacts.json", ".gitignore", ".gitattributes"):
            with self.subTest(name=name):
                manifest = {"schema": artifacts.MANIFEST_SCHEMA, "files": {
                    name: {"sha256": hashlib.sha256(self.big).hexdigest(), "bytes": len(self.big)}}}
                (linked / ".artifacts.json").write_text(json.dumps(manifest))
                sh(linked, "git", "add", ".artifacts.json", env=self.env)
                result = self.art("pull", cwd=linked, check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("invalid manifest entry", result.stderr)
                self.assertEqual((linked / ".git").read_bytes(), original)

    def test_manifest_paths_cannot_escape_their_directory(self):
        self.art("push", "research/r1/runs/a.jsonl")
        target = Path(self.tmp.name) / "escaped.jsonl"
        for name in ("../escaped.jsonl", "nested/file.jsonl", "..\\escaped.jsonl", "", ".", ".."):
            with self.subTest(name=name):
                manifest = {"schema": artifacts.MANIFEST_SCHEMA, "files": {
                    name: {"sha256": hashlib.sha256(self.big).hexdigest(), "bytes": len(self.big)}}}
                (self.repo / ".artifacts.json").write_text(json.dumps(manifest))
                self.git("add", ".artifacts.json")
                result = self.art("pull", check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("invalid manifest entry", result.stderr)
                self.assertFalse(target.exists())

    def test_manifest_metadata_is_validated_before_any_pull(self):
        for entry in ({"sha256": "0" * 64, "bytes": True},
                      {"sha256": "0" * 64, "bytes": -1},
                      {"sha256": "BAD", "bytes": 1}, None):
            with self.subTest(entry=entry):
                (self.repo / ".artifacts.json").write_text(json.dumps(
                    {"schema": artifacts.MANIFEST_SCHEMA, "files": {"data.jsonl": entry}}))
                self.git("add", ".artifacts.json")
                result = self.art("pull", check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("invalid manifest entry", result.stderr)

    def test_manifest_directory_symlink_cannot_redirect_hydration(self):
        self.art("push", "research/r1/runs/a.jsonl")
        directory = self.repo / "research/r1/runs"
        outside = Path(self.tmp.name) / "outside"
        directory.rename(outside)
        directory.symlink_to(outside, target_is_directory=True)
        target = outside / "a.jsonl"
        target.unlink()
        result = self.art("pull", check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("outside the repository", result.stderr)
        self.assertFalse(target.exists())

    def test_push_commit_clone_pull_verify(self):
        plan = self.art("plan").stdout
        self.assertIn("research/r1/runs/a.jsonl", plan)
        self.assertIn("research/r1/blob.unknown", plan)
        self.assertNotIn("small.jsonl", plan)
        self.assertNotIn("README.md", plan)

        self.art("push", "research")
        self.git("commit", "-qm", "move data to the store")
        tracked = self.git("ls-files").split("\n")
        self.assertNotIn("research/r1/runs/a.jsonl", tracked)
        self.assertIn("research/r1/runs/.artifacts.json", tracked)
        self.assertIn("research/r1/runs/small.jsonl", tracked)
        self.assertEqual(self.git("status", "--porcelain"), "")  # hydrated files are ignored

        digest = hashlib.sha256(self.big).hexdigest()
        self.assertTrue((self.remote / "sha256" / digest[:2] / digest).exists())
        m = json.loads((self.repo / "research/r1/runs/.artifacts.json").read_text())
        self.assertEqual(m["files"]["a.jsonl"], {"sha256": digest, "bytes": len(self.big)})

        # A fresh clone has pointers only; pull a subtree, then everything.
        clone = Path(self.tmp.name) / "clone"
        sh(self.tmp.name, "git", "clone", "-q", str(self.repo), str(clone))
        self.assertFalse((clone / "research/r1/runs/a.jsonl").exists())
        self.art("pull", "research/r1/runs/a.jsonl", cwd=clone)
        self.assertEqual((clone / "research/r1/runs/a.jsonl").read_bytes(), self.big)
        self.assertFalse((clone / "research/r1/blob.unknown").exists())
        self.art("pull", cwd=clone)
        self.assertTrue((clone / "research/r1/runs/weird [1]*.log").exists())
        self.assertEqual(self.art("verify", "--remote", cwd=clone).returncode, 0)
        self.assertEqual(sh(clone, "git", "status", "--porcelain").stdout, "")

        # Editing a hydrated file in place is caught, and never corrupts the cache.
        (clone / "research/r1/runs/a.jsonl").write_bytes(b"tampered")
        self.assertEqual(self.art("verify", cwd=clone, check=False).returncode, 1)
        self.art("pull", cwd=clone)
        self.assertEqual((clone / "research/r1/runs/a.jsonl").read_bytes(), self.big)

    def test_corrupt_remote_object_is_refused(self):
        self.art("push", "research/r1/runs/a.jsonl")
        digest = hashlib.sha256(self.big).hexdigest()
        obj = self.remote / "sha256" / digest[:2] / digest
        obj.write_bytes(b"x" * len(self.big))
        os.remove(self.repo / "research/r1/runs/a.jsonl")
        for p in (self.cache / "sha256").rglob("*"):
            if p.is_file():
                p.chmod(0o644)
                p.unlink()
        r = self.art("pull", check=False)
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("refused", r.stderr)
        self.assertFalse((self.repo / "research/r1/runs/a.jsonl").exists())

    def test_push_merges_existing_manifest_and_gitignore(self):
        (self.repo / "research/r1/runs/.gitignore").write_text("/tmp/\n")
        self.art("push", "research/r1/runs/a.jsonl")
        self.art("push", "research/r1/runs")
        gi = (self.repo / "research/r1/runs/.gitignore").read_text()
        self.assertTrue(gi.startswith("/tmp/\n"))
        self.assertEqual(gi.count(artifacts.BEGIN), 1)
        self.assertIn("/weird \\[1\\]\\*.log", gi)
        m = json.loads((self.repo / "research/r1/runs/.artifacts.json").read_text())
        self.assertEqual(sorted(m["files"]), ["a.jsonl", "weird [1]*.log"])

    def test_push_from_sparse_checkout_reads_blobs(self):
        (self.repo / "research/r1/runs/.gitignore").write_text("/keep-me/\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "gitignore")
        self.git("sparse-checkout", "set", "--no-cone", "/*", "!/research/")
        self.assertFalse((self.repo / "research").exists())
        self.art("push", "research")
        self.git("commit", "-qm", "move from sparse")
        self.git("sparse-checkout", "disable")
        gi = (self.repo / "research/r1/runs/.gitignore").read_text()
        self.assertTrue(gi.startswith("/keep-me/\n"))
        self.assertNotIn("research/r1/runs/a.jsonl", self.git("ls-files"))
        self.art("pull")
        self.assertEqual((self.repo / "research/r1/runs/a.jsonl").read_bytes(), self.big)
        self.assertEqual(self.git("status", "--porcelain"), "")

    def install_content_filter(self):
        # Public reversible fixture exercises Git's filter contract, not AES.
        script = Path(self.tmp.name) / "content_filter.py"
        script.write_text("import sys\nraw=sys.stdin.buffer.read()\n"
                          "if sys.argv[1]=='clean': out=b'FILTERED:'+raw.hex().encode()\n"
                          "else: out=bytes.fromhex(raw[9:].decode()) if raw.startswith(b'FILTERED:') else raw\n"
                          "sys.stdout.buffer.write(out)\n")
        command = shlex.quote(sys.executable) + " " + shlex.quote(str(script))
        self.git("config", "filter.fixture.clean", command + " clean")
        self.git("config", "filter.fixture.smudge", command + " smudge")
        self.git("config", "filter.fixture.required", "true")
        (self.repo / '.gitattributes').write_text('research/** filter=fixture\n')
        self.git('add', '.gitattributes')
        self.git('add', '--renormalize', 'research')
        self.git('commit', '-qm', 'public filter fixture')

    def test_sparse_offload_uses_filtered_plaintext_size_and_preserves_ignore(self):
        (self.repo / 'research/r1/runs/.gitignore').write_text('/keep-me/\n')
        self.git('add', '-A')
        self.git('commit', '-qm', 'ignore fixture')
        self.install_content_filter()
        self.git('sparse-checkout', 'set', '--no-cone', '/*', '!/research/')
        self.art('push', 'research/r1/runs/a.jsonl')
        digest = hashlib.sha256(self.big).hexdigest()
        self.assertEqual((self.remote/'sha256'/digest[:2]/digest).read_bytes(), self.big)
        manifest=json.loads((self.repo/'research/r1/runs/.artifacts.json').read_text())
        self.assertEqual(manifest['files']['a.jsonl'], {'sha256':digest,'bytes':len(self.big)})
        self.assertTrue((self.repo/'research/r1/runs/.gitignore').read_text().startswith('/keep-me/\n'))

    def test_required_filter_failure_cannot_upload_raw_staged_bytes(self):
        self.install_content_filter()
        self.git('sparse-checkout', 'set', '--no-cone', '/*', '!/research/')
        self.git('config', 'filter.fixture.smudge', 'false')
        result=self.art('push', 'research/r1/runs/a.jsonl', check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.repo/'research/r1/runs/.artifacts.json').exists())
        self.assertFalse(list(self.remote.rglob('*')) if self.remote.exists() else [])
        self.assertFalse(list((self.cache/'tmp').glob('*')))

    def test_staged_metadata_filter_failure_is_not_absence(self):
        (self.repo / 'research/r1/runs/.gitignore').write_text('/keep-me/\n')
        self.git('add', '-A')
        self.git('commit', '-qm', 'ignore fixture')
        self.install_content_filter()
        self.git('sparse-checkout', 'set', '--no-cone', '/*', '!/research/')
        self.git('config', 'filter.fixture.smudge', 'false')
        with mock.patch.object(artifacts, 'repo_root', return_value=self.repo):
            with self.assertRaises(subprocess.CalledProcessError):
                artifacts.read_text_or_index(self.repo/'research/r1/runs/.gitignore')

    def test_check_asks_git_for_full_object_ids(self):
        """A blobless sparse checkout can fetch a missing blob by full id, not by the
        7-character one `git diff --raw` prints by default."""
        shim = Path(self.tmp.name) / "shim"
        shim.mkdir()
        log = shim / "git.log"
        real = subprocess.run(["which", "git"], capture_output=True, text=True, check=True).stdout.strip()
        (shim / "git").write_text(f'#!/bin/sh\necho "$@" >> {log}\nexec {real} "$@"\n')
        (shim / "git").chmod(0o755)
        base = self.git("rev-parse", "HEAD").strip()
        (self.repo / "small.txt").write_text("x\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "small change")
        env = {**self.env, "PATH": f"{shim}{os.pathsep}{os.environ['PATH']}"}
        r = sh(self.repo, sys.executable, str(TOOL), "check", base, env=env)
        self.assertEqual(r.returncode, 0)
        diffs = [line for line in log.read_text().splitlines() if " diff " in f" {line} " and "--raw" in line]
        self.assertTrue(diffs, "check never ran git diff --raw")
        self.assertTrue(all("--no-abbrev" in line for line in diffs), diffs)
        sizes = [line for line in log.read_text().splitlines() if " cat-file " in f" {line} "]
        self.assertEqual(len(sizes), 1)
        self.assertIn("--batch-check", sizes[0])

    def test_pull_into_absent_sparse_directory_preserves_bytes_and_clean_tree(self):
        self.art("push", "research/r1/runs/a.jsonl")
        self.git("commit", "-qm", "archive fixture")
        (self.repo / "research/r1/runs/a.jsonl").unlink()
        self.git("sparse-checkout", "set", "--no-cone", "/*", "!/research/")
        self.assertFalse((self.repo / "research").exists())
        self.art("pull", "research/r1/runs/a.jsonl")
        self.assertEqual((self.repo / "research/r1/runs/a.jsonl").read_bytes(), self.big)
        self.assertEqual(self.git("status", "--porcelain"), "")

    def test_check_gate(self):
        base = self.git("rev-parse", "HEAD").strip()
        (self.repo / "new.jsonl").write_bytes(os.urandom(100_000))
        (self.repo / "tool.py").write_text("x = 1\n" * 300_000)
        self.git("add", "-A")
        self.git("commit", "-qm", "add data")
        r = self.art("check", base, check=False)
        self.assertEqual(r.returncode, 1)
        self.assertIn("new.jsonl", r.stdout)
        self.assertNotIn("tool.py", r.stdout)

        base = self.git("rev-parse", "HEAD").strip()
        self.art("push", "new.jsonl")
        self.git("commit", "-qm", "move it")
        self.assertEqual(self.art("check", base).returncode, 0)

    def test_migrate_from_subdirectory_resolves_root_level_candidates(self):
        base = self.git("rev-parse", "HEAD").strip()
        payload = os.urandom(100_000)
        (self.repo / "new.jsonl").write_bytes(payload)
        self.git("add", "new.jsonl")
        self.git("commit", "-qm", "add root-level data")
        caller = self.repo / "research"

        dry = self.art("migrate", base, "--dry-run", cwd=caller)
        self.assertIn("new.jsonl", dry.stdout)
        self.assertEqual(self.art("migrate", base, cwd=caller).returncode, 0)

        tracked = self.git("ls-files").splitlines()
        self.assertNotIn("new.jsonl", tracked)
        manifest = self.repo / ".artifacts.json"
        self.assertTrue(manifest.is_file())
        entry = json.loads(manifest.read_text())["files"]["new.jsonl"]
        digest = hashlib.sha256(payload).hexdigest()
        self.assertEqual(entry, {"sha256": digest, "bytes": len(payload)})
        self.assertEqual((self.remote / "sha256" / digest[:2] / digest).read_bytes(), payload)

    def test_migrate_selects_changed_data_and_replays_remote(self):
        base = self.git("rev-parse", "HEAD").strip()
        payload = os.urandom(100_000)
        (self.repo / "new.jsonl").write_bytes(payload)
        (self.repo / "code.py").write_text("x = 1\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "new files")
        dry = self.art("migrate", base, "--dry-run").stdout
        self.assertIn("new.jsonl", dry)
        self.assertNotIn("research/r1/runs/a.jsonl", dry)
        self.art("migrate", base)
        digest = hashlib.sha256(payload).hexdigest()
        self.assertEqual((self.remote / "sha256" / digest[:2] / digest).read_bytes(), payload)
        staged = self.git("diff", "--cached", "--name-status")
        self.assertIn("D\tnew.jsonl", staged)
        self.assertIn("A\t.artifacts.json", staged)
        self.assertIn("A\t.gitignore", staged)
        self.assertNotIn("research/r1/runs/a.jsonl", staged)

    def test_migrate_refuses_same_size_corrupt_remote_before_staging(self):
        base = self.git("rev-parse", "HEAD").strip()
        payload = os.urandom(100_000)
        (self.repo / "new.jsonl").write_bytes(payload)
        self.git("add", "-A")
        self.git("commit", "-qm", "new data")
        digest = hashlib.sha256(payload).hexdigest()
        obj = self.remote / "sha256" / digest[:2] / digest
        obj.parent.mkdir(parents=True)
        obj.write_bytes(b"x" * len(payload))
        result = self.art("migrate", base, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("remote byte replay failed", result.stderr)
        self.assertIn("D\tnew.jsonl", self.git("diff", "--cached", "--name-status"))
        # A failed replay can leave a staged migration for inspection, but
        # the workflow's commit step must not run after the nonzero exit.

    def test_migrate_from_sparse_checkout_fetches_changed_blob(self):
        base = self.git("rev-parse", "HEAD").strip()
        payload = os.urandom(100_000)
        path = self.repo / "research/r1/runs/new.jsonl"
        path.write_bytes(payload)
        self.git("add", "-A")
        self.git("commit", "-qm", "new data")
        self.git("sparse-checkout", "set", "--no-cone", "/*", "!/research/")
        self.assertFalse(path.exists())
        self.art("migrate", base)
        digest = hashlib.sha256(payload).hexdigest()
        self.assertEqual((self.remote / "sha256" / digest[:2] / digest).read_bytes(), payload)
        self.assertIn("D\tresearch/r1/runs/new.jsonl", self.git("diff", "--cached", "--name-status"))

    def test_check_gate_in_blobless_clone(self):
        # CI checks out with `filter: blob:none`: a modified file's blob is
        # not local until asked for, and a partial clone fetches a missing
        # blob only by its full id, never by an abbreviated one.
        (self.repo / "tool.py").write_text("x = 2\n" * 1_000)
        self.git("add", "-A")
        self.git("commit", "-qm", "edit code")
        self.git("config", "uploadpack.allowFilter", "true")
        clone = Path(self.tmp.name).resolve() / "clone"
        sh(Path(self.tmp.name), "git", "clone", "-q", "--filter=blob:none", "--no-checkout", "--depth", "2",
           f"file://{self.repo}", str(clone), env=self.env)
        r = self.art("check", "HEAD^1", "HEAD", check=False, cwd=clone)
        self.assertEqual(r.returncode, 0, r.stderr)


class Policy(unittest.TestCase):
    P = artifacts.DEFAULT_POLICY

    def test_rules(self):
        f = artifacts.belongs_in_store
        self.assertTrue(f("a/b.jsonl", 70_000, self.P))
        self.assertFalse(f("a/b.jsonl", 60_000, self.P))
        self.assertTrue(f("a/b.json", 2 << 20, self.P))
        self.assertFalse(f("a/b.json", 500_000, self.P))
        self.assertFalse(f("src/big.rs", 5 << 20, self.P))
        self.assertTrue(f("x/fb.tar.part-0001", 100_000, self.P))
        self.assertFalse(f("x/.artifacts.json", 5 << 20, self.P))
        self.assertFalse(f("docs/browser/data.json", 30 << 20, {**self.P, "keep": ["docs/browser/data.json"]}))

    def test_the_committed_policy_keeps_every_pdf_in_git(self):
        # Owner decision (2026-10-26): reports stay in Git at any size; the
        # large-files gate (10 MiB) still applies.
        policy = artifacts.Config.load(Path(__file__).resolve().parents[2]).policy
        f = artifacts.belongs_in_store
        self.assertFalse(f("research/x_20261026/REPORT.pdf", 900_000, policy))
        self.assertFalse(f("docs/notes/big.pdf", 5 << 20, policy))
        self.assertTrue(f("research/x_20261026/figures/plot.png", 900_000, policy))


class WorkflowWiringPaths(unittest.TestCase):
    def setUp(self):
        self.moved = {"research/scalar_mul_ct_timing_20261022/REPORT.pdf"}
        self.dirs = wire_workflows.moved_dirs(self.moved)

    def test_unrelated_research_path_does_not_pull_all_research_data(self):
        paths = wire_workflows.needed(
            "run: python research/notes/other_study/verify.py",
            self.moved,
            self.dirs,
        )
        self.assertEqual(paths, [])

    def test_research_study_path_pulls_its_moved_data(self):
        paths = wire_workflows.needed(
            "run: python research/scalar_mul_ct_timing_20261022/verify.py",
            self.moved,
            self.dirs,
        )
        self.assertEqual(paths, ["research/scalar_mul_ct_timing_20261022"])

    def test_nested_artifact_does_not_make_sibling_notes_a_reader(self):
        moved = {"research/notes/index-calculus/reports/run-1/report.pdf"}
        dirs = wire_workflows.moved_dirs(moved)
        self.assertEqual(
            wire_workflows.needed("run: python research/notes/index-calculus/other.py", moved, dirs),
            [],
        )
        self.assertEqual(
            wire_workflows.needed("run: python research/notes/index-calculus/reports/run-1/verify.py", moved, dirs),
            ["research/notes/index-calculus/reports/run-1"],
        )
        self.assertEqual(
            wire_workflows.needed("run: ls research/notes/index-calculus/reports/run-1", moved, dirs),
            ["research/notes/index-calculus/reports/run-1"],
        )


class WorkflowSparsePatterns(unittest.TestCase):
    """The pull step needs tools/artifacts and the .artifacts config in a sparse checkout."""

    def test_block_list_gets_tools_and_config(self):
        block = ["      - uses: actions/checkout@v4",
                 "        with:",
                 "          sparse-checkout-cone-mode: false",
                 "          sparse-checkout: |",
                 "            /docs/x/"]
        out = wire_workflows.add_sparse_pattern(block, "      ")
        self.assertEqual(out[-3:], ["            /docs/x/", "            /tools/artifacts/", "            /.artifacts/"])

    def test_scalar_pattern_becomes_a_list(self):
        block = ["      - uses: actions/checkout@v4",
                 "        with:",
                 "          sparse-checkout: taskq/deploy/eks-karpenter"]
        out = wire_workflows.add_sparse_pattern(block, "      ")
        self.assertEqual(out[2:], ["          sparse-checkout: |",
                                   "            taskq/deploy/eks-karpenter",
                                   "            tools/artifacts",
                                   "            .artifacts"])

    def test_existing_patterns_are_not_duplicated(self):
        block = ["      - uses: actions/checkout@v4",
                 "        with:",
                 "          sparse-checkout: |",
                 "            tools/artifacts",
                 "            .artifacts"]
        self.assertEqual(wire_workflows.add_sparse_pattern(block, "      "), block)


class WorkflowPullStep(unittest.TestCase):
    WF = "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n      - run: make\n"

    def test_step_installs_the_aws_cli_when_absent(self):
        out, _ = wire_workflows.wire(self.WF, ["docs/x"])
        self.assertIn("if ! command -v aws >/dev/null; then", out)
        self.assertIn('export ARTIFACTS_AWS="$artifacts_cli_aws"', out)
        self.assertIn("awscli==1.46.2", out)
        self.assertTrue(out.endswith("      - run: make\n"))

    def test_rewiring_is_idempotent_and_refreshes_a_stale_step(self):
        once, _ = wire_workflows.wire(self.WF, ["docs/x"])
        self.assertEqual(wire_workflows.wire(once, ["docs/x"])[0], once)
        stale = once.replace("pull --jobs 16 docs/x", "pull docs/old")
        self.assertEqual(wire_workflows.wire(stale, ["docs/x"])[0], once)


class WorkflowStalePullEntries(unittest.TestCase):
    """The generated pull line is output, not a reference: entries it names
    must go once the rest of the workflow stops naming them."""

    MOVED = {"research/study_a_20261031/establishment/certificate.json",
             "research/study_b_20261031/data.bin"}
    BEFORE = ("on:\n  push:\n    paths:\n"
              "      - 'research/study_a_20261031/**'\n"
              "      - 'research/study_b_20261031/**'\n"
              "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n"
              "      - run: python3 build.py\n")

    def _paths(self, text):
        return wire_workflows.needed(text, self.MOVED, wire_workflows.moved_dirs(self.MOVED))

    def _main(self, wf: Path, *extra: str) -> subprocess.CompletedProcess:
        script = ("import sys, wire_workflows as w\n"
                  "w.moved_paths = lambda root: set(%r)\n"
                  "sys.argv = ['wire_workflows.py', *sys.argv[1:]]\n"
                  "sys.exit(w.main())\n" % sorted(self.MOVED))
        return subprocess.run([sys.executable, "-c", script, *extra, str(wf)],
                              cwd=str(TOOL.parent), capture_output=True, text=True)

    def _wired(self):
        old = self._paths(self.BEFORE)
        self.assertEqual(old, ["research/study_a_20261031", "research/study_b_20261031"])
        return wire_workflows.wire(self.BEFORE, old)[0]

    def test_moved_directory_is_dropped_on_regeneration(self):
        # The paths: glob narrows to study_a only; the old pull line still names study_b.
        stale = self._wired().replace("      - 'research/study_b_20261031/**'\n", "")
        self.assertIn("research/study_b_20261031", stale)
        self.assertEqual(self._paths(stale), ["research/study_a_20261031"])
        regenerated = wire_workflows.wire(stale, self._paths(stale))[0]
        self.assertNotIn("research/study_b_20261031", regenerated)
        self.assertIn("pull --jobs 16 research/study_a_20261031\n", regenerated)

    def test_check_flags_and_regeneration_fixes_the_stale_entry(self):
        stale = self._wired().replace("      - 'research/study_b_20261031/**'\n", "")
        with tempfile.TemporaryDirectory() as d:
            wf = Path(d) / "wf.yml"
            wf.write_text(stale)
            r = self._main(wf, "--check")
            self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
            self.assertIn("wf.yml: needs", r.stdout)
            self.assertEqual(wf.read_text(), stale)
            r = self._main(wf)
            self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
            self.assertNotIn("research/study_b_20261031", wf.read_text())
            self.assertEqual(self._main(wf, "--check").returncode, 0)

    def test_declared_runtime_inputs_survive_and_stale_declarations_do_not(self):
        bare = ("# tools/artifacts pull: research/study_b_20261031 research/gone_20261031\n"
                "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n")
        self.assertEqual(self._paths(bare), ["research/study_b_20261031"])
        wired = wire_workflows.wire(bare, self._paths(bare))[0]
        self.assertEqual(self._paths(wired), ["research/study_b_20261031"])
        self.assertEqual(wire_workflows.wire(wired, self._paths(wired))[0], wired)

    def test_step_is_removed_when_nothing_is_named(self):
        wired = self._wired()
        bare = wired.replace("      - 'research/study_a_20261031/**'\n", "").replace(
            "      - 'research/study_b_20261031/**'\n", "")
        self.assertEqual(self._paths(bare), [])
        with tempfile.TemporaryDirectory() as d:
            wf = Path(d) / "wf.yml"
            wf.write_text(bare)
            r = self._main(wf, "--check")
            self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
            self.assertIn("stale", r.stdout)
            self.assertEqual(self._main(wf).returncode, 0)
            text = wf.read_text()
            self.assertNotIn(wire_workflows.STEP_NAME, text)
            self.assertIn("      - uses: actions/checkout@v4\n      - run: python3 build.py\n", text)


class S3Argv(unittest.TestCase):
    def setUp(self):
        cfg = artifacts.Config(root=Path("."), remote="s3://bucket/store", endpoint=None, read_url=None,
                               cache=Path("/c"), aws="aws")
        self.r = artifacts.Remote(cfg)
        self.d = hashlib.sha256(b"x").hexdigest()

    def test_layout_matches_crypto_store(self):
        self.assertEqual(self.r.key(self.d), f"store/sha256/{self.d[:2]}/{self.d}")

    def test_put_is_write_once_and_checksummed(self):
        a = self.r.put_args(self.d, Path("f"))
        self.assertEqual(a[a.index("--if-none-match") + 1], "*")
        self.assertEqual(a[a.index("--checksum-sha256") + 1], base64.b64encode(bytes.fromhex(self.d)).decode())
        self.assertEqual(a[a.index("--bucket") + 1], "bucket")

    def test_rejects_non_digest(self):
        with self.assertRaises(SystemExit):
            artifacts.object_key("../../etc/passwd")

    def test_get_uses_exact_object_without_head_preflight(self):
        self.assertEqual(self.r.get_args(self.d, Path("download")),
                         ["aws", "s3api", "get-object", "--bucket", "bucket",
                          "--key", f"store/sha256/{self.d[:2]}/{self.d}", "download"])
        self.r.cfg.endpoint = "https://store.example.invalid"
        self.assertEqual(self.r.get_args(self.d, Path("download"))[-2:],
                         ["--endpoint-url", self.r.cfg.endpoint])

    def test_download_failures_remove_partial_bytes_and_explain_cause(self):
        cases = [
            ("AllAccessDisabled", "bucket owner must restore access"),
            ("NoSuchKey", "restore the exact manifest SHA-256"),
            ("AccessDenied", "do not treat it as absence"),
            ("(403) Forbidden", "do not treat it as absence"),
        ]
        for error, hint in cases:
            with self.subTest(error=error), tempfile.TemporaryDirectory() as tmp:
                dest = Path(tmp) / "partial"
                dest.write_bytes(b"partial response")
                result = subprocess.CompletedProcess([], 1, "", error)
                with mock.patch.object(self.r, "_run", return_value=result), \
                     mock.patch("sys.stderr") as stderr, self.assertRaises(SystemExit):
                    self.r.get(self.d, dest)
                self.assertFalse(dest.exists())
                message = "".join(call.args[0] for call in stderr.write.call_args_list)
                self.assertIn(hint, message)
                self.assertIn("s3://bucket/store", message)

    def test_direct_download_still_rejects_corrupt_object(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.r.cfg.cache = Path(tmp) / "cache"
            def corrupt_get(digest, dest):
                dest.write_bytes(b"y")  # same size, different SHA-256
            with mock.patch.object(self.r, "get", side_effect=corrupt_get), \
                 mock.patch("sys.stderr"), self.assertRaises(SystemExit):
                artifacts.ensure_cached(self.r.cfg, self.r, self.d, 1)
            self.assertFalse(artifacts.cache_path(self.r.cfg, self.d).exists())


class WorkflowBootstrap(unittest.TestCase):
    def test_sparse_clone_can_load_tool_and_remote_configuration(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sh(root, "git", "init", "-q", "-b", "main")
            for name, content in {".github/scripts/scope.py": "", "tools/artifacts/artifacts.py": TOOL.read_text(), ".artifacts/config.json": '{"remote":"file:///test-store"}'}.items():
                p = root / name
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text(content)
            sh(root, "git", "add", ".")
            sh(root, "git", "-c", "user.name=test", "-c", "user.email=test@example.invalid", "commit", "-qm", "fixture")
            block = ["      - uses: actions/checkout@v4", "        with:", "          sparse-checkout: .github/scripts"]
            wired = wire_workflows.add_sparse_pattern(block, "      ")
            patterns = [line.strip() for line in wired[3:]]
            sh(root, "git", "sparse-checkout", "set", "--cone", *patterns)
            self.assertTrue((root / "tools/artifacts/artifacts.py").is_file())
            self.assertEqual(artifacts.Config.load(root).remote, "file:///test-store")

    def test_conditional_full_checkout_is_preserved(self):
        block = ["      - uses: actions/checkout@v4", "        with:", "          sparse-checkout: ${{ needs.changes.outputs.full_checkout != 'true' && env.SPARSE_RULES || '' }}", "          sparse-checkout-cone-mode: false"]
        self.assertEqual(wire_workflows.add_sparse_pattern(block, "      "), block)

    def test_rewiring_refreshes_paths_and_keeps_checkout_condition(self):
        source = "jobs:\n  test:\n    steps:\n      - uses: actions/checkout@v4\n        if: needs.changes.outputs.run == 'true'\n      - run: make check\n"
        once, _ = wire_workflows.wire(source, ["data/new"])
        self.assertEqual(once, wire_workflows.wire(once, ["data/new"])[0])
        stale = once.replace("pull --jobs 16 data/new", "pull data/old")
        self.assertEqual(once, wire_workflows.wire(stale, ["data/new"])[0])
        self.assertEqual(once.count("if: needs.changes.outputs.run == 'true'"), 2)
        self.assertIn("shell: bash", once)
        self.assertIn("working-directory: ${{ github.workspace }}", once)
        self.assertIn('export ARTIFACTS_AWS="$artifacts_cli_aws"', once)


class GateEvidenceDerivation(unittest.TestCase):
    """The validation gate's pull list follows VALIDATION.json -> report -> raw_data."""

    GATE = ("jobs:\n  gate:\n    steps:\n      - uses: actions/checkout@v4\n"
            "      - name: tests\n        run: python3 scripts/test_validation_gate.py\n")

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name).resolve()
        ev = self.root / "benchmarks" / "20261106-new"
        (self.root / "research" / "new_20261106" / "sessions" / "a").mkdir(parents=True)
        (self.root / "research" / "new_20261106" / "sessions" / "b").mkdir(parents=True)
        ev.mkdir(parents=True)
        for s in ("a", "b"):
            d = self.root / "research" / "new_20261106" / "sessions" / s
            (d / ".artifacts.json").write_text(json.dumps({"schema": "artifacts.manifest/v1", "files": {"records.jsonl": {"sha256": "0" * 64, "bytes": 9}}}))
        (ev / "validator-report.json").write_text(json.dumps(
            {"raw_data": [{"path": "../../research/new_20261106/sessions/a/records.jsonl"}]}))
        (ev / "replay-certificate.json").write_text("{}")
        (ev / "VALIDATION.json").write_text(json.dumps({
            "replay_certificate": {"path": "replay-certificate.json"},
            "validator_report": {"path": "validator-report.json"}}))
        self.moved = {"research/new_20261106/sessions/a/records.jsonl",
                      "research/new_20261106/sessions/b/records.jsonl"}

    def tearDown(self):
        self.tmp.cleanup()

    def test_historical_manifest_does_not_overwrite_a_path_restored_to_git(self):
        sh(self.root, "git", "init", "-q", "-b", "main")
        d = self.root / "pcc"
        d.mkdir()
        (d / ".artifacts.json").write_text(json.dumps({"schema": "artifacts.manifest/v1", "files": {
            "ledger.jsonl": {"sha256": "0" * 64, "bytes": 9},
            "old-records.jsonl": {"sha256": "1" * 64, "bytes": 9}}}))
        (d / "ledger.jsonl").write_text("current ledger\n")
        sh(self.root, "git", "add", "pcc/ledger.jsonl", "pcc/.artifacts.json")
        moved = wire_workflows.moved_paths(self.root)
        self.assertNotIn("pcc/ledger.jsonl", moved)
        self.assertIn("pcc/old-records.jsonl", moved)
        self.assertEqual(wire_workflows.needed("run: cat pcc/ledger.jsonl", moved,
                                             wire_workflows.moved_dirs(moved)), ["pcc"])
        # A broad explicit read may still pull siblings, but cannot select the
        # tracked ledger itself as a moved file.
        self.assertEqual(wire_workflows.cover({"pcc/old-records.jsonl"}, moved),
                         ["pcc/old-records.jsonl"])

    def test_only_the_store_backed_raw_data_the_report_lists_is_derived(self):
        need, notes = wire_workflows.evidence_store_files(self.root, self.moved)
        self.assertEqual(need, {"benchmarks/20261106-new/VALIDATION.json":
                                {"research/new_20261106/sessions/a/records.jsonl"}})
        self.assertFalse([n for n in notes if "store-backed" in n])

    def test_cover_never_pulls_unread_store_data(self):
        want = {"research/new_20261106/sessions/a/records.jsonl"}
        self.assertEqual(wire_workflows.cover(want, self.moved), ["research/new_20261106/sessions/a"])
        extra = self.moved | {"research/new_20261106/sessions/a/other.jsonl"}
        self.assertEqual(wire_workflows.cover(want, extra), ["research/new_20261106/sessions/a/records.jsonl"])
        both = want | {"research/new_20261106/sessions/b/records.jsonl"}
        self.assertEqual(wire_workflows.cover(both, self.moved),
                         ["research/new_20261106/sessions/a", "research/new_20261106/sessions/b"])

    def test_store_backed_report_pulls_its_whole_study(self):
        moved = self.moved | {"benchmarks/20261106-new/validator-report.json"}
        (self.root / "benchmarks/20261106-new/validator-report.json").unlink()
        need, notes = wire_workflows.evidence_store_files(self.root, moved)
        self.assertIn("benchmarks/20261106-new/validator-report.json", need["benchmarks/20261106-new/VALIDATION.json"])
        self.assertTrue([n for n in notes if "not hydrated" in n])

    def test_fixtures_are_not_evidence(self):
        fx = self.root / "scripts" / "fixtures" / "validation_gate" / "positive"
        fx.mkdir(parents=True)
        (fx / "VALIDATION.json").write_text((self.root / "benchmarks/20261106-new/VALIDATION.json").read_text())
        need, _ = wire_workflows.evidence_store_files(self.root, self.moved)
        self.assertEqual(list(need), ["benchmarks/20261106-new/VALIDATION.json"])

    def test_only_a_step_that_runs_the_gate_derives(self):
        self.assertTrue(wire_workflows.reads_validation_gate(self.GATE))
        self.assertFalse(wire_workflows.reads_validation_gate(
            "on:\n  push:\n    paths:\n      - 'scripts/validation_gate.py'\n"))
        self.assertFalse(wire_workflows.reads_validation_gate("      # python3 scripts/validation_gate.py check\n"))

    def test_check_names_the_evidence_and_regeneration_fixes_it(self):
        wf = self.root / ".github" / "workflows" / "gate.yml"
        wf.parent.mkdir(parents=True)
        wf.write_text(self.GATE)
        script = Path(wire_workflows.__file__)
        env = {**os.environ, "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t"}
        sh(self.root, "git", "init", "-q", "-b", "main")
        sh(self.root, "git", "add", "-A", env=env)
        bad = sh(self.root, sys.executable, str(script), "--check", str(wf), check=False, env=env)
        self.assertEqual(bad.returncode, 1, bad.stdout + bad.stderr)
        self.assertIn("reads store-backed evidence of benchmarks/20261106-new/VALIDATION.json", bad.stdout + bad.stderr)
        self.assertIn("do not hand-edit", bad.stdout + bad.stderr)
        sh(self.root, sys.executable, str(script), str(wf), env=env)
        self.assertIn("pull --jobs 16 research/new_20261106/sessions/a\n", wf.read_text())
        ok = sh(self.root, sys.executable, str(script), "--check", str(wf), check=False, env=env)
        self.assertEqual(ok.returncode, 0, ok.stdout + ok.stderr)


if __name__ == "__main__":
    unittest.main()
