#!/usr/bin/env python3
"""Independent check of small_de instances (dumped from committed construct()) and of
every recovered target log in a small_de.json (replayed raw data)."""
import json, sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
dump = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
raw = json.load(open(sys.argv[2]))
random.seed(7)
inst = {}; errs = []
for d in dump:
    if d.get("none"): errs.append(("no instance", d)); continue
    p, a, b, n, h, r = d["p"], d["a"], d["b"], d["n"], d["h"], d["r"]
    E = EC(p, 0, a, b)
    G = (d["g"][0], d["g"][1]); assert not d["g"][2]
    chk = {"p_prime": is_prime(p), "r_prime": is_prime(r), "p_bits": p.bit_length() == d["bits"], "n_eq_hr": n == h * r,
           "h_le_4": h <= 4, "r_gt_p_over_5": 5 * r > p, "hasse": abs(p + 1 - n) <= 2 * int(p ** 0.5) + 2,
           "G_on_curve": E.on(G), "rG_zero": E.mul(r, G) is None, "G_nonzero": True}
    ok_n = True
    for _ in range(4):
        while True:
            x = random.randrange(p); y2 = E.rhs(x)
            if y2 == 0 or pow(y2, (p - 1) // 2, p) == 1: break
        # sqrt
        y = tonelli(y2, p)
        if E.mul(n, (x, y)) is not None: ok_n = False
    chk["n_kills_random_points"] = ok_n
    for k, t in enumerate(d["targets"]):
        T = None if t["inf"] else (t["x"], t["y"])
        if E.mul(t["t"], G) != T: chk[f"target{k}_eq_tG"] = False
    bad = [k for k, v in chk.items() if v is not True]
    if bad: errs.append((d["bits"], d["d"], bad))
    inst[(d["bits"], d["d"])] = (E, G, d)
ncert = 0; nfail = 0; nrho = 0
for cell in raw["cells"]:
    E, G, d = inst[(cell["bits"], cell["d"])]
    if cell["inst"]["p"] != d["p"] or cell["inst"]["r"] != d["r"] or cell["inst"]["a"] != d["a"] or cell["inst"]["b"] != d["b"]:
        errs.append(("instance mismatch", cell["bits"], cell["d"])); continue
    for arm in cell["arms"]:
        assert len(arm["targets"]) == 16
        for k, ts in enumerate(arm["targets"]):
            t = d["targets"][k]; T = None if t["inf"] else (t["x"], t["y"])
            kk = ts["recovered"]
            ok = kk is not None and kk == t["t"] and E.mul(kk, G) == T
            ncert += ok; nfail += (not ok)
            if ok != ts["certified"]: errs.append(("certified flag disagrees", cell["bits"], cell["m"], cell["d"], arm["arm"], k))
    for rho in cell["rho"]:
        for run in rho["runs"]:
            nrho += 1
            if not (run["certified"] and run["verified"]): errs.append(("rho run not certified", cell["bits"], cell["m"], cell["d"], rho["kind"]))
        if len(rho["runs"]) != 16: errs.append(("rho runs != 16", cell["bits"], cell["m"], cell["d"], rho["kind"], len(rho["runs"])))
print(f"instances {len(inst)}; target logs independently certified {ncert}, failed {nfail}; rho runs {nrho}; errors {len(errs)}")
for e in errs[:20]: print("  ", e)
