#!/usr/bin/env python3
"""Direct and cofactor-cleared subgroup base on the frozen H4 n19 curve."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
for key, digest in (("control_result", "control_sha256"),
                    ("coverage_result", "coverage_sha256")):
    assert hashlib.sha256((ROOT / SPEC[key]).read_bytes()).hexdigest() == SPEC[digest]
CONTROL = json.loads((ROOT / SPEC["control_result"]).read_text())["cells"]
COVERAGE = json.loads((ROOT / SPEC["coverage_result"]).read_text())
assert COVERAGE["group_order"] == SPEC["group_order"]
assert SPEC["prime_subgroup_order"] * SPEC["cofactor"] == SPEC["group_order"]

def timeout(*_):
    raise TimeoutError("frozen 90-second subgroup diagnostic cap")

signal.signal(signal.SIGALRM, timeout)
signal.alarm(SPEC["cap_seconds"])
result = {"schema": "h4-subgroup-projection-result-v1",
          "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
          "status": "running"}

def save():
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

save()
try:
    t = time.monotonic()
    row = CONTROL[0]
    n, poly = row["n"], row["field_modulus"]
    F2 = GF(2)
    PR = PolynomialRing(F2, "z")
    z = PR.gen()
    mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
    K = GF(1 << n, "z", modulus=mod)
    def elt(v):
        return K([(v >> i) & 1 for i in range(n)])
    E = EllipticCurve(K, [1, elt(row["curve_a2"]), 0, 0, elt(row["curve_a6"])])
    zero = E(0)
    points = []
    for bits in range(1 << len(row["basis"])):
        x = 0
        for j, v in enumerate(row["basis"]):
            if (bits >> j) & 1:
                x ^= v
        points.extend(E.lift_x(elt(x), all=True))
    assert len(points) == len(set(points)) == COVERAGE["base_points"]
    lift_seconds = time.monotonic() - t
    q, h = SPEC["prime_subgroup_order"], SPEC["cofactor"]
    t = time.monotonic()
    direct = [P for P in points if q * P == zero]
    direct_seconds = time.monotonic() - t
    t = time.monotonic()
    projected_all = [h * P for P in points]
    projected = set(projected_all) - {zero}
    assert projected
    assert all(q * P == zero for P in projected)
    generator = next(iter(projected))
    subgroup = {k * generator for k in range(q)}
    assert len(subgroup) == q and q * generator == zero
    assert projected <= subgroup
    clearing_seconds = time.monotonic() - t
    t = time.monotonic()
    pair_counts = Counter(A + B for A in projected for B in projected)
    pair_seconds = time.monotonic() - t
    t = time.monotonic()
    triple_counts = Counter()
    for AB, m in pair_counts.items():
        for C in projected:
            triple_counts[AB + C] += m
    assert sum(triple_counts.values()) == len(projected)**3
    assert set(triple_counts) <= subgroup
    image_seconds = time.monotonic() - t
    t = time.monotonic()
    targets = []
    for prior in CONTROL:
        R = E(elt(prior["target"][0]), elt(prior["target"][1]))
        T = h * R
        assert T in subgroup
        targets.append({"arm": prior["arm"], "projected_target_ordered_triples": triple_counts[T],
                        "projected_target_is_identity": T == zero})
    target_seconds = time.monotonic() - t
    result.update(status="complete", group_order=SPEC["group_order"],
                  prime_subgroup_order=q, cofactor=h,
                  original_lifted_points=len(points),
                  direct_prime_subgroup_points=len(direct),
                  cleared_identity_count=projected_all.count(zero),
                  cleared_distinct_points_including_identity=len(set(projected_all)),
                  cleared_nonidentity_points=len(projected),
                  cleared_collision_count=len(projected_all)-len(set(projected_all)),
                  projected_ordered_pair_entries=len(projected)**2,
                  projected_distinct_pair_sums=len(pair_counts),
                  projected_ordered_triples=len(projected)**3,
                  projected_three_sum_coverage=len(triple_counts),
                  projected_nonidentity_coverage=len(set(triple_counts)-{zero}),
                  subgroup_enumerated_points=len(subgroup),
                  targets=targets,
                  stage_seconds={"lift": lift_seconds, "direct_membership": direct_seconds,
                                 "cofactor_clear_and_subgroup_check": clearing_seconds,
                                 "pair_table": pair_seconds,
                                 "three_sum_image": image_seconds,
                                 "two_target_queries": target_seconds},
                  peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
except TimeoutError as exc:
    result.update(status="censored_timeout", error=str(exc))
finally:
    signal.alarm(0)
    save()
print(json.dumps(result, indent=2, sort_keys=True))
