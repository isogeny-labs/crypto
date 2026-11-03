#!/usr/bin/env python3
"""Source-bound audit of the marked-23 degree-16 lambda equation."""

import argparse
import hashlib
import json
import re
from pathlib import Path

DONOR_SHA256 = "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409"
ASSIGNMENT = re.compile(r"(?:^|;)poly G51=(.*?);", re.DOTALL)
DENOMINATOR = re.compile(r"/(?:\((x(?:\^[1-9][0-9]*)?)\)|(x(?:\^[1-9][0-9]*)?))")


def audit(source: str) -> dict:
    matches = ASSIGNMENT.findall(source)
    if len(matches) != 1:
        raise ValueError(f"expected one G51 assignment, found {len(matches)}")
    expression = matches[0]
    denominators = list(DENOMINATOR.finditer(expression))
    slash_positions = [i for i, character in enumerate(expression) if character == "/"]
    matched_positions = [match.start() for match in denominators]
    if slash_positions != matched_positions:
        raise ValueError("G51 contains a non-monomial or unrecognized denominator")
    exponents = [int((match.group(1) or match.group(2)).partition("^")[2] or "1") for match in denominators]
    if not exponents:
        raise ValueError("G51 denominator scan is empty")
    return {
        "source_polynomial": "G51",
        "denominator_count": len(exponents),
        "denominator_exponents": exponents,
        "distinct_exponents": sorted(set(exponents)),
        "maximum_exponent": max(exponents),
        "finite_denominator_support": ["x=0"],
        "all_denominators_divide": f"x^{max(exponents)}",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("donor", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    data = args.donor.read_bytes()
    actual_hash = hashlib.sha256(data).hexdigest()
    if actual_hash != DONOR_SHA256:
        raise ValueError(f"donor SHA-256 mismatch: {actual_hash}")
    result = audit(data.decode("ascii"))
    result["donor_sha256"] = actual_hash
    result["donor_bytes"] = len(data)
    # The negative control must traverse the same scan and be rejected.
    corrupted = data.decode("ascii").replace("/(x^161)", "/(x+1)", 1)
    if corrupted == data.decode("ascii"):
        raise ValueError("negative control was not planted")
    try:
        audit(corrupted)
    except ValueError:
        result["corrupted_denominator_rejected"] = True
    else:
        raise ValueError("corrupted denominator passed the audit")
    text = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(text)
    else:
        print(text, end="")


if __name__ == "__main__":
    main()
