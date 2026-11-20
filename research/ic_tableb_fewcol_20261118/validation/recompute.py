#!/usr/bin/env python3
"""Independent validator recompute for the Table B round-2/3 studies. Stdlib only.
Reads only sessions/<P>/{records.jsonl,spec.json}. Does not use plot.py, summary.json or compare-*.txt
(those are read only afterwards, by diff.py, to list discrepancies).
Usage: recompute.py SESSIONS_DIR OUT_JSON
Pricing (AGENTS.md 5a): priced_gae(w) = G + w*U, U = uncharged-counter ops + table work
(table_insert_attempts, sibling_second_points, sibling_lift_products when marked 'counted:').
S = priced_gae / sqrt(r).  Pair ratio = sum over workloads of mean-S(B) / sum of mean-S(A), 95% interval by
linearised t (delta method) over workloads, interval on the log scale R*exp(+-t*se/R) as ecbench documents (stats.rs ci95_log).  Every recovered log is re-checked with own prime-field EC arithmetic."""
import json, math, os, sys, hashlib
from collections import defaultdict
W = [0.0, 0.1, 1.0]
TABLE_WORK = ["table_insert_attempts", "sibling_second_points", "sibling_lift_products"]
NON_OP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"}

def t975(df):
    c = math.exp(math.lgamma((df+1)/2) - math.lgamma(df/2)) / math.sqrt(df*math.pi)
    pdf = lambda x: c*(1+x*x/df)**(-(df+1)/2)
    def cdf(x, m=4000):
        h = x/m; s = pdf(0)+pdf(x)
        for i in range(1, m): s += (4 if i % 2 else 2)*pdf(i*h)
        return 0.5 + s*h/3
    lo, hi = 0.0, 50.0
    for _ in range(80):
        mid = (lo+hi)/2
        if cdf(mid) < 0.975: lo = mid
        else: hi = mid
    return (lo+hi)/2

def unpriced(r):
    out = {}
    counters = r.get("counters") or {}
    for name in r["cost"].get("unpriced") or []:
        if name in NON_OP: continue
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters: out[name] = counters[name]; continue
        tot, found = 0, False
        for p in r["phases"]:
            nat = p.get("native") or {}
            v = nat.get(name, nat.get(base))
            if v is not None: tot += v; found = True
        if not found: raise SystemExit("cannot resolve unpriced counter %s in %s" % (name, r["record_id"]))
        out[name] = tot
    return out

PRICED_NATIVE = [("sqrt_solves","ns_per_sqrt"),("as_solves","ns_per_as_solve"),("s4_pairs","ns_per_s4_pair"),
 ("lookups","ns_per_lookup"),("row_ops","ns_per_row_op"),("legendre_symbols","ns_per_legendre"),
 ("inversions","ns_per_inversion"),("frobenius_maps","ns_per_frobenius"),("canonicalisations","ns_per_canon"),
 ("target_guard_probes","ns_per_lookup"),("hashed_tensor_steps","ns_per_hashed_tensor_step")]
ROW = {1048291: "prime/bench-20bit", 3290411: "prime/generated-22bit-3290411", 10935329: "prime/generated-24bit-10935329"}
def calibrated(r, ratios):
    pinned = (r.get("detail") or {}).get("calibration_pinned_units") or []
    if not pinned: return 0.0, 0
    if r.get("solver") and r["solver"].get("ops", 0) > 0: raise SystemExit("solver ops present")
    gae, ops = 0.0, 0
    for p in r["phases"]:
        nat = p.get("native") or {}
        for c, u in PRICED_NATIVE:
            n = nat.get(c, 0)
            if n > 0 and u in pinned: gae += n*ratios[u]; ops += n
    return gae, ops

def table_work(r):
    t = 0
    for name in TABLE_WORK:
        if any(("counted:"+name) in (p.get("work_native") or {}) for p in r["phases"]):
            t += sum((p.get("work_native") or {}).get(name, 0) for p in r["phases"])
    return t

# ---- prime-field affine arithmetic (own)
def inv(a, p): return pow(a, p-2, p)
def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1]+Q[1]) % p == 0: return None
        l = (3*P[0]*P[0]+a)*inv(2*P[1], p) % p
    else:
        l = (Q[1]-P[1])*inv(Q[0]-P[0], p) % p
    x = (l*l-P[0]-Q[0]) % p
    return (x, (l*(P[0]-x)-P[1]) % p)
