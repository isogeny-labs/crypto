#!/usr/bin/env python3
"""Unit tests of the optimization-level scoring arithmetic, schema and parsers.

    python3 tools/optlevel/test_optlevel.py

No PMU, GPU or compiler is needed: measurements are synthetic documents.
"""
import copy
import json
import math
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import optlevel as O  # noqa: E402
import plot as P      # noqa: E402
import score as S     # noqa: E402

SCHEMA = json.loads((HERE / "schema" / "optlevel.record.v1.schema.json").read_text())


def st(v, lo=None, hi=None, n=5):
    return {"median": v, "min": lo if lo is not None else v, "max": hi if hi is not None else v, "n": n}


def calib():
    host = {"fingerprint": "f" * 16, "cpu_model": "test cpu", "logical_cpus": 4, "caches_bytes": {"L2": 1 << 20, "L3": 32 << 20}}
    return {"schema": S.CALIBRATION_SCHEMA, "created_utc": "2026-11-17T00:00:00Z", "host": host, "flags": [],
            "compute": {"modmuladd_p31": {"total_ops_per_s": {"1": st(2e9), "4": st(8e9)}, "scalar_ops_per_s": 5e8, "simd_ops_per_s": 2e9},
                        "ptr_hop": {"total_ops_per_s": {"1": st(5e8), "4": st(2e9)}, "scalar_ops_per_s": 5e8, "simd_ops_per_s": 5e8},
                        "fma_f64": {"total_ops_per_s": {"1": st(4e10), "4": st(1.6e11)}, "scalar_ops_per_s": 2e9, "simd_ops_per_s": 4e10}},
            "memory": {"bandwidth_bytes_per_s": {"1": st(1e10), "4": st(2e10)}, "llc_bytes": 32 << 20, "latency_ns": st(100.0)}}


def meas(cls, r1, rn=None, bytes_per_op=None, tier="T-timing", ws=None, noise=0.02, **kw):
    pts = {"1": {"ops_per_s": st(r1, r1 * 0.98, r1 * 1.01), "ops": st(1e9)}}
    if rn is not None:
        pts["4"] = {"ops_per_s": st(rn, rn * 0.97, rn * 1.02), "ops": st(4e9)}
    m = {"schema": S.MEASUREMENT_SCHEMA, "tier": tier, "op_class": cls, "host": calib()["host"], "command": ["k"],
         "ops_source": "test", "points": pts, "label": cls, "created_utc": "2026-11-17T00:00:01Z",
         "noise_floor": {"aa_ratio_deviation": noise, "rel_range": noise, "value": noise, "pairs": 5, "sufficient": True},
         "declared_bytes_per_op": bytes_per_op, "working_set_bytes": ws, "flags": [], "conditions": None}
    m.update(kw)
    return m


