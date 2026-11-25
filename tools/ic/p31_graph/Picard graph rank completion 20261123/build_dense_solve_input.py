"""Materialize 24,999 exact p31 factor-base equations for native FLINT solve."""
from array import array
import hashlib
import json
from pathlib import Path
import sys

assert sys.byteorder == "little"
HERE = Path(__file__).resolve().parent
EARLIER = HERE.parent / "Picard graph stream 20261123"
ELL = 11832483282198303559
COLS = 25000
ROWS = 24999
PREFIX = 26000
files = [EARLIER / "rows_00000_2000.jsonl"] + [
    (EARLIER if s < 17000 else HERE) / f"rows_{s:05d}_1000.jsonl"
    for s in range(2000, PREFIX, 1000)
]
pair_file = HERE / "pivot_pair_rows26000_25k.jsonl"
out_file = HERE / "factor_base_24999x25000.bin"
assert not out_file.exists()
balanced = pairs = nnz = 0
digest = hashlib.sha256()

def emit(row, output):
    global nnz
    values = array("Q", [0]) * COLS
    last = -1
    for j, value in row:
        assert last < j < COLS and 0 < value < ELL
        values[j] = value
        last = j
        nnz += 1
    data = values.tobytes()
    digest.update(data)
    output.write(data)

with out_file.open("wb") as output:
    for path in files:
        with path.open() as source:
            for line in source:
                record = json.loads(line)
                if record["pivot_x"] is None:
                    emit(record["factor_base_row"], output)
                    balanced += 1
    with pair_file.open() as source:
        for line in source:
            if balanced + pairs == ROWS:
                break
            emit(json.loads(line)["factor_base_row"], output)
            pairs += 1
assert balanced + pairs == ROWS and out_file.stat().st_size == ROWS * COLS * 8
manifest = {
    "schema": "p31-factor-base-dense-solve-input-v1",
    "p": 2147483647, "ell": ELL, "rows": ROWS, "columns": COLS,
    "cycle_prefix": PREFIX, "balanced_rows": balanced, "pair_rows": pairs,
    "input_nonzeros": nnz, "matrix_sha256": digest.hexdigest(),
    "row_order": "all balanced cycles in cycle-rank order, then pair rows in forest order",
    "cycle_files": [str(p) for p in files], "pair_file": str(pair_file),
}
out_file.with_suffix(".manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({k: v for k, v in manifest.items() if k != "cycle_files"}))
