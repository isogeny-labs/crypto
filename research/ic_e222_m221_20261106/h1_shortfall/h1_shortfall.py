#!/usr/bin/env python3
"""Why does H1 (--project-cofactor) recover less than 1/h of the trials?  (unvalidated, pre-gate)

Model, read from src/cryptanalysis/ec_index_calculus.rs (build_factor_base,
build_factor_base_projected, build_pair_sum_table, find_one_relation_table_with_rng):

* A trial draws R = aG + bQ, uniform on <G> (prime order n).  It succeeds iff R is a point of the
  pair-sum table, a set of the DISTINCT affine points +-F_i +-F_j (i <= j; i == j gives +-2F_i).
* Only table points that lie in <G> can match R.  With H1 on, every entry is [h]F in <G>, so the
  whole table is usable.  So per-trial success probability is P = |T cap <G>| / n and
  E[trials per relation] = 1 / P, the same number of relations (fb + 2) in both arms.
* Hence  E[on/off trials] = P_off / P_on = |T_off cap <G>| / |T_on|   ("exact" model, per instance).
* Uniform-class model U: classes t_i in T = E/<G> iid uniform, m = 32 entries, all table points
  distinct:  fraction usable f_U = (4*C(m,2)/h + 2*m*|T[2]|/h) / (4*C(m,2) + 2*m).
  The 4*C(m,2) off-diagonal points are usable w.p. 1/h each (t_i = +-t_j), the 2*m diagonal points
  +-2F_i are usable w.p. |T[2]|/h (2 t_i = 0).
Assumptions: R uniform on <G> (it is, up to O), independent trials, no rank-deficiency retry (the
rows show fb + 2 relations in both arms), geometric trial counts per relation.
"""
import json, sys, os, math, random, collections, statistics

HERE = os.path.dirname(os.path.abspath(__file__))
INST = os.path.join(HERE, "instances")
ROWS = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "..", "results", "h1_defaults", "rows.jsonl")
M = 32                       # factor-base size (the default)
RAND_STARTS = 20             # random-start factor bases per instance (H-c control)
MC_REPS = 400

# ---------------------------------------------------------------- arithmetic (Weierstrass, affine)
def sqrt_mod(a, p):
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r

def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def neg(P, p):
    return None if P is None else (P[0], (-P[1]) % p)

def mul(k, P, a, p):
    R = None
    if k < 0:
        k, P = -k, neg(P, p)
    while k:
        if k & 1:
            R = add(R, P, a, p)
        P = add(P, P, a, p)
        k >>= 1
    return R

# ---------------------------------------------------------------- factor bases (as in the repo)
def scan_points(a, b, p, start, m):
    """Points with square rhs scanning x = start, start+1, ... (wrapping at p); y as the sqrt returns it."""
    out, x, tried = [], start, 0
    while len(out) < m and tried < p:
        rhs = (x * x * x + a * x + b) % p
        y = sqrt_mod(rhs, p)
        if y is not None:
            out.append((x, y))
        x = x + 1 if x + 1 < p else 1
        tried += 1
    return out

def canon(P, p):
    if P is None:
        return None
    x, y = P
    return (x, y if 2 * y < p else p - y)

def fb_off(a, b, p, start, m):
    """build_factor_base: first m small-x points, y canonicalised to the smaller root."""
    return [canon(P, p) for P in scan_points(a, b, p, start, m)]

def fb_on(a, b, p, h, start, m):
    """build_factor_base_projected: entries [h]F, skipping O and repeated x; returns (entries, #skipped_inf, #skipped_dup, #scanned)."""
    out, seen, x, tried, s_inf, s_dup, scanned = [], set(), start, 0, 0, 0, 0
    while len(out) < m and tried < p:
        rhs = (x * x * x + a * x + b) % p
        y = sqrt_mod(rhs, p)
        if y is not None:
            scanned += 1
            Pp = mul(h, (x, y), a, p)
            if Pp is None:
                s_inf += 1
            elif Pp[0] in seen:
                s_dup += 1
            else:
                seen.add(Pp[0])
                out.append(canon(Pp, p))
        x = x + 1 if x + 1 < p else 1
        tried += 1
    return out, s_inf, s_dup, scanned

