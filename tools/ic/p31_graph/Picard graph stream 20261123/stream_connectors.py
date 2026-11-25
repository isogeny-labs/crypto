"""Bounded-memory, resumable replay of the p31 10,000-cycle connector forest."""
from functools import lru_cache
import argparse
import hashlib
import json
import os
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
BASE = HERE.parent / "Picard full graph 25k boundary"
parser = argparse.ArgumentParser()
parser.add_argument("--prefix", type=int, default=10000)
args = parser.parse_args()
assert args.prefix in (10000, 15000)
source = BASE / "replay_short_cycles.py"
prelude = source.read_text().split("selection_path = Path(sys.argv[1])", 1)[0]
env = {"__file__": str(source)}
exec(prelude, env)
line_row, add_scaled, ell = env["line_row"], env["add_scaled"], env["ell"]
forest_file = HERE / f"pivot_forest{args.prefix}_25k.json"
forest = json.loads(forest_file.read_text())
assert forest["selected_connectors"] == forest["pivot_count"] - 3
files = [HERE / "rows_00000_2000.jsonl"] + [HERE / f"rows_{s:05d}_1000.jsonl" for s in range(2000, args.prefix, 1000)]
offsets = {}
handles = {}
for path in files:
    handles[path] = path.open("rb")
    with path.open("rb") as stream:
        expected = 0 if path.name == "rows_00000_2000.jsonl" else int(path.name.split("_")[1])
        while True:
            position = stream.tell()
            line = stream.readline()
            if not line: break
            offsets[expected] = (path, position)
            expected += 1
assert len(offsets) == args.prefix and set(offsets) == set(range(args.prefix))

@lru_cache(maxsize=2048)
def cycle(rank):
    path, position = offsets[rank]
    stream = handles[path]
    stream.seek(position)
    row = json.loads(stream.readline())
    assert row["cycle_rank"] == rank
    return row

out_path = HERE / f"pivot_pair_rows{args.prefix}_25k.jsonl"
manifest_path = HERE / f"pivot_pair_rows{args.prefix}_25k.manifest.json"
manifest = {
    "schema": "p31-pivot-pair-rows-v1", "p": env["p"], "ell": ell,
    "connector_count": forest["selected_connectors"],
    "forest_sha256": hashlib.sha256(forest_file.read_bytes()).hexdigest(),
    "cycle_row_files_sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in files},
    "replay_source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
}
if manifest_path.exists():
    assert json.loads(manifest_path.read_text()) == manifest
else:
    assert not out_path.exists()
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
completed = 0
if out_path.exists():
    with out_path.open() as done:
        for line in done:
            row = json.loads(line)
            assert row["connector_index"] == completed
            item = forest["connectors"][completed]
            assert (row["cycle_a"], row["cycle_b"]) == (item["cycle_a"], item["cycle_b"])
            completed += 1
assert completed <= forest["selected_connectors"]

start = time.perf_counter()
with (BASE / "core25k.bin").open("rb") as edge_stream, out_path.open("a") as output:
    for connector_index in range(completed, forest["selected_connectors"]):
        connector = forest["connectors"][connector_index]
        a, b = cycle(connector["cycle_a"]), cycle(connector["cycle_b"])
        xa, xb = connector["pivot_a"], connector["pivot_b"]
        assert a["pivot_x"] == xa and b["pivot_x"] == xb
        assert 0 < a["pivot_coefficient"] < ell and 0 < b["pivot_coefficient"] < ell
        vertices, indexes = connector["vertex_x_path"], connector["core_edge_indexes"]
        assert len(vertices) == len(indexes) + 1 and vertices[0] == xa and vertices[-1] == xb
        small = dict(a["factor_base_row"])
        large = {xa: a["pivot_coefficient"]}
        scales = []
        for k, edge_index in enumerate(indexes):
            ordinal, fs, lp = line_row(edge_index, edge_stream)
            u, v = vertices[k:k+2]
            assert set(lp) == {u, v} and set(large) == {u}
            scale = -large[u] * pow(lp[u], -1, ell) % ell
            add_scaled(small, fs, scale)
            add_scaled(large, lp, scale)
            assert set(large) == {v}
            scales.append([edge_index, ordinal, scale])
        assert set(large) == {xb}
        scale_b = -large[xb] * pow(b["pivot_coefficient"], -1, ell) % ell
        add_scaled(small, dict(b["factor_base_row"]), scale_b)
        add_scaled(large, {xb: b["pivot_coefficient"]}, scale_b)
        assert not large and small
        result = {
            "connector_index": connector_index, "cycle_a": connector["cycle_a"],
            "cycle_b": connector["cycle_b"], "source_line_count": a["cycle_edges"] + b["cycle_edges"] + len(indexes),
            "factor_base_row_weight": len(small), "factor_base_row": sorted(small.items()),
            "connector_source_scales": scales, "cycle_b_scale": scale_b,
        }
        output.write(json.dumps(result, separators=(",", ":")) + "\n")
        if (connector_index + 1) % 100 == 0:
            output.flush(); os.fsync(output.fileno())
            print(json.dumps({"completed": connector_index + 1,
                              "elapsed_seconds": time.perf_counter() - start}), flush=True)
    output.flush(); os.fsync(output.fileno())
for stream in handles.values(): stream.close()
print(json.dumps({"status": "complete", "completed": forest["selected_connectors"],
                  "elapsed_seconds": time.perf_counter() - start}), flush=True)
