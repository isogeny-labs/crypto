"""Stream a deterministic right projection of replayed p31 factor-base rows."""
from array import array
import argparse
import glob
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument("--rows", type=int, required=True)
parser.add_argument("--seed", type=int, default=20261123)
parser.add_argument("--balanced-buckets", action="store_true")
parser.add_argument("--prefix", type=int, default=10000)
args = parser.parse_args()
n = args.rows
assert args.prefix in (10000, 15000)
balanced_target = {10000: 1631, 15000: 2434}[args.prefix]
pair_target = {10000: 8366, 15000: 12563}[args.prefix]
assert balanced_target <= n <= balanced_target + pair_target
ell = 11832483282198303559
seed = args.seed
mask = (1 << 64) - 1

def splitmix64(x):
    x = (x + 0x9e3779b97f4a7c15) & mask
    x = ((x ^ (x >> 30)) * 0xbf58476d1ce4e5b9) & mask
    x = ((x ^ (x >> 27)) * 0x94d049bb133111eb) & mask
    return x ^ (x >> 31)

if args.balanced_buckets:
    order = sorted(range(25000), key=lambda j: (splitmix64(seed ^ j), j))
    bucket = [0] * 25000
    for position, j in enumerate(order): bucket[j] = position % n
else:
    bucket = [splitmix64(seed ^ j) % n for j in range(25000)]
cycle_files = [HERE / "rows_00000_2000.jsonl"] + [HERE / f"rows_{s:05d}_1000.jsonl" for s in range(2000, args.prefix, 1000)]
pair_file = HERE / f"pivot_pair_rows{args.prefix}_25k.jsonl"
suffix = "balanced" if args.balanced_buckets else "hash"
out_file = HERE / f"rank_projection_{n}_seed{seed}_{suffix}_25k.bin"
assert not out_file.exists()
balanced = 0
pair = 0
nonzeros = 0

def project(row):
    global nonzeros
    out = array("Q", [0]) * n
    previous = -1
    for j, value in row:
        assert previous < j < 25000 and 0 < value < ell
        previous = j
        k = bucket[j]
        out[k] = (out[k] + value) % ell
        nonzeros += 1
    return out.tobytes()

with out_file.open("wb") as output:
    for path in cycle_files:
        with path.open() as stream:
            for line in stream:
                row = json.loads(line)
                if row["pivot_x"] is None:
                    output.write(project(row["factor_base_row"]))
                    balanced += 1
    assert balanced == balanced_target
    with pair_file.open() as stream:
        for line in stream:
            if balanced + pair == n: break
            row = json.loads(line)
            output.write(project(row["factor_base_row"]))
            pair += 1
assert balanced + pair == n and out_file.stat().st_size == n*n*8
manifest = {
    "schema": "p31-rank-panel-projection-v1", "p": 2147483647, "ell": ell,
    "rows": n, "factor_base_columns": 25000, "balanced_rows": balanced,
    "pair_rows": pair, "input_nonzeros": nonzeros, "seed": seed,
    "cycle_prefix": args.prefix,
    "bucket_rule": "sort columns by splitmix64(seed xor column), then position mod rows" if args.balanced_buckets else "splitmix64(seed xor column) mod rows",
    "row_order": "balanced cycles in sorted cycle order, then connector rows in forest order",
    "cycle_row_sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in cycle_files},
    "pair_rows_sha256": hashlib.sha256(pair_file.read_bytes()).hexdigest(),
    "matrix_sha256": hashlib.sha256(out_file.read_bytes()).hexdigest(),
}
out_file.with_suffix(".manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({k:v for k,v in manifest.items() if k!="cycle_row_sha256"}))
