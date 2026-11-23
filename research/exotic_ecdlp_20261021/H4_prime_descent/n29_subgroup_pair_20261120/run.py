#!/usr/bin/env python3
"""Frozen H4 n29 subgroup pair-index and fresh-target lookup diagnostic."""
from sage.all import *
import hashlib, json, random, resource, signal, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
assert hashlib.sha256((ROOT / SPEC["source"]).read_bytes()).hexdigest() == SPEC["source_sha256"]
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def enc(e): return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))
def record(P): return [enc(P[0]), enc(P[1])]
def save():
    (HERE / "result.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
def timeout(*_): raise TimeoutError("frozen 300-second process cap")
signal.signal(signal.SIGALRM, timeout)
signal.alarm(SPEC["caps"]["process_seconds"])
out = {"schema": "h4-n29-subgroup-pair-result-v1", "status": "running",
       "protocol_sha256": digest(HERE / "protocol.json"),
       "runner_sha256": digest(HERE / "run.py"), "stages": {}, "targets": []}
save()
t0 = time.monotonic_ns()
try:
    n, idx = 29, 0
    rng = random.Random(20261022 * 1000 + n * 10 + idx)
    set_random_seed(20261022 + n * 10 + idx)
    mod = PolynomialRing(GF(2), "x").irreducible_element(n)
    K = GF(2**n, "z", modulus=mod)
    poly = sum(int(c) << i for i, c in enumerate(mod.list()))
    def elt(v): return K([(v >> i) & 1 for i in range(n)])
    a2 = elt(rng.getrandbits(n)); a6 = elt(rng.getrandbits(n))
    while a6 == 0: a6 = elt(rng.getrandbits(n))
    E = EllipticCurve(K, [1, a2, 0, 0, a6])
    while True:
        basis = [rng.getrandbits(n) for _ in range(10)]
        rows = list(basis); rank = 0
        for bit in range(n):
            pivot = next((v for v in rows if (v >> bit) & 1), None)
            if pivot is None: continue
            rows.remove(pivot); rank += 1
            rows = [v ^ pivot if (v >> bit) & 1 else v for v in rows]
        if rank == 10: break
    order = int(E.cardinality()); fac = [(int(p), int(e)) for p,e in factor(order)]
    r = max(p for p,e in fac); h = order // r
    assert r >= 2**20 and is_prime(r)
    out.update(field_modulus=poly, curve_a2=enc(a2), curve_a6=enc(a6),
               subspace_basis=basis, group_order=order, order_factorization=fac,
               subgroup_order=r, cofactor=h)
    out["stages"]["instance_ns"] = time.monotonic_ns()-t0; save()
    t = time.monotonic_ns(); raw = []
    for bits in range(1 << 10):
        x = 0
        for j,v in enumerate(basis):
            if (bits >> j) & 1: x ^= v
        raw.extend(E.lift_x(elt(x), all=True))
    zero = E(0)
    assert len(raw) == len(set(raw))
    direct = sum(r*P == zero for P in raw)
    cleared_all = [h*P for P in raw]
    base = sorted(set(cleared_all)-{zero}, key=record)
    assert all(r*P == zero for P in base)
    G = next(P for P in cleared_all if P != zero)
    assert r*G == zero
    out.update(raw_count=len(raw), direct_subgroup_points=direct,
               cleared_identity_images=cleared_all.count(zero),
               cleared_duplicate_images=len(cleared_all)-len(set(cleared_all)),
               base_count=len(base), base_sha256=hashlib.sha256(json.dumps([record(P) for P in base],separators=(",",":")).encode()).hexdigest(),
               generator=record(G))
    out["stages"]["base_ns"] = time.monotonic_ns()-t; save()
    t = time.monotonic_ns(); first = {}
    for i,A in enumerate(base):
        for j,B in enumerate(base):
            S = A+B
            if S not in first: first[S] = (i,j)
    out.update(pair_attempts=len(base)**2, distinct_pair_sums=len(first))
    out["stages"]["index_ns"] = time.monotonic_ns()-t; save()
    for i in range(32):
        label = f"h4-n29-target-{i}"
        scalar = int.from_bytes(hashlib.sha256(label.encode()).digest(),"big")%(r-1)+1
        Q = scalar*G
        t = time.monotonic_ns(); witness = None; attempts = 0
        for k,C in enumerate(base):
            attempts += 1
            ij = first.get(Q-C)
            if ij is not None:
                a,b = ij
                if base[a]+base[b]+C == Q:
                    witness = [a,b,k]; break
        elapsed = time.monotonic_ns()-t
        row = {"index":i, "label":label, "fixture_scalar":scalar,
               "target":record(Q), "status":"verified" if witness else "exhausted",
               "attempts":attempts, "lookup_wall_ns":elapsed, "witness_indices":witness}
        out["targets"].append(row); save()
    out["status"] = "complete"
except Exception as e:
    out["status"] = "timeout" if isinstance(e,TimeoutError) else "error"
    out["error"] = f"{type(e).__name__}: {e}"
finally:
    signal.alarm(0)
    out["total_wall_ns"] = time.monotonic_ns()-t0
    out["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save()
    print(json.dumps({k:out.get(k) for k in ("status","error","group_order","subgroup_order","base_count","distinct_pair_sums","peak_rss_bytes")},sort_keys=True))
