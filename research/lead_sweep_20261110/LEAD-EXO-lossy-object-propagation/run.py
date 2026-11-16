import sys, json, random, math
SMOKE = '--smoke' in sys.argv
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True
def prime_near(bits, rng):
    while True:
        p = rng.getrandbits(bits) | (1<<(bits-1)) | 3
        if p % 4 == 3 and is_prime(p): return p
def sample(p, n, rng):
    """random points Q on random curve, with 2Q; returns (x,y,x2,y2) list"""
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p: break
    out = []
    while len(out) < n:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        y = pow(r, (p+1)//4, p)
        if y*y % p != r or y == 0: continue
        l = (3*x*x + a) * pow(2*y, -1, p) % p
        x2 = (l*l - 2*x) % p; y2 = (l*(x-x2) - y) % p
        out.append((x, y, x2, y2))
    return out
def projs(p):
    return {
      "top3x": lambda x,y: x*8//p,
      "low3x": lambda x,y: x & 7,
      "top2x_ypar": lambda x,y: (x*4//p)*2 + (y & 1),
    }
def mi(pairs, ka, kb):
    n = len(pairs); ca = [0]*ka; cb = [0]*kb; cab = {}
    for u, v in pairs:
        ca[u]+=1; cb[v]+=1; cab[(u,v)] = cab.get((u,v),0)+1
    s = 0.0
    for (u,v), c in cab.items():
        s += c/n*math.log(c*n/(ca[u]*cb[v]))
    return s
def zstat(us, vs, ka, kb, rng, nperm):
    obs = mi(list(zip(us,vs)), ka, kb)
    vs2 = vs[:]; null = []
    for _ in range(nperm):
        rng.shuffle(vs2); null.append(mi(list(zip(us,vs2)), ka, kb))
    m = sum(null)/len(null); sd = (sum((t-m)**2 for t in null)/(len(null)-1))**.5
    return obs, m, sd, (obs-m)/sd
def run():
    rng = random.Random(20261110)
    bits_list = [14, 16] if SMOKE else [16, 18, 20, 22]
    n = 3000 if SMOKE else 40000
    nperm = 10 if SMOKE else 30
    ncurves = 1 if SMOKE else 3
    cells = []; maxz = 0.0; aa = []
    for bits in bits_list:
        for c in range(ncurves):
            p = prime_near(bits, rng); pts = sample(p, n, rng)
            for name, f in projs(p).items():
                k = 8
                us = [f(x,y) for x,y,_,_ in pts]; vs = [f(x,y) for _,_,x,y in pts]
                obs, m, sd, z = zstat(us, vs, k, k, rng, nperm)
                # A/A: halves, same instrument, independent halves of the null-relation
                h = n//2
                o1 = mi(list(zip(us[:h],vs[:h])),k,k); o2 = mi(list(zip(us[h:],vs[h:])),k,k)
                aa.append(abs(o1-o2)/max(sd,1e-12))
                maxz = max(maxz, z)
                cells.append({"bits":bits,"p":p,"proj":name,"mi":obs,"null_mean":m,"null_sd":sd,"z":z})
    # positive control: lossy tracking of Z_N, top3 of k vs top3 of 2k mod N (known closed structure)
    N = 1<<(14 if SMOKE else 20); ks = [rng.randrange(N) for _ in range(n)]
    pu = [t*8//N for t in ks]; pv = [(2*t % N)*8//N for t in ks]
    pobs, pm, psd, pz = zstat(pu, pv, 8, 8, rng, nperm)
    # negative control: independent random pairs
    nu = [rng.randrange(8) for _ in range(n)]; nv = [rng.randrange(8) for _ in range(n)]
    nobs, nm, nsd, nz = zstat(nu, nv, 8, 8, rng, nperm)
    thr = 4.0 + 0.0  # per-cell, Bonferroni-ish for <=36 cells
    pos_ok = pz > 20; neg_ok = abs(nz) < 3.5
    aa_ok = (sum(aa)/len(aa)) < 4.0
    if not (pos_ok and neg_ok and aa_ok): verdict = "inconclusive"
    elif maxz > thr: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    return {"lead":"LEAD-EXO-lossy-object-propagation","smoke":SMOKE,"seed":20261110,
      "params":{"bits":bits_list,"n_points":n,"nperm":nperm,"curves_per_size":ncurves,"threshold_z":thr},
      "statistic":"max over cells of z=(MI(proj(Q);proj(2Q))-perm_null_mean)/null_sd","max_z":maxz,
      "positive_control":{"z":pz,"ok":pos_ok},"negative_control":{"z":nz,"ok":neg_ok},
      "aa_mean_halfsplit_diff_in_null_sd":sum(aa)/len(aa),"aa_ok":aa_ok,
      "cells":cells,"verdict":verdict}
print(json.dumps(run()))
