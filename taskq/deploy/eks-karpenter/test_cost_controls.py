import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import yaml

HERE = Path(__file__).resolve().parent


def module(name, filename):
    loader = importlib.machinery.SourceFileLoader(name, str(HERE / filename))
    spec = importlib.util.spec_from_loader(name, loader)
    result = importlib.util.module_from_spec(spec)
    loader.exec_module(result)
    return result


controls = module("controls", "apply-cost-controls.py")
publication = module("publication", "check-experiment-git")


class ControlsTest(unittest.TestCase):
    def test_rendered_manifests_match_policy(self):
        policy = json.loads((HERE / "cost-controls.json").read_text())
        pools = {}
        for path in (HERE / "nodepool.yaml", HERE / "arc/nodepool.yaml"):
            for obj in yaml.safe_load_all(path.read_text()):
                if obj and obj["kind"] == "NodePool":
                    pools[obj["metadata"]["name"]] = obj["spec"]["limits"]
        self.assertEqual(pools, policy["nodepools"])
        quota = next(o for o in yaml.safe_load_all((HERE / "workloads.yaml").read_text()) if o["kind"] == "ResourceQuota")
        self.assertEqual(quota["spec"]["hard"], policy["quota"])
        for path in (HERE / "arc").glob("*values.yaml"):
            values = yaml.safe_load(path.read_text())
            if "maxRunners" in values:
                self.assertEqual((values["minRunners"], values["maxRunners"]), (0, 0))

    def test_never_raise_stricter_live_caps(self):
        obj = {"metadata": {"resourceVersion": "9"}, "spec": {"limits": {"cpu": "4000m", "memory": "16Gi", "nodes": "1"}}}
        result = controls.reduction_patch(obj, "limits", {"cpu": "16", "memory": "128Gi"})
        self.assertEqual(result[-1]["value"], obj["spec"]["limits"])
        self.assertEqual(result[0], {"op": "test", "path": "/metadata/resourceVersion", "value": "9"})

    def test_unknown_and_missing_pools_refuse(self):
        policy = json.loads((HERE / "cost-controls.json").read_text())
        with self.assertRaisesRegex(ValueError, "unknown"):
            controls.plan({"items": [{"metadata": {"name": "expensive-new-pool"}}]}, {}, policy)

    def test_quantity_equivalence_and_rejection(self):
        self.assertEqual(controls.quantity("128Gi"), controls.quantity("131072Mi"))
        self.assertEqual(controls.quantity("8"), controls.quantity("8000m"))
        self.assertEqual(controls.quantity("0Gi"), 0)
        with self.assertRaises(ValueError):
            controls.quantity("unbounded")


class PublicationTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.cwd = os.getcwd()
        os.chdir(self.tmp.name)
        self.git("init", "-q")
        self.git("config", "user.email", "test@example.invalid")
        self.git("config", "user.name", "Fixture")
        Path("run.py").write_text("print(1)\n")
        self.git("add", "run.py")
        self.git("commit", "-qm", "fixture")
        self.sha = self.git("rev-parse", "HEAD").decode().strip()

    def tearDown(self):
        os.chdir(self.cwd)
        self.tmp.cleanup()

    def git(self, *args):
        return subprocess.check_output(["git", *args])

    def published_command(self, *args):
        if args[0] == "gh":
            return (self.sha + "\n").encode()
        return subprocess.check_output(args)

    def test_published_clean_inputs(self):
        with patch.object(publication, "command", side_effect=self.published_command):
            result = publication.verify("owner/repository", ["run.py"])
        self.assertEqual(result["source_revision"], self.sha)
        self.assertEqual(result["inputs"][0]["bytes"], 9)

    def test_dirty_worktree_refuses_before_network(self):
        Path("run.py").write_text("print(2)\n")
        with self.assertRaisesRegex(ValueError, "Uncommitted"):
            publication.verify("owner/repository", ["run.py"])

    def test_staged_change_refuses_even_if_worktree_restored(self):
        Path("run.py").write_text("print(2)\n")
        self.git("add", "run.py")
        Path("run.py").write_text("print(1)\n")
        with self.assertRaisesRegex(ValueError, "Uncommitted"):
            publication.verify("owner/repository", ["run.py"])

    def test_unpublished_and_auth_failure_refuse(self):
        def unavailable(*args):
            if args[0] == "gh":
                raise subprocess.CalledProcessError(1, args)
            return subprocess.check_output(args)
        with patch.object(publication, "command", side_effect=unavailable):
            with self.assertRaises(subprocess.CalledProcessError):
                publication.verify("owner/repository", ["run.py"])

    def test_path_escape_refuses(self):
        with self.assertRaisesRegex(ValueError, "repository-relative"):
            publication.verify("owner/repository", ["../run.py"])


if __name__ == "__main__":
    unittest.main()
