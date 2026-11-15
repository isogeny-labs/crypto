#!/usr/bin/env python3
import sys, json, time, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ns_degree import *

RAW = os.environ.get("N1_RAW") or os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw")
T0 = time.time()
def log(s): print(f"[{time.time()-T0:7.1f}s] {s}", flush=True)
def dump(name, obj):
    with open(os.path.join(RAW, name + ".json"), "w") as fh: json.dump(obj, fh, indent=1)

def span_at(C, F, E, c, degF, ks, tot, nu, p):
    rows = ns_columns(C, F, degF, c, E, ks, tot, nu)
    b = np.zeros((E + nu) * C.N, dtype=np.int64); b[:C.N] = 1
    ok, rank = in_span(rows, b, p) if len(rows) else (False, 0)
    return ok, dict(c=c, ncols=int(rows.shape[0]), nrows=int(rows.shape[1]), rank=int(rank), in_span=bool(ok))

def d_ns_desc(C, F, E, start):
    nu = F[0].shape[0]; p = C.p
    degF = [max(C.popdeg_of(F[i][v]) + v for v in range(nu) if F[i][v].any()) for i in range(len(F))]
    ks, tot = C.monos(); info = []
    c = start
    ok, r = span_at(C, F, E, c, degF, ks, tot, nu, p); info.append(r); log(f"   E={E} {r}")
    if not ok: return None, info, degF           # not in span at start: caller must raise start
    while True:
        ok, r = span_at(C, F, E, c - 1, degF, ks, tot, nu, p); info.append(r); log(f"   E={E} {r}")
        if not ok: return c, info, degF
        c -= 1

def control_m2():
    out = []
    for p in (40961, 12289):
        for t in (2, 3, 4, 5):
            for idx in range(3):
                t0 = time.time()
                C, meta, f = make_instance(p, t, idx, m=2)
                inv = np.array([pow(int(v), p - 2, p) for v in f], dtype=np.int64)
                pd = C.popdeg_of(inv); pred = 2 + pd
                d, info, degF = d_ns(C, [f.reshape(1, -1)], 0, cmin=2, cmax=2 * t + 4)
                row = dict(meta, m=2, d_NS=d, popdeg_finv=pd, pred_2_plus_popdeg=pred, two_t_plus_2=2 * t + 2,
                           degF=degF, match_pred=(d == pred), match_2t2=(d == 2 * t + 2), steps=info, runtime_s=time.time() - t0)
                dump(f"ctrlm2_p{p}_t{t}_i{idx}", row); out.append(row)
                log(f"C1 p={p} t={t} i={idx}: d_NS={d} 2+popdeg={pred} 2t+2={2*t+2}")
    return out

def control_explicit():
    """C3: t=2, explicit group algebra product vs pointwise column"""
    p, t = 12289, 2
    C, meta, F = make_instance(p, t, 0)
    K = C.K; a, b, g, xR = meta["a"], meta["b"], meta["g"], meta["x_R"]
    # polynomials: dict {(k1,k2,k3): {upow: coeff}}; z_i^K = 1
    def mul(P, Q):
        R = {}
        for k, cu in P.items():
            for l, dv in Q.items():
                kk = tuple((x + y) % K for x, y in zip(k, l))
                tgt = R.setdefault(kk, {})
                for e1, c1 in cu.items():
                    for e2, c2 in dv.items():
                        tgt[e1 + e2] = (tgt.get(e1 + e2, 0) + c1 * c2) % p
        return R
    def add(P, Q, s=1):
        R = {k: dict(v) for k, v in P.items()}
        for k, v in Q.items():
            tgt = R.setdefault(k, {})
            for e, c in v.items(): tgt[e] = (tgt.get(e, 0) + s * c) % p
        return R
    def const(c, k=(0, 0, 0), e=0): return {k: {e: c % p}}
    def zi(i): k = [0, 0, 0]; k[i] = 1; return {tuple(k): {0: g % p}}        # x_i = g z_i
    U = const(1, e=1)
    def S3sym(X1, X2, X3):
        s = add(X1, X2); pr = mul(X1, X2)
        x32 = mul(X3, X3)
        d = add(X1, X2, -1)
        t1 = mul(mul(d, d), x32)
        inner = add(mul(s, add(pr, const(a))), const(2 * b))
        t2 = mul(const(-2), mul(inner, X3))
        q = add(pr, const(-a)); t3 = add(mul(q, q), mul(const(-4 * b), s))
        return add(add(t1, t2), t3)
    E1 = S3sym(zi(0), zi(1), U)
    E2 = S3sym(U, zi(2), const(xR))
    mu = (1, 2, 3); k = [mu[0], mu[1], mu[2]]
    results = []
    ks, tot = C.monos()
    for i, Ei in enumerate((E1, E2)):
        prod = mul({tuple(k): {0: 1}}, Ei)
        # evaluate at grid
        N = C.N; nu = 3
        vec = np.zeros((nu + 0, N), dtype=np.int64)
        for kk, cu in prod.items():
            val = np.ones(N, dtype=np.int64)
            for j in range(3): val = (val * C.pw[(C.A[j] * kk[j]) % K]) % p
            for e, c in cu.items(): vec[e] = (vec[e] + c * val) % p
        sel = np.nonzero((ks[0] == k[0]) & (ks[1] == k[1]) & (ks[2] == k[2]))[0]
        mv = C.mono_vals(ks, sel)[0]
        pw_vec = np.array([(mv * F[i][v]) % p for v in range(nu)])
        results.append(bool((vec == pw_vec).all()))
    # also compare symbolic E1 expansion degrees
    degs = [max(pc(k[0]) + pc(k[1]) + pc(k[2]) + e for k, cu in Ei.items() for e, c in cu.items() if c) for Ei in (E1, E2)]
    row = dict(p=p, t=t, monomial=mu, column_matches=results, symbolic_degF=degs)
    dump("ctrl_explicit_t2", row); log(f"C3 {row}")
    return row

