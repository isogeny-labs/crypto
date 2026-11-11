#!/usr/bin/env python3
"""Toy elimination-degree vs tower depth t for Kummer-style PKM towers over F_p (stdlib only)."""
import sys, json, random, time, itertools, statistics
P = 101
T0 = time.time()

def monos(nv, d):
    out = []
    def rec(i, left, cur):
        if i == nv:
            out.append(tuple(cur)); return
        for e in range(left + 1):
            cur.append(e); rec(i + 1, left - e, cur); cur.pop()
    rec(0, d, [])
    return out

def pmul_mono(poly, m):
    return {tuple(a + b for a, b in zip(k, m)): c for k, c in poly.items()}

def elim_degree(eqs, nv, dmax, tlimit):
    """Smallest D s.t. Macaulay span (deg<=D) holds a nonzero poly in last variable only. None if not reached."""
    last = nv - 1
    def key(m):  # pure last-variable monomials smallest; else graded
        pure = all(e == 0 for e in m[:last])
        return (0 if pure else 1, sum(m), m)
    for D in range(1, dmax + 1):
        if time.time() - T0 > tlimit: return None
        piv = {}
        for f in eqs:
            df = max(sum(k) for k in f)
            if df > D: continue
            for m in monos(nv, D - df):
                row = pmul_mono(f, m)
                while row:
                    lead = max(row, key=key)
                    if lead in piv:
                        r = piv[lead]; c = row[lead] * pow(r[lead], P - 2, P) % P
                        for k, v in r.items():
                            nvv = (row.get(k, 0) - c * v) % P
                            if nvv: row[k] = nvv
                            else: row.pop(k, None)
                    else:
                        piv[lead] = row; break
                if time.time() - T0 > tlimit: return None
        for lead, r in piv.items():
            if all(all(e == 0 for e in k[:last]) for k in r) and any(k[last] > 0 for k in r) or \
               (len(r) == 1 and all(e == 0 for e in next(iter(r))[:last]) and False):
                return D
    return None

def build(t, rng, null):
    nv = 2 * t
    def var(i, e=1):
        k = [0] * nv; k[i] = e; return tuple(k)
    one = tuple([0] * nv)
    eqs = []
    for base in (0, t):  # tower u_k^2 = a u_{k-1} + b, u_0 = const
        for k in range(t):
            a, b = rng.randrange(1, P), rng.randrange(P)
            f = {var(base + k, 2): 1}
            if k == 0: f[one] = (-b - a * rng.randrange(P)) % P
            else:
                f[var(base + k - 1)] = (-a) % P; f[one] = (-b) % P
            eqs.append({m: c % P for m, c in f.items() if c % P})
    u, v = t - 1, 2 * t - 1
    if null:  # random polynomial of S_3 shape (deg<=2 in each of u_t, v_t)
        f = {}
        for i in range(3):
            for j in range(3):
                c = rng.randrange(P)
                if c:
                    k = [0] * nv; k[u] += i; k[v] += j; f[tuple(k)] = c
        f[one] = (f.get(one, 0) + 1) % P
    else:  # Semaev S_3(x1,x2,R) on y^2=x^3+Ax+B
        A, B, R = rng.randrange(P), rng.randrange(1, P), rng.randrange(P)
        def mono(i, j, c):
            k = [0] * nv; k[u] += i; k[v] += j; return tuple(k), c % P
        terms = [mono(2, 0, R * R), mono(0, 2, R * R), mono(1, 1, -2 * R * R),
                 mono(2, 1, -2 * R), mono(1, 2, -2 * R), mono(1, 0, -2 * R * 0 - 2 * R * A - 4 * B),
                 mono(0, 1, -2 * R * A - 4 * B), mono(0, 0, -4 * R * B), mono(2, 2, 1),
                 mono(1, 1, -2 * A), mono(0, 0, A * A)]
        # remaining S_3 terms: (x1x2-A)^2 -4B(x1+x2) -2R(x1+x2)(x1x2+A) -4BR
        f = {}
        for k, c in terms:
            f[k] = (f.get(k, 0) + c) % P
        f = {k: c for k, c in f.items() if c}
    eqs.append(f)
    return eqs, nv

def positive_control():
    # x_last = 5 style linear system; elimination degree must be 1
    nv = 2
    return elim_degree([{(1, 0): 1, (0, 0): P - 3}, {(0, 1): 1, (1, 0): 1, (0, 0): P - 8}], nv, 3, 5)

def study(ts, reps, seed, dmax, tlimit):
    res = {}
    for t in ts:
        for arm in ("s3", "null"):
            ds = []
            for r in range(reps):
                rng = random.Random(f"{seed}-{t}-{arm}-{r}")
                eqs, nv = build(t, rng, arm == "null")
                ds.append(elim_degree(eqs, nv, dmax[t], tlimit))
            res[f"t{t}_{arm}"] = ds
    return res

def med(ds):
    v = [d if d is not None else 99 for d in ds]
    return statistics.median(v)

def main():
    smoke = "--smoke" in sys.argv
    if smoke: ts, reps, dmax, tl = [1, 2], 2, {1: 5, 2: 5}, 8
    else: ts, reps, dmax, tl = [1, 2, 3], 8, {1: 8, 2: 9, 3: 8}, 540
    seed = 20261110
    res = study(ts, reps, seed, dmax, tl)
    aa = study([1], reps, seed + 1, dmax, tl) if not smoke else {}
    pc = positive_control()
    meds = {k: med(v) for k, v in res.items()}
    aa_gap = abs(med(aa["t1_s3"]) - meds["t1_s3"]) if aa else None
    verdict = "inconclusive"
    ok_pc = pc == 1
    if ok_pc and not smoke:
        m = [meds.get(f"t{t}_s3") for t in ts]
        resolved = all(v < 99 for v in m)
        if resolved and all(m[i + 1] > m[i] for i in range(len(m) - 1)): verdict = "consistent_with_prediction"
        elif resolved and m[-1] <= m[-2]: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-ICP-pkm-tower-algebraic-oracle", "smoke": smoke, "p": P, "seed": seed,
        "reps": reps, "depths": ts, "dmax": dmax, "elim_degrees": res, "medians": meds,
        "positive_control_degree": pc, "positive_control_ok": ok_pc, "aa_median_gap_t1": aa_gap,
        "elapsed_s": round(time.time() - T0, 2), "verdict": verdict}))
main()