def pair_table(fb, a, p):
    """build_pair_sum_table: dict point -> (i, si, j, sj), first insertion wins, O is not stored."""
    tab = {}
    def put(pt, key):
        if pt is not None and pt not in tab:
            tab[pt] = key
    for i, fi in enumerate(fb):
        for j in range(i, len(fb)):
            fj = fb[j]
            s = add(fi, fj, a, p)
            put(s, (i, 1, j, 1))
            put(neg(s, p), (i, -1, j, -1))
            if i != j:
                d = add(fi, neg(fj, p), a, p)
                put(d, (i, 1, j, -1))
                put(neg(d, p), (i, -1, j, 1))
    return tab

# ---------------------------------------------------------------- torsion classes
def torsion_class(P, e, N, a, p):
    """t = (1-e)P with e = 1 mod n, 0 mod h: the component of P in the order-h subgroup T (None = O)."""
    return mul((1 - e) % N, P, a, p)

def tsum(t1, s1, t2, s2, a, p):
    """s1*t1 + s2*t2 on the torsion subgroup (None = O)."""
    return add(t1 if s1 == 1 else neg(t1, p), t2 if s2 == 1 else neg(t2, p), a, p)

def class_of_entry(key, tcl, a, p):
    i, si, j, sj = key
    return tsum(tcl[i], si, tcl[j], sj, a, p)

def torsion_group(a, b, p, e, N, h, rng):
    """T = image of random points under P -> (1-e)P, closed under addition."""
    S = {None}
    for _ in range(80):
        while True:
            x = rng.randrange(p)
            y = sqrt_mod((x ** 3 + a * x + b) % p, p)
            if y is not None:
                break
        S.add(torsion_class((x, y), e, N, a, p))
    changed = True
    while changed:
        changed = False
        for u in list(S):
            for v in list(S):
                w = add(u, v, a, p)
                if w not in S:
                    S.add(w)
                    changed = True
    t2 = sum(1 for u in S if add(u, u, a, p) is None)
    orders = collections.Counter()
    for u in S:
        k, Pk = 1, u
        while Pk is not None:
            Pk = add(Pk, u, a, p)
            k += 1
        orders[k] += 1
    return len(S), t2, dict(orders)

def geometric(P, rng):
    u = rng.random()
    if P >= 1:
        return 1
    return int(math.log(1.0 - u) / math.log(1.0 - P)) + 1

