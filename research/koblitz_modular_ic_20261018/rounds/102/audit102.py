#!/usr/bin/env python3
"""Exact x+1 valuations of every rational coefficient in R51 N51."""
import argparse
import collections
import hashlib
import json
import re
from pathlib import Path

DONOR_SHA256 = "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409"
FRACTION = re.compile(r"\(([^()]*)\)/(?:\(([^()]*)\)|(x(?:\^[1-9][0-9]*)?))(.*)")


def split_sum(expression: str) -> list[str]:
    parts = []
    depth = start = 0
    for index, char in enumerate(expression):
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
        elif char == "+" and depth == 0:
            parts.append(expression[start:index])
            start = index + 1
        if depth < 0:
            raise ValueError("unbalanced expression")
    if depth:
        raise ValueError("unbalanced expression")
    parts.append(expression[start:])
    return parts


def parse_xpoly(expression: str) -> int:
    terms = expression.split("+")
    if len(terms) != len(set(terms)):
        raise ValueError("duplicate x monomial")
    bits = 0
    for term in terms:
        if term == "1":
            exponent = 0
        elif term == "x":
            exponent = 1
        else:
            match = re.fullmatch(r"x\^([1-9][0-9]*)", term)
            if match is None:
                raise ValueError("unrecognized x monomial")
            exponent = int(match.group(1))
        bits ^= 1 << exponent
    if bits == 0:
        raise ValueError("zero rational coefficient polynomial")
    return bits


def divide_x_plus_one(bits: int) -> int:
    quotient = 0
    while bits.bit_length() > 1:
        degree = bits.bit_length() - 2
        quotient ^= 1 << degree
        bits ^= 3 << degree
    if bits:
        raise ValueError("x+1 division had nonzero remainder")
    return quotient


def valuation_x_plus_one(bits: int) -> int:
    order = 0
    while bin(bits).count("1") % 2 == 0:
        bits = divide_x_plus_one(bits)
        order += 1
    return order


def audit(source: str) -> dict:
    assignments = re.findall(r"(?:^|;)poly N51=(.*?);", source)
    if len(assignments) != 1:
        raise ValueError("N51 assignment is not unique")
    parts = split_sum(assignments[0])
    if len(parts) != 106 or parts[0] != "n":
        raise ValueError("N51 source term count/lead mismatch")
    rows = []
    seen = set()
    for term in parts[1:]:
        match = FRACTION.fullmatch(term)
        if match is None:
            raise ValueError("unrecognized N51 coefficient")
        numerator, denominator, bare_denominator, suffix = match.groups()
        den = denominator or bare_denominator
        if suffix and not suffix.startswith("*"):
            raise ValueError("unrecognized N51 monomial suffix")
        powers = {"l": 0, "y": 0}
        for atom in (suffix[1:].split("*") if suffix else []):
            m = re.fullmatch(r"([ly])(?:\^([1-9][0-9]*))?", atom)
            if m is None or powers[m.group(1)]:
                raise ValueError("unrecognized N51 monomial")
            powers[m.group(1)] = int(m.group(2) or "1")
        key = (powers["l"], powers["y"])
        if key in seen or key[0] >= 16 or key[1] >= 7:
            raise ValueError("duplicate or out-of-range N51 monomial")
        seen.add(key)
        numerator_order = valuation_x_plus_one(parse_xpoly(numerator))
        denominator_order = valuation_x_plus_one(parse_xpoly(den))
        rows.append({"lambda_exponent": key[0], "y_exponent": key[1], "numerator_order": numerator_order, "denominator_order": denominator_order, "coefficient_order": numerator_order - denominator_order})
    if len(rows) != 105 or len(seen) != 105:
        raise ValueError("incomplete N51 coefficient grid")
    orders = [row["coefficient_order"] for row in rows]
    pole_bound = max(0, -min(orders))
    if any(row["coefficient_order"] + pole_bound < 0 for row in rows):
        raise ValueError("clearing bound failed")
    histogram = {str(order): count for order, count in sorted(collections.Counter(orders).items())}
    return {"source_polynomial": "N51", "coefficient_count": 105, "distinct_monomials": len(seen), "coefficient_order_histogram": histogram, "minimum_coefficient_order": min(orders), "maximum_coefficient_order": max(orders), "x_plus_one_clearing_exponent": pole_bound, "worst_terms": [row for row in rows if row["coefficient_order"] == min(orders)], "rows": rows}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("donor", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    raw = args.donor.read_bytes()
    actual = hashlib.sha256(raw).hexdigest()
    if actual != DONOR_SHA256:
        raise ValueError("R51 donor hash mismatch")
    source = raw.decode("ascii")
    result = audit(source)
    result["donor_sha256"] = actual
    result["donor_bytes"] = len(raw)
    try:
        parse_xpoly("x+y")
    except ValueError:
        result["planted_non_x_denominator_rejected"] = True
    else:
        raise ValueError("negative control passed")
    output = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(output)
    else:
        print(output, end="")


if __name__ == "__main__":
    main()
