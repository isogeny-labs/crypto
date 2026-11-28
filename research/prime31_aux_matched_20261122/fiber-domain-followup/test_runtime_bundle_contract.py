#!/usr/bin/env python3
"""Fixed metadata-only constructor contracts; no compiler or scientific binary."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("runtime_bundle", HERE / "prepare_runtime_bundle.py")
BUNDLE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BUNDLE)


class RuntimeBundleContract(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="prime31-source-payload-contract-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)

    def fixture(self):
        repo = self.root / "repository"
        repo.mkdir()
        self.git_fixture(repo, "init", "--quiet", "--template=")
        (repo / "src").mkdir()
        (repo / "src/selected.txt").write_text("historical selected bytes\n")
        (repo / "unselected.txt").write_text("synthetic unselected-body canary\n")
        self.git_fixture(repo, "add", ".")
        self.git_fixture(repo, "commit", "-qm", "First real synthetic commit")
        (repo / "src/selected.txt").write_text("current selected bytes\n")
        (repo / "input.json").write_text('{"metadata_only":true}\n')
        self.git_fixture(repo, "add", ".")
        self.git_fixture(repo, "commit", "-qm", "Second real synthetic commit")
        self.git_fixture(repo, "config", "remote.origin.url", "https://example.invalid/synthetic-config-canary")
        (repo / ".git/hooks").mkdir(exist_ok=True)
        (repo / ".git/hooks/canary").write_text("synthetic-hook-canary")
        return repo

    def git_fixture(self, repo, *arguments):
        return BUNDLE.command([BUNDLE.tool("git"), "-c", "core.hooksPath=/dev/null",
                               "-c", "commit.gpgsign=false", "-c", "user.name=Metadata fixture",
                               "-c", "user.email=metadata-fixture@invalid", "-C", repo, *arguments])

    def test_preserves_real_ancestry_and_only_selected_file_contents(self):
        repo = self.fixture()
        revision = BUNDLE.revision_of(repo)
        output = BUNDLE.fresh_directory(self.root / "bundle")
        source, report = BUNDLE.export_source(repo, output, {"src/selected.txt", "input.json"}, revision)
        self.assertEqual(report["ancestor_commits_including_head"], 2)
        self.assertEqual(BUNDLE.revision_of(source), revision)
        self.assertEqual((source / "src/selected.txt").read_bytes(), (repo / "src/selected.txt").read_bytes())
        self.assertFalse((source / "unselected.txt").exists())
        self.assertEqual(BUNDLE.git(source, "status", "--porcelain"), b"")
        self.assertNotIn("synthetic-config-canary", (source / ".git/config").read_text())
        self.assertFalse((source / ".git/hooks/canary").exists())
        unselected_blob = BUNDLE.head_blob(repo, revision, "unselected.txt")[1]
        result = subprocess.run([str(BUNDLE.tool("git")), "-C", str(source), "cat-file", "-e", unselected_blob],
                                capture_output=True, env=BUNDLE.metadata_environment(), timeout=15)
        self.assertNotEqual(result.returncode, 0)
        parent = BUNDLE.git(repo, "rev-parse", "HEAD^").decode().strip()
        old_selected_blob = BUNDLE.head_blob(repo, parent, "src/selected.txt")[1]
        result = subprocess.run([str(BUNDLE.tool("git")), "-C", str(source), "cat-file", "-e", old_selected_blob],
                                capture_output=True, env=BUNDLE.metadata_environment(), timeout=15)
        self.assertNotEqual(result.returncode, 0)

    def test_dirty_source_is_rejected_before_export(self):
        repo = self.fixture()
        revision = BUNDLE.revision_of(repo)
        (repo / "src/selected.txt").write_text("uncommitted change\n")
        output = BUNDLE.fresh_directory(self.root / "bundle")
        with self.assertRaises(BUNDLE.BundleError):
            BUNDLE.export_source(repo, output, {"src/selected.txt"}, revision)
        self.assertFalse((output / "source.bundle").exists())

    def test_existing_and_dangling_output_are_not_modified(self):
        existing = self.root / "existing"
        existing.mkdir()
        sentinel = existing / "sentinel"
        sentinel.write_bytes(b"keep existing bytes")
        link = self.root / "dangling"
        link.symlink_to(self.root / "missing")
        for candidate in (existing, link):
            with self.assertRaises(BUNDLE.BundleError):
                BUNDLE.fresh_directory(candidate)
        self.assertEqual(sentinel.read_bytes(), b"keep existing bytes")
        self.assertTrue(link.is_symlink())

    def test_source_symlinks_and_traversal_are_rejected(self):
        repo = self.fixture()
        (repo / "linked").symlink_to("src/selected.txt")
        self.git_fixture(repo, "add", "linked")
        self.git_fixture(repo, "commit", "-qm", "Real synthetic symlink commit")
        with self.assertRaises(BUNDLE.BundleError):
            BUNDLE.head_blob(repo, BUNDLE.revision_of(repo), "linked")
        for path in ("../escape", "/absolute", "a/../escape", ".git/config", "a/*", "a//b"):
            with self.assertRaises(BUNDLE.BundleError):
                BUNDLE.relative_path(path)

    def test_duplicate_json_and_conflicting_source_inventory_are_rejected(self):
        with self.assertRaises(BUNDLE.BundleError):
            BUNDLE.json_object(b'{"schema":"a","schema":"b"}')
        manifest = {"schema": "prime31-aux-matched-source-manifest/v1",
                    "whole_files": [{"path": "src/a", "sha256": "0" * 64, "bytes": 1},
                                    {"path": "src/a", "sha256": "1" * 64, "bytes": 1}],
                    "extracted_production_ranges": [], "layout_preserving_f4_copy": {"copied_files": []}}
        with self.assertRaises(BUNDLE.BundleError):
            BUNDLE.inventory(manifest)

    def test_shell_script_is_not_executed_as_a_supplied_elf(self):
        script = self.root / "not-elf"
        sentinel = self.root / "would-have-executed"
        script.write_text(f"#!/bin/sh\ntouch '{sentinel}'\n")
        script.chmod(0o755)
        with self.assertRaises(BUNDLE.BundleError):
            BUNDLE.elf_metadata(script)
        self.assertFalse(sentinel.exists())

    def test_existing_system_elf_dependency_bytes_are_captured_without_running_main(self):
        output = BUNDLE.fresh_directory(self.root / "runtime")
        # The loader's --list operation inspects this existing system ELF.
        # /usr/bin/true itself is never executed; no new binary is built.
        report = BUNDLE.inspect_runtime({"fixture-true": Path("/usr/bin/true")}, output)
        native = report["executables"]["fixture-true"]
        self.assertEqual(native["linkage"], "dynamic")
        self.assertIn("libc.so.6", native["resolved_dependencies"])
        self.assertTrue(native["required_glibc_versions"])
        self.assertFalse(report["container_runtime_qualified"])
        for entry in report["runtime_files"]:
            copied = output / "rootfs" / entry["install_path"].lstrip("/")
            self.assertEqual(BUNDLE.digest(copied.read_bytes()), entry["sha256"])
            self.assertEqual(copied.stat().st_size, entry["bytes"])


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(RuntimeBundleContract)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    print(json.dumps({
        "scope": "metadata_source_packaging_contracts_only",
        "tests_run": result.testsRun, "failures": len(result.failures), "errors": len(result.errors),
        "compiler_invoked": False, "scientific_arithmetic_executed": False,
        "native_prime31_end_to_end_bundle_qualified": False,
        "container_runtime_qualified": False, "image_published": False, "launch_admitted": False,
        "constructor_sha256": BUNDLE.digest((HERE / "prepare_runtime_bundle.py").read_bytes()),
        "test_sha256": BUNDLE.digest(Path(__file__).read_bytes()),
    }, sort_keys=True, indent=2))
    raise SystemExit(0 if result.wasSuccessful() else 1)
