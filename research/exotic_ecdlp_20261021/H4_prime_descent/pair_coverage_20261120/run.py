#!/usr/bin/env python3
"""Count the exact three-sum image of the frozen H4 n19 lifted base."""

from sage.all import *
import ast
from collections import Counter
import hashlib
import json
from pathlib import Path
import random
import resource
import signal
import time
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source"]
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]
CONTROL = json.loads((ROOT / SPEC["control_result"]).read_text())
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance"}
funcs = [node for node in tree.body if isinstance(node, ast.FunctionDef)
         and node.name in needed]
assert {node.name for node in funcs} == needed
for node in funcs:
    if node.name == "build_instance":
        returns = [sub for sub in ast.walk(node) if isinstance(sub, ast.Return)
                   and isinstance(sub.value, ast.Call)
                   and isinstance(sub.value.func, ast.Name)
                   and sub.value.func.id == "dict"]
        assert len(returns) == 1
        returns[0].value.keywords += [ast.keyword(arg="E", value=ast.Name(id="E", ctx=ast.Load())),
                                      ast.keyword(arg="R", value=ast.Name(id="R", ctx=ast.Load()))]
ast.fix_missing_locations(tree)
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=funcs, type_ignores=[]), str(SOURCE), "exec"), globals())

def save(result):
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

def alarm_handler(*_):
    raise TimeoutError("three-sum enumeration reached frozen wall cap")

signal.signal(signal.SIGALRM, alarm_handler)
signal.alarm(SPEC["cap_seconds"])
result = {"schema": "h4-three-sum-coverage-result-v1", "status": "running",
          "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
          "source_sha256": SPEC["source_sha256"]}
save(result)
try:
    inst = SPEC["instance"]
    sysd = build_instance(inst["n"], inst["index"], inst["arm"])
    E, R = sysd.pop("E"), sysd.pop("R")
    l, basis = sysd["l"], sysd["basis"]
    previous = CONTROL["cells"]
    assert to_int(E.a2()) == previous[0]["curve_a2"]
    assert sysd["a6"] == previous[0]["curve_a6"]
    assert basis == previous[0]["basis"]
    assert [to_int(R[0]), to_int(R[1])] == previous[0]["target"]
    t = time.monotonic()
    points = []
    K = E.base_ring()
    for bits in range(1 << l):
        xi = 0
        for j, v in enumerate(basis):
            if (bits >> j) & 1:
                xi ^= v
        x = K([(xi >> i) & 1 for i in range(inst["n"])])
        points.extend(E.lift_x(x, all=True))
    assert len(points) == len(set(points)) == previous[0]["factor_base_points"]
    assert all(-P in points for P in points)
    pair_counts = Counter(A + B for A in points for B in points)
    setup_seconds = time.monotonic() - t
    result.update(base_points=len(points), ordered_pair_entries=len(points)**2,
                  distinct_pair_sums=len(pair_counts), setup_seconds=setup_seconds)
    save(result)
    t = time.monotonic()
    target_counts = Counter()
    for pair_sum, multiplicity in pair_counts.items():
        for C in points:
            target_counts[pair_sum + C] += multiplicity
    image_enumeration_seconds = time.monotonic() - t
    assert sum(target_counts.values()) == len(points)**3
    order = int(E.cardinality())
    assert 0 < len(target_counts) <= order
    zero = E(0)
    nonidentity_hit = len(target_counts) - int(zero in target_counts)
    planted_count = target_counts[R]
    assert planted_count == previous[0]["group_triple_count"]
    ordinary = build_instance(inst["n"], inst["index"], "random")
    Rordinary = ordinary["R"]
    assert [to_int(Rordinary[0]), to_int(Rordinary[1])] == previous[1]["target"]
    ordinary_count = target_counts[Rordinary]
    assert ordinary_count == previous[1]["group_triple_count"]
    result.update(status="complete", group_order=order,
                  ordered_triples=len(points)**3,
                  distinct_three_sums=len(target_counts),
                  identity_in_image=zero in target_counts,
                  nonidentity_coverage_numerator=nonidentity_hit,
                  nonidentity_coverage_denominator=order - 1,
                  nonidentity_coverage_decimal=nonidentity_hit / (order - 1),
                  multiplicity_histogram={str(k): v for k, v in sorted(Counter(target_counts.values()).items())},
                  max_ordered_triples_per_target=max(target_counts.values()),
                  planted_target_ordered_triples=planted_count,
                  ordinary_target_ordered_triples=ordinary_count,
                  image_enumeration_seconds=image_enumeration_seconds,
                  peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
except TimeoutError as exc:
    result.update(status="censored_timeout", error=str(exc),
                  peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
finally:
    signal.alarm(0)
    save(result)
print(json.dumps(result, indent=2, sort_keys=True))
