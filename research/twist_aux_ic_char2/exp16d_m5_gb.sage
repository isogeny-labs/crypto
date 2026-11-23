# EXP16d-3c: Groebner basis of the reduced m = 5 system (t1..t4 = s1^2..s4^2, s5) under wdegrevlex(2,2,2,2,1),
# Singular via libsingular (std or slimgb). Run under ulimit -t for a hard CPU limit. Prints progress-free result:
# GB cpu time, GB size, ideal degree (if normal basis is countable), planted-point check.
# usage: sage exp16d_m5_gb.sage sobj_path algo(std|slimgb) [order: w (default) | dp]
import sys, time
dat = load(sys.argv[1]); algo = sys.argv[2]; ordr = sys.argv[3] if len(sys.argv) > 3 else 'w'
d = dat['d']; Q = dat['Q']
Fq = GF(2^d, 'z', modulus=PolynomialRing(GF(2), 'x')(dat['modulus'])); z = Fq.gen()
o = TermOrder('wdegrevlex', (2,2,2,2,1)) if ordr == 'w' else 'degrevlex'
Rr = PolynomialRing(Fq, 't1,t2,t3,t4,s5', order=o)
red = [Rr(s) for s in dat['red']]; pl = [Fq(s) for s in dat['planted_red']]
print(f"d={d} Q={Q} algo={algo} order={o}; planted point satisfies system: {all(f(*pl) == 0 for f in red)}", flush=True)
c0 = time.process_time(); w0 = time.time()
gb = Rr.ideal(red).groebner_basis('libsingular:' + algo)
tg = time.process_time()-c0
print(f"GB done: cpu {tg:.1f}s wall {time.time()-w0:.1f}s; {len(gb)} elements; max deg {max(g.degree() for g in gb)}", flush=True)
c0 = time.process_time()
I = Rr.ideal(gb)
print(f"dimension {I.dimension()}", flush=True)
dg = I.vector_space_dimension()
print(f"ideal degree (vector_space_dimension) {dg}  ({time.process_time()-c0:.1f}s)", flush=True)
