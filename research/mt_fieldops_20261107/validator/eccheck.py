# Independent EC check: Jacobian-coordinate scalar multiplication (no affine slope formulas), Miller-Rabin.
import json, random, subprocess, sys, os
from concurrent.futures import ProcessPoolExecutor
DBG = sys.argv[2]
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True
class C:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def dbl(s, P):
        X, Y, Z = P; p = s.p
        if Z == 0 or Y == 0: return (1, 1, 0)
        YY = Y*Y % p; S_ = 4*X*YY % p; M_ = (3*X*X + s.a*pow(Z, 4, p)) % p
        X3 = (M_*M_ - 2*S_) % p; Y3 = (M_*(S_ - X3) - 8*YY*YY) % p; Z3 = 2*Y*Z % p
        return (X3, Y3, Z3)
    def add(s, P, Q):
        p = s.p
        if P[2] == 0: return Q
        if Q[2] == 0: return P
        X1, Y1, Z1 = P; X2, Y2, Z2 = Q
        Z1Z1 = Z1*Z1 % p; Z2Z2 = Z2*Z2 % p
        U1 = X1*Z2Z2 % p; U2 = X2*Z1Z1 % p; S1 = Y1*Z2*Z2Z2 % p; S2 = Y2*Z1*Z1Z1 % p
        if U1 == U2: return s.dbl(P) if S1 == S2 else (1, 1, 0)
        H = (U2 - U1) % p; R = (S2 - S1) % p; HH = H*H % p; HHH = H*HH % p
        X3 = (R*R - HHH - 2*U1*HH) % p; Y3 = (R*(U1*HH - X3) - S1*HHH) % p; Z3 = H*Z1*Z2 % p
        return (X3, Y3, Z3)
    def mul(s, P, k):  # Montgomery ladder (different shape from the Rust double-and-add)
        R0, R1 = (1, 1, 0), P
        for bit in bin(k)[2:] if k > 0 else "":
            if bit == "1": R0, R1 = s.add(R0, R1), s.dbl(R1)
            else: R1, R0 = s.add(R0, R1), s.dbl(R0)
        return R0
    def aff(s, P):
        if P[2] == 0: return None
        zi = pow(P[2], -1, s.p); return (P[0]*zi*zi % s.p, P[1]*zi*zi*zi % s.p)
    def on(s, x, y): return (y*y - x*x*x - s.a*x - s.b) % s.p == 0
def check_run(argv):
    env = dict(os.environ, VAL_DUMP="1")
    pr = subprocess.run([DBG] + argv, capture_output=True, text=True, env=env)
    out = json.loads(pr.stdout.strip().splitlines()[-1])
    lines = pr.stderr.splitlines(); inst = dict(kv.split("=") for kv in lines[0].split()[2:])
    p, a, b, n, gx, gy = (int(inst[k]) for k in ("p","a","b","n","gx","gy"))
    c = C(p, a, b); G = (gx, gy, 1)
    res = dict(argv=argv[:2]+argv[2:], ok=out["ok"], targets=out["targets"], seen_uncounted=int(inst["seen_uncounted"]))
    res["inst_ok"] = is_prime(p) and is_prime(n) and c.on(gx, gy) and c.mul(G, n)[2] == 0 and (4*a**3 + 27*b*b) % p != 0
    bad = 0; badx = 0; offcurve = 0; distinct = set()
    for ln in lines[1:]:
        d = dict(kv.split("=") for kv in ln.split()[1:]); t, x, fo, qx, qy = (int(d[k]) for k in ("t","x","found","qx","qy"))
        if not c.on(qx, qy): offcurve += 1
        if c.aff(c.mul(G, fo)) != (qx, qy): bad += 1
        if c.aff(c.mul(G, x)) != (qx, qy): badx += 1
        distinct.add(fo)
    res.update(checked=len(lines)-1, wrong_found=bad, planted_mismatch=badx, q_offcurve=offcurve, distinct_logs=len(distinct))
    # negative control: corrupt found[0]; the binary's own verifier must drop ok by exactly 1
    env["VAL_CORRUPT"] = "1"
    o2 = json.loads(subprocess.run([DBG] + argv, capture_output=True, text=True, env=env).stdout.strip().splitlines()[-1])
    res["ok_corrupt"] = o2["ok"]
    return res
if __name__ == "__main__":
    rows = [json.loads(l) for l in open(sys.argv[1])]
    rnd = random.Random(20261107); pick = []
    for cell in sorted({(r["pb"], r["T"]) for r in rows}):
        for arm in ("ic4", "rho", "rho2", "bsgs2"):
            pick.append(rnd.choice([r for r in rows if (r["pb"], r["T"]) == cell and r["arm"] == arm])["argv"])
    with ProcessPoolExecutor(4) as ex:
        for r in ex.map(check_run, pick): print(json.dumps(r))
