#!/usr/bin/env python3
"""Write the frozen ecbench specs of research/conductor_gap_rho_large_sample_20261018.

Configuration only: the measurement is ecbench (native). Curves come from
  (S) research/conductor_gap_cm_pairs_20261010/specs/gap-ell{8209,131101}-*.json (prime_explicit),
  (M) the native `conductor_gap` constructor (D = -19) at new 45-bit parameters,
  (L) research/conductor_gap_cm_orbits_20261015/inputs/pairs.json entries 0..23 (prime_explicit).
Isomorphic twins: (a u^4, b u^6), generator (u^2 x, u^3 y), u = 3.
"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
U = 3
MEAS_SEED = 20261018

def explicit(p, a, b, gx, gy, r):
    return {"kind": "prime_explicit", "p": p, "a": a, "b": b, "gx": gx, "gy": gy, "r": r, "group_order": r}

def twin(c):
    p = c["p"]
    return explicit(p, c["a"] * pow(U, 4, p) % p, c["b"] * pow(U, 6, p) % p,
                    c["gx"] * U * U % p, c["gy"] * pow(U, 3, p) % p, c["r"])

def arms(positive_control):
    a = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]
    if positive_control:
        a.append({"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}})
    a.append({"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}})
    return a

def spec(label, desc, curves, roles, targets, target_seed, arms_, timeout):
    return {"schema": "ecbench.spec/v1", "label": label, "description": desc + " Curve roles, in order: " + ", ".join(roles) + ".",
            "workloads": {"curves": curves, "target_kind": "planted_shared", "target_seed": target_seed, "targets_per_curve": targets},
            "arms": arms_,
            "measurement": {"rounds": 1, "warmup": 0, "order": "alternate", "seed": MEAS_SEED,
                            "isolation_required": "L2", "timeout_seconds": timeout}}

out = {}
# ---- Tier S: hard gap (kernel field degree k = 2736 and 65550), small fields, large target counts
for name, ell, targets, tseed, timeout in [("s1a-ell8209-q29", 8209, 1024, 2026101801, 120), ("s1b-ell8209-q29", 8209, 1024, 2026101803, 120),
                                            ("s1c-ell8209-q29", 8209, 1024, 2026101804, 120), ("s1d-ell8209-q29", 8209, 1024, 2026101805, 120),
                                            ("s2-ell131101-q37", 131101, 1024, 2026101802, 300)]:
    src = json.load(open(f"{ROOT}/research/conductor_gap_cm_pairs_20261010/specs/gap-ell{ell}-q{29 if ell==8209 else 37}.json"))
    cs = [explicit(c["p"], c["a"], c["b"], c["gx"], c["gy"], c["r"]) for c in src["workloads"]["curves"]]
    curves = cs + [twin(cs[0]), twin(cs[1])]
    roles = ["crater"] + [f"floor{i}" for i in range(1, len(cs))] + ["crater-twin(u=3)", "floor1-twin(u=3)"]
    out[name] = spec(f"conductor gap ell={ell}, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, {targets} shared targets",
                     f"Large-sample rho retest of research/conductor_gap_cm_pairs_20261010 at ell={ell} under fresh seeds (ell=8209: four shards a-d of 1024 shared targets each, distinct target seeds, 4096 per curve in all); rho.plain is the sqrt(2) positive control; twins are the representation-noise negative control.",
                     curves, roles, targets, tseed, arms(True), timeout)
# ---- Tier M: new hard-gap parameters at 45 bits (k = ell-1, ell-1, (ell-1)/2), native constructor
for i, (ell, t) in enumerate([(1211653, 6688791), (1138019, 10055507), (1609903, 9290211)], 1):
    curves = [{"kind": "conductor_gap", "ell": ell, "t": t, "level": "crater", "seed": 1}] + \
             [{"kind": "conductor_gap", "ell": ell, "t": t, "level": "floor", "seed": s} for s in (1, 2, 3)]
    roles = ["crater", "floor(seed1)", "floor(seed2)", "floor(seed3)"]
    out[f"m{i}-ell{ell}-q45"] = spec(f"conductor gap ell={ell} (2^{ell.bit_length()-1}..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets",
                                     f"New parameters t={t}: q=(t^2+19 ell^2)/4 and N=q+1-t prime; lambda=t/2 mod ell has order given in PROTOCOL.md, so no small-degree kernel field exists.",
                                     curves, roles, 128, 2026101810 + i, arms(False), 900)
# ---- Tier L: easy gap (twist has rational ell-torsion), ell ~ 2^25, 53 bits, pairs 0..23
pairs = json.load(open(f"{ROOT}/research/conductor_gap_cm_orbits_20261015/inputs/pairs.json"))
for name, idx, twins in [("l1-pairs00-03", range(0, 4), True), ("l2-pairs04-07", range(4, 8), True),
                         ("l3-pairs08-15", range(8, 16), False), ("l4-pairs16-23", range(16, 24), False),
                         # replication r1 (AMENDMENTS.md item 3): same pairs and twins, fresh target seeds 2026101835-38
                         ("l5r-pairs00-03", range(0, 4), True), ("l6r-pairs04-07", range(4, 8), True),
                         ("l7r-pairs08-15", range(8, 16), False), ("l8r-pairs16-23", range(16, 24), False)]:
    curves, roles = [], []
    for i in idx:
        pr = pairs[i]; assert pr["status"] == "PASS"
        c0 = explicit(pr["p"], pr["a0"], pr["b0"], pr["G0"][0], pr["G0"][1], pr["N"])
        c1 = explicit(pr["p"], pr["a1"], pr["b1"], pr["G1"][0], pr["G1"][1], pr["N"])
        curves += [c0, c1]; roles += [f"pair{i}-crater", f"pair{i}-floor"]
        if twins:
            curves.append(twin(c0)); roles.append(f"pair{i}-crater-twin(u=3)")
    out[name] = spec(f"conductor gap ell~2^25, D=-19, 53-bit, pairs {idx.start}..{idx.stop-1} of cm_orbits: crater vs floor" + (" vs crater twins" if twins else "") + ", 16 shared targets",
                     "Pairs from research/conductor_gap_cm_orbits_20261015/inputs/pairs.json (t = ell-2, so the quadratic twist has rational ell-torsion and the gap is crossable by one Velu isogeny).",
                     curves, roles, 16, (2026101830 if name.endswith("r") or "r-" in name else 2026101820) + int(name[1]), [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}], 1800)
# ---- Tier T (AMENDMENTS.md item 4): every tier-L pair with a twin (u = 5) of BOTH the crater and the floor,
# fresh target seeds 2026101841-48, 8 sessions of 3 pairs. Separates "crater representation" from "conductor".
UT = 5
def twin5(c):
    p = c["p"]
    return explicit(p, c["a"] * pow(UT, 4, p) % p, c["b"] * pow(UT, 6, p) % p, c["gx"] * UT * UT % p, c["gy"] * pow(UT, 3, p) % p, c["r"])
for k in range(8):
    idx = range(3 * k, 3 * k + 3); curves, roles = [], []
    for i in idx:
        pr = pairs[i]; assert pr["status"] == "PASS"
        c0 = explicit(pr["p"], pr["a0"], pr["b0"], pr["G0"][0], pr["G0"][1], pr["N"])
        c1 = explicit(pr["p"], pr["a1"], pr["b1"], pr["G1"][0], pr["G1"][1], pr["N"])
        curves += [c0, twin5(c0), c1, twin5(c1)]
        roles += [f"pair{i}-crater", f"pair{i}-crater-twin(u=5)", f"pair{i}-floor", f"pair{i}-floor-twin(u=5)"]
    name = f"t{k+1}-pairs{idx.start:02d}-{idx.stop-1:02d}"
    out[name] = spec(f"conductor gap ell~2^25, D=-19, 53-bit, pairs {idx.start}..{idx.stop-1}: crater, crater twin, floor, floor twin (u=5), 16 shared targets",
                     "Twin round (AMENDMENTS.md item 4): both endpoints of every pair with an isomorphic model u=5; same pairs.json pairs as tier L.",
                     curves, roles, 16, 2026101841 + k, [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}], 1800)
for name, s in out.items():
    json.dump(s, open(f"{HERE}/specs/{name}.json", "w"), indent=1)
    print(name, len(s["workloads"]["curves"]), "curves", s["workloads"]["targets_per_curve"], "targets", len(s["arms"]), "arms")
