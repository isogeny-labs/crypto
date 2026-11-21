"""Census tangent-compatible local a4 witnesses in the same-point stratum."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
GATES = ROOT / "xedni_local_collision_crt_20261120" / "run.py"
PRIOR = ROOT / "xedni_full_law_crt_20261120" / "run.py"
spec = importlib.util.spec_from_file_location("collision_gates", GATES)
gates = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gates)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def valuation(n, p):
    if n == 0:
        return None
    n = abs(n)
    v = 0
    while n % p == 0:
        n //= p
        v += 1
    return v


def predicted_a4(points, p, a):
    (x1, y1), (x2, y2) = points
    dx, dy = x1 - x2, y1 - y2
    assert dx % p == dy % p == 0
    u, v = dx // p, dy // p
    assert u % p != 0
    a1, a2, a3 = a[:3]
    n = (v * (y1+y2) + a1*(x1*v+y2*u) + a3*v
         -u*(x1*x1+x1*x2+x2*x2) -a2*u*(x1+x2))
    return n * pow(u, -1, p) % p


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    rows, counts = [], Counter()
    for index, old in enumerate(source):
        points = [tuple(P) for P in old["lifted_points"]]
        (x1, y1), (x2, y2) = points
        if x1 == x2:
            continue
        p = 7 if (x1-x2) % 7 == 0 else (11 if (x1-x2) % 11 == 0 else None)
        if p is None or (y1-y2) % p != 0:
            continue
        vp_dx = valuation(x1-x2, p)
        vp_dy = valuation(y1-y2, p)
        assert vp_dx == 1 and (vp_dy is None or vp_dy >= 1)
        local = {}
        for q, order in ((7, 4), (11, 6)):
            qpoints = tuple(tuple(z % q for z in P) for P in points)
            local[q] = (gates.collided_gate(qpoints, q, order)[0] if q == p
                        else gates.prior.local_gate(qpoints, q, order)[0])
        other = 11 if p == 7 else 7
        compatible_local = sum(predicted_a4(points, p, a) == a[3]
                               for a in local[p])
        compatible = compatible_local * len(local[other])
        total = len(local[p]) * len(local[other])
        row = {"pair_index": index, "prime": p, "v_dx": vp_dx, "v_dy": vp_dy,
               "gate_at_collision": len(local[p]), "gate_at_other": len(local[other]),
               "local_pairs": total, "compatible": compatible,
               "incompatible": total-compatible}
        rows.append(row)
        counts["source_pairs"] += 1
        counts["local_pairs"] += total
        counts["compatible"] += compatible
        counts["incompatible"] += total-compatible
        counts[f"source_pairs_p{p}"] += 1
    print(json.dumps({"schema": "xedni-same-point-invariance-result-v1",
                      "source_sha256": sha(source_bytes),
                      "protocol_sha256": sha(protocol_bytes),
                      "script_sha256": sha((HERE / "screen.py").read_bytes()),
                      "gate_script_sha256": sha(GATES.read_bytes()),
                      "prior_script_sha256": sha(PRIOR.read_bytes()),
                      "counts": dict(counts), "rows": rows}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
