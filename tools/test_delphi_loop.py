#!/usr/bin/env python3
"""Tests for tools/delphi_loop on synthetic lane records.

Run: python3 tools/test_delphi_loop.py
"""

import inspect
import json
import os
import random
import subprocess
import sys
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE / "delphi_loop"))
sys.path.insert(0, str(HERE))
import loop  # noqa: E402
import task_card as tc  # noqa: E402

SONNET, HAIKU, OPUS = ("anthropic:claude-sonnet-5-5@high", "anthropic:claude-haiku-5-5@high",
                       "anthropic:claude-opus-5-5@high")
FREE = "openrouter:openrouter/free"
USAGE = {"input_tokens": 20000, "output_tokens": 2000}

try:
    from langsmith import Client as RealClient
except ImportError:
    RealClient = None


def target(spec):
    return tc._target(spec)


def write_lane(repo, name, calls, outcome="completed", routes=None, replies=None):
    workdir = repo / "runs" / f"{name}.lane"
    workdir.mkdir(parents=True)
    for stage in ("compile", "assess"):
        (workdir / f"{stage}.prompt.md").write_text(f"{name} {stage} prompt\n")
    for call, text in (replies or {}).items():
        (workdir / f"{call}.reply.md").write_text(text)
    record = {"backend": "delphi", "card": f"runs/{name}.yaml", "assessment": f"runs/{name}.assess.yaml",
              "calls": calls, "outcome": outcome}
    if routes:
        record["routes"] = routes
    (workdir / "lane.json").write_text(json.dumps(record))
    return workdir


def call(spec, accepted=True, sha="r"):
    return {"prompt_sha256": "p" + sha, "reply_sha256": sha, "target": target(spec) if spec else None,
            "accepted": accepted, "status": "succeeded", "usage": USAGE, "batch_id": "b-" + sha}


class FakeClient:
    """Records calls; binds each against the real langsmith.Client signature when installed."""

    def __init__(self):
        self.runs, self.feedback, self.datasets, self.examples, self.review = [], [], set(), [], {}

    def _check(self, _method, *a, **kw):
        if RealClient is not None:
            inspect.signature(getattr(RealClient, _method)).bind(None, *a, **kw)

    def create_run(self, *a, **kw):
        self._check("create_run", *a, **kw)
        self.runs.append(kw)

    def read_project(self, **kw):
        self._check("read_project", **kw)
        raise type("LangSmithNotFoundError", (Exception,), {})("no project")

    def create_project(self, *a, **kw):
        self._check("create_project", *a, **kw)
        return SimpleNamespace(id="6f1d0c1e-0000-4000-8000-000000000001")

    def create_feedback(self, *a, **kw):
        self._check("create_feedback", *a, **kw)
        self.feedback.append(kw)

    def list_runs(self, **kw):
        self._check("list_runs", **kw)
        return [SimpleNamespace(extra=r["extra"]) for r in self.runs if r["run_type"] == "llm"]

    def list_feedback(self, **kw):
        self._check("list_feedback", **kw)
        now = datetime.now(timezone.utc)
        return [SimpleNamespace(run_id=rid, score=s, created_at=now) for rid, s in self.review.items()
                if rid in kw["run_ids"]]

    def has_dataset(self, **kw):
        self._check("has_dataset", **kw)
        return kw["dataset_name"] in self.datasets

    def create_dataset(self, *a, **kw):
        self._check("create_dataset", *a, **kw)
        self.datasets.add(a[0])

    def create_examples(self, **kw):
        self._check("create_examples", **kw)
        self.examples += kw["examples"]


class LoopTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.repo = Path(self.tmp.name)
        subprocess.run(["git", "init", "-q", str(self.repo)], check=True)
        os.environ["DELPHI_LOOP_HOME"] = str(self.repo / ".delphi-loop")
        os.environ.pop("DELPHI_FREE", None)
        self.config = loop.load_config()
        self.quiet = lambda *_: None

    def tearDown(self):
        os.environ.pop("DELPHI_LOOP_HOME", None)
        self.tmp.cleanup()

    def harvest(self):
        loop.harvest(self.repo, say=self.quiet)
        return loop.read_episodes(self.repo)

    def test_labels_and_spec_errors(self):
        write_lane(self.repo, "a", {
            "compile": {**call(HAIKU, accepted=False, sha="c0")},
            "compile-escalation-1": call(SONNET, sha="c1"),
            "assess": {"prompt_sha256": "pa", "target": target(SONNET), "refused": "the model declined"}},
            outcome="assessment_refused",
            replies={"compile": "```yaml\nspecification_errors: []\n```\n",
                     "compile-escalation-1": "ok"})
        write_lane(self.repo, "b", {"compile": call(None, accepted=False, sha="s")}, outcome="card_refused",
                   replies={"compile": "```yaml\nschema: x\n```\n```yaml\nspecification_errors: [no seed]\n```\n"})
        eps = {(e["group"], e["call"]): e for e in self.harvest().values()}
        self.assertEqual(eps["runs/a.lane", "compile"]["label"], "rejected")
        self.assertEqual(eps["runs/a.lane", "compile-escalation-1"]["label"], "accepted")
        self.assertEqual(eps["runs/a.lane", "compile-escalation-1"]["attempt"], 1)
        self.assertEqual(eps["runs/a.lane", "assess"]["label"], "refused")
        self.assertEqual(eps["runs/b.lane", "compile"]["label"], "spec_error")
        self.assertEqual(eps["runs/b.lane", "compile"]["target"], SONNET)   # no route: the default director
        # spec errors do not count against the model
        reg = loop.fit(eps.values(), self.config)
        sonnet = {t["target"]: t for t in reg["families"]["card-compile"]["targets"]}[SONNET]
        self.assertEqual((sonnet["n"], sonnet["accepted"], sonnet["spec_error"]), (1, 1, 1))
        # harvest is idempotent
        self.assertEqual(len(self.harvest()), 4)

    def test_exploit_keeps_incumbent_until_cheaper_target_is_proven(self):
        for i in range(4):
            write_lane(self.repo, f"h{i}", {"compile": call(HAIKU, sha=f"h{i}")})
        reg = loop.fit(self.harvest().values(), self.config)
        r = loop.route(reg, "card-compile")["route"]
        self.assertEqual(r["choice"], SONNET)
        self.assertEqual(r["escalation"], [OPUS])
        self.assertIn("incumbent", r["basis"])
        for i in range(4, 20):
            write_lane(self.repo, f"h{i}", {"compile": call(HAIKU, sha=f"h{i}")})
        reg = loop.fit(self.harvest().values(), self.config)
        r = loop.route(reg, "card-compile")["route"]
        self.assertEqual(r["choice"], HAIKU)
        self.assertEqual(r["escalation"], [SONNET, OPUS])
        h = reg["families"]["card-compile"]["targets"][1]
        self.assertGreater(h["lcb"], 0.8)
        self.assertAlmostEqual(h["usd_per_call"], 0.5 * (20000 * 0.10 + 2000 * 0.50) / 1e6)

    def test_failures_push_choice_back(self):
        for i in range(20):
            write_lane(self.repo, f"h{i}", {"compile": call(HAIKU, accepted=i % 2 == 0, sha=f"h{i}")})
        reg = loop.fit(self.harvest().values(), self.config)
        self.assertEqual(loop.route(reg, "card-compile")["route"]["choice"], SONNET)

    def test_thompson_explores_thin_evidence(self):
        reg = loop.fit([], self.config)
        picks = {loop.route(reg, "assess", "thompson", rng=random.Random(s))["route"]["choice"] for s in range(50)}
        self.assertIn(HAIKU, picks)
        self.assertEqual(loop.route(reg, "nope")["error"][:6], "family")
        self.assertEqual(loop.route(reg, "assess", free="local")["route"]["choice"], FREE)

    def test_free_selects_only_the_free_target(self):
        reg = loop.fit([], self.config)
        r = loop.route(reg, "card-compile", "thompson", free="public", rng=random.Random(0))["route"]
        self.assertEqual((r["choice"], r["escalation"]), (FREE, []))
        self.assertEqual(loop.route(reg, "assess", free="public")["route"]["choice"], FREE)
        reg["families"]["assess"]["targets"][0]["alpha"] = 200.0   # a draw that clears the threshold
        r = loop.route(reg, "assess", "thompson", free="public", rng=random.Random(1))["route"]
        self.assertEqual(r["choice"], FREE)
        self.assertIn("draw", r["basis"])

    def test_export_delphi_episode_rows(self):
        os.environ["DELPHI_LOOP_HOME"] = str(self.repo / ".delphi-loop")
        base = {"schema": loop.EPISODE_SCHEMA, "family": "card-compile", "target": HAIKU, "label": "accepted",
                "source": "lane", "observed_at": "2026-11-04T00:00:00+00:00", "stage": "compile", "attempt": 0}
        eps = {
            "b": {**base, "id": "id-b", "group": "runs/b.lane", "call": "compile", "review": 1.0,
                  "usage": {"input_tokens": 20000, "output_tokens": 0, "cache_write_tokens": 5,
                            "cache_read_tokens": None, "stop": "x"}},
            "a": {**base, "id": "id-a", "group": "runs/a.lane", "call": "compile", "label": "refused",
                  "usage": None, "review": None},
        }
        loop.write_episodes({r["id"]: r for r in eps.values()}, self.repo)
        out = self.repo / "x.jsonl"
        loop.export_episodes(self.repo, out, say=self.quiet)
        self.assertFalse(out.with_name("x.jsonl.tmp").exists())
        rows = [json.loads(line) for line in out.read_text().splitlines()]
        self.assertEqual([r["id"] for r in rows], ["id-a", "id-b"])
        self.assertEqual(rows[0], {"schema": "delphi.episode.v1", "id": "id-a", "family": "card-compile",
                                   "target": HAIKU, "label": "refused", "group": "runs/a.lane",
                                   "call": "compile", "source": "lane",
                                   "observed_at": "2026-11-04T00:00:00+00:00"})
        self.assertEqual(rows[1]["review"], 1.0)
        self.assertEqual(rows[1]["usage"], {"input_tokens": 20000, "cache_write_tokens": 5})
        self.assertNotIn("stage", rows[1])
        proc = subprocess.run([sys.executable, str(HERE / "delphi_loop" / "loop.py"), "--repo", str(self.repo),
                               "export"], capture_output=True, text=True, check=True)
        self.assertEqual(proc.stdout, out.read_text())
        self.assertTrue(proc.stderr.startswith("EXPORT 2 episodes"))

    def test_review_overrides_and_push_pull_dataset(self):
        write_lane(self.repo, "a", {"compile": call(HAIKU, sha="a1"), "assess": call(HAIKU, sha="a2")},
                   replies={"compile": "card text", "assess": "assessment text"})
        self.harvest()
        client = FakeClient()
        loop.push(client, "proj", include_text=False, repo=self.repo, say=self.quiet)
        roots = [r for r in client.runs if r["run_type"] == "chain"]
        llm = [r for r in client.runs if r["run_type"] == "llm"]
        self.assertEqual((len(roots), len(llm)), (1, 2))
        self.assertNotIn("prompt", llm[0]["inputs"])                     # hashes only by default
        self.assertTrue(all(r["dotted_order"].startswith(roots[0]["dotted_order"] + ".") for r in llm))
        self.assertEqual({f["key"] for f in client.feedback}, {"lane_outcome", "ingest_accepted"})
        loop.push(client, "proj", include_text=False, repo=self.repo, say=self.quiet)
        self.assertEqual(len(client.runs), 3)                             # idempotent
        # another container: pull rebuilds episodes; a reviewer rejects the compile reply
        compile_id = next(r["id"] for r in llm if r["extra"]["metadata"]["stage"] == "compile")
        client.review[compile_id] = 0.0
        os.environ["DELPHI_LOOP_HOME"] = str(self.repo / ".other")
        loop.pull(client, "proj", repo=self.repo, say=self.quiet)
        eps = loop.read_episodes(self.repo)
        self.assertEqual(len(eps), 2)
        self.assertEqual(loop.effective_label(eps[compile_id]), "rejected")
        os.environ["DELPHI_LOOP_HOME"] = str(self.repo / ".delphi-loop")
        loop.dataset(client, "assess", None, repo=self.repo, say=self.quiet)
        self.assertEqual(client.datasets, {"delphi-assess"})
        self.assertEqual(client.examples[0]["outputs"]["reply"], "assessment text")

    def test_eval_replays_and_feeds_back(self):
        write_lane(self.repo, "a", {"compile": call(SONNET, sha="a1")}, replies={"compile": "x"})
        self.harvest()

        def ask(prompt, workdir, name, opts):
            self.assertEqual(opts["target"]["model"], "claude-haiku-5-5")
            return "```yaml\nschema: x\n```\n```yaml\nspecification_errors: [gap]\n```\n", {"usage": USAGE}

        loop.evaluate("card-compile", HAIKU, 5, repo=self.repo, ask=ask, say=self.quiet)
        evals = [e for e in loop.read_episodes(self.repo).values() if e["source"] == "eval"]
        self.assertEqual([(e["target"], e["label"]) for e in evals], [(HAIKU, "spec_error")])
        # a later harvest re-reads the eval record and keeps it an eval episode
        self.assertEqual(sum(e["source"] == "eval" for e in self.harvest().values()), 1)

    def test_shim_answers_task_card_route_for(self):
        """The real consumer (task_card.route_for) reads the shim's answer."""
        for i in range(20):
            write_lane(self.repo, f"h{i}", {"assess": call(HAIKU, sha=f"h{i}")})
        env_home = os.environ["DELPHI_LOOP_HOME"]
        subprocess.run([sys.executable, str(HERE / "delphi_loop" / "loop.py"), "--repo", str(self.repo), "harvest"],
                       check=True, capture_output=True, env=dict(os.environ, DELPHI_LOOP_HOME=env_home))
        plan = tc.route_for("assess", {"delphi": str(HERE / "delphi_loop" / "delphi"), "free": None})
        self.assertEqual(plan["choice"], HAIKU)
        self.assertEqual(plan["escalation"], [SONNET, OPUS])
        self.assertIn("delphi-loop exploit", plan["basis"])
        free = tc.route_for("assess", {"delphi": str(HERE / "delphi_loop" / "delphi"), "free": "local"})
        self.assertEqual((free["choice"], free["escalation"]), (FREE, []))


if __name__ == "__main__":
    unittest.main(verbosity=2)
