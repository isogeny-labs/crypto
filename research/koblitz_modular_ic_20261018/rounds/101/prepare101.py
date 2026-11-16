#!/usr/bin/env python3
"""Exhaustively specialize the R51 triangular source at x=1 over F2."""
import hashlib
import json
import re
import sys
from pathlib import Path

EXPECTED = "7a92cf0f1665b80bef5a38c57ca45136e530bfcf8cd13a430942af75684b7409"


def split_sum(expression: str) -> list[str]:
    pieces = []
    depth = start = 0
    for index, char in enumerate(expression):
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
        elif char == "+" and depth == 0:
            pieces.append(expression[start:index])
            start = index + 1
        if depth < 0:
            raise ValueError("unbalanced parentheses")
    if depth:
        raise ValueError("unbalanced parentheses")
    pieces.append(expression[start:])
    return pieces


def parity_at_one(expression: str) -> int:
    terms = expression.split("+")
    if any(not re.fullmatch(r"1|x(?:\^[1-9][0-9]*)?", term) for term in terms):
        raise ValueError("unknown x-coefficient term")
    if len(terms) != len(set(terms)):
        raise ValueError("duplicate x-coefficient term")
    return len(terms) % 2


def monomial(exponent: int, variable: str) -> str:
    return "" if exponent == 0 else variable if exponent == 1 else f"{variable}^{exponent}"


def render(terms: set[tuple[int, int]]) -> str:
    if not terms:
        return "0"
    return "+".join("*".join(filter(None, (monomial(l, "l"), monomial(y, "y")))) or "1" for l, y in sorted(terms, reverse=True))


def main() -> None:
    source, output = map(Path, sys.argv[1:3])
    data = source.read_bytes()
    if hashlib.sha256(data).hexdigest() != EXPECTED:
        raise ValueError("R51 donor hash mismatch")
    text = data.decode("ascii")
    found = re.findall(r"(?:^|;)poly G51=(.*?);", text)
    base = re.findall(r"(?:^|;)poly f51=(.*?);", text)
    if len(found) != 1 or len(base) != 1:
        raise ValueError("source assignments not unique")
    gparts = split_sum(found[0])
    if len(gparts) != 29 or gparts[0] != "l^16":
        raise ValueError("unexpected G51 term shape")
    gterms = {(16, 0)}
    l_counts = {16: 1, 8: 0, 4: 0, 1: 0, 0: 0}
    for term in gparts[1:]:
        match = re.fullmatch(r"\(([^()]*)\)/\(x\^[1-9][0-9]*\)(.*)", term)
        if match is None:
            raise ValueError("unrecognized G51 coefficient")
        suffix = match.group(2)
        if suffix and not suffix.startswith("*"):
            raise ValueError("missing G51 monomial separator")
        powers = {"l": 0, "y": 0}
        for atom in (suffix[1:].split("*") if suffix else []):
            m = re.fullmatch(r"([ly])(?:\^([1-9][0-9]*))?", atom)
            if m is None or powers[m.group(1)]:
                raise ValueError("unrecognized G51 monomial")
            powers[m.group(1)] = int(m.group(2) or "1")
        le, ye = powers["l"], powers["y"]
        if le not in (8, 4, 1, 0) or ye > 6:
            raise ValueError("unexpected G51 exponents")
        l_counts[le] += 1
        if parity_at_one(match.group(1)):
            gterms.symmetric_difference_update({(le, ye)})
    if l_counts != {16: 1, 8: 7, 4: 7, 1: 7, 0: 7}:
        raise ValueError("incomplete G51 coefficient grid")
    fterms = set()
    for term in split_sum(base[0]):
        m = re.fullmatch(r"(?:\(([^()]*)\)\*)?(y(?:\^([1-9][0-9]*))?)", term)
        if m:
            coefficient = m.group(1) or "1"
            ye = int(m.group(3) or "1")
        else:
            m = re.fullmatch(r"\(([^()]*)\)", term)
            if m is None:
                raise ValueError("unrecognized f51 term")
            coefficient, ye = m.group(1), 0
        if parity_at_one(coefficient):
            fterms.symmetric_difference_update({(0, ye)})
    if fterms != {(0, 7), (0, 3)}:
        raise ValueError("f51 x=1 specialization mismatch")
    cterms = {(0, y) for l, y in gterms if l == 1}
    if cterms:
        raise ValueError("expected x=1 lambda derivative degeneration was absent")
    output.mkdir(parents=True, exist_ok=True)
    (output / "specialization101.sing").write_text(f"poly f101={render(fterms)};\npoly G101={render(gterms)};\npoly C101={render(cterms)};\n")
    y1terms = set()
    for l, _ in gterms:
        y1terms.symmetric_difference_update({(l, 0)})
    info = {"donor_sha256": EXPECTED, "source_terms": len(gparts), "lambda_exponents": sorted(l_counts), "specialized_f": render(fterms), "specialized_G": render(gterms), "specialized_derivative": render(cterms), "y0_lambda": render({(l, 0) for l, y in gterms if y == 0}), "y1_lambda": render(y1terms)}
    (output / "specialization101.json").write_text(json.dumps(info, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
