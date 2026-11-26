#!/usr/bin/env python3
"""Contract checks for the hourly relation-yield telemetry parser."""
import unittest
from unittest import mock
import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import tempfile
from hourly_cpu_telemetry import compare_cpu, append_row, read_history, perf_values, fingerprint

from hourly_relation_finding import compare, percent_change, verified_metrics


def fixture():
    return {
        "schema": "koblitz_relation_yield_bridge.v1",
        "status": "complete",
        "production_defaults_used": True,
        "frozen_policy": {"n": 23, "a": 0, "m": 2},
        "factor_base": {"factor_base_blake3": "fixed-base"},
        "arms": {
            "natural": {"count": 256, "hits": 32, "verified_point_witnesses": 32},
            "planted_sat": {"count": 64, "hits": 64, "verified_point_witnesses": 64},
            "proven_unsat": {"count": 64, "hits": 0, "verified_point_witnesses": 0},
        },
    }


class TelemetryTests(unittest.TestCase):
    def test_verified_metric_definitions(self):
        measured = verified_metrics(fixture(), 8.0)
        self.assertEqual(measured["natural_relations"], 32)
        self.assertEqual(measured["natural_trials"], 256)
        self.assertEqual(measured["relations_per_sec"], 4.0)
        self.assertEqual(measured["success_rate"], 0.125)
        self.assertEqual(measured["planted_verified"], 64)
        self.assertEqual(measured["unsat_hits"], 0)

    def test_reject_smoke_results_and_wrong_curve(self):
        data = fixture()
        data["production_defaults_used"] = False
        with self.assertRaises(ValueError):
            verified_metrics(data, 1.0)
        data = fixture()
        data["frozen_policy"]["n"] = 131
        with self.assertRaises(ValueError):
            verified_metrics(data, 1.0)

    def test_reject_invalid_control_or_witness(self):
        for arm, field, value in [
            ("natural", "verified_point_witnesses", 31),
            ("planted_sat", "hits", 63),
            ("proven_unsat", "hits", 1),
        ]:
            data = fixture()
            data["arms"][arm][field] = value
            with self.subTest(arm=arm), self.assertRaises(ValueError):
                verified_metrics(data, 1.0)

    def test_threshold_and_identity(self):
        m = verified_metrics(fixture(), 8.0)
        baseline = {
            "curve_n": 23, "curve_a": 0, "factor_base_hash": "fixed-base",
            "relations_per_sec": 4.0, "success_rate": 0.125,
        }
        self.assertEqual(compare(m, baseline), [])
        m["relations_per_sec"] = 4.8
        self.assertEqual(compare(m, baseline), [])
        m["relations_per_sec"] = 4.81
        self.assertEqual(len(compare(m, baseline)), 1)
        m["factor_base_hash"] = "other"
        self.assertIn("identity", compare(m, baseline)[0])

    def test_zero_baseline(self):
        self.assertEqual(percent_change(0, 0), 0.0)
        self.assertEqual(percent_change(1, 0), float("inf"))


class CPUCohortTests(unittest.TestCase):
    def row(self, i, cpu="CPU A", build="build-a", rate=10):
        return dict(host_cpu=cpu, runner_arch="x86_64", build_id=build,
                    measurement_mode="plain", curve_n=23, curve_a=0,
                    factor_base_hash="frozen", natural_trials=256,
                    natural_relations=163, planted_count=64, planted_verified=64,
                    unsat_count=64, unsat_hits=0, relations_per_sec=rate,
                    wall_seconds=163/rate, status="success", anomaly="",
                    run_url=f"run-{i}")

    def test_provisional_and_isolation(self):
        history = [self.row(i) for i in range(9)]
        history += [self.row(i+20, cpu="CPU B") for i in range(12)]
        history += [self.row(i+40, build="build-b") for i in range(12)]
        history += [history[0].copy()]
        row = self.row(100, rate=50)
        store = {}
        self.assertEqual(compare_cpu(row, history, store), [])
        self.assertEqual(row["baseline_samples"], 9)
        self.assertEqual(row["baseline_state"], "provisional")

    def test_freeze_and_threshold(self):
        history = [self.row(i) for i in range(10)]
        row, store = self.row(100), {}
        self.assertEqual(compare_cpu(row, history, store), [])
        version = row["baseline_version"]
        self.assertEqual(row["baseline_samples"], 10)
        row = self.row(101, rate=14)
        self.assertEqual(len(compare_cpu(row, history + [self.row(200, rate=100)], store)), 2)
        self.assertEqual(row["baseline_version"], version)
        self.assertEqual(row["throughput_deviation_pct"], 40)

    def test_fingerprint_diagnostics_preserve_identity(self):
        with tempfile.TemporaryDirectory() as d:
            binary = Path(d) / "sample"
            binary.write_bytes(b"synthetic telemetry fixture")
            flags = {"RUSTFLAGS": "-C debuginfo=0", "CARGO_INCREMENTAL": "0",
                     "CARGO_PROFILE_RELEASE_DEBUG": "0", "CARGO_BUILD_TARGET": ""}
            compiler = "fixture compiler\n"
            expected = json.dumps({
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "compiler": compiler, "flags": flags,
            }, sort_keys=True)
            output = io.StringIO()
            with mock.patch.dict(os.environ, flags), mock.patch(
                    "hourly_cpu_telemetry.subprocess.check_output",
                    return_value=compiler), contextlib.redirect_stderr(output):
                actual = fingerprint(binary)
            self.assertEqual(actual, hashlib.sha256(expected.encode()).hexdigest())
            self.assertEqual(output.getvalue(),
                             "hourly_build_fingerprint_components=" + expected + "\n")

    def test_legacy_csv_and_unavailable_counters(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "log.csv"
            append_row(path, ["old"], {"old": "line1\nline2"})
            append_row(path, ["old", "new"], {"old": "next", "new": "cpu"})
            self.assertEqual(read_history(path),
                             [{"old": "line1\nline2", "new": ""}, {"old": "next", "new": "cpu"}])
            path.write_text("123;;cycles;1\n<not supported>;;instructions;0\n")
            self.assertEqual(perf_values(path), {"cycles": 123.0})


if __name__ == "__main__":
    unittest.main()
