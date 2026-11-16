#!/usr/bin/env python3
"""Precision-aware N51 Laurent valuation on R107's x=1 branches."""
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path

HASHES = (
    "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409",
    "e6812ce976f403b612ee0b0a80c02bdb58d6d05202c80a9920c4fdd6001eb525",
    "485c48de9128cf7d7939d5c73cfb3c7dd738592b93144cab45f9a1171c8b031d",
    "a58481e36b510f06a8bb1c2aeb92fce61d49cf7c1ba2eb1ce872b584b1379b33",
)
P = 128
MASK = (1 << P) - 1


def mul(a, b, width=P):
    out = 0
    while b:
        if b & 1: out ^= a
        a <<= 1; b >>= 1
    return out & ((1 << width) - 1)


def square(a, width):
    return sum(1 << (2 * i) for i in range((width + 1) // 2) if a >> i & 1)


def power(a, n):
    out = 1
    while n:
        if n & 1: out = mul(out, a)
        a = mul(a, a); n >>= 1
    return out


def valuation(a):
    return None if not a else (a & -a).bit_length() - 1


def invert_unit(q, width):
    if not q & 1: raise ValueError("nonunit denominator")
    inv, precision = 1, 1
    while precision < width:
        precision = min(2 * precision, width)
        inv = mul(q, square(inv, precision), precision)
    if mul(q, inv, width) != 1: raise ValueError("inverse verification failed")
    return inv


def f4mul(a, b):
    out = 0
    for i in range(2):
        if b >> i & 1: out ^= a << i
    if out & 4: out ^= 7
    return out


def f4pow(a, n):
    out = 1
    for _ in range(n): out = f4mul(out, a)
    return out


def parse_terms(donor, audit):
    assignments = re.findall(r"(?:^|;)poly N51=(.*?);", donor)
    if len(assignments) != 1: raise ValueError("N51 assignment not unique")
    parts = audit.split_sum(assignments[0])
    if len(parts) != 106 or parts[0] != "n": raise ValueError("N51 term shape")
    parsed = []
    for term in parts[1:]:
        m = audit.FRACTION.fullmatch(term)
        if m is None: raise ValueError("unrecognized N51 rational term")
        numerator, denominator, bare, suffix = m.groups()
        powers = {"l": 0, "y": 0}
        if suffix:
            if not suffix.startswith("*"): raise ValueError("N51 suffix")
            for atom in suffix[1:].split("*"):
                z = re.fullmatch(r"([ly])(?:\^([1-9][0-9]*))?", atom)
                if z is None: raise ValueError("N51 monomial")
                powers[z.group(1)] = int(z.group(2) or "1")
        parsed.append((powers["l"], powers["y"], audit.parse_xpoly(numerator),
                       audit.parse_xpoly(denominator or bare)))
    if len(parsed) != 105 or len({(l, y) for l, y, _, _ in parsed}) != 105:
        raise ValueError("N51 coefficient grid")
    return parsed


def eval_xpoly(bits, x):
    out = 0
    for i in range(bits.bit_length()):
        if bits >> i & 1: out ^= power(x, i)
    return out


def rational_series(numerator, denominator, x, y, ye):
    a = mul(eval_xpoly(numerator, x), power(y, ye))
    b = eval_xpoly(denominator, x)
    n, d = valuation(a), valuation(b)
    if n is None or d is None: raise ValueError("source precision hides rational numerator/denominator")
    width = P - max(n, d)
    if width <= 0: raise ValueError("no Laurent precision")
    ratio = mul(a >> n, invert_unit(b >> d, width), width)
    return n - d, ratio, n - d + width, n, d


def summarize_coefficients(parsed, z_exponents, shift):
    z = sum(1 << i for i in z_exponents)
    t, y = mul(2, z), z ^ shift
    x = 1 ^ t
    coeff = {j: {} for j in range(15)}
    bounds = {j: P for j in range(15)}
    term_stats = []
    for le, ye, num, den in parsed:
        offset, bits, until, n, d = rational_series(num, den, x, y, ye)
        term_stats.append((le, ye, offset, until, n, d))
        for j in range(le + 1):
            if j & ~le: continue  # Lucas parity in characteristic 2
            scalar = f4pow(2, le - j)
            bounds[j] = min(bounds[j], until)
            for k in range(bits.bit_length()):
                if bits >> k & 1:
                    e = offset + k
                    coeff[j][e] = coeff[j].get(e, 0) ^ scalar
    rows = []
    for j in range(15):
        limit = bounds[j]
        nonzero = sorted(e for e, v in coeff[j].items() if e < limit and v)
        order = nonzero[0] if nonzero else None
        rows.append({"u_exponent": j, "s_order": order,
                     "s_order_lower_bound": limit if order is None else order,
                     "known_until_exclusive": limit,
                     "leading_F4": None if order is None else coeff[j][order],
                     "nonzero_support_below_bound": nonzero})
    return rows, term_stats


def weighted(rows, s_weight, u_weight, u_lead):
    candidates = [(s_weight * row["s_order"] + u_weight * row["u_exponent"], row)
                  for row in rows if row["s_order"] is not None]
    if not candidates: return {"certified": False, "reason": "all coefficient orders unresolved"}
    minimum = min(w for w, _ in candidates)
    uncertain = [(row["u_exponent"], s_weight * row["s_order_lower_bound"] + u_weight * row["u_exponent"])
                 for row in rows if row["s_order"] is None and
                 s_weight * row["s_order_lower_bound"] + u_weight * row["u_exponent"] <= minimum]
    tied = [row for w, row in candidates if w == minimum]
    scalar = 0
    for row in tied:
        scalar ^= f4mul(row["leading_F4"], f4pow(u_lead, row["u_exponent"]))
    return {"candidate_order": minimum, "tied_u_exponents": [row["u_exponent"] for row in tied],
            "leading_F4_sum": scalar, "precision_competitors": uncertain,
            "certified": not uncertain and scalar != 0,
            "valuation": minimum if not uncertain and scalar != 0 else None}


def main():
    donor, parser, edges, base, output = map(Path, sys.argv[1:6])
    for path, expected in zip((donor, parser, edges, base), HASHES):
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch: {path}")
    spec = importlib.util.spec_from_file_location("audit102", parser)
    audit = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(audit)
    raw = donor.read_text()
    if audit.audit(raw)["coefficient_count"] != 105: raise ValueError("R102 audit mismatch")
    parsed = parse_terms(raw, audit)
    edge_data = json.loads(edges.read_text())
    base_data = json.loads(base.read_text())
    out = {"source_sha256": HASHES, "precision": P, "term_count": len(parsed)}
    for key, shift, su in (("y0", 0, 5), ("y1", 1, 4)):
        rows, stats = summarize_coefficients(parsed, base_data[key]["z_exponents"], shift)
        erows = {r["u_exponent"]: r for r in edge_data[key]["translated_rows"]}
        if erows[0]["leading_F4"] != 2 or erows[1]["leading_F4"] != 1:
            raise ValueError("R107 edge leading data mismatch")
        if erows[0]["s_order"] - erows[1]["s_order"] != su:
            raise ValueError("R107 degree-one slope mismatch")
        out[key] = {"coefficient_rows": rows,
                    "denominator_s_order_max": max(x[5] for x in stats),
                    "numerator_s_order_max": max(x[4] for x in stats),
                    "degree_one": weighted(rows, 1, su, 2),
                    "degree_seven": weighted(rows, 7, 23, 1)}
    output.mkdir(parents=True, exist_ok=True)
    (output / "result.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("NORM109_ALL_CHECKS_PASS")


if __name__ == "__main__": main()
