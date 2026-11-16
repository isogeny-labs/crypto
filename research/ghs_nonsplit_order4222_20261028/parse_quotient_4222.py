#!/usr/bin/env python3
"""Turn the pinned Magma quotient receipt into replayable map data."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
receipt = json.loads((HERE / "quotient_4222_receipt.json").read_text())
out = receipt["response"]["stdout"]
assert "GENUS 1\nSOURCE_ORDER 4222\nISOMORPHIC true" in out
assert "SCALAR 2\nFIBERS\n" in out


def segment(start, end):
    return json.loads(out.split(start + "\n", 1)[1].split("\n" + end, 1)[0].strip())


def code(bits):
    return sum(int(bit) << i for i, bit in enumerate(bits))


data = {
    "schema": "ghs-order4222-quotient-data-v1",
    "input_sha256": receipt["input_sha256"],
    "to_E0_exponents": segment("TO_E_EXPONENTS", "TO_E_COEFF_BITS"),
    "to_E0_coefficients": [[code(bits) for bits in row]
                           for row in segment("TO_E_COEFF_BITS", "ISO_BITS")],
    "E0_to_E_isomorphism_rst_u": [code(bits) for bits in segment("ISO_BITS", "SCALAR 2")],
}
(HERE / "quotient_map_4222.json").write_text(json.dumps(data, indent=2) + "\n")
rows = [code(bits) for bits in json.loads(out.split("FIBERS\n", 1)[1].strip())]
assert len(rows) == 12
fibers = {
    "schema": "ghs-order4222-split-fibers-v1",
    "translated_base_scalar": 2,
    "source_scalars": [2, 3, 9],
    "fields": ["x", "y0", "y1a", "y1b"],
    "fibers": {name: rows[i:i+4] for i, name in zip((0,4,8),
                ("R", "R_plus_G", "R_plus_Q"))},
}
(HERE / "preimages_4222.json").write_text(json.dumps(fibers, indent=2) + "\n")
print("PASS quotient isomorphism and three split fibers")
