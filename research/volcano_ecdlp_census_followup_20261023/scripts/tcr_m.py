#!/usr/bin/env python3
"""Setup helper for the follow-up census (Phase A). It builds the inputs of two
new families and nothing else; it measures nothing.

  tcr : V5 family Tcr. For every non-crater vertex v, carry v's native generator
        gn_v up to the crater by verified 2-isogenies.
        - ca walks from v with its own root generator P_v and carries [k]P_v along
          its tree (`--ks`) up to the single level-1 vertex w. gn_v = [c]P_v is
          found by a baby-step giant-step discrete log; ks = [c, c*k_i mod r].
        - ca does not step onto j = 0, so the last edge (w -> crater) is a Velu
          2-isogeny done here, then moved to the census crater model by the
          isomorphism u^6 = b_crater / b_codomain. Every step is checked.
  m   : family M. Choose the curves with the seeded rule of census/seeds.json and
        write 8 isomorphic models each: (u^4 a, u^6 b), generator (u^2 x, u^3 y).

Usage: python3 -I tcr_m.py tcr|m
"""
import hashlib, json, math, os, subprocess, sys, tempfile

STUDY = os.environ.get("STUDY", "/home/user/crypto/research/volcano_ecdlp_census_followup_20261023")
CA = os.environ.get("CA", "/home/user/cryptanalysis/build/ca")
ECB = os.environ.get("ECB", "/home/user/crypto/target/release/ecbench")
SEEDS = json.load(open(f"{STUDY}/census/seeds.json"))
CA_SEED = SEEDS["ca_seed"]


def H(s):
    return int(hashlib.sha256(s.encode()).hexdigest()[:8], 16)


# ---- short Weierstrass arithmetic over F_p (affine; None = infinity) --------
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)


def neg(P, p):
    return None if P is None else (P[0], (-P[1]) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R


def on_curve(P, a, b, p):
    return P is None or (P[1] * P[1] - (P[0] ** 3 + a * P[0] + b)) % p == 0


def jinv(a, b, p):
    d = (4 * a ** 3 + 27 * b * b) % p
    return (1728 * 4 * a ** 3 * pow(d, -1, p)) % p


def pt(s):
    x, y = s.split(","); return (int(x), int(y))


def dlog_in_subgroup(P, Q, r, a, p):
    """c with Q = [c]P, P of prime order r (BSGS)."""
    m = math.isqrt(r) + 1
    baby = {}
    R = None
    for j in range(m):
        baby[R] = j; R = add(R, P, a, p)
    giant = neg(mul(m, P, a, p), p)
    G = Q
    for i in range(m + 1):
        if G in baby: return (i * m + baby[G]) % r
        G = add(G, giant, a, p)
    raise SystemExit("dlog failed")


def factor(n):
    f = {}; d = 2
    while d * d <= n:
        while n % d == 0: f[d] = f.get(d, 0) + 1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0) + 1
    return f


