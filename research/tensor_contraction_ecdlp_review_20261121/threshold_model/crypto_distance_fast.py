import math, sys, time
sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else ".")
import threshold_model as tm, qb_model as qm
from threshold_model import run_levels, lemmaC_threshold
from qb_model import run_levels_qb, top_qb
tm_thin, qm_thin = tm.thin, qm.thin
tm.thin = lambda f, keep=250: tm_thin(f, 40)
qm.thin = lambda f, keep=250: qm_thin(f, 40)
R2 = 1/math.sqrt(2)
ZG = [2.0 ** (-j / 1.0) for j in range(1, 45)]
ZTOP = [2.0 ** (-j / 4.0) for j in range(4, 240)]
SG = [2 ** j for j in range(0, 80, 2)]
STOP = [2 ** j for j in range(0, 90)]
EPS = [2.0 ** (-j / 2.0) for j in range(0, 12)]
def hoeff(b, oz, oK, rho=R2):
    best=None
    for x0 in (6, 8, 10):
        fr=run_levels(x0,0.2554,rho,b,oz,oK,zgrid=ZG,max_levels=70)
        r=lemmaC_threshold(fr,b,eps_grid=EPS,zgrid_top=ZTOP)
        if r and (best is None or r[0]<best[0]): best=(r[0],r[1])
    return best
def quant(b, rho=R2):
    best=None
    for x0 in (6, 8, 10):
        fr=run_levels_qb(x0,0.2554,rho,b,1.0,max_levels=70,sgrid=SG)
        r=top_qb(fr,b,eps_grid=EPS,sgrid=STOP)
        if r and (best is None or r[0]<best[0]): best=(r[0],r[1])
    return best
C_D, C_D_OPT = 0.0881*math.sqrt(4.4e10), 0.0881*math.sqrt(3.7e5)
rows=[("check: #3-SAT density 1",0.1321,None),("check: #3-SAT density 4.26",0.0302,None),
      ("general 3-SAT (reference)",3.2e-4,None),("Ring-LWE-1024 binary secret",128/1.13e6,math.log2(1.13e6)),
      ("AES-128 key recovery (CNF)",128/6e6,math.log2(6e6)),("ECDLP-256 (Bochum IC, m=4)",64/6.8e7,math.log2(6.8e7))]
for name,b,have in rows:
    t=time.time()
    m0=hoeff(b,3,3); m1=hoeff(b,2,1); m2=quant(b); m3=quant(b,0.5)
    hv=f"{have:5.1f}" if have else "  -  "
    print(f"{name:30s} bar={b:9.2e} have={hv} | Majd {m0[0]:9.3g} ({m0[1]} lev) | reread {m1[0]:9.3g} | LemmaQ {m2[0]:9.3g} ({m2[1]} lev) | Q+rho1/2 {m3[0]:9.3g} ({m3[1]} lev) | AppD {(C_D/b)**2:8.2g} | AppD-opt {(C_D_OPT/b)**2:8.2g} | floor {2/b:8.3g} | poly {2*math.log2(1/b)+2:5.1f}   [{time.time()-t:.0f}s]", flush=True)
