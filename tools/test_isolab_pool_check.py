#!/usr/bin/env python3
"""Tests for scripts/isolab_pool_check.sh in --from-json mode (no cluster needed).

The fixture reconstructs 2026-11-10T00:15Z (docs/session-logs/2026-11-10.md):
one cpu-spot node with 14 CPU allocatable, another lab Job holding 2 CPU and
64Gi, DaemonSets at 180m. A 12-CPU request does not fit and the check must name
the other Job, which the study-scoped pool check could not see.

Run: python3 -m unittest tools.test_isolab_pool_check
"""
from __future__ import annotations

import copy
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "isolab_pool_check.sh"
FIXTURE = ROOT / "tools" / "testdata" / "isolab_pool_20261110.json"


def check(*args: str, bundle: Path = FIXTURE) -> subprocess.CompletedProcess:
    return subprocess.run([str(SCRIPT), "--from-json", str(bundle), *args],
                          capture_output=True, text=True, timeout=60)


class PoolCheck(unittest.TestCase):
    def test_the_20261110_case_does_not_fit_and_names_the_tenant(self):
        result = check("--cpu", "12", "--memory", "56Gi")
        self.assertEqual(result.returncode, 10, result.stdout + result.stderr)
        self.assertIn("DOES NOT FIT", result.stdout)
        self.assertIn("Job isolab/p1r3-rr-a3-20261108", result.stdout)
        self.assertIn("free 11.82 CPU", result.stdout)
        # DaemonSets count against the node but are not listed as blocking jobs,
        # and a pod on a node outside the pool is ignored.
        blocking = result.stdout.split("DOES NOT FIT", 1)[1]
        self.assertNotIn("kube-proxy", blocking)
        self.assertNotIn("unrelated-web", result.stdout)

    def test_json_report_carries_the_numbers(self):
        result = check("--cpu", "12", "--memory", "56Gi", "--json")
        self.assertEqual(result.returncode, 10)
        report = json.loads(result.stdout)
        self.assertFalse(report["fits"])
        self.assertAlmostEqual(report["nodes"][0]["free"]["cpu"], 11.82, places=6)
        self.assertFalse(report["new_node"]["fits"])          # pool limit 16 already in use
        self.assertTrue(all(q["fits"] for q in report["quotas"]))   # the quota was not the blocker
        self.assertEqual([b["owner"] for b in report["blocking"]], ["p1r3-rr-a3-20261108"])

    def test_a_request_that_fits_the_free_allocatable_passes(self):
        result = check("--cpu", "10", "--memory", "56Gi")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("FITS on ip-10-0-3-17", result.stdout)

    def test_memory_is_checked_too(self):
        result = check("--cpu", "2", "--memory", "64Gi")
        self.assertEqual(result.returncode, 10, result.stdout)

    def test_without_the_tenant_twelve_cpus_fit(self):
        result = check("--cpu", "12", "--memory", "56Gi", "--exclude-job", "p1r3-rr-a3-20261108")
        self.assertEqual(result.returncode, 0, result.stdout)

    def test_a_pending_pod_for_the_pool_takes_new_node_headroom_and_is_named(self):
        bundle = json.loads(FIXTURE.read_text())
        bundle["nodepool"]["spec"]["limits"] = {"cpu": "32", "memory": "256Gi"}
        empty = copy.deepcopy(bundle)
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "b.json"
            path.write_text(json.dumps(empty))
            self.assertEqual(check("--cpu", "12", "--memory", "56Gi", bundle=path).returncode, 0)
            bundle["pods"]["items"].append({
                "metadata": {"name": "other-x-abc", "namespace": "isolab", "creationTimestamp": "2026-11-10T00:01:00Z",
                             "labels": {"job-name": "other-x"}},
                "spec": {"nodeSelector": {"isolab.dev/pool": "cpu-spot"},
                         "containers": [{"name": "t", "resources": {"requests": {"cpu": "12", "memory": "8Gi"}}}]},
                "status": {"phase": "Pending"}})
            path.write_text(json.dumps(bundle))
            result = check("--cpu", "12", "--memory", "56Gi", bundle=path)
            self.assertEqual(result.returncode, 10, result.stdout)
            self.assertIn("Job isolab/other-x", result.stdout)
            self.assertIn("pending", result.stdout)

    def test_live_mode_issues_only_reads_and_saves_its_bundle(self):
        """A fake kubectl serves the fixture; every call must be a `get`."""
        bundle = json.loads(FIXTURE.read_text())
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            for key in ("nodepool", "nodes", "pods", "quotas"):
                (tmp / f"{key}.json").write_text(json.dumps(bundle[key]))
            (tmp / "bin").mkdir()
            fake = tmp / "bin" / "kubectl"
            fake.write_text(f"""#!/usr/bin/env bash
echo "$*" >> {tmp}/calls.log
[[ $1 == get ]] || exit 9
case $2 in
  nodepool.karpenter.sh) cat {tmp}/nodepool.json ;;
  nodes) cat {tmp}/nodes.json ;;
  pods) cat {tmp}/pods.json ;;
  resourcequota) cat {tmp}/quotas.json ;;
  *) exit 9 ;;
esac
""")
            fake.chmod(0o755)
            import os
            env = dict(os.environ, PATH=f"{tmp / 'bin'}:{os.environ['PATH']}")
            result = subprocess.run([str(SCRIPT), "--cpu", "12", "--memory", "56Gi", "--save", str(tmp / "saved.json")],
                                    capture_output=True, text=True, env=env, timeout=60)
            self.assertEqual(result.returncode, 10, result.stdout + result.stderr)
            self.assertIn("Job isolab/p1r3-rr-a3-20261108", result.stdout)
            calls = (tmp / "calls.log").read_text().splitlines()
            self.assertEqual(len(calls), 4)
            self.assertTrue(all(c.startswith("get ") for c in calls), calls)
            self.assertIn("pods -A", " ".join(calls))
            self.assertEqual(check("--cpu", "12", "--memory", "56Gi", bundle=tmp / "saved.json").returncode, 10)

    def test_usage_and_unreadable_input(self):
        self.assertEqual(subprocess.run([str(SCRIPT)], capture_output=True).returncode, 2)
        self.assertEqual(check("--cpu", "1", "--memory", "1Gi", bundle=Path("/nonexistent.json")).returncode, 3)


if __name__ == "__main__":
    unittest.main()
