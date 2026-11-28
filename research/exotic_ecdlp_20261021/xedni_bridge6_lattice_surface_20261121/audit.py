"""Independent sparse integer factor-product and grid-specialization audit."""

import hashlib
import json
import re
from collections import defaultdict
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
GRID = ROOT / "xedni_bridge6_k1k2_grid_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_terms(folder):
    files = sorted((HERE / folder).glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(f.read_bytes() for f in files)
    terms = {}
    for line in raw.splitlines():
        row = json.loads(line)
        key = tuple(row["exponents"])
        assert len(key) == 3 and key not in terms
        terms[key] = int(row["coefficient"])
    return terms, raw, len(files)


def multiply(a, b):
    result = defaultdict(int)
    for ea, ca in a.items():
        for eb, cb in b.items():
            result[tuple(x+y for x, y in zip(ea, eb))] += ca*cb
    return {e: c for e, c in result.items() if c}


def specialize(poly, u, v):
    coeff = defaultdict(int)
    for (eu, ev, et), c in poly.items():
        coeff[et] += c*(u**eu)*(v**ev)
    coeff = {e: c for e, c in coeff.items() if c}
    assert coeff
    content = gcd(*coeff.values())
    coeff = {e: c//content for e, c in coeff.items()}
    if coeff[max(coeff)] < 0:
        coeff = {e: -c for e, c in coeff.items()}
    return [coeff.get(i, 0) for i in range(max(coeff)+1)]


def main():
    result = json.loads((HERE / "result.json").read_text())
    factor_result = json.loads((HERE / "factor-result.json").read_text())
    N, nraw, nparts = read_terms("numerator-terms")
    D, draw, dparts = read_terms("denominator-terms")
    assert sha(nraw) == result["numerator_raw_sha256"]
    assert sha(draw) == result["denominator_raw_sha256"]
    assert (len(N), nparts, len(D), dparts) == (
        result["numerator_terms"], result["numerator_parts"],
        result["denominator_terms"], result["denominator_parts"])
    factors = []
    for row in factor_result["factors"]:
        terms, raw, parts = read_terms(f"factor-{row['index']:02d}-terms")
        assert sha(raw) == row["sha256"]
        assert len(terms) == row["terms"] and parts == row["parts"]
        assert max(map(sum, terms)) == row["total_degree"]
        factors.append(terms)
    assert len(factors) == 2 and factor_result["factor_unit"] == "1"
    assert multiply(*factors) == N
    lean_source = (HERE / "formal" / "Residue.lean").read_text()
    lean_block = lean_source.split("def terms : List Term := [", 1)[1].split("\n]", 1)[0]
    lean_rows = re.findall(r"⟨(\d+), (\d+), (\d+), (-?\d+)⟩", lean_block)
    lean_terms = {tuple(map(int, row[:3])): int(row[3]) for row in lean_rows}
    assert len(lean_rows) == len(factors[0]) and lean_terms == factors[0]
    lean_build = (HERE / "formal" / "build-output.txt").read_text()
    assert "error:" not in lean_build and "sorryAx" not in lean_build
    assert "'XedniBridge6LatticeSurface.necessary_congruence' depends on axioms: [propext, Quot.sound]" in lean_build
    grid_files = sorted((GRID / "rows").glob("[0-9][0-9][0-9].jsonl"))
    assert len(grid_files) == 25
    numerator_matches = 0
    denominator_matches = 0
    for row_file in grid_files:
        row = json.loads(row_file.read_text())
        got_num = specialize(N, row["u"], row["v"])
        got_den = specialize(D, row["u"], row["v"])
        numerator_matches += got_num == list(map(int, row["numerator_coefficients"]))
        denominator_matches += got_den == list(map(int, row["denominator_coefficients"]))
    print(json.dumps({"schema": "xedni-bridge6-lattice-surface-audit-v1",
                      "status": "PASS" if numerator_matches == denominator_matches == 25 else "SPECIALIZATION_MISMATCH",
                      "exact_sparse_factor_product": True,
                      "lean_factor_terms_identical": True,
                      "lean_axioms_checked": True,
                      "numerator_grid_specializations": numerator_matches,
                      "denominator_grid_specializations": denominator_matches,
                      "numerator_terms": len(N), "factor_terms": [len(x) for x in factors],
                      "numerator_raw_sha256": sha(nraw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
