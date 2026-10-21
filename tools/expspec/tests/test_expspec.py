"""End-to-end tests for tools/expspec/expspec.py.

Each test builds a throwaway git repository holding only tools/task_card.py,
tools/expspec/ and the fixture files, approves the spec with a committed
coordinator decision, then plays the main session: call `next`, run every
card with `task_card.py run` (lint and all), answer every ask, fast-forward
every wait, until `next` says done. Nothing here touches the real checkout.

    python3 -m unittest discover -s tools/expspec/tests -v
"""
from __future__ import annotations

import datetime as dt
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
FIX = HERE / "fixtures"
SMOKE = REPO / "experiments" / "specs" / "EXP-EXPSPEC-SMOKE-001.yaml"


def sh(cwd: Path, *argv: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(argv, cwd=cwd, capture_output=True, text=True, check=check)


def commit(cwd: Path, msg: str) -> str:
    sh(cwd, "git", "add", "-A")
    sh(cwd, "git", "-c", "user.email=t@t", "-c", "user.name=t", "commit", "-qm", msg)
    return sh(cwd, "git", "rev-parse", "HEAD").stdout.strip()


class Sandbox:
    def __init__(self, files: dict[str, Path], decision_ids: list[str]):
        self.dir = Path(tempfile.mkdtemp(prefix="expspec-"))
        (self.dir / "tools" / "expspec").mkdir(parents=True)
        shutil.copy(REPO / "tools" / "task_card.py", self.dir / "tools")
        for f in ("expspec.py", "experiment.v1.schema.json", "experiment-state.v1.schema.json"):
            shutil.copy(REPO / "tools" / "expspec" / f, self.dir / "tools" / "expspec")
        for rel, src in files.items():
            (self.dir / rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy(src, self.dir / rel)
        (self.dir / "decisions").mkdir()
        (self.dir / "decisions" / "DEC-TEST.yaml").write_text(yaml.safe_dump(
            {"coordinator_decision": {"decision": "approve", "target_ids": decision_ids, "by": "owner"}}))
        (self.dir / ".gitignore").write_text(".fakehost/\n")
        sh(self.dir, "git", "init", "-q")
        self.code_commit = commit(self.dir, "code")

    def approve(self, rel: str, frozen: str) -> None:
        p = self.dir / rel
        t = p.read_text()
        digest = hashlib.sha256((self.dir / frozen).read_bytes()).hexdigest()
        t = t.replace("COMMIT", self.code_commit).replace("FSHA", digest)
        t = re.sub(r"sha256: [0-9a-f]{64}", f"sha256: {digest}", t)
        if "status: draft" in t:
            t = re.sub(r"status: draft.*", "status: approved", t)
            t = t.replace("  ref: main\n", f"  ref: main\n  commit: {self.code_commit}\n")
            t = re.sub(r"\ntags: (.*)\n", "\ntags: \\1\napproval: {decision: decisions/DEC-TEST.yaml, "
                                          "approved_by: owner, approved_at: '2026-10-20T20:00:00Z'}\n", t)
        p.write_text(t)
        commit(self.dir, "approve")

    def expspec(self, *args: str, check: bool = True) -> subprocess.CompletedProcess:
        return sh(self.dir, sys.executable, "tools/expspec/expspec.py", *args, check=check)

    def drive(self, rel: str, answer: str = "--yes", limit: int = 60) -> list[dict]:
        """Play the main session until a terminal action. Returns every action."""
        actions = []
        state_p = self.dir / rel.replace(".yaml", ".state.yaml")
        for _ in range(limit):
            result = self.expspec("next", rel, check=False)
            assert result.stdout, result.stderr
            act = json.loads(result.stdout)
            actions.append(act)
            if act["action"] == "run_card":
                lint = sh(self.dir, sys.executable, "tools/task_card.py", "lint", act["card"], check=False)
                assert lint.returncode == 0, lint.stdout + lint.stderr
                sh(self.dir, sys.executable, "tools/task_card.py", "run", act["card"], check=False)
            elif act["action"] in ("ask", "escalate"):
                gate = yaml.safe_load(state_p.read_text())["blocked_on"]
                if not gate:
                    return actions
                self.expspec("answer", rel, "--gate", gate, answer(gate) if callable(answer) else answer, "--by", "test-owner")
            elif act["action"] == "wait":
                state = yaml.safe_load(state_p.read_text())
                for h in state["hosts"]:
                    for k in ("last_poll_at", "started_at"):
                        if h.get(k) and h["state"] in ("up", "provisioning"):
                            t = dt.datetime.fromisoformat(h[k].replace("Z", "+00:00")) - dt.timedelta(seconds=400)
                            h[k] = t.isoformat().replace("+00:00", "Z")
                state_p.write_text(yaml.safe_dump(state, sort_keys=False))
            else:
                return actions
        raise AssertionError(f"no terminal action after {limit} steps: {actions[-3:]}")

    def state(self, rel: str) -> dict:
        return yaml.safe_load((self.dir / rel.replace(".yaml", ".state.yaml")).read_text())

    def cleanup(self) -> None:
        shutil.rmtree(self.dir, ignore_errors=True)


class SchemaTests(unittest.TestCase):
    def test_schemas_are_valid_json_schema(self):
        import jsonschema
        for name in ("experiment.v1.schema.json", "experiment-state.v1.schema.json"):
            jsonschema.Draft202012Validator.check_schema(json.loads((HERE.parent / name).read_text()))

    def test_committed_specs_validate(self):
        for spec in sorted((REPO / "experiments" / "specs").glob("EXP-*.yaml")):
            if spec.name.endswith(".state.yaml"):
                continue
            r = sh(REPO, sys.executable, "tools/expspec/expspec.py", "validate", str(spec), check=False)
            self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_draft_refuses_to_run(self):
        r = sh(REPO, sys.executable, "tools/expspec/expspec.py", "next", str(SMOKE), check=False)
        self.assertEqual(json.loads(r.stdout)["action"], "refused")


class LocalRun(unittest.TestCase):
    rel = "experiments/specs/EXP-EXPSPEC-SMOKE-001.yaml"

    def setUp(self):
        self.box = Sandbox({self.rel: SMOKE,
                            "research/expspec_smoke_20261020/run.py":
                                REPO / "research" / "expspec_smoke_20261020" / "run.py"},
                           ["EXP-EXPSPEC-SMOKE-001"])
        self.box.approve(self.rel, "research/expspec_smoke_20261020/run.py")

    def tearDown(self):
        self.box.cleanup()

    def test_runs_to_done(self):
        actions = self.box.drive(self.rel)
        self.assertEqual(actions[-1]["action"], "done", actions[-1])
        keys = [a["key"] for a in actions if a["action"] == "run_card"]
        self.assertEqual(keys, ["host#0", "measure@n3", "measure@n10", "measure@n100", "summarize"])
        st = self.box.state(self.rel)
        self.assertEqual(st["status"], "completed")
        self.assertEqual(st["vars"]["last_cell"], "100")
        facts = json.loads(self.box.expspec("facts", self.rel).stdout)
        self.assertEqual(facts["metrics"]["value"]["values"], {"n3": 6, "n10": 55, "n100": 5050})
        self.assertEqual(facts["missing_artifacts"], [])

    def test_editing_a_running_spec_is_drift(self):
        self.box.expspec("next", self.rel)
        p = self.box.dir / self.rel
        p.write_text(p.read_text().replace("max_wall_hours: 1", "max_wall_hours: 2"))
        act = json.loads(self.box.expspec("next", self.rel, check=False).stdout)
        self.assertEqual(act["action"], "escalate")
        self.assertIn("spec_drift", act["reason"])

    def test_resume_skips_done_cells(self):
        out = self.box.dir / "research/expspec_smoke_20261020/out"
        out.mkdir(parents=True)
        (out / "cell-n10.json").write_text('{"n": 10, "value": 55, "ok": true}\n')
        actions = self.box.drive(self.rel)
        keys = [a["key"] for a in actions if a["action"] == "run_card"]
        self.assertNotIn("measure@n10", keys)
        self.assertEqual(self.box.state(self.rel)["instances"]["measure@n10"]["status"], "skipped_done")

    def test_human_after_gate_requires_answer_before_more_work(self):
        p = self.box.dir / self.rel
        spec = yaml.safe_load(p.read_text())
        spec["gates"].append({"id": "owner-after", "kind": "human", "description": "review output",
                              "ask": "Continue after this output.", "on_fail": "stop"})
        spec["steps"][1]["gates"]["after"].append("owner-after")
        p.write_text(yaml.safe_dump(spec))
        commit(self.box.dir, "add after gate")
        actions = self.box.drive(self.rel, answer="--no")
        self.assertEqual(actions[-1]["action"], "stopped")
        self.assertEqual([a["gate"] for a in actions if a["action"] == "ask"],
                         ["owner-after:measure@n3"])
        keys = [a["key"] for a in actions if a["action"] == "run_card"]
        self.assertNotIn("measure@n10", keys)
        self.assertEqual(self.box.state(self.rel)["status"], "stopped")

    def test_refresh_receipt_resumes_wait_gate(self):
        p = self.box.dir / self.rel
        spec = yaml.safe_load(p.read_text())
        spec["gates"].append({"id": "refresh-host", "kind": "wait_until", "description": "host metadata",
                              "checks": [{"kind": "file_exists", "path": "{{paths.out}}/host.json"}],
                              "refresh": spec["steps"][0]["argv"], "every_s": 60,
                              "max_wait_s": 60, "on_fail": "stop"})
        spec["steps"][0]["gates"] = {"before": ["refresh-host"]}
        p.write_text(yaml.safe_dump(spec))
        commit(self.box.dir, "add refresh gate")
        actions = self.box.drive(self.rel)
        self.assertEqual(actions[-1]["action"], "done")
        self.assertEqual(self.box.state(self.rel)["instances"]["_refresh-refresh-host"]["status"], "done")

    def test_wait_after_gate_refreshes_before_dependent_work(self):
        p = self.box.dir / self.rel
        spec = yaml.safe_load(p.read_text())
        refresh = list(spec["steps"][0]["argv"])
        refresh[-1] = "{{paths.out}}/after"
        spec["gates"].append({"id": "after-ready", "kind": "wait_until", "description": "after metadata",
                              "checks": [{"kind": "file_exists", "path": "{{paths.out}}/after/host.json"}],
                              "refresh": refresh, "every_s": 60, "max_wait_s": 120, "on_fail": "stop"})
        spec["steps"][0]["gates"] = {"after": ["after-ready"]}
        p.write_text(yaml.safe_dump(spec))
        commit(self.box.dir, "add wait after gate")
        actions = self.box.drive(self.rel)
        self.assertEqual(actions[-1]["action"], "done")
        keys = [a["key"] for a in actions if a["action"] == "run_card"]
        self.assertLess(keys.index("_refresh-after-ready"), keys.index("measure@n3"))

    def test_resume_still_validates_cached_output(self):
        out = self.box.dir / "research/expspec_smoke_20261020/out"
        out.mkdir(parents=True)
        (out / "cell-n10.json").write_text('{"n": 10, "value": 55, "ok": false}\n')
        self.box.drive(self.rel)
        self.assertEqual(self.box.state(self.rel)["instances"]["measure@n10"]["status"], "void")


class BatchRun(unittest.TestCase):
    rel = "experiments/specs/EXP-EXPSPEC-BATCH-001.yaml"

    def setUp(self):
        self.box = Sandbox({self.rel: FIX / "EXP-EXPSPEC-BATCH-001.yaml",
                            "research/expspec_batch_test/fake_spot.py": FIX / "fake_spot.py",
                            "research/expspec_batch_test/score.py": FIX / "score.py"},
                           ["EXP-EXPSPEC-BATCH-001"])
        self.box.approve(self.rel, "research/expspec_batch_test/fake_spot.py")

    def tearDown(self):
        self.box.cleanup()

    def test_two_hosts_to_done(self):
        actions = self.box.drive(self.rel)
        self.assertEqual(actions[-1]["action"], "done", actions[-3:])
        asks = [a["gate"] for a in actions if a["action"] == "ask"]
        self.assertFalse([a for a in actions if a["action"] == "escalate"], actions)
        self.assertEqual(asks, ["launch-small-0", "go:score@a1", "launch-big-1", "go:score@a2"])
        phases = [a["key"].split("#")[0] for a in actions if a["action"] == "run_card"]
        self.assertEqual(phases[:2], ["_provision", "_poll"])
        self.assertEqual(phases.count("_provision"), 2)
        self.assertEqual(phases.count("_teardown"), 2)
        st = self.box.state(self.rel)
        self.assertEqual([(h["candidate"], h["state"]) for h in st["hosts"]],
                         [("small", "torn_down"), ("big", "torn_down")])
        for cell in ("a1", "a2"):
            self.assertEqual(st["instances"][f"measure@{cell}"]["status"], "done")
            self.assertEqual(st["instances"][f"score@{cell}"]["status"], "done")
        for h in st["hosts"]:
            self.assertTrue((self.box.dir / ".fakehost" / h["handle"] / "TERMINATED").exists())

    def test_no_to_launch_stops(self):
        actions = self.box.drive(self.rel, answer="--no")
        self.assertEqual(actions[-1]["action"], "stopped")
        self.assertEqual(self.box.state(self.rel)["hosts"], [])

    def test_no_to_running_host_gate_tears_down_and_stays_stopped(self):
        actions = self.box.drive(self.rel, answer=lambda g: "--yes" if g.startswith("launch-") else "--no")
        self.assertEqual(actions[-1]["action"], "stopped")
        state = self.box.state(self.rel)
        self.assertEqual(state["status"], "stopped")
        self.assertEqual(len(state["hosts"]), 1)
        self.assertEqual(state["hosts"][0]["state"], "torn_down")
        self.assertTrue((self.box.dir / ".fakehost" / state["hosts"][0]["handle"] / "TERMINATED").exists())
        again = json.loads(self.box.expspec("next", self.rel).stdout)
        self.assertEqual(again["action"], "stopped")

    def test_failed_step_stop_survives_teardown(self):
        p = self.box.dir / self.rel
        spec = yaml.safe_load(p.read_text())
        score = spec["steps"][2]
        score["argv"] = ["false"]
        score["outputs"] = []
        score["on_failure"] = "stop"
        p.write_text(yaml.safe_dump(spec))
        commit(self.box.dir, "stop on failed score")
        actions = self.box.drive(self.rel)
        self.assertEqual(actions[-1]["action"], "stopped")
        state = self.box.state(self.rel)
        self.assertEqual(state["status"], "stopped")
        self.assertEqual(len(state["hosts"]), 1)
        self.assertEqual(state["hosts"][0]["state"], "torn_down")

    def test_teardown_permission_ask_preserves_stop(self):
        p = self.box.dir / self.rel
        spec = yaml.safe_load(p.read_text())
        spec["agent"]["autonomy"]["terminate_own_hosts"] = "ask"
        p.write_text(yaml.safe_dump(spec))
        commit(self.box.dir, "ask before teardown")
        actions = self.box.drive(self.rel, answer=lambda g: "--no" if g.startswith("go:") else "--yes")
        self.assertEqual(actions[-1]["action"], "stopped")
        self.assertIn("teardown-0", [a["gate"] for a in actions if a["action"] == "ask"])
        state = self.box.state(self.rel)
        self.assertEqual(state["status"], "stopped")
        self.assertEqual(len(state["hosts"]), 1)
        self.assertEqual(state["hosts"][0]["state"], "torn_down")


if __name__ == "__main__":
    unittest.main()
