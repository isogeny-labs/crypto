#!/usr/bin/env python3
"""Exact x=1 Hasse jets of the R51 degree-112 triangular donor."""
import hashlib
import json
import math
import re
import sys
from pathlib import Path

SOURCE_SHA256 = "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409"


def split_sum(s):
    out, depth, start = [], 0, 0
    for i, c in enumerate(s):
        if c == "(": depth += 1
        if c == ")": depth -= 1
        if depth < 0: raise ValueError("unbalanced expression")
        if c == "+" and depth == 0:
            out.append(s[start:i]); start = i + 1
    if depth: raise ValueError("unbalanced expression")
    out.append(s[start:])
    return out


def x_exponents(s):
    terms = s.split("+")
    out = []
    for term in terms:
        m = re.fullmatch(r"1|x(?:\^([1-9][0-9]*))?", term)
        if m is None: raise ValueError("invalid x numerator")
        out.append(0 if term == "1" else int(m.group(1) or "1"))
    if len(out) != len(set(out)): raise ValueError("duplicate x monomial")
    return out


def binom_parity(n, k):
    if n >= 0: return (math.comb(n, k) & 1) if k <= n else 0
    return math.comb(-n + k - 1, k) & 1


def toggle(s, item):
    if item in s: s.remove(item)
    else: s.add(item)


def parse_source(data):
    f = re.findall(r"(?:^|;)poly f51=(.*?);", data)
    g = re.findall(r"(?:^|;)poly G51=(.*?);", data)
    if len(f) != 1 or len(g) != 1: raise ValueError("source assignments not unique")
    fterms = []
    for term in split_sum(f[0]):
        m = re.fullmatch(r"(?:\(([^()]*)\)\*)?y(?:\^([1-9][0-9]*))?", term)
        if m:
            numerator, ye = m.group(1) or "1", int(m.group(2) or "1")
        else:
            m = re.fullmatch(r"\(([^()]*)\)", term)
            if m is None: raise ValueError("unknown f term")
            numerator, ye = m.group(1), 0
        fterms.append((ye, x_exponents(numerator)))
    if len(fterms) != 8 or {ye for ye, _ in fterms} != set(range(8)):
        raise ValueError("incomplete f coefficient grid")
    gparts = split_sum(g[0])
    if len(gparts) != 29 or gparts[0] != "l^16": raise ValueError("G shape")
    gterms = [(16, 0, 0, [0])]
    for term in gparts[1:]:
        m = re.fullmatch(r"\(([^()]*)\)/\(x\^([1-9][0-9]*)\)(.*)", term)
        if m is None: raise ValueError("unknown G coefficient")
        le = ye = 0
        suffix = m.group(3)
        if suffix and not suffix.startswith("*"): raise ValueError("G monomial separator")
        for atom in (suffix[1:].split("*") if suffix else []):
            z = re.fullmatch(r"([ly])(?:\^([1-9][0-9]*))?", atom)
            if z is None: raise ValueError("unknown G monomial")
            if z.group(1) == "l": le = int(z.group(2) or "1")
            else: ye = int(z.group(2) or "1")
        if le not in (8, 4, 1, 0) or ye > 6: raise ValueError("G exponent range")
        gterms.append((le, ye, int(m.group(2)), x_exponents(m.group(1))))
    return fterms, gterms


def local_f_support(fterms, shift):
    out = set()
    for ye, exps in fterms:
        for e in exps:
            for tk in range(e + 1):
                if not binom_parity(e, tk): continue
                for zk in range(ye + 1):
                    if shift == 0 and zk != ye: continue
                    if shift and not binom_parity(ye, zk): continue
                    toggle(out, (tk, zk))
    return sorted(out)


def full_g_jets(gterms):
    jets = [set() for _ in range(5)]
    for le, ye, den, exps in gterms:
        for k in range(5):
            if sum(binom_parity(e - den, k) for e in exps) & 1:
                toggle(jets[k], (le, ye))
    return [sorted(j) for j in jets]


def project_g_jets(full, shift):
    out = []
    for support in full:
        projected = set()
        for le, ye in support:
            if shift == 0 and ye != 0: continue
            toggle(projected, le)
        out.append(sorted(projected))
    return out


def divmod2(a, b):
    quotient = 0
    while a and a.bit_length() >= b.bit_length():
        d = a.bit_length() - b.bit_length()
        quotient ^= 1 << d
        a ^= b << d
    return quotient, a


def q_order(exponents):
    a = sum(1 << e for e in exponents)
    if not a: return None
    v = 0
    while True:
        d, r = divmod2(a, 7)
        if r: return v
        v += 1; a = d


def main():
    source, output = map(Path, sys.argv[1:3])
    data = source.read_bytes()
    if hashlib.sha256(data).hexdigest() != SOURCE_SHA256:
        raise ValueError("R51 donor hash mismatch")
    fterms, gterms = parse_source(data.decode("ascii"))
    f0 = local_f_support(fterms, 0)
    f1 = local_f_support(fterms, 1)
    gfull = full_g_jets(gterms)
    g0, g1 = project_g_jets(gfull, 0), project_g_jets(gfull, 1)
    if sorted(z for t, z in f0 if t == 0) != [3, 7]: raise ValueError("R101 f fiber mismatch")
    if gfull[0] != [(0, 0), (8, 0), (16, 0)]: raise ValueError("R101 full G fiber mismatch")
    if g0[0] != [0, 8, 16] or g1[0] != [0, 8, 16]: raise ValueError("R101 G fiber mismatch")
    result = {"source_sha256": SOURCE_SHA256, "f_terms": len(fterms), "G_terms": len(gterms),
              "f_y0_support": f0, "f_y1_support": f1,
              "f_y0_min_t_by_z": {str(z): min(t for t, zz in f0 if zz == z) for z in range(8)},
              "f_y1_min_t_by_z": {str(z): min(t for t, zz in f1 if zz == z) for z in range(8)},
              "G_full_jets": gfull,
              "G_y0_lambda_jets": g0, "G_y1_lambda_jets": g1,
              "G_y0_q_orders": [q_order(j) for j in g0], "G_y1_q_orders": [q_order(j) for j in g1]}
    output.mkdir(parents=True, exist_ok=True)
    (output / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print("JETS105_ALL_CHECKS_PASS")


if __name__ == "__main__": main()
