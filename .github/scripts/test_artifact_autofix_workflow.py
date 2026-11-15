"""Stacked-base workflow regression; local Git only, no AWS or PR code execution."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/artifact-autofix.yml"
HELPER = ".github/scripts/artifact_prefetch.py"


class WorkflowRevision(unittest.TestCase):
    def setUp(self):
        self.steps = yaml.safe_load(WORKFLOW.read_text())["jobs"]["migrate"]["steps"]

    def step(self, name):
        return next(step for step in self.steps if step.get("name") == name)

    def test_helper_is_pinned_without_changing_artifact_policy_trust(self):
        helper = self.step("Checkout prefetch helper from the workflow revision")["with"]
        self.assertEqual(helper["ref"], "${{ github.workflow_sha }}")
        self.assertEqual(helper["path"], "workflow-tools")
        self.assertIs(helper["persist-credentials"], False)
        self.assertEqual(helper["sparse-checkout"].strip(), ".github/scripts")
        base = self.step("Checkout trusted artifact tool and policy")["with"]
        self.assertEqual(base["ref"], "${{ github.event.pull_request.base.sha }}")
        self.assertEqual(base["path"], "trusted")
        self.assertIs(base["persist-credentials"], False)
        candidate = self.step("Checkout exact PR head without data blobs")["with"]
        self.assertEqual(candidate["ref"], "${{ github.event.pull_request.head.sha }}")
        self.assertIn("../trusted/tools/artifacts/artifacts.py migrate",
                      self.step("Upload changed data and replay remote bytes")["run"])
        self.assertEqual(self.step("Require the trusted artifact destination and policy")["run"],
                         "cmp trusted/.artifacts/config.json candidate/.artifacts/config.json")

    def test_old_base_without_helper_and_untrusted_candidate_helper(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            candidate = root / "candidate"
            candidate.mkdir()
            env = {"PATH": os.environ["PATH"], "GIT_CONFIG_NOSYSTEM": "1",
                   "GIT_CONFIG_GLOBAL": os.devnull}

            def git(*args):
                return subprocess.check_output(["git", "-C", str(candidate), *args],
                                               env=env, stderr=subprocess.PIPE).decode().strip()

            git("init", "-q")
            git("config", "user.name", "fixture")
            git("config", "user.email", "fixture@example.invalid")
            (candidate / "base.txt").write_text("before\n")
            git("add", ".")
            git("commit", "-qm", "old base without helper")
            base = git("rev-parse", "HEAD")
            script = candidate / HELPER
            script.parent.mkdir(parents=True)
            script.write_bytes((ROOT / HELPER).read_bytes())
            git("add", ".")
            git("commit", "-qm", "workflow revision with helper")
            workflow_sha = git("rev-parse", "HEAD")
            marker = root / "candidate-code-executed"
            script.write_text("from pathlib import Path\nPath(" + repr(str(marker))
                              + ").touch()\nraise SystemExit(99)\n")
            (candidate / "base.txt").write_text("after\n")
            git("add", ".")
            git("commit", "-qm", "candidate helper must never execute")
            trusted = root / "trusted"
            trusted.mkdir()
            self.assertFalse((trusted / HELPER).exists())
            workflow_helper = root / "workflow-tools" / HELPER
            workflow_helper.parent.mkdir(parents=True)
            workflow_helper.write_text(git("show", workflow_sha + ":" + HELPER) + "\n")
            command = self.step("Prefetch changed Git blobs in bounded batches")["run"]
            env["BASE_SHA"] = base
            old = subprocess.run(["bash", "-c", command.replace("../workflow-tools/", "../trusted/")],
                                 cwd=root, env=env, capture_output=True, text=True, timeout=20)
            self.assertNotEqual(old.returncode, 0)
            self.assertIn("No such file", old.stderr)
            fixed = subprocess.run(["bash", "-c", command], cwd=root, env=env,
                                   capture_output=True, text=True, timeout=20)
            self.assertEqual(fixed.returncode, 0, fixed.stderr)
            self.assertEqual(json.loads(fixed.stdout),
                             {"changed_blobs": 2, "missing_blobs": 0, "batches": 0})
            self.assertFalse(marker.exists())


if __name__ == "__main__":
    unittest.main()
