"""Extract Tian's F_257 genus-3 map from the original arXiv TeX.

Usage: python3 extract_tian_genus3_map.py dlpiso.tex > fixtures/tian-genus3-f257-map-v1.json
The source is arXiv:2007.03172v1. Repeated powers are retained in diagnostics.
"""

import hashlib
import json
import re
import sys
from pathlib import Path

SOURCE_SHA256 = "46092fc37bc6bac7e0f42c67e6764150557f63ac4292da86d739980268d3eaae"
P = 257


def normalize(value: str) -> str:
    return value.replace("\\\\", "").replace("&", "").replace("\n", "").replace(" ", "")


def polynomial(value: str) -> tuple[list[int], list[int]]:
    terms = value.split("+")
    result: dict[int, int] = {}
    repeats = []
    for term in terms:
        match = re.fullmatch(r"(\d+)?(?:x(?:\^\{(\d+)\})?)?", term)
        if match is None or not term:
            raise ValueError(f"unparsed monomial: {term!r}")
        coefficient, power = match.groups()
        degree = int(power) if power is not None else (1 if "x" in term else 0)
        if degree in result:
            repeats.append(degree)
        result[degree] = (result.get(degree, 0) + int(coefficient or "1")) % P
    return [result.get(i, 0) for i in range(max(result) + 1)], repeats


def main() -> None:
    source = Path(sys.argv[1]).read_bytes()
    digest = hashlib.sha256(source).hexdigest()
    if digest != SOURCE_SHA256:
        raise ValueError(f"unexpected TeX digest: {digest}")
    tex = source.decode()
    functions = []
    warnings = []
    for i in range(1, 7):
        start = tex.index(r"\mathbf{C}_" + str(i) + r"(x,y)&=")
        end = tex.index(r"\end{align*}", start)
        body = normalize(tex[start:end])
        if i == 1:
            # The TeX prints 103x+207x. The two published mapped point
            # examples both require 103+207x; retain the emendation explicitly.
            if body.count("103x+207x") != 1:
                raise ValueError("expected published C_1 typo is absent")
            body = body.replace("103x+207x", "103+207x")
            warnings.append({"function": 1, "source": "103x+207x",
                             "emendation": "103+207x",
                             "evidence": "both printed mapped divisor examples"})
        numerator = re.search(r"\[\((.*?)\)\+y\((.*?)\)\]", body)
        if numerator is None:
            raise ValueError(f"cannot parse C_{i}")
        a, a_repeats = polynomial(numerator.group(1))
        b, b_repeats = polynomial(numerator.group(2))
        if a_repeats or b_repeats:
            warnings.append({"function": i, "x_only_repeated_degrees": a_repeats,
                             "y_factor_repeated_degrees": b_repeats})
        functions.append({"x_only": a, "y_factor": b})
    start = tex.index(r"\mathbf{p}(x)&=")
    end = tex.index(r"\end{align*}", start)
    denominator, repeats = polynomial(normalize(tex[start:end]).split("=")[1].rstrip("."))
    assert not repeats and len(denominator) == 35
    assert [len(f["x_only"]) for f in functions] == [35, 35, 36, 35, 35, 36]
    assert [len(f["y_factor"]) for f in functions] == [31, 32, 32, 31, 32, 32]
    print(json.dumps({
        "schema": "tian-genus3-f257-map-coefficients/v1",
        "source": "https://arxiv.org/abs/2007.03172v1",
        "source_tex_sha256": SOURCE_SHA256,
        "field": "F_257",
        "coefficient_order": "ascending x degree",
        "denominator": denominator,
        "functions": functions,
        "source_warnings": warnings,
    }, indent=2))


if __name__ == "__main__":
    main()
