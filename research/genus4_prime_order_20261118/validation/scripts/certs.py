#!/usr/bin/env python3
"""C5: independent certificates in the validator's own arithmetic (jac.py, polyfp.py).

Per confirmation curve (5 per prime): f re-derived from the seed equals the instance row and the library dump;
f monic of degree 9, squarefree, irreducible (DDF; so infinity is the only rational Weierstrass point);
r prime (deterministic Miller-Rabin below 3.3e24); order = r, h = 1; lo <= r <= hi in the exact Hasse-Weil
interval and 2 lo > hi; F_p-automorphisms: with f irreducible every rational automorphism fixes infinity, so it is
x -> a x + b with f(a x + b) = a^9 f(x); comparing x^8 coefficients forces b = (a - 1) f_8 / 9, and all p - 1
values of a are scanned (positive controls with planted automorphisms must be found).
Per key (6 targets per curve, 60 per prime): G re-derived from the target seed (instance_from), G != 0,
[r]G = 0 (so ord G = r, r | #J, and #J = r because 2r > hi), t re-derived, T = [t]G; G, T, t equal the library
dump (out/replay/h_inst_all.jsonl).
Per recovered log (out/replay/h_log_*.jsonl): recovered = t and [recovered]G = T in the validator's arithmetic,
and the dumped counts equal the committed row.
Per drawn rejected seed (out/replay/h_ord_*.jsonl): the library's order N is composite, lo <= N <= hi, f is
irreducible, and [N]D = 0 for a non-zero divisor D of the validator's own; then #J is not prime (if #J = q were
prime, [N]D = 0 would give q | N and N < 2q, so N = q).  A tamper control (t + 1, a count + 1, one T coefficient)
must be caught.  Writes out/certs.json."""
import glob, sys, os, random
sys.path.insert(0, os.path.dirname(__file__))
from common import *
from jac import Jac, random_f, instance_from, random_divisor, is_prime, hasse_weil_bounds, trim, mix64
from polyfp import squarefree, reducible_certificate

R = os.path.join(OUT, "replay")
CHOSEN = {16411: [373, 455, 548, 752, 875], 32771: [355, 882, 1461, 1623, 2374]}
fails = []


