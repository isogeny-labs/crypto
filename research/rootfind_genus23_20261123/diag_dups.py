#!/usr/bin/env python3
"""Post-hoc diagnostic (not in PROTOCOL.md): where do the g=2, p=65537 sibling duplicates come from?
Replays arm B's walk for one curve and classifies each duplicate sibling."""
import json, sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
import sibwalk as S

cells = json.load(open(sys.argv[1]))
cs = int(sys.argv[2]); n = int(sys.argv[3])
cell = [c for c in cells if c['curve_seed'] == cs and c['g'] == 2][0]
p, g, r, h = cell['p'], cell['g'], cell['r'], cell['h']
C = S.JCurve(p, g, cell['f'])
rng = S.seed_rng(f"rootfind_genus23|{p}|{cs}|1")
while True:
    G = C.mul_ltr(C.random_divisor(rng), h)
    if G != S.IDENT: break
klog = rng.randrange(1, r); T = C.mul_ltr(G, klog)
short = min(r, 1 << 12)
jumps = []
for _ in range(16):
    a, b = rng.randrange(1, short), rng.randrange(1, short)
    jumps.append((C.cantor(C.mul_ltr(G, a), C.mul_ltr(T, b)), a, b))
# order of G divides r?
print("r*G == 0:", C.mul_ltr(G, r) == S.IDENT)
fwd_first = {}; sib_first = {}
fwd_dup = sib_dup_fwd = sib_dup_sib = 0
cur = S.IDENT; since = 4096; jprev = None; seg = -1; cyc = 0
for i in range(n):
    if since >= 4096:
        a, b = rng.randrange(1, r), rng.randrange(1, r)
        cur = C.cantor(C.mul_ltr(G, a), C.mul_ltr(T, b)); since = 0; jprev = None; seg += 1
    j = (S.mix64(C.key(cur)) >> 24) % 16
    M = jumps[j][0]
    nxt = C.cantor(cur, M)
    sib = C.cantor(cur, C.negate(M)) if j != jprev else None
    cur = nxt; jprev = j; since += 1
    if cur in fwd_first:
        fwd_dup += 1
        if fwd_first[cur][0] == seg: cyc += 1
    else: fwd_first[cur] = (seg, i)
    if sib is not None:
        if sib in fwd_first: sib_dup_fwd += 1
        elif sib in sib_first: sib_dup_sib += 1
        else: sib_first[sib] = i
print(dict(curve=cs, r=r, steps=n, fwd_dup=fwd_dup, fwd_dup_same_segment=cyc, sib_dup_fwd=sib_dup_fwd, sib_dup_sib=sib_dup_sib))
