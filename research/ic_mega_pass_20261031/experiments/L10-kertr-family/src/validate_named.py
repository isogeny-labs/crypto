"""Builder equivalence + named-instance closure reproduction."""
import sys, os, json, time
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from lib10 import *
from gf2n import Field
from s3sys import Subspace, s3_descent
R = '/Volumes/SSD990-2/crypto-autoresearcher/analysis/semaev-m2-last-fall-degree/results/instances.json'
recs = {r['name']: r for r in json.load(open(R))}
names = sys.argv[1].split(','); ds = [int(x) for x in sys.argv[2].split(',')]
out = open(os.path.join(HERE, '..', 'raw', 'named.jsonl'), 'a')
for nm in names:
    r = recs[nm]; n = r['n']; mod = int(r['modulus_hex'], 16)
    B = [int(x, 16) for x in r['V_basis_hex']]; z = int(r['z_hex'], 16); a6 = int(r['a6_hex'], 16)
    mine = build(n, mod, B, z, a6)
    F = Field(n, mod); ref = s3_descent(F, Subspace(F, B), z, a6)
    same = [frozenset(e) for e in mine] == [frozenset(p) for p in ref]
    N = 2 * len(B)
    so = call('bsols', N, 2, mine)
    rk = span_rank(mine, N)
    for d in ds:
        t = time.time(); c = call('closure', N, d, mine)
        pub = r['closures'].get(str(d), {}).get('W_dims')
        rec = dict(name=nm, n=n, d=d, mod_eq_N1=(modulus(n) == mod), builder_equal=same, nsol=so['nsol'],
                   pub_nsol=r['boolean_solutions'], span_rank=rk, low=c['low'], dim=c['dim'], one=c['one'],
                   pub=pub, match=(pub is not None and [c['low'], c['dim'], c['one']] == [pub[d-1], pub[d], int(r['closures'][str(d)]['W_one'])]),
                   cpu=c['cpu'], wall=round(time.time() - t, 1))
        print(json.dumps(rec), flush=True); out.write(json.dumps(rec) + '\n'); out.flush()
