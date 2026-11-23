#!/usr/bin/env python3
"""Independent column-major replay of the triangular n35 pair index."""
import hashlib
import importlib.util
import json
from pathlib import Path
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
spec = json.loads((HERE / "protocol.json").read_text())
result = json.loads((HERE / "result.json").read_text())
assert hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest() == result["protocol_sha256"]
source = ROOT / spec["source_pipeline"]
assert hashlib.sha256(source.read_bytes()).hexdigest() == spec["source_pipeline_sha256"]
module_spec = importlib.util.spec_from_file_location("n35_source_for_replay", source)
source_module = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(source_module)


def key(P, zero):
    if P == zero:
        return 1 << 70
    return int(P[0].to_integer()) + (int(P[1].to_integer()) << 35)


start = time.perf_counter()
E, elt = source_module.build_context()
base, _, _, _ = source_module.build_base(E, elt)
B = len(base)
zero = E(0)
index = {}
for j in range(B):
    for i in range(j + 1):
        k = key(base[i] + base[j], zero)
        packed = i * B + j
        previous = index.get(k)
        if previous is None or packed < previous:
            index[k] = packed
assert len(index) == spec["expected_distinct_pair_sums"]
checks = 0
for repeat in result["repeats"]:
    assert repeat["exact_dictionary_equal"]
    assert repeat["ordered_distinct"] == repeat["triangular_distinct"] == len(index)
    for k, packed in repeat["witness_samples"]:
        assert index[k] == packed
        i, j = divmod(packed, B)
        assert i <= j and key(base[i] + base[j], zero) == k
        checks += 1
out = {"schema": "h4-n35-triangular-pair-audit-v1", "status": "passed",
       "base_points": B, "distinct_pair_sums": len(index),
       "sample_payloads_and_points_replayed": checks,
       "column_major_seconds": time.perf_counter() - start}
(HERE / "verify-output.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print(json.dumps(out, sort_keys=True))
