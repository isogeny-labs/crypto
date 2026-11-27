"""Independently check the p31 graph kernel against every replayed relation."""
import argparse
import hashlib
import json
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("data_root", type=Path, help="parent of the three Picard graph run directories")
parser.add_argument("--prefix", type=int, default=26000)
parser.add_argument("--kernel", type=Path)
parser.add_argument("--output", type=Path)
args = parser.parse_args()
assert args.prefix >= 2000 and args.prefix % 1000 == 0
ell = 11832483282198303559
earlier = args.data_root / "Picard graph stream 20261123"
later = args.data_root / "Picard graph rank completion 20261123"
kernel_path = args.kernel or later / "graph_kernel_24999.json"
kernel_bytes = kernel_path.read_bytes()
record = json.loads(kernel_bytes)
logs = record["kernel"]
assert record["ell"] == ell and len(logs) == 25000 and logs[0] == 1
assert all(isinstance(x, int) and 0 <= x < ell for x in logs)
files = [earlier / "rows_00000_2000.jsonl"] + [
    (earlier if s < 17000 else later) / f"rows_{s:05d}_1000.jsonl"
    for s in range(2000, args.prefix, 1000)
]
pair_file = later / f"pivot_pair_rows{args.prefix}_25k.jsonl"
balanced = pivots = pairs = nnz = 0

def residual(row):
    global nnz
    last = -1
    total = 0
    for j, value in row:
        assert last < j < 25000 and 0 < value < ell
        total = (total + value * logs[j]) % ell
        last = j
        nnz += 1
    return total

for path in files:
    with path.open() as source:
        for line in source:
            item = json.loads(line)
            assert item["cycle_rank"] == balanced + pivots
            if item["pivot_x"] is None:
                assert residual(item["factor_base_row"]) == 0, (path, item["cycle_rank"])
                balanced += 1
            else:
                pivots += 1
assert balanced + pivots == args.prefix
with pair_file.open() as source:
    for line in source:
        item = json.loads(line)
        assert item["connector_index"] == pairs
        assert residual(item["factor_base_row"]) == 0, (pair_file, pairs)
        pairs += 1
assert pairs == pivots - 3
result = {
    "schema": "p31-graph-all-row-kernel-check-v1", "p": 2147483647, "ell": ell,
    "cycle_prefix": args.prefix, "balanced_rows": balanced, "pivot_cycles": pivots,
    "pair_rows": pairs, "full_rows_verified": balanced + pairs,
    "nonzero_coefficients_checked": nnz,
    "kernel_sha256": hashlib.sha256(kernel_bytes).hexdigest(),
    "cycle_files": [str(p) for p in files], "pair_file": str(pair_file),
}
if args.output:
    args.output.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: v for k, v in result.items() if k != "cycle_files"}))
