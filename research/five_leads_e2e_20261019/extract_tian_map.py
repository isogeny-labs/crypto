"""Transcribe the six published map functions from Tian's original TeX.

Usage: python3 extract_tian_map.py Cover_attack.tex > fixtures/tian-cubic-map-v1.json
The source is arXiv:2012.07173v1, whose TeX SHA-256 is checked below.
"""

import hashlib
import json
import re
import sys
from pathlib import Path

SOURCE_SHA256 = "5f0284c0039854cf11436168eb308417f80b680378f1ebe6e1922d5ce36f25d7"


def coefficients(block: str) -> list[list[int]]:
    body = block.replace("\\\\", "").replace("&", "").replace("\n", "").replace(" ", "")
    body = re.sub(r"\\(?:bigl|bigr|Bigl|Bigr)", "", body)
    expression = r"\((\d*)w\^2\+(\d+)w\+(\d+)\)x(?:\^\{?(\d+)\}?)?"
    terms = re.findall(expression, body)
    values = {int(power or 1): [int(c), int(b), int(a or "1")] for a, b, c, power in terms}
    tail = body[body.rfind(")x") + 2 :]
    constant = re.search(r"\+(\d*)w\^2\+(\d+)w\+(\d+)", tail)
    if constant is None:
        raise ValueError("missing constant term")
    a, b, c = constant.groups()
    values[0] = [int(c), int(b), int(a or "1")]
    if sorted(values) != list(range(max(values) + 1)):
        raise ValueError("missing or repeated polynomial coefficient")
    return [values[i] for i in sorted(values)]


def main() -> None:
    source = Path(sys.argv[1]).read_bytes()
    digest = hashlib.sha256(source).hexdigest()
    if digest != SOURCE_SHA256:
        raise ValueError(f"unexpected paper source SHA-256: {digest}")
    tex = source.decode()
    functions = []
    for i in range(1, 7):
        start = tex.index(r"\mathbf{C}_" + str(i) + r"(x,y)&=")
        end = tex.index(r"\end{align*}", start)
        parts = re.split(r"\+\s*y\\bigl", tex[start:end])
        if len(parts) != 2:
            raise ValueError(f"cannot split C_{i} numerator")
        x_only, y_factor = map(coefficients, parts)
        functions.append({"x_only": x_only, "y_factor": y_factor})
    start = tex.index(r"\mathbf{p}(x)&=")
    end = tex.index(r"\end{align*}", start)
    denominator = coefficients(tex[start:end])
    denominator.append([1, 0, 0])
    assert len(denominator) == 11
    assert [len(row["x_only"]) for row in functions] == [11, 11, 12, 11, 11, 12]
    assert [len(row["y_factor"]) for row in functions] == [9, 10, 10, 9, 10, 10]
    print(json.dumps({
        "schema": "tian-cubic-map-coefficients/v1",
        "source": "https://arxiv.org/abs/2012.07173v1",
        "source_tex_sha256": SOURCE_SHA256,
        "field": "F_523[w]/(w^3+2)",
        "coefficient_order": "ascending x degree; each coefficient is [constant,w,w^2]",
        "denominator": denominator,
        "functions": functions,
    }, indent=2))


if __name__ == "__main__":
    main()
