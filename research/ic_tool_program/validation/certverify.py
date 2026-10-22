#!/usr/bin/env python3
"""Independent scalar verifier for `ic price --single-target` records.

Shares no code with the repository: its own GF(2^m) arithmetic and its own
group law on the Koblitz curve  y^2 + xy = x^3 + a*x^2 + 1.  For every `ic`
and `rho` certificate in every `*.price.json` it checks

    [scalar] G == target,   [n] G == O,   [scalar + 1] G != target

(the last is a negative control: the checker must be able to fail).

    certverify.py PATH... [--out FILE]

A PATH is a directory (searched for `*.price.json`) or a `.tar.xz` holding
them.  Stdlib only.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tarfile
from pathlib import Path


class Field:
    def __init__(self, m: int, low_terms: list[int]):
        self.m = m
        self.mod = (1 << m)
        for t in low_terms:
            self.mod |= 1 << t

    def reduce(self, x: int) -> int:
        m, mod = self.m, self.mod
        while x.bit_length() > m:
            x ^= mod << (x.bit_length() - 1 - m)
        return x

    def mul(self, a: int, b: int) -> int:
        r = 0
        while b:
            if b & 1:
                r ^= a
            a <<= 1
            b >>= 1
        return self.reduce(r)

    def inv(self, a: int) -> int:
        if a == 0:
            raise ZeroDivisionError
        # a^(2^m - 2)
        r, e, base = 1, (1 << self.m) - 2, a
        while e:
            if e & 1:
                r = self.mul(r, base)
            base = self.mul(base, base)
            e >>= 1
        return r


class Curve:
    """y^2 + xy = x^3 + a x^2 + 1 over GF(2^m); None is the point at infinity."""

    def __init__(self, field: Field, a: int):
        self.f, self.a = field, a

    def on_curve(self, p) -> bool:
        if p is None:
            return True
        f, (x, y) = self.f, p
        lhs = f.mul(y, y) ^ f.mul(x, y)
        x2 = f.mul(x, x)
        rhs = f.mul(x2, x) ^ (x2 if self.a else 0) ^ 1
        return lhs == rhs

    def neg(self, p):
        return None if p is None else (p[0], p[0] ^ p[1])

    def add(self, p, q):
        if p is None:
            return q
        if q is None:
            return p
        f = self.f
        x1, y1 = p
        x2, y2 = q
        if x1 == x2:  # same abscissa: q is p or -p
            return self.dbl(p) if y1 == y2 else None
        lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2))
        x3 = f.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ (1 if self.a else 0)
        y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def dbl(self, p):
        if p is None:
            return None
        f = self.f
        x1, y1 = p
        if x1 == 0:
            return None
        lam = x1 ^ f.mul(y1, f.inv(x1))
        x3 = f.mul(lam, lam) ^ lam ^ (1 if self.a else 0)
        y3 = f.mul(x1, x1) ^ f.mul(lam ^ 1, x3)
        return (x3, y3)

    def mul(self, k: int, p):
        r = None
        if k < 0:
            k, p = -k, self.neg(p)
        for bit in bin(k)[2:] if k else "":
            r = self.dbl(r)
            if bit == "1":
                r = self.add(r, p)
        return r


def point(v):
    return (int(v[0]), int(v[1]))


def check_certificate(cert: dict, cache: dict) -> dict:
    c = cert["curve"]
    key = (c["degree"], tuple(c["irreducible_low_terms"]), c["curve_a"],
           tuple(c["generator"]), c["subgroup_order"])
    if key not in cache:
        curve = Curve(Field(c["degree"], c["irreducible_low_terms"]), int(c["curve_a"]))
        g = point(c["generator"])
        n = int(c["subgroup_order"])
        cache[key] = (curve, g, n, curve.on_curve(g), curve.mul(n, g) is None)
    curve, g, n, g_on, order_ok = cache[key]
    t = point(cert["target"])
    s = int(cert["scalar"])
    ok = curve.mul(s, g) == t
    neg = curve.mul(s + 1, g) == t
    return {"generator_on_curve": g_on, "order_kills_generator": order_ok,
            "target_on_curve": curve.on_curve(t), "scalar_ok": ok,
            "negative_control_rejected": not neg}


def records(path: Path):
    if path.is_dir():
        for p in sorted(path.rglob("*.price.json")):
            yield str(p), p.read_bytes()
    else:
        with tarfile.open(path) as tf:
            for m in sorted(tf.getmembers(), key=lambda m: m.name):
                if m.isfile() and m.name.endswith(".price.json"):
                    yield f"{path}!{m.name}", tf.extractfile(m).read()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("paths", nargs="+", type=Path)
    ap.add_argument("--out", type=Path)
    a = ap.parse_args()
    cache: dict = {}
    out = {"tool": "research/ic_tool_program/validation/certverify.py",
           "independent": True, "records": 0, "certificates": 0,
           "failures": [], "curves": set(), "inputs": []}
    for root in a.paths:
        n_in = 0
        for name, raw in records(root):
            n_in += 1
            out["records"] += 1
            doc = json.loads(raw)
            certs = doc.get("certificates") or {}
            arms = [k for k in ("ic", "rho") if isinstance(certs.get(k), dict)]
            if not arms:
                out["failures"].append({"record": name, "reason": "no certificate"})
                continue
            for arm in arms:
                r = check_certificate(certs[arm], cache)
                out["certificates"] += 1
                out["curves"].add(certs[arm]["curve"]["degree"])
                if not all(r.values()):
                    out["failures"].append({"record": name, "arm": arm, **r})
            rec = [str(x) for x in (doc.get("counts") or {}).get("recovered", [])]
            if rec and any(str(certs[a]["scalar"]) not in rec for a in arms):
                out["failures"].append({"record": name, "reason": "recovered != certificate scalar"})
        digest = hashlib.sha256(root.read_bytes()).hexdigest() if root.is_file() else None
        out["inputs"].append({"path": str(root), "records": n_in, "sha256": digest})
    out["curves"] = sorted(out["curves"])
    out["status"] = "pass" if out["records"] and not out["failures"] else "fail"
    text = json.dumps(out, indent=1)
    if a.out:
        a.out.write_text(text + "\n")
    print(json.dumps({k: out[k] for k in ("status", "records", "certificates", "curves")}))
    for f in out["failures"][:10]:
        print("FAIL", f, file=sys.stderr)
    return 0 if out["status"] == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
