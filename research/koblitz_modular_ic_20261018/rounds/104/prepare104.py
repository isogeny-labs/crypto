#!/usr/bin/env python3
"""Build the x=1 leading residue of (x+1)^9 n from R102 rows."""
import hashlib
import json
import re
import sys
from pathlib import Path

R102_SHA256 = "4d11617b41d426e8e055e90c797fa5529f3b2a6b013eedb3e953cb07e94ecbec"
R101_SHA256 = "92d795e666b43874c37af1f8e0e146b5aae1a2a3ab9a61954b4395738ab4405e"


def monomial(l: int, y: int) -> str:
    atoms = []
    if l:
        atoms.append("l" if l == 1 else f"l^{l}")
    if y:
        atoms.append("y" if y == 1 else f"y^{y}")
    return "*".join(atoms) or "1"


def main() -> None:
    result_path, spec_path, output = map(Path, sys.argv[1:4])
    for path, expected in ((result_path, R102_SHA256), (spec_path, R101_SHA256)):
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch: {path}")
    result = json.loads(result_path.read_text())
    spec = spec_path.read_text()
    fmatch = re.search(r"poly f101=([^;]+);", spec)
    gmatch = re.search(r"poly G101=([^;]+);", spec)
    if fmatch is None or gmatch is None or fmatch.group(1) != "y^7+y^3" or gmatch.group(1) != "l^16+l^8+1":
        raise ValueError("R101 fiber shape mismatch")
    rows = result["rows"]
    worst = [row for row in rows if row["coefficient_order"] == -9]
    if len(rows) != 105 or len(worst) != 22:
        raise ValueError("R102 row/worst count mismatch")
    pairs = [(row["lambda_exponent"], row["y_exponent"]) for row in worst]
    if len(set(pairs)) != 22 or any(l >= 16 or y >= 7 for l, y in pairs):
        raise ValueError("leading residue outside standard basis")
    terms = [monomial(l, y) for l, y in sorted(pairs, reverse=True)]
    leading = "+".join(terms)
    changed = "+".join(terms[1:])
    output.mkdir(parents=True, exist_ok=True)
    (output / "input104.sing").write_text(f"poly f104={fmatch.group(1)};\npoly G104={gmatch.group(1)};\npoly H104={leading};\npoly changed104={changed};\n")
    (output / "input104.json").write_text(json.dumps({"r102_sha256": R102_SHA256, "r101_sha256": R101_SHA256, "leading_term_count": 22, "leading_polynomial": leading, "changed_polynomial": changed, "standard_basis_max_l": max(l for l, _ in pairs), "standard_basis_max_y": max(y for _, y in pairs)}, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
