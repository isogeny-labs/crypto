#!/usr/bin/env python3
"""Independent complete replay of the Lean height-4096 modular sieve."""

import hashlib
import json
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol-4096.json").read_text())
    original = json.loads((HERE / protocol["source_instance_protocol"]).read_text())
    assert original["instance"]["public_target"] == protocol["instance"]["public_target"]
    raw = (ROOT / original["source_fixtures"]).read_bytes()
    assert sha(raw) == original["source_fixtures_sha256"]
    fixture = [json.loads(s) for s in raw.splitlines() if
               (json.loads(s)["p"],json.loads(s)["t"],json.loads(s)["k"]) == (1009,9,3)]
    assert len(fixture) == 1 and fixture[0]["target"] == [744,244]
    p,t,xt,cap = 1009,9,744,4096
    primes = protocol["witness_primes"]
    squares = {m: {z*z%m for z in range(m)} for m in primes}
    counts = {"x_compatible":0,"noncoprime":0,"negative_rhs":0,"modular_nonsquare":0}
    witness_counts = {str(m):0 for m in primes}
    digest = hashlib.sha256()
    for b in range(1,cap+1):
        for a in range(-cap,cap+1):
            if (a-xt*b*b)%p:
                continue
            counts["x_compatible"] += 1
            if gcd(abs(a),b) != 1:
                counts["noncoprime"] += 1
                label = "noncoprime"
            else:
                C = a**3+t*a*b**4+b**6
                if C < 0:
                    counts["negative_rhs"] += 1
                    label = "negative_rhs"
                else:
                    witness = next((m for m in primes if C%m not in squares[m]),None)
                    assert witness is not None,(a,b,C)
                    counts["modular_nonsquare"] += 1
                    witness_counts[str(witness)] += 1
                    label = str(witness)
            digest.update(f"{a},{b},{label}\n".encode())
    assert sum(counts[k] for k in ("noncoprime","negative_rhs","modular_nonsquare")) == counts["x_compatible"]
    return {"schema":"xedni-lift-full-certificate-audit-v1","status":"PASS",
            "protocol_sha256":sha((HERE/"protocol-4096.json").read_bytes()),
            "counts":counts,"witness_counts":witness_counts,
            "ordered_candidate_labels_sha256":digest.hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
