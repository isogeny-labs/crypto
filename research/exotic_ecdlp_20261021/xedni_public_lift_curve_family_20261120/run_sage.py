#!/usr/bin/env python3
"""Frozen public-point lift panel on selected short Weierstrass curves."""

import hashlib
import importlib.util
import json
import os
import platform
import resource
import signal
import time
from math import gcd, isqrt
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def timeout(_sig, _frame):
    raise TimeoutError("per-arm wall cap")


def lift_search(target, p, t, cap):
    xt, yt = target
    b_steps = pairs = tests = roots = 0
    H = 0
    start = time.monotonic_ns()
    try:
        for H in range(1, cap + 1):
            for b in range(1, H + 1):
                b_steps += 1
                if b % p == 0:
                    continue
                residue = xt * b * b % p
                if b == H:
                    aa = range(-H + (residue + H) % p, H + 1, p)
                else:
                    aa = (a for a in (-H, H) if a % p == residue)
                for a in aa:
                    pairs += 1
                    if gcd(abs(a), b) != 1:
                        continue
                    tests += 1
                    C = a**3 + t * a * b**4 + b**6
                    if C < 0:
                        continue
                    c0 = isqrt(C)
                    if c0 * c0 != C:
                        continue
                    for c in ((c0,) if c0 == 0 else (c0, -c0)):
                        roots += 1
                        if c * pow(b, -3, p) % p == yt:
                            assert c*c == C
                            assert a * pow(b, -2, p) % p == xt
                            stop = time.monotonic_ns()
                            return {"status": "found", "height": H, "lift": [a, b, c],
                                    "b_steps": b_steps, "residue_pairs": pairs,
                                    "exact_tests": tests, "rational_roots": roots,
                                    "start_ns": start, "stop_ns": stop, "online_ns": stop-start}
    except TimeoutError:
        stop = time.monotonic_ns()
        return {"status": "wall_cap_exhausted", "height_reached": H, "lift": None,
                "b_steps": b_steps, "residue_pairs": pairs,
                "exact_tests": tests, "rational_roots": roots,
                "start_ns": start, "stop_ns": stop, "online_ns": stop-start}
    stop = time.monotonic_ns()
    return {"status": "height_cap_exhausted", "height": cap, "lift": None,
            "b_steps": b_steps, "residue_pairs": pairs,
            "exact_tests": tests, "rational_roots": roots,
            "start_ns": start, "stop_ns": stop, "online_ns": stop-start}


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    rho_path = ROOT / protocol["rho_source"]
    assert sha(rho_path.read_bytes()) == protocol["rho_source_sha256"]
    spec = importlib.util.spec_from_file_location("xedni_curve_family_rho", rho_path)
    source = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(source)
    signal.signal(signal.SIGALRM, timeout)
    selection, fixtures, rows = [], [], []
    for p in protocol["primes"]:
        F = GF(p)
        selected = 0
        for t in range(1, 65):
            if (4*t**3 + 27) % p == 0:
                selection.append({"p": p, "t": t, "status": "singular"})
                continue
            Ep = EllipticCurve(F, [0, 0, 0, F(t), F(1)])
            q = int(Ep.cardinality())
            if not Ep.cardinality().is_prime():
                selection.append({"p": p, "t": t, "q": q, "status": "composite_order"})
                continue
            G = Ep([F(0), F(1)])
            if int(G.order()) != q:
                selection.append({"p": p, "t": t, "q": q, "status": "nongenerator"})
                continue
            selection.append({"p": p, "t": t, "q": q, "status": "selected"})
            selected += 1
            specs = [("control", k, None) for k in protocol["control_scalars"]]
            seen = set(protocol["control_scalars"])
            for j in range(protocol["seeded_targets_per_curve"]):
                digest = sha(f"{protocol['seed']}:{p}:{t}:{j}".encode())
                k = 1 + int(digest, 16) % (q-1)
                assert k not in seen
                seen.add(k)
                specs.append(("seeded", k, digest))
            for index, (kind, k, digest) in enumerate(specs):
                T = k * G
                target = [int(T[0]), int(T[1])]
                fixture = {"p": p, "t": t, "q": q, "target_index": index,
                           "input_kind": kind, "fixture_scalar": k,
                           "target": target, "digest": digest}
                fixtures.append(fixture)
                order = ("lift", "rho") if index % 2 == 0 else ("rho", "lift")
                for position, arm in enumerate(order, 1):
                    seconds = (protocol["lift_wall_cap_seconds"] if arm == "lift"
                               else protocol["rho_wall_cap_seconds"])
                    signal.setitimer(signal.ITIMER_REAL, seconds)
                    try:
                        if arm == "lift":
                            r = lift_search(target, p, t, protocol["height_cap"])
                        else:
                            r = source.rho_solve(Ep, G, T, q, p, index,
                                                 f"{protocol['seed']}:{t}")
                    except Exception as e:
                        r = {"status": "error", "error": type(e).__name__, "detail": str(e)}
                    finally:
                        signal.setitimer(signal.ITIMER_REAL, 0)
                    if arm == "rho" and r.get("status") == "verified":
                        assert r["scalar"] == k and r["scalar"] * G == T
                    if arm == "lift" and kind == "control":
                        assert r["status"] == "found"
                    rows.append({"p": p, "t": t, "q": q, "target_index": index,
                                 "input_kind": kind, "target": target,
                                 "position": position, "arm": arm, **r})
                    (HERE / "progress.jsonl").write_text("".join(
                        json.dumps(x, sort_keys=True) + "\n" for x in rows))
            if selected == protocol["selected_per_prime"]:
                break
        assert selected == protocol["selected_per_prime"]
    outputs = {"selection-rows.jsonl": selection, "target-fixtures.jsonl": fixtures,
               "arm-rows.jsonl": rows}
    hashes = {}
    for name, content in outputs.items():
        raw = "".join(json.dumps(x, sort_keys=True) + "\n" for x in content).encode()
        (HERE / name).write_bytes(raw)
        hashes[name] = sha(raw)
    good = all(x["status"] in ("found", "height_cap_exhausted", "wall_cap_exhausted")
               if x["arm"] == "lift" else x["status"] == "verified" for x in rows)
    return {"schema": "xedni-public-lift-curve-family-result-v1",
            "status": "PASS" if good else "PARTIAL",
            "protocol_sha256": sha(protocol_raw),
            "script_sha256": sha((HERE / "run_sage.py").read_bytes()),
            "rho_source_sha256": sha(rho_path.read_bytes()),
            "output_hashes": hashes, "selected_curves": len(fixtures)//5,
            "targets": len(fixtures), "arms": len(rows),
            "lift_found": sum(x["arm"] == "lift" and x["status"] == "found" for x in rows),
            "lift_height_cap_exhausted": sum(x["arm"] == "lift" and x["status"] == "height_cap_exhausted" for x in rows),
            "lift_wall_cap_exhausted": sum(x["arm"] == "lift" and x["status"] == "wall_cap_exhausted" for x in rows),
            "rho_verified": sum(x["arm"] == "rho" and x["status"] == "verified" for x in rows),
            "host": {"system": platform.system(), "machine": platform.machine(),
                     "cpu_count": os.cpu_count(),
                     "process_max_rss_bytes": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
