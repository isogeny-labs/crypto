#!/usr/bin/env python3
"""Independent bit-polynomial arithmetic replay of R105 source parsing."""
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DONOR = ROOT.parent / "51" / "runs" / "base23-struct51-03" / "marked23-triangular51.sing"
RESULT = ROOT / "result.json"
spec = importlib.util.spec_from_file_location("jets105", ROOT / "jets105.py")
producer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(producer)
assert hashlib.sha256(DONOR.read_bytes()).hexdigest() == producer.SOURCE_SHA256
f_terms, g_terms = producer.parse_source(DONOR.read_text())


def mulmod(a, b, width):
    out = 0
    for i in range(width):
        if b >> i & 1:
            out ^= a << i
    return out & ((1 << width) - 1)


def powmod(a, n, width):
    out = 1
    while n:
        if n & 1:
            out = mulmod(out, a, width)
        a = mulmod(a, a, width)
        n >>= 1
    return out


def toggled(s, item):
    if item in s: s.remove(item)
    else: s.add(item)


g_support = [set() for _ in range(5)]
for le, ye, den, exps in g_terms:
    inverse = powmod(31, den, 5)  # (1+t)^-1 modulo t^5
    coeff = 0
    for e in exps:
        coeff ^= mulmod(powmod(3, e, 5), inverse, 5)
    for k in range(5):
        if coeff >> k & 1:
            toggled(g_support[k], (le, ye))

f_support = []
for shift in (0, 1):
    support = set()
    for ye, exps in f_terms:
        zpoly = powmod(3, ye, 8) if shift else 1 << ye
        for e in exps:
            tpoly = powmod(3, e, 8)
            for k in range(8):
                if tpoly >> k & 1:
                    for j in range(8):
                        if zpoly >> j & 1:
                            toggled(support, (k, j))
    f_support.append(sorted(support))

r = json.loads(RESULT.read_text())
assert r["source_sha256"] == producer.SOURCE_SHA256
assert r["G_full_jets"] == [sorted(map(list, s)) for s in g_support]
assert r["f_y0_support"] == [list(x) for x in f_support[0]]
assert r["f_y1_support"] == [list(x) for x in f_support[1]]
assert g_support[1] == g_support[2] == g_support[3] == set()
assert g_support[4] == {(l, y) for l in (1, 4) for y in (3, 4, 5, 6)}
print("JETS105_INDEPENDENT_BIT_ARITHMETIC_PASS")
