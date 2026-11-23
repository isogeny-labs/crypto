import json, random, sys, math
from math import comb

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d = n-1; s = 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def setup(nbits_key):
    q = (1 << (nbits_key+1)) + 1
    while not is_prime(q): q += 2
    k = 2
    while not is_prime(k*q+1): k += 2
    P = k*q+1
    r = random.Random(1)
    while True:
        g = pow(r.randrange(2, P-1), k, P)
        if g != 1: break
    return P, q, g

def combos(offset, half, wt):
    out = []
    def rec(start, left, acc):
        if left == 0: out.append(acc); return
        for i in range(start, half-left+1): rec(i+1, left-1, acc | (1 << (offset+i)))
    rec(0, wt, 0)
    return out

def run(n, w, trials, sfr, tfr, seed):
    hw = w//2; half = n//2
    P, q, g = setup(n)
    ginv = pow(g, -1, P)
    Lv = combos(0, half, hw); Rv = combos(half, half, hw)
    Lmap = {pow(g, a, P): i for i, a in enumerate(Lv)}
    Rinv = [pow(ginv, b, P) for b in Rv]
    L = len(Lv); R = len(Rv)
    psplit = comb(half,hw)**2 / comb(n,w)
    rng = random.Random(seed)
    cells = []
    for sf in sfr:
        for tf in tfr:
            S = max(1, round(sf*L)); T = max(1, round(tf*R)); hit = 0; wrong = 0
            for _ in range(trials):
                pos = rng.sample(range(n), w); x = sum(1 << i for i in pos)
                tab = set(rng.sample(range(L), S)); rs = rng.sample(range(R), T)
                h = pow(g, x, P); found = None
                for j in rs:
                    i = Lmap.get(h*Rinv[j] % P)
                    if i is not None and i in tab:
                        found = Lv[i] + Rv[j]; break
                if found is not None:
                    if pow(g, found, P) == h: hit += (found == x) or 1
                    else: wrong += 1
            e = S/L*T/R*psplit
            cells.append(dict(S=S, T=T, obs=hit, trials=trials, pred=e, wrong=wrong))
    return dict(P=P, q=q, L=L, R=R, psplit=psplit), cells

def stat(cells):
    z2 = 0.0; k = 0
    for c in cells:
        mu = c["trials"]*c["pred"]; var = mu*(1-c["pred"])
        c["z"] = (c["obs"]-mu)/math.sqrt(var) if var > 0 else 0.0
        z2 += c["z"]**2; k += 1
    return z2, k

def main():
    smoke = "--smoke" in sys.argv
    if smoke: n, w, trials, sfr, tfr = 16, 4, 60, (0.5, 1.0), (0.5, 1.0)
    else: n, w, trials, sfr, tfr = 30, 8, 1500, (0.25, 0.5, 1.0), (0.25, 0.5, 1.0)
    info, cells = run(n, w, trials, sfr, tfr, 20261110)
    z2, k = stat(cells)
    # A/A arm: same protocol, different seed
    info2, cells2 = run(n, w, trials, sfr, tfr, 777)
    z2b, _ = stat(cells2)
    # negative control: weight w+2 keys vs same table; no hit expected beyond collisions
    rng = random.Random(5); P, q, g = setup(n); ginv = pow(g, -1, P)
    Lv = combos(0, n//2, w//2); Rv = combos(n//2, n//2, w//2)
    Lmap = {pow(g, a, P): i for i, a in enumerate(Lv)}; Rinv = [pow(ginv, b, P) for b in Rv]
    neg = 0; nt = 200 if smoke else 600
    for _ in range(nt):
        x = sum(1 << i for i in rng.sample(range(n), w+2)); h = pow(g, x, P)
        for r_ in Rinv:
            i = Lmap.get(h*r_ % P)
            if i is not None and Lv[i]+Rv[Rinv.index(r_)] == x: neg += 1; break
    pos = cells[-1]  # S=L,T=R cell: full table, full right enumeration
    pos_rate = pos["obs"]/pos["trials"]
    pos_ok = abs(pos_rate - info["psplit"]) < 4*math.sqrt(info["psplit"]*(1-info["psplit"])/pos["trials"])
    thr = k + 4*math.sqrt(2*k)  # chi2 ~ k, 4 sigma
    wrong = sum(c["wrong"] for c in cells)
    if smoke: verdict = "inconclusive"
    elif neg > 0 or not pos_ok or wrong: verdict = "inconclusive"
    elif z2 <= thr and z2b <= thr: verdict = "consistent_with_prediction"
    elif z2 > thr and z2b > thr: verdict = "falsified"
    else: verdict = "inconclusive"
    rho = math.sqrt(math.pi*info["q"]/4)
    print(json.dumps(dict(lead="LEAD-RHO-lhw-preprocessing-frontier", smoke=smoke, n=n, w=w,
        seeds=[20261110, 777, 1, 5], params=info, chi2=z2, chi2_AA=z2b, chi2_threshold=thr, dof=k,
        cells=cells, cells_AA=cells2, negative_control_hits=neg, positive_control_rate=pos_rate,
        positive_control_ok=pos_ok, wrong_recoveries=wrong, rho_online_ops_reference=rho,
        T_over_rho_per_cell=[c["T"]/rho for c in cells], verdict=verdict)))
main()
