#!/usr/bin/env python3
"""Extract the exact Frobenius root from the frozen R99 norm output."""
import hashlib
import json
import re
import sys
from pathlib import Path

EXPECTED = "f2ad5d4ea0328e902254efcb6f2b6390f8aa5097d0bb18d82c53cf18ffac62d6"


def parse_polynomial(expression: str) -> set[int]:
    terms = expression.split("+")
    if len(terms) != len(set(terms)):
        raise ValueError("duplicate terms")
    result = set()
    for term in terms:
        if term == "1":
            result.add(0)
        elif term == "x":
            result.add(1)
        elif re.fullmatch(r"x\^[1-9][0-9]*", term):
            result.add(int(term[2:]))
        else:
            raise ValueError(f"unrecognized term: {term}")
    return result


def render(terms: set[int]) -> str:
    return "+".join("1" if e == 0 else "x" if e == 1 else f"x^{e}" for e in sorted(terms, reverse=True))


def main() -> None:
    source, output = map(Path, sys.argv[1:3])
    data = source.read_bytes()
    if hashlib.sha256(data).hexdigest() != EXPECTED:
        raise ValueError("R99 norm source hash mismatch")
    text = data.decode("ascii")
    match = re.fullmatch(
        r"// Exact resultant norm in F2\(x\), from actual R51 G51 and f51\.\n"
        r"number normNumerator99=\(([^()]*)\);\n"
        r"number normDenominator99=\(([^()]*)\);\n", text,
    )
    if match is None or match.group(2) != "x^2608":
        raise ValueError("unexpected R99 norm shape")
    numerator = parse_polynomial(match.group(1))
    if len(numerator) != 73 or max(numerator) != 1968 or 0 not in numerator:
        raise ValueError("unexpected numerator terms")
    if any(e % 16 for e in numerator):
        raise ValueError("numerator is not a sixteenth power")
    root = {e // 16 for e in numerator}
    if {16 * e for e in root} != numerator or max(root) != 123:
        raise ValueError("Frobenius power mismatch")
    output.mkdir(parents=True, exist_ok=True)
    (output / "root.sing").write_text(f"poly root100={render(root)};\npoly numerator100={render(numerator)};\n")
    (output / "root.json").write_text(json.dumps({"source_sha256": EXPECTED, "numerator_degree": 1968, "numerator_terms": 73, "root_degree": 123, "root_terms": 73, "root_expression": render(root), "denominator_expression": "x^2608"}, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
