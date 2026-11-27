"""Checkpoint exact p31 weighted relations for one sorted kernel-cycle batch."""
import argparse
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
BASE = HERE.parent / "Picard full graph 25k boundary"

parser = argparse.ArgumentParser()
parser.add_argument("--start", type=int, required=True)
parser.add_argument("--count", type=int, default=1000)
args = parser.parse_args()
assert 0 <= args.start < 37445 and 1 <= args.count <= 1000
assert args.start + args.count <= 37445

source = BASE / "replay_short_cycles.py"
prelude = source.read_text().split("selection_path = Path(sys.argv[1])", 1)[0]
env = {"__file__": str(source)}
exec(prelude, env)
line_row, add_scaled, ell = env["line_row"], env["add_scaled"], env["ell"]

selection_file = HERE / f"cycles_{args.start:05d}_{args.count}.json"
rows_file = HERE / f"rows_{args.start:05d}_{args.count}.jsonl"
manifest_file = HERE / f"rows_{args.start:05d}_{args.count}.manifest.json"
generator = BASE / "kernel_cycles.bin"
core = BASE / "core25k.bin"
command = [str(generator), str(core), str(BASE / "chains25k.bin"),
           str(BASE / "paths25k.bin"), str(args.count), str(args.start)]
if not selection_file.exists():
    result = subprocess.run(command, capture_output=True, check=True)
    temporary = selection_file.with_suffix(".json.tmp")
    temporary.write_bytes(result.stdout)
    json.loads(temporary.read_text())
    temporary.replace(selection_file)
selection = json.loads(selection_file.read_text())
assert selection["first_cycle_rank"] == args.start and len(selection["top_cycles"]) == args.count
manifest = {
    "schema": "p31-kernel-cycle-batch-v1", "p": env["p"], "ell": ell,
    "start": args.start, "count": args.count,
    "selection_sha256": hashlib.sha256(selection_file.read_bytes()).hexdigest(),
    "core_sha256": "b74888f5ccdcfd9d93779df14acf32a9cfdf1ca45a8ab302813dab0f88db44f3",
    "replay_source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
}
if manifest_file.exists():
    assert json.loads(manifest_file.read_text()) == manifest
else:
    assert not rows_file.exists()
    manifest_file.write_text(json.dumps(manifest, indent=2) + "\n")
completed = 0
if rows_file.exists():
    with rows_file.open() as done:
        for line in done:
            row = json.loads(line)
            assert row["cycle_rank"] == args.start + completed
            completed += 1
assert completed <= args.count

start_time = time.perf_counter()
with core.open("rb") as stream, rows_file.open("a") as output:
    for local_index in range(completed, args.count):
        cycle = selection["top_cycles"][local_index]
        indexes, vertices = cycle["core_edge_indexes"], cycle["vertex_x_path"]
        assert len(indexes) == cycle["cycle_edges"] and len(vertices) == len(indexes) + 1
        assert vertices[0] == vertices[-1]
        small, large, source_scales = {}, {}, []
        for k, edge_index in enumerate(indexes):
            ordinal, fs, lp = line_row(edge_index, stream)
            u, v = vertices[k:k+2]
            assert set(lp) == {u, v}
            scale = 1 if k == 0 else -large[u] * pow(lp[u], -1, ell) % ell
            add_scaled(small, fs, scale)
            add_scaled(large, lp, scale)
            if k: assert not large.get(u)
            source_scales.append([edge_index, ordinal, scale])
        assert not large or set(large) == {vertices[0]}
        row = {
            "cycle_rank": args.start + local_index,
            "cycle_edges": len(indexes), "kernel_edges": cycle["kernel_edges"],
            "pivot_x": vertices[0] if large else None,
            "pivot_coefficient": large.get(vertices[0], 0),
            "factor_base_row": sorted(small.items()),
            "source_scales": source_scales,
        }
        output.write(json.dumps(row, separators=(",", ":")) + "\n")
        if (local_index + 1) % 100 == 0:
            output.flush(); os.fsync(output.fileno())
            print(json.dumps({"completed": local_index + 1, "count": args.count,
                              "elapsed_seconds": time.perf_counter() - start_time}), flush=True)
    output.flush(); os.fsync(output.fileno())
print(json.dumps({"status": "complete", "start": args.start, "count": args.count,
                  "elapsed_seconds": time.perf_counter() - start_time}), flush=True)
