import sys, json, time, random, itertools
from n3lib import *
out = []; T0 = time.time()
def run_cell(m, d, l, p, Mname, M, rng, f, seed):
    Mx = max(max(s) for s in M); n = len(M); N = 6*n; hist = []
    while True:
        pts = sample_U(f, m, p, l, N, rng)
        g = gain(eval_matrix(M, pts, p**l), p, l); hist.append((N, g))
        if len(hist) >= 3 and hist[-1][1] == hist[-2][1] == hist[-3][1]: break
        if N > 40*n: break
        N *= 2
    K = ksum(M, d, l); D = degsum(M)
    rec = dict(m=m, l=l, p=p, M=Mname, n=n, Mmax=Mx, seed=seed, gain=hist[-1][1], ksum=K, plateau=hist,
               delta_sect=hist[-1][1]/D, delta_k=K/D, artifact_zone=(p <= Mx),
               status=("equal" if hist[-1][1]==K else ("above" if hist[-1][1]>K else "below")))
    print(json.dumps(rec), flush=True); out.append(rec)

def box(m, M): return [s for s in itertools.product(range(M+1), repeat=m)]
mode = sys.argv[1]
if mode == "m2":
    for seed in (0, 1):
      for p in (3,5,7,11,13,17,23,31):
        rng = random.Random(1000*seed+p); f, crv = fpoly(2, p, rng)
        for l in (1,2,3):
            for t in (0,2):
                if p**l > 4*10**4: continue
                run_cell(2,2,l,p,f"box{2*l+t}",box(2,2*l+t),rng,f,seed)
            Mx = 2*l+2; B = box(2, Mx)
            holes = [s for s in B if not (1 <= s[0] <= Mx-1 and 1 <= s[1] <= Mx-1 and (s[0]+s[1]) % 3 == 0)]
            cut = [s for s in B if not (s[0] >= Mx-1 and s[1] >= Mx-1)]
            r2 = random.Random(7+l); rnd = [s for s in B if r2.random() < 0.7]
            for nm, M in (("holes",holes),("cutTopCorner",cut),("rand70",rnd)):
                if p**l > 4*10**4: continue
                run_cell(2,2,l,p,nm+str(Mx),M,rng,f,seed)
elif mode == "m2hp":
    # POST-HOC extension (not in protocol): larger p, record #V(F_p) to locate saturation threshold
    for seed in (0, 1, 2):
      for p in (29, 31, 37, 41, 43, 47, 53):
        rng = random.Random(5000*seed+p); f, crv = fpoly(2, p, rng)
        npts = sum(1 for x in range(p) for y in range(p) if evalf(f,[x,y],p)==0)
        for l in (2,3):
            Mx = 2*l+2; B = box(2, Mx)
            holes = [s for s in B if not (1 <= s[0] <= Mx-1 and 1 <= s[1] <= Mx-1 and (s[0]+s[1]) % 3 == 0)]
            cut = [s for s in B if not (s[0] >= Mx-1 and s[1] >= Mx-1)]
            for nm, M in ((f"box{2*l}",box(2,2*l)),(f"box{Mx}",B),("holes",holes),("cutTop",cut)):
                run_cell(2,2,l,p,nm,M,rng,f,seed); out[-1]["npts"]=npts
elif mode == "m3":
    for p in (5,7,11,13,17):
        rng = random.Random(p); f, crv = fpoly(3, p, rng)
        for l in (1,2):
            if l == 2 and time.time()-T0 > 600: print("skip l=2 time"); continue
            run_cell(3,4,l,p,f"box{4*l}",box(3,4*l),rng,f,0)
json.dump(out, open(f"raw/armA_{mode}.json","w"), indent=0)
print("time", time.time()-T0)