def primitive_root(p):
    fs = factor(p - 1)
    for g in range(2, p):
        if all(pow(g, (p - 1) // q, p) != 1 for q in fs): return g


def dlog_fp(g, t, p):
    m = math.isqrt(p) + 1
    baby = {}; x = 1
    for j in range(m):
        baby.setdefault(x, j); x = x * g % p
    gi = pow(g, -m, p); y = t
    for i in range(m + 1):
        if y in baby: return (i * m + baby[y]) % (p - 1)
        y = y * gi % p
    raise SystemExit("fp dlog failed")


def sixth_root(t, p):
    g = primitive_root(p)
    L = dlog_fp(g, t, p)
    assert L % 6 == 0, "not a sixth power"
    u = pow(g, L // 6, p)
    assert pow(u, 6, p) == t % p
    return u


def velu2_codomain(a, b, x0, p):
    t = (3 * x0 * x0 + a) % p; w = x0 * t % p
    return (a - 5 * t) % p, (b - 7 * w) % p


def velu2_map(P, a, x0, p):
    if P is None or P[0] == x0: return None
    x, y = P; t = (3 * x0 * x0 + a) % p
    i = pow(x - x0, -1, p)
    return ((x + t * i) % p, y * (1 - t * i * i) % p)


def jl(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def run_ca(args, outdir, tag):
    vf, ef, rf = f"{outdir}/{tag}.v.jsonl", f"{outdir}/{tag}.e.jsonl", f"{outdir}/{tag}.R.json"
    cmd = [CA, "volcano", *args, "--seed", str(CA_SEED), "--trials", "1", "--per-level", "0", "--controls", "0",
           "--radius", "40", "--max-vertices", "20000", "--census-out", vf, "--edges-out", ef]
    with open(rf, "w") as fo:
        subprocess.run(cmd, stdout=fo, stderr=subprocess.DEVNULL, check=True)
    return jl(vf), jl(ef), json.load(open(rf))


def ks_for_seed(V, seed):
    row = jl(f"{STUDY}/census/{V}/transport/D.jsonl")[0]
    spec = {"schema": "ecbench.spec/v1", "label": "k probe", "description": "reads the planted_shared scalars",
            "workloads": {"curves": [{"kind": "prime_explicit", "p": row["p"], "a": row["a"], "b": row["b"],
                                      "group_order": row["order"], "r": row["r"], "gx": int(row["g"].split(",")[0]),
                                      "gy": int(row["g"].split(",")[1])}],
                          "targets_per_curve": SEEDS["targets_per_curve"], "target_seed": seed,
                          "target_kind": "planted_shared"},
            "arms": [{"name": "bsgs", "role": "reference", "method": {"id": "bsgs.negation"}}],
            "measurement": {"rounds": 1, "warmup": 0, "order": "alternate", "seed": 1, "isolation_required": "L0",
                            "timeout_seconds": 60}}
    f = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False); json.dump(spec, f); f.close()
    out = subprocess.run([ECB, "plan", "--spec", f.name, "--json"], capture_output=True, text=True, check=True).stdout
    os.unlink(f.name)
    return [w["planted"] for w in json.loads(out)["workloads"]]


def tcr():
    V = "V5"
    D = jl(f"{STUDY}/census/{V}/transport/D.jsonl")
    crater = D[0]; assert crater["id"] == 0 and crater["level"] == 0 and crater["a"] == 0
    p, r, order, bc = crater["p"], crater["r"], crater["order"], crater["b"]
    ks = ks_for_seed(V, SEEDS["target_seed_native"])
    open(f"{STUDY}/census/{V}/ks_native.txt", "w").write(",".join(map(str, ks)) + "\n")
    outdir = f"{STUDY}/census/{V}/tcr"; os.makedirs(outdir, exist_ok=True)
    work = tempfile.mkdtemp(prefix="tcr_", dir=os.environ.get("SP"))
    rows = []
    for v in D:
        if v["id"] == 0: continue
        a, b = v["a"], v["b"]; gn = pt(v["gn"]); assert on_curve(gn, a, b, p) and mul(r, gn, a, p) is None
        # 1. ca root generator P_v
        V0, _, R0 = run_ca(["--p", str(p), "--a", str(a), "--b", str(b), "--ell", "2"], work, "p0")
        Pv = pt(V0[0]["g"]); assert V0[0]["id"] == 0 and mul(r, Pv, a, p) is None
        c = dlog_in_subgroup(Pv, gn, r, a, p); assert mul(c, Pv, a, p) == gn
        scal = [c] + [c * k % r for k in ks]
        # 2. carry [c]P_v and [c k_i]P_v up to w (level 1)
        V1, E1, R1 = run_ca(["--p", str(p), "--a", str(a), "--b", str(b), "--ell", "2", "--ks", ",".join(map(str, scal))], work, "p1")
        assert R1["walk"]["vertices"] == 63 and R1["walk"]["per_level"][1:] == [1, 2, 4, 8, 16, 32] and R1["walk"]["special_j_skipped"] >= 1
        ws = [x for x in V1 if x["level"] == 1]; assert len(ws) == 1
        w = ws[0]; hk = [pt(s) for s in w["hk"]]
        # 3. last edge, up to j = 0
        ups = [e for e in E1 if e["from"] == w["id"] and e["codomain_j"] == 0]; assert len(ups) == 1
        e = ups[0]; x0 = e["kernel_x"]
        a2, b2 = velu2_codomain(w["a"], w["b"], x0, p)
        assert (a2, b2) == (e["codomain_a"], e["codomain_b"]) and a2 == 0, "python Velu != ca codomain"
        # python Velu == ca on the preceding ca edge (parent(w) -> w), same points
        par = V1[w["parent"]]
        if w["parent"] >= 0:
            ph = [pt(s) for s in par["hk"]]
            assert [velu2_map(q, par["a"], w["kernel_x"], p) for q in ph] == hk, "python Velu != ca point map"
        img = [velu2_map(q, w["a"], x0, p) for q in hk]
        assert all(on_curve(q, 0, b2, p) and q is not None for q in img)
        # homomorphism of the python map on this step
        g1 = hk[0]
        for (m1, m2) in ((2, 5), (7, 11)):
            A, B = mul(m1, g1, w["a"], p), mul(m2, g1, w["a"], p)
            assert velu2_map(add(A, B, w["a"], p), w["a"], x0, p) == add(velu2_map(A, w["a"], x0, p), velu2_map(B, w["a"], x0, p), 0, p)
        # 4. onto the census crater model
        u = sixth_root(bc * pow(b2, -1, p) % p, p)
        gen = (u * u % p * img[0][0] % p, pow(u, 3, p) * img[0][1] % p)
        tg = [(u * u % p * q[0] % p, pow(u, 3, p) * q[1] % p) for q in img[1:]]
        assert on_curve(gen, 0, bc, p) and mul(r, gen, 0, p) is None and gen is not None
        # self-consistency: [k_i] psi(gn) == psi([k_i] gn)
        assert all(mul(k, gen, 0, p) == t for k, t in zip(ks, tg)), "carried targets != [k_i] psi(gn)"
        rec = {"id": v["id"], "level": v["level"], "j": v["j"], "a": a, "b": b, "gn": v["gn"], "root_P": f"{Pv[0]},{Pv[1]}", "c": c,
               "w_id_in_walk": w["id"], "w_a": w["a"], "w_b": w["b"], "up_kernel_x": x0, "codomain_b": b2, "u_to_crater": u,
               "path_len_to_crater": v["level"], "ups": v["level"],
               "g": f"{gen[0]},{gen[1]}", "hk": [f"{t[0]},{t[1]}" for t in tg], "ks": ks,
               "checks": {"ca_vs_python_velu_last_ca_edge": True, "python_velu_codomain_eq_ca_edge": True,
                          "homomorphism_on_last_step": True, "gen_on_crater_order_r": True, "targets_eq_k_times_gen": True}}
        rows.append(rec)
        json.dump(rec, open(f"{outdir}/{v['id']}.json", "w"))
    with open(f"{STUDY}/census/{V}/tcr.jsonl", "w") as f:
        for x in rows: f.write(json.dumps(x, separators=(",", ":")) + "\n")
    print("tcr", len(rows), "vertices carried to the crater and checked")


def mchoice():
    out = {}
    # V5: crater + one vertex at each of levels 4, 5, 6
    for V, plan in (("V5", "V5"), ("V6", "V6")):
        D = jl(f"{STUDY}/census/{V}/transport/D.jsonl")
        by = {}
        for x in D: by.setdefault(x["level"], []).append(x)
        for L in by: by[L].sort(key=lambda x: x["j"])
        picks = []
        if V == "V5":
            picks.append(by[0][0])
            for L in (4, 5, 6): picks.append(by[L][H(f"M:V5:L{L}") % len(by[L])])
        else:
            for L, cnt in zip(range(5), (2, 2, 2, 1, 1)):
                got = []
                for i in range(cnt):
                    s = 0
                    while True:
                        c = by[L][H(f"M:V6:L{L}:{i}:{s}") % len(by[L])]
                        if c["id"] not in [g["id"] for g in got]: break
                        s += 1
                    got.append(c)
                picks += got
        curves = []
        for x in picks:
            p, a, b, j, r = x["p"], x["a"], x["b"], x["j"], x["r"]
            gn = pt(x["gn"]); assert on_curve(gn, a, b, p) and mul(r, gn, a, p) is None
            models, seen_u2, seen_m = [], set(), {(a, b)}
            for m in range(8):
                t = 0
                while True:
                    u = 2 + H(f"M-u:{V}:{j}:{m}:{t}") % (p - 3)
                    mod = (pow(u, 4, p) * a % p, pow(u, 6, p) * b % p)
                    if u * u % p not in seen_u2 and mod not in seen_m and u * u % p != 1: break
                    t += 1
                seen_u2.add(u * u % p); seen_m.add(mod)
                g = (u * u % p * gn[0] % p, pow(u, 3, p) * gn[1] % p)
                assert on_curve(g, mod[0], mod[1], p) and jinv(mod[0], mod[1], p) == j
                models.append({"m": m, "t": t, "u": u, "a": mod[0], "b": mod[1], "gx": g[0], "gy": g[1]})
            curves.append({"V": V, "id": x["id"], "level": x["level"], "j": j, "p": p, "a": a, "b": b, "order": x["order"], "r": r,
                           "gn": x["gn"], "models": models})
        out[V] = curves
        json.dump({"V": V, "rule": "census/seeds.json M", "curves": curves}, open(f"{STUDY}/census/{V}/M.json", "w"), indent=1)
        print(V, [(c["id"], c["level"]) for c in curves])


if __name__ == "__main__":
    {"tcr": tcr, "m": mchoice}[sys.argv[1]]()
