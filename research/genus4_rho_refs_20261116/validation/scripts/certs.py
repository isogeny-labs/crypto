#!/usr/bin/env python3
"""Independent certificates for the genus-4 instances of both studies.

For every (p, curve_seed, target_seed) that a run row of either study uses:
  - f = random_f(p, 4, curve_seed) equals the instance row's f; f squarefree (genus 4);
  - r prime (deterministic Miller-Rabin), order = h*r, h <= 2;
  - Hasse-Weil: lo <= h*r <= hi, and r > hi - lo, so h*r is the only multiple of r
    in the interval; with G != 0 and [r]G = 0 (r prime), r | #J, hence #J = h*r;
  - [order]D = 0 for 4 random divisors (consistency);
  - G, t re-derived from instance_from's seeding with the validator's arithmetic;
    G != 0, [r]G = 0; T = [t]G; [t]G checked again via [t-k]G + [k]G;
  - the instance equals the r-bound study's instance row (G1 instance part).
Writes certs.json (list of per-target records with G, T, t in Mumford form)."""
import json, random, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jac import Jac, random_f, instance_from, is_prime, hasse_weil_bounds, random_divisor, M64

ROOT = sys.argv[1]
OUT = sys.argv[2]
S1 = f"{ROOT}/research/genus4_positive_control_20261116/run.jsonl"
S2 = f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl"
BR = f"{ROOT}/research/genus_ladder_bound_r_20261114/bound_r.jsonl"


def load(fn):
    return [json.loads(l) for l in open(fn)]


inst_rows = {}
for fn in (S1, S2):
    for r in load(fn):
        if r["kind"] == "instance":
            key = (r["p"], r["curve_seed"])
            if key in inst_rows:
                assert inst_rows[key] == r, ("instance rows differ between studies", key)
            inst_rows[key] = r
br = {(r["p"], r["curve_seed"]): r for r in load(BR) if r["kind"] == "instance" and r["g"] == 4}

targets = set()
for fn, study in ((S1, 1), (S2, 2)):
    for r in load(fn):
        if r["kind"] == "run":
            ts = r["target_seed"] if "target_seed" in r else r["curve_seed"]  # study 1 rows carry no target_seed
            targets.add((r["p"], r["curve_seed"], ts))

rng = random.Random(20261116)
out = []
allok = True
for (p, cs, ts) in sorted(targets):
    ir = inst_rows[(p, cs)]
    chk = {}
    f = random_f(p, 4, cs)
    chk["f_matches_row"] = f == ir["f"]
    chk["f_matches_bound_r"] = br.get((p, cs), {}).get("f") == f
    chk["order_h_r_match_bound_r"] = all(br[(p, cs)][k] == ir[k] for k in ("order", "h", "r"))
    J = Jac(p, f)
    chk["squarefree_genus4"] = J.squarefree and J.g == 4
    h, r, order = ir["h"], ir["r"], ir["order"]
    chk["r_prime"] = is_prime(r)
    chk["order_eq_h_r"] = order == h * r
    chk["h_le_2"] = h <= 2
    lo, hi = hasse_weil_bounds(p, 4)
    chk["order_in_hasse_weil"] = lo <= order <= hi
    chk["r_gt_interval_width"] = r > hi - lo
    # random divisors killed by the order
    st = rng.getrandbits(64)
    killed = True
    for _ in range(4):
        D, st = random_divisor(J, st)
        killed &= J.is_zero(J.mul(D, order))
    chk["order_kills_random_divisors"] = killed
    G, t = instance_from(J, h, r, ts)
    chk["G_valid"] = J.valid(G) and not J.is_zero(G)
    chk["rG_zero"] = J.is_zero(J.mul(G, r))
    T = J.mul(G, t)
    k = rng.randrange(1, r)
    chk["tG_two_chains_agree"] = J.add(J.mul(G, (t - k) % r), J.mul(G, k)) == T
    chk["T_valid"] = J.valid(T) and not J.is_zero(T)
    chk["t_in_range"] = 1 <= t < r
    ok = all(chk.values())
    allok &= ok
    out.append({"p": p, "curve_seed": cs, "target_seed": ts, "h": h, "r": r, "order": order,
                "t": t, "G": [list(G[0]), list(G[1])], "T": [list(T[0]), list(T[1])],
                "hasse_weil": [lo, hi], "checks": chk, "all_pass": ok})
    print(p, cs, ts, "t =", t, "PASS" if ok else "FAIL " + str([k for k, v in chk.items() if not v]))
json.dump(out, open(OUT, "w"), indent=1)
print("targets:", len(out), "all pass:", allok)
