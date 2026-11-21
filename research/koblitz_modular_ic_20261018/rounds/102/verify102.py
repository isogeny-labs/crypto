#!/usr/bin/env python3
"""Independent Hasse-coefficient check of all R102 x+1 orders."""
import hashlib
import json
import re
import sys
from pathlib import Path

import audit102

RESULT_SHA256 = "4d11617b41d426e8e055e90c797fa5529f3b2a6b013eedb3e953cb07e94ecbec"


def hasse_order(bits: int) -> int:
    exponents = [e for e in range(bits.bit_length()) if (bits >> e) & 1]
    for k in range(max(exponents) + 1):
        # In F2, binomial(e,k) is odd exactly when every 1-bit of k is in e.
        if sum(1 for e in exponents if e & k == k) % 2:
            return k
    raise ValueError("zero polynomial")


def main() -> None:
    donor, result_path = map(Path, sys.argv[1:3])
    raw = donor.read_bytes()
    result_raw = result_path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != audit102.DONOR_SHA256:
        raise ValueError("donor hash mismatch")
    if hashlib.sha256(result_raw).hexdigest() != RESULT_SHA256:
        raise ValueError("result hash mismatch")
    data = json.loads(result_raw)
    source = raw.decode("ascii")
    expression = re.findall(r"(?:^|;)poly N51=(.*?);", source)[0]
    terms = audit102.split_sum(expression)[1:]
    rows = data["rows"]
    if len(terms) != len(rows) or len(rows) != 105:
        raise ValueError("source/result length mismatch")
    checked = []
    for term, row in zip(terms, rows):
        match = audit102.FRACTION.fullmatch(term)
        if match is None:
            raise ValueError("source fraction mismatch")
        numerator, denominator, bare_denominator, _ = match.groups()
        num_order = hasse_order(audit102.parse_xpoly(numerator))
        den_order = hasse_order(audit102.parse_xpoly(denominator or bare_denominator))
        if (num_order, den_order, num_order - den_order) != (row["numerator_order"], row["denominator_order"], row["coefficient_order"]):
            raise ValueError("Hasse valuation mismatch")
        checked.append(num_order - den_order)
    if min(checked) != -9 or checked.count(-9) != 22 or max(checked) != -1:
        raise ValueError("summary mismatch")
    print(json.dumps({"check": "R102_INDEPENDENT_HASSE_PASS", "coefficients": len(checked), "minimum": min(checked), "minimum_count": checked.count(-9), "maximum": max(checked), "clearing_exponent": 9}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
