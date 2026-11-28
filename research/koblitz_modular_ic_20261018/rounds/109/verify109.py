#!/usr/bin/env python3
"""Independent Hasse evaluation and schoolbook rational inversion for N51."""
import hashlib
import importlib.util
import json
import math
from pathlib import Path

root = Path(__file__).resolve().parent
paths = [root.parent / "51" / "runs" / "base23-struct51-03" / "marked23-triangular51.sing",
         root.parent / "102" / "audit102.py", root.parent / "107" / "result.json",
         root.parent / "108" / "result.json"]
r = json.loads((root / "result.json").read_text())
for path, expected in zip(paths, r["source_sha256"]):
    assert hashlib.sha256(path.read_bytes()).hexdigest() == expected


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


producer = load("norm109", root / "norm109.py")
audit = load("audit102", paths[1])
terms = producer.parse_terms(paths[0].read_text(), audit)
base = json.loads(paths[3].read_text())
mask = (1 << 128) - 1


def mul(a, b):
    out = 0
    for i in range(128):
        if b >> i & 1: out ^= a << i
    return out & mask


def pow2(a, n):
    out = 1
    for _ in range(n): out = mul(out, a)
    return out


def eval_xpoly(bits, t):
    out = 0
    powers = [1]
    for _ in range(1, 128): powers.append(mul(powers[-1], t))
    for e in range(bits.bit_length()):
        if not bits >> e & 1: continue
        for k in range(min(e, 127) + 1):
            if math.comb(e, k) & 1: out ^= powers[k]
    return out


def val(a):
    return None if not a else (a & -a).bit_length() - 1


def inverse(q, width):
    assert q & 1
    inv = 1
    for k in range(1, width):
        bit = 0
        for j in range(1, k + 1):
            bit ^= (q >> j & 1) & (inv >> (k-j) & 1)
        if bit: inv |= 1 << k
    assert mul(q, inv) & ((1 << width) - 1) == 1
    return inv


def alpha_power(n):
    return (1, 2, 3)[n % 3]


for key, shift in (("y0", 0), ("y1", 1)):
    z = sum(1 << i for i in base[key]["z_exponents"])
    t, y = mul(2, z), z ^ shift
    sums = {i: {} for i in range(15)}
    bounds = {i: 128 for i in range(15)}
    for le, ye, pbits, qbits in terms:
        a = mul(eval_xpoly(pbits, t), pow2(y, ye))
        b = eval_xpoly(qbits, t)
        n, d = val(a), val(b)
        assert n is not None and d is not None
        width = 128 - max(n, d)
        ratio = mul(a >> n, inverse(b >> d, width)) & ((1 << width) - 1)
        offset, until = n-d, n-d+width
        for j in range(le+1):
            if j & ~le: continue
            bounds[j] = min(bounds[j], until)
            scalar = alpha_power(le-j)
            for k in range(width):
                if ratio >> k & 1:
                    e=offset+k;sums[j][e]=sums[j].get(e,0)^scalar
    for row in r[key]["coefficient_rows"]:
        j=row["u_exponent"]
        exact=sorted(e for e,v in sums[j].items() if e<bounds[j] and v)
        assert row["known_until_exclusive"]==bounds[j]
        assert row["nonzero_support_below_bound"]==exact
        assert row["s_order"]==(exact[0] if exact else None)
        if exact: assert row["leading_F4"]==sums[j][exact[0]]
    assert r[key]["degree_one"]["certified"] and r[key]["degree_one"]["valuation"]==0
    assert not r[key]["degree_seven"]["certified"]
    assert r[key]["degree_seven"]["tied_u_exponents"]==[0,7]
    assert r[key]["degree_seven"]["leading_F4_sum"]==0

print("NORM109_INDEPENDENT_HASSE_PASS")