class Arithmetic(unittest.TestCase):
    def test_stats(self):
        self.assertEqual(S.stats([3, 1, 2]), {"median": 2.0, "min": 1.0, "max": 3.0, "n": 3})
        self.assertEqual(S.stats([1, 2, 3, 4])["median"], 2.5)
        self.assertIsNone(S.stats([None]))

    def test_interp(self):
        pts = {"1": 10.0, "4": 34.0}
        self.assertEqual(S.interp_scale(pts, 4), (34.0, {"interpolated": False, "extrapolated": False}))
        v, meta = S.interp_scale(pts, 2)
        self.assertAlmostEqual(v, 18.0)
        self.assertTrue(meta["interpolated"])
        v, meta = S.interp_scale(pts, 8)
        self.assertEqual(v, 34.0)          # never extrapolates a ceiling upward
        self.assertTrue(meta["extrapolated"])

    def test_roofline(self):
        r = S.roofline(2e9, 1e10, 24)      # memory roof 4.17e8 < peak
        self.assertEqual(r["binding"], "memory")
        self.assertAlmostEqual(r["bound_ops_per_s"], 1e10 / 24)
        self.assertAlmostEqual(r["next_roof_ratio"], 2e9 / (1e10 / 24))
        r = S.roofline(2e9, 1e10, 1)       # memory roof 1e10 > peak
        self.assertEqual((r["binding"], r["bound_ops_per_s"]), ("compute", 2e9))
        r = S.roofline(2e9, 1e10, None)
        self.assertEqual(r["binding"], "compute (traffic unknown)")
        self.assertIsNone(r["memory_roof_ops_per_s"])
        self.assertEqual(S.roofline(2e9, 1e10, 0.0)["binding"], "compute")
        with self.assertRaises(ValueError):
            S.roofline(0, 1e10, 1)

    def test_attainment_interval(self):
        a = S.attainment(st(50.0, 40.0, 60.0), 100.0, 80.0)
        self.assertEqual((a["value"], a["low"], a["high"]), (0.5, 0.4, 0.75))

    def test_noise_floor(self):
        nf = S.noise_floor([100, 101, 99, 100, 100], [102, 103, 101, 102, 102])
        self.assertAlmostEqual(nf["aa_ratio_deviation"], abs(100 / 102 - 1))
        self.assertAlmostEqual(nf["rel_range"], (103 - 99) / 101)
        self.assertEqual(nf["value"], max(nf["aa_ratio_deviation"], nf["rel_range"]))
        self.assertTrue(nf["sufficient"])
        self.assertFalse(S.noise_floor([1, 1], [1, 1])["sufficient"])

    def test_letters(self):
        self.assertEqual([S.letter(x) for x in (0.95, 0.70, 0.69, 0.40, 0.2, 0.08, 0.01)], list("AABBCDE"))
        self.assertEqual(S.letter_range(0.68, 0.05), "A/B")
        self.assertEqual(S.letter_range(0.55, 0.05), "B")

    def test_parallel(self):
        p = S.parallel_block(100.0, 200.0, 4, 1000.0, 2000.0)
        self.assertAlmostEqual(p["raw_efficiency"], 0.5)
        self.assertAlmostEqual(p["roofline_normalised_efficiency"], 1.0)   # the machine itself only scales 2x
        self.assertAlmostEqual(p["machine_scaling"], 0.5)

    def test_grade_rule(self):
        g = S.summary_grade(0.9, 0.4, 0.0, [])
        self.assertAlmostEqual(g["score"], math.sqrt(0.36))
        self.assertEqual((g["letter"], g["scope"]), ("B", "single+parallel"))
        self.assertEqual(S.summary_grade(0.9, None, 0.0, [])["scope"], "single-thread only")
        self.assertIsNone(S.summary_grade(0.9, 0.9, 0.0, ["too short"])["score"])


