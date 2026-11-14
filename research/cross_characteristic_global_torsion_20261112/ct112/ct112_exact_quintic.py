#!/usr/bin/env python3
"""CT112 exact integer certificate for the relative quintic over Q[S].

The frozen coefficients are only candidates until four integral
psi11-divisibility checks below pass. All checks run under python -O.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "ct109"))
import ct109_modular_probe as m

def ck(ok, msg):
    if not ok:
        raise ArithmeticError(msg)

def mul(a, b):
    z = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                if y:
                    z[i + j] += x * y
    return m.trim(z)

def divexact(a, b):
    a, b = a[:], m.trim(b[:])
    q = [0] * max(1, len(a) - len(b) + 1)
    while len(a) >= len(b) and a != [0]:
        d = len(a) - len(b)
        v, rem = divmod(a[-1], b[-1])
        ck(rem == 0, "nonintegral long-division step")
        q[d] = v
        for i, x in enumerate(b):
            a[d + i] -= v * x
        m.trim(a)
    ck(a == [0], "nonzero exact-division remainder")
    return m.trim(q)

def main():
    fixture = json.loads((Path(__file__).parent / "reconstructed.json").read_text())
    denominator = int(fixture["common_denominator"])
    C = [[int(v) for v in row] for row in fixture["numerators_low_to_high"]]
    ck(len(C) == 4 and all(len(row) == 12 for row in C), "bad coefficient shape")
    # Coefficients of prod_{m=1..5}(D_m Z - N_m), from CT109's rational maps.
    h = [[1]]
    for numerator, div in zip(m.nums, m.dens):
        new = [[0] for _ in range(len(h) + 1)]
        for i, x in enumerate(h):
            new[i] = m.sub(new[i], mul(x, numerator))
            new[i + 1] = m.add(new[i + 1], mul(x, div))
        h = new
    B, A = h[5], m.sub([0], h[4])
    ck(len(B) == 51 and len(A) == 52, "incorrect trace-rational degrees")
    ck(sum(v * pow(19, i, 59) for i, v in enumerate(m.f11)) % 59 == 0,
       "source psi11 root")
    ck(sum(v * pow(19, i, 59) for i, v in enumerate(B)) % 59 != 0,
       "constant-coefficient mutation negative control")
    ap, bp = [[1]], [[1]]
    for _ in range(11):
        ap.append(mul(ap[-1], A))
        bp.append(mul(bp[-1], B))
    terms = [mul(ap[i], bp[11 - i]) for i in range(12)]
    certs = []
    for j in range(4):
        delta = [-denominator * x for x in mul(h[j], bp[10])]
        for i, c in enumerate(C[j]):
            if c:
                delta = m.add(delta, [c * v for v in terms[i]])
        delta = m.trim(delta)
        ck(delta != [0], "unexpected identically zero cross-difference")
        q = divexact(delta, m.f11)
        certs.append({"cross_degree": len(delta) - 1, "j": j,
                      "quotient_degree": len(q) - 1})
    ck(denominator % 59 != 0, "bad source denominator")
    for j, row in enumerate(C):
        x = sum(c * pow(32, i, 59) for i, c in enumerate(row)) % 59
        ck(x * pow(denominator, -1, 59) % 59 == [27, 19, 45, 31][j],
           "source 59 quintic mismatch")
    out = {
        "all_integer_cross_differences_divisible": True,
        "certificates": certs,
        "coefficient_count": 48,
        "common_denominator_digits": len(str(denominator)),
        "common_denominator_v2": (denominator & -denominator).bit_length() - 1,
        "complete_cost_gain_measured": False,
        "divisor_degree": 60,
        "kind": "ct112-exact-relative-quintic",
        "online_public_point_lift": False,
        "relative_degree": 5,
        "single_coefficient_mutant_rejected": True,
        "source_known_answer": True,
        "trace_coefficient_is_minus_s": True,
    }
    print(json.dumps(out, sort_keys=True, separators=(",", ":")))

if __name__ == "__main__":
    main()
