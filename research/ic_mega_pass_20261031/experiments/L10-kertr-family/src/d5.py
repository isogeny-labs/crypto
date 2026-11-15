"""usage: d5.py n s CPUCAP -- gf2close5 d=5 closure on T draw (n,s)."""
import sys, os, json, time
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from lib10 import *
n, s, cap = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
eqs, meta = draw('T', n, s); N = 2 * meta['k']; t = time.time()
c = call('closure', N, 5, eqs, binary=BIN5, env=dict(CPUCAP=cap, PROGRESS='1'))
rec = dict(fam='T', n=n, s=s, d=5, cap=float(cap), **c, wall=round(time.time() - t, 1))
open(os.path.join(HERE, '..', 'raw', 'd5_T%d_s%d.jsonl' % (n, s)), 'a').write(json.dumps(rec) + '\n'); print(json.dumps(rec))
