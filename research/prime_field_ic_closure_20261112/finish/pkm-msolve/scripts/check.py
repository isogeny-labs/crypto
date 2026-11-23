#!/usr/bin/env python3
"""Independent checks of PRESET section 5 on every finished run.

(i)   unit (GB == {1}) iff the grid enumerator finds 0 zeros (random and planted draws)
(ii)  planted: GB non-unit and every GB element vanishes at the planted point
(iii) consistent random draws: every GB element vanishes at enumerated solutions
      (a seeded sample of at most 40 solutions per draw)
Writes OUT/checks.jsonl.  Usage: check.py OUTDIR
"""
import json
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen as G  # noqa: E402
import msparse as MP  # noqa: E402


def recover_c0(path):
    """(A, B, g) of a repo system, as validate_gen.py recovers them"""
    import validate_gen as VG
    names, p, polys = MP.read_ms(path)
    VG.P_MOD[0] = p
    m = 1 + max(int(n[1:].split("_")[0]) for n in names if n.startswith("y"))
    t = 1 + max(int(n.split("_")[1]) for n in names if n.startswith("y"))
    nv = len(names)
    zero = tuple([0] * nv)
    first = polys[0]
    u0, y00 = names.index("u0"), names.index("y0_0")
    e_u = [0] * nv
    e_u[u0] = 1
    B = (-first.get(tuple(e_u), 0)) * pow(4, p - 2, p) % p
    e_g = [0] * nv
    e_g[y00] = 2
    e_g[u0] = 2
    A_c = G.sqrt_mod(first.get(zero, 0), p)
    g_c = G.sqrt_mod(first.get(tuple(e_g), 0), p)
    xR = int(path.split("_x")[-1].split(".")[0])
    target = set(map(VG.canon, polys))
    for A in (A_c, (-A_c) % p):
        for g in (g_c, (-g_c) % p):
            _, eqs, _ = G.build_system(p, A, B, g, m, t, xR)
            if set(map(VG.canon, eqs)) == target:
                return p, A, B, g, m, t, xR
    raise RuntimeError("no match")


def mu(p, t):
    rng = random.Random(5)
    z = G.root_of_unity(p, t, rng)
    return [pow(z, k, p) for k in range(1 << t)]


def main():
    out = sys.argv[1]
    rd = os.path.join(out, "runs")
    sd = os.path.join(out, "systems")
    done = set()
    cf = os.path.join(out, "checks.jsonl")
    if os.path.exists(cf):
        for ln in open(cf):
            done.add(json.loads(ln)["name"])
    names = sorted(f[:-9] for f in os.listdir(rd) if f.endswith(".res.json"))
    for name in names:
        if name in done:
            continue
        res = json.load(open(os.path.join(rd, name + ".res.json")))
        if res.get("placeholder"):
            continue  # claimed by a runner, not finished
        meta = json.load(open(os.path.join(sd, name + ".json")))
        rec = dict(name=name, exit=res["exit"], stop=res["stop"], unit=res.get("unit"))
        if res["exit"] != 0:
            rec["check"] = "not_applicable_unfinished"
            with open(cf, "a") as fh:
                fh.write(json.dumps(rec) + "\n")
            print(json.dumps(rec), flush=True)
            continue
        if meta["arm"] == "C0":
            p, A, B, g, m, t, xR = recover_c0(meta["source"])
            V = [g * w % p for w in mu(p, t)]
            yR = None
        else:
            p, A, B, g, m, t, xR = meta["p"], meta["A"], meta["B"], meta["g"], meta["m"], meta["t"], meta["xR"]
            V = meta["V"]
            yR = meta.get("yR")
        F = G.Fp2(p)
        # enumerate (R as a point over F_{p^2}; yR in F_p for every arm except C0 where we take any root)
        if yR is None:
            w = F.sqrt_of_fp((xR ** 3 + A * xR + B) % p)
            yR_pt = w
        else:
            yR_pt = (yR % p, 0)
        zeros, sols = enumerate_with_point(p, A, B, V, m, xR, yR_pt)
        rec["grid_zeros"] = len(zeros)
        rec["chain_solutions_found"] = len(sols)
        ok_i = (res.get("unit") is True) == (len(zeros) == 0)
        rec["check_i_unit_iff_no_zero"] = ok_i
        gb = None
        vn = [f"y{i}_{j}" for i in range(m) for j in range(t)] + [f"u{k}" for k in range(m - 2)]
        if not res.get("unit"):
            gb = MP.read_gb(os.path.join(rd, name + ".out"), vn, p)
            rec["gb_len_parsed"] = len(gb)
        if meta.get("kind") == "planted":
            pt = [(meta["planted"][nm] % p, 0) for nm in vn]
            vals = [MP.eval_poly(P, pt, F) for P in gb] if gb else None
            rec["check_ii_planted"] = bool(gb) and all(v == (0, 0) for v in vals)
        if gb and sols:
            rng = random.Random(name)
            sample = sols if len(sols) <= 40 else rng.sample(sols, 40)
            bad = 0
            for tup, us in sample:
                pt = []
                for i in range(m):
                    yv = V[tup[i]] * pow(g, p - 2, p) % p
                    for j in range(t):
                        pt.append((yv, 0))
                        yv = yv * yv % p
                pt += list(us)
                if any(MP.eval_poly(P, pt, F) != (0, 0) for P in gb):
                    bad += 1
            rec["check_iii_sample"] = len(sample)
            rec["check_iii_bad"] = bad
        with open(cf, "a") as fh:
            fh.write(json.dumps(rec) + "\n")
        print(json.dumps(rec), flush=True)


def enumerate_with_point(p, A, B, V, m, xR, Rpt_y):
    """enumerate_zeros with R = (xR, Rpt_y) given as an F_{p^2} y-coordinate"""
    F = G.Fp2(p)
    sp = G.signed_points(F, A, B, V)
    R = ((xR % p, 0), Rpt_y)
    negR = G.ec_neg(F, R)
    h = (m + 1) // 2
    nV = len(V)

    def walk(k):
        res = [((), [], None)]
        for _ in range(k):
            nxt = []
            for idx, partial, S in res:
                for vi in range(nV):
                    for s, P in sp[vi]:
                        S2 = G.ec_add(F, A, S, P)
                        nxt.append((idx + (vi,), partial + [S2], S2))
            res = nxt
        return res

    left = walk(h)
    right = walk(m - h)
    rmap = {}
    for idx, partial, S in right:
        rmap.setdefault(S, []).append((idx, partial))
    zeros, sols = set(), []
    for idx, partial, SL in left:
        # global sign fixed: first summand's sign +1  (sp[vi][0] is the + point)
        if partial[0] != sp[idx[0]][0][1]:
            continue
        for target in (R, negR):
            need = G.ec_add(F, A, target, G.ec_neg(F, SL))
            for ridx, rpartial in rmap.get(need, ()):
                full = list(partial) + [G.ec_add(F, A, SL, rp) for rp in rpartial]
                if not all(full[k] is not None for k in range(1, m - 1)):
                    continue
                tup = idx + ridx
                zeros.add(tup)
                if len(sols) < 200000:
                    sols.append((tup, [full[k][0] for k in range(1, m - 1)]))
    return zeros, sols


if __name__ == "__main__":
    main()
