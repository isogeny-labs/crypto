"""Pollard rho on classes (SPEC section 9)."""
import math, time
import ff, jac, aut
from ff import CTR, uncounted, rmul
from seeds import rng
from ic import make_instance

M64 = (1 << 64) - 1

def run_rho(C, s, arm, check=0):
    t0 = time.time()
    ff.set_p(C.p)
    p, r, g = C.p, C.r, C.g
    G, T, t_pl = make_instance(C, s)
    CTR.reset(); jac.OPS[0] = 0
    if arm == "Rneg":
        m = 2
        def canon(D):
            D2, sg = aut.canon_neg(D); return D2, sg, 0
    else:
        m = 2 * C.aq if C.aq > 1 else 2
        def canon(D):
            return aut.canon_aut(D, C)
    x = rng("rho:%s:%d" % (C.curve_id, s))
    def upd(a, b, sg, k):
        if k:
            lk = C.LP[k]
            a = rmul(a, lk, r); b = rmul(b, lk, r)
        if sg == -1:
            a = (-a) % r; b = (-b) % r
        return a, b
    jumps = []
    for j in range(32):
        al = x.randrange(r); be = x.randrange(r)
        jumps.append((al, be, jac.add(jac.smul(al, G, C), jac.smul(be, T, C), C)))
    E = math.sqrt(math.pi * r / (2 * m))
    k_dp = max(2, int(math.floor(math.log2(E) / 2)))
    maxlen = 20 * 2 ** k_dp
    cap = 40 * math.ceil(E)
    steps = 0; look = 0; started = 0; abandoned = 0; aband_steps = 0
    dp = {}
    t_found = None; status = "cap"
    checks_done = 0

    def new_walk():
        a = x.randrange(r); b = x.randrange(r)
        W = jac.add(jac.smul(a, G, C), jac.smul(b, T, C), C)
        W, sg, k = canon(W)
        a, b = upd(a, b, sg, k)
        return W, a, b

    def verify(W, a, b):
        with uncounted():
            D = jac.add(jac.smul(a, G, C), jac.smul(b, T, C), C)
            return D == W

    W, a, b = new_walk(); started += 1; ln = 0
    done = False
    while not done:
        j = ((jac.K(W, g, p) * 0x9E3779B97F4A7C15) & M64) >> 59
        for i in range(32):
            jj = (j + i) % 32
            al, be, Rj = jumps[jj]
            W2, sg, k = canon(jac.add(W, Rj, C))
            a2, b2 = upd((a + al) % r, (b + be) % r, sg, k)
            steps += 1
            if i > 0:
                look += 1
            if ((jac.K(W2, g, p) * 0x9E3779B97F4A7C15) & M64) >> 59 != jj or i == 31:
                W, a, b = W2, a2, b2
                break
        ln += 1
        if check and checks_done < check:
            assert verify(W, a, b), "bookkeeping W != aG+bT"
            checks_done += 1
        if ((jac.K(W, g, p) * 0xC2B2AE3D27D4EB4F) & M64) >> (64 - k_dp) == 0:
            kk = tuple(jac.key(W, g, p))
            if kk in dp:
                a1, b1 = dp[kk]
                if (b - b1) % r != 0:
                    tt = ((a1 - a) * pow(b - b1, -1, r)) % r
                    with uncounted():
                        ok = (tt == t_pl and jac.smul_rl(tt, G, C) == T)
                    if ok:
                        t_found = tt; status = "ok"; break
            else:
                dp[kk] = (a, b)
            W, a, b = new_walk(); started += 1; ln = 0
        elif ln > maxlen:
            abandoned += 1; aband_steps += ln
            W, a, b = new_walk(); started += 1; ln = 0
        if steps > cap:
            status = "cap"; break
    with uncounted():
        certified = (t_found is not None and t_found == t_pl and jac.smul_rl(t_found, G, C) == T)
    return {
        "run_id": "%s|%s|%d" % (C.curve_id, arm, s), "family": C.family, "curve_id": C.curve_id,
        "bits": C.bits, "p": p, "g": g, "q": C.q, "h": C.h, "r": r, "seed": s, "arm": arm,
        "m": m, "k_dp": k_dp, "E": E, "steps": steps, "lookahead_extra": look,
        "walks_started": started, "walks_abandoned": abandoned, "abandoned_steps": aband_steps,
        "dps_stored": len(dp), "grp_ops": jac.OPS[0], "mul": CTR.mul, "inv": CTR.inv,
        "rmul_bookkeeping": CTR.rmul, "t_found": t_found, "t_planted": t_pl,
        "certified": bool(certified), "status": status, "wall_s": round(time.time() - t0, 3)}
