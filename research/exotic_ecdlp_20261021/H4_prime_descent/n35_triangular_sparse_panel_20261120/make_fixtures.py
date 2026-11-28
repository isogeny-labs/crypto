#!/usr/bin/env python3
"""Freeze five distinct n35 public targets before the local timing panel."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
FULL = ROOT / "research/exotic_ecdlp_20261021/H4_prime_descent/n35_triangular_sparse_pipeline_20261120/run.py"
spec = importlib.util.spec_from_file_location("n35_combined_fixture_source", FULL)
full = importlib.util.module_from_spec(spec)
spec.loader.exec_module(full)
E, elt = full.build_context()
base, reps, encoding, G = full.build_base(E, elt)
assert len(base) == 4096 and len(reps) == 2048
items = []
for i in range(5):
    tag = f"h4-n35-triangular-sparse-local-panel-v1-{i}"
    scalar = int.from_bytes(hashlib.sha256(tag.encode()).digest(), "big") % (full.R - 1) + 1
    point = scalar * G
    items.append({"index": i, "tag": tag, "fixture_scalar": scalar,
                  "point": full.point_record(point),
                  "order": ["IC", "rho"] if i % 2 == 0 else ["rho", "IC"]})
assert len({tuple(item["point"]) for item in items}) == len(items)
out = {"schema": "n35-triangular-sparse-local-panel-fixtures-v1",
       "curve_id": "EC1N35Ch4randha95af81414ea",
       "candidate_id": "IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h21421ad7a3b3",
       "targets": items}
(HERE / "fixtures.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print(json.dumps(out, indent=2, sort_keys=True))
