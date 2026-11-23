#!/usr/bin/env python3
"""Frozen n35 triangular pair-index component comparison."""
import gc
import hashlib
import importlib.util
import json
from pathlib import Path
import random
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source_pipeline"]
INSTANCE = ROOT / SPEC["source_instance"]
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_pipeline_sha256"]
assert hashlib.sha256(INSTANCE.read_bytes()).hexdigest() == SPEC["source_instance_sha256"]

loaded_spec = importlib.util.spec_from_file_location("n35_original_pipeline", SOURCE)
original = importlib.util.module_from_spec(loaded_spec)
loaded_spec.loader.exec_module(original)


def triangular_index(base):
    sums = {}
    size = len(base)
    for i, A in enumerate(base):
        for j in range(i, size):
            sums.setdefault(original.point_key(A + base[j]), i * size + j)
    return sums


def run():
    t0 = time.perf_counter()
    E, elt = original.build_context()
    base, _, _, _ = original.build_base(E, elt)
    assert len(base) == SPEC["base_points"]
    fixture_seconds = time.perf_counter() - t0
    results = []
    for arm_order in SPEC["order"]:
        gc.collect()
        arm_result = {}
        indexes = {}
        for name in arm_order:
            t = time.perf_counter_ns()
            indexes[name] = original.build_index(base) if name == "ordered" else triangular_index(base)
            arm_result[name + "_build_ns"] = time.perf_counter_ns() - t
            arm_result[name + "_distinct"] = len(indexes[name])
            assert len(indexes[name]) == SPEC["expected_distinct_pair_sums"]
            assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 8 * 1024**3
        arm_result["exact_dictionary_equal"] = indexes["ordered"] == indexes["triangular"]
        assert arm_result["exact_dictionary_equal"]
        rng = random.Random(120235 + len(results))
        keys = list(indexes["ordered"])
        samples = []
        for _ in range(256):
            k = keys[rng.randrange(len(keys))]
            packed = indexes["triangular"][k]
            i, j = divmod(packed, len(base))
            assert i <= j
            assert original.point_key(base[i] + base[j]) == k
            samples.append([k, packed])
        arm_result["witness_samples"] = samples
        results.append(arm_result)
        del indexes, keys
        gc.collect()
    out = {
        "schema": "h4-n35-triangular-pair-result-v1",
        "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
        "runner_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "fixture_seconds": fixture_seconds,
        "base_points": len(base),
        "pair_visits_ordered": len(base)**2,
        "pair_visits_triangular": len(base) * (len(base) + 1) // 2,
        "repeats": results,
        "peak_rss_bytes": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        "total_seconds": time.perf_counter() - t0,
        "status": "passed"
    }
    (HERE / "result.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({k: v for k, v in out.items() if k != "repeats"}, sort_keys=True), flush=True)
    for row in results:
        print(json.dumps({k: v for k, v in row.items() if k != "witness_samples"}, sort_keys=True), flush=True)


def timeout(*_):
    raise TimeoutError("frozen 1200-second process cap")


if __name__ == "__main__":
    signal.signal(signal.SIGALRM, timeout)
    signal.alarm(1200)
    run()
