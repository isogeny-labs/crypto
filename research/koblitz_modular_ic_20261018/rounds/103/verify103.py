#!/usr/bin/env python3
"""Independent GF(2) comparison of R99, R100, and direct R103 norms."""
import hashlib
import json
import re
import sys
from pathlib import Path

HASHES = {
    "direct": "b1bb5d3f810b1167719c883cf6e924aa7a4330e574df2efb56959a4c79298146",
    "full": "f2ad5d4ea0328e902254efcb6f2b6390f8aa5097d0bb18d82c53cf18ffac62d6",
    "root": "edffe01c7dca7dbe83ae3d77c58e9ec2a2274608b48a4d6d93cbd0a3feb52d0f",
}


def polynomial(expression: str) -> set[int]:
    terms = expression.split("+")
    if len(terms) != len(set(terms)):
        raise ValueError("duplicate monomial")
    result = set()
    for term in terms:
        if term == "1":
            result.add(0)
        elif term == "x":
            result.add(1)
        else:
            match = re.fullmatch(r"x\^([1-9][0-9]*)", term)
            if match is None:
                raise ValueError("non-x polynomial")
            result.add(int(match.group(1)))
    return result


def extract(text: str, name: str) -> str:
    match = re.search(r"(?:number|poly) " + re.escape(name) + r"=\(?([^;()]*)\)?;", text)
    if match is None:
        raise ValueError(f"missing {name}")
    return match.group(1)


def main() -> None:
    paths = dict(zip(("direct", "full", "root"), map(Path, sys.argv[1:4])))
    if len(paths) != 3:
        raise ValueError("need direct, full, and root files")
    texts = {}
    for name, path in paths.items():
        data = path.read_bytes()
        if hashlib.sha256(data).hexdigest() != HASHES[name]:
            raise ValueError(f"{name} source hash mismatch")
        texts[name] = data.decode("ascii")
    direct = polynomial(extract(texts["direct"], "directNumerator103"))
    full = polynomial(extract(texts["full"], "normNumerator99"))
    root = polynomial(extract(texts["root"], "root100"))
    if direct != root or {16 * exponent for exponent in direct} != full:
        raise ValueError("independent Frobenius norm mismatch")
    if extract(texts["direct"], "directDenominator103") != "x^163" or extract(texts["full"], "normDenominator99") != "x^2608":
        raise ValueError("denominator mismatch")
    if 16 * 163 != 2608 or max(direct) != 123 or max(full) != 1968:
        raise ValueError("degree mismatch")
    print(json.dumps({"check": "R103_INDEPENDENT_NORM_MATCH_PASS", "direct_numerator_terms": len(direct), "direct_numerator_degree": 123, "direct_denominator_exponent": 163, "full_numerator_degree": 1968, "full_denominator_exponent": 2608, "matches_R100_root": True}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
