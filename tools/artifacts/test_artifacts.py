"""End-to-end tests for tools/artifacts against a file:// remote in a scratch repository.

    python3 -m unittest tools/artifacts/test_artifacts.py
"""

import base64
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import unittest
import unittest.mock
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


class SparsePattern(unittest.TestCase):
    """The pull step runs tools/artifacts/artifacts.py with .artifacts/config.json, so a
    sparse checkout must fetch both, whether its patterns are a block scalar or one inline path."""

    def test_inline_cone_path_becomes_a_block(self):
        block = ["      - uses: actions/checkout@v4", "        with:",
                 "          sparse-checkout: taskq/deploy/eks-karpenter"]
        self.assertEqual(wire_workflows.add_sparse_pattern(block, "      ")[2:], [
            "          sparse-checkout: |",
            "            taskq/deploy/eks-karpenter",
            "            tools/artifacts",
            "            .artifacts",
        ])

    def test_inline_non_cone_path(self):
        block = ["      - uses: actions/checkout@v4", "        with:",
                 "          sparse-checkout-cone-mode: false", "          sparse-checkout: /Cargo.toml"]
        self.assertEqual(wire_workflows.add_sparse_pattern(block, "      ")[-3:],
                         ["            /Cargo.toml", "            /tools/artifacts/", "            /.artifacts/"])

    def test_expression_and_block_forms(self):
        expr = ["      - uses: actions/checkout@v4", "        with:",
                "          sparse-checkout: ${{ matrix.paths }}"]
        self.assertEqual(wire_workflows.add_sparse_pattern(expr, "      "), expr)
        blk = ["      - uses: actions/checkout@v4", "        with:",
               "          sparse-checkout: |", "            src"]
        self.assertEqual(wire_workflows.add_sparse_pattern(blk, "      ")[-2:],
                         ["            tools/artifacts", "            .artifacts"])
        done = wire_workflows.add_sparse_pattern(blk, "      ")
        self.assertEqual(wire_workflows.add_sparse_pattern(done, "      "), done)


class AwsBootstrap(unittest.TestCase):
    """The pip fallback for a missing aws CLI runs only on CI (or when asked) and never
    overrides a CLI named by ARTIFACTS_AWS."""

    def cfg(self, aws="aws"):
        return artifacts.Config(root=Path("."), remote="s3://b/p", endpoint=None, read_url=None,
                                cache=Path(tempfile.gettempdir()) / "unused", aws=aws)

    def test_off_outside_ci(self):
        env = {k: v for k, v in os.environ.items()
               if k not in ("GITHUB_ACTIONS", "ARTIFACTS_BOOTSTRAP_AWS", "ARTIFACTS_AWS")}
        with unittest.mock.patch.dict(os.environ, env, clear=True):
            self.assertIsNone(artifacts.bootstrap_aws(self.cfg()))

    def test_named_cli_is_respected(self):
        with unittest.mock.patch.dict(os.environ, {"GITHUB_ACTIONS": "true", "ARTIFACTS_AWS": "/opt/aws"}):
            self.assertIsNone(artifacts.bootstrap_aws(self.cfg("/opt/aws")))
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

if __name__ == "__main__":
    unittest.main()
