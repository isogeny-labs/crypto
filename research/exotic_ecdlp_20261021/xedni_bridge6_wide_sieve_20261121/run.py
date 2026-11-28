"""Frozen full-numerator modular sieve over 441 coefficient classes."""

import hashlib
import json
from collections import Counter, defaultdict
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
SURFACE = HERE.parent / "xedni_bridge6_lattice_surface_20261121"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def primes_between(lo, hi):
    for p in range(lo, hi + 1):
        if p > 1 and all(p % q for q in range(2, int(p**0.5) + 1)):
            yield p


def read_terms():
    raw = b"".join(p.read_bytes() for p in sorted((SURFACE / "numerator-terms").glob("[0-9][0-9][0-9].jsonl")))
    return [(tuple(row["exponents"]), int(row["coefficient"]))
            for row in map(json.loads, raw.splitlines())], raw


def specialize(terms, u, v):
    coeffs = defaultdict(int)
    for (eu, ev, et), c in terms:
        coeffs[et] += c * u**eu * v**ev
    coeffs = {e: c for e, c in coeffs.items() if c}
    if not coeffs:
        return None
    content = gcd(*coeffs.values())
    coeffs = {e: c // content for e, c in coeffs.items()}
    if coeffs[max(coeffs)] < 0:
        coeffs = {e: -c for e, c in coeffs.items()}
    return [coeffs.get(i, 0) for i in range(max(coeffs) + 1)]


def roots_mod(coeffs, p):
    residues = [c % p for c in coeffs]
    roots = []
    for t in range(p):
        value = 0
        for c in reversed(residues):
            value = (value * t + c) % p
        if value == 0:
            roots.append(t)
    return roots


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    terms, raw = read_terms()
    assert sha(raw) == protocol["source_numerator_sha256"]
    rows = []
    counts = Counter()
    prime_counts = Counter()
    for u in range(-10, 11):
        for v in range(-10, 11):
            row = {"u": u, "v": v, "status": "producer_failure"}
            try:
                coeffs = specialize(terms, u, v)
                if coeffs is None:
                    row["error"] = "zero_specialization"
                else:
                    prime = next((p for p in primes_between(13, 257)
                                  if not roots_mod(coeffs, p)), None)
                    row.update({"status": "excluded_all_integer_t" if prime else "unresolved",
                                "coefficients": [str(c) for c in coeffs],
                                "certificate_prime": prime})
                    if prime:
                        prime_counts[prime] += 1
            except Exception as exc:
                row["error"] = repr(exc)
            rows.append(row)
            counts[row["status"]] += 1
    outdir = HERE / "rows"
    outdir.mkdir(exist_ok=True)
    raw_rows = b""
    for i, row in enumerate(rows):
        line = (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode()
        assert len(line) < 48000
        (outdir / f"{i:03d}.jsonl").write_bytes(line)
        raw_rows += line
    print(json.dumps({"schema": "xedni-bridge6-wide-sieve-result-v1",
                      "status": "PASS" if not counts["producer_failure"] else "PARTIAL",
                      "cells": len(rows), "counts": dict(counts),
                      "certificate_prime_counts": dict(sorted(prime_counts.items())),
                      "source_numerator_sha256": sha(raw),
                      "rows_raw_sha256": sha(raw_rows)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
