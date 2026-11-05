#!/usr/bin/env python3
"""Independent F2 support check of the R104 leading term and its socle."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "runs" / "leading104-01"
R102 = ROOT.parent / "102" / "result.json"
EXPECTED_R102_SHA256 = "4d11617b41d426e8e055e90c797fa5529f3b2a6b013eedb3e953cb07e94ecbec"
assert hashlib.sha256(R102.read_bytes()).hexdigest() == EXPECTED_R102_SHA256
rows = json.loads(R102.read_text())["rows"]
support = {(r["lambda_exponent"], r["y_exponent"])
           for r in rows if r["coefficient_order"] == -9}
assert len(rows) == 105 and len(support) == 22


def mul(a, b):
    out = set()
    for i, j in a:
        for k, m in b:
            term = (i + k, j + m)
            if term in out:
                out.remove(term)
            else:
                out.add(term)
    return out


q = {(2, 0), (1, 0), (0, 0)}
q7 = {(0, 0)}
for _ in range(7):
    q7 = mul(q7, q)
expected = mul(q7, {(0, 6), (0, 2)})
assert support == expected
assert q7 == {(i, 0) for i in (14, 13, 11, 10, 8, 7, 6, 4, 3, 1, 0)}
assert all(i < 16 and j < 7 for i, j in support)
assert not {(i, j) for i, j in support if j == 0}  # y=0 residue
y1 = set()
for i, _ in support:
    if i in y1:
        y1.remove(i)
    else:
        y1.add(i)
assert not y1  # y=1 residue

# The y=1 component has (y+1)^4=0; at y=0 the expression is y^2 q^7.
# Multiplication by y is a multiple of f=y^3(y+1)^4; multiplication by
# q is a multiple of G=q^8. Thus H is nonzero in the standard basis and
# killed by the maximal ideal (y,q) of the y=0 component.
out = {
    "check": "R104_INDEPENDENT_SOCLE_PASS",
    "r102_sha256": EXPECTED_R102_SHA256,
    "leading_terms": len(support),
    "q_adic_power": 7,
    "factor": "y^2*(y+1)^4*(l^2+l+1)^7",
    "y0_component": "y^2*(l^2+l+1)^7",
    "y1_component": "0",
    "closed_point_residues": [0, 0],
}
(ROOT / "independent-replay.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print(out["check"])
