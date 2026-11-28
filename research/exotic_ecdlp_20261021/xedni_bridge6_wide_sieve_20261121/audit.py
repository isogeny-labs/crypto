"""Independent factor-product, specialization, and first-prime audit."""

import hashlib
import json
import re
from collections import Counter, defaultdict
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
SURFACE = HERE.parent / "xedni_bridge6_lattice_surface_20261121"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def terms(folder):
    raw = b"".join(p.read_bytes() for p in sorted((SURFACE / folder).glob("[0-9][0-9][0-9].jsonl")))
    return {tuple(row["exponents"]): int(row["coefficient"])
            for row in map(json.loads, raw.splitlines())}


def product(a, b):
    c = defaultdict(int)
    for (i, j, k), x in a.items():
        for (l, m, n), y in b.items():
            c[(i+l, j+m, k+n)] += x*y
    return {e: x for e, x in c.items() if x}


def specialize(poly, u, v):
    c = defaultdict(int)
    for (i, j, k), x in poly.items():
        c[k] += x * (u**i) * (v**j)
    c = {k: x for k, x in c.items() if x}
    assert c
    content = gcd(*c.values())
    sign = 1 if c[max(c)] > 0 else -1
    return [sign*c.get(k, 0)//content for k in range(max(c)+1)]


def is_prime(n):
    return n >= 2 and all(n % d for d in range(2, int(n**0.5)+1))


def has_root(coeffs, p):
    # Independent explicit-power evaluation, distinct from the producer's Horner loop.
    return any(sum((c % p)*pow(t, i, p) for i, c in enumerate(coeffs)) % p == 0
               for t in range(p))


def main():
    a = terms("factor-00-terms")
    b = terms("factor-01-terms")
    n = terms("numerator-terms")
    assert product(a, b) == n
    files = sorted((HERE / "rows").glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(f.read_bytes() for f in files)
    expected = json.loads((HERE / "result.json").read_text())
    assert len(files) == expected["cells"] == 441
    assert sha(raw) == expected["rows_raw_sha256"]
    counts = Counter()
    prime_counts = Counter()
    for index, file in enumerate(files):
        assert file.name == f"{index:03d}.jsonl"
        row = json.loads(file.read_text())
        u, v = divmod(index, 21)
        assert (row["u"], row["v"]) == (u-10, v-10)
        coeffs = [int(c) for c in row["coefficients"]]
        assert coeffs == specialize(n, row["u"], row["v"])
        p = row["certificate_prime"]
        for q in range(13, (p or 257)+1):
            if is_prime(q):
                assert has_root(coeffs, q) == (p is None or q < p)
        assert row["status"] == ("excluded_all_integer_t" if p else "unresolved")
        counts[row["status"]] += 1
        if p:
            prime_counts[p] += 1
    assert dict(counts) == expected["counts"]
    assert {str(k): v for k, v in sorted(prime_counts.items())} == expected["certificate_prime_counts"]
    survivor = json.loads((HERE / "survivor-result.json").read_text())
    assert survivor["cell"] == [-5, -8]
    assert survivor["extended_certificate_prime"] == 293
    coeffs = [int(c) for c in json.loads((HERE / "rows" / "107.jsonl").read_text())["coefficients"]]
    assert all(has_root(coeffs, q) for q in range(13, 293) if is_prime(q))
    assert not has_root(coeffs, 293)
    lean_raw = (HERE / "formal" / "WideSieve.lean").read_bytes()
    lean_receipt = json.loads((HERE / "formal" / "source-result.json").read_text())
    assert sha(lean_raw) == lean_receipt["lean_sha256"]
    lean = lean_raw.decode()
    definitions = re.findall(r"def coeffs(\d{3}) : List ℤ := \[\n(.*?)\n\]", lean, re.S)
    certificates = re.findall(r"theorem rootless(\d{3}) : ∀ z : ZMod (\d+)", lean)
    assert len(definitions) == len(certificates) == 441
    for i, ((label, block), (label2, prime)) in enumerate(zip(definitions, certificates)):
        assert label == label2 == f"{i:03d}"
        row = json.loads(files[i].read_text())
        assert [int(x) for x in re.findall(r"-?\d+", block)] == [int(x) for x in row["coefficients"]]
        assert int(prime) == (row["certificate_prime"] or 293)
    output = (HERE / "formal" / "build-output.txt").read_text().splitlines()
    assert len(output) == 882 and all("depends on axioms: [propext, Classical.choice, Quot.sound]" in x
                                      for x in output)
    assert all("sorryAx" not in x and "error:" not in x for x in output)
    assert not (HERE / "formal" / "build-stderr.txt").read_text()
    print(json.dumps({"schema": "xedni-bridge6-wide-sieve-audit-v1",
                      "status": "PASS", "factor_product": True,
                      "specialized_cells": len(files), "first_prime_certificates": sum(prime_counts.values()),
                      "extended_certificate": {"cell": [-5, -8], "prime": 293},
                      "lean_polynomial_bindings": 441,
                      "lean_checked_theorems": 882,
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