def load_jsonl(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def poly_compose_affine(f, a, b, p):
    acc = []
    for c in reversed(f):
        nxt = [0] * (len(acc) + 1)
        for i, x in enumerate(acc):
            nxt[i] = (nxt[i] + x * b) % p
            nxt[i + 1] = (nxt[i + 1] + x * a) % p
        nxt[0] = (nxt[0] + c) % p
        acc = nxt
    return acc


def affine_automorphisms(f, p):
    inv9 = pow(9, p - 2, p)
    found = []
    for a in range(1, p):
        b = (a - 1) * f[8] % p * inv9 % p
        g = poly_compose_affine(f, a, b, p)
        a9 = pow(a, 9, p)
        if all(g[i] == a9 * f[i] % p for i in range(10)):
            found.append((a, b))
    return found


def as_div(uv):
    return (trim(uv[0]), trim(uv[1]))


def main():
    res = {"curves": {}, "keys": 0, "keys_ok": 0, "logs": [], "ord": [], "controls": {}}
    dump_inst = {(r["p"], r["curve_seed"], r["target_seed"]): r for r in load_jsonl(os.path.join(R, "h_inst_all.jsonl"))}
    rows = {}
    for p in (16411, 32771):
        for arm, fn in (("x", f"b_p{p}_x.jsonl.gz"), ("xstar", f"b_p{p}_xstar.jsonl.gz")):
            for r in rows_of(f"{D}/{fn}")[0]:
                if r["kind"] == "run":
                    rows[(arm, p, r["curve_seed"], r["target_seed"], r["seed"])] = r
                elif r["kind"] == "instance":
                    rows[("inst", p, r["curve_seed"])] = r
    for r in rows_of(f"{D}/b_p16411_q.jsonl.gz")[0]:
        if r["kind"] == "run":
            rows[("Q", 16411, r["curve_seed"], r["target_seed"], r["seed"])] = r
    Gs = {}
    # positive controls for the automorphism scan
    pc_p = 16411
    pc1 = [0, 7, 0, 11, 0, 5, 0, 3, 0, 1]                    # odd polynomial: x -> -x
    pc2 = [5, 0, 0, 9, 0, 0, 4, 0, 0, 1]                     # f(x) = g(x^3): x -> zeta_3 x (16411 = 1 mod 3)
    res["controls"]["automorphism_scan"] = {
        "odd_f_finds_a=-1": (pc_p - 1, 0) in affine_automorphisms(pc1, pc_p),
        "f(x^3)_finds_order_3": sum(1 for a, b in affine_automorphisms(pc2, pc_p) if a != 1) == 2}
    if not all(res["controls"]["automorphism_scan"].values()):
        fails.append("automorphism positive control")
    for p in (16411, 32771):
        lo, hi = hasse_weil_bounds(p, 4)
        for cs in CHOSEN[p]:
            row = rows[("inst", p, cs)]
            f = random_f(p, 4, cs)
            J = Jac(p, f)
            r = row["r"]
            aut = affine_automorphisms(f, p)
            c = {"f_matches_row": f == row["f"], "monic_deg9": len(f) == 10 and f[-1] == 1, "squarefree": J.squarefree,
                 "irreducible_validator_ddf": reducible_certificate(f, p) is None,
                 "r_prime": is_prime(r), "order_eq_r_h1": row["order"] == r and row["h"] == 1,
                 "hasse_weil": lo <= r <= hi, "two_lo_gt_hi": 2 * lo > hi, "log2_r": r.bit_length(),
                 "affine_automorphisms": aut, "only_identity": aut == [(1, 0)]}
            c["ok"] = all(v for k, v in c.items() if k not in ("log2_r", "affine_automorphisms"))
            res["curves"][f"{p}:{cs}"] = c
            if not c["ok"]:
                fails.append(f"curve {p} {cs}")
            for k in range(6):
                ts = cs + k * 1000003
                G, t = instance_from(J, 1, r, ts)
                T = J.mul(G, t)
                dm = dump_inst[(p, cs, ts)]
                ok = (not J.is_zero(G) and J.valid(G) and J.is_zero(J.mul(G, r)) and J.valid(T)
                      and as_div(dm["G"]) == G and as_div(dm["T"]) == T and dm["t"] == t and dm["f"] == f
                      and dm["r"] == r and dm["order"] == r and dm["h"] == 1)
                Gs[(p, cs, ts)] = (J, G, T, t, r)
                res["keys"] += 1
                res["keys_ok"] += ok
                if not ok:
                    fails.append(f"key {p} {cs} {ts}")
    # recovered logs
    tamper = {"t+1": 0, "count+1": 0, "T coeff": 0, "tried": 0}
    for fn in sorted(glob.glob(os.path.join(R, "h_log_*.jsonl"))):
        arm_file = os.path.basename(fn)
        for r in load_jsonl(fn):
            if r["kind"] != "run":
                continue
            p, cs, ts, s = r["p"], r["curve_seed"], r["target_seed"], r["seed"]
            J, G, T, t, order_r = Gs[(p, cs, ts)]
            arm = "Q" if r["arm"] == "Q" else ("xstar" if "_xstar_" in arm_file else "x")
            com = rows[(arm, p, cs, ts, s)]
            rec = r["recovered"]
            fields = ["group_ops"] + (["rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations", "fb_cols"] if arm != "Q"
                                      else ["steps", "native"])
            counts_equal = all(r[k] == com[k] for k in fields) and (arm != "Q" or r["compares"] == com["compares_uncharged"])
            log_ok = rec == t and J.mul(G, rec) == T
            e = {"file": arm_file, "arm": arm, "p": p, "cs": cs, "ts": ts, "seed": s, "recovered_eq_t": rec == t,
                 "recovered_G_eq_T": log_ok, "certified": r["certified"], "ok": r["ok"], "counts_equal_committed": counts_equal,
                 "committed_certified_ok": com["certified"] and com["ok"]}
            res["logs"].append(e)
            if not (log_ok and counts_equal and r["certified"] and r["ok"] and e["committed_certified_ok"]):
                fails.append(f"log {arm} {p} {cs} {ts} {s}")
            if tamper["tried"] < 3:
                tamper["tried"] += 1
                tamper["t+1"] += J.mul(G, (rec + 1) % order_r) != T
                tamper["count+1"] += (r["group_ops"] + 1) != com["group_ops"]
                u, v = T
                T2 = (u, trim(((v[0] + 1) % p,) + tuple(v[1:])) if v else (1,))
                tamper["T coeff"] += J.mul(G, rec) != T2
    res["tamper_control"] = tamper
    if not (tamper["t+1"] == tamper["count+1"] == tamper["T coeff"] == tamper["tried"] == 3):
        fails.append("tamper control")
    # rejected seeds: #J not prime
    rnd_state = 0x5EED_0000_0000_0001
    for fn in sorted(glob.glob(os.path.join(R, "h_ord_*.jsonl"))):
        for r in load_jsonl(fn):
            p, cs = r["p"], r["cs"]
            lo, hi = hasse_weil_bounds(p, 4)
            f = random_f(p, 4, cs)
            e = {"p": p, "cs": cs, "error": r.get("err")}
            if "order" in r:
                N = r["order"]
                J = Jac(p, f)
                D_, rnd_state = random_divisor(J, mix64(rnd_state ^ cs))
                e.update(order=N, composite=not is_prime(N), in_hasse_weil=lo <= N <= hi, two_lo_gt_hi=2 * lo > hi,
                         f_matches=r["f"] == f, irreducible=reducible_certificate(f, p) is None,
                         D_nonzero=not J.is_zero(D_), N_kills_D=J.is_zero(J.mul(D_, N)),
                         smallest_factor=next((q for q in range(3, 10 ** 6, 2) if N % q == 0), None))
                e["certifies_not_prime"] = all(e[k] for k in ("composite", "in_hasse_weil", "two_lo_gt_hi", "f_matches",
                                                                "irreducible", "D_nonzero", "N_kills_D"))
            else:
                e["certifies_not_prime"] = False
            res["ord"].append(e)
            if not e["certifies_not_prime"]:
                fails.append(f"ord {p} {cs}")
    res["n_logs"] = len(res["logs"])
    res["n_ord"] = len(res["ord"])
    res["fail"] = fails
    res["ok"] = not fails
    dump("certs.json", res)
    print({k: v for k, v in res.items() if k not in ("curves", "logs", "ord")})
    for k, v in res["curves"].items():
        print(k, v["ok"], v["log2_r"], v["affine_automorphisms"])
    for e in res["ord"]:
        print(e)


if __name__ == "__main__":
    main()