# ---------------------------------------------------------------- main
def main():
    rng = random.Random("h1-shortfall-v1")
    obs = collections.defaultdict(dict)       # name -> seed -> {arm: trials}
    for line in open(ROWS):
        r = json.loads(line)
        obs[r["name"]].setdefault(r["seed"], {})[r["arm"]] = r["trials"]
    results = []
    for fn in sorted(os.listdir(INST)):
        if not fn.endswith(".json") or fn == "MANIFEST.json":
            continue
        d = json.load(open(os.path.join(INST, fn)))
        p = int(d["field"]["modulus"]); a = int(d["a"]); b = int(d["b"]); n = int(d["subgroup_order"]); h = int(d["cofactor"])
        N = int(d["group_order"]); assert N == n * h
        name = d["name"]; fam = d["family"]; rung = int(name[1:name.index("-")])
        e = h * pow(h, -1, n) if h > 1 else 1            # e = 1 mod n, 0 mod h (h = 1: no torsion)
        rec = dict(name=name, family=fam, rung=rung, r_bits=d["r_bits"], p=p, n=n, h=h)
        # torsion group structure
        if h > 1:
            Tsize, T2, ords = torsion_group(a, b, p, e, N, h, rng)
        else:
            Tsize, T2, ords = 1, 1, {1: 1}
        rec.update(T_size=Tsize, T_2torsion=T2, T_orders=ords)
        # ---- actual factor bases (scan from x = 1, as the repo does)
        F = fb_off(a, b, p, 1, M)
        Fp, s_inf, s_dup, scanned = fb_on(a, b, p, h, 1, M) if h > 1 else (F, 0, 0, M)
        tcl = [torsion_class(P, e, N, a, p) if h > 1 else None for P in F]
        Toff = pair_table(F, a, p)
        Ton = pair_table(Fp, a, p)
        # usable off-entries: class method and explicit [n]P test must agree
        inG_cls = {pt for pt, k in Toff.items() if class_of_entry(k, tcl, a, p) is None} if h > 1 else set(Toff)
        inG_exp = {pt for pt in Toff if mul(n, pt, a, p) is None}
        on_all_in_G = all(mul(n, pt, a, p) is None for pt in Ton)
        nominal = 4 * (M * (M - 1) // 2) + 2 * M
        cls_counts = collections.Counter(("O" if t is None else "t") for t in tcl)
        order_of = {}
        def ordr(t):
            if t is None:
                return 1
            k, Pk = 1, t
            while Pk is not None:
                Pk = add(Pk, t, a, p)
                k += 1
            return k
        cls_orders = collections.Counter(ordr(t) for t in tcl) if h > 1 else {1: M}
        distinct_classes = len({t for t in tcl})
        rec.update(
            fb_off_size=len(F), fb_on_size=len(Fp), on_skipped_inf=s_inf, on_skipped_dup=s_dup, on_scanned=scanned,
            table_off_distinct=len(Toff), table_on_distinct=len(Ton), table_nominal=nominal,
            off_in_G_class=len(inG_cls), off_in_G_explicit=len(inG_exp), class_matches_explicit=(inG_cls == inG_exp),
            on_all_in_G=on_all_in_G,
            class_orders={str(k): v for k, v in sorted(cls_orders.items())}, n_distinct_classes=distinct_classes,
            frac_points_in_G_off=len(inG_exp) / max(1, len(Toff)),
            pred_exact=len(inG_exp) / len(Ton),
        )
        mm = M
        rec["pred_uniform_U"] = ((4 * (mm * (mm - 1) // 2)) / h + 2 * mm * T2 / h) / nominal
        rec["pred_uniform_no_diag"] = 1.0 / h
        # ---- H-c: random-start factor bases
        preds = []
        for _ in range(RAND_STARTS):
            st = rng.randrange(1, p)
            Fr = fb_off(a, b, p, st, M)
            Frp = fb_on(a, b, p, h, st, M)[0] if h > 1 else Fr
            tr = [torsion_class(P, e, N, a, p) if h > 1 else None for P in Fr]
            Tr = pair_table(Fr, a, p)
            Trp = pair_table(Frp, a, p)
            ing = sum(1 for k in Tr.values() if (class_of_entry(k, tr, a, p) is None if h > 1 else True))
            preds.append(ing / len(Trp))
        rec["pred_exact_random_start_mean"] = statistics.mean(preds)
        # ---- observed
        o = obs[name]
        ron = sum(o[s]["on"] for s in o); roff = sum(o[s]["off"] for s in o)
        rec["obs_pooled_ratio"] = ron / roff
        rec["obs_pair_ratios"] = [o[s]["on"] / o[s]["off"] for s in sorted(o)]
        # ---- Monte Carlo of the paired-ratio statistic for this instance
        P_off = len(inG_exp) / n; P_on = len(Ton) / n
        rec["P_off"] = P_off; rec["P_on"] = P_on
        results.append(rec)
        print(f"{name:8} h={h} T={Tsize} T2={T2} off-in-G {len(inG_exp):4}/{len(Toff):4} on {len(Ton):4} pred_exact {rec['pred_exact']:.3f} "
              f"U {rec['pred_uniform_U']:.3f} rand {rec['pred_exact_random_start_mean']:.3f} obs {rec['obs_pooled_ratio']:.3f} "
              f"cls_match={rec['class_matches_explicit']} on_in_G={on_all_in_G}", flush=True)

    # ---- Monte Carlo family medians (same statistic as RESULTS_h1_defaults.md: median over instances x seeds of on/off)
    summary = {}
    for fam in ("E", "M", "W"):
        rs = [r for r in results if r["family"] == fam]
        obs_pairs = [x for r in rs for x in r["obs_pair_ratios"]]
        meds = []
        mrng = random.Random("h1-shortfall-mc")
        for _ in range(MC_REPS):
            pairs = []
            for r in rs:
                for _s in range(2):
                    t_off = sum(geometric(r["P_off"], mrng) for _ in range(M + 2))
                    t_on = sum(geometric(r["P_on"], mrng) for _ in range(M + 2))
                    pairs.append(t_on / t_off)
            meds.append(statistics.median(pairs))
        meds.sort()
        summary[fam] = dict(
            n_instances=len(rs), obs_median_pair_ratio=statistics.median(obs_pairs),
            pred_exact_median=statistics.median([r["pred_exact"] for r in rs]),
            pred_exact_mean=statistics.mean([r["pred_exact"] for r in rs]),
            pred_uniform_U_median=statistics.median([r["pred_uniform_U"] for r in rs]),
            pred_1_over_h=1.0 / rs[0]["h"],
            pred_exact_random_start_median=statistics.median([r["pred_exact_random_start_mean"] for r in rs]),
            mc_median_of_pair_ratio_p50=meds[len(meds) // 2], mc_p025=meds[int(0.025 * len(meds))], mc_p975=meds[int(0.975 * len(meds)) - 1],
            T_structures={f"{k[0]}/{k[1]}": v for k, v in collections.Counter((r["T_size"], r["T_2torsion"]) for r in rs).most_common()},
            frac_dup_skipped_mean=statistics.mean([r["on_skipped_dup"] for r in rs]),
            frac_inf_skipped_mean=statistics.mean([r["on_skipped_inf"] for r in rs]),
            table_on_distinct_over_nominal_mean=statistics.mean([r["table_on_distinct"] / r["table_nominal"] for r in rs]),
            table_off_distinct_over_nominal_mean=statistics.mean([r["table_off_distinct"] / r["table_nominal"] for r in rs]),
        )
        by = collections.defaultdict(list)
        for r in rs:
            by[r["rung"]].append(r)
        summary[fam]["by_rung"] = {str(k): dict(
            n=len(v), obs_median=statistics.median([x for r in v for x in r["obs_pair_ratios"]]),
            pred_exact_median=statistics.median([r["pred_exact"] for r in v]),
            pred_U=statistics.median([r["pred_uniform_U"] for r in v]),
            pred_random_start=statistics.median([r["pred_exact_random_start_mean"] for r in v]),
            table_off_distinct_mean=statistics.mean([r["table_off_distinct"] for r in v]),
            table_on_distinct_mean=statistics.mean([r["table_on_distinct"] for r in v]),
            dup_skipped_mean=statistics.mean([r["on_skipped_dup"] for r in v]),
        ) for k, v in sorted(by.items())}
        # class-order distribution over the 32 points, pooled over the family
        pooled = collections.Counter()
        for r in rs:
            for k, v in r["class_orders"].items():
                pooled[int(k)] += v
        tot = sum(pooled.values())
        summary[fam]["class_order_distribution_32_points"] = {str(k): round(v / tot, 4) for k, v in sorted(pooled.items())}
        # expected distribution under uniform classes of the same T (average over instances)
        exp = collections.Counter()
        for r in rs:
            for k, v in r["T_orders"].items():
                exp[int(k)] += v / r["T_size"]
        te = sum(exp.values())
        summary[fam]["class_order_distribution_uniform"] = {str(k): round(v / te, 4) for k, v in sorted(exp.items())}
    # observed pooled ratio per family (sum on / sum off over all rows)
    for fam in ("E", "M", "W"):
        ron = roff = 0
        for r in results:
            if r["family"] == fam:
                o = obs[r["name"]]
                ron += sum(o[s]["on"] for s in o); roff += sum(o[s]["off"] for s in o)
        summary[fam]["obs_pooled_ratio"] = ron / roff
    out = dict(config=dict(M=M, rand_starts=RAND_STARTS, mc_reps=MC_REPS, rows=os.path.relpath(ROWS, HERE)), summary=summary, instances=results)
    json.dump(out, open(os.path.join(HERE, "results_h1_shortfall.json"), "w"), indent=1, default=str)
    print(json.dumps({k: {kk: vv for kk, vv in v.items() if kk != "by_rung"} for k, v in summary.items()}, indent=1, default=str))

if __name__ == "__main__":
    main()
