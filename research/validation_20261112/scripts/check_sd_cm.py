#!/usr/bin/env python3
"""Independent check of the small_de curve data (validator's own arithmetic): trace, CM
discriminant and conductor (t^2 - 4p = d_K f^2), j-invariant of the requested D, lambda as a
root of the endomorphism's characteristic polynomial mod r (either sign for the +-psi maps),
and the automorphism action psi(G) = [lambda]G for D = -3, -4.
Usage: check_sd_cm.py sd_dump.jsonl small_de.json"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import EC, is_prime, tonelli
dump = {(d["bits"], d["d"]): d for d in map(json.loads, open(sys.argv[1])) if not d.get("none")}
raw = json.load(open(sys.argv[2]))
J = {-3: 0, -4: 1728, -7: -3375, -8: 8000, -11: -32768}
CP = {-3: lambda l, r: (l * l + l + 1) % r, -4: lambda l, r: (l * l + 1) % r, -7: lambda l, r: (l * l - l + 2) % r,
      -8: lambda l, r: (l * l + 2) % r, -11: lambda l, r: (l * l - l + 3) % r}
seen = set(); errs = []; n = 0
for c in raw["cells"]:
    i = c["inst"]; key = (i["bits"], i["d_requested"])
    if key in seen: continue
    seen.add(key); n += 1
    d = dump[key]; p, a, b, r = i["p"], i["a"], i["b"], i["r"]; D = i["d_requested"]
    chk = {"t": i["t"] == p + 1 - i["n"], "hasse": i["t"] ** 2 <= 4 * p, "p_bits": p.bit_length() == i["bits"], "r_prime": is_prime(r),
           "n_hr": i["n"] == i["h"] * r, "dump_match": (d["p"], d["a"], d["b"], d["r"]) == (p, a, b, r)}
    disc = i["t"] ** 2 - 4 * p
    if i["d_k"] is not None and i["conductor_of_pi"] is not None:
        chk["disc_eq_dK_f2"] = disc == i["d_k"] * i["conductor_of_pi"] ** 2
    if D != 0:
        chk["d_k_eq_D"] = i["d_k"] == D
        num = 4 * a ** 3 % p; den = (4 * a ** 3 + 27 * b * b) % p
        chk["j_eq_jD"] = den != 0 and 1728 * num * pow(den, -1, p) % p == J[D] % p
        lam = i["lambda"]
        chk["lambda_root"] = CP[D](lam, r) == 0 or CP[D]((-lam) % r, r) == 0
        E = EC(p, 0, a, b); G = (d["g"][0], d["g"][1]); LG = E.mul(lam, G)
        if D == -3:
            # omega: a primitive cube root of unity mod p; psi(x, y) = (omega x, y) for one of the two
            w = None
            for g in range(2, 200):
                w = pow(g, (p - 1) // 3, p)
                if w != 1: break
            chk["psi_G_eq_lambda_G"] = LG in ((w * G[0] % p, G[1]), (w * w * G[0] % p, G[1]))
        if D == -4:
            ii = tonelli(p - 1, p)
            chk["psi_G_eq_lambda_G"] = LG in (((-G[0]) % p, ii * G[1] % p), ((-G[0]) % p, (-ii * G[1]) % p))
    else:
        chk["generic_large_dK"] = i["d_k"] is None or abs(i["d_k"]) > 100
    bad = [k for k, v in chk.items() if v is not True]
    if bad: errs.append((key, bad))
print(f"curves checked {n}; errors {len(errs)}")
for e in errs: print("  ", e)
sys.exit(1 if errs else 0)
