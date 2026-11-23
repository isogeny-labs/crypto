#!/usr/bin/env python3
"""Frozen n23 H4 subgroup base and one-target pair-index diagnostic."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path
import random
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source"]
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]

def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))

def point_record(P):
    return [to_int(P[0]), to_int(P[1])]

def save(result):
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

def timeout(*_):
    raise TimeoutError("frozen 180-second n23 diagnostic cap")

signal.signal(signal.SIGALRM, timeout)
signal.alarm(SPEC["cap_seconds"])
result = {"schema": "h4-n23-subgroup-pair-result-v1", "status": "running",
          "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
          "source_sha256": SPEC["source_sha256"], "stages": {}}
save(result)
try:
    inst = SPEC["instance"]
    n, idx, l = inst["field_degree"], inst["instance_index"], inst["subspace_dimension"]
    r, h = SPEC["prime_subgroup_order"], SPEC["cofactor"]
    t = time.monotonic()
    rng = random.Random(20261022 * 1000 + n * 10 + idx)
    set_random_seed(20261022 + n * 10 + idx)
    mod = PolynomialRing(GF(2), "x").irreducible_element(n)
    K = GF(2**n, "z", modulus=mod)
    poly = sum(int(c) << i for i, c in enumerate(mod.list()))
    def elt(v):
        return K([(v >> i) & 1 for i in range(n)])
    a2 = elt(rng.getrandbits(n))
    a6 = elt(rng.getrandbits(n))
    while a6 == 0:
        a6 = elt(rng.getrandbits(n))
    E = EllipticCurve(K, [1, a2, 0, 0, a6])
    while True:
        basis = [rng.getrandbits(n) for _ in range(l)]
        rows = list(basis)
        rank = 0
        for bit in range(n):
            piv = None
            for row in rows:
                if (row >> bit) & 1:
                    piv = row
                    break
            if piv is None:
                continue
            rows.remove(piv)
            rank += 1
            rows = [row ^ piv if (row >> bit) & 1 else row for row in rows]
        if rank == l:
            break
    order = int(E.cardinality())
    assert order == SPEC["known_order_from_screen"] == r * h
    result.update(field_modulus=poly, curve_a2=to_int(a2), curve_a6=to_int(a6),
                  subspace_basis=basis, group_order=order, prime_subgroup_order=r,
                  cofactor=h)
    result["stages"]["curve_and_subspace_fixture_seconds"] = time.monotonic() - t
    save(result)

    t = time.monotonic()
    raw = []
    for bits in range(1 << l):
        x = 0
        for j, v in enumerate(basis):
            if (bits >> j) & 1:
                x ^= v
        raw.extend(E.lift_x(elt(x), all=True))
    assert len(raw) == len(set(raw))
    zero = E(0)
    result["raw_lifted_points"] = len(raw)
    result["raw_points_sha256"] = hashlib.sha256(json.dumps(
        [point_record(P) for P in raw], separators=(",", ":")).encode()).hexdigest()
    result["stages"]["lift_base_seconds"] = time.monotonic() - t
    save(result)

    t = time.monotonic()
    direct = [P for P in raw if r * P == zero]
    result["direct_subgroup_points"] = len(direct)
    result["stages"]["direct_membership_seconds"] = time.monotonic() - t
    save(result)

    t = time.monotonic()
    cleared_all = [h * P for P in raw]
    cleared = sorted(set(cleared_all) - {zero}, key=lambda P: point_record(P))
    assert cleared
    assert all(r * P == zero for P in cleared)
    H = next(P for P in cleared_all if P != zero)
    assert r * H == zero
    result.update(cleared_identity_images=cleared_all.count(zero),
                  cleared_duplicate_images=len(cleared_all)-len(set(cleared_all)),
                  cleared_nonidentity_points=len(cleared),
                  cleared_points_sha256=hashlib.sha256(json.dumps(
                      [point_record(P) for P in cleared], separators=(",", ":")).encode()).hexdigest(),
                  generator=point_record(H))
    result["stages"]["cofactor_clear_seconds"] = time.monotonic() - t
    save(result)

    t = time.monotonic()
    first_pair = {}
    for A in cleared:
        for B in cleared:
            first_pair.setdefault(A + B, (A, B))
    result.update(pair_entries=len(cleared)**2, distinct_pair_sums=len(first_pair))
    result["stages"]["pair_index_build_seconds"] = time.monotonic() - t
    save(result)

    label = b"h4-n23-subgroup-target-0"
    scalar = int.from_bytes(hashlib.sha256(label).digest(), "big") % (r - 1) + 1
    target = scalar * H
    assert target != zero and r * target == zero
    result.update(target_scalar_fixture=scalar, target=point_record(target))
    t = time.monotonic()
    witness = None
    attempts = 0
    for C in cleared:
        attempts += 1
        pair = first_pair.get(target - C)
        if pair is not None:
            A, B = pair
            assert A + B + C == target
            witness = [point_record(A), point_record(B), point_record(C)]
            break
    result.update(target_lookup_status="verified_witness" if witness else "zero_yield",
                  target_lookup_attempts=attempts, target_witness=witness)
    result["stages"]["one_target_lookup_seconds"] = time.monotonic() - t
    save(result)

    t = time.monotonic()
    scalar_of = {}
    current = zero
    for k in range(r):
        assert current not in scalar_of
        scalar_of[current] = k
        current += H
    assert current == zero and len(scalar_of) == r
    base_scalars = [scalar_of[P] for P in cleared]
    pair_counts = [0] * r
    for a in base_scalars:
        for b in base_scalars:
            pair_counts[(a + b) % r] += 1
    triple_counts = [0] * r
    for ab, multiplicity in enumerate(pair_counts):
        if multiplicity:
            for c in base_scalars:
                triple_counts[(ab + c) % r] += multiplicity
    assert sum(pair_counts) == len(cleared)**2
    assert sum(triple_counts) == len(cleared)**3
    assert sum(v > 0 for v in pair_counts) == len(first_pair)
    assert (triple_counts[scalar] > 0) == (witness is not None)
    result.update(status="complete", subgroup_enumerated_points=len(scalar_of),
                  exact_ordered_triples=sum(triple_counts),
                  exact_pair_sum_targets=sum(v > 0 for v in pair_counts),
                  exact_three_sum_targets=sum(v > 0 for v in triple_counts),
                  exact_nonidentity_three_sum_targets=sum(v > 0 for v in triple_counts[1:]),
                  exact_nonidentity_coverage_denominator=r - 1,
                  exact_nonidentity_coverage_decimal=sum(v > 0 for v in triple_counts[1:])/(r-1),
                  target_ordered_triples=triple_counts[scalar],
                  multiplicity_histogram={str(k): v for k, v in sorted(
                      Counter(v for v in triple_counts if v > 0).items())},
                  peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
    result["stages"]["scalar_coordinate_audit_seconds"] = time.monotonic() - t
except TimeoutError as exc:
    result.update(status="censored_timeout", error=str(exc),
                  peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
finally:
    signal.alarm(0)
    save(result)
print(json.dumps({k:v for k,v in result.items() if k!="multiplicity_histogram"}, indent=2, sort_keys=True))