def mul(k, P, p, a):
    R = None
    while k:
        if k & 1: R = add(R, P, p, a)
        P = add(P, P, p, a); k >>= 1
    return R
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(q, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def main():
    sd, outp, calp = sys.argv[1:4]
    cal = json.load(open(calp))["instances"]
    out = {"sessions": {}}
    for ses in sorted(os.listdir(sd)):
        d = os.path.join(sd, ses)
        if not os.path.isdir(d): continue
        rp = os.path.join(d, "records.jsonl")
        raw = open(rp, "rb").read()
        recs = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
        cs = recs[0]["workload"]["curve_spec"]
        r_ord = cs["r"]; p = cs["p"]; a = cs["a"]
        assert is_prime(p) and is_prime(r_ord) and cs["kind"] == "prime_explicit"
        G = (cs["gx"], cs["gy"]); b = cs["b"]
        assert (G[1]**2 - G[0]**3 - a*G[0] - b) % p == 0 and mul(r_ord, G, p, a) is None, "generator order"
        sq = math.sqrt(r_ord); ratios = cal[ROW[p]]
        # scalar check on every record (warm-ups included)
        bad = 0; seen = {}
        for r in recs:
            o = r["outcome"]
            if o["status"] != "verified": bad += 1; continue
            t = tuple(int(x, 16) for x in r["workload"]["target"])
            assert (t[1]**2 - t[0]**3 - a*t[0] - b) % p == 0
            key = (t, o["recovered"])
            if key not in seen:
                seen[key] = (mul(int(o["recovered"]), G, p, a) == t)
            if not seen[key]: bad += 1
        # pricing
        meas = [r for r in recs if not r["warmup"]]
        arms = sorted({r["arm"] for r in meas})
        # S_w per (arm, workload, round-run)
        data = defaultdict(lambda: defaultdict(lambda: [[] for _ in range(5)]))
        for r in meas:
            if r["outcome"]["status"] != "verified": continue
            g = r["cost"]["total_gae"]; u = sum(unpriced(r).values()) + table_work(r)
            C, nC = calibrated(r, ratios)
            for i, w in enumerate(W):
                data[r["arm"]][r["workload"]["workload_id"]][i].append((g - C + w*(u + nC))/sq)
            data[r["arm"]][r["workload"]["workload_id"]][3].append(g/sq)
            bo = (r.get("work") or {}).get("bitops_per_sqrt_r")
            if bo is not None: data[r["arm"]][r["workload"]["workload_id"]][4].append(float(bo))
        S = {}; counts = {}
        for arm in arms:
            counts[arm] = sum(len(v[0]) for v in data[arm].values())
            S[arm] = [sum(sum(v[i])/len(v[i]) for v in data[arm].values())/len(data[arm]) if all(v[i] for v in data[arm].values()) else None for i in range(5)]
        # note: band mean = mean over workloads of per-workload mean (balanced design => equals mean over runs)
        pairs = {}
        for A in arms:
            for B in arms:
                if A == B: continue
                wl = sorted(set(data[A]) & set(data[B])); n = len(wl)
                res = []
                for i in range(5):
                    if not all(data[A][k][i] and data[B][k][i] for k in wl):
                        res.append(None); continue
                    x = [sum(data[A][k][i])/len(data[A][k][i]) for k in wl]
                    y = [sum(data[B][k][i])/len(data[B][k][i]) for k in wl]
                    mx, my = sum(x)/n, sum(y)/n; R = my/mx
                    e = [yy - R*xx for xx, yy in zip(x, y)]
                    se = math.sqrt(sum(v*v for v in e)/(n-1)/n)/mx
                    tq = t975(n-1)
                    h = tq*se/R
                    res.append({"ratio": R, "lo": R*math.exp(-h), "hi": R*math.exp(h)})
                pairs[A+"|"+B] = {"n_workloads": n, "w": res}
        out["sessions"][ses] = {"records_sha256": hashlib.sha256(raw).hexdigest(), "records": len(recs), "measured": len(meas),
            "log2_r": math.log2(r_ord), "r": r_ord, "unverified_or_bad_scalar": bad, "scalar_checks": len(seen),
            "arms": arms, "runs_per_arm": counts, "S": S, "pairs": pairs}
    json.dump(out, open(outp, "w"), indent=1, sort_keys=True)
main()