def control_planted():
    out = []
    for p in (40961, 12289):
        for t in (2, 3, 4, 5):
            t0 = time.time()
            C, meta, F = planted_instance(p, t)
            dets = gate_det(p, F[0], F[1]); nsing = int((dets == 0).sum())
            row = dict(meta, gate_singular_points=nsing, gate_flags_inconsistent_rejected=bool(nsing > 0))
            if t <= 3:
                E = 2; c = 3 * t + 5
                nu = 3
                degF = [max(C.popdeg_of(F[i][v]) + v for v in range(nu) if F[i][v].any()) for i in range(2)]
                ks, tot = C.monos()
                ok, r = span_at(C, F, E, c, degF, ks, tot, nu, p)
                row.update(E=E, c=c, one_in_span=ok, step=r)
            else:
                row.update(unique_cert_E1=None if unique_cert_E1(C, F) is None else "found (unexpected)")
            row["runtime_s"] = time.time() - t0
            dump(f"ctrlplant_p{p}_t{t}", row); out.append(row); log(f"C2 {row}")
    return out

def main_m3(cfg):
    out = []
    for p in (40961, 12289):
        for t in cfg["ts"]:
            for idx in range(3):
                t0 = time.time()
                C, meta, F = make_instance(p, t, idx)
                res = unique_cert_E1(C, F)
                assert res is not None
                d1, terms, degF = res
                row = dict(meta, m=3, bound_3t_plus_5=3 * t + 5, degF=degF, d_NS_E1_unique_pointwise=d1,
                           unique_terms=[dict(i=i, e=e, popdeg_coef=pd, degree=dg) for i, e, pd, dg in terms],
                           grid_points=C.N, d_NS={})
                log(f"p={p} t={t} i={idx} seed={meta['seed']}: E=1 unique-cert degree {d1} (3t+5={3*t+5})")
                for E in cfg["E"].get(t, []):
                    t1 = time.time()
                    start = d1 if E >= 1 else None
                    d, info, _ = d_ns_desc(C, F, E, start)
                    row["d_NS"][str(E)] = dict(d=d, steps=info, runtime_s=time.time() - t1)
                    log(f"   => d_NS(E={E}) = {d}")
                row["runtime_s"] = time.time() - t0
                dump(f"m3_p{p}_t{t}_i{idx}", row); out.append(row)
    return out

if __name__ == "__main__":
    which = sys.argv[1]
    if which == "controls":
        control_explicit(); control_m2(); control_planted()
    elif which == "m3_small":
        main_m3(dict(ts=[2], E={2: [1, 8]}))
    elif which == "m3_t3":
        main_m3(dict(ts=[3], E={3: [1, 2]}))
    elif which == "m3_big":
        main_m3(dict(ts=[4, 5], E={}))
