"""Independently replay p31 line witnesses in the shortest kernel cycles."""
import hashlib
import json
import math
import struct
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "Picard full graph pilot" / "audit_edges.py"
fields = list(map(int, (HERE / "input25k.txt").open().readline().split()))
p, tw, a, b, c, ell, mu, zeta, seed, B, tree_cap, pair_cap, fixed, count = fields
assert p == 2147483647 and B == count == 25000 and fixed == 1
fb = list(struct.iter_unpack("<II", (HERE / "fb25k.bin").read_bytes()))
fb_by_x = {x: i for i, (x, y) in enumerate(fb)}
assert len(fb_by_x) == B
helper = "MASK =" + SOURCE.read_text().split("MASK =", 1)[1].split("positions=sorted", 1)[0]
scope = dict(math=math, p=p, tw=tw, a=a, b=b, c=c, ell=ell, mu=mu, zeta=zeta,
             seed=seed, B=B, fb=fb, fb_x=set(fb_by_x))
exec(helper, scope)

def canonical(point):
    x, y = point
    assert y != 0
    ys = [y, y * zeta % p, y * zeta * zeta % p]
    k = ys.index(min(ys))
    return x, ys[k], pow(mu, (-k) % 3, ell)

def line_row(core_index, stream):
    stream.seek(core_index * 12)
    ordinal, x0, x1 = struct.unpack("<III", stream.read(12))
    i, j, phase = scope["pair_for"](ordinal)
    x_a, y_a = fb[i]
    x_b, y_j = fb[j]
    y_b = y_j * pow(zeta, phase, p) % p
    slope = (y_a - y_b) * pow((x_a - x_b) % p, -1, p) % p
    offset = (y_a - slope * x_a) % p
    assert scope["replay"](ordinal) == (x0, x1)
    points = [(x_a, y_a), (x_b, y_b),
              (x0, (slope*x0 + offset) % p), (x1, (slope*x1 + offset) % p)]
    small, large = {}, {}
    for point in points:
        x, y, coefficient = canonical(point)
        assert y**3 % p == scope["f"](x)
        if x in fb_by_x:
            assert fb[fb_by_x[x]] == (x, y)
            key, row = fb_by_x[x], small
        else:
            key, row = x, large
        row[key] = (row.get(key, 0) + coefficient) % ell
    small = {k: v for k, v in small.items() if v}
    large = {k: v for k, v in large.items() if v}
    assert set(large) == {x0, x1} and all(large.values())
    return ordinal, small, large

def add_scaled(target, source, scale):
    for key, value in source.items():
        new = (target.get(key, 0) + scale * value) % ell
        if new: target[key] = new
        else: target.pop(key, None)

selection_path = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "short_cycles25k.json"
result_path = Path(sys.argv[2]) if len(sys.argv) > 2 else HERE / "short_cycles_replay25k.json"
start = time.perf_counter()
cycles = json.loads(selection_path.read_text())["top_cycles"]
results = []
with (HERE / "core25k.bin").open("rb") as stream:
    for cycle in cycles:
        indexes, vertices = cycle["core_edge_indexes"], cycle["vertex_x_path"]
        assert len(indexes) == cycle["cycle_edges"] and len(vertices) == len(indexes) + 1
        assert vertices[0] == vertices[-1]
        small, lp_sum, scales, ordinals = {}, {}, [], []
        for k, edge_index in enumerate(indexes):
            ordinal, fs, lp = line_row(edge_index, stream)
            u, v = vertices[k:k+2]
            assert set(lp) == {u, v}
            if k == 0:
                scale = 1
            else:
                scale = -lp_sum[u] * pow(lp[u], -1, ell) % ell
            add_scaled(small, fs, scale)
            add_scaled(lp_sum, lp, scale)
            assert not lp_sum.get(u) if k else True
            scales.append(scale)
            ordinals.append(ordinal)
        lp_sum = {x: value for x, value in lp_sum.items() if value}
        assert not lp_sum or set(lp_sum) == {vertices[0]}
        results.append({
            "cycle_edges": len(indexes), "kernel_edges": cycle["kernel_edges"],
            "cycle_type": "balanced_lp_free" if not lp_sum else "unbalanced_lp_pivot",
            "residual_large_prime": lp_sum,
            "factor_base_row_weight": len(small),
            "factor_base_row": sorted(small.items()),
            "source_core_edge_indexes": indexes,
            "source_pair_ordinals": ordinals,
            "source_scales": scales,
        })
out = {
    "status": "exact_short_kernel_cycle_replay_passed", "p": p, "ell": ell, "B": B,
    "core_sha256": hashlib.sha256((HERE / "core25k.bin").read_bytes()).hexdigest(),
    "kernel_cycle_selection_sha256": hashlib.sha256(selection_path.read_bytes()).hexdigest(),
    "balanced_cycles": sum(r["cycle_type"] == "balanced_lp_free" for r in results),
    "unbalanced_cycles": sum(r["cycle_type"] == "unbalanced_lp_pivot" for r in results),
    "results": results, "elapsed_internal_seconds": time.perf_counter() - start,
    "factor_base_rank_tested": False, "original_curve_scalar_tested": False,
}
result_path.write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps({"status": out["status"], "balanced_cycles": out["balanced_cycles"],
                  "unbalanced_cycles": out["unbalanced_cycles"],
                  "cycle_edges": [r["cycle_edges"] for r in results],
                  "row_weights": [r["factor_base_row_weight"] for r in results],
                  "elapsed_internal_seconds": out["elapsed_internal_seconds"]}))