class Records(unittest.TestCase):
    def check(self, rec):
        self.assertEqual(S.validate(rec, SCHEMA), [])
        self.assertTrue(S.enforce_tier(rec))
        return rec

    def test_positive_control_near_compute_roof(self):
        r = self.check(S.build_record(calib(), meas("modmuladd_p31", 1.9e9, 7.4e9, bytes_per_op=12, ws=16384)))
        self.assertEqual(r["roofline"]["single"]["binding"], "compute")
        self.assertTrue(r["intensity"]["in_cache"])
        self.assertAlmostEqual(r["attainment"]["single"]["value"], 0.95)
        self.assertEqual(r["grade"]["letter"], "A")
        self.assertIn("near the roofline", r["grade"]["limiting_factor"])
        self.assertIn("algorithmic", r["grade"]["limiting_factor"])

    def test_scalar_control_is_core_bound(self):
        r = self.check(S.build_record(calib(), meas("modmuladd_p31", 5.2e8, 2.0e9, bytes_per_op=12, ws=16384),
                                      extra_diag={"vector_share_static": 0.0}))
        self.assertAlmostEqual(r["attainment"]["single"]["value"], 0.26)
        self.assertIn("not vectorized", r["grade"]["limiting_factor"])
        self.assertEqual(r["grade"]["letter"], "C")
        r2 = self.check(S.build_record(calib(), meas("modmuladd_p31", 5.2e8, 2.0e9, bytes_per_op=12, ws=16384)))
        self.assertIn("inferred from timing", r2["grade"]["limiting_factor"])

    def test_stream_on_bandwidth_slope(self):
        r = self.check(S.build_record(calib(), meas("fma_f64", 4.0e8, 7.9e8, bytes_per_op=24, ws=1 << 30)))
        self.assertEqual(r["roofline"]["single"]["binding"], "memory")
        self.assertAlmostEqual(r["attainment"]["single"]["value"], 4.0e8 / (1e10 / 24))
        self.assertAlmostEqual(r["parallel"]["raw_efficiency"], 7.9e8 / 1.6e9)
        self.assertGreater(r["parallel"]["roofline_normalised_efficiency"], 0.95)
        self.assertEqual(r["grade"]["letter"], "A")
        self.assertIn("memory-bandwidth roof", r["grade"]["limiting_factor"])

    def test_pointer_chase_is_latency_bound(self):
        r = self.check(S.build_record(calib(), meas("ptr_hop", 1.0e7, 4.0e7, bytes_per_op=64, ws=1 << 30)))
        self.assertLess(r["attainment"]["single"]["value"], 0.08)
        self.assertAlmostEqual(r["diagnosis"]["latency_attainment"], 1.0)
        self.assertIn("latency bound", r["grade"]["limiting_factor"])
        self.assertEqual(r["grade"]["letter"], "D")       # sqrt(0.064 * 0.128)

    def test_parallel_limits(self):
        r = self.check(S.build_record(calib(), meas("modmuladd_p31", 1.9e9, 2.4e9, bytes_per_op=12, ws=16384)))
        self.assertIn("parallel scaling", r["grade"]["limiting_factor"])

    def test_contended_full_point_is_reported_but_not_graded(self):
        m = meas("modmuladd_p31", 1.9e9, 4.0e9, bytes_per_op=12, ws=16384)
        m["points"]["4"]["other_cpu_cores"] = st(2.0)
        r = self.check(S.build_record(calib(), m))
        self.assertIn("FULL_POINT_CONTENDED", r["flags"])
        self.assertEqual(r["grade"]["scope"], "single-thread only")
        self.assertAlmostEqual(r["grade"]["score"], 0.95)
        self.assertAlmostEqual(r["attainment"]["full"]["value"], 0.5)      # still reported
        self.assertNotIn("parallel scaling", r["grade"]["limiting_factor"])

    def test_unknown_intensity_is_flagged(self):
        r = self.check(S.build_record(calib(), meas("fma_f64", 1e10)))
        self.assertIn("INTENSITY_UNKNOWN", r["flags"])
        self.assertEqual(r["roofline"]["single"]["binding"], "compute (traffic unknown)")
        self.assertEqual(r["grade"]["scope"], "single-thread only")
        self.assertIsNone(r["parallel"])

    def test_ceiling_exceeded_invalidates(self):
        r = self.check(S.build_record(calib(), meas("modmuladd_p31", 3.0e9, bytes_per_op=0)))
        self.assertIn("CEILING_EXCEEDED", r["flags"])
        self.assertIsNone(r["grade"]["score"])
        self.assertEqual(r["grade"]["scope"], "invalid")
        self.assertEqual(r["attainment"]["single"]["value"], 1.5)       # reported, never clamped

    def test_too_short_invalidates(self):
        r = self.check(S.build_record(calib(), meas("modmuladd_p31", 1e9, bytes_per_op=0, flags=["TOO_SHORT"])))
        self.assertIsNone(r["grade"]["letter"])

    def test_timing_tier_carries_no_pmu_values(self):
        m = meas("modmuladd_p31", 1e9, bytes_per_op=0, pmu={"ipc": 2.5, "llc_misses_per_op": 0.1},
                 memory_traffic={"source": "llc-miss", "bytes_per_op": 6.4})
        r = self.check(S.build_record(calib(), m))
        self.assertIsNone(r["diagnosis"]["ipc"])
        self.assertIsNone(r["diagnosis"]["llc_misses_per_op"])
        self.assertEqual(r["intensity"]["source"], "declared")
        self.assertIn("ipc", r["diagnosis"]["unavailable"])
        bad = copy.deepcopy(r)
        bad["diagnosis"]["ipc"] = 2.5
        with self.assertRaises(ValueError):
            S.enforce_tier(bad)
        bad = copy.deepcopy(r)
        bad["intensity"]["source"] = "llc-miss"
        with self.assertRaises(ValueError):
            S.enforce_tier(bad)

    def test_pmu_tier_uses_measured_traffic_and_topdown(self):
        m = meas("modmuladd_p31", 4e8, bytes_per_op=12, tier="T-pmu", ws=1 << 30,
                 pmu={"ipc": 0.8, "cycles_per_op": 7.0, "instructions_per_op": 5.6, "llc_misses_per_op": 0.01, "dtlb_misses_per_op": 0.0},
                 memory_traffic={"source": "llc-miss", "bytes_per_op": 0.64})
        td = {"frontend_bound": 0.05, "bad_speculation": 0.05, "backend_bound": 0.6, "retiring": 0.3, "memory_bound": 0.1, "core_bound": 0.5}
        r = self.check(S.build_record(calib(), m, extra_diag={"topdown": td}))
        self.assertEqual(r["intensity"]["source"], "llc-miss")
        self.assertAlmostEqual(r["intensity"]["declared_over_measured"], 12 / 0.64)
        self.assertEqual(r["diagnosis"]["ipc"], 0.8)
        self.assertIn("core bound", r["grade"]["limiting_factor"])
        self.assertEqual(r["grade"]["limiting_basis"], "pmu top-down")

    def test_gpu_tiers(self):
        m = meas("fma_f64", 1e10, bytes_per_op=0, tier="T-gpu-timing", gpu={"nvidia_smi": {"samples": 3}})
        r = self.check(S.build_record(calib(), m))
        self.assertEqual(r["device"], "gpu")
        bad = copy.deepcopy(r)
        bad["diagnosis"]["gpu"]["ncu"] = {"sm_throughput_pct": 90}
        with self.assertRaises(ValueError):
            S.enforce_tier(bad)
        cpu = self.check(S.build_record(calib(), meas("fma_f64", 1e10, bytes_per_op=0)))
        cpu["diagnosis"]["gpu"] = {}
        with self.assertRaises(ValueError):
            S.enforce_tier(cpu)

    def test_graded_record_needs_noise_floor(self):
        m = meas("fma_f64", 1e10, bytes_per_op=0)
        m["noise_floor"] = None
        with self.assertRaises(ValueError):
            S.build_record(calib(), m)

    def test_host_mismatch_and_algorithmic_axis(self):
        m = meas("fma_f64", 3.9e10, bytes_per_op=0)
        m["host"] = dict(m["host"], fingerprint="0" * 16)
        alg = {"counted_ops": 1e12, "best_known_ops": 1e9, "ratio": 1e3}
        r = self.check(S.build_record(calib(), m, algorithmic=alg))
        self.assertIn("CALIBRATION_HOST_MISMATCH", r["flags"])
        self.assertEqual(r["grade"]["letter"], "A")          # a tuned bad algorithm still scores high
        self.assertEqual(r["algorithmic"]["ratio"], 1e3)

    def test_schema_rejects(self):
        r = S.build_record(calib(), meas("fma_f64", 1e10, bytes_per_op=0))
        for mutate in (lambda d: d.pop("noise_floor"), lambda d: d.__setitem__("tier", "T-guess"),
                       lambda d: d["grade"].__setitem__("letter", "S"), lambda d: d["attainment"]["single"].__setitem__("value", "high")):
            bad = copy.deepcopy(r)
            mutate(bad)
            self.assertNotEqual(S.validate(bad, SCHEMA), [])

    def test_benchrec_units(self):
        r = S.build_record(calib(), meas("fma_f64", 1e10, 3e10, bytes_per_op=0))
        units = {m["unit"] for m in S.benchrec_metrics(r)}
        self.assertEqual(units, {"ops_per_s", "ratio"})


