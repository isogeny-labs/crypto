#!/usr/bin/env python3
"""R51 lambda equation over R106's two base completions, modulo s^32."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

SOURCE_HASHES = (
    "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409",
    "4306bbc6a7c5de1b21716a5fc5a1fd1a8030ff1340409c814c39d6217292a3e4",
    "c112aeedde039a3b36376a0e720e00f62da64c5abe336d77296b97baa4cc12b8",
)
PRECISION = 32
MASK = (1 << PRECISION) - 1


def mul(a, b):
    out = 0
    while b:
        if b & 1: out ^= a
        a <<= 1; b >>= 1
    return out & MASK


def power(a, n):
    out = 1
    while n:
        if n & 1: out = mul(out, a)
        a = mul(a, a); n >>= 1
    return out


def inverse_one_plus_t(t):
    out, term = 0, 1
    while term:
        out ^= term
        term = mul(term, t)
    if mul(out, 1 ^ t) != 1: raise ValueError("inverse check")
    return out


def valuation(a, b=0):
    x = a | b
    return None if not x else (x & -x).bit_length() - 1


def support(bits):
    return [i for i in range(PRECISION) if bits >> i & 1]


def branch(gterms, z_exponents, shift):
    z = sum(1 << i for i in z_exponents)
    t = mul(2, z)
    x = 1 ^ t
    y = z ^ shift
    inv_x = inverse_one_plus_t(t)
    coeff = {i: 0 for i in (0, 1, 4, 8, 16)}
    for le, ye, den, exps in gterms:
        numerator = 0
        for e in exps: numerator ^= power(x, e)
        coeff[le] ^= mul(mul(numerator, power(inv_x, den)), power(y, ye))
    if coeff[16] != 1: raise ValueError("G not monic")
    # F4 = F2[alpha]/(alpha^2+alpha+1). Each pair is (1,alpha) series.
    translated = {i: [0, 0] for i in (0, 1, 4, 8, 16)}
    translated[0][0] = coeff[0] ^ coeff[8]
    translated[0][1] = coeff[1] ^ coeff[4] ^ coeff[8] ^ coeff[16]
    for i in (1, 4, 8, 16): translated[i][0] = coeff[i]
    if translated[0][0] & 1 or translated[0][1] & 1:
        raise ValueError("R101 alpha not a root")
    if not (translated[8][0] & 1 and translated[16][0] & 1):
        raise ValueError("R101 u^8+u^16 fiber mismatch")
    if any(translated[i][0] & 1 for i in (1, 4)):
        raise ValueError("R101 low-u fiber mismatch")
    rows = []
    for i in (0, 1, 4, 8, 16):
        a, b = translated[i]
        v = valuation(a, b)
        rows.append({"u_exponent": i, "s_order": v, "s_order_lower_bound": PRECISION if v is None else v,
                     "leading_F4": None if v is None else ((a >> v & 1) | ((b >> v & 1) << 1)),
                     "one_support": support(a), "alpha_support": support(b)})
    return {"shift": shift, "z_valuation": valuation(z), "t_valuation": valuation(t),
            "lambda_coefficients": {str(i): support(coeff[i]) for i in coeff},
            "translated_rows": rows}


def main():
    donor, parser, base, output = map(Path, sys.argv[1:5])
    for path, expected in zip((donor, parser, base), SOURCE_HASHES):
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch: {path}")
    spec = importlib.util.spec_from_file_location("jets105", parser)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    _, gterms = module.parse_source(donor.read_text())
    if len(gterms) != 29: raise ValueError("source term count")
    series = json.loads(base.read_text())
    result = {"source_sha256": SOURCE_HASHES, "precision": PRECISION,
              "y0": branch(gterms, series["y0"]["z_exponents"], 0),
              "y1": branch(gterms, series["y1"]["z_exponents"], 1)}
    output.mkdir(parents=True, exist_ok=True)
    (output / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print("LAMBDA107_ALL_CHECKS_PASS")


if __name__ == "__main__": main()
