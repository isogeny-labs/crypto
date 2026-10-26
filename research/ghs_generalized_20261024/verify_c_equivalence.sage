"""Independently normalize every low-genus quadratic GHS parameter.

Magma's documented Artin--Schreier presentation is
    v^2 + v = c/z + a + d*z,       c*d = sqrt(b).
For d in k, w=d*z gives sqrt(b)/w + w.  For c in k, w=c/z
gives the same expression.  Both changes of variable are over k.

The second branch has w=X rather than the canonical w=sqrt(b)/X.
On the original elliptic curve, the two point presentations differ by
P -> T-P, where T=(0,sqrt(b)) is rational 2-torsion.  We check this
coordinate identity on every nonzero point of each tested subgroup.
"""

import json
from pathlib import Path
from sage.all import EllipticCurve, GF, PolynomialRing, is_prime


HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"
SPECS = (
    (6, 0x43, 0, 32, 19, (6, 31), (6, 25), "screen-n6-a0-b20.json", "exact-n6-a0-b32-direct.json"),
    (8, 0x11B, 0, 32, 67, (244, 61), (4, 8), "screen-n8-a0-b20.json", "exact-n8-a0-b32-direct.json"),
    (10, 0x409, 0, 32, 43, (262, 371), (626, 155), "screen-n10-a0-b20.json", "exact-n10-a0-b32-direct.json"),
    (12, 0x1009, 1, 512, 199, (23, 94), (2212, 1499), "screen-n12-a1-b200.json", "exact-n12-a1-b512-direct.json"),
)


def word(value):
    return sum(int(coefficient) << i for i, coefficient in enumerate(value.polynomial().list()))


def audit():
    out = []
    f2 = GF(2)
    t = PolynomialRing(f2, "t").gen()
    for n, polynomial, a, b, r, pxy, qxy, screen_file, receipt_file in SPECS:
        modulus = sum(t**i for i in range(n + 1) if (polynomial >> i) & 1)
        field = GF(2**n, "z", modulus=modulus)
        z = field.gen()
        elt = lambda bits: sum(z**i for i in range(n) if (bits >> i) & 1)
        beta = elt(b) ** (2 ** (n - 1))
        assert beta * beta == elt(b)
        s = n // 2
        q = 2**s
        assert beta**q != beta
        assert is_prime(r)
        curve = EllipticCurve(field, [field.one(), elt(a), field.zero(), field.zero(), elt(b)])
        p = curve(elt(pxy[0]), elt(pxy[1]))
        target = curve(elt(qxy[0]), elt(qxy[1]))
        torsion = curve(field.zero(), beta)
        assert p.order() == r and 2*torsion == curve(0)
        assert target in [j*p for j in range(r)]
        for j in range(1, r):
            x, y = (j*p).xy()
            xr, yr = (torsion - j*p).xy()
            assert x != 0 and xr == beta/x
            assert (yr + beta)/xr == (y + beta)/x

        base_elements = [elt(j) for j in range(1, 2**n) if elt(j)**q == elt(j)]
        assert len(base_elements) == q-1
        profiles = []
        for c_word in range(1, 2**n):
            c = elt(c_word)
            d = beta/c
            c_fixed, d_fixed = c**q == c, d**q == d
            if not (c_fixed or d_fixed):
                continue
            assert c_fixed != d_fixed
            assert c*d == beta
            # Verify the rational substitution at every z in the base field;
            # the coefficient identity above proves it as a function identity.
            for base_z in base_elements:
                w = d*base_z if d_fixed else c/base_z
                assert c/base_z + d*base_z == beta/w + w
            profiles.append({
                "c": f"0x{c_word:x}",
                "d": f"0x{word(d):x}",
                "branch": "d_in_subfield" if d_fixed else "c_in_subfield",
                "base_change": "w=d*z" if d_fixed else "w=c/z",
                "induced_degree_zero_map_sign": 1 if d_fixed else -1,
            })
        counts = {
            "d_in_subfield": sum(p["branch"] == "d_in_subfield" for p in profiles),
            "c_in_subfield": sum(p["branch"] == "c_in_subfield" for p in profiles),
        }
        assert counts == {"d_in_subfield": q-1, "c_in_subfield": q-1}
        screen = json.loads((RESULTS / screen_file).read_text())
        recorded = next(row for row in screen["base_rows"] if row["base_degree"] == s)
        assert recorded["genus_histogram"].get("2", 0) == 2*(q-1)
        receipt = json.loads((RESULTS / receipt_file).read_text())
        assert receipt["ghs_status"] == "verified"
        assert receipt["ghs"]["map"]["beta"] == f"0x{word(beta):x}"
        assert receipt["ghs"]["map"]["cover_degree"] == 2
        out.append({
            "absolute_degree": n,
            "modulus": f"0x{polynomial:x}",
            "a": f"0x{a:x}",
            "b": f"0x{b:x}",
            "canonical_c": f"0x{word(beta):x}",
            "canonical_d": "0x1",
            "subgroup_order": r,
            "checked_nonzero_subgroup_points": r-1,
            "base_degree": s,
            "canonical_receipt": receipt_file,
            "genus_two_profiles": len(profiles),
            "branches": counts,
            "profiles": profiles,
        })
        print(f"n={n}: {len(profiles)} genus-two c profiles normalize; {r-1} nonzero subgroup points checked")
    return {
        "schema": "ghs-quadratic-c-equivalence/v1",
        "normal_form": "v^2+v=sqrt(b)/w+a+w",
        "coordinate_result": "For c in k, w=X; for d in k, w=sqrt(b)/X. The former equals the latter on T-P with T=(0,sqrt(b)), with the same Artin-Schreier ordinate.",
        "map_scope": "The degree-zero pullback maps are identified up to sign by this coordinate change; per-c Jacobian arithmetic is not independently executed.",
        "cases": out,
        "total_profiles": sum(case["genus_two_profiles"] for case in out),
    }


if __name__ == "__main__":
    output = audit()
    assert output["total_profiles"] == 232
    path = RESULTS / "c-equivalence-audit.json"
    path.write_text(json.dumps(output, indent=2) + "\n")
    print(f"wrote {path}")
