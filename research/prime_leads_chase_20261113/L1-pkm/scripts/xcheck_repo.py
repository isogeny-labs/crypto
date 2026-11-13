import json, re, sys
ms, gj = sys.argv[1], sys.argv[2]
lines = open(ms).read().split('\n')
vars_ = lines[0].split(','); p = int(lines[1])
body = ''.join(lines[2:]).split(',')
G = json.load(open(gj)); m, t = G['m'], G['t']
def nf(poly):
    D = {}
    for term in poly.replace('-', '+-').split('+'):
        term = term.strip()
        if not term: continue
        fs = term.split('*'); c = int(fs[0]) % p
        be = [0]*m; ue = [0]*max(m-2,0)
        for f in fs[1:]:
            v, _, e = f.partition('^'); e = int(e) if e else 1
            if v.startswith('y'):
                i, j = map(int, v[1:].split('_')); be[i] = (be[i] + e * (1 << j)) % (1 << t)
            else:
                ue[int(v[1:])] += e
        k = tuple(be + ue); D[k] = (D.get(k, 0) + c) % p
    return {k: v for k, v in D.items() if v}
semaev = [b for b in body if 'u' in b or b.count('y') > 2 or ('y' in b and '^2' in b and b.count('*')>2)]
mine = [{tuple(k): v for k, v in P} for P in G['polys']]
theirs = [nf(b) for b in body]
ok = 0
for P in mine:
    for Q in theirs:
        if set(P) == set(Q):
            k0 = next(iter(P)); r = P[k0] * pow(Q[k0], p - 2, p) % p
            if all(P[k] == r * Q[k] % p for k in P): ok += 1; break
print('matched', ok, 'of', len(mine))
