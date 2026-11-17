"""usage: run.py FAM n U [s0] -- scan seeds, d=4 closure on UNSAT draws until U UNSAT. FAM in T,R,A."""
import sys, os, json, time
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from lib10 import *
fam, n, U = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]); s = int(sys.argv[4]) if len(sys.argv) > 4 else 0
k = (n + 1) // 2; N = 2 * k
if fam == 'A':
    sys.path.insert(0, os.path.join(EXP, 'N1-gf2-d4-profile', 'src')); import lib as N1
fn = os.path.join(HERE, '..', 'raw', '%s_n%d.jsonl' % (fam, n)); f = open(fn, 'a'); u = 0
while u < U and s < 400:
    t = time.time()
    if fam == 'A': eqs = N1.system(n, k, s)['A']; meta = dict(fam='A', n=n, k=k, s=s)
    else: eqs, meta = draw(fam, n, s)
    so = call('bsols', N, 2, eqs); rk = span_rank(eqs, N)
    rec = dict(meta, nsol=so['nsol'], span_rank=rk[0], span_low=rk[1], cpu_sols=so['cpu'])
    if so['nsol'] == 0:
        c = call('closure', N, 4, eqs); u += 1
        rec.update(low4=c['low'], dim4=c['dim'], one=c['one'], ncols=c['ncols'], nprod=c['nprod'], cpu=c['cpu'])
    rec['wall'] = round(time.time() - t, 1)
    f.write(json.dumps(rec) + '\n'); f.flush(); print(json.dumps(rec), flush=True); s += 1