class Parsers(unittest.TestCase):
    def test_ops_protocol(self):
        out = ('noise\n{"schema":"optlevel.ops/v1","ops":10,"kernel_seconds":1.0,"bytes_declared":80,"op_class":"x","working_set_bytes":64}\n'
               '{"schema":"optlevel.ops/v1","ops":5,"kernel_seconds":0.5,"bytes_declared":40}\n{"schema":"other","ops":99}\n')
        o = O.parse_ops(out, "", None, None)
        self.assertEqual((o["ops"], o["kernel_seconds"], o["bytes_declared"], o["op_class"], o["working_set_bytes"]), (15, 1.5, 120.0, "x", 64))
        self.assertEqual(O.parse_ops("walk_steps=1,000\n", "walk_steps=24", None, r"walk_steps=([\d,]+)")["ops"], 1024)
        self.assertEqual(O.parse_ops("", "", 7, None)["ops"], 7)
        self.assertIsNone(O.parse_ops("nothing", "", None, None))

    def test_perf_csv(self):
        text = "# started\n1000,,cycles:u,5,100.00,,\n2500,,instructions:u,5,100.00,2.5,insn per cycle\n40,,cache-misses:u,5,62.50,,\n<not supported>,,dTLB-load-misses:u,0,100.00,,\n"
        c, mux, nc = O.parse_perf_csv(text)
        self.assertEqual((c["cycles"], c["instructions"]), (1000.0, 2500.0))
        self.assertEqual((mux, nc), (["cache-misses"], ["dTLB-load-misses"]))

    def test_pmu_summary(self):
        runs = {"core": [{"ops": 100, "counters": {"cycles": 400.0, "instructions": 1000.0, "branches": 100.0, "branch-misses": 2.0}}],
                "mem": [{"ops": 100, "counters": {"cycles": 400.0, "instructions": 1000.0, "cache-misses": 50.0}, "multiplexed": ["cache-misses"]}]}
        pmu, mux = O.summarise_pmu(runs, None)
        self.assertEqual((pmu["ipc"], pmu["cycles_per_op"], pmu["branch_miss_rate"], pmu["llc_misses_per_op"]), (2.5, 4.0, 0.02, 0.5))
        self.assertEqual(mux, ["cache-misses"])
        self.assertIsNone(pmu["dtlb_misses_per_op"])

    def test_topdown_normalise(self):
        doc = {"metrics": [{"metric": "tma_frontend_bound", "value": 12.0}, {"metric": "Bad Speculation", "value": 3.0},
                           {"metric": "tma_backend_bound", "value": 55.0}, {"metric": "tma_retiring", "value": 30.0},
                           {"metric": "tma_memory_bound", "value": 40.0}, {"metric": "tma_core_bound", "value": 15.0}], "multiplexed": []}
        td = O.normalise_topdown(doc)
        self.assertAlmostEqual(td["backend_bound"], 0.55)
        self.assertAlmostEqual(td["bad_speculation"], 0.03)
        self.assertIn("memory bound", S.topdown_limit(td))
        self.assertIn("retiring", S.topdown_limit({"frontend_bound": 0.1, "bad_speculation": 0.0, "backend_bound": 0.2, "retiring": 0.7}))

    def test_smi_samples(self):
        s = O.parse_smi_samples("98, 40, 310.5, 71, 0x0000000000000000\n97, 41, 300.0, 72, 0x0000000000000004\nbad line\n")
        self.assertEqual(s["samples"], 2)
        self.assertEqual(s["utilization_gpu_pct"]["median"], 97.5)
        self.assertEqual(s["throttled_sample_share"], 0.5)

    def test_scale_spec(self):
        self.assertEqual(O.parse_scale("copies:4,1"), ("copies", None, [1, 4]))
        self.assertEqual(O.parse_scale("env:RAYON_NUM_THREADS:1,2,4"), ("env", "RAYON_NUM_THREADS", [1, 2, 4]))

    def test_gpu_path_refuses_without_gpu(self):
        gpus, why = O.gpu_list()
        if gpus is None:
            self.assertTrue(why)

    def test_plot_is_wellformed_and_two_surface(self):
        c = calib()
        recs = [S.build_record(c, meas("modmuladd_p31", 1.9e9, 7.4e9, bytes_per_op=12, ws=16384)),
                S.build_record(c, meas("fma_f64", 4.0e8, 7.9e8, bytes_per_op=24, ws=1 << 30)),
                S.build_record(c, meas("ptr_hop", 1.0e7, 4.0e7, bytes_per_op=64, ws=1 << 30))]
        with tempfile.TemporaryDirectory() as d:
            out = Path(d) / "r.svg"
            P.write_roofline(c, recs, out, title="t & <t>")
            text = out.read_text()
            ET.fromstring(text)
            self.assertIn("prefers-color-scheme: dark", text)
            self.assertEqual(text.count("<circle"), 6)

    def test_registry(self):
        reg = json.loads((HERE / "registry.json").read_text())
        self.assertEqual(reg["schema"], "optlevel.registry/v1")
        ids = [k["id"] for k in reg["kernels"]]
        self.assertEqual(len(ids), len(set(ids)))
        for k in reg["kernels"]:
            for f in ("id", "repository", "path", "device", "op_class", "ops_counter", "measure", "status"):
                self.assertIn(f, k, k.get("id"))
            self.assertIn(k["device"], ("cpu", "gpu"))
            self.assertIn(k["status"], ("measured here", "needs PMU host", "needs GPU", "needs build"))
            self.assertIn(k["ops_counter"]["kind"], ("emitted", "derivable", "none"))


if __name__ == "__main__":
    unittest.main()
